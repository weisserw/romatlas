# Round 1 disassembly pass 1 - handoff (COMPLETE for this pass; ROM incomplete)

- Scope: baseline, observed coverage from gameplay scenarios, local static descent,
  handler-table leads, data classification (CHR/header/vectors, read clusters,
  low-decode-run generic data, padding).
- Checkpoint: **pass-001-final** (maps/pass-001-final.json, generated/pass-001-final,
  analysis/pass-001-final-verification.json: ok, byte_identical). Earlier intermediate
  checkpoint pass-001 (93.66%) retained. analysis/state.json = maps/current.json source.
- Classification: ROM classification: 97.60% (255857 / 262160 bytes classified; 6303 bytes unknown).
  Per-bank table in analysis/progress.md.
- Game: Chip 'n Dale Rescue Rangers (Capcom). RAM/progression contract and warp recipe in
  analysis/gameplay.md; scenarios S01-S09 + deferred milestones in analysis/scenarios.json.

## Findings
- Fixed bank 7 (PRG14/15) holds NMI ($C000), reset ($FFE0->$FE00), cooperative task
  scheduler ($FE90, tasks $80-$8F, yield $FF04), PRG bank switch $FF53 (A=16 KiB bank),
  CHR switches $FF1F/$FF39, joypad $C162, main loop $C77B, level start $C60E.
- 16 KiB bank 6 (PRG12/13) = object AI; dispatch table $80A7/$810C (101 entries, L-001).
- 16 KiB bank 3 (PRG06/07) = sound engine ($8000/$8003 entry, inline dispatcher $8023).
- 16 KiB bank 2 (PRG04/05) = cutscene $9000, zone map $8B00.
- Banks 0,1,4,5 (PRG00-03, 08-11) are almost entirely data.

## Verification
Checkpoint run validated the exported map, regenerated and byte-identically verified.

## Unresolved / deferred
- L-003: bosses, ending/credits not reached (need position/checkpoint RAM contract).
- L-004: 6303 unknown bytes (analysis/evidence/pass-001/unknown-ranges.json), mostly
  code-like unexecuted ranges in bank 6, bank 2 and fixed bank; look for further pointer
  tables ($81D9/$81E0, $8D25/$8D2A/$8D2F in bank 6; sound $8023 inline tables).
- L-005: generic data (75 KiB low-run + 19.6 KiB read clusters) needs specific formats;
  re-check against any new code evidence.
- Manual 2P game start not run (demo covered 2P).

## Leads produced
L-001 (verified), L-002 (verified), L-003 (ready), L-004 (ready), L-005 (ready), L-006 (verified).

## Artifacts
analysis/progress.md, analysis/investigation.md, analysis/leads.json,
analysis/scenarios.json, analysis/gameplay.md, analysis/toolkit-issues.md (TK-001..003),
analysis/evidence/pass-001/ (incl. tools/ scratch scripts; they reference the scratch
socket bridge /root/rascratch, which is not preserved), analysis/snapshots/ (zoneA_start,
zoneA_play; same-build snapshot exports).
