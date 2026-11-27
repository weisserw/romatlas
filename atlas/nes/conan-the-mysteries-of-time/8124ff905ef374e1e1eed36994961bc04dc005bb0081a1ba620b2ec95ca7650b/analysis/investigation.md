# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.


## Round 1 disassembly

ROM classification: 14.54% (19055 / 131088 bytes classified; 112033 bytes unknown).

Code 13531 bytes; data 5524 bytes, including 16-byte header.
639/639 classified regions have evidence; 6462 supported decoded instruction starts.
Execution is separate: at least 4129 unique fetched opcode starts in captured contextual pages;
2473 in natural startup/gameplay windows and 3580 in artificial windows (overlap included).
The startup context page omits one of 1001 roots; no unrecorded root was used for classification.
Zero aggregate drops in saved trace summaries; detail capacity 0 intentionally drops detail.

| Physical 8 KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 0 | 128 | 8064 |
| 1 | 0 | 1537 | 6655 |
| 2 | 0 | 0 | 8192 |
| 3 | 0 | 0 | 8192 |
| 4 | 0 | 0 | 8192 |
| 5 | 0 | 513 | 7679 |
| 6 | 0 | 0 | 8192 |
| 7 | 0 | 0 | 8192 |
| 8 | 0 | 0 | 8192 |
| 9 | 0 | 0 | 8192 |
| 10 | 0 | 0 | 8192 |
| 11 | 0 | 1615 | 6577 |
| 12 | 1879 | 0 | 6313 |
| 13 | 1290 | 626 | 6276 |
| 14 | 4490 | 118 | 3584 |
| 15 | 5872 | 971 | 1349 |

Validated raw baseline, then annotated fetched instructions and static connected CFG
with physical identity and effective mapper2 windows. Upper banks 14/15 are permanent;
call returns/mapper-capable stores discard lower-window knowledge. No unofficial
opcode candidates, overlaps or classification warnings survived final traversal.
R1-L01 naive sequential CFG rejection was narrowly due to inline words after three
$D611 wrappers, not rejection of their bodies. Reopened with consumer-supported
two-byte skips and handler targets; corrected traversal verified.
R1-L02 stage transition and R1-L03 finite stage tables verified; see leads/scenarios.
Uniform data inspected for references and change boundaries in
[data-pattern-review](round-01/data-pattern-review.json); no conflicting code evidence,
no fill-purpose claim. Large neighboring ranges remain unknown because code/data
and exact bounds need separate consumer/format work, not because names are unknown.
Broad gameplay sampled every stage index and ending plus inventory. Button-only
exploration supplied natural cave evidence; later-stage edits are explicitly artificial.
No unchanged failed experiment repeated: ending repeat removed B, which the decoded
terminal loop treats as exit. Initial wrong input schema and failed file-overwrite
capture attempts never served as gameplay evidence. Logs preserve errors/results.

Next redirect should investigate R1-Q01 asset consumers and R1-Q02 indirect engine
handlers as connected batches. These need source/selector contracts, so another
blind gameplay replay has lower prospect; no claim of exhaustive easy gains in
every unknown interval or terminal exhaustion. Commentary has all current source
to review before redirect. [Verified checkpoint](round-01-disassembly-verification.json).
No toolkit issues found; [report](toolkit-issues.md).

## Round 1 redirect — r01-connected-20261127
Combined ready batch saved in handoffs/round-01-redirect.md and unique attempt.
Recovered all37stage initializer graph plus immediate task producers; complete
stage/metatile/CHR/animation/image/raw tile/music/palette domains; residual engine
API/stack contracts. Static proposals112032new bytes,13318code/98714data; no kind
conflicts; hypothetical one-byte residual1CAF1/CAE1 remains ambiguous RTS. Accepted
classification stays19055,unknown112033. All practical connected and identified
independent avenues processed, no arbitrary first-lead phase stop. No new runtime
replay needed because prior stage0..7/terminal/inventory coverage already exists.
Decoder repeat/literal commentary correction and CE9F RAM-inline-word barrier
must carry forward. Independent disassembly review is next, not terminal assessment.


# Round 2 disassembly completed; ROM incomplete

round=2 phase=disassembly status=pass-complete; next=commentary.
ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).
Rounded percentage does not establish completion. Code26849, data104238 including
16-byte header. Gain112032 (13318code,98714data). 906/906 classified regions have
evidence. 5964 proposed instruction records independently decoded;
all accepted structural proposals Static, no new execution/reachability claim.

| Physical 8 KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 7646 | 546 | 0 |
| 13 | 6833 | 1359 | 0 |
| 14 | 5409 | 2782 | 1 |
| 15 | 6961 | 1231 | 0 |

Accepted R1-Q01/Q02/L04 as the combined connected structural batch after independent
consumer, exact-boundary, bank/stack and data-pattern checks. No proposal slices
rejected; unproposed lone1CAF1/CAE1 remains unknown. Existing accepted regions all
preserved exactly. [Review](round-02/review.md), [primitive audits](round-02/review.json),
[counts](round-02/counts.json), [lead history](../leads.json). Historical failed
methods remain preserved; no unchanged gameplay replays. Prior scenarios visited
stages0..7, inventory and ending with artificial/natural provenance kept separate.

Remaining1byte60 at physical1CAF1 CPUCAE1 is isolated RTS between NMI tailJMP and
IRQentryCAE2. No incoming edge/discriminating routine contract found. Keep unknown;
no dead/unused/code/data assumption. This local uncertainty does not reject coherent
neighboring engine APIs. No productive runtime/connected unknown target remains in
this pass; commentary then redirect should review this actual residual frontier.
No terminal exhaustion assertion by this disassembly pass.

Fresh map.validate ok=true; state saved analysis/state.json; export maps/current.json.
[Checkpoint](../maps/round-02-disassembly.json) byte-equal current export;
[verification](round-02-disassembly-verification.json) ok=true,byte_identical=true;
[source](../generated/round-02-disassembly/game.asm). ROM SHA256
`8124ff905ef374e1e1eed36994961bc04dc005bb0081a1ba620b2ec95ca7650b` unchanged.
Map SHA256 `171e9c2d3ffa53f33df490ade67191a3b7d8342fca8f67167e635f4ad3b7948e`. Generated source is local reconstruction,
not a sharing deliverable. Existing commentary/source, reading-after and historical
checkpoints unchanged; new source now requires full commentary review.

Next commentary: carry forward all223 functional units and symbols from previous
source into expanded source, review all newly classified ranges and current source.
Correct decoder token prose:01..3Frepeat,40..7Fzero,81..FFliteral,00/80skip.
Honor physical1CEAF..1CEB1 exclusive (CPUCE9F/CEA0) inline pointer05A7 after
JSRD360->D611; CEA1 resumes code. No structural reclassification needed for that word.
Detailed roles/reachability limits and exact contracts are in review and redirect
r01-connected-20261127 data-contracts.md/residual-contracts.md. Preserve generic-data
honesty where identities or original subtable layouts remain unresolved.

No toolkit issues found; [persistent report](../toolkit-issues.md), zero blockers.
No ROM/toolkit changes, cleanup, deletion, archive, publication or commit. No live
server remains. Disk~3.4GiB before checkpoint; bounded evidence retained locally.

## Authorized manual fallback disassembly phase

User explicitly authorized a batch-only exception to the structural evidence gate:
verify the complete residual complement is <=10 bytes, then record the listed
residual as generic Data, unknown meaning, Manual provenance. Authoritative saved
state/current export agreed; the exact complement was [117489,117490), length1
(physical $1CAF1, CPU $CAE1, fixed PRG8KiBbank14). Precheck and supported
map.annotate/save/export transcript are in [fallback](fallback/authorization-and-precheck.json).
One Manual Data annotation was added; all906 prior regions remain unchanged.
This is user-directed fallback, not new structural proof. Prior redundant RTS/no-op
callback versus spare-data ambiguity, reference/stack review and history remain valid.
No redirect research or gameplay reruns were performed.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
This annotation metric includes1 user-directed Manual fallback byte. Proven structural
classification remains131087/131088 (99.999237%);1byte remains structurally ambiguous.
Code26849;Data104239 (104238proven+1manual). The fallback does not satisfy the
original evidence-backed completion gate or prove execution/reachability.
Per-bank classified/unknown: banks0..15 each8192/0. Proven/manual counts: banks0..13
and15 each8192/0;bank14=8191proven+1manual. Header16proven Data. Full per-bank
ledger: [counts](fallback/counts-and-verification.json).

Map and store validations pass. New [checkpoint](../maps/manual-fallback-disassembly.json)
exactly equals current map and state; [verification](manual-fallback-disassembly-verification.json)
confirms byte-identical rebuild. Prior map/state copies retained in analysis/fallback;
all historical checkpoints/final audit artifacts preserved. Next separate commentary
worker should carry the current readable source forward, update only the residual
note and affected review/checkpoint metadata, preserve the Manual distinction and
perform fresh source verification. No toolkit issues found; [report](toolkit-issues.md).
ROM/toolkit unchanged. No deletion, archive, publish or commit.
