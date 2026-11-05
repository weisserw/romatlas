# Round 1 commentary

Current classified-source readability is complete for frozen pass-001: 15,111 code
bytes, 203,670 data bytes, and explanatory boundary notes on 43,379 unknown bytes.
This is 1,762 full-ROM scaffold entries (1,182 Code,357 Data,223 Unknown), including
213 separated physical code groups and 423 meaningful routine/control-flow units and four data-format units.
Every code byte belongs to at least one reviewed functional unit. review.json retains
the original physical scaffold and embeds unit records with exact source-label lines.
Final editorial_audit remains pending for the coordinator's later acceptance gate.

The source now has 973 bank-qualified entry/branch labels and 296 operand/field/table
aliases. Four banked renderer copies retain separate identities. Hardware registers,
actor planar fields, camera/scroll state, audio channel fields and screen/resource
pointers use consistent symbols; scratch aliases are scoped to the consumer. Frame
field offsets and unresolved table origins are explicit constants. Equates naming
unclassified callees/table origins do not assert classification or recovered extent.

Functional coverage includes level/effect culling and drawing, four-channel audio
sequence/command/modulation mechanics, screen resource/text/fade/title commands,
actor OAM flip/clipping and frame motion, inventory/pickup/health/HUD work, player
motion/probes, nested actor dispatch, collision rectangles/material subcells, demo
input, coarse map strips, scroll staging, raster IRQ and NMI/audio scheduling.
Prefix-only accepted fragments explicitly describe their missing continuations.
Generic non-code and uniform data carry boundary and uncertainty notes without
inventing layouts. Header, vector words, CHR tile planes and pause shortcut are
explained with their established formats.

Semantic consistency review corrected three draft entry locations against opcode
starts, kept a fourth draft entry in its unknown gap, corrected the camera-direction
names against dispatch (1 up,2 right,3 down,4 left), and scoped collision scratch aliases.
No map, internal structural state, ROM, frozen source or toolkit was modified.

Fresh byte-identical verification passed after the final coherent source edit:
verification.json. reading-after.asm refreshed at handoff. Source inventory checks
confirmed all code bytes covered and valid source links. No classification-complete,
full hardware compatibility or final editorial acceptance claim follows.

Unresolved semantics and concrete redirect continuations are in uncertainties.md;
R1-L03/04/05/07 remain active in analysis/leads.json. No remaining classified-source
readability backlog is deferred. Toolkit report retains two confirmed doc/API bugs,
zero suspected bugs,zero missing features and no blockers; no new toolkit issue found.
