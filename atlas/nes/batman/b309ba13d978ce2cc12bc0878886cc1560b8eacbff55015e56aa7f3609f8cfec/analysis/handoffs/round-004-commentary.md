# Round 4 — commentary handoff

Status: **COMPLETE** for current-source readability (all classified source of checkpoint
pass-004). Final editorial audit is reserved for final acceptance.

## Input / carry
- Input pass-004 (maps/current.json == analysis/state.json, map_sha256 d6e11e81…).
- Map changed since round 3 (provenance only), so the full round-3 commentary is preserved at
  analysis/round-3/commentary/ (+ analysis/round-3/scratch-commentary/); round-1/2 copies untouched.
  Fresh baseline via `commentary.py prepare` (2,195 scaffold entries); notes/tools carried by
  physical identity. Regenerated source differed from round 3 only in the inline provenance
  notes of the 4 split regions (now "observed executing only in an artificial (RAM-patched)
  scenario"); 2,653 labels / 459 routine headers carried 1:1; no record invalidated beyond L-020 targets.

## Edits (lead L-020, commentary part — now consumed)
- SkipArea16Flag ($76) bit7 = area-16 boss (kind 33) beaten: set by its defeat, cleared only by
  TitleAndNewGame, kept by continue; continue in area 17 restarts at area 15 and AreaCleared then
  skips to area 17. Name kept; symbol note, ObjKind33 note, AreaCleared / GameFrameLoop_continueGame
  headers (cross-linked) and an inline note at the skip test ($DE8C) updated. Uncertainty resolved.
- SetupBossSlotsArea17: wrong link to the area-16 skip removed (not fetched on that path; caller unknown).
- ObjKind42Defeated clamp: non-indexed slot-0 X read = boss's own X (BossSlotTemplate0 puts kind 42
  in slot 0); spawn-list placement not ruled out.
- $38/$3E named CsQueuePokeOffset / CsJoyStrobeBase (bank-2 orphans only); only the reset clear
  writes them, so effectively 0 (run-time-pointer writes not exhaustively excluded).
- AreaIntro_bossType5Script given a header (boss-type-5 intro, seen at the area-16 start).

## Gate
- Readability: all 16 PRG banks + header/CHR/vectors; 0 Sub_/L_/Dat_ labels, 0 mechanical branch labels.
- review.json 2,195/2,195 entries reviewed, 608 units reviewed; editorial_audit pending.
- [commentary/verification.json](../../commentary/verification.json): ok=true, byte_identical=true.
  Reading copy refreshed: commentary/reading-after.asm (32,813 lines).
- Classification unchanged: 262,158 / 262,160 B; 2 B unknown (0x1C5AE-0x1C5AF, L-018).

## Open (commentary/uncertainties.md)
- Kind-42 placement by spawn lists; SetupBossSlotsArea17 caller; orphan code; object identities,
  area-map byte formats, weak-role data.

## Toolkit
No toolkit issues found.
