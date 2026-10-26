"""Pinned Wasm bundle builder and deliberately small configurable port server."""
import hashlib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import mimetypes
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
from urllib.parse import urlsplit

ROOT = Path(__file__).resolve().parents[1]
BINDGEN_VERSION = "0.2.129"
MAX_BYTES = 64 * 1024 * 1024


def execute(command, work):
    log = Path(work) / "checks/web-build.log"
    log.parent.mkdir(exist_ok=True)
    result = subprocess.run(command, cwd=work, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    with log.open("a") as output:
        output.write(result.stdout)
    if result.returncode:
        raise ValueError(f"browser build failed; see {log}:\n" + result.stdout[-8000:])


def build(work):
    work = Path(work)
    if not (work / "port/web-enabled").is_file():
        raise ValueError("this workspace has no browser scaffold; existing ports are not upgraded")
    tool = ROOT / "tools/wasm-bindgen/bin/wasm-bindgen"
    if not tool.is_file():
        raise ValueError("missing wasm-bindgen; run bash scripts/setup-web.sh")
    version = subprocess.check_output([tool, "--version"], text=True).strip()
    if version != f"wasm-bindgen {BINDGEN_VERSION}":
        raise ValueError("wasm-bindgen version mismatch; run bash scripts/setup-web.sh")
    cargo = ["cargo", "--manifest-path", str(work / "port/Cargo.toml")]
    execute([cargo[0], "build", *cargo[1:], "--lib", "--features", "web",
             "--target", "wasm32-unknown-unknown", "--release", "--locked"], work)
    metadata = json.loads(subprocess.check_output([cargo[0], "metadata", *cargo[1:], "--no-deps", "--format-version", "1"], cwd=work))
    package = next(p for p in metadata["packages"] if p["manifest_path"] == str(work / "port/Cargo.toml"))
    library = next(t for t in package["targets"] if "cdylib" in t["kind"])
    wasm = Path(metadata["target_directory"]) / "wasm32-unknown-unknown/release" / (library["name"].replace("-", "_") + ".wasm")
    bundle = work / "web-dist"
    with tempfile.TemporaryDirectory(prefix=".web-build-", dir=work) as temporary:
        stage = Path(temporary)
        execute([tool, "--target", "web", "--out-dir", stage, "--out-name", "game", wasm], work)
        for filename in ("index.html", "shell.js", "save-state.mjs", "browser-gamepads.mjs"):
            shutil.copyfile(ROOT / "scripts/port-web" / filename, stage / filename)
        # Only generated shell/bindings: never copy the ROM or decoded assets.
        bundle.mkdir(exist_ok=True)
        for path in stage.iterdir():
            os.replace(path, bundle / path.name)
    return bundle


def read_bounded(path, limit):
    with Path(path).open("rb") as file:
        data = file.read(limit + 1)
    if len(data) > limit:
        raise ValueError(f"file exceeds {limit} bytes: {path}")
    return data


def asset_routes(root):
    manifests, files, routes = {}, {}, {}
    if root is None:
        return {"enabled": root is not None, "manifests": manifests, "files": files}, routes
    root = Path(root).resolve(strict=True)
    total = 0
    # One top-level manifest per collection, as used by the desktop loader.
    for path in sorted(root.glob("*.json")):
        if not path.resolve(strict=True).is_relative_to(root):
            raise ValueError("asset manifest escapes root")
        raw = read_bounded(path, 16 * 1024 * 1024)
        try:
            manifest = json.loads(raw)
        except (ValueError, UnicodeError):
            continue  # May be a replacement asset rather than a manifest.
        if not isinstance(manifest, dict) or "entries" not in manifest:
            continue
        if len(raw) > 1024 * 1024:
            raise ValueError("asset manifest exceeds 1 MiB")
        if (set(manifest) - {"version", "entries", "order"} or manifest.get("version") != 1
                or not isinstance(manifest.get("entries"), list)):
            raise ValueError("invalid asset manifest")
        manifests[path.name] = manifest
        for entry in manifest["entries"]:
            name = entry.get("file")
            if (not isinstance(name, str) or "\\" in name or not name
                    or any(part in {"", ".", ".."} for part in name.split("/"))
                    or Path(name).is_absolute()):
                raise ValueError("asset path must remain inside asset directory")
            physical = (root / name).resolve(strict=True)
            if not physical.is_relative_to(root):
                raise ValueError("asset path escapes root")
            if name not in files:
                data = read_bounded(physical, 16 * 1024 * 1024)
                total += len(data)
                if total > MAX_BYTES:
                    raise ValueError("asset collection exceeds 64 MiB")
                route = f"/assets/{len(routes)}"
                routes[route] = (data, "application/octet-stream")
                files[name] = route
    return {"enabled": root is not None, "manifests": manifests, "files": files}, routes


def server(work, rom, bundle, assets=None, port=8000, host="0.0.0.0"):
    if not 0 <= port <= 65535:
        raise ValueError("port must be in 0..65535")
    rom_bytes = read_bounded(rom, MAX_BYTES)
    sha256 = hashlib.sha256(rom_bytes).hexdigest()
    overlays, routes = asset_routes(assets)
    routes["/rom"] = (rom_bytes, "application/octet-stream")
    config = {"rom_sha256": sha256, "assets": overlays, "title": Path(work).name}
    routes["/config.json"] = (json.dumps(config).encode(), "application/json")
    for name in ["index.html", "shell.js", "save-state.mjs", "browser-gamepads.mjs", "game.js", "game_bg.wasm"]:
        path = Path(bundle) / name
        mime = "application/wasm" if name.endswith(".wasm") else mimetypes.guess_type(name)[0]
        routes["/" if name == "index.html" else "/" + name] = (path.read_bytes(), mime or "application/octet-stream")

    class Handler(BaseHTTPRequestHandler):
        def do_GET(self):
            # No directory listing, traversal, source trees, filesystem paths or writes.
            route = routes.get(urlsplit(self.path).path)
            if route is None:
                self.send_error(404)
                return
            data, mime = route
            self.send_response(200)
            self.send_header("Content-Type", mime)
            self.send_header("Content-Length", str(len(data)))
            self.send_header("Cache-Control", "no-store")
            self.send_header("X-Content-Type-Options", "nosniff")
            self.end_headers()
            try:
                self.wfile.write(data)
            except (BrokenPipeError, ConnectionResetError):
                pass

        def log_message(self, *_args):
            pass

    return ThreadingHTTPServer((host, port), Handler)


def serve(work, rom, bundle, assets, port, host="0.0.0.0"):
    with server(work, rom, bundle, assets, port, host) as http:
        address = http.server_address[0]
        browser_host = "127.0.0.1" if address == "0.0.0.0" else address
        print(json.dumps({"ok": True, "host": address, "url": f"http://{browser_host}:{http.server_port}/", "bundle": str(bundle)}), flush=True)
        try:
            http.serve_forever()
        except KeyboardInterrupt:
            pass
