# Map-expansion data review

Physical16KiB PRG bank4 (effective8KiB[8,9,14,15]) data file$10040-$11218.
Validated consumer fixed$E722-$E7E9, especially bank switch$E724 and8-byte copy
$E7A9-$E7B1; read-only primary decode field-e722-decode.json, stage-expand-tail.json.
Three16-entry stage-selector arrays at CPU$8030,$8D7F,$91E9. Selector$38 constrained
by16-map menu and constructor; runtime map1/15/final selector experiments retained.
Data pattern is nested regular pointers, eight-byte record slabs and small indices;
accepted code/fetch has no overlapping instruction or operand. These bytes are
non-code independently of complete semantic decoding. Existing generic uniform
annotations inside span preserved rather than overwritten.

Stage0-13 pointer lists lead to exactly regular8-byte records ending at next
stage-pointer-list start; streams are countN plus3N indices, bounded by next start,
all indices within record count. Stage14 pointer-record end overlaps next pointer
list by5bytes; shared bytes/extent do not invalidate data classification, but no
exclusive object partition is claimed. Stage15 count28's first56indices fit its
22-record list, while inferred third group overlaps common pointer data at$91E9
and contains larger values. Thus a universal complete3N stream claim is rejected
for that selector. This is a local semantic/consumer-usage question, not evidence
of executable code in the densely repeated pointer/record/index content.

Final field shortcut was artificial and initialized a distinct visible field. Its
exact selector/third-group use should be checked in redirect before claiming all
final-map expansion behavior matches natural password-route initialization.
