# Round002 mechanical uncertainties and actionable questions

All current classified source has naming and behavioral explanation. These are
limits of interpretation, not deferred readability ranges. Classification remains
131086/131088 bytes; the two-byte interval below remains genuinely unclassified.

- Physical [6428,6430), B0 CPU990C..990D, effective slots [0,1,2,3]: A4 A4
  fits `LDY ActorScheduleCursor` before the live-Y increment/wrap/store tail.
  Existing bypass entry already establishes that tail contract but does not
  independently select the proposed prefix boundary. Constants have no established
  read consumer either. Reopen only with a discriminating entry-family/boundary or
  consumer constraint; runtime selection is not a universal prerequisite.
- B0 CPU8F87, physical3991: the score-ranking path loads RAMB0 and compares it to
  RAMB0, so its BNE cannot distinguish the two ranks. The following six-digit
  comparison still determines ordering. A bounded comparison of two equal-rank and
  two distinct-rank candidate records could clarify the behavior; no ROM patch or
  speculative instruction correction is justified. Source preserves actual mechanics.
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
  progression. The illustrated ending remains confirmed only under artificial entry.
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
