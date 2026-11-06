# Round 001 disassembly handoff

Incomplete. ROM classification: 89.86% (58908 / 65552 bytes classified; 6644 bytes unknown).

Verified checkpoint: maps/pass-001.json, generated/pass-001/game.asm,
analysis/pass-001-verification.json (ok and byte_identical true). Internal state
analysis/state.json and current map saved/validated. No ROM/toolkit edits.

12,949 code bytes, 13,191 data bytes, 32,768 image bytes. Fixed CNROM PRG context
[0,1,2,3]. Per-bank counts and exact unknown complement in round001-accounting.json;
PRG1 retains 5,017 unknown bytes and PRG2 1,518. Structural accepted graph comes
from vectors, audio inline dispatch contracts and seven-way split dispatcher.
Generic asset data classification makes no semantic-layout/unused claim.

Gameplay: natural stage1; artificial stages8/24/31/32 with initialized maps and
movement/shooting; fourth-loop ending text observed then later status page. Scripts
and exact RAM recipes in scenarios.json. First shortcut failed sprite-zero wait,
corrected by transition precondition $19=0; failed artifacts preserved.

Next commentary reviews entire current generated source. Then redirect should
follow R001-entity-dispatch and R001-small-routines in leads.json; do not repeat
completed broad sightseeing without a new handler/selector question. No terminal
exhaustion assessment. No previously failed structural lead permanently rejected.

Evidence: round001-final-static-candidates.json, round001-split-dispatch-decode.json,
round001-runtime-connected-candidates.json, round001-stages-fixed-coverage.json,
round001-rpc.jsonl. Runtime conflicts: zero. Opcode starts observed lower bound
3,498; classification and reachability remain separate.

Toolkit: NES-001 confirmed silent input-schema mismatch, workaround full buttons
array; no blocker. See toolkit-issues.md. Annotation evidence complete at structural
level; only 4 labels, readability/editorial completion pending.
