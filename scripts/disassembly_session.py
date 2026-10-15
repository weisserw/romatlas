"""Local RPC bridge with an identity-bound cache for immutable physical decoding.

Installed as analysis/session.py. Existing `from session import call` clients work.
Only analysis.decode is cached; discovery, runtime, map and memory calls always run.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import sqlite3
import subprocess
import urllib.request
from http.server import BaseHTTPRequestHandler, HTTPServer

ROOT = Path(__file__).resolve().parents[1]
URL = os.environ.get("ROMATLAS_RPC_URL", "http://127.0.0.1:8766")
MAX_ENTRIES = 65536
MAX_CACHE_BYTES = 64 * 1024 * 1024
_identity = None
_fingerprint = None
_stats = {"hits": 0, "misses": 0}


def identity():
    global _identity, _fingerprint
    meta_path = ROOT / "workspace.json"
    meta = json.loads(meta_path.read_text())
    rom = ROOT / meta["rom"]
    binary = Path(meta["binary"])
    paths = [meta_path, rom, binary]
    fingerprint = tuple((str(p.resolve()), p.stat().st_mtime_ns,
                         p.stat().st_ctime_ns, p.stat().st_size) for p in paths)
    if fingerprint != _fingerprint:
        rom_hash = hashlib.sha256(rom.read_bytes()).hexdigest()
        if rom_hash != meta["sha256"]:
            raise ValueError("input ROM changed; cached decoding is unavailable")
        binary_hash = hashlib.sha256(binary.read_bytes()).hexdigest()
        _identity = hashlib.sha256(json.dumps(["physical-decode-v1", rom_hash,
                                               binary_hash]).encode()).hexdigest()
        _fingerprint = fingerprint
    return _identity


def cache_key(params):
    # Full parameters are part of the key, including future decoder options.
    return hashlib.sha256(json.dumps([identity(), params], sort_keys=True,
                                    separators=(",", ":")).encode()).hexdigest()


def cache_db():
    directory = ROOT / "analysis/cache"
    directory.mkdir(exist_ok=True)
    connection = sqlite3.connect(directory / "decode.sqlite", timeout=20)
    connection.execute("CREATE TABLE IF NOT EXISTS decoded "
                       "(key TEXT PRIMARY KEY, value TEXT NOT NULL, size INTEGER NOT NULL)")
    connection.execute("CREATE TABLE IF NOT EXISTS budget (id INTEGER PRIMARY KEY, entries INTEGER, bytes INTEGER)")
    if connection.execute("SELECT id FROM budget WHERE id=1").fetchone() is None:
        connection.execute("INSERT OR IGNORE INTO budget SELECT 1,COUNT(*),COALESCE(SUM(size),0) FROM decoded")
    connection.executescript("""
        CREATE TRIGGER IF NOT EXISTS cache_insert AFTER INSERT ON decoded BEGIN
          UPDATE budget SET entries=entries+1, bytes=bytes+NEW.size WHERE id=1;
        END;
        CREATE TRIGGER IF NOT EXISTS cache_delete AFTER DELETE ON decoded BEGIN
          UPDATE budget SET entries=entries-1, bytes=bytes-OLD.size WHERE id=1;
        END;
        CREATE TRIGGER IF NOT EXISTS cache_update AFTER UPDATE ON decoded BEGIN
          UPDATE budget SET bytes=bytes-OLD.size+NEW.size WHERE id=1;
        END;
    """)
    return connection


def cached_decode(params, fetch):
    key = cache_key(params)
    with cache_db() as db:
        row = db.execute("SELECT value FROM decoded WHERE key=?", (key,)).fetchone()
        if row is not None:
            _stats["hits"] += 1
            return json.loads(row[0])
    _stats["misses"] += 1
    result = fetch()
    value = json.dumps(result, separators=(",", ":"))
    size = len(value.encode())
    if size <= MAX_CACHE_BYTES:
        with cache_db() as db:
            db.execute("BEGIN IMMEDIATE")
            count, total = db.execute("SELECT entries,bytes FROM budget WHERE id=1").fetchone()
            # Bounded persistent storage; delete old entries in chunks, not per read.
            while count >= MAX_ENTRIES or total + size > MAX_CACHE_BYTES:
                db.execute("DELETE FROM decoded WHERE rowid IN "
                           "(SELECT rowid FROM decoded ORDER BY rowid LIMIT 1024)")
                count, total = db.execute("SELECT entries,bytes FROM budget WHERE id=1").fetchone()
            db.execute("INSERT INTO decoded VALUES (?,?,?) ON CONFLICT(key) "
                       "DO UPDATE SET value=excluded.value,size=excluded.size", (key, value, size))
    return result


def call(method, **params):
    def fetch():
        body = json.dumps(dict(method=method, params=params)).encode()
        request = urllib.request.Request(URL, data=body,
                                         headers={"Content-Type": "application/json"})
        with urllib.request.urlopen(request, timeout=330) as response:
            if method == "analysis.decode" and response.headers.get("X-ROMAtlas-Identity") != identity():
                raise RuntimeError("bridge identity differs; restart analysis/session.py")
            reply = json.load(response)
        if "error" in reply:
            raise RuntimeError(reply["error"])
        return reply["result"]
    return cached_decode(params, fetch) if method == "analysis.decode" else fetch()


def cache_stats():
    with cache_db() as db:
        count, size = db.execute("SELECT entries,bytes FROM budget WHERE id=1").fetchone()
    return dict(_stats, entries=count, payload_bytes=size, namespace=identity())


def serve(journal, port=8766):
    # The cached client is bound to this workspace's immutable ROM/binary, so the
    # bridge rejects switching ROMs and rejects calls after either file changes.
    initial_identity = identity()
    meta = json.loads((ROOT / "workspace.json").read_text())
    expected_rom = (ROOT / meta["rom"]).resolve()
    with Path(journal).open("x") as output:
        process = subprocess.Popen([str(meta["binary"]), "serve", "--rom", str(expected_rom)],
                                   cwd=ROOT, stdin=subprocess.PIPE,
                                   stdout=subprocess.PIPE, text=True)
        sequence = 0

        class Handler(BaseHTTPRequestHandler):
            def log_message(self, *_):
                pass

            def reply(self, value):
                payload = json.dumps(value).encode()
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("X-ROMAtlas-Identity", initial_identity)
                self.send_header("Content-Length", str(len(payload)))
                self.end_headers()
                self.wfile.write(payload)

            def do_GET(self):
                self.reply({"alive": process.poll() is None, "requests": sequence,
                            "identity": initial_identity})

            def do_POST(self):
                nonlocal sequence
                sequence += 1
                request = {}
                try:
                    length = int(self.headers.get("Content-Length", "0"))
                    if not 0 < length <= 1048576:
                        raise ValueError("invalid request length")
                    request = json.loads(self.rfile.read(length))
                    if identity() != initial_identity:
                        raise ValueError("workspace ROM/binary changed; restart the bridge")
                    if request.get("method") == "rom.open":
                        candidate = Path(request.get("params", {}).get("path", ""))
                        if not candidate.is_absolute():
                            candidate = ROOT / candidate
                        if candidate.resolve() != expected_rom:
                            raise ValueError("bridge is bound to the workspace ROM")
                        request["params"]["path"] = str(expected_rom)
                    request.update(jsonrpc="2.0", id=sequence)
                    process.stdin.write(json.dumps(request) + "\n")
                    process.stdin.flush()
                    reply = json.loads(process.stdout.readline())
                except (OSError, ValueError, TypeError, KeyError) as error:
                    reply = {"jsonrpc": "2.0", "id": sequence,
                             "error": {"code": -32000, "message": str(error)}}
                output.write(json.dumps({"request": request, "reply": reply}) + "\n")
                output.flush()
                self.reply(reply)

        try:
            with HTTPServer(("127.0.0.1", port), Handler) as server:
                print(f"Local RPC bridge ready on http://127.0.0.1:{server.server_port}", flush=True)
                server.serve_forever()
        finally:
            process.stdin.close()
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                process.terminate()
                process.wait(timeout=10)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--journal")
    parser.add_argument("--port", type=int, default=8766)
    parser.add_argument("--cache-stats", action="store_true")
    args = parser.parse_args()
    if args.cache_stats:
        print(json.dumps(cache_stats()))
    elif args.journal:
        serve(args.journal, args.port)
    else:
        parser.error("supply --journal or --cache-stats")
