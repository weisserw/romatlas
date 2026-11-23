# Round 1 redirect handoff

STATUS: COMPLETE — ready leads for round-2 disassembly. Attempt r1-a7c2.
Input: checkpoint pass-001 (map sha256 54a37984...) + commentary round 1 (verify ok,
byte_identical). No changes to ROM, map, state.json, or commentary source.
Records: analysis/redirect/r1-a7c2/unknown-ledger.md (all 19 ranges / 814 B, each with a disposition),
analysis/redirect/r1-a7c2/evidence.json (claims, bytes, competing readings, residual reasons),
analysis/leads.json (pass entry + L-007..L-014), tools/dis.py (linear decoder).

## Ready leads (proposed 811 B of 814 unknown; check the expected bytes before annotating)
| Lead | Physical ranges | Role | Bytes | Key evidence |
|---|---|---|---|---|
| L-007 | 0BD0E-0BD1E, 1684B-1685B | data: ApplyMotionScript records (4x4 + $FF) | 34 | Tmp5D set to $BCFE/$A83B right before the call; terminator ends at the next code |
| L-008 | 12100-12102 `11 f0 00`, 13843-13845 `11 f5 00`, 12C7D-12C7F `02 02 ff` | data: 1x1 metasprites / tile-rect-fill record | 9 | SrcPtr loads -> DrawMetasprite / QueueTileRectFill; same formats as classified $B490, $B41A |
| L-009 | 120A7-120AA `a9 00 85 58` | code: blink-counter reset prefix that falls into WaitButtonBlinking | 4 | $58 is the counter DrawBlinkingArrow uses; no caller (lower confidence) |
| L-010 | 1C2BB-1C2C3, 1E948-1E95D, 1F21D-1F23F, 1F266-1F274, 1F5F6-1F609, 1F63B-1F644, 1FFEE-20005; data 20006-20009 | code: variants of the adjacent live routines (InitMirroring, ClearScreenAndShow, FillAttributeBuffers, RequestAttrUploads, EnableWram...) + 4 B data (copies of the NMI vector word) | 135+4 | same contracts and callees; jumps land on instruction boundaries in live code |
| L-011 | 1EA83-1EAC5, 1EB66-1EBB6 | code: twins of DrawInlineScreen/DecodeRleStream for nametable $2400 and AttrBuffer1 | 148 | byte-for-byte twins except for the parameter bytes; re-enters live $EB47 |
| L-012 | data 1E8BB-1E8BF, code 1E8C0-1E8CB, data 1E8CC-1E8CF | stale duplicate split by source role (replaces L-003) | 21 | byte-identical to $E89A-$E8AA, then $FF fill before Reset |
| L-013 | code 1F421-1F5D4, data 1F5D5-1F5E8 | VRAM-queue/attribute helper library + OR/AND mask tables | 436+20 | internal call graph; tables read at $F5B9/$F5A4 end exactly at $F5D9 |
| L-014 | 1A034-1A035 `05 03` (currently code) | reclassify as data: StageSelect limits | 0 net | the preceding branch is always taken; read by `cmp $A024,y` with Y only 0 or 1; $A026 is a dispatcher entry |

Structure and reachability are separate claims. No READY code has a known caller.
Searches for an entry word, entry-1 (RTS-dispatch), or a computed BankCall8000 selector found nothing.
None of these is claimed unused. Reviewers can reject L-009 or the 12-byte code part of L-012
on their own; the other leads do not depend on them.

## Residual groups (3 B) — compact rejection review
- $C43F-$C440 (phys 1C44F-1C450, `01 00`, PRG14). Claim tested: data or code. Evidence:
  follows an RTS. No absolute/indexed operand references $C420-$C440 and no word references it.
  As code, ORA ($00,X) would overwrite A, which is the input of the next routine
  PutMetatileInColumn, so a code-prefix reading is implausible. Strongest competitor: 2 bytes
  of generic data. Reason it stays unknown: there is no consumer, no format, and no
  uniform or visibly non-instruction pattern ($01/$00 are official encodings). Any role
  would be invented. Revisit if a consumer of $C43F appears.
- $C587 (phys 1C597, `60`). A lone RTS between two live routines with no reference.
  evidence-policy.md states that an unreferenced return opcode lacks sufficient structure,
  and one byte gives no pattern. Remains unknown.

## Gameplay (analysis/scenarios.json)
- Bonus game: visited (L-004 verified). $042A=48 and $043F=8 entered the panel game, which
  ran 799 code addresses never observed before (none in unknown ranges).
- Game over: visited. $0429=0 and $0428=0 led to the "GAME OVER / WHAT A PITY" screen
  (119 newly observed addresses).
- Bosses: deferred (L-005). Spawn records and object slots are in PRG-RAM, which the
  debugger cannot write (new toolkit issue TK-006). Teleporting the player/camera broke the scene.
  Next recipe: decode the spawn-record coordinates (SpawnColumnPassed/SpawnRowInMap; stage 6
  round 4 record 1 = type $12 `70 06 a0 00`), or play a round 4 normally with $0428 health
  refills. No unknown-byte gain is expected, because the boss code is already classified statically.
- Hidden stage select (pair 6 entry 8): no static caller. All 35 BankCall8000 selectors are
  immediates and none is $68. Not exercised artificially because it is low value.

## Deferred avenues
- L-006 (semantics of the generic data banks): readability only, no classification gain;
  left for commentary.
- Further reachability of the fixed-bank helper twins: the reference searches are
  exhausted, and every observed run (title, stages, bonus, game over, ending) shows no fetch.

## Next step
A round-2 disassembly is warranted. Validate and annotate L-007..L-014 (up to 811 B; expected
classification 261346+811 = 262157/262160 if all are accepted), run a checkpoint, then commentary
for the new regions, then redirect. Scratch was cleaned up (about 3.4 GB free).
