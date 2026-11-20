# Round 1 redirect handoff — combined ready batch

Redirect complete for this combined investigation; ROM remains incomplete.
No classification, source, toolkit or ROM changes. Next phase: disassembly review.

Input/output accepted checkpoint: `round01-disassembly-final`; map SHA256
`ecde055d22cf42971290adbfd33853bdcb7abe0d42907ed8ad330a9693f68c62`.
ROM SHA256 unchanged `c0637229cffcc7eab627423ebfc47e561ef8c64df06a1a6b22e2b26ed8fb37f7`.
Current classification remains36.80%:48,238 classified /131,088 total;
82,850 Unknown (15,218 Code,33,020 Data). Prior byte-identical verification and
current-source readability remain valid; this phase did not rebuild unchanged
source. Final editorial acceptance remains pending.

Research: `analysis/redirect/r01-structural-contracts/`. Authoritative manifests:
`validation.json`, `code-proposals.json`, `data-proposals.json`, `unknown-ledger.json`.
Leads: `analysis/leads.json`, nine ready IDs (seven new; PERIOD-GATE and
STATIC-RETURN materially improved with prior outcomes preserved).

Proposed, not accepted:
- 30,318 unique Unknown bytes ->Data.
- 14,500 unique Unknown bytes ->Code (14,784 summed context bytes;6,789 contextual
 instruction proposals).762 new starts match actual artificial fetch identities.
- 219 existing Code bytes ->Data correction;15,888 existing Data bytes refined.
- No code/data proposal overlap. Hypothetical residual38,032 if every proposal is
 accepted; actual residual is still82,850. Do not report proposed bytes as accepted.

## Supported batch

C01: physical000328..00115C exclusive: two consumer-anchored pointer spans and142
exactly adjacent count+4N metasprite records. Preserve000326..000328 genuine BCC
renderer/dummy selector-zero slot.201 current Code bytes need correction;3,435 new.

C02: physical0040BF..0040CF copy-untilFF template and0043EB..0043ED signed strip
coordinate deltas; preserve neighboring genuine handlers.16 Code corrected,2 new.

C03: physical01DF49..01E063:64-byte direction records plus mixed patterned animation
frames/attributes/descriptors to independently accepted E053 routine. Generic mixed
remainder is sufficient; do not invent exact overlapping schema.2 Code corrected
(not3),280 new. Consumer and preceding unconditional branch are independent evidence.

PPU graphics:15 token streams with exact FF endpoints, destinations, encoded
literal/repeat lengths and source-bank contexts; fixed C8CA stream resolved as fixed
memory. Bank3 prior generic data refined; large bank4 containers newly supported.
6,854 new bytes including stream pointer/bank tables. `graphics.py/json`.

PPU templates:36 pointers/35 distinct starts, one suffix alias; FE andFF termination,
FD internal continuation. Physical01C971..01CC9F (814bytes) ends exactly at next
accepted routine. `templates.py/json`.

PPU raw copies:17 specs+payload physical0080A7..00A99E and6 specs+payload
0157FF..017799. Encoded source/end/count/destination tables and clamps provide
18,353 new bytes. `chunks.py/json`, `chunks_bank5.py/chunks-bank5.json`.

Dispatch/data:580 new bytes across inspected match/menu/actor/presentation,
summary/intermission/fight tables and tiny masks/effect arrays. Full manifests retain
exact physical bounds/context; generic pointer-pattern prefixes make no complete
original-table/selector claim. Source-wide triage added10 independently accepted
handler-boundary tables,13 pointer-pattern prefixes, corrected scene/actor cases,
then6 and3 nested tables. Every selected handler and supported ordinary continuation
was followed. `nested-frontier.json` is empty. First8-pointer heuristic failures
are corrected; do not inherit them as whole-table rejections.

Code: read-only `static_contracts.py/json` starts accepted views, explicit bounded
handlers and validated actual fetches, follows direct/local flow and caller returns
only under314 stack/bank summaries. Most summaries mechanically reject unbalanced
stack, caller-stack pops, TXS/RTI/KIL/BRK, mapper/indirect writes and indirect JMP.
C79A is a nonordinary-return barrier, with handlers separately followed. C816/C818
has an inspected stacked-old-bank and15 finite-stream contract; opened caller X
values are literals even0..28 or inspected04/08/0A table. C3CB/C3D8 conditional
queue bound<=128 plus chunk<=80 gives last destination07D4, below saved07E0.
Other wrappers are not universally certified. Read source contracts and independently
validate each proposed instruction/bank and relevant caller precondition. Actual
fetches may beObserved only for matching starts/context; all other inference isStatic.

## Gameplay and reproducible evidence

`analysis/scenarios.json` updated with previous attempts preserved. Counter34's
D1C5 increment and>=2/>=3 tests independently confirm second/third periods. Original
expiryfrontier6C was already0; old wait was not proof of a permanent pause. New
route reaches stateA/B/C then live period2, timer reset20:00 and19:48. Second expiry
visits arcade intermission; finite timeout reaches thirdperiod. Optional66=7 invokes
proven terminal handler9DF0; never set66=FF before dispatch. `third-live.snapshot`
and `terminal_probe.py` are completed actual third-period continuation evidence.

RAM-only timer and tiedscore07E5/07E9=0 at thirdperiod lead counter34=3 and state17
shootout. Fight from period2-before-expiry snapshot:30=16,31=0,80=0,6D=0; service
initializes the fight. A/B/right changedpower display; arcadeA/right advanced to
NICE SHOT. ShootoutA/right/B exercised; completedshot not claimed. Selected screens,
exercised snapshots, exact edits/inputs/budgets and bank-aware actual fetches are
saved (`exercise.txt`, `contexts-exercised.json`, `observed-roots.json`, RPC log).
All resulting reachability is artificial, not a natural gameplay claim.

Long later_game.py process exited143 before final snapshot; partial samples/screens
are retained and later valid snapshots replace its missing continuation. Its batch
named3 patched timer while summary, not thirdperiod; do not trust filenames as
milestones. Old live PTY46898 not relied on. New read-only servers loaded checkpoint,
started emulator and imported explicit persisted snapshots; helper processes ended.

## Verification, deferrals and next recipe

`validation.json` confirms unchanged ROM/map identities, exact counts/complement,
zero proposal overlap and no accepted bytes this phase. Review all219 conflicting
Code bytes using `data-proposals.json.code_conflicts`; do not blanket-remove renderer
or genuine adjacent handlers. Then independently validate/classify the combined
batch, preserve provenance, refresh exact complement, checkpoint and byte-identical
rebuild, followed by full current-source commentary.

Actual missing prerequisites and reduced-prospect gameplay continuations are in
`deferrals.md`, with source/bank barriers in `static-contracts.json.blocked`.
Sound page1 storage versus maximum nested/NMI stack reservation, shared saved-bank
nonclobber outside certified wrappers, and genuinely unresolved indirect destinations
remain local barriers. They do not reject nearby data or demand natural callers.
Winning every fight/shot/tournament adds no current concrete new structure test;
representative systems and terminal scene are already visited. No terminal exhaustion
review or whole-ROM completion claim is made; ordinary ready-lead handoff is intended.

No toolkit issues found. `analysis/toolkit-issues.md` preserved. Disk~4GiB free;
research~75MiB, bounded logs/snapshots/captures; no cleanup/publication/archive.
All ROM, state, source, assets and evidence remain local.
