# Round 1 commentary handoff

Status: complete for the current-source readability scope (classification unchanged: 31 unknown bytes remain).
Input checkpoint: pass-001 (maps/pass-001.json == maps/current.json; state frozen in commentary/phase1-state.json).

## Covered scope
- Prepared with the toolkit (`scripts/commentary.py prepare`); baseline byte-identical.
- All 709 code regions / all banks (fixed, 0-6): 657 global routines/entries named (0 address-only names left),
  block comments on meaningful routines, 837 branch targets as cheap locals (hand-named or condition-derived:
  `@if_GameMode_zero`, `@if_IsEnemyBlocked_carry_set`, `@loop`, `@return`, `@join`, case labels).
- 226 RAM/cartridge-RAM symbols + 57 per-slot arrays; hardware/MMC1 register names; used in operands.
- Data: far-call triples as `.byte bank/.word Target-1`; 9 verified code-pointer tables as `.word` label lists;
  vectors as `.word NMI/Reset/IRQ`; ~160 named data tables with format notes; generic blocks have boundary/role notes.
- Unknown ranges: honest notes (commentary/uncertainties.md).
- review.json: 978/978 scaffold entries reviewed with summaries and source lines; 657 units; editorial_audit pending.
- Durable records: commentary/annotations/*.py (resume/edit point), commentary/tools/annotate.py (builder; see
  commentary/progress.md). Do not hand-edit source/game.asm without porting into annotations.

## Examples
- Fixed: FarCall/FarCallReturn, SelectPrgBank (MMC1 retry on NMI interruption), VramQueueEntry (queue format),
  ReadJoypads (Four Score + double read), GameFlowMain/StageLoop/StageCleared/RoundOver, sound byte-code interpreter
  (SoundCommandTable $D0-$EB).
- Bank 0: SpawnEnemy, IsEnemyBlocked (EnemySolidMask), EnemyAiTable/EnemyDrawTable (27 kinds), RunBossScript, AddScoreDigits, CreditsScroll.
- Bank 2: SoundDriverCommand (cmd ranges), UpdateMusic (SongTable), EnvelopeStep, SoundControlTable $80-$8C, SfxTable.
- Bank 4: BuildStageMap/DrawStageLayout (mask-coded columns), PlaceSoftBlocks/PlaceItems (map values $20/$21/$22).
- Bank 5: UpdatePlayers (Cur* work copy), MovePlayer/corner sliding, PlaceBomb/SpreadExplosion/BlastCell,
  ItemEffectTable (12 items), PasswordScreen/GeneratePassword/SpecialPasswords, TitleScreen, ModeMenu, BonusStage,
  SoundTest, EndingSequence.

## Unresolved meanings / questions for redirect
1. L-001: 4 of 5 unknown ranges now have concrete consumers (bank 5 $9508, $A0B1 metasprites; $B8E9 metatile lists;
   $B9C7 text "SC") - propose classifying as data (details in analysis/leads.json L-001 commentary_round1).
   Bank 6 $B027 (0x1B037+6) still has no consumer.
2. Fixed $D2F9-$D7FF, $DDA9-$DFFF, $E000-$FF67 generic data: no reader (L-004).
3. Data-structure extents inferred but unproven: StageItemChoices ($80CB, 3 bytes/stage), BossActionTable ($9664,
   13 words used by jmp (PpuDest) at bank 0 $969B - an extra dispatch table not in phase-1 tables_spec), StageTimeTable.
4. Enemy kinds 12-14, 22-24 partly analysed; several sound commands only mechanically named.
5. Reachability candidates: HiddenP2WorldSelect ($A94A, pad-2 Up -> ending), DemoInputPlayback recording path,
   special passwords (could be tested via password screen; values in SpecialPasswords $9703 with PasswordDigitMap).

## Verification
- `scripts/commentary.py verify`: byte_identical true (commentary/verification.json; source game.asm sha256
  4e01e93c...). Final `audit` not run (reserved for final editorial acceptance).
- Reading copy: commentary/reading-after.asm (refreshed). Frozen baseline: commentary/reading-before.asm.
- Toolkit: no new toolkit issues found in this phase (analysis/toolkit-issues.md unchanged; TK-001..TK-004 remain).
