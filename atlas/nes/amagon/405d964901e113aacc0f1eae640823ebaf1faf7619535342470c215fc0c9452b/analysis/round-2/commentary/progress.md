# Round 1 commentary — active

Input: maps/round1-disassembly-final.json and commentary/baseline.json.
Preparation verification passed byte-identical. Complete first source reading done:
audio bank 8, graphics/UI/collision banks 10–11, scene/boss dispatch banks 12–13,
and fixed banks 14–15. No classifications changed.

Established mechanical groups include seven 32-byte audio voices, sign/magnitude
object velocity integration, metatile commands, bank-preserving wrappers, graphics
upload lists/descriptors, main-state transitions, object AI and animation dispatch.
Naming/comment edits and inventory recording are in progress across all groups.
Remaining: complete source symbols and comments, inspect consistency, fresh verify,
refresh reading copy and compact handoff. Final editorial audit remains pending.

Structural question: fixed-bank $fed8/$ff2d classified instructions resemble
metatile payload and include unofficial opcodes/KIL. Preserve classification here;
redirect should inspect discovery roots/context and compare consumer provenance.

## Source batch applied

All 5,239 classified instruction starts now covered by 260 mechanical routine
and control-flow group explanations. Added 941 CPU/RAM/hardware symbols with
explicit addressing widths preserved. All 1,552 scaffold entries retain storage
partition; source inventory recorded. Fresh verification after this batch PASSED
byte-identical (game.asm hash 62a1710aa58e7490ee1d53c06be07f356b9a70232048b924406dbdb480f1f908).

Covered physical code spans: audio $10010..$10886; banked UI/interface/collision
$14010..$165aa; scene-bank interface/handlers $18030..$1bb0e; fixed-bank classified
fragments $1c010..$1fffA. These envelopes include unknown gaps, which are not
claimed as decoded code. Actual partition remains review.json and baseline map.
Remaining before handoff: inspect unmatched comment anchors, ensure every unknown
slice has an explicit boundary note, expose established graphics records as fields,
check source aliases/prose consistency and refresh verification + reading copy.

## Current-source readability complete — final handoff

All classified source covered: 10,873 Code bytes (5,239 instruction starts), 52,000
Image bytes and 2,934 Data bytes. All 1,552 scaffold records and 265 compact
routine/structure units reviewed. Every one of 193 unknown slices has an explicit
physical boundary/uncertainty note, covering 65,281 bytes; no unknown classification
was fabricated. Source names distinguish CPU bank aliases from physical storage.
The first batch's four unmatched group anchors were checked and explained directly
at their actual instructions. Final consistency included shared RAM aliases,
object field arrays, correct fixed-bank context and exposed selector/descriptor/vector
fields. No classified-source readability ranges remain deferred.

Fresh final verification PASSED byte-identical; commentary/verification.json records
ROM SHA256 405d964901e113aacc0f1eae640823ebaf1faf7619535342470c215fc0c9452b,
game.asm SHA256 7bad001a4dbb336ea4de3ebe9cf2bfd899ed0e77fce4e97a76028399b7c9d0f0,
symbols.inc SHA256 fa3330160c2ce80789b2c4c373fd7f948a62b398296e9a8d3a64091b0c46b6d9.
reading-after.asm refreshed after that verification. Review locations refreshed;
editorial_audit remains pending and was not run. maps/current.json still equals
maps/round1-disassembly-final.json exactly. Classification remains 50.20%.

Priority redirect finding: wrong byte-order NMI premise led to a suspect static root
at $FED8; true NMI is $D8FE. Exact root/annotation evidence and narrow affected
99-byte ranges are in uncertainties.md and analysis/round1-commentary/vector-root-review.json.
Preserve existing evidence; independently review those ranges before reclassification.
Other unresolved semantics are local mechanical limits, not skipped readability.

Toolkit report: NES-001 confirmed documentation/API mismatch, zero blockers, no
new toolkit issues. No toolkit changes, archive/audit/cleanup/publication performed.
Disk check: 8.9 GiB available; commentary 6.6 MiB, phase notes/scripts 76 KiB;
no workspace-root scratch directory exists. Compact handoff:
analysis/round1-commentary-handoff.md.
