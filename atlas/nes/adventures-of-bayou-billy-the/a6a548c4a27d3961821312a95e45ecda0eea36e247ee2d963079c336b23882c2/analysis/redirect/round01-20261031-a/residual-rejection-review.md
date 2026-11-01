# Localized residual review — not a terminal workflow assessment

The accepted map still has 110796 unknown bytes. The combined proposals would
leave only the five intervals below (7 bytes). This review concerns those narrow
subclaims; it does not reject the surrounding connected routines or data pools.
It is not a reason to skip independent disassembly, commentary or another redirect.

Primary bytes and nearby instructions were retrieved from immutable ROM reads,
`small-residual-inspection.json`, and the final `brawl-graph.json`,
`driving-graph.json`, `driving-render-graph.json` and `fixed-graph.json`.
`proposal-residual-final.json` and `final-audit.json` preserve exact counts/context.
All full finite dispatch tables and fourteen E116 sites were inspected. A missing
natural caller is not itself a rejection criterion.

| Physical interval (half open) | Decoding view | Actual evidence and its limit |
| --- | --- | --- |
| $C1E2..$C1E5 | $81D2, slots [6,7,14,15] | Bytes decode as JMP $E84B, the known fixed-bank game-over route. The preceding stage-restart helper ends with RTS at $C1E1; the following completion helper starts at $C1E5. No accepted/proposed control edge or bounded dispatch entry selects $81D2. Legal JMP decoding and its valid destination support a stub hypothesis, but do not independently establish that these three stored bytes are instructions. A small constant sequence remains a competing interpretation; generic data is also not proved. |
| $C7A6..$C7A7 | $8796, [6,7,14,15] | A single $60 lies between an ended helper and indexed constants. The following 24 constant bytes are independently supported and proposed separately. $60 could be RTS or a scalar constant; neither role has independent support for this byte. It is not rejected merely because unexecuted. |
| $14379..$1437A | $8369, [10,11,14,15] | A lone $60 between complete local code units. There is no remaining instruction/stack contract spanning this byte. RTS decoding alone does not establish a routine or continuation; an unused constant/padding byte is a concrete alternative. |
| $1ACD0..$1ACD1 | $ACC0, [12,13,14,15] | A lone $60 adjacent to HUD/palette helpers. All supported local calls were followed. The byte does not complete an otherwise open instruction or a demonstrated shared-return path. Code and non-code remain materially ambiguous. |
| $1EE00..$1EE01 | $EDF0, [null,null,14,15] | A second isolated $60 follows the sprite-zero-split routine's established return and precedes the road-profile entry. Duplicated return bytes alone do not establish a replicated routine template. Static code and generic data are both insufficiently constrained. |

Artificially entering any of these addresses would only test the decoded mechanics;
it would not discriminate intended code from stored constants. Repeating ordinary
late-game play without a specific selection/entry hypothesis has diminished prospect.
A new bounded table/entry/stack constraint or a larger independently identified
stored template could reopen a claim; no such actionable method remains in this
redirect's retrieved evidence. They remain unknown rather than being forced into
code/data to satisfy a numerical completion target.

Two earlier provisional rejections were corrected before stopping:
`localized-reassessment.json` combines the parallel menu timer initializers to
support $1E0D7's two-byte prefix, and the complete screen/target-reset continuation
to support $102B3's three-byte call. Both are now Static Code proposals despite
unknown natural entries. This localizes the actual insufficiency to the five
intervals above and avoids treating a caller as a universal prerequisite.
