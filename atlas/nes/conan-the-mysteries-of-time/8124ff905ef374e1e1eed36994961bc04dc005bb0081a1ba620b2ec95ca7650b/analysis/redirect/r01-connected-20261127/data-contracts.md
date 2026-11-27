# Combined structural data evidence

All claims use physical offsets including16-byte header. Mapper selections0..7 are
16KiB selections; physical8KiB contexts are [2*b,2*b+1,14,15], never [b,b+1,...].
Nothing here establishes additional execution or natural reachability.

## Stage maps, physical bank pair0/1
E2F3 reads StageMapBank (allzero), E2FF reads eight four-byte roots selected by
StageMapRootOffset0,4,...28. Width/height tables bound the rectangular layouts.
EB8? / EC03 resolves layout cell -> MutableCellIds -> two-byte metatile pointer;
F13? / F269 / F327 additionally reads palette at offset25. Thus records26bytes,
not25. stage-container-audit.json shows all8 independently delimited containers:
layout w*h -> exact multiple26 definitions -> pointer table -> next rooted layout.
Every pointer is aligned within definitions, every definition referenced, palette
values0..2; no holes. Last pointer table ends at independently established FF run
physical3B36. Bounds and content constrain each other rather than relying on
selected runtime IDs. Generic data remains a sound narrower fallback if a semantic
subtable claim is rejected: the whole container's non-code nature is independently
supported by consumer+packing+repeated tile/palette patterns.

## Counted and screen streams
CB8A reads output count word; CCBE forces1024 output; CDA7 token engine exits when
output count exhausted. Correct classes:00/80skip;01..3Frepeat following byte;
40..7Fzero;81..FFliteral low7count. See CDCD source read with no increment inside
loop versus CDEB increment every literal. Source pointer increment after repeat is
outside loop. Current commentary reversed repeat/literal classes and must be fixed.
All saved streams terminate without crossing selected16KiB bank; stage count roots
and independently selected title/inventory/terminal roots agree with packed bounds.
A partial final literal token can have unread stored suffix: propose that suffix
as data from its encoded run count; do not claim it was read. bank1 trailing FF is
one generic separator between complete last literal payload and nextbank root.
The50-byte FF bank3 tail has exact change/bank bounds and no conflicting code.

## Animation/image bank pair6/7 (mapper3)
F808 selects mapper3; F80F reads pointer at8002+2*action. First pointer81EE bounds
246 consecutive two-byte entries; every entry selects a valid five-byte sequence
with zero-duration terminal. Shared suffix atB0B0 is valid (same record alignment),
not conflicting structure. F84B image pointer has width/height, then exactly
2*ceil(width/8)*ceil(height/8) tile/attribute bytes per D787 consumer. 383 referenced
images all have nonzero sane dimensions. Union of sequence bytes and image bytes
exactly tiles81EE..B5A8, zero holes or cross-role overlaps. The independently
consumed terminal CHR root at8000 equalsB5A8; its counted stream follows exactly.
This whole-domain packing is independent extent evidence, not an assertion that
all246 action selectors occur.118 immediate producers from traced handler CFG are
additional support, not a full selector proof. If detailed layout extent is disputed,
all pointer and record/image content remains strongly generic non-code evidence.

## Raw CHR banks8..11 (mapper4/5)
D652 explicitly copies4096 bootstrap bytes from mapper4:8000. Image consumer D787
passes tile/attribute to D69? cache resolver: key=((attribute&1C)<<3)|(tile>>3),
8-bit; mapper4+(key>>7), source8000+(key&7F)*128. Key-derived group222 ranges cover
all128groups in mapper4 and groups0..93 in mapper5, exact source128-byte blocks.
The rest through mapper5:B100 is already known uniformFF (may overlap lastgroup).
E14? selects mapper5 and reads B100 counted inventory stream, which ends at mapped
FFbanktail. Every tile claim has source image pair references in chr-tile-groups.json;
no full arbitrary selector sweep or visual-only CHR guessing was used.

## Music, palettes, object fields and samples
Stage music IDs0..6 independently bound three7-entry tables at mapper2:800C/801A/
8028. All21 selected streams have note/duration pairs ending in FF first byte;
stored two-byte FF/FF markers exactly meet next independent pointer. Never claim
secondFF is read: firstFF loops immediately. Pitch wordsFB43..FBD5 plus duration
bytesFBD5..FBF5 follow known audio consumers and coherent monotonic period values.
Eight palette pointers at mapper6:BB68 select seven unique bounded high-bit-ended
streamsBA7E..BB68. Stored stage count comes from prior stage<8 loader, each start
and terminator parsed in stage-palettes.json; pointertable ends at known BB78.
Other tables in supplemental-data-proposals.json use bounded references, numeric
arrays/palettes/record patterns and exact adjacent code boundaries. Semantics not
fully recovered; proposed role is generic data rather than invented actor names.
residual-references.json includes aliases; refs to lower8000 must be interpreted
in source bank context (fixed map/graphic/music consumers do NOT refer mapper6
path table8000). The first20 mapper6 bytes are bounded4-byte path records consumed
at808C/80A0; remaining12 have non-instruction tile pattern adjacent initializer8020.

C068 has two four-byte DMC descriptors: C080/C420 and C440/CAA5. C000 decodes these
as start/end addresses; endpoint byte supplies DAC. Physical1C090..1C431 and
1C450..1CAB6 are sample data, ending immediately before NMI CAA6. C070..C080 and
C421..C440 are uniformFF padding. This independent descriptor/body/end alignment
supports full non-code classification of physical1C078..1CAB6; no extra DMC
selector-domain claim is needed. Source body mechanics and descriptor values are
primary evidence, not just absent instruction fetch.

No competing decoded code or accepted inline-word overlaps occur in proposals.json.
Independent disassembly should validate exact primitive contracts and all proposed
physical spans; generic table evidence is sufficient without recovering identities.
