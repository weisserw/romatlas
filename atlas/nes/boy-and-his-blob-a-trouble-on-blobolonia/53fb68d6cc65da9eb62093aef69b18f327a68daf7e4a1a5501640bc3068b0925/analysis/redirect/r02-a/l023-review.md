# L-023 review — bank 0 $9AB8 (physical 0x1AC8, 1 byte $60)

Attempt r02-a. Input checkpoint pass-003 / commentary-002. Static analysis only (no map edits).

## Claim under test
Physical 0x1AC8 = bank 0 CPU $9AB8, context prg_banks [0,1,14,15] (bank 0 at $8000-$BFFF, the
context of every caller of its neighbours). Candidate role: **Code — an unreachable second RTS that ends
UpdateBoyStop ($9A93-$9AB7)**, Static provenance, no reachability claim. Competing role: a 1-byte non-code
remnant (data/filler).

## Prior barrier (round-2 disassembly, analysis/pass-003/verify.log; round-1 redirect ledger)
No JSR/JMP/branch target, no $9AB7/$9AB8 word, no LDA #hi/#lo + PHA pair. Decision: "a lone return opcode is
insufficient for code and nothing supports data". That rejection evaluated reference evidence only. What it
rules out: a *live* reference. It did not test the competing non-code interpretation or the local
source-layout constraints; those are combined below (corrected premise: reachability is not required for a
structural Code claim, evidence-policy.md "Evaluate the claim actually needed").

## New independent constraints (r02-a, scripts + logs in this directory)
1. **No data consumer can address the byte** (absrefs.py / absrefs.log): every absolute operand in bank-0
   and bank-7 code pointing into $9A00-$9AC0 is either a JSR/JMP to a routine entry ($9A93, $9AB9, ...) or
   an indexed read of BlobAnimNext/Len/State ($9A84/$9A89/$9A8E,x; index = BlobWalkState, 5-entry tables).
   No `#$9A` immediate exists in any code region (no pointer construction to page $9A). No $9AB7/$9AB8
   word anywhere in PRG (rtsrefs.py). Coverage exports (pass-001 natural play, r01 pharmacy/treasure/ending;
   cov9ab8.log) record 0x1AC7 and 0x1AC9 thousands of times and 0x1AC8 never (neither fetch nor read).
   This excludes table/operand data with any discovered consumer; it does not exclude unread data.
2. **No fill/alignment convention fits** (Padding survey in handoff): bank 0's only padding is the
   171-byte $FF bank tail $BF14; all ROM padding is $FF. $9AB8 holds $60, and the next routine starts at
   the odd address $9AB9 (no alignment boundary). A filler byte would break both conventions.
3. **Source-style precedent** (rtsrts.py / rtsrefs.log): the ROM contains 16 other RTS,RTS pairs inside
   code (13 in bank 0). In every one the second RTS is a separate label (branch/JMP target or jump-table
   entry, e.g. $AEB4 via 4 BNEs, $9ADC via SetScripts, $863A via PickupHandlers) — i.e. this author
   routinely writes `...; rts / Label: rts`. $9AB8 is the only instance whose label lost all references
   (the branches in UpdateBoyStop target the first RTS $9AB7). An orphaned `Done: rts` is a common
   source-edit artefact; a stray one-byte data value equal to the RTS opcode between two routines has no
   analogue in this ROM (all 10 small bank-0 non-code gaps between code regions are read by indexed
   operands in bank-0 code; gapcons.py / gapcons.log).
4. **Bank-0 layout is contiguous code/data**: neighbours are Observed code ($9AB7 RTS, executed;
   $9AB9 LandOnFloors entry, executed) — the byte is enclosed in a code run, not at a table boundary.

## Strongest competing interpretation
A deliberately placed or leftover 1-byte data value ($60) without a consumer. Nothing positive supports
it: no consumer, no fill convention, no alignment, no table adjacency (BlobAnim tables end at $9A92 and are
followed by Observed code $9A93-$9AB7). It is not excluded logically (unread bytes can be anything), but it
needs an unsupported assumption whereas the code reading is supported by (3) + (2) + (4).

## Assessment
Combined constraints (1)-(4) discriminate an orphaned RTS from data/fill better than the lone opcode
alone; this is the "newly combined independent constraints" case of evidence-policy.md, not a renamed
retry. Proposal (lead L-023, ready, low confidence 0.6): annotate 0x1AC8 len 1 as Code, Static,
label e.g. `UpdateBoyStop_OrphanRts`, comment "second RTS after UpdateBoyStop; no reference (orphaned
`Done: rts` label, 16 analogous RTS,RTS pairs in the ROM all have referenced second RTS); unreachable as far
as known — no reachability claim". Merging with region $9A93-$9AB7 is optional.

If disassembly review judges (3) insufficient (a style precedent is not a structural proof), the
remaining reason is exactly: no evidence source can observe intent for an unreferenced, unread byte;
the byte then stays unknown and the ROM stops incomplete at 262159/262160 with this review as the
terminal rejection record. No further experiment can change this (no consumer exists to execute).
