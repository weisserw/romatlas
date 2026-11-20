# Final acceptance — commentary editorial audit

Status: **DONE**. The editorial audit passed. The ROM is **INCOMPLETE**: 3 bytes are still unknown.
Checkpoint: pass-004 (maps/pass-004.json), with 262157/262160 bytes classified.

## Scope
- Targeted final editorial pass over commentary/source/game.asm, which is built from
  commentary/notes/annotations.txt by analysis/tools/commentary_build.py. Review credit from rounds 1-4 was
  reused; there was no whole-source reread.
- Checks covered:
  - address-only or numbered symbols;
  - collisions between equate values across banks;
  - stale round/pass references;
  - overclaims ("unused", "dead", "never", "no writer", and "observed" inside Static code);
  - the 3 unknown-range notes;
  - commentary/uncertainties.md.
- Pre-audit copies are in scratch/final-acceptance/pre/.
- The 3 unknown bytes were not reclassified. The map, state and input/game.nes were not touched.

## Defects fixed (details in commentary/editorial-audit.md)
- Four RAM equates had address-only names, which the round-4 "0 address-only" count missed:
  - $07AF is now EngineLevel;
  - $0500 is now RamPage05;
  - $05F6 is now HarbourReturnFlag (tentative);
  - $04A0 is now TourWrapFlags.

  Derived local labels were renamed to match.
- ScriptArgC6/C8/CA had empty comments; their roles are now commented.
- ScrCmd_End: "unused entries" is replaced by the exact table slots ($16/$17). The $E5FD end byte is no longer
  described with the handler's name.
- The $ADB6 unknown note no longer suggests a "dead epilogue from a jsr->jmp edit"; the L-023 twin test was negative.
- FatigueDrainFlag and BitArrayPtr: "no writer in the ROM" became "no writer found". This was rechecked
  against the operand bytes.
- The L-016 whole-bank notes for banks 3, 5 and 8-11 said "never mapped". They now say "no bank-select path maps
  ... (static audit)"; this was changed in the builder.
- The bank-12 data at $98B5-$98D0 now has a note: probable script bytecode that resumes after
  ScriptInline_TimeUpTest, including the Poke $05F6 reading. The Data kind is unchanged.
- uncertainties.md: the round-3 note about 5 unknown bytes is marked superseded, the wording is softened, and a
  final-audit section was added.
- analysis/progress.md: the stale round-3 coordinator note (5-byte residual) is labelled as historical.

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify <ws>`: ok, byte_identical true
  (commentary/verification.json).
- `python3 /root/romatlas/scripts/commentary.py audit <ws>`:
  - ok true, byte_identical true, editorial_audit true;
  - reviewed_regions 3272, reviewed_units 1081;
  - results in commentary/audit.json;
  - game.asm sha256 54734f860faef7b796e83fa226267ed5425c2bc88dc925c5061cdadcf247c82a.
- commentary/reading-after.asm was refreshed by the audit
  (sha256 b77623bbe88e301b82ac17b14d800187c24f334bc57dcc981217f82ff01edcd9).
- commentary/review.json:
  - editorial_audit status "reviewed", with substantive notes;
  - it was refreshed by analysis/tools/commentary_review.py, and only summaries and source line numbers changed;
  - the scaffold, statuses and pass fields are unchanged.
- commentary/editorial-audit.md records the findings, fixes and residuals.

## Not produced
- analysis/current-audit.json and analysis/annotations.json are in the archive.py FINAL list. No documented
  command or workflow step in this workspace defines how to produce them, so they were not fabricated.
- archive.py, publish and delete were not run. The toolkit was not modified.

## Residual limitations
- **Whole-ROM classification is NOT complete.** 3 lone RTS bytes in bank 1 are still unknown: 0x02DC6 ($ADB6),
  0x032FA ($B2EA) and 0x03353 ($B343). Each has a source note. L-023 was rejected, and the terminal rejection
  review is in analysis/handoffs/round-04-redirect.md.
- 482 data regions keep only generic phase-1 notes. 343 named stubs and continuations have no header.
- Open semantic items are listed in commentary/uncertainties.md, including the meaning of HarbourReturnFlag and
  of FishState $A0, the species for each HookedFishType, the fight model and sound semantics.
- No toolkit issues were found; analysis/toolkit-issues.md is unchanged.
