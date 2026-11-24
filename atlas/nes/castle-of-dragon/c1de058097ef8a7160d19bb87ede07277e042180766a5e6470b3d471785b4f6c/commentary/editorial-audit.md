# Final editorial audit (baseline pass-005)

## Scope
commentary/source/game.asm and symbols.inc (generated from commentary/notes/*.txt by
commentary/tools/apply.py), review.json (752 scaffold entries, 855 units) and
uncertainties.md. Prior review credit was reused; no whole-source reread. Reviewed material
was revisited where a scripted scan or targeted reading found a concrete inconsistency.

## Checks run
1. $C415 / L-001 wording: every mention in source, review.json and uncertainties.md says
   generic Data by explicit user-directed Manual fallback, meaning unknown, no role proven,
   orphan RTS vs stray byte unresolved. No text calls it proven data or "unclassified".
2. Provenance markers: all 192 Static code regions of maps/current.json match the 191
   "[static-only code ...]" markers plus one region ($A681 EnemyIdle) that starts its own
   "; Code; Static" block. No stale markers.
3. Stale numeric aliases: scanned comments for $xxxx addresses equal to a named label.
4. Stale/undefined names: scanned comments for CamelCase identifiers that are not
   defined labels/symbols.
5. Symbols: duplicate global labels (none), address-suffixed or generic names, RAM names
   checked against all their uses.
6. TODO/FIXME/XXX/TBD/placeholder/"???" (none), doubled words (none), rare-word typo scan
   (none found), mechanical line-by-line end-of-line comments (78 EOL comments in total,
   none merely restating the opcode).
7. Every incbin (header, image, data, vectors) checked for a preceding note or a shared
   group note.
8. Certainty wording: "always/proven/confirmed/unused/Japanese" style claims read in context.

## Findings and fixes
- PauseFlags ($053C) was a misleading name with contradictory comments ("stage frozen",
  "game paused/frozen", "boss screen"). Bit 7 is set only by the boss-screen loader and
  cleared by StageStart; every reader gates a boss-screen path; Select-pause uses
  SchedFlags. Renamed BossScreenMode, comments aligned, explanation added to symbols.inc.
- LoadStage loads the StageBoss* tile sets/palette and sets boss mode; ordinary stages use
  SetupStage. Renamed LoadBossScreen, header rewritten (removed the stale "StageParam
  tables" and "freezes play" wording); StageStartPageTbl -> StageBossPageTbl.
- Stale numeric aliases replaced by names: RunBoss ($AC25), GuardianMoveFlyer ($A8DF),
  CheckBossHit caller ("$BCE5 after $B8E3" -> UpdateCombat, runs CheckPlayerCollisions),
  SetStatusFromProbeY caller ("$F8F0" -> GetTerrain on boss screens).
- SpriteTest header named non-existent SpriteTest*Tbl tables; now ObjTypeTileSetTbl/
  ObjTypeFrameTbl/ObjTypePaletteTbl (what the code reads).
- Phase-1 address names with established roles given source-only names: Stage5AnimTiles
  (bank 2 $B200), Stage6AnimTiles (bank 3 $8000) via a new apply.py "N" directive; the
  portable map keeps its labels. Fill/blank-tile names (Bank4ZeroFill_B381 etc.) kept:
  bank+address is their only identity.
- BitMaskTbl had no note: added.
- ObjType table group note placed the "one unreferenced byte" at the wrong label; the
  byte $E78C (after StageMusicTbl's 8 entries) is now noted at StageMusicTbl. A broken
  line wrap was fixed and "game identities unknown" added.
- CartridgeFooter: "ASCII Japanese title ... unused/footer bytes" overclaimed; now the
  ASCII text "DRAGON UNIT" plus ten bytes with no known reader, contents not interpreted.
- Vectors note now names the handlers (NMI, Reset, IRQ). A historical remark about an old
  name was removed from SpawnLiftPlatform (history stays in progress.md).

Result: fresh byte-identical rebuild; review.json scaffold unchanged; units regenerated
(855, all reviewed, no "see source" summaries).

## Residual known limitations (explained in source and uncertainties.md)
- $C415: Manual user-directed generic Data fallback, meaning unknown (L-001 open).
- L-002 unreferenced static fragments (reachability not proven either way).
- Object types, sounds and quest items named by number; game identities unknown.
- ObjFieldD/ObjFieldF only cleared; TerrainFlags bit names inferred.
- MsprFrameData $8A1D-$8A40 (probably an unused frame grid) and frames $E6-$EF usage.
- SndSequenceData exact end not proven; zero runs in banks 4/5 have no reader.
- Stage 5/6 boss screens ran only under forced entry.
- Weaker style (unchanged): some sound/AI data described per block; a few generic @Loop
  locals in short routines.
The audit tool cannot judge prose quality; this review is the editorial judgement.
