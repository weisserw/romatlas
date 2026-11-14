# Round006 independent structural review

Accepted L010 P01/P02/P03: 123199..123229, 123239..123259,
123269..123281, 62 Static Code bytes. Physical bank15, effective
[null,null,14,15], CPU E12F/E157/E175. No functioning complete handler,
observed fetch, natural entry, relocation or obsolete-origin claim.

Independently compared immutable ROM with actor-primary.json and proposal bytes;
configured release decoder corroborated 25 instructions and exact ends without
diagnostics. Canonical123175..123197 supplies coordinate0428,Y test60,
conditional06AD,X store, test48, mask20 load, JSR and JMP layout. P01's
seven-byte test/store suffix exactly equals canonical123178..123185; its BCC
lands at123206. Each subsequent test48/BCC+5/LDA20/JSR/JMP block has exact
nine-byte branch skip to its JMP. P02/P03 start at established joins after
accepted complete coordinate-copy operations. These field identities, repeated
threshold/mask semantics and local branch topology jointly establish the code
pattern; this is stronger than legal decoding or a known destination alone.

Weaker components reviewed separately: P01 JSR/JMP123218..123224 shares its
JSR operand with the following mask40/JSR123224..123229, mirroring the canonical
mask40-call prefix that ends at a coordinate load. P02 JMP123251..123254 and
mask40/JSR123254..123259 share encoded transfer/call destinations with the
neighboring comparison and following P03. Repeated bounded instruction placement,
mask40 setup, identical encoded call family and independently accepted coordinate
load boundaries support their inclusion as instruction fragments. This argument
does not extend to independent isolated transfers or return opcodes elsewhere.
P03 ends exactly before independently classified639-byte FF Data. Other ends
meet existing Code; no Data or classified instruction is crossed.

Independently checked prior r002 raw target evidence: E0EA123130 enters06 in
STA06AD,X; E10B123163 enters20 in LDA20; E102123154 enters03 in BCS03;
E129123193 entersE0 in JSR E0D5; E120123184 enters06 in STA06AD,X.
E0E1123121 is a canonical LDA start. These targets refute the working current
helper contract, not recognition of encoded instruction fragments. They remain
unchanged and are not repaired. Stored instruction-template Data is conceivable
but has no consumer/format evidence to outweigh the unusually specific repeated
field/control family. No explanation of original provenance is needed or asserted.

Orphan123197..123199=2804 remains Unknown: missing LDA cannot be fabricated.
Other singleton RTS, isolated JSR/JMP, BIT trailer and E608 split retain the exact
localized ambiguities reviewed in r005 contracts.md; no automatic extension.
All encoded actor targets lie in classified code, so no unknown callee closure.
No new supported gameplay transition/RAM recipe follows; reuse S005/S008/S009
and failed S006/S010 evidence without unchanged replay. This is an ordinary
supported-batch pass, not terminal exhaustion. Full changed-source commentary
and redirect follow; final812-row editorial frontier remains pending.
