# Round 001 redirect — combined batch ready for independent review

Research complete for this pass. Input checkpoint remains **round-001-disassembly**.
Nothing was classified or retyped in redirect. Actual classification remains
165586/262160 bytes (63.16%); **96574 bytes in390 exact unknown ranges** remain.
Current-source commentary gate is complete; final editorial audit remains deferred.

[proposals.json](proposals.json) proposes **74695 unique new bytes:25289 Code,
49406 Data**, plus **one existing Code byte retyped Data** underL007. Accepted:0.
If every proposal is accepted, residual unknown would be21879; this is conditional,
not a changed map or ROM-completion claim. Physical code has772 newly observed
bytes (all artificial scenarios) and24517 Static bytes. Structural evidence and
natural reachability remain separate.

Disjoint reporting allocation ([lead-counts.json](lead-counts.json)):

| Lead | New Code | New Data | Total |
|---|---:|---:|---:|
| L001 stage/graphics data | 0 | 29661 | 29661 |
| L002 Factory/terminal contracts | 4066 | 77 | 4143 |
| L003 sound/DAC | 367 | 9916 | 10283 |
| L004 actor/fixed helpers | 20749 | 615 | 21364 |
| L007 duty conflict | 0 | 3 | 3 |
| L008 screen initializers | 107 | 135 | 242 |
| L009 actor metasprites | 0 | 7497 | 7497 |
| L010 player metasprites | 0 | 1502 | 1502 |

L007 additionally retypes physical$576. Allocation is bookkeeping; every supported
CPU view remains in the proposal. Global leads/pass history is updated in
analysis/leads.json, preservingL005 verified andL006 locally rejected recipes.

The batch follows connected handlers and independent avenues. Its decisive
primary artifacts are:

- [graphics-pointer-contracts.json](graphics-pointer-contracts.json): seven
  independently matching4:1 metatile/attribute extents, counts187/95/84/92/199/89/242.
  Alternate graphics is a+156 slice, not an eighth independent table. Full prefix
  inspection and generic fallback evidence are in data-block-inspection.json and
  stage-byte-grid.png; mixed bank5 code/parameter boundaries are split.
- [sound-command-table.json](sound-command-table.json) and
  [sound-safe-store-graph.json](sound-safe-store-graph.json):51 command targets and
  bounded driver continuations. [sound-gap-0x9cb.json](sound-gap-0x9cb.json) proves
  direct$4011 DAC source CPU$A700..$B100 exclusive, physical$2710..$3110; no DMC claim.
  A fourth sound API JMP at physical$19 defeated an initial Data-prefix claim;
  Data was narrowed to$1C..$24 using actual fetched counterevidence.
- [actor-type-entry-contracts.json](actor-type-entry-contracts.json),
  [actors-safe-store-final-graph.json](actors-safe-store-final-graph.json),
  [actor-final-safe-audit.json](actor-final-safe-audit.json):47 type records,
  28 distinct low-bank APIs plus7/9 high-bank APIs,58 conditional state records,
  bank-safe RAM-write continuations and literal handler writer/consumer contracts.
  [ninth-adapter.json](ninth-adapter.json) adds the previously unclassified$63/$64
  adapter and its$94CE handler; actor-local-helper-contracts.json separately
  supports nine complete mechanical helpers without requiring known callers.
- [fixed-helper-final-graph.json](fixed-helper-final-graph.json) and
  [fixed-helper-final-audit.json](fixed-helper-final-audit.json): the finite61-root
  helper investigation, exact stack/control/bank checks and conditional HUD-pointer,
  mapper-register and terrain API contracts. Fixed banks14/15 are preserved;
  lower banks remain unresolved. Historical stops are not current blockers.
- [metasprite-contracts.json](metasprite-contracts.json),
  [metasprite-residual-inspection.json](metasprite-residual-inspection.json) and
  [player-sprite-contracts.json](player-sprite-contracts.json): all301 actor and53
  player pointer/header/geometry records checked, residual separators/extra records
  inspected, and bank identity bytes separated. Generic Data does not claim unknown
  sprite IDs or original layout semantics.

**Do not promote the original actors-safe-store-graph.json.** It is a preserved
invalid-root attempt:31 low type records were mistakenly treated as31 consecutive
slots, rather than28 distinct targets. Exact table-derived roots and clean
instruction-overlap audits supersede it. The valid final actor graph has8753
instructions/20353 total bytes including previously classified bytes; counts alone
are not structural proof. See [findings.md](findings.md) for the local correction,
contracts, negative results and preserved chronology.

Practical progression went beyond ordinary Factory play. The C399 predicate
shortcut reached an artificial desk/official/flag terminal scene with typing text
(terminal-90.png); holdingA60frames did not immediately skip it. A supported
initialization hook with level6/variant8D produced an initialized helicopter scene
(variant8d-2.png), actual variantCD/subvariant3/BD0B and representative controls.
A final RAM-only script shortcut fetched the real8E writer at$FB65 but ended atCE,
without terminalA00C/C399 fetch (factory-script-results/context files). This local
result does not establish ending unavailability. Missing earlier phase/player/entry
flags make arbitrary further forcing unproductive; natural boss completion remains
unresolved. Recipes, clean snapshots, budgets and outcomes are in analysis/scenarios.json.
No runtime process remains.

[unknown-ledger.json](unknown-ledger.json) is the exact unchanged complement with
neighbors, physical banks and evidence-linked dispositions. Conditional residuals:
banks0/1:1/10; bank5:1169; banks6/7/8:5978/2605/3204; banks9/10/11:54/10/3;
bank13:3362; banks14/15:2247/3236. Other banks have no conditional residual.

Important deferral scopes:

- The isolated zero at$4C6 and ten nonuniform bytes$369A..$36A4 lack either a
  discriminating non-code pattern/consumer or an executable-entry contract. The
  PCM extent and neighboring fill do not settle them.
- Remaining short actor tails are mostly isolatedRTS/NOP fragments; complete
  helper ABI evidence cannot be generalized to a lone return opcode.
- Bank5's code-like tail beyond reviewed initializers has an apparent$C607 call
  into the operand of an accepted bank14 branch. The
  [primary boundary decode](bank5-tail-call-target-boundaries.json) rejects that
  assumed canonical call contract, not every possible code interpretation or
  proves the bytes unused/Data. Alternate ABI/relocation/context remains missing.
- Other screen/event/player/path/engine fragments interleave code and variable
  inline records. The examined consumers do not establish their local boundaries
  or pointer-space contracts. They are explicit structural backlog, not unknown
  meaning used to withhold otherwise clear Data. No whole-ROM exhaustion is claimed;
  this is an ordinary supported batch, not a terminal incomplete assessment.

[protected-hashes.json](protected-hashes.json) confirms ROM identity, state equals
frozen phase1 state, map equals frozen baseline/checkpoint, and both source hashes
match the completed commentary gate. Source/fidelity were not regenerated because
nothing protected changed. Toolkit binary stayed configured and unchanged. Report:
analysis/toolkit-issues.md has one confirmed documentation bugNES-001 and one grounded
missing featureNES-002; zero suspected bugs and no blocker. Buttons-list and bounded
static bank-range workarounds suffice. NES-002 now cites555 valid final actor writes,
not the superseded invalid475 measurement.

Next phase: independently review exact candidate roles, boundaries, inline barriers,
stack contracts and CPU/bank views; classify only accepted claims, validate/rebuild,
then refresh full current-source commentary. No archive, cleanup, deletion,
publication, toolkit edit or new phase was performed by this worker. This completed
attempt is immutable after handoff.
