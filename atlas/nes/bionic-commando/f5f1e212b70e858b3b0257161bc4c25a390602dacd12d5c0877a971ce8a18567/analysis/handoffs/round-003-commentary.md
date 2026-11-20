# Round003 commentary — full current-source readability complete

Current checkpoint **round-003-classified**. Frozen commentary baseline and current
map/state SHA256 **29e4403b8b27d1c3ae75a0aec4c7214f1191a434339eca88ecdb9a1fb51359c9**.
Classification unchanged: **261877/262160=99.89%**, **283 Unknown bytes/24 intervals**.
Current-source readability complete; whole-ROM classification and final editorial
acceptance remain separate. Ready for redirect; final audit remains pending.

The entire prior commentary was preserved at **analysis/round-3/commentary-round-002**
before fresh preparation. **carry-report.json** records 1,853 exact physical spans
and 213 changed classified spans. Sound names/prose and all 1,819 valid compact
units were retained. Carried blocks were bounded separately from trailing unknown
bytes and reverified; no stale whole-source/assembly import was accepted.

All **2,090 scaffold entries** and **1,992 compact units** are reviewed, with current
source locations. Scope: **991 Code spans/44,667 bytes**, **1,059 Data/117,882 bytes**,
**16 Image/99,328 bytes** and **24 Unknown/283 bytes**. Source has 35,391 lines.
All **37 new Code** and **176 new Data** spans are explained; older dependency and
concrete naming/prose defects are corrected. **No current-source readability gap
or pending range remains.** Generic Data has mechanical explanations and explicit
schema uncertainty; uncertainty is not treated as skipped commentary.

Examples in commentary/source/game.asm:

- Line12586, physical36BA7: cancelable nine-stage scene, B-button gating, overlay
  behavior, independent handlers and stack-consuming inline table. Closing-scene
  BF36 outer/nested return behavior is distinguished from table fallthrough.
- Line18765, physical392B0: grapple endpoint/segment OAM loop, nine-direction
  offset/tile/attribute shelves, count/collision result, stack-balanced X save and
  cursor update. Grapple attachment/launch and camera/player displacement helpers
  explain reused animation/reference fields, ordinary slot five and OAM-cursor reuse.
- Line29684, physical3D5F6: temporary mapper0B→03 without remembered-bank mutation,
  room save/restore and AC03 physicalEC13 in effective6/7; slot16/17 initialization
  and remembered scene bank26/27 restore are distinct. Source does not confuse
  mapper13 with physical bank13.
- Line30921, physical3DAC5: directional little-endian animation-pointer wrapper;
  line31732, physical3DF20: bounded sixteen-round arithmetic with one-byte trial
  remainder limits; line31755, physical3DF45: literal eleven-slot scan Y=11..21,
  including its surprising extension beyond ordinary allocated actor slots.
- Line32105, physical3E160: high/low nibble tiles at PPU237C/237D; line32821,
  physical3E89A: mapper0B A817 physical2E827 and accepted0D restoration; line32871,
  physical3E8D0: alternate ABI clears **sixteen pages at1000..1FFF**, rather than
  a misleading sixteen-tile-row interpretation.

**data-consumer-review.json** records **130 new shelves/310 direct reads**. Consumer
references use named CPU-local data symbols plus exact offsets; .org/.reloc keeps
CPU aliases separate from physical placement and rebuilds unchanged bytes. Six
new inline tables have exact word-count, bank and stack contracts. Other shelves
explain repeated signed coordinates, dimensions, tile/control streams, small lookup
fields, sentinel/pointer patterns, or honest generic byte boundaries. Unknown reset
prefixes stay separate from their now-classified footer suffixes.

Corrected older/current dependencies: all four prior numeric control targets are
now named; **no numeric branch/JSR/JMP operand remains**. Source corrects audio reset
versus palette fading, scroll-column preparation versus palette stepping, rendering
disable wording, D718 collision-coordinate continuation versus animation reset,
and nonnegative-health return versus underflow hit-state entry. Bank contracts and
review summaries/source locations were refreshed. The type-search and division
limits are documented rather than hidden by game-facing naming.

Unresolved meanings: exact scene/actor/equipment identities and generic record
subdivisions, division outside its bounded remainder contract, natural final shot
and complete timed credits. Structural frontier remains exactly the disassembly
handoff: seven MMC1 reset/alias prefixes; renderer-like3D62A+80;
counter/mapper3D67E+50; trap3E010+23; arithmetic3E9B0+64; isolated return/JMP/FFFF
fragments and3B612 suffix; vector/wrapper3D7E5/3E99A with actual mapper0B BC00
physical2FC10 dictionary conflict. Source gives honest local boundary notes for
all24 intervals. The primary competing interpretations and bounded next checks
remain in **analysis/evidence/round-003/review.md** and prior redirect research.
No new structural annotation, terminal-exhaustion claim or unchanged gameplay
replay is made. Next redirect should challenge these narrowed hypotheses under
evidence-policy.md, not require natural callers universally.

Fresh final **commentary.py verify** passed **ok=true, byte_identical=true**;
**commentary.py read** refreshed reading-after.asm. Verification:
**commentary/verification.json**. Completion counts/hashes:
**analysis/round-3/commentary-completion.json**. Source SHA256 game.asm
**e055143cef984cb3d596a8f1124e8f0a61f932b6aeb6c53f576a671b86bb2b55**;
symbols.inc **df5160e53f5663077376929f05becf664036fd646a46701e2113ba1202fad5d4**.
Reading copy SHA256 **c0f74d1aae49c3276b0b64478b295320e68fee4ffd0819a34078c45e1f302a79**.
ROM unchanged **f5f1e212b70e858b3b0257161bc4c25a390602dacd12d5c0877a971ce8a18567**.

No map/classification/ROM/toolkit edit, archive, cleanup/deletion or publication.
All generated assembly/assets/internal evidence remain local. No new toolkit issue;
existing **analysis/toolkit-issues.md** preserved. Disk4.0GiB free; cited180KiB
analysis/scratch retained. Final editorial audit intentionally not run; it is
reserved for final acceptance after redirect, independently of this completed
current-source readability pass.
