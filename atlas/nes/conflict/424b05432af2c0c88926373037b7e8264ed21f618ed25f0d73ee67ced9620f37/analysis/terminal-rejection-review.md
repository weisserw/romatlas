# Coordinator terminal rejection review

Accepted pass-005 retains twelve Unknown bytes in three half-open physical intervals.
This is an incomplete ROM classification and a justified stopping decision, subject
to final editorial/verification acceptance. Read the primary-linked redirect review
at redirect/r5-terminal-review/rejection-review.md and its exact complement proof.

I independently inspected immutable bytes and current source at both signature and
footer boundaries, including C048/C04F consumers, E000 veneers, FFD7 bank wrapper,
FFDC call, title and interrupt vectors. Signature consumers establish only 01/29;
the next C0 can be an opaque extension or overlapping instruction prefix. Extending
the two-byte signature lacks positive role evidence. The 60 after the outcome call
fits conventional dormant RTS closure, while also fitting a printable footer prefix;
reset-sink evidence rejects a fetched return only. Neither adjacent title membership
nor the instruction's legal stack behavior independently distinguishes those roles.
The ten numeric bytes have exact title/vector bounds, but no coherent self-contained
routine or positive data representation: decoding crosses the vector boundary.
Trailer placement favors data without resolving the competing truncated fragment.
These are localized role insufficiencies, not mandatory natural callers, consumers,
original generators, schemas, or semantic labels.

Compared with accepted dormant NOPs and cleanup jump, these candidates lack the
combined copied-role homolog/vector origin/filler boundary or repeated meaningful
peer-arm semantics. No universal homolog rule is inferred. Earlier false no-read and
wrong-jump-target premises were corrected and classified; remapped fetch was correctly
separated from copied code structure. No useful revised inference/test survives this
review. Arbitrary PC entry or unchanged gameplay would not discriminate the roles.
Saved broad gameplay includes final fields, tactical/world commit, production birth,
defeat, default ending and credits/END/reset, with explicit artificial provenance.
No practical untried representative milestone is identified. Reuse the sound review
while premises remain unchanged; no further empty phase cycle is warranted.

Residuals: 0x1E00F length1; 0x1FFEF length1; 0x20000 length10.
Map e7d6e4a1b6e7fa1bf3e42d5cd6ecfdf37d94df7758bf4f52b83129957346bc23.
