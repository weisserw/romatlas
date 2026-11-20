# Commentary progress — round003 current-source readability complete

Checkpoint round-003-classified; current map/state and frozen baseline SHA256
29e4403b8b27d1c3ae75a0aec4c7214f1191a434339eca88ecdb9a1fb51359c9.
The entire previous commentary is preserved at
analysis/round-3/commentary-round-002. Fresh preparation rebuilt byte-identically;
1,853 classified spans carried by exact physical interval, with prior review and
useful prose. Trailing unknown bytes were separated from their preceding classified
blocks rather than importing stale assembly. Carried source was reverified.

All current classified source is readable: 991 Code, 1,059 Data and 16 Image spans,
plus honest explanations of 24 Unknown intervals. All 2,090 scaffold entries and
1,992 compact units are reviewed. No readability range remains pending. Source has
35,391 lines. Physical coverage: 44,667 Code, 117,882 Data, 99,328 Image and 283
Unknown bytes. Structural classification remains 261,877/262,160 = 99.89%; it is
independently incomplete. Final editorial audit remains pending and was not run.

This pass explains all 37 newly classified Code spans and 176 Data spans, reuses
all 1,819 valid older units, and adds 173 compact named units. It follows scene
cancel/transition and closing-scene dispatch, grapple launch/attachment/retraction,
directional OAM segments, auxiliary room-slot initialization, directional animation
pointers, bounded arithmetic, active/type search, hexadecimal PPU output, vectors
and alternate entry ABIs. All six inline dispatch tables explain their stack
contract. Across 130 new data shelves, 310 direct same-bank/fixed-bank reads now
use stable source symbols with exact within-shelf offsets. Generic data retains
mechanical byte/pattern/boundary explanations and unresolved schema/game meanings.

Dependency corrections resolve all four formerly numeric control targets; no
numeric branch/JSR/JMP operand remains. Fixed wrappers preserve explicit MMC1
value versus effective physical bank distinctions. Concrete prose/name corrections:
D718 enters collision-world coordinate code; E8C0 clears sixteen PPU pages at
$1000..$1FFF; scene setup resets audio and steps scrolling columns; contact health
underflow enters actor-hit state; DF35 searches literal slots 11..21. Grapple fields
explain coordinate/OAM-cursor reuse, and auxiliary slot bases resolve to slots
16/17. Older conflicting unknown-target notes and rendering wording are corrected.

Uncertainties remain semantic or structural boundaries already identified in
analysis/evidence/round-003/review.md: narrative scene/actor identities, individual
resource subdivisions, the one-byte trial-remainder division limit, seven reset
prefixes, renderer/counter/trap/arithmetic fragments and the BC00 dictionary conflict.
No natural final-shot/full timed-credits claim is added. See uncertainties.md.

Final fresh commentary.py verify passed ok=true, byte_identical=true. Source SHA:
- game.asm: e055143cef984cb3d596a8f1124e8f0a61f932b6aeb6c53f576a671b86bb2b55
- symbols.inc: df5160e53f5663077376929f05becf664036fd646a46701e2113ba1202fad5d4
ROM SHA remains f5f1e212b70e858b3b0257161bc4c25a390602dacd12d5c0877a971ce8a18567.
Fresh reading-after.asm SHA:
c0f74d1aae49c3276b0b64478b295320e68fee4ffd0819a34078c45e1f302a79.
Source locations refreshed for every scaffold entry and unit. Completion facts:
analysis/round-3/commentary-completion.json; full handoff:
analysis/handoffs/round-003-commentary.md. No ROM/map/classification/toolkit edits,
archive, deletion/cleanup or publication. No new toolkit issue; existing reports
remain preserved. Disk 4.0GiB free; analysis/scratch 180KiB retained locally.
