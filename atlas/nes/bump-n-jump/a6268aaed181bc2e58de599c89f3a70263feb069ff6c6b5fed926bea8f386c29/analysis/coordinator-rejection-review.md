# Coordinator rejection review

The round-3 terminal research assessment is accepted as justified incomplete,
subject to final editorial and verification acceptance. This is not whole-ROM
classification completion. No new disassembly cycle is justified by unchanged
premises. Final checkpoint: pass-003-reviewed; 65,541/65,552 classified, 11 unknown.

I inspected [the worker's reasoning](redirect/round-3/findings.md),
[decisive source excerpts](redirect/round-3/primary-source.txt), the
[preceding corrected inference](redirect/round-2/findings.md), and independently
read the relevant literal bytes from input/game.nes. Fixed PRG context is
[0,1,2,3]; physical offsets include the 16-byte header. The map, state and
checkpoint share SHA256 90c4126c01fcd06e3dc687064fda1288f02ec1e022ec37049170ffe359a75c16.

| End-exclusive physical range | Claim assessed and decisive limit |
|---|---|
| $007C..$0080 | `4C 00 80` can jump to genuine Reset; the separate zero can be BRK or stored data. The real destination and clean neighboring boundaries do not distinguish a transfer fragment from stored bytes. No field, independent consumer or repeated data pattern supplies that distinction. |
| $0238..$023C and $1219..$121A | `20 09 92 40` and the second `60` support a mechanically balanced interrupt shell. I considered the combined shell/return interpretation, not just absent vectors. Its operand and target describe one possible call relation; the independently bounded eight-entry NMI table selects $9208 rather than $9209. Adjacent duplicate RTS bytes alone leave code versus data contested. |
| $049E..$049F | The isolated `40` has no independently supported interrupt-frame relation. The accepted following controller conditional starts at $848F and needs no interrupt frame; its proof does not extend backward. |
| $3B8A..$3B8B | The singleton zero follows an unconditional parser jump. Pointed audio streams are elsewhere, and no inline-data/read contract or repeated pattern covers this byte. That excludes neither stored data nor BRK. |

The review does not require an actual caller for static code. R2-001 was reopened
and accepted when corrected BIT-Z semantics combined an established input field,
exact conditional default-course join and downstream consumer. That discrimination
does not extend to the remaining transfer-only fragments. Accepted arithmetic
helpers likewise have independently meaningful field and finite-control contracts.
Generic data was considered separately from identifying a specific format; here
the code-versus-data decision itself remains ambiguous. Artificial entry would
exercise assumed decoding and would not discriminate the competing claims.

The review localized counterexamples and did not propagate a failed whole-range
inference to supported subintervals. The first RTS at $1218 and conditional code
at $049F..$04A3 remain accepted independently. I found no corrected premise,
narrower supported interval or useful discriminating experiment to return to
redirect. Broad progression and unvisited-milestone triage are documented in
[runtime results](redirect/round-1/runtime-results.md) and scenarios.json;
repeating natural defeat, landing or ordinary ending timing supplies no identified
residual structural discriminator. These limits are not a hardware compatibility
or natural full-playthrough claim.

Independent ROM reads confirmed the residual literals and the vectors at physical
$800A..$8010 (`F9 81 00 80 00 80`). The pass-001 handoff's `$7FFA` physical-vector
reference omitted the header; that historical record remains preserved, with this
correction. Input SHA256 remains
a6268aaed181bc2e58de599c89f3a70263feb069ff6c6b5fed926bea8f386c29.

Final acceptance must still verify current-source editorial review, rebuild,
reading copy, portable map validity and document selection. No archive or cleanup
is authorized or performed.
