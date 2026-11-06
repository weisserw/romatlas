# Round 1 disassembly handoff

Pass complete; ROM incomplete.

ROM classification: 32.52% (42631 / 131088 bytes classified; 88457 bytes unknown).

6483 code /36148 data; 381 regions. Full bank counts and evidence in progress.md. Verified final checkpoint round01-disassembly-final: map.validate and byte-identical ca65 rebuild passed (analysis/round01-disassembly-final-verification.json). Baseline and earlier checkpoint preserved. State analysis/state.json; map maps/current.json.

Scope: vector startup, observed bank-aware gameplay/init contracts, static connected successors and direct-call entry closure, generic graphics backing and uniform-run data. Each region records evidence. Observed fetches and static reachability claims remain separate; readability/commentary pending.

Natural route reached title and scene frame716; later stage2/4/6/7 RAM/PC experiments produced distinct scenes but did not establish active player/boss/ending. See scenarios.json, gameplay.md, retained snapshots/captures. Trace final aggregate_dropped0, detail capacity0 intentional drops; initialization acceleration untraced.

Ready redirect R01-PROGRESSION: resolve F200..F273 player initialization, mode94 and C2B2/C2F2 task transition; E73C scene-only entry insufficient. Do not replay unchanged long inputs. Other unknown code/data consumers can support further static work; no terminal exhaustion claim. No initial leads existed.

No toolkit issues found; analysis/toolkit-issues.md. Input unchanged; only workspace evidence/maps/source created. Free disk8.5GiB scratch28KiB; no cleanup. RPC bridge exec session51699 remains live (scratch/rpc.py); durable state and snapshots suffice for restart.
