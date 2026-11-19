# Round 4 — disassembly phase handoff (pass-006)

Status: INCOMPLETE (1 Unknown byte). One pass; both ready leads from r3-redirect-01 reviewed and accepted.

## Checkpoint and verification
- Input maps/pass-005.json (407f32c4...). Output maps/pass-006.json
  (sha256 ac37c43fff6441132a9919788f6769764a5c76829235748e4edc86cbc982f073), generated/pass-006,
  analysis/pass-006-verification.json: ok=true, byte_identical=true. state.json == current.json == pass-006.
- ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown) — exact 99.9996%, NOT complete.
- Per-bank table in analysis/progress.md (pass-006 entry). Code 62,293 (Observed 10,804), Data 68,794, Image 131,072.

## Lead outcomes
- L-R2-04 (b13 $A07A, 1 B): ACCEPTED. Region 3514 now phys $1A08A len 62, view $A07A [12,13,14,15], Static
  (`bcc $A07D` / `rts` / `jmp ThrowToBase`). $A07A is the fall-through of classified `sbc #$01` at $A078. The observed
  alternate start $A07B (`ora ($60,x)`, FieldCommonUpdate `jsr $A07B`) is not a map instruction boundary; it is
  documented in region evidence. Generated source keeps numeric `jsr a:$a07b`; byte_identical. No toolkit metric
  flags hidden starts (analysis.conflicts unsupported on NES), noted under TK-003.
- L-R5-01 (b12 $9026-$9028, 3 B): ACCEPTED. New Code region 3988 `jmp RunnerMoveAlongPath`, Static, not reachable
  as written (no reachability claim). Basis: exact bounded gap, operand = verified entry, ObjState 0/body pairing.
- L-R3-01 (b10 $9B08, 1 B, $60): deferred, unchanged. Rejection review in analysis/handoffs/round3-redirect.md.

## Next
- Commentary: bank13.asm, region 3514 now starts at $A07A. Emit `bcc` + `FieldUpdateThrowEntry = * - 1` (explain the
  overlapping `ora ($60,x)` entry) and make the FieldCommonUpdate `jsr $A07B` use it if the generator keeps the bytes.
  bank12.asm $9026, label the superseded `jmp RunnerMoveAlongPath` tail (unreachable as written). Remove the old
  Unknown notes for these 4 bytes.
- Redirect: only $9B08 remains. Nothing actionable without a reference, consumer or execution.
- Edits: scratch/r4/{build.py,batch.json,apply.py}. Details: analysis/investigation.md (Round 4 section).
