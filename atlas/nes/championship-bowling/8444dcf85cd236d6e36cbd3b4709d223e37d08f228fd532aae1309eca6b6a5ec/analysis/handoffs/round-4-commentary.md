# Round 4 commentary handoff

- Phase: commentary, round 4, on checkpoint **pass-004** (maps/current.json = analysis/state.json
  = maps/pass-004.json, sha256 d0ea19c3...2db6137; 512 regions).
- Status: **complete. Current-source readability covers ALL classified source**: all 512 scaffold
  entries, file range $0000-$FFFF (physical 0-65551), header and CHR included. **No ranges remain.**
- Final editorial audit / archive acceptance was intentionally **not** run; editorial_audit stays pending.
- ROM classification is unchanged and reported separately: 100.00% (65552/65552, 0 unknown), structural only.

## Baseline change handling

1. The map changed (pass-003 -> pass-004: 11 code regions Static -> Observed; region 379 split so
   $A84E-$A850 `jmp` is new Static region 512; L-001/L-002 evidence text updated).
2. The whole round-3 commentary directory moved unchanged to **analysis/round-3/commentary/** (source,
   review, frozen baseline, notes, reading copies). It is the only copy; round-1/round-2 copies untouched.
3. `commentary.py prepare` made a fresh frozen baseline (before/ = generated pass-004 source), byte-identical.
4. Carry by physical ranges (analysis/tools/commentary-round4/carry.py, round-3 method): round-3 game.asm
   copied, 12 markers rewritten (provenance / region 379 end), 1 marker inserted at $A84E; no .incbin
   split needed; marker list equals before/game.asm; verify byte-identical right after the carry.
5. review.json regenerated with `cm.py review`: 497 entries keep their round-3 text, 13 revised (reworded
   headers), 2 new (region 379 piece; region 512 with a hand-written summary). 512/512 reviewed, 988 units.

## Edits this round (analysis/tools/commentary-round4/batches/, applied by replace.py)

- **L-018 (resolved):**
  - RNG: RandomSeed "zeroed only by Reset"; Random documents the four call sites and that every result
    follows from the call history since power-on; AdjustHookForLane notes its two draws advance the seed;
    StrikeCelebration gives the Random(10) split (double: 0-5 TilesB, 6-10 TilesA; turkey: 0-3 Walk,
    4-10 Pair) and that both tile variants ran in natural play.
  - Tiles: new constants TILE_MISS $42 "-", TILE_GUTTER $43 "G", TILE_SPARE $44, TILE_STRIKE $45; the 12
    tile operands in WriteTenthFrameMarks/WriteFrameMarks use them.
  - L-001/L-002 wording: the header explains the vector-rooted closure / computed-transfer census once;
    the 23 runs (KnockDeckPinSideways, SetBowlerColors, DebugActorFrameViewer, DrawBlock9FFB/A045,
    SetFrameAreaAttrs, FinalAwardsTest, DrawRawScreenNt1, SndCmdTransposeNote/AddAttenuation, math and
    PPU library, CheckPause/WaitSeconds, DelayLoop/ReadVramByte/ActivePadDirection, WaitButtonA/B, fades,
    SetSpritesX/MoveSprites, tile uploaders) now say "unreachable from the vectors (L-001/L-002)", still code.
    GameStart's pause flag notes there is no pause in play. Gated NMI jobs (AttrRowPending, VramReadMode,
    FillPending) and UnusedBestScoreData say "no reachable code" instead of "discovered code".
- 10th frame (E15): ScoreBall's third-ball encoding resolved ($B0+n = spare after "X n"; "X 9 /" = $A9 $B1);
  ScoreFrames 10th-frame sum note; region 512 `jmp @countBall` explained as a defensive path that an open
  10th never reaches.
- Slow roll (E16): ReleaseBall uses the sound names and notes the slow roll was heard at power 6.

## Readability assessment

- One source file, commentary/source/game.asm (11.4k lines, 802 global labels).
- No auto/address-only labels apart from deliberate unknown-role labels (UnrefTilePairs_CDE7,
  UnrefSound*_xxxx, Stream_xxxx); 1 intentional raw RAM operand (Reset RAM clear); no raw ROM addresses.
- No remaining "uncalled" / "no known caller" / "discovered code" wording (checked by grep).
- Rest of the source carried unchanged from round 3, whose full-source assessment still applies: the
  pass-004 changes are provenance only, and their dependencies were rechecked (all 12 affected markers,
  ScoreBall/ScoreFrames/WriteTenthFrameMarks, ReleaseBall, StrikeCelebration/CelebrationTilesA).

## Verification

- `commentary.py verify`: ok=true, **byte_identical=true** after the carry, after each batch and fresh at
  handoff (commentary/verification.json, source sha256 4f703ac0...57d8).
- Reading copy commentary/reading-after.asm refreshed with `commentary.py read`.

## Unresolved meanings

See commentary/uncertainties.md: character 0 handedness; "PND" units; CONTROL label; celebration and
selection-object naming; portrait pose B; sign number tiles; UnrefTilePairs_CDE7, ScorecardData's first
16 bytes and UnusedBestScoreData; ClearScores `,y/,x` slip; one ignored 4-frame B press; sound id meanings
(SndCmdTransposeNote/AddAttenuation tentatively older $F5/$F6 handlers); purposes of unreachable routines.

## Structural questions / leads

- No new structural question; commentary proposes no map change.
- L-018 resolved (leads.json, round-4-commentary pass entry). L-010 stays open/low (sprite record
  splitting, formatting for the final editorial audit). No new lead IDs.

## Housekeeping

- Markdown created this phase: this handoff, plus the moved round-3 copy under analysis/round-3/commentary/.
  commentary/*.md come from prepare (progress.md and uncertainties.md rewritten).
- Tools: analysis/tools/commentary-round4/ (cm.py copy, carry.py, replace.py, batches/). No scratch left.
- No server started; no toolkit change; no new toolkit issue.
