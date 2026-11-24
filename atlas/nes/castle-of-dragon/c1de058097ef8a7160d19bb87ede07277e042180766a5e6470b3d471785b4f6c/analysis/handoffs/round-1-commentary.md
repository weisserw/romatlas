# Round 1 commentary handoff - COMPLETE

- Input checkpoint: pass-001b (maps/current.json = maps/pass-001b.json, sha256 0ee481b5...).
  commentary/ prepared from it (frozen before/, baseline.json, phase1-state.json).
- Verification: commentary/verification.json - byte_identical true (game.asm a2033791...,
  symbols.inc d101b65d...). Reading copy refreshed: commentary/reading-after.asm.
- ROM classification unchanged: 100.00% (131087/131088; 1 unknown byte $C415).
  Source readability (separate metric): all classified source covered, no deferred ranges.

## Covered scope
All code in bank 6 ($90D8-$BFE9) and bank 7 ($C000-$FD99), all code-embedded tables,
data/image blocks of banks 0-6, vectors/footer, the unknown byte. 236 routine/table
labels, 1221 named branch labels, 257 symbols (RAM, object fields, registers, constants).
review.json: 497/497 scaffold entries reviewed, 632 units; editorial_audit pending
(final acceptance only). Details: commentary/progress.md, commentary/uncertainties.md.
How to edit: commentary/notes/*.txt -> commentary/tools/apply.py -> review.py -> verify.

## Readability assessment
Good: every routine has a purpose comment, branch labels are descriptive, operands use
RAM/field/table symbols and bit-mask constants (BTN_*, PLR_*, EQUIP_*, PRG_BANK_*,
BITOP_*). Comments are block-level rather than per instruction. Weaker spots: guardian
and enemy types are named by type number (identities unknown), sound numbers are
numeric, bank 4/5 and BossPatchData are described by sub-range rather than split.

Examples: RunTasks/TaskYield (scheduler), AllocFrame (zero-page frames), DrawMapBlock
(bank-4 block map), GetTerrain (bank-5 collision), DrawMetasprite, StepTrack (sound
commands), GuardianMoveFighter + AiDuelist (AI via pad-2 bytes), BossStage7/DragonFight,
UpdateHudAndStageEnd, FinalBossDefeated, PlayEnding, TestModeMain/BgTest.

## Findings useful for redirect (structural questions)
1. L-003 ending route (static): stage 7 dragon fight needs armour (PlayerEquip $0512
   bit 7) and quest item 1 (QuestItems $0511 bit 7), else DragonOverwhelms; boss hits
   need QuestItems bit 5 (CheckBossHit $E3D9); BossHitPoints $03CA < 0 ->
   FinalBossDefeated $E476 -> PlayEnding $D71B only with QuestItems bit 4. Bounded check:
   on the map screen set $0511=$F0, $0512=$80, $053B=$7F, $052A=7, run the dragon and
   observe the ending (also locates the consumer of the staff text at bank 6 $8F92).
2. L-005 resolved statically: JSR $BEFE at $F151 runs with bank 6 mapped
   (PlayerControl is called after SetPrgBank(6) at $E563; intervening calls restore the
   bank) -> DamagePlayer; the returning weapon costs energy while it flies.
3. L-004 bank 4/5 structure: bank 4 = map layout $8000-$8E3F, block attributes $8E40+,
   block definitions $9071/$9E31/$A621/$ADE1, metasprite size tables $BA00/$BB00,
   palettes $BC00; bank 5 = collision map $8000-$8E3F, sound tables $9000/$9100/$9200 +
   sound data; bank 6 $8000-$81FF metasprite frame pointer table. A redirect could
   split these into typed regions (exact ends of block sets/sound data still open).
4. L-002 fragments: each explained (see uncertainties.md); all look like dead leftovers;
   $9A27 is a table entry filtered out by the dispatch range check.
5. L-001 $C415 unchanged: padding vs dead code.
6. L-006 test mode: rows map to TestPlayerOpt $0510 (no reader outside test mode),
   WeaponsOwned $06FC (bit 7 becomes armour on EXIT), MaxEnergy $06FD (4-8),
   sound/sprite/BG test numbers. Bank 2/3 shared pages are stage 5/6 animated tiles.
7. Open semantics: object type identities, sound IDs, ObjFieldD/F, staff text consumer.

## Remaining gaps
None in readability scope. Final editorial audit not run (by design).
