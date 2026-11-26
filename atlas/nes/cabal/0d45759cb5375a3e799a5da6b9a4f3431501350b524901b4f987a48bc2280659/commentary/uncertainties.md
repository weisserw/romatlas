# Round003 mechanical uncertainties and actionable questions

All 19412 Code bytes and 111676 Data bytes have current-source naming and behavioral
explanation. There are no Unknown bytes or deferred readability ranges. Final editorial
acceptance remains separate.

- Physical [6428,6430), B0 CPU990C..990D, slots [0,1,2,3], is now Static Code.
  The independently anchored $98BC family, terminal JMP boundary and actor-loop
  continuation support the cursor-load prefix. Actual entry/natural reachability is
  unobserved. BCC/BNE equivalence requires 1<=count and cursor<count; special
  overrides violate the global bound (count6/cursor8 yields BCC0 versus BNE9).
- B0 CPU8F87 self-comparison remains literal. Bounded reset-state cases confirm
  lower-score rank adjustment for equal and distinct ranks: 1/1 -> 1/2 or 2/1,
  0/3 -> 0/4, 3/0 -> 4/0. Saved insertion ranks are recomputed at $81EC/$81FD.
  Intended design remains unresolved; no patch is implied.
- Banked BRK convention is now resolved structurally: X service, Y bank, saved PC
  minus two followed by RTS resumes at BRK+1; no inline selector is consumed.
  FFA2 saves mapper state, each directory resolver calls RAM00's patched JMP, and
  FFC4 restores the physical bank before FFC7/FFC9 restore shadow/return. Return
  claims require balanced service stacks and any documented wait prerequisites.
- Natural Start gate research and the corrected NMI semaphore premise are retained
  in round001-a contracts. Later scenes/boss combat still lack the full renderer/
  actor-initialization prerequisite despite targeted timing/relocation/bypass trials.
  Do not repeat unchanged state patches. `RunBossSequence`, boss component records
  and scene animation channels explain mechanics without claiming successful natural
  progression. The illustrated ending and subsequent BEST 5 PLAYERS scoreboard are confirmed
  only under supported artificial continuations; natural finalboss victory is unresolved.
- Six-byte player score arrays, ten-byte ranked score records and three-byte name
  buffers now have coherent static interpretations from comparison/insertion/render
  consumers. Actor identities, many exact music/effect selections and some actor
  state nibbles remain unknown; mechanical array/mode names are intentionally used.
- Signed RLE source-count conventions differ by bank. Bank1 separately tracks output
  count and tests exact source-count zero; banks2/3 use their distinct borrow path.
  Do not infer compressed source extents from PPU output length.
- Mixed graphic/audio/text/record bodies retain sound bytewise and consumer evidence
  from round001 plus round002 residual review. The disproved E05A data span is now
  a named service routine, and its original data flanks remain separate. Generic
  blocks are not claims of single assets; original asset subdivisions remain unknown.
- NMI/reset/service tail bytes are physically distinct in all four images. After a
  mapper write the next fetch uses the destination image; nonzero-image fallback JMPs
  must not be mistaken for the normal same-image NMI continuation. No ROM upload
  origin is inferred from executable RAM alone.

Relevant preserved evidence: analysis/evidence/round002/review.md,
return-audit.json, cursor-primary.json; analysis/redirect/round001-a/contracts.md;
analysis/round-002/commentary-round001/uncertainties.md for superseded premises and
historical questions. No new structural classification or runtime research occurred
in this phase. No toolkit issues found.

Round003 revisions use analysis/evidence/round003/review.md and
analysis/redirect/round002-a/contracts.md, family-contract.json and score-cases.json.
Superseded notes remain in analysis/round-003/commentary-round002.
