# Round 2 — redirect phase handoff (continuation attempt r002-b)

Status: **complete; 1 ready classification lead (L-017).** Recommendation: **next: disassembly round 3**
(apply L-017; 2 residual bytes then remain unknown with the rejection review below).

- Scope: coordinator-directed re-evaluation of 23:$B402 (premise error in r002-a) plus a brief check
  that no analogous corrected inference applies to 28:$8620 / 28:$9883. r002-a and its handoff
  (analysis/handoffs/round-002-redirect.md) are preserved unchanged; its scans were reused, not rerun.
- Input checkpoint: pass-002 (byte_identical). ROM, maps/current.json, analysis/state.json, source and
  binary unchanged (sha256 verified after toolkit calls; toolkit served a scratch copy of the store,
  since deleted). Research: analysis/redirect/r002-b/ (notes.md, tools/pred_b400.py, evidence/,
  leads.before.json); analysis/leads.json (+L-017, L-015 attempt appended, pass redirect-r002-b).
- Proposed bytes: 1 (physical 193554). Accepted: pending disassembly review. Residual after L-017:
  physical 230960 (28:$8620) and 235667 (28:$9883).

## Decision: 23:$B402 -> L-017 ready (Code, Static)
- Claim: physical 193554, 1 byte, bank 23, CPU $B402, ctx prg_banks [22,23,30,31]: an `rts` instruction.
  Reachability recorded separately: statically dead fall-through (unobserved).
- Structural evidence: it is the fall-through successor of the conditional `bcc $B403` at $B400
  (phys 193552) inside Code region 955 (NextLevelFrom4). The configured toolkit agrees: analysis.decode
  gives successors [$B402,$B403], and static-only analysis.discover from NextLevelFrom4 (193529/$B3E9)
  emits candidate {193554, len 1, Static, $B402, ctx [22,23,30,31]} with no diagnostics
  (r002-b/evidence/toolkit_discover.txt, discover_req_A/resp_A.jsonl). The byte decodes as a terminal
  instruction that ends the path cleanly before SetLevel0 ($B403, Code 956). Bounds are exact.
- Reachability: no branch/JMP/JSR targets $B400 or $B402 (r002-b/evidence/pred_b400.txt), so $B400 is
  entered only by fall-through of the untaken `bcs` at $B3FE (C=0) -> `bcc` always taken -> $B402
  statically unreachable; no observed fetch (r002-a coverage).
- Competitors checked: inline data skipped by the branch has no reader or pointer (r002-a
  rts_refscan.txt); BIT-skip/alternate-entry idioms do not apply (no entry at $B402 or mid-instruction,
  no hiding opcode); filler has no run/pattern. None has a concrete consumer; the code reading has an
  independent control-flow edge. The policy's "unreferenced return opcode" example does not apply
  because the byte is referenced by a static control edge.
- Annotation recipe: see L-017 in analysis/leads.json (map.annotate offset 193554 length 1 kind Code
  provenance Static view cpu_address 46082 ctx [22,23,30,31]; evidence string included).
  Commentary: replace the '; Unknown:' note with "dead fall-through RTS (carry is clear here)".

## Updated rejection review for the remaining bytes (evidence-policy.md)
- Candidate claim: 28:$8620 (phys 230960) and 28:$9883 (phys 235667), each 1 byte $60, ctx
  [28,29,30,31], are code (dead RTS).
- Decisive evidence: r002-a rts_refscan/rtstrick/rts_idiom_census (no branch/JMP/JSR target, no data or
  immediate pointer, no indexed reader, no RTS-trick dispatcher, no precedent); r002-b toolkit discover
  (run B) does not emit either byte; each is preceded by a terminal `rts` ($861F; $9882), and the `bne`
  at $9880 targets $9884 with fall-through $9882. What it rules out: any static control edge, reader or
  observed fetch; it does not decide code vs data.
- Independent constraints together: exact 1-byte bounds between code; no edge of any kind; no reader.
  Unlike $B402, no control-flow successor exists, so the coordinator's corrected premise does not
  transfer. The remaining support is the byte value alone (policy: insufficient on its own).
- Strongest competitor: 1-byte non-code leftover/constant between routines (no reader required).
- Result: no supported proposal and no discriminating test; keep unknown. Reopen only with a new
  reference source or a policy decision accepting idiom-based classification.

## Recommendation
**next: disassembly round 3** — validate and apply L-017 (1 byte; 393230/393232 afterwards), then
commentary/redirect as usual; the two remaining bytes are expected to stay unknown under the review
above, leading to terminal incomplete at 2 bytes unless new evidence appears.
Scratch: removed; no live serve processes left.
