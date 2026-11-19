# Round 4: commentary phase handoff

Game: Baseball Stars (SNK 1989), MMC1, PRG 128 KiB, CHR 128 KiB.
Status: the current-source readability work is **complete** for every classified byte in
pass-006. Whole-ROM classification is a separate requirement and is unchanged at 262159 / 262160
bytes (1 Unknown byte, b10 $9B08). editorial_audit stays pending; the audit was not run.

## Scope and checkpoint
- Baseline: `maps/pass-006.json` (sha256 ac37c43f...), which equals `maps/current.json` and
  `analysis/state.json`. The map had changed since round 3, so the whole round-3 commentary
  directory was first copied to `analysis/round-3/commentary/` (checked identical with `diff -r`).
  `analysis/round-1/` and `analysis/round-2/` were not touched. Then `commentary.py prepare` built
  a fresh baseline with 1225 scaffold regions.
- Carry-over: annotations, tools, notes, progress.md and uncertainties.md were copied back. All
  names landed by bank:CPU address except FieldUpdateThrowEntry ($A07B), which now falls inside
  an instruction. The first rebuild was byte-identical.
- Records invalidated (only these): the 3 pass-005 Unknown notes, and the operand/comment on the
  old $A07B instruction. The new records are in `commentary/annotations/r4-commentary.ann`.

## Changed regions
| range | source | content |
|---|---|---|
| b13 $A07A-$A07C | bank13.asm `CpuThrow_ThrowIfBaseNear` | The `bcc CpuThrow_Jump` / `rts` tail of CpuThrow_TryBase's distance test. The head explains the second reading: FieldCommonUpdate calls $A07B every frame, where the bytes decode as `ora ($60,x)` and then `jmp ThrowToBase` (TK-003). `FieldUpdateThrowEntry = * + 1` is emitted before the BCC, so the bank-14 call reads `jsr a:FieldUpdateThrowEntry` and stays byte-identical. |
| b12 $9026-$9028 | bank12.asm `RunnerInput_ReturnOldTail` | `jmp RunnerMoveAlongPath`. It has no known caller, which does not make it dead. It is read as a superseded tail of RunnerInput_Return (state 0 / body pairing). |
| b10 $9B08 | bank10.asm note | Remains Unknown. The note was updated for pass-006 (lone RTS, L-R3-01 deferred). |

Local tool change (commentary/tools only, not the toolkit): build.py turns an annotated name that
falls inside a classified instruction into a `Name = * + n` assignment and resolves references to
it. check.py accepts that form.

## Round-3 open item: menu code reusing zero-page ball names (done)
Every Ball* operand outside field-play code now has a scoped scratch name. The ones in bank 1's CPU
batting/pitching AI are left alone because they use those variables in their field role:
- MenuCursor and MenuCursor2 ($BB/$BC) in banks 7 and 11.
- NameCharPos, NameGridChar and NameScrollStep (bank 10).
- PointSum, PointMax, PointRoom, AbilityRow and NameRowCount (bank 7).
- LeaguePickCount, LeaguePickCpu, ChampRecord, ChampPause, NumGlyph, NumColumn, LeaderPage and
  LeaderBest (bank 11).
- LogoFadeStep, LogoFadeTimer and TitlePageCount (bank 1).
- LineupPalPtr, LineupRow, SwapPick1, SwapPick2, IconSlot, MatchupSide, MatchupOfsHi and
  ResultTilePtr (banks 2 and 3).
- MsgCharIndex, MsgPadCount, PitchSaveSide and PostGameTimer (bank 9).

The MenuBuf scope now also covers bank 2 $8D17-$8FD3. The heads that named the old operands were
rewritten. Per-object arrays reused as menu variables (ObjState.., ObjTimer, TeamScore, ...) still
use their field names, as documented under "RAM reuse" in `commentary/uncertainties.md`.

## Verification
- `commentary.py verify`: ok=true, byte_identical=true (commentary/verification.json, fresh run).
- `commentary.py read`: reading-after.asm refreshed (43224 lines). input/game.nes is unchanged
  (sha256 a71e4cc8...).

## Readability counts
- 0 auto sub_/loc_/dat_ labels and 0 address-only locals.
- 0 raw RAM operands (rawram.py), and 0 Ball* operands outside field-play code.
- regioncheck: 0 unexplained Unknown or Data ranges.
- check.py: all 3414 names and every comment landed.
- nohead: the same 5 fall-through entries as before have no head of their own; each is explained
  by its parent's comment.
- review.json: 1225/1225 regions reviewed, 1405 units; editorial_audit pending.

## Unresolved (in source and in commentary/uncertainties.md)
- b10 $9B08 is still Unknown. It may be a leftover RTS or padding; it needs a reference, consumer
  or execution to decide.
- $A07B overlap: it is unknown whether the overlap is intentional and what the ORA does to A when
  FieldCommonUpdate enters there. The check would be a bounded trace of one field play.
- The fragments with no known caller (round 3, plus RunnerInput_ReturnOldTail) have unknown
  reachability.

## Remaining gaps
None in the readability backlog.
