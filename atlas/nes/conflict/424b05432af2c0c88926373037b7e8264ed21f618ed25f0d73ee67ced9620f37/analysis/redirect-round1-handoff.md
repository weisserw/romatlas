# Round1 redirect handoff — combined ready batch

Phase COMPLETE for this redirect; ROM classification remains INCOMPLETE. This is
not terminal research exhaustion. No map/source/ROM/toolkit mutation occurred.
No live emulator, server, map writer or unfinished experiment remains.

Input checkpoint pass-001g/current map SHA256:
`7c2b49e7c4417c10fe6a7b5a6f107cb5b48a40b0c46b734bf80c3d139c5c4fbd`.
Accepted classification stays178867/262160; Unknown83293 in275 exact intervals.
ROM hash424b05432af2c0c88926373037b7e8264ed21f618ed25f0d73ee67ced9620f37.
Frozen source hash7fefa470f60fbfb9b9f3c25176fd57ceb5161ae6adffb1740bed743a24b9f661;
existing byte-identical verification remains applicable because source is unchanged.

Ready proposals: **40517 unique physical bytes**, comprising10094 Code and30423
Data. If independently accepted, residual Unknown would be42776; this is not an
accepted count.4531 code instructions,1220 with actual artificial-scenario fetch
provenance. Inferred contexts remain distinguished from observed views. Internal
proposals are not a portable sharing map.

Primary review bundle:
- [Summary](redirect/r1-static-continuations/summary.json),
  [code](redirect/r1-static-continuations/combined-code-proposals.json),
  [data](redirect/r1-static-continuations/data-proposals.json).
- [Conflict review](redirect/r1-static-continuations/combined-conflicts.json):
  zero code/data overlaps, conflicting instruction starts, existing-map overlaps
  or physical-view identity errors; enclosing data claims also have no Code conflict.
- [Exact complement/dispositions](redirect/r1-static-continuations/unknown-ledger.json)
  retains all275 current intervals, bank/CPU context, neighbors and proposed versus
  residual subintervals. Accepted complement independently recomputed unchanged.
- [Observed union](redirect/r1-static-continuations/observed-union-summary.json),
  [new fetched instructions](redirect/r1-static-continuations/observed-new-code.json).
  ROM, cartridge RAM and internal RAM identities remain separate. Detail capacity0
  was intentional; aggregate drops are0. Untraced waits do not establish fetches.
- [Structural method/contracts](redirect/r1-static-continuations/graph-contracts.json),
  [decoded instructions](redirect/r1-static-continuations/graph-decode.json),
  [reproducible graph method](redirect/r1-static-continuations/graph.py).

Combined findings, not separate artificial workflow rounds:
1. R1-L03 follows wrappers and field/overlay/movement/turn/battle-commit continuations,
   all five distinct C749 handler targets, plus the connected production consumers.
   Field mode3, paired bank history, balanced stack, bounded RAM-pointer and
   inline-data contracts are explicit. Post-F7EA middle calls use[8,9,14,15]; the
   final wrapper RTS uses the incoming restored view. Do not copy caller banks
   across a bank-changing call or treat summaries as unconditional RAM contracts.
2. R1-L06 proposes bank5's stage directories, ten-byte row windows, encoded maps,
   47 six-byte patterns and patterned generic tail. Stage2 reuses an earlier row;
   stage14 overlaps windows at stride5. Specific tail purpose is unproved and is
   not a prerequisite for non-code classification. Exact evidence:
   [bank5 structure](redirect/r1-static-continuations/bank5-structure.json).
3. R1-L07 resolves the two eight-word audio dispatch tables, parameter tables,
   58 request records and185 stream roots.11661 parsed stream bytes end atB5E4;
   one unreachable-to-this-parser byte A8FD remains only generic stream-slab data.
   UniformFF suffix is separate. Audio footer words areFFFF/C000/FFFF, **not**
   C01B/C000/C01B; bank5's latter mirror is independently confirmed.
   [Audio bounds](redirect/r1-static-continuations/audio-data-boundaries.json),
   [flow proof](redirect/r1-static-continuations/audio-stream-proof.json).
4. R1-L02 entered the actual Map16 password UI, then accepted a ROM-selected key
   through the real comparator after explicit buffer/latch edits. All960 expanded
   bytes equal the old bypass snapshot. Final encoded map uses2*52 indices; header
   bound20 corroborates two column blocks. No universal proof that every later
   read excludes the third expanded slab is claimed.
5. R1-L01 resolved the actual terminal predicate: C446->bank4 AC1E->AF13 tests
   saved battle tile codes6FFE/FFF masked42 equal40/42 and corresponding descriptor
   death bit80. Earlier artificial center83 could not satisfy it. Independent unit
   graphics/dictionary evidence gives M1A1 code41 andT80 code43. A five-instruction
   game-writer test changed only saved codes[83,43]->[41,43], preserving all16 result
   bytes, and reached0420=1 plus rendered BLUE HAS BEEN DEFEATED. No repeated fight
   or earned/natural defeat claim. [Contract](redirect/r1-static-continuations/terminal-entry-contract.json),
   [assertions/route](redirect/r1-static-continuations/terminal-route.json),
   [screen](redirect/r1-static-continuations/terminal-outcome.png).
6. R1-L04 reached a production panel with ordinary inputs after an artificial
   cursor/cache warp and a confirmed parent refresh; this route did not patch PC
   or the center tile code. A confirmation created pending descriptor14. Actual
   EndTurn/selection-clear and paired upper-window contracts then allowed8090/B7CB
   to insert exactly its eight bytes: world15->16, with all15 old eight-byte records
   shifted unchanged relative to the already-reset baseline. Actual INC6000 fetch
   at physical1B85F is in production-upper-contexts-1.json. The final image is black
   during rebuild; no rendered new-unit claim follows from that capture.
   [Panel](redirect/r1-static-continuations/factory-parent.png),
   [world assertions](redirect/r1-static-continuations/production-upper-route.json).
7. R1-L08 followed the outcome and all six primary/seven secondary ending handlers,
   their typed tables and10 bounded PPU-packet assets. Artificial final outcome
   state and proper initializers visited default ending, staff credits, END and
   A/release restart. Intro/page counter skips are recorded. Two LSRs test mask02
   (zero-based bit1), which the third Fame choice produces through0A->3B->00.
   The erroneous04 test is retained as a wrong premise, not negative eligibility.
   [Ending handlers](redirect/r1-static-continuations/ending-state-handlers.json),
   [credits](redirect/r1-static-continuations/ending-hard.png),
   [END](redirect/r1-static-continuations/credits-end.png),
   [exit](redirect/r1-static-continuations/credits-exit-route.json).

Commentary dependency reconciliation is REQUIRED after independent disassembly:
[invalidations](redirect/r1-static-continuations/commentary-invalidations.json)
lists25 source occurrences and nine old reviewed units. FIELD_UNIT_STRENGTH_AMMO
at03BA is wrong: record byte3 is a movement/travel-budget field; actual packed
combat strength/ammo is byte4/03BB. Retained values prove byte4 changes1E->0D while
byte3 stays86. Counter producerB1C6 must be described mechanically as testing both
nibbles of byte3, not combat health/ammo. Current CB39 property-lookup wording also
needs correction: E048 counts units for the active player and compares16. Turn
phase/quota semantics and precise ending mask02 naming must be reconciled. Preserve
old review history and reuse unrelated sound review; do not edit source in redirect.

Remaining avenues/dispositions:
- All concrete connected continuations identified here were followed through the
  production birth and ending exits, plus independent bank5/audio/password avenues.
  Typed8093/8096 AI-choice/neighbor continuations close statically; no every-AI-state
  execution or arbitrary state sweep is needed to propose their structure.
- Precise universal C749 primary/secondary table partition is still not proved;
  generic pointer-data and independently supported handlers are separate claims.
- Natural campaign victory, every credit page and all final third-slab consumer uses
  were not proved. Further artificial successes would not establish those stronger
  claims; they do not block this supported structural batch.
- Existing footer Data first byte physical1FFEF is RTS-shaped after the trampoline's
  JSR8033, but8033's normal outcome routes reset rather than return. The preceding
  eight trampoline bytes are proposed; the existing footer byte is not silently
  relabeled. See [claim review](redirect/r1-static-continuations/claim-review.md).
- Other residual unknown intervals retain exact boundary/entry/code-versus-data
  ambiguity. Ledger disposition is bookkeeping, not a claim of exhaustive research
  or that unexecuted bytes are dead/data. New accepted source may expose new leads.

Next: independently review/annotate this combined batch, validate/save/export and
make a new byte-identical checkpoint; then full current-source commentary with the
listed dependency corrections, followed by redirect. Reuse evidence; no unchanged
runtime replay is necessary. No new toolkit issue; NES-DOC-001 remains worked around.
No publication, archive, cleanup/deletion or toolkit build occurred. Disk2.8GiB free.
