# Commentary progress

## Round 1 commentary pass (2026-11-18)

Baseline: maps/round1-disassembly-b.json (map sha256 2b95d9dd...ccac), prepared with
`commentary.py prepare`; before/ frozen, source/ maintained by hand-written specs.

Scope covered: all currently classified source (1,727 classified regions) plus notes on all
239 unknown ranges; review.json: 1,966/1,966 scaffold entries reviewed, 847 units.

Method (tools in commentary/tools/, edit batches in commentary/specs/ 01-27):
- transform.py (one-time): branch/jump/data operands -> labels (byte-identical), hardware and
  MMC1 register names, proven pointer tables rendered as `.word` label lists (93 lines),
  phase-1 evidence notes on data/padding regions, NOTE placeholders for unknown ranges.
- edit.py: RAM symbols, label renames, doc blocks, end-of-line notes; fixlocals.py keeps
  cheap-local labels legal; autonote.py annotates every FarCallBank call with its target.
- Manual reading: all of bank 0, bank 15 $C000-$E000, bank 1, front-end entry points;
  other banks named from per-routine reference summaries (summ.py) plus targeted reads.
- autodoc.py: remaining routines received mechanical reference summaries, marked
  "Role inferred from the name; tentative." where only the name carries the interpretation.

Key names/interpretations introduced: MainLoop, NMI pipeline (NmiVramJob jobs, VramBuffer
packets, FlushVramBuffer), FarCallBank/entry tables, ball physics (BallPos*/BallVel*, Gravity,
PredictBallLanding, StepBallPhysics), fielding AI (FielderSlots, AssignNearestFielder,
CoverBase*/BackupBase*), PlayState machine (PlayState_NN), counts (StrikeCount/BallCount/
OutCount; note phase-1 handoff called $0354 a ball count - it is strikes), umpire voice
(VoiceSample/VoiceDelay, previously thought a boot flag), password (PasswordEncode/Decode,
base 25), CPU virtual controller (bank 11), sound driver (bank 12), field scroller (banks 8/9).

Verification: commentary.py verify -> ok, byte_identical true (after every batch and at
handoff); commentary.py read refreshed reading-after.asm.

Deferred / weaker areas: see uncertainties.md and analysis/handoffs/round1-commentary.md.

Status: INTERRUPTED (coverage complete, depth incomplete).  Resume this commentary phase on the
remaining depth ranges listed in analysis/handoffs/round1-commentary.md before redirect.

## Round 1 commentary resume (depth pass, 2026-11-18)

Same baseline (maps/round1-disassembly-b.json, unchanged).  Specs 28-48 in commentary/specs/;
new tool commentary/tools/branchnames.py (role-based names for address-only branch labels);
edit.py: scoped renames for cheap locals, scope-change guard, cached index.

- All 1,547 address-only labels (@l_xxxx/@loop_xxxx and Owner_XXXX entry labels) renamed
  (structural role names); role names written by hand in every routine read in this pass.
- Read in full and commented: bank 2 $C696-$FF44 (title, set-up menus, play-day entry,
  lineups, starting pitcher, line score, BCD helpers), bank 4 $C0C3-$E01F (password,
  cutscene scripts, path figures, scenes, season-day advance, biorhythm, season screens,
  ending), bank 6 $C045-$CC80 (CPU fielding/baserunning AI, base markers), bank 11
  $C043-$D320 (batted-ball launch, CPU demo/opponent controller), bank 12 $C5A3-$CC40
  (sound driver: channel record layout named), bank 13 $C04B-$EC20 (batter control/drawing,
  at-bat end, pitcher sprites), bank 15 $E014-$FBDD (runners, fielders, throws, jump/dive,
  play judgment, scoring), banks 8/9 field scroller internals (rotated field mapping,
  staging buffers named).
- All remaining 128 "Mechanically:" summaries replaced with functional comments (0 left).
- ~110 new RAM symbols / multi-byte field offsets (raw operands 663 -> 289).
- Findings: biorhythm system (bank 4 entry 1); SetupTeamPalette -> LoadPlayerBiorhythm;
  Bank2_Return -> BinToDecimal; CpuFielding_Abs16/CompareDistance are Div16/Mul16x8;
  FielderSlots_Facing is the X fraction; ControlledFielder is the loop index (CurFielder);
  PitchScreenX was PredPosY+1; base coordinate table at $EFF0 misclassified (L-010).
- review.json regenerated: 1,966/1,966 reviewed, 869 units; editorial_audit pending.
- Verification: commentary.py verify ok, byte_identical true; reading copy refreshed.

Status: current-source readability complete for the classified source (see handoff
analysis/handoffs/round1-commentary-2.md); final editorial audit still reserved.

## Round 2 commentary pass (2026-11-19)

Baseline: maps/round2-disassembly.json (= maps/current.json, sha256 8b43ff3e...), prepared
with `commentary.py prepare` after preserving the entire round-1 commentary in
analysis/round-1/commentary/ (source, specs, tools, review, progress, uncertainties).

Carry-over (tools/merge_r2.py, promote.py, ramplus.py):
- 1,728 regions with unchanged physical range and kind (including 5 unknown bytes) keep
  their round-1 block verbatim; 480 new/changed regions were emitted from the generated
  listing with operands symbolized through the round-1 labels/RAM symbols and new
  address names; 23 proven jump tables rendered as .word lists.
- Labels round 1 defined in replaced blocks were re-defined at the same physical offsets
  (219 U_ labels renamed D_/C_ now that their bytes are classified); 4 round-1 cheap locals
  targeted by new code were promoted to Owner_role globals; cheap locals whose scope was
  split by a new global were promoted by exact branch-target binding (promote.py).
- Byte-identical after the merge (verify ok), then after every spec batch.

Readability work (specs r2-01 .. r2-11, datanote.py, r2check.py):
- All 266 new code regions are under documented routines (r2check: 0 of 266 missing):
  bank 0 NMI tasks 1/4/6/8 and static fragments; bank 2 front-end screens (lineup checks,
  relief pitcher, biorhythm, batter substitution, final score/season check, editors);
  bank 4 object-script handlers; bank 11 CPU input states $18-$79, CpuSideMode handlers,
  CpuPitcherDecision; bank 15 PlayState $26/$42-$70, mercy checks, force/tag-up handlers,
  AdvanceRunnersTwoBases; banks 5/6/8/9 fragments.
- ~330 new names (no Sub_/J_/L_ address names remain in code); new local branch labels
  role-named by branchnames.py; B02_Task0605_NN renamed FrontEnd_<role>.
- Data: 151 "Used by" notes, 81 L-014 context notes, 15 role notes; ForceHandlers,
  TagUpHandlers, CpuSideModeJumpTable and BaseCoordTable rendered/labelled.
- Stale round-1 comments fixed (dispatcher docs that called tables unclassified, unknown
  notes now L-016 notes, preamble conventions).
- review.json regenerated: 2,208/2,208 scaffold entries reviewed, 1,150 units;
  editorial_audit pending (final acceptance).

Verification: commentary.py verify -> ok, byte_identical true (final run after the last
edit); commentary.py read refreshed reading-after.asm.
Leads produced: L-017 (jump tables still generic data; table-reachable L-015 fragments),
L-018 (debug-looking code).  Status: current-source readability complete for this map.

## Round 3 commentary pass (2026-11-19)

Baseline: maps/round3-disassembly.json (= maps/current.json = analysis/state.json, sha256
bfba143a...), prepared with `commentary.py prepare` after moving the entire round-2
commentary to analysis/round-2/commentary/ (source, specs, tools, review, progress,
uncertainties, before/).  Map has 2,209 regions and 0 unknown bytes.

Carry-over (tools/merge_r3.py): every round-2 block whose physical range and kind are
unchanged is carried verbatim (2,200 blocks); the seven former L-016 unknown blocks became
Code blocks (stray returns, documented as unreachable as far as known, no new labels so
cheap-local scopes are unchanged); region 517 was split into ScreenLoaderStepTable
($C048, .word ScreenLoader_Init/Stream/Finish) and the descriptor-table remainder.
Byte-identical immediately after the merge.

Readability work (specs r3-01 .. r3-04, plus targeted text edits):
- L-017: ForceHandlers/TagUpHandlers/CpuSideModeJumpTable notes rewritten (now labelled in
  the map; generic L-013 boilerplate removed); force/tag-up handlers and CpuSide_* handlers
  say they are reached only through those tables; CpuSideMode table note corrected (code
  stores modes 0, 1, 2 and $40; other entries are filler).
- L-011: constants REC_BIO_PHYSICAL/EMOTIONAL/INTELLECTUAL ($0C-$0E) used at all 17 read
  sites (11 in play, 6 on the pitching-screen panel); docs of LaunchPitchedBall,
  PlayState_21, ReleasePitch, ComputeBattedBall, PlayState_27, DrawPitcherInfo, CpuBat_Wait,
  CpuBat_PitchReleased (new), CpuBat_NoSwing (new) and CpuPitcherDecision describe the
  biorhythm as the bound of random variation; PlayState_27 corrected (batter, not pitcher).
- L-018: debug-code notes cite the redirect's negative reference search.
- Screen loader: the six 22-entry layout descriptor tables at $C04E-$C155 named
  (Layout_PpuAddr/MetatileMap/AttrData/Palette/MetatileDefs/ChrBank) and the following
  800-byte block described as palettes/metatile definitions; loader docs rewritten.
- RAM: raw RAM operands 289 -> 18 (14 addresses left, all single-purpose stores with no
  established role).  New names include LoaderTileIndex/LoaderPpuAddr, PitchCurve,
  PitchCurveVel/Frac, PitchDriftAccum, PredVelX/Y, PredCurve*, PredBounceCount,
  PredScreenH, PitchTargetX/Y/Z, ReleaseOriginX/Y, PitcherRecordA/B, BatterRecordsA/B,
  PitcherRunsAllowed, PitcherWildness, SoundQueue, RunnerProjectCursor, TmpD,
  BankEntryTable; PaletteBuffer+n, RunnerSlots+n, ScriptObjState+n and category-count +n
  forms.  DemoFlag renamed PitcherStintMark (per-side array, only ever set to $FF; L-020).
- Preamble updated (round 3, no unknown regions).
- review.json regenerated (tools/review.py): 2,209/2,209 scaffold entries reviewed,
  1,152 routine/data units plus continuation units for every code fragment (0 code
  regions without units); source locations fixed to the audit's `game.asm:N` form.
  editorial_audit pending.

Verification: commentary.py verify -> ok, byte_identical true (final run after the last
edit); commentary.py read refreshed reading-after.asm.  commentary.py audit run
informationally: every inventory check passes; it stops only at "final editorial audit is
pending" (reserved for final acceptance).
Leads: L-011 consumed; L-018 updated; L-020 produced.  Status: current-source readability
complete for this map.

## Round 4 commentary pass (map maps/round4-disassembly.json)

Map change: bank 0 region 1985 ($919B-$91D4) split into StaminaVarPtrs, StaminaLimitPtrs
and StaminaLimitTable0/1/2 (lead L-021).  Round-3 commentary preserved in full in
analysis/round-3/commentary/; fresh baseline prepared with `commentary.py prepare`.

- Carry-over: tools/merge_r4.py replaces the one changed block and carries every other
  block verbatim by physical range (2,208 unchanged scaffold entries).  Old names were
  mapped: D_B00_919B/919C -> StaminaVarPtrs(+1), D_B00_919F/91A0 -> StaminaLimitPtrs(+1).
  The new blocks use .word pointer lists (StaminaVarPtrs -> PitcherStamina+1/PitcherStamina;
  StaminaLimitPtrs -> the three tables) and documented 16-byte tables.
- L-020: PitcherStintMark renamed PitcherStamina at all uses.  The RAM definition,
  LaunchPitchedBall, CpuPitcherDecision and PlayState_22 docs now describe the
  per-pitch decrement, the relief threshold and its reachability; "unreachable" notes
  removed.  PlayState_22's Gravity doubling is now stated exactly (stamina >= RandomA).
- L-022: password letter roll is 0-25 (PasswordEditInput and the PasswordDigits note);
  the encoder note says 0-24; the CONTINUE screen doc notes the real-input visit.
- Preamble updated to round 4.  Specs and tools carried from round 3.
- review.json regenerated by tools/review.py on the new scaffold: 2,213/2,213 entries
  reviewed, 1,157 units (5 new data units).  Compared with the round-3 review, only the
  5 new entries and the 4 routines whose docs were edited changed.  editorial_audit pending.
