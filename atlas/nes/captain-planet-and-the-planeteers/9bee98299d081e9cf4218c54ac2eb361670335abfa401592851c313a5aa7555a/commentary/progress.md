# Commentary progress

## Round 4 (input checkpoint pass-004) - COMPLETE; final editorial audit PASSED

Preparation: maps/current.json (== maps/pass-004.json, sha256 3debb849...) differed from the
round-3 frozen baseline (pass-003, ba45ade7...), so the whole round-3 commentary/ was moved
unchanged to analysis/round-3/commentary/ (game.asm c74a7949...) and `commentary.py prepare`
built a fresh verified baseline (1940 entries).  tools/, uncertainties.md and this file were
copied from round 3; tools/mapdiff.py now compares with the round-3 baseline and reports only
the four 1-byte ranges ($0C185, $0C19B, $1FB79, $1FDD0) Unknown -> Data (Manual).  Every other
region id/offset/kind is unchanged, so names, comments and units carry by physical address
(units identical to round 3 apart from shifted source line numbers).

Map change handled: the four unknown() notes were removed (tools/strip_unknown.py) and
tools/ann/r04.py gives each Data byte a note: generic Data, meaning unknown; classified only
by the user-directed manual fallback (Manual provenance), not an observed or statically proven
data role; prior ambiguity (lone unreferenced, never-fetched $60 after RTS, code vs data
unresolved, analysis/handoffs/round-003-redirect.md).  No labels.  build.py marks Manual
regions in the block header and no longer prints their truncated phase-1 evidence line;
review.py writes the disclosure summary.  Header line "Unknown = phase-1 unclassified" replaced.

Final editorial fixes (details in review.json editorial_audit and uncertainties.md):
RAM addresses in comment prose rendered with bank-aware symbols (build.py prose(), mirrored in
review.py); MapScrollLeft/Down/Up ($E6-$E8); SpawnLog/SpawnLogLo aliases for $057B in stage
code (ObjKill via oper override, PasswordTask keeps PasswordDigits); BriefingSpeakerIndex
(bank 15 $0795); SetPriorityIfInWater; SelectPower/ObjKill/PasswordTable comments; HUD
.word RAM sprite pointers symbolised; inline-word review summaries name helper + target.

Regenerate (from commentary/): `python3 tools/build.py && python3 tools/review.py &&
python3 tools/metrics.py`, then `commentary.py verify` / `read` (or `audit`).

Results: 1399 code labels, 727 with routine comments, 0 address-only globals, 0 raw RAM
operands, 0 raw code targets, 0 generic locals (29 flag-only), 106 Unk_* RAM symbols with
notes; review.json 1940/1940 entries and 2184/2184 units reviewed, 0 Unknown entries
(4 Manual Data entries disclosed); editorial_audit reviewed.
Verification: `commentary.py audit` ok, byte_identical=true, reviewed_regions 1940,
reviewed_units 2184, editorial_audit true (commentary/audit.json; game.asm c3041b93...);
reading-after.asm refreshed.  archive.editorial_status: complete, no reasons.

## Round 3 (input checkpoint pass-003) - COMPLETE for current-source readability

Preparation: maps/current.json (== maps/pass-003.json, sha256 ba45ade7...) differed from
the round-2 frozen baseline (pass-002), so the entire round-2 commentary directory was
moved unchanged to analysis/round-2/commentary/ and `commentary.py prepare` built a fresh
verified baseline (before/, baseline.json, phase1-state.json, reading-before.asm,
1940-entry review.json).  tools/ and uncertainties.md were copied from round 2 and carried
by physical address (tools/mapdiff.py now compares with the round-2 baseline).

Map changes handled (tools/mapdiff.py): bank 6 $A175/$A18B Code -> Unknown (old labels
Bank6StubReturn/2 removed, Unknown notes added); bank 13 $BA5D-$BA5F Unknown -> Code
(PusherTask_deadJump, note replaces the Unknown note); bank 8 region split at $B20E
(EmptySpriteDef8).  The builder reported no annotation off an instruction/data start
after these fixes.  tools/asmparse.py now skips phase-1 region comment lines.

Edits (new module tools/ann/r03.py plus in-place fixes to carried modules):
- L010: EmptySpriteDef8 labelled/explained; TransformFrames rendered as
  `.word TransformSpriteN / .byte dx, dy` records; 13 transformation sprite definitions
  named; stale "trailing 11 00 00 01, no reader" note removed.
- L011: bank 7 $B0CD task word rendered as `.word $A015` with a CpStageBank note and a
  block comment on the boss-zone entry; CreateInitialObjects(13) -> CreateBossZoneObjects(13),
  LevelInitTask* -> BossZoneTask*, "level N start object" -> "boss-zone object";
  StageEndDispatch / StartGaiaMessage / GaiaMessageTask / GameOverMenuTask (+ Gaia*
  screen helpers); RingLevel -> Lives, EndingFlag -> StageEndFlag with value notes;
  Level3BossTask header rewritten (escape vs defeat), @bossEscaped; CpPlayerDeath note.
- Other defects fixed: 23 owner+address branch labels named (LoadStagePalette_level2..,
  TextWindowNextChar_fetch/decode.., GaiaSpriteTask_active/arrived, ...); stale
  TerrainHitCount mentions; wrong "empty list" note on MapSpawnTable13; RAM
  AccelCounterX/Y, AccelDelay, CpLastStepX/Y, TextReadOffsetLo/Hi, TextEndPause and
  3-byte palette buffers; multi-line RAM notes rendered as comment blocks (build.py).

Regenerate (from commentary/): `python3 tools/build.py && python3 tools/review.py &&
python3 tools/metrics.py`, then `commentary.py verify` and `commentary.py read`.

Results: 1398 code labels, 0 address-only globals (20 remaining _A0xx/_XXXX suffixes are
CallCpStageBank_A0xx-style trampolines named by their variable-bank entry), 726 with
routine comments, 0 raw RAM operands, 0 generic locals (29 flag-only), 109 Unk_* RAM
symbols with "role uncertain" notes; review.json 1940 entries reviewed, 2180 units,
4 Unknown entries with notes; editorial_audit pending.

Verification: `commentary.py verify` ok=true, byte_identical=true (game.asm sha256
5f4f505d...); `commentary.py read` refreshed reading-after.asm.

Follow-up: review.py now spans a bank's last label over its contiguous code run; added
PasswordInputLoop_bank15/_checkUp (bank 15 $E000).  All 1940 entries have units where
code (2184 units).  verify ok/byte_identical (game.asm c74a7949...); read refreshed.

## Earlier rounds
Round 2: analysis/round-2/commentary/progress.md.  Round 1: analysis/round-1/commentary/progress.md.
