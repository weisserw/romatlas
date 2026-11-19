# Round 5 — commentary handoff (with final review and strict editorial audit)

Status: **COMPLETE**. Current-source readability covers all classified source of checkpoint
pass-005. The final editorial audit was done and `commentary.py audit` **PASSED**.

## Input / carry
- Input pass-005 (maps/current.json == analysis/state.json, map_sha256 a320e4a3…2dba). The map
  changed relative to the round-4 baseline (d6e11e81…) by exactly one added region: id 2265, Data,
  Manual, file 0x1C5AE+2.
- Round-4 commentary preserved: the whole commentary/ moved to analysis/round-4/commentary/ and
  scratch/commentary/ copied to analysis/round-4/scratch-commentary/. Round 1-3 copies untouched.
- Fresh baseline from `commentary.py prepare`: 2,195 scaffold entries, the same count as before
  because the Unknown entry was replaced by region-2265. tools/, notes/, progress.md,
  uncertainties.md and remaining-branch-labels.txt were carried.
- Carry stats: 2,653 labels and 459 routine headers carried 1:1 by physical identity (notes are keyed
  by bank + CPU address). Before any edits, the regenerated source differed from round 4 only at
  0x1C5AE. A review.json rebuilt with review.py matched round 4 for all 2,194 carried entries except
  the new region; 114 bank-14 source locations shifted. Only the new region's records were
  invalidated and rewritten.

## Manual-fallback region ($1C5AE-$1C5AF, bank 14 $C59E-$C59F)
- Source (bank14.asm:442) is headed "data (Manual, user-authorized fallback; meaning unknown)". The
  note says the two bytes are generic data of unknown meaning, classified only by the user's
  fallback, not observed and not statically proven. They follow JumpTableDispatch's unconditional
  indirect JMP and precede HideUnusedSprites. If executed they would be a dead zero-page load. No
  pointer, branch, call or traced fetch reaches them. Unused entry, filler or something else is
  unresolved. They have no label and no format and are kept raw.
- review.json region-2265: reviewed, with a dedicated summary and source bank14.asm:442.
  uncertainties.md was rewritten to match. L-018 stays exhausted-unknown.

## Final editorial audit ([commentary/editorial-audit.md](../../commentary/editorial-audit.md))
Fixes found by the targeted searches:
- $DE14 `jsr $8003` after mapping bank 8 was mis-commented as Area_Scroll. It is now resolved to
  AreaSvc_GameOver through annotate.py STATIC_CALL_BANK. The GameFrameLoop header was rewritten,
  `.playerUpdate` was renamed `.areaHook` and `.noAreaHook` was renamed `.skipSpawn`.
- A bank-12 note cited $9BB9 (a file-offset tail) instead of CPU $9BA9.
- BossBlastSprites was renamed BossBlastSlotBits. Bank-2 joypad orphan temporaries $20/$21 are
  named CsJoyExpBits1/2 by the cutscene RAM override.
- 14 stale numeric aliases in comments were replaced by symbols. 111 uncommented data tables got
  role/consumer notes. The iNES header was described.
- Certainty wording was corrected for artificial-only, effectively-zero and no-reference evidence.
- 1,126 generic data review summaries now name the neighbouring structure and the bank context.
- Code regions that continue a routine started earlier now carry clipped unit records, because the
  audit requires units on every Code entry.

## Gate / results
- `commentary.py verify`: ok=true, byte_identical=true ([commentary/verification.json](../../commentary/verification.json)).
- `commentary.py audit`: **ok=true**, byte_identical=true, reviewed_regions=2195,
  reviewed_units=1322 (720 distinct routines/structures plus clipped continuation records),
  editorial_audit=true ([commentary/audit.json](../../commentary/audit.json)). Reading copy refreshed:
  commentary/reading-after.asm (32,967 lines).
- Readability: all 16 PRG banks plus header/CHR/vectors. Auto labels 0, mechanical branch labels 0,
  TODO 0. Every routine has a purpose header. Every labelled data table has a note.
- Classification (separate): 262,160 / 262,160 B classified. 262,158 B are evidence-backed and 2 B
  are a Manual fallback.

## Remaining semantic uncertainties (commentary/uncertainties.md)
- Object-kind identities beyond weapons, item drop, explosion and boss-defeat.
- Area-map byte formats and bank-10 attributes. Sound-command semantics and tune mapping.
- Kind-42 placement by spawn lists. The SetupBossSlotsArea17 caller. Orphan code with no known
  caller. Weak-role data (D-SNDX, $F4F1-$F7FF, bank-3 period-16 records, duplicate bank tails).
- The role of the $C59E bytes.
- The per-entry meanings of the round-5 data-table notes are mechanical only.

## Files
New Markdown: commentary/editorial-audit.md and this handoff. No other new .md files. Helper
scripts: scratch/r5/data_consumers.py, data_notes_gen.py, data_notes.py. Outputs:
scratch/r5/data_consumers.json, data_ctx.txt, audit-out.json, review-scaffold-pass005.json.

## Toolkit
No toolkit issues found.
