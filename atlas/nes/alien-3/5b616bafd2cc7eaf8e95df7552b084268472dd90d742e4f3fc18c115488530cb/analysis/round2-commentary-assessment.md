# Round2 current-source readability assessment

Complete for actual pass-003: 30338 Code, 231814 Data, and four explained Unknown
intervals totalling8 bytes. ROM classification is still INCOMPLETE:262152/262160,
99.99694842844065% (rounded100.00%). No structural map/state changes.

The entire prior phase is preserved at round-2/prior-commentary. Its1539 classified
physical scaffold entries retain valid review credit and useful prose. Reviewed all
7227 added opcode starts by physical bank using commentary-bank*.txt and compact
listings in round2, with the independent round2 review and redirect contracts.
Renderer copies use separate bank-qualified identities; their selected instruction
slices share mechanics, while stage tails remain distinct.

Current review.json has1836 reviewed scaffold entries:1391 Code,441 Data,4 Unknown.
719 functional code units cover every code byte;69 data units describe meaningful
formats/domains. Source has1881 routine/branch labels plus68 physical data labels,
351 address/table/field aliases, and zero unresolved external consumer equates.
No current readability/naming/comment backlog remains. Generic-data explanations
and unresolved visual meanings are intentional documented limits, not pending review.
Final editorial_audit remains pending for later acceptance; no final audit ran.

Source explains screen option/configuration loops, finite mode/command dispatch,
decimal bonus arithmetic, ending/final-score continuation and corrected raster IRQ;
all actor type/state/initializer families, frame/component rendering and clipping;
player directional states, falling/reactions, support routes, weapon/effect creation,
contact and type-specific damage; mapper restore and nonlocal stack contracts.
Data notes explain eleven-byte frame headers/four-byte components, sentinel frame
sequences/replacement states, fifteen matched state domains, screen resource columns
and stream syntax, five51-byte audio command columns, interpreter stream/table origins,
launch/motion/subcell/exit tables and distinct level-bank initialization tails.

Obsolete prefix/unknown-continuation claims were revised at affected dependencies.
Known numeric operands now use consistent mechanical symbols; frame-buffer high bytes
use +1 expressions. Support-route scratch is named separately from collision-box
scratch at the same addresses. Game-specific labels remain tentative where needed.

Fresh commentary/verification.json reports byte_identical=true; reading-after.asm is
refreshed. Current maps/state/phase baseline/pass-003 are byte-equal. Exact source,
review, reading, ROM and map hashes plus coverage/source-link checks are recorded in
round2/commentary-readability-check.json. That scoped check is not the final editorial
acceptance audit and cannot mechanically certify semantic prose quality.

Residual redirect R2-L01 remains exactly1D0FD..1D100,1D4F5..1D4F6,
1D59B..1D59E,1DACE..1DACF in bank14. Current uncertainties are in
../commentary/uncertainties.md. No new execution or natural reachability claim.
Toolkit:0new findings; existing2confirmed doc/API mismatches,0suspected,0missing
features, no blockers (toolkit-issues.md). ROM/toolkit unchanged; no archive,
publication, deletion or cleanup of history/cited evidence.
