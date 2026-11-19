# Round 001 commentary — interrupted B, resume this phase

Current-source readability gate is **not complete**. This is a context-capacity
checkpoint, not successful commentary completion or readiness for redirect.
Resume existing commentary/source/ and review inventory; never prepare/regenerate.
Historical first interrupted checkpoint is round-001-commentary-a-handoff.md;
this checkpoint is also preserved as round-001-commentary-b-handoff.md.

Classification unchanged165586/262160=63.16%: Code22423,Data12091,Image131072;
unknown96574. No maps/state/classification changes; input/game.nes untouched.

Reviewed Code16667 bytes. All current classified Code in banks0,5,6,7,8,9,12,13
is reviewed. Bank14 reviewed API slots physical[114710,115253), main stage/session
setup[115256,117352), and actor primitives[121328,122346), only classified entries
within these spans. Prior reset physical[130880,131058) stays reviewed. All Data/
Image and unknown-gap notes retain credit.414 meaningful inventory units reviewed.

Remaining **291 Code scaffold entries/5756bytes**, firstphysical117379(bank14 NMI).
Exact ranges/IDs are in analysis/round-001-commentary-remaining.json, matching
pending commentary/review.json entries. Continue NMI/IRQ/video/controller/HUD/sound
work in bank14, skip already reviewed actor primitives, then bank15 runtime helpers.
An entry atphysical122889 crosses the physical14/15 boundary; use exact physical
ranges and execution views, not merely start-bank arithmetic. Reset retains credit.
Do not mark the whole source reviewed until every pending entry is explained.

Substantive additions: bank7 stage/scroll transitions,224metatiles->896tiles,
64-byte attribute packing, ten-byte copied bank5 screen records, stage palettes and
partial OAM templates; bank8 event flags/trigger directions/class timers and nine
21-byte actor scans, collision shape formats/sign-saturating rectangles and effects;
bank9 parameter/handler ABI and response/collision/offset actor creation; bank12
actor metasprite stream/flips/clipping/bidirectional OAM; bank13 player action
priority/health/immunity/animation clocks and three frame streams, movement records,
cell/quadrant collision and player OAM; bank14 timers/record/state/shape/update flags,
viewport/depth/facing/range tests and main stage/session setup.

Evidence refinements: bank9 separation helper has same-page absolute low-byte
subtraction but different-page actor-player subtraction (can be negative), so no
full absolute-distance claim. Bank12/13 OAM consumers establish X/Y coordinates;
neutral bank8/9 aliases sharpened accordingly. Fixed-body review shows $C0B7 consumes
contact flags and $C0C3/$C0C6 change update flag3, correcting provisional bank9
initialization prose. Numeric families/animations/terrain types avoid invented game
identities. API address-based names for unresolved bodies are intentionally retained.
Later fixed-bank bodies may refine these dependencies without wholesale reread.

Fresh supported verify passed byte-identically after all source edits. One earlier
failure found an alias collision ($F2 viewport scratch vs$A8 bound scratch), fixed
with separate local aliases and reverified. Reading-after.asm refreshed. ROM SHA256:
ad0ba0b8b4f6dc25d0244cf649a1a60e240c86218e4069dd40a3beff04fd9860.
Source SHA256 game.asm:
f0b395054ebdfe9cdb67973fe42964243263a2ea5c5cdfae6c1e02bc02662b3f.
Source SHA256 symbols.inc:
7c5ea0e0e97c93fe68e57920f6e404aa5cb111cc36fb9dbb4559a3c2615512b2.
Inventory source locations refreshed after final edits. Final editorial audit unrun.

L007 stays for structural redirect: physical$576/bank0CPU$8566 is classified Code
BRK but also read by alternate duty lookup. Preserve classification until examined.
Other unresolved table/gap roles remain explicit in source. No new structural claim.
Toolkit unchanged; NES-001 buttons-list workaround remains sufficient (one confirmed
bug, zero suspected/missing features, no blocker). Disk7.2GiB free at checkpoint.
No archive, cleanup, publication or competing phase. Continue commentary until all
current-source readability is complete, fresh verify passes and read copy refreshed.
