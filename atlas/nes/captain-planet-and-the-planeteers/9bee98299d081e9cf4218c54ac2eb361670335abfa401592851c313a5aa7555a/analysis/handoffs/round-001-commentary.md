# Round 1 commentary handoff (input checkpoint pass-001)

Status: COMPLETE - current-source readability done for all classified source; fresh
byte-identical verification passed; reading copy refreshed.  This is not ROM completion
(classification still 92.64%, 19288 bytes unknown) and not final editorial acceptance
(review.json editorial_audit pending).

## Covered scope
- Whole ROM, all 1568 review.json scaffold entries (physical $00000-$3FFFF): header, PRG banks
  0-15, vectors, CHR.  Code banks 3,6,7,8,12,13 ($A000), 14 ($C000), 15 ($E000) fully named and
  commented at routine level; data banks/blocks carry phase-1 role notes plus labels where a
  consumer is known; all 71 Unknown ranges have boundary/role notes.
- No remaining resume ranges.

## Readability assessment
- 1132 global labels, all descriptive (kernel: TaskDispatch, CreateTask, TaskYield, BuildOam,
  QueuePaletteInline, MapCollisionTest, PlaySong...; stage logic: GeoStageLoop, ShipInput,
  PlayerShotTask, EnemySpawnScan, CpPlayerTask, CpMovePlayer, SpawnFromMapCell...).
  530 routine/block comments explain purpose, inputs/outputs, state and bank context; bank
  banners explain the task model, bank roles and stub tables.
- ~150 RAM symbols with comments (object/task tables, scroll, map, sound, stage state), bank
  aliases where RAM is reused; hardware/MMC3 register names; `.word` inline task/sprite args.
- Gate follow-up (same phase): every RAM operand is now symbolic (0 raw RAM operands). Bytes
  without an understood role get mechanical names Unk_<owning routine>_<addr> (139 symbols,
  declared once at the top of game.asm with "role uncertain", uses and owners; list in
  commentary/tools/autoram.json). All local branch labels are descriptive: forward targets are
  named by their first action or by the branch condition (@ifLevelEq03, @ifJoyInput08Clear,
  @ifObjVarBNonzero ...), loops by what they iterate (@forY, @repeatWhile..., @loopCall...).
  0 generic @skip/@loop labels remain; 20 labels carry only a flag-level condition
  (@ifCarryClear/@ifZero/@join...) where the tested value is not nameable.
- Residual: game identities of many enemy objects and of the Unk_* bytes remain tentative.

## Examples
- commentary/source/game.asm: `TaskDispatch` (bank 15), `EnemySpawnScan` (bank 3),
  `ChaserTask`, `MapCollisionTest` (bank 14), `CpPlayerTask` (bank 7), `GeoStageLoop` (bank 12).

## Unresolved meanings (details: commentary/uncertainties.md)
- Power numbering vs. Fire/Water/Wind/Earth/Heart; identities of bank 3/6/8/13 objects.
- Pause interpretation of PauseState/$07C2 (Select) untested; several $07xx event variables.
- Bank 13 StepObjectAnimation13 reads a table inside its own code (stale copy?).

## Structural questions for redirect (concrete, with bounded checks)
1. Bank 3 $A4B6-$A8B4 / $AA4D-$AB16 / $ABC3-$AC5F: JMP table SpecialObjectHandlers ($A3BD,
   index ObjVarA-$1E) points at $A4D6,$A4DD,$A507,$A55D,$A6D1,$A777,$AA4D -> code roots (L002).
2. Bank 6 $A6E5-$A984 (+ $A9BE, $AA2D gaps): EventObjectHandlers table $A2FE (index ObjVarA*2)
   targets -> code roots; prove the index domain from spawn records.
3. Fixed bank: $F1E6-$F239 and $F3D9-$F49E are SpawnTaskGroup entries (lda #<X / ldy #>X before
   jsr $F73C) -> task code; $C0AB/$C0D2/$C0DE/$C0F0 and bank 7 $A012/$A018 are 'jmp $0000/$C000'
   filler jump-table slots; $C23E-$C24D decodes as an unreferenced SubFromPtr0.
4. $C639-$C734: word table read by MetatileLookup (lda #$39/#$C6 base, index id*2) -> data.
5. Bank 12 $A003-$A018: 11-word briefing-text pointer table read by TextWindowSelectText (bank 12
   mapped, lda $A003,y); the ten single unknown bytes between bank 12 text runs are terminators.
6. Small consumer-proven data: $D252 (HUD template), bank 3 $A054 (5 data + 6 code bytes),
   bank 6 $A054, $B79B; bank 7 $AA3B, $B338-$B367, $B755-$B76C (palette table), $BB76-$BB95,
   $BD6D-$BFEF (Captain Planet sprite definitions); bank 12 $A69A, $AB03 (table tail);
   bank 3 $ADBF (table tail).
7. Banks 8/13 (L001): follow MapSpawnTable/MapSpawnTable13 and LevelInitTasks word lists
   (bank 8 $B19C, $AA23; bank 13 $A6C6, $A3C5) to classify the task code in the gaps.
8. Possibly dead copies: bank 12 $A879 (SetHitBoxB6 copy), bank 13 $A35D and $A71E.

## Remaining gaps
- Readability: none outstanding for the current scope (residuals above are accepted minor items).
- Classification: unchanged (71 Unknown ranges, 19288 bytes; see uncertainties.md list).

## Verification
`python3 /root/romatlas/scripts/commentary.py verify <ws>`: ok=true, byte_identical=true
(commentary/verification.json, source game.asm sha256 in that report); `read` refreshed
commentary/reading-after.asm.

## Artifacts (local only)
commentary/source/game.asm (buildable, byte-identical), commentary/reading-after.asm,
commentary/review.json, commentary/progress.md, commentary/uncertainties.md,
commentary/verification.json, commentary/tools/ (build.py, review.py, ann/*.py, q.py, compact.py).
Regenerate source with `python3 commentary/tools/build.py && python3 commentary/tools/review.py`.
No toolkit issues found in this phase (analysis/toolkit-issues.md unchanged).
