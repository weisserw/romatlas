# Final editorial audit (pass-004)

## Scope
- Input: the round-4 commentary on checkpoint pass-004. maps/current.json and analysis/state.json
  are unchanged and equal to pass-004 (100.00% classified, 0 unknown bytes).
- Review credit is carried over from round 4: 1130/1130 regions and 642 units. The source was
  **not** reread in full.
- The audit consisted of:
  - applying lead L-013;
  - running mechanical whole-source checks;
  - a targeted reread only where a check, or L-013, pointed at a concrete inconsistency. In
    practice that meant the routines around each hit, and the bank-3 tournament-opponent
    cluster those headers led to.
- All edits went into commentary/tools/annotations.txt (the edit record) and were rendered with
  build.py, then review.py. before/, the map, input/game.nes and the toolkit were not touched.

## L-013 applied
| Item | Change |
|---|---|
| SpinPanelRect / SpinPanelAttrs | Traced extent $9043-$909A, with exactly 6 attribute bytes. The 32-byte uniform run $909B-$90BA is noted as unread, purpose unknown. |
| BallMarkerSprite | Renamed **CalledBallMarkerSet**: a one-entry set pointer, with bank 4 runtime-confirmed. Its record is labelled **CalledBallMarkerSprite** ($B5D8, piece count 4, 13 bytes). DrawBallMarker's header is exact. |
| ChallengeResult | The Speed Pool total is elapsed seconds (60-frame clock, TIME heading, unlock below 141). Only THREE BALL is inverted, so a slower clear ranks higher. Recorded as a likely original bug whose intent cannot be proven. |

## Mechanical checks (whole source, 21,478 instructions, 706 regular labels)
| Check | Result |
|---|---|
| Address-only labels (sub_/loc_/tbl_/...) | 0 |
| Address-only RAM names (ram_/zp_) | 0 |
| Address-suffixed locals | 0 |
| @skipN / @join / @if_* labels | 0 |
| TODO / FIXME / XXX / UNKNOWN / "??" | 0 |
| Wording about unclassified, unknown or gap bytes | 0 |
| Regular code labels without a routine header | 4 found. Three are now headed (MatchOverCheck, DelayReturn, ResolvePointContact). EndShotSameTurn's stray block comment was merged into its header. |
| Comment identifiers that are not defined symbols | 19 names. 6 were stale and fixed: CollisionResponse, ControllerSelect, NmiPlayPending, SetTargetScore20/8, ShotStat. The other 13 are intentional: historical "formerly X" notes, prefix wildcards such as EndShot\* and MatchSetting\*, and terms like KiB and UxROM. |
| Immediate pointer pairs (ld?/adc lo then hi, high byte $80 or above) that name a label | 9 hits. 2 were real bases and are now symbolic: $9300 became PreviewFrameRects and $B7E1 became ContactSqrtTable. The other 7 are numeric constants ($FF00, $F000, $FFFF, $FF10 and so on). The BreakLayouts10/16 bases (built with adc after Multiply8) were found while rereading the cluster headers, and are also symbolic now. |
| Raw absolute operands | 0. Only symbol+offset forms remain, such as PalBuf+$1D and BallFlags+$20. |
| Flag compares on BallState/BallFlags/PlayerInputMode/ScreenFlags/SysFlags with a raw value that has a constant | 2 (TableControlOff/On). Both now use BALL_VISIBLE. |
| RAM symbols defined but never used | ShotStat0590 was stale (no access) and is removed. SYS_BOOT and OAM_HIDDEN_Y are documented constants and are kept. |
| Duplicate stacked comments (header followed by a block comment, or repeated lines) | 0 after the EndShotSameTurn merge. |

## Issues fixed (concrete inconsistencies)
1. **EmitMetasprite header** gave the piece order as (dy, dx, tile). The code stores the first
   byte to OAM X, so the order is (dx, dy, tile). The header also now covers flip negation and
   the zero count. The locals were renamed @has_pieces, @dx_done and @dy_done.
2. **PendingSfxFlag/PendingSfxArg → RowCopyPending/RowCopyRow.**
   - Their only uses are the zoomed-map row-copy request (ScrollCameraTo → NMI → NmiCopyTableRow).
   - The RAM comment named a nonexistent NmiPlayPending.
3. **Ball slot $10 is the phantom (aim) ball, not the cue ball.** Ball 0 is the cue ball. The
   following were corrected:
   - DrawBallSprite's header;
   - DrawBallShadowSprite, renamed **DrawPhantomSprite** with an exact header;
   - TableControlOff/On, which hide and show the phantom via BALL_VISIBLE and no longer say
     "meaning unknown".
4. **Tournament computer-opponent cluster (bank 3 $9D84-$A193).** The headers contradicted the
   code ("starts a new rack", "Per-GameType setup", "sets ball X's position", "challenge
   layout" in a tournament-only path, "re-seeds" for an LFSR step). Renamed and re-headed:

   | Old name | New name |
   |---|---|
   | StartNewRack | SimulateOpponentTurn (also Far_) |
   | SetupGameType | SimulateOpponentShots |
   | SetupGameTypeTail | SimulateOpponentShots8Ball |
   | RackForChallenge | LoadBreakLayout |
   | LoadChallengeLayout | LoadBreakLayout8Ball |
   | ChallengeLayoutFlags | AssignOpponentGroup |
   | PlaceBallAt | ClearBallPositions |
   | FindFirstBall | CountGroupsOnTable |
   | SetupRotationRules | PocketOpponentGroupBall |
   | RemoveBallsFrom | PickRandomBall |
   | CountRemovable | AnyObjectBallActive |
   | SeedRandom | RandomBank3 |
   | GameSetupTables/B/C/D | OppSinkCounts8Ball / OppSinkCounts / OppRunOutLimit8Ball / OppRunOutLimit |
   | SetupBallCount / SetupRemovedCount / SetupVariant | OppSinkCount / OppSunkTotal / OppGroup |

   The following were also rewritten:
   - the BeginMatchTurns and RemoveBallIfActive headers;
   - the EndShot\* finisher headers;
   - the GameOptions bit2 (open table) and bit3 (group assignment) comment.
5. **New data labels from consumer evidence** (commentary labels only; region kinds and bounds
   are unchanged):
   - **BreakLayouts10** (bank 6 $8000, 80 x 25 bytes) and **BreakLayouts16** ($87D0, 80 x 40
     bytes) are post-break ball layouts decoded by LoadRackPositions. All 160 records decode to
     table-range coordinates.
   - **ContactSqrtTable** (bank 4 $B7E1) is the word table of LookupBank4Word. The extent is
     not established.
   - **PreviewFrameRects** (bank 3 $9300) holds six 4 x 4 DrawTileRect frames, and
     **ModeMenuSpriteTile** ($92FF) is one byte. These replace the inaccurate "CoinTossData"
     header.
6. Further stale headers were corrected:
   - ResolveBallCollision: re-predicts contacts and no longer claims "CollisionResponse".
   - The InputHeld and TargetScore RAM comments.
   - LookupBank4Word, DrawPreviewFrame and ClassifyShot (which now names
     PocketedThisShot/LowestBallAtStart).

## Remaining accepted uncertainties
These are also listed in commentary/uncertainties.md.
- **Meaning unknown:**
  - the $909B-$90BA uniform run;
  - the 9 bytes after PreviewFrameRects;
  - the extent of ContactSqrtTable and the rest of that bank-4 block;
  - the role of OppSunkTotal, which has no reader.
- **Intent unprovable:** the Speed Pool ranking.
- **GameCmd values** other than $12 stay numeric with comments. Physics constants, rule flag
  bits and generic data blocks keep their reviewed boundary/role notes.
- **r2-q1 low-confidence code** (L-006 fragments, the one-byte prologues): unchanged and
  explained in source.

## Verification
| Step | Result |
|---|---|
| build.py | 706 regular labels, all named, 0 unresolved |
| review.py | 651 discovered units (1514 unit records across scaffold entries), 0 pending; 1130/1130 regions reviewed |
| `commentary.py verify` | ok, byte_identical true |
| `commentary.py audit` | ok; byte_identical true; reviewed_regions 1130; reviewed_units 1514; editorial_audit true (commentary/audit.json) |
| `commentary.py read` | Refreshed commentary/reading-after.asm |
