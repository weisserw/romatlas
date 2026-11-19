# Coordinator terminal rejection review

Accepted stopping rationale: justified terminal incomplete, not ROM completion or a blocked handoff. The final verification and document-selection checks are recorded separately in the terminal handoff.

I inspected goal.md/evidence-policy.md, the round03 redirect handoff and rejection review, primary-review.json, the earlier tail-JMP/RTS comparison, current map neighborhoods, immutable ROM bytes and current renderer/tile-writer source. Both candidates were evaluated separately: physical 1CC9C (CPU CC8C) and 1CDE5 (CPU CDD5), each byte 60 in physical upper banks 14/15.

The predecessor JMP CDD6 transfers bypass both bytes. The paired renderer restores its PHA/PLA temporaries before the transfer; CDD6's tile writers perform ordinary calls/returns without rewriting a return address, and the shared row renderer returns at CDF9. This excludes a required sequential return after either jump. It does not exclude an independently entered stored RTS.

The thirteen accepted comparison RTS bytes have explicit branch references and explain return joins; that evidence does not establish an unconditional redundant-return convention for these two isolated bytes. The apparent CC8C pointer at physical 18027 is a misaligned coincidence: immutable 18026..1802A is 4F 8C CC 8E, two aligned music pointers 8C4F and 8ECC. Other entry/minus-one matches belong to independently supported graphics, metatile, bitstream or selector data. These checks close those particular pointer hypotheses, not every hypothetical computed/split producer.

Both local sprite readers use pointer CE8C plus unsigned eight-bit Y, with no pointer rewrite in the tile writers. Their possible CE8C..CF8B read domain cannot contain either singleton. This excludes the concrete neighboring sprite-payload interpretation, not all data interpretations.

The remaining alternatives are intentional redundant/stored return instructions and inter-routine padding/literals. Exact isolation and legal opcode decoding fit both; neither nonexecution nor a one-byte return supplies positive role discrimination. Generic Data would be speculative here, unlike the twelve copied literal bytes now independently supported by a counted graphics source. The old broad fourteen-byte rejection is not inherited.

I inspected the continuous-investigation dispositions for entrance extent, finite dispatches, copied code/reset contracts, container endpoints and gameplay. Their remaining semantic/natural-route uncertainties are distinguished from classification; no concrete independent producer, consumer, container bound or discriminating next test remains for these two bytes. Arbitrary execution would return under both interpretations. Representative late areas, NPC dialogue and artificial rescue/final scenes remain documented; natural boss defeat/full ending reachability and full hardware compatibility are not claimed.

The worker's review localizes negative evidence and does not impose a universal caller, execution or original-generator prerequisite. I found no corrected premise or useful revised test requiring another redirect continuation. There is no interrupted productive frontier or ready batch. Reopen on a supported entry/producer, data consumer/pattern or independent source-layout constraint. Retain both bytes Unknown and stop this workflow incomplete on the current evidence.
