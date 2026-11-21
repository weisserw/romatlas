# Commentary progress

Round 2: prepared a fresh verified baseline from checkpoint pass-002 (round 1 used pass-001) with
`python3 /root/romatlas/scripts/commentary.py prepare <workspace>` (byte-identical).

## Method
source/game.asm is produced by the workspace-local helper tools/annotate.py from the frozen
before/game.asm plus the durable annotation records annotations/*.py:
common.py (RAM / cartridge-RAM symbols), fixed.py (fixed bank), bank0..bank6.py (16K banks).
The helper only adds symbols, comments, label placement and byte-identical re-expression
of data (.incbin splits with equates, .byte/.word for far-call triples and verified code
pointer tables).  Workflow: edit annotations, then
`python3 commentary/tools/annotate.py review` (rebuilds source/game.asm and refreshes the
review.json inventory), then `commentary.py verify` and `commentary.py read`.
`annotate.py listing B` prints a reading listing of 16K bank B (7 = fixed);
`annotate.py todo` lists globals still carrying address-only names (currently none).
Do not hand-edit source/game.asm without porting the edit into the annotations, or the
next build overwrites it.

## Round 1 pass (complete current-source scope)
- All 709 code regions in all banks: every global label (657 routines/entries) has a
  descriptive name; meaningful routines carry block comments; 837 branch targets have
  cheap-local labels (hand-named where useful, otherwise derived from the branch condition,
  e.g. `@if_GameMode_zero`, `@if_IsEnemyBlocked_carry_set`, `@loop`, `@return`, `@join`).
- RAM: 226 zero-page/RAM/cartridge-RAM symbols and 57 per-slot arrays (players, enemies,
  bombs, flames, items, sound) used consistently in operands; hardware and MMC1 register names.
- Data: far-call triples as `.byte bank / .word Target-1`; the 9 verified code-pointer
  tables as `.word` label lists; ~160 named data tables (equates at their bytes) with format
  notes; generic data blocks carry boundary/role-unknown notes in the bank headers or
  block notes.
- Unknown ranges (31 bytes): honest notes; four bank-5 ranges now have concrete consumer
  evidence (see uncertainties.md) - proposed to redirect, not reclassified here.
- review.json: all 978 scaffold entries status=reviewed with summaries/source lines and
  657 units; editorial_audit remains pending (final acceptance only).

## Main interpretations established
Game = Bomberman II.  Systems: kernel/NMI/VRAM queue/far calls (fixed), sound driver
(fixed $D800-$DD9B byte-code interpreter + bank 2 envelopes/effects/APU), normal-mode
enemies with 27 kinds and boss/cutscene scripts (bank 0), stage map builder (bank 4),
players/bombs/explosions/items/password/title/menus/intro/ending (bank 5), CHR graphics
(banks 1, 6), music data (banks 3, 4).

## Round 2 pass (checkpoint pass-002; complete current-source scope)
Carry-over: the whole round-1 commentary directory was copied to analysis/round-1/commentary/ (byte-identical,
diff -r) before `commentary.py prepare` built a fresh baseline from pass-002 (1287 scaffold regions, byte-identical).
tools/annotate.py and annotations/*.py were copied in; annotations are keyed by bank + CPU address, i.e. by
physical identity, so names/comments carried by physical range.  Builder changes: Text regions parsed; L-009
provenance split points that are only branch/jump targets stay cheap locals (label sets identical to round 1
apart from the intended additions); map labels kept as block equates; review units for continuation pieces;
`@exit` instead of `@loop` for backward-referenced shared exits; `#<Sym/#>Sym` for verified same-bank
pointer pairs (MsPtr/TilePtr/PaletteSrcPtr/SrcPtr only; graphics/layout loaders excluded).
Invalidated/revised records (changed ranges):
- Bank 0: BossActionTable now a `.word` table (13 named handlers); new BossActionIdle/Rise/Fall/MoveSpeed2/3
  names; BossActionBlink "unreferenced" note fixed (table entry 12, runs in the opening); BossBounceOffsets 31 B;
  $967E unused words; $9650 = K25Frames + BossAnimateFlags (was "BossScriptData, unresolved").
- Bank 5: U notes replaced by data names (PasswordCursorSprite, SoundTestCursorSprite, VsMatchPointTiles9/
  VsMatchPointTiles, IntroScoreText) with #</#> operands; StageItemChoices 144 B + ItemChrIndex 13 B;
  StageTimeTable extent; SpecialPasswords lists the seven codes and effects.
- Bank 6 $B027 and fixed $E000-$FF67: DMC sample regions DmcSamples* with sample numbers, 5 alignment gaps;
  DmcSampleTable (bank 2) 13 records; UpdateDmcSample/DmcRepeatTable explained.
Backlog/defects fixed in previously reviewed material:
- 641 raw RAM operands -> 6 left ($0200 power-on page, write-only sound bytes $0209-$0212/$020B): full sound
  channel model (TrackPtr, Ch* per-channel arrays, MusicRegs/SfxRegs/SfxRegDirty, loop/call stacks, vibrato,
  pitch sweep, envelope), pointer high bytes, per-pad/per-row arrays, title/intro/credits/password work bytes.
- Sound commands renamed from address names to roles: SndCmdSetDuty (was SetVolume), SetSweep, SetAttenuation,
  ReduceAttenuation, SetVibrato, SetVibratoDelay, SetDetune, SetPitchSweep.
- 67 raw ROM operands -> 0: EnemyKxxAnimPtr (16), K22/K23/K24 tables, BossMirrorFlags, BossFramesK18/K19,
  BossScriptK18-K26, DeathFramePtrs, EnemyBaseScore, ScoreValueTable (11 B) + ScorePopupPtrs/Sprites,
  NoiseDrumTable, Sfx06Periods, Sfx00-11Handlers as `.word` pairs; power-on stubs use PowerOnStubPayloadN+$4000.
- 79 routines of >= 10 instructions that had no block comment now have one (menus, password, title, intro,
  VS/battle screens, player stepping, flames); enemy draw-handler comments now say what each kind draws.
- iNES header no longer carries the generic-data note.
Result: 662 named globals (0 address-only), 836 locals, review.json 1287/1287 reviewed, 1041 units
(every code region has units); editorial_audit pending (final acceptance only).
Remaining: none for current-source readability.  Unresolved meanings: uncertainties.md.

## Final editorial acceptance (2026-11-20, pass-002)
Scope: whole source, targeted scans (no whole-source reread); classification unchanged. Record: commentary/editorial-audit.md.
- Raw RAM operands 8 -> 0 (PowerOnStubRam, SndInitClear0209/0211/0212, SndCmdParam020B/0210, DemoInputPtrHi, ZeroPageFF).
- Renames: SfxData8AF4/8B60 -> Sfx10VoiceSteps/Sfx11VolumeSteps, SetDirFromTemp -> SetEnemyDirDown.
- Corrected comments: SoundTest buttons, Sfx11Repeat (volume/duty, not pitch), CtlFadeMusic result, IsEnemyBlocked lane logic,
  bank 5 $95F9 note; 33 routine comments added; pad bit layout; ending via StageCleared noted.
- annotate.py: header counted as one line -> review.json source lines were 26 short; fixed, 2328 locations checked exact.
- review.json 1287/1287 + 1041 units reviewed; editorial_audit reviewed. verify + audit ok (byte-identical); reading copy refreshed.
