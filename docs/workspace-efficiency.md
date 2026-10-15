# Efficient disassembly workspaces

Build with Rust 1.90.0 using `cargo build --locked --release`. New workspaces use
that binary by default. Upgrade an existing local workspace with
`python3 scripts/upgrade-disassembly.py /path/to/workspace` to preview, then add
`--apply`. The upgrade requires an unchanged input ROM and an idle local RPC
bridge. Existing setup files are copied into `analysis/setup-backups/<timestamp>`
before replacement. The manifest includes prior setup hashes and protected ROM,
map and history hashes. Any installation failure restores replaced setup files.
Generated queue/audit files are advisory; they cannot annotate the ROM.

The helper installs current AGENTS.md, workflow.md, run.py, start/resume/goal
prompts, analysis/session.py and analysis/work.py. It updates workspace.json's
binary path. The original state, maps, historical progress, investigation,
gameplay and issue records stay intact. To roll back setup, restore only the
backed-up configuration/scripts/prompts listed in the manifest; do not replace
ROM/state/maps from an unrelated checkpoint. Preserve new evidence and cache
files locally. A different binary digest automatically uses a different decode
cache namespace.

## Start and resume

Read `prompts/resume.md` and `analysis/frontier.md`. `prompts/goal.md` preserves
the strict whole-ROM objective and adds the efficient workflow. A provider's
stored goal must be updated through its own supported interface; changing files
does not resume an agent or alter its model. Keep the economical existing model
for routine work. Compare stronger reasoning only on specific difficult tickets,
with matched starting evidence and measured usage plus cooldown time.

```sh
python3 analysis/session.py --journal traces/session-NEW.jsonl
```

The bridge binds to localhost:8766 (`--port` overrides it; clients use
`ROMATLAS_RPC_URL`). Existing `from session import call` scripts work unchanged.
Load analysis/state.json with map.load, and start/replay the emulator when needed.
Snapshots from another process cannot be restored.

Only `analysis.decode` is cached. Actual ROM SHA-256, toolkit executable digest
and full parameters identify entries in local `analysis/cache/decode.sqlite`.
Successful results persist across scripts and sessions; a hit avoids RPC.
Misses verify the identity attached to the bridge response. ROM changes are rejected;
binary changes require restarting the bridge. The bridge rejects loading another
ROM. Cache entries are limited to 65536 and 64 MiB of result payload, with old
entries evicted in chunks. Errors are not cached. Maps, contextual discovery,
memory reads and runtime operations always execute. Use bounded multi-instruction
queries instead of one RPC per instruction where appropriate.

`python3 analysis/session.py --cache-stats` shows persistent storage and this
client process's counters. Cache hits are not new runtime evidence. Cached values
are private ROM-derived artifacts, never portable mapping metadata.

## Work queue and reports

After saving a classification state, run `python3 analysis/work.py refresh`.
It validates the internal store against the ROM, computes exact per-bank totals,
and writes current-audit.json, work-queue.json and a concise frontier.md. It does
not overwrite the old classification-audit.json or rewrite historical ledgers.
Gaps are split at bank boundaries. IDs use bank/start; split intervals retain
parent IDs and superseded entries stay in queue history. Changed state flags
dependency-bearing tickets for review. A missing plan is untriaged, not exhausted.

Use `python3 analysis/work.py plan analysis/PLAN.json` to rank an experiment:

```json
{
  "id": "U-B00-00003",
  "hypothesis": "A reviewed caller establishes this table's bounds",
  "experiment": "Inspect that caller and compare its complete index domain",
  "expected_outcome": "A proven bounded extent or a specific rejected hypothesis",
  "expected_bytes": 32,
  "estimated_minutes": 10,
  "evidence": ["analysis/prior-review.json"],
  "dependencies": ["caller index invariant"],
  "revisit_when": "New evidence of another writer to the index"
}
```

Use the actual current ticket ID and evidence; this is a schema example. Initial
ranking uses gap size, then reviewed expected gain/time estimates. Neither size
nor an estimate licenses classification. A small helper can rank highly if it
unlocks a large family. A blocked ticket requires a specific recorded result and
reopen condition. Preserve competing explanations and prior negative results.

After an accepted batch, validate/save/export through normal APIs. Run
`python3 run.py checkpoint pass-NEW --timeout-ms 120000` for a coherent pass.
The runner validates the internal state when present, snapshots it, saves the
exact stock map, generates, verifies and records stage timings and identities.
It rejects a state/ROM that changes during the checkpoint. Use a new name; no
existing checkpoint is overwritten. Large raw stock exports no longer require
the old coalescing workaround for import performance.

Then run `python3 analysis/work.py record analysis/BATCH.json`:

```json
{
  "id": "pass-NEW",
  "summary": "What new structural evidence was established",
  "evidence": ["analysis/consumer-review.json"],
  "verification": "analysis/pass-NEW-verification.json",
  "ticket_updates": [{
    "id": "U-B00-00003",
    "status": "waiting_dependency",
    "result": "The reviewed caller cannot bound all index writes",
    "experiment": "Inspect the identified alternate writer",
    "revisit_when": "That writer's complete domain is established"
  }],
  "model_trial": {
    "model": "existing model", "reasoning": "existing setting",
    "metered_usage_delta": null, "active_seconds": null,
    "cooldown_seconds": null, "accepted_bytes": null,
    "resolved_dependencies": [], "corrections": []
  }
}
```

Evidence files must exist locally. Verification, when supplied, must match the
current ROM and current.sym hash; old/compact-map proofs remain historical unless
their exact map matches. New classification is measured from the last recorded
batch's audit, not from the most recent queue refresh. Metadata refinements may
produce zero new bytes; record the useful dependency/result rather than inventing
progress. A batch adds one accounting artifact and concise progress/ledger deltas.
Record runtime routes and sourced gameplay findings in the existing dedicated
files as needed. These helpers never mark the whole-ROM goal complete.

## Confirmed control-flow profiles

`analysis.discover_context` now supports independent reference/diagnostic pages
and optional ROM-bound profiles for confirmed inline dispatch and non-mapper
stores. See [protocol](protocol.md). Profiles require complete instruction bytes,
CPU/bank context and an evidence-backed invariant for all arrivals in that context.
They are explicit assertions, not automatic register inference. Reached code is
still Static unless fetched, data barriers remain effective, and unknown memory
mapping stays unknown. Keep profile JSON local, and pass it explicitly; none is
automatically inferred or installed during an upgrade.

Operational budgets, low yield, usage limits and byte-identical raw fallback
never replace evidence-backed classification or justify marking the goal complete.
