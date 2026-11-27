# Round 001 disassembly handoff

Status: phase completed, ROM incomplete. ROM classification: 77.60% (101725 / 131088 bytes classified; 29363 bytes unknown).

Durable map: analysis/state.json and maps/current.json. Verified checkpoint:
analysis/round-001-verification.json; maps/round-001.json; generated/round-001/.
Input hash unchanged. Exact counts/per-bank breakdown: round-001/counts.json.

Accepted batch: 16-byte iNES header; hardware vector words; bank-aware observed
instruction fetches and bounded static successors; seven full graphics banks
(1–7) visually reviewed as coherent 2bpp tile sheets; banks8–11 repetitive
non-instruction records as generic Data; exact uniform runs as generic Data.
Evidence: round-001/review-decode.jsonl, *-expanded.json, *-contexts.json,
tiles-*.ppm.png, record-bank-inspection.json, uniform-runs.json and rpc.jsonl.
Structural claims and natural/artificial reachability remain separate.

Gameplay: Commando NES, verified natural helicopter, landing, combat and scrolling.
Successful replay: restore againstart.snapshot, inputs=[]180f, Up+B300f,
Up+A+B300f; trace windows2f after each milestone. title at play-0.snapshot;
correct input uses buttons arrays (singular button/pressed silently ignored).
Later/final mission attempts failed: pre-start $049D/E patches overwritten,
active $0060=$08 plus index edits corrupted terrain. No later-area visit or
ending claimed. Preserve all failed artifacts; do not repeat unchanged tests.

Highest-priority redirect: find exact natural stage initialization/continue contract
for $049D mission0..15 and $049E area0..3. At physical1CC63 increments/masks
prove domains. Physical1B1EC/1B1EF clear indices, but CPU B1E2 breakpoint after
these clears did not match during bounded newgame attempt. Determine actual
calling route/banks before reusing it. Built-in controller2 area-select partly
works (select-3 screenshot1-2) but timing/state transfer remains unresolved.
Continue until a coherent batch is ready; this handoff does not declare terminal
exhaustion. Other concrete leads: fixed-bank dispatchers/mapper call contracts
and bank0 music/table boundaries. Full current-source commentary remains.

Toolkit: NES-001 parameter documentation mismatch (one confirmed documentation
bug/missing schema capability), nonblocking; see toolkit-issues.md.
