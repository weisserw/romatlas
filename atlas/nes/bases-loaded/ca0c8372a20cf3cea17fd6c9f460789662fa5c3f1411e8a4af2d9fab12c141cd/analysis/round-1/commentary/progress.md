# Commentary progress

Prepared verified baseline (2026-11-19, from checkpoint pass-001, map sha256 6a0d90d1...).

## Round 1 commentary pass (2026-11-19)

### How the source is maintained
- `source/` is the buildable commented assembly. It is rendered by the commentary-owned
  script `tools/build.py` from the frozen listing `before/game.asm` plus the editable notes
  in `notes/` (never by the toolkit generator). Edit `notes/*.txt`, then run
  `tools/check.sh [banks]` (build + byte-identical verify + list of remaining automatic
  labels). `tools/review.py` refreshes `review.json` from `tools/linemap.json`.
- Layout: `game.asm` (header, includes, CHR), `hardware.inc` (PPU/APU/MMC1 registers),
  `ram.inc` (RAM symbols and the Runner/Fielder/Marker record layouts), `prg32_0.asm` ..
  `prg32_7.asm` (one `.scope BankN` per 32 KiB PRG bank; CPU $8000-$FFFF).
- Notes syntax: `= ADDR Name` label, `; ADDR text` block comment, `> ADDR text` inline
  comment, `~ ADDR text` comment after the label, `include bankM LO-HI` reuse of identical
  copies (module copies, banks 5-7 = bank 4). `notes/bank10N.txt` cover the lower-half reset
  stub views assembled at $FFB0.
- Safety checks in build.py: labels may not shadow RAM/hardware symbols; cheap-local labels
  that would cross a global label fall back to numeric operands (reported as warnings).

### Scope covered (all classified source)
- Every 32 KiB bank 0-7, header, vectors and CHR. All automatic `Sub_xxxx`/`Lxxxx` routine and
  jump labels were replaced by descriptive names (1,054 names in bank 0, 578 bank 1, 180 bank 2,
  795 bank 3, 134 per voice bank). No address-only routine labels remain.
- RAM: ~300 named symbols plus record layouts (Runner, Fielder, Marker); all RAM operands
  used more than once are symbolic; hardware and MMC1 registers are symbolic. Jump tables are
  rendered as `.word Label` lists; data tables referenced by code carry labels (`Label+1`
  for high-byte reads).
- Hand-written explanations for the engine (reset/main loop/NMI/bank switching and mapper-
  write recovery, ball physics, fielding AI, runners, at-bat state machine, pitch/bat control,
  scoring, camera/strip streaming, VRAM queue, sprites), bank 1 (pitching view, scoreboard,
  computer opponent states, demo input), bank 2 (field map builders, sound driver), bank 3
  (scene engine, menu families, password chain, music starters) and banks 4-7 (voice engine,
  DPCM player). Routines without a dedicated comment get a generated "Mechanical summary"
  (calls, writes, reads, hardware) and sit under a family comment.
- Data, padding and unknown ranges: boundary notes with the phase-1 evidence; unknown ranges
  are marked as such ("kept as raw bytes"), with specific explanations where established
  (e.g. EnterPitchingMode at $8173 decodes as code -> L-009).
- review.json: all 1,465 scaffold entries have status reviewed, a summary and source
  locations; 1,596 units (routines/structures). editorial_audit remains pending (final
  acceptance only).

### Interpretations changed during the pass
- $2A/$2B is the fielding (pitcher) pad, $2C/$2D the batting pad (first guess was reversed).
- $F8/$F9 are the batter's box position/stance, $67 the swing timer (not pitch aim).
- Depth axis: home plate near BallX=$04E0, outfield toward 0; CheckBoundary uses $DD39 at the
  plate end and $DF59 at the outfield end.
- Module copies in banks 3-7 are at the same CPU addresses (not relocated); JSRs are stubbed.

### Remaining editorial work (not blocking readability, recorded for final acceptance)
- Branch-level labels are generated mechanically but carry their meaning: forward targets
  are named after the condition that branches there (`@ifBallFlagsZero`,
  `@afterCheckXCarrySet`, `@ifBelow` when no named state is involved), loops after the state
  they walk (`@loopRunnerFlags`, `@waitPPUSTATUS`, `@loopX`), plus `@return`/`@continue`;
  about 90 hand-chosen names in bank 0. A final editorial pass may still rename important
  branches by game meaning.
- Bank 1 computer-opponent states and bank 3 scene/menu routines have shorter, partly
  mechanical explanations; several bank-3 handlers continue into phase-1 unknown bytes.
- Generic data blocks are described only by boundary/evidence notes (allowed fallback):
  bank 0 $C567-$DC0F and $DC10-$E33E, bank 1 $947C-$A000/$A000-$D31B, bank 2 sound data
  $ADCA-$BFDC and the upper 16 KiB, bank 3 data islands.

### Verification
- `python3 /root/romatlas/scripts/commentary.py verify <workspace>`: ok, byte_identical true
  (commentary/verification.json). Reading copy refreshed with `read` (reading-after.asm).
