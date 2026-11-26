# Final editorial acceptance (checkpoint pass-005)

**Status: ACCEPTED.** `commentary.py audit` -> ok, byte_identical true, 4280 regions + 1896 unit records
reviewed, editorial_audit reviewed. Artifact: commentary/audit.json; reading copy commentary/reading-after.asm.
input/game.nes unchanged (sha256 36a51481...5dce). Toolkit, maps/ and analysis/state.json untouched. Nothing
archived or published; no processes left running.

## Checks performed (targeted; round-5 review credit reused, no whole-source reread)
- Raw numeric operands in source/bank*.asm (abs/zp); auto/address-style labels (B<n>_Sub/Data/Entry..., @loop_/@skip_).
- Stale prose: unknown / Text / padding / not proven / unused / dead.
- Unit names vs label_index offsets; source mapping labels vs label_index; unit extents vs map region kinds.
- Name collisions (same name, different offsets) and name/kind contradictions (code-like names on data and the reverse).
- Repeated comment lines; generic RAM symbols in use (Var/Stack/WorkRam), cross-checked against event context.
- Pre-edit snapshot: analysis/final-acceptance/pre-edit/ (source, notes, tools, review/index files).

## Issues fixed
1. Stale numeric aliases:
   - Reset stubs in banks 0-6: `jsr $cf10` / `jmp $cf24` now call InitRecords / WarmStart. MMC1 mode 3 is set just
     before, so bank 7 is fixed at $C000.
   - Bank 5 CL_InitBracket: `$93db,x` and `CL_InitBracketRts,x` are now CL_BracketHandlers-2 / -1.
   - Bank 6: `$ae2b,x` is now DV_OutcomeTable-12.
   - Bank 2: the self-modifying store is now FS_UnusedEmphasisCycle+1.
   - Bank 7 Draw/ErasePauseBanner: the generic Var04AD symbol was removed. It is DvVaultPhase, because the read only
     happens when EventId = 6. The banner tables hold 7 records: events 1-6 plus a Dino Vault alternate at $90.
   - WarmStart `jmp $8000` is annotated: it enters the selected bank's EntryJump.
2. Wrong boundaries: 21 code units no longer extend over trailing data, padding or vectors (B0-B6_Reset 170 -> 35 B,
   PlayCursorSfx, MT_SetMsprFrame, FS_QueueTileRows, ...).
3. Missing structures: the first audit failed with "code region lacks reviewed units: region-6". 847 continuation
   Code rows now carry split inventory records of their covering routine. Each record keeps the routine name, a
   clipped extent, a summary and a source location. The audit then passed.
4. Misleading certainty: FS_UnusedEmphasisCycle and DR_DrawBothRacers are now "treated as unused (no static
   reference); reachability not proven" (L-004 consistent).
5. Cosmetic: trailing comments on long instruction lines now keep a separating space.

Workspace-local tool changes (commentary/tools):
- build_source.py: new `~ADDR expr` operand-expression note directive.
- update_review.py: trims trailing non-code bytes from code units and adds split records.
- uncertainties.md: new "Final acceptance" section.

## Verification
- `commentary.py verify` and the audit's fresh rebuild: byte_identical true.
- rom_sha256 36a51481de96e0848e435e96c9d1788eef578fc71e4de20e883fca4311ad5dce.

## Remaining explained uncertainties (commentary/uncertainties.md)
- Role-unknown generic data with no consumer: bank 0 $A108, $A8CF; bank 2 $9420, $9C25, $ACE0; bank 3 $954B, $B375;
  bank 4 $B348. Also the unused metatile records (bank 0 $ADBF, bank 7 $D881).
- L-004: unreferenced static routines (reachability only). L-005/L-006: deferred roles.
- IrqHandler: never observed.
- Generic RAM symbols with "role not established" comments (Var82/83, Stack0111/0112).
- Local-label address suffixes (@done_84F0 etc.) are the build tool's per-bank uniqueness scheme. They are not stale
  aliases.

## Readiness
Every scaffold row and discovered unit is reviewed, and editorial_audit is recorded. The audit passes with a fresh
byte-identical rebuild. The commentary is ready for archive acceptance. The audit cannot judge prose quality; the
editorial review above covered that with targeted checks. This makes no claim of full hardware/game compatibility.
