# Round 02 source uncertainties

Current-source readability is complete. These are explained limits, not deferred
source ranges or a claim of whole-ROM completion.

* Physical $00CBDF..$00CBE0, effective bank6 CPU$8BCF, byte$60, remains unknown.
  The preceding sound continuation returns/jumps and the next routine begins$8BD0.
  No supported entry or finite data structure discriminates the byte's role.
  Reuse [the primary assessment](../analysis/redirect/r01-consumers-20261105/residual-assessment.md)
  and [independent review](../analysis/round-02/review-notes.md) for redirect;
  arbitrary execution of RTS would establish no new storage role.
* Sound command-directory layout has96 cells, including RAM$0088. This finite
  storage boundary does not establish that every command naturally occurs. Stream
  call/return/random-call/wait contracts are localized in source. Tail-jumping
  substreams do not imply a returning parser continuation after a call.
* DMC start alignment and length subtraction use different values. Sample7's
  actual hardware payload ends at$E0D1; the following47 bytes are separately
  inspected generic data. Matching unselected descriptor patterns are not proof
  of additional sample use.
* The final-event control-reader copy preserves its omitted selector reload and
  sequence increment. It is not silently repaired to match the bank-two utility:
  a zero sequence field can loop indefinitely. A useful/natural caller remains
  unestablished. Literal carry-dependent sound attenuation and duplicated APU
  writes likewise remain unchanged and explained.
* Generic data uses per-range inspected patterns/references and established
  neighboring boundaries; original subtable schemas, fill purpose and complete
  natural selector domains remain uncertain where the evidence does not supply
  them. Motion paths consume ten packed coordinates at eleven-byte storage
  stride; the last byte has no invented meaning.
* Actor fields and event-local RAM have mechanical aliases; exact gameplay names
  for some phases, response indices and score units remain tentative. CHR has
  the established NES planar tile format, but complete artwork identities are
  not recovered. These meanings are not prerequisites for source readability.

Prior uncertainties and investigation history are preserved without alteration in
[the entire earlier commentary](../analysis/round-02/commentary-round-01-preserved/)
and [round01 handoff](../analysis/handoffs/round-01-commentary.md). Inline-record,
palette, description and source-container structural questions from that pass
now have accepted consumer-backed boundaries and source explanations.
