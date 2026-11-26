# Final acceptance: commentary editorial audit

## Status
**Accepted.** The final editorial audit is complete, and `commentary.py audit` passes with a
byte-identical rebuild.

| Item | State |
|---|---|
| ROM classification | 100.00% (131088 / 131088 bytes; 0 unknown) at checkpoint pass-004 |
| maps/current.json, analysis/state.json | Unchanged; cmp-equal to maps/pass-004.json, state sha256 35ed6454... |
| Not modified | input/game.nes, the toolkit, before/, the round-1..3 commentary copies |
| Not run | archive.py, cleanup |

## L-013 applied (lead now `verified` in analysis/leads.json, with a history entry)
1. **SpinPanelRect / SpinPanelAttrs** (bank 3).
   - A natural trace read exactly $9043-$909A: the 82-byte layout plus 6 attribute bytes.
   - The 32-byte uniform run $909B-$90BA is never read. Its purpose is unknown, and the source
     says so.
2. **BallMarkerSprite → CalledBallMarkerSet** (bank 4 $B209).
   - It is a one-entry pointer to the sprite record **CalledBallMarkerSprite** ($B5D8).
   - Bank 4 is confirmed via BuildSprites and the runtime trace.
   - The rest of the block is other bank-4 data: metasprite records, plus ContactSqrtTable (see
     below).
3. **ChallengeResult.**
   - The Speed Pool total is elapsed seconds: a 60-frame clock, the TIME heading, and an unlock
     that needs a time below 141.
   - Only THREE BALL is inverted, so a slower Speed Pool clear ranks higher.
   - This is documented as a likely original bug; the intent cannot be proven.

## Audit checks and fixes
The full table of checks and counts is in commentary/editorial-audit.md. Round-4 review credit
was reused, and the source was reread only where a check found a concrete inconsistency.

**Mechanical checks over the whole source** (21,478 instructions, 706 regular labels):

| Check | Result |
|---|---|
| Address-only labels or RAM names | 0 |
| TODO / UNKNOWN / unclassified-range wording | 0 |
| Address-suffixed or @skip/@join locals | 0 |
| Stale comment identifiers | 6, all fixed |
| Raw pointer bases | 4 bases made symbolic |
| Raw flag values that have a constant | 2, now BALL_VISIBLE |
| Code entries without a header | 3, now headed |
| Stacked duplicate comment | 1, merged |
| Unused stale RAM symbol | 1 (ShotStat0590), removed |

**Concrete inconsistencies fixed:**
- **EmitMetasprite piece order:** it is (dx, dy, tile), not (dy, dx, tile).
- **Row-copy request:** PendingSfxFlag/Arg are now RowCopyPending/RowCopyRow. The sound naming
  was wrong; their only use is the zoomed-map row copy.
- **Ball slot $10 is the phantom (aim) ball, not the cue ball:**
  - DrawBallShadowSprite is now **DrawPhantomSprite**.
  - TableControlOff/On hide and show the phantom (BALL_VISIBLE).
- **Bank-3 tournament computer-opponent cluster:** the old headers contradicted the code.
  - Routines renamed: StartNewRack → **SimulateOpponentTurn**, SetupGameType(Tail) →
    **SimulateOpponentShots(8Ball)**, RackForChallenge / LoadChallengeLayout →
    **LoadBreakLayout(8Ball)**. Also AssignOpponentGroup, PocketOpponentGroupBall,
    ClearBallPositions, CountGroupsOnTable, PickRandomBall, AnyObjectBallActive and
    RandomBank3.
  - Tables renamed: OppSinkCounts(8Ball) and OppRunOutLimit(8Ball).
  - RAM renamed: OppSinkCount, OppSunkTotal and OppGroup.
  - Headers and comments rewritten: BeginMatchTurns, the EndShot* finishers and the GameOptions
    bit2/bit3 comment.
- **New consumer-backed data labels.** These are commentary-only splits of incbin lines; no
  region kind or boundary changed.

  | Label | Location | Contents |
  |---|---|---|
  | BreakLayouts10 | bank 6 $8000 | 80 records of 25 bytes |
  | BreakLayouts16 | bank 6 $87D0 | 80 records of 40 bytes; packed post-break ball layouts; all 160 records decode to table coordinates |
  | ContactSqrtTable | bank 4 $B7E1 | LookupBank4Word's sqrt table; extent not established |
  | PreviewFrameRects | bank 3 $9300 | six 4x4 frames |
  | ModeMenuSpriteTile | bank 3 $92FF | one byte |

## audit.json result
| Field | Value |
|---|---|
| ok | true |
| byte_identical | **true** |
| reviewed_regions | **1130** (of 1130) |
| reviewed_units | **1514** unit records; these are 651 discovered units, some split across scaffold entries |
| editorial_audit | true |
| review.json editorial_audit | `reviewed`, with notes |

`commentary.py verify` returned ok and byte_identical true. `commentary.py read` refreshed
commentary/reading-after.asm.

## Remaining accepted uncertainties
These are in commentary/uncertainties.md under the "Final acceptance" section.
- Purpose unknown for the $909B-$90BA uniform run and for the 9 bytes after PreviewFrameRects.
- The extent of ContactSqrtTable, and the rest of that bank-4 block, which stays one generic
  block.
- OppSunkTotal has no reader. The carry result of the forced-rebreak path was not traced.
- The intent behind the Speed Pool ranking cannot be proven.
- GameCmd values other than $12 stay numeric, with comments.
- The r2-q1 low-confidence code fragments are unchanged and explained in the source.

## Final file list (changed or created in this step)
- commentary/tools/annotations.txt: the edit record.
- commentary/source/*.asm and ram.inc: rendered by build.py.
- commentary/tools/labels.json, linemap.json and .last_build.json: regenerated by build.py.
- commentary/review.json: units refreshed, and editorial_audit set to reviewed.
- commentary/editorial-audit.md (new), progress.md, uncertainties.md.
- commentary/verification.json, audit.json and reading-after.asm: written by the toolkit.
- analysis/leads.json: L-013 is now verified.
- analysis/handoffs/final-acceptance.md: this file.

Disk: about 2.5 GB free. The only scratch was /tmp/fa-2707 (edit scripts and pre-edit backups),
which was removed at handoff. No emulator or server processes were started.
