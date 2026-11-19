# Redirect r002-b (round 2, continuation of r002-a) — 23:$B402 corrected inference

Input checkpoint: pass-002 (byte_identical). ROM/map/state/source/binary unchanged (sha256 checked
before and after toolkit calls; the toolkit was run only against a scratch copy of analysis/state.json,
since deleted). r002-a is not modified; its scans are reused, not repeated.

## Retry record (evidence-policy "Revisit reasoning")
- Target claim: physical 193554 (1 byte, $60), bank 23, CPU $B402, ctx prg_banks [22,23,30,31] is Code.
- Prior barrier: r002-a treated the byte as an *unreferenced* return opcode (policy example) and
  rejected it as code-vs-data ambiguous.
- Changed inference: premise correction. The byte is the static fall-through successor of the
  conditional `bcc $B403` at $B400 (phys 193552, Code region id 955). It is referenced by a
  control-flow edge of classified code. "Always taken" is a reachability property, not structure.
- Discriminating checks: (1) does the toolkit's own static decoding/discovery treat it as a successor?
  (2) does any reader or alternate entry make a data/skip-idiom reading concrete?
- Stop condition: done after these checks; no runtime work needed.

## Checks
1. Toolkit decode (configured binary, analysis.decode ctx [22,23,30,31]): `bcc $b403` at phys 193552
   has successors [$B402, $B403]. analysis.discover (use_observed=false) from NextLevelFrom4
   (phys 193529, $B3E9) emits candidate {offset 193554, length 1, Static, cpu $B402, ctx
   [22,23,30,31]} with no diagnostics. Evidence: evidence/toolkit_discover.txt,
   evidence/discover_req_A.jsonl, evidence/discover_resp_A.jsonl. The pass-001/002 "flow closure"
   (region 955 evidence) evidently pruned this fall-through; the toolkit's own discovery does not.
2. Predecessors of $B400 (tools/pred_b400.py -> evidence/pred_b400.txt): no relative branch in bank
   23 and no JMP/JSR in classified code targets $B400 or $B402. So $B400 is reached only by
   fall-through from the untaken `bcs $B407` at $B3FE, where C=0; `bcc` is therefore always taken on
   every known static path and $B402 is statically unreachable (reachability: statically dead
   fall-through; never fetched in recorded coverage per r002-a).
3. Competitors:
   - Inline data skipped by an always-taken branch: needs a reader; r002-a rts_refscan.txt found no
     abs/indexed read spanning 193554 and no pointer to it (hits only coincidental words in data banks
     2/3/4/18, imm pairs in banks 26/27 not in this bank context). No reader -> no concrete data role.
   - BIT-skip / overlapping-entry idiom ($2C/$24 hiding an operand, or a branch "skipping" a byte that
     is an alternate entry): no instruction or pointer enters at $B402 or mid-instruction; the byte
     is not an operand of a hiding opcode. Not applicable.
   - Padding/filler: single byte, no uniform run, value $60 decodes as a terminal instruction that
     closes the fall-through path cleanly before SetLevel0 ($B403). Bounds are exact either way:
     1 byte between Code 955 (ends 193554) and Code 956 (starts 193555, $B403).
   The code reading uses an independent structural fact (CFG successor of a classified conditional
   branch, as decoded by the toolkit) plus exact bounds and a well-formed terminal decode; the data
   readings have no consumer. This discriminates in favour of Code (Static).
4. Bank context: physical = 16 + 23*8192 + ($B402-$A000) = 193554; bank 23 in the $A000 window,
   identical to neighbouring regions 955/956 views (prg_banks [22,23,30,31]).

## Other two bytes (no analogous correction)
- 28:$8620 (phys 230960): preceded by `rts` at $861F (terminal, no successor). Toolkit discover from
  $8614 (ctx [28,29,30,31]) does not emit it (evidence/toolkit_discover.txt run B).
- 28:$9883 (phys 235667): preceded by `rts` at $9882; `bne` at $9880 targets $9884; fall-through of
  the bne is $9882, not $9883. Toolkit discover from PitchPhaseTest $987B does not emit it.
- Neither has a control edge, reader or pointer (r002-a). The r002-a rejection stands for both.

## Decision
- 23:$B402 -> ready lead L-017 (Code, Static; reachability: statically dead fall-through, unobserved).
- 28:$8620, 28:$9883 -> remain unknown (L-015 rejection stands for these two).
