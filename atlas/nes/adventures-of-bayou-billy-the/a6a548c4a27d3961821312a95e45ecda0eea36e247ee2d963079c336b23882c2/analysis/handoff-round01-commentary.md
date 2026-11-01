# Round 01 commentary handoff — current-source readability complete

Classification remains **57.74%: 151364/262160 bytes**, with **110796 unknown**.
No map/state/ROM classification or evidence changes were made. Baseline remains
pass-001-final; maps/current.json SHA256
687e9f29ec9f58f16b94a00c952536b7f9464a679019f8d37fe13100509a097b.
This is a successful commentary handoff, not whole-ROM or final editorial completion.

All current classified source was inspected: **15706 code bytes / 7527 instruction
regions**, **4586 data bytes** and **131072 CHR bytes**, plus local notes at all
**273 unknown intervals**. Source now has **350 curated code routine/control units**,
**14 header/vector/CHR/inline-table units**, **1194 routine/branch labels** and
consistent common/mode-specific RAM, object, hardware, controller and OAM symbols.
All 7821 immutable scaffold records carry review/source locations; there are
**no remaining current-source readability ranges**. The 364 logical units may span
unknown gaps but explicitly do not classify or establish fallthrough over them.

Buildable local source: commentary/source/game.asm and symbols.inc.
Refreshed reading copy: commentary/reading-after.asm.
Curated mechanical interpretations: commentary/research/round01-units.txt.
Scope/inventory check: commentary/readability-round01.json.
Review: commentary/review.json; prose progress: commentary/progress.md.
Frozen before/, baseline.json and phase1-state.json preserved.

Examples: return-address-consuming E116 dispatch and eleven inline data barriers;
six-channel audio ownership/stream/pitch/envelope/DMC mechanics; packed-decimal
software arithmetic; brawl fixed-point movement, terrain nibbles and attack slots;
shooting hit rectangles; driving projection/road/steering/projectile physics; PPU
queue/RLE and palette formats; MMC1 bank stack, NMI and cycle-sensitive raster paths.
Descriptions distinguish exact mechanics from tentative game identities. Raw work
operands remain where roles are scratch/overlaid or exact game meaning is unknown;
local descriptions explain those mechanics without inventing field identities.

Fresh final `commentary.py verify` passed **ok=true, byte_identical=true**, followed
by `commentary.py read`. Evidence: commentary/verification.json. Final source hashes:
- game.asm: 21dbc6b36b0068bc24f2822273a5a508f8dc37bc8c120148bc4feae31c135c26
- symbols.inc: 2ed38d06041b22f726deed90328bc04154c7ae795b807f3e25760cee70868133
Input ROM SHA256 remains the workspace identity a6a548c4a27d3961821312a95e45ecda0eea36e247ee2d963079c336b23882c2.
Final editorial_audit is pending by contract; no final audit was invoked.

Concrete new redirect question **C01**: bank10 physical **$14010..$1401D** / CPU
$8000..$800D is Static Code with unusual NOP/ORA/SLO decoding, while actual driving
entry is $800F. $80D7 reads indexed constants at $8009/$800C, overlapping this prefix.
Inspect the accepted static predecessor and effective slots [10,11,14,15] across
mapper/call contracts; compare against this consumer before any classification fix.
Do not reject code solely because of unofficial opcodes or claim a whole table from
a few reads. Source flags DrivingEntryUncertain; full primary records and bounded
recipe are in commentary/uncertainties.md. No classification change was made here.

Continue redirect on C01 plus existing R01-L03 (ending/text and terminal credits),
R01-L05 (audio/sprite/level data bounds) and R01-L06 (mirrored reset footers).
Unresolved meanings include audio instrument/stream domains, table extents, sprite
identities, terrain bit meanings, driving item flags and RAM raster upload origin.
Natural/artificial gameplay claims remain unchanged in scenarios.json; no new runs.

Toolkit: existing **1 missing feature NES-001**, **0 confirmed bugs**, **0 suspected
bugs**, no blockers or new findings. See analysis/toolkit-issues.md. No toolkit
changes, final archive audit, publication, deletion or cleanup performed. Free disk
was 9.7 GiB and scratch 696 KiB at the final consistency check.
