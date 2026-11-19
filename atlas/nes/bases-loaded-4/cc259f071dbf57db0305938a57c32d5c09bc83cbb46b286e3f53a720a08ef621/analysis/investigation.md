# Investigation ledger

## Round 1 disassembly (2026-11-18), checkpoint pass-001

### Platform/bank model (supported by static code and all observed contexts)

- MMC3, PRG mode 0 (writes to $8000 set $80|$06/$07, never bit 6), so the fixed 16 KiB
  unit at $C000-$FFFF is physical banks 30/31.
- PRG is organized as 16 KiB units (even bank N mapped at $8000 + bank N+1 at $A000).
  Byte 0 of each unit is its id (= N). The far-call routine `FarCall` $C862 (bank 30)
  reads the caller's unit id with `lda $8000` and writes R6=$3B, R7=$3B+1. It then loads
  the target from the unit entry table `lda $8001,x / $8002,x` (x=$3A) and jumps with
  `jmp ($3A)` at $C8D2. Afterwards it restores the caller pair. Reset requires
  `$C000 == $1E`.
- All $8000/$8001 PRG-select writes are in fixed code ($C249-$C304 CHR/IRQ, $C5F2,
  $C879-$C8C0 far call, $FDF1-$FE5A init, $FF3F-$FF76 CHR sets). Switchable units
  never write the mapper, so unit code always runs with prg_banks [N,N+1,30,31].
  All 17.3k observed contextual roots satisfy this (0 violations;
  checked against traces/round1-observed-contexts.json during the pass).
- Dispatchers `JumpTableDispatch` $C734 and `JumpTableDispatchZp` $C617 take index A,
  pop the return address and jump through the inline word table that follows the
  `jsr`. Their callers never fall through.

### Method (scripts preserved in analysis/round1-tools/ as a method record)

The scripts expect a scratch/ working directory, with scratch/obs_ctx.json equal to
traces/round1-observed-contexts.json and a live `run.py serve` relay. The scratch
directory was removed at handoff.


1. Observed execution: traced scenarios S01-S08 (analysis/scenarios.json);
   17,278 distinct ROM instruction starts with effective contexts.
2. Static traversal (trav.py/tables.py). 6502 official opcodes only. Follows
   branches, jsr/jmp abs and fallthrough in the unit context. It stops after rts/rti/
   brk/jmp, after branches whose condition is fixed by a preceding immediate load or
   clc/sec (tracked through non-flag instructions), after complementary branch pairs,
   and after `clc/adc #imm; bne`. It does not continue after a jsr to a callee that does
   pla/plp/tsx in its first 8 instructions (stack-manipulating/inline-argument
   routines). A straight-line run that reaches an undefined opcode is discarded, with
   its successors. Roots:
   - the reset, NMI and IRQ vectors;
   - unit entry-table words proven by an exact far-call site
     (`lda #idx; sta $3A; lda #unit; sta $3B; jsr/jmp $C862`, farcalls.py), by
     observed execution, or by a closed table (the table ends exactly at its lowest
     target);
   - closed inline jump tables after `jsr/jmp $C734|$C617`;
   - observed instruction starts;
   - jump stubs $FF82/$FF91/$FFDD.

   Final traversal errors: none. Code overlaps: none. Data/code overlaps: none.
3. Specific data (Static): unit id bytes, entry tables, 46 closed inline jump tables,
   vectors, the DMC sound map ($CECC, 32 entries bounded by `cmp #$20`), the DMC sample
   table ($CEEC, 10 records) and the DPCM sample bodies ($D5CE-$FD40, ranges derived
   from the table: addr=$C000+v*64, len=v*16+1). The first 14 bytes of the $D5C0
   sample are executed code and stay code. Also: the CHR bank-set table $FF25, the
   "BASES LOADED 4" string $FFE0, the $FF tails of the odd banks, the iNES header and
   CHR ROM (16 x 8 KiB Image regions).
4. Generic data (Static, build.py/clean.py), applied only to gaps left after 1-3:
   - (a) a gap of at most 1 KiB whose first byte is the base operand of an absolute
     data access from classified code (4,607 bytes);
   - (b) uniform runs of 16 or more identical bytes;
   - (c) runs of 32 or more bytes where no linear 6502 decode passing through any byte
     lasts 20 bytes (or reaches rts/rti/jmp after 3 or more bytes) without an
     undefined opcode or BRK. All ~57 KiB of classified code contains no undefined
     opcode and no BRK, so this content is visibly non-instruction.
   - (b)+(c) cover 141,368 bytes. Spot checks show sprite/OAM-like 4-byte records,
     tile/attribute maps and score/text tables.

### Results

- ROM classification: 94.08% (369945 / 393232 bytes classified; 23287 bytes unknown).
- Code 57,337 bytes. 34,437 are Observed (every instruction fetched in a scenario) and
  22,900 are Static (structural; reachability unproven).
- Data 157,012, Padding 24,524, Image (CHR) 131,072.
- Per-bank counts: analysis/progress.md.

### Leads (analysis/leads.json)

L001 non-closed inline jump tables; L002 residual coverable runs inside data areas
(~23 KiB); L003 unit 28 entry 7 points to data; L004 unreferenced bank 30 gaps;
L005 season/ending RAM shortcut; L006 bank 31 small residues; L007 bank 0/2/4/6
shared code reachability; L008 (deferred) manual fielding stall.

### Rejected/limited hypotheses

- Treating every unit entry-table word as code was rejected. In unit 28, word 7 = $84D8
  decodes `00 02 03 05...` (BRK/KIL) and no far-call site uses index $0E. Entry words
  are code roots only when proven (far call, observed or closed table).
- First-pass inline-table parsing that accepted non-closed tables produced a
  target ($E520) decoding into an undefined opcode. Non-closed tables are now excluded
  (L001).
- Using ROM `Read` trace events to find data reads was abandoned. On this adapter the
  Read events include opcode/operand fetches, and a 64 MiB stream filled within
  seconds (238k records). This method would need fetch-subtraction against coverage
  and is deferred.

### Unresolved questions

- Exact role/consumers of most generic data (graphics/nametable/sprite/record layouts).
- Natural reachability of the 22.9 KiB of static-only code (notably units 0-6 shared
  code, unit 10/12/14 secondary paths).
- Season bookkeeping and the ending (not visited).

## Round 2 disassembly (2026-11-18), checkpoint pass-002

- Input pass-001 (23,287 unknown) + redirect r1-a leads L009-L015. All leads re-validated
  independently (analysis/round2-tools/check.py) and applied (annotate.py). Result: 0 unknown,
  pass-002 verified byte-identical.
- Independent checks: exact coverage of the unknown complement; per-region code closure
  (decode end, targets, fall-off, operand space) incl. L015; all-code control-flow scan into
  proposed data (none); inline jump-table words are instruction starts; L015 index bound
  re-derived from $AC8B/$ACA2/$ACC2; PRG/CHR checksums recomputed; hand inspection of all
  A-style/A2 L014 runs and all 16 L013 ranges.
- Rejected: none. Weakest accepted: L011 $FD41-$FD4D (generic data), L014 A2 tier.
- Remaining work is editorial (commentary rebase on pass-002, final audit) and optional
  reachability/semantics; no classification unknowns remain.
