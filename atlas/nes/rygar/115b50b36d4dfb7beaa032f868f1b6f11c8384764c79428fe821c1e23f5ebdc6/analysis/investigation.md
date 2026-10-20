# Investigation ledger

## Round 1 disassembly — pass-001

- Established raw baseline and then independent classification. Baseline fidelity
  did not count as structural evidence. Final verified map/source: `pass-001`.
- Validated reset $C000 and interrupt $C0F6 in fixed banks14/15. Startup and normal
  gameplay supplied actual fetch roots. Bounded contextual static closure stopped
  at calls/possibly mapper-affecting writes; observed successors were separate
  evidence. Stable final static-only recomputation from persisted roots/contracts
  reproduces every classified instruction start.
- Natural title/start, Sueru rightward combat and jump pulses, death/continue,
  status/selection and pause exercised. Gran not reached; brute repetition after
  death had reduced prospect. Routes and snapshots in scenarios.json.
- Full structural audit: 6034 instruction starts, all exact official encodings;
  5131 distinct physical instruction starts were actually fetched in saved traces
  (5315 observed bank/CPU context records). Static-only entries are not observed.
- Rejected physical103264 ($9350 under [12,13,14,15]): preceding BCC $9314 and
  BCS $9316 use unchanged carry. Neither path falls through; the apparent trailing
  BRK lacks independent entry evidence. Removed one byte; do not re-add unchanged.
  See pass-001/final-structural-review.json. This is not a terminal rejection review.
- Accepted Data consumers: startup seed $C0AB[8] copied with X=7..0 by reset;
  bank selector $C0D3[8] consumed by self-write bank helper $C0EC, finite complete
  UxROM 0..7 bank domain and exact adjacent helper boundaries; task bank table
  $C193[16] and handler pointers $C1A8[16x2] consumed by scheduler $C15B/$C17D
  with Y<16 and doubled pointer index. Zero-valued task fields remain bounded
  fields, not code claims. Copyright $C092-$C0AA has semantic text and independently
  proved adjacent idle-JMP/initializer boundaries. iNES header and vectors have
  architectural/parser evidence. Proofs also retained in map evidence/comments.
- Static task handler entry contract checked from bounded dispatch table:
  task-entry-inspection.json records the eleven nonzero bank/pointer destinations.
  Inline dispatcher payload following calls to $C0DB is never treated as a normal
  JSR return continuation. A pointer alone was not used to assert code.

New redirect questions L001-L003 are actionable investigations, not ready proposals.
No unchanged failed lead should be retried. No terminal exhaustion claim; current
source commentary comes next, then investigate these frontiers together as useful.

Final counts: Code11775, Data86, Text25; classified11886/131088, unknown119202.
Per-bank counts and exact complement: pass-001/classification-counts.json and
pass-001/unknown-ranges.json. No toolkit issues found; see toolkit-issues.md.

## Round1 redirect r1-20261020 — completed combined batch

[Handoff](redirect/r1-20261020/handoff.md), [reasoning](redirect/r1-20261020/findings.md),
[proposals](redirect/r1-20261020/proposals.json), exact unknown ledger in same
directory. L001–L003 moved investigate→ready after materially improved static
contracts/consumer bounds and bounded natural graphics transfer traces.161
proposals /66393 bytes remain unaccepted; classification unchanged11886,unknown
119202. Full preidentified type/handler avenues followed rather than selecting
only a first ready graph. Four nested bounds used independent BCC endpoints when
a first origin criterion was too narrow. Universal animation local-container
model counterexamples preserved; only bounded330 pointer bytes proposed there.
Evidence-linked deferrals are selector/geometry/phase constraints, not a live
caller requirement. No terminal exhaustion; independent disassembly next.
Mandatory graphics-loop commentary correction persists in editorial-corrections.
No toolkit issues found; see toolkit-issues.md.

## Round2 disassembly — pass-002-reviewed

ROM classification: 59.75% (78330 / 131088 bytes classified; 52758 bytes unknown).

L001/L002/L003 ready→verified for the supported batch, after independent primary
consumer/role/bounds/context review and fresh code decoding. Original161 proposals
accepted66393 bytes; additional51 Data bytes in initialized ending/event consumers
accepted, total66444. Exact per-bank counts/unknown complement in
[disassembly-r2/classification-counts.json](disassembly-r2/classification-counts.json)
and [unknown-ranges.json](disassembly-r2/unknown-ranges.json); new Static code is
not natural reachability evidence. All new4408 starts independently accounted by
226 entry roots in closure-audit.json; no graph-blind import or payload fallthrough.
Detailed discrimination/lead outcomes and localized deferrals:
[review.md](disassembly-r2/review.md). Existing103264 rejection retained; new
complementary branch pair B41F/B421 does have independently targeted B423 code
(B407 BNE), so no overbroad rejection there. Animation types2/3/25 counterexamples
remain local to the stronger pointed-container model; bounded arrays accepted.
Unconsumed graphics400 bytes/audio separators and ending threshold6/7 stay unknown.
No changed normal-gameplay route/reachability claim; prior diminishing Sueru/menu
evidence retained. No terminal exhaustion; next full-source commentary then redirect.

Internal save/export/map validation and final pass-002-reviewed ca65/ld65 rebuild
all passed, ok=true byte_identical=true. Intermediate pass-002 is superseded old
9.07% map; expected annotation validation failures were restored before final batch.
No toolkit issues found; see [report](toolkit-issues.md).

## Redirect r2-20261020 combined ready

Input pass-002-reviewed/current-source commentary-r2. Persistent L004-L007 ready;21012uniqueproposedbytes,0accepted. Connected sourcefrontier865newstarts, localizedphase/sharedstreammodel andspritecountgrammar,15area/override/trigger/text/timedspawncontracts,independentpalette/UI/tileplaneandfinitefielddomains. No unchangedgameplayreplays. Five animationcells, matrixdimensions/normalization,eventWork8rowwidths,eligibleattack/pickupstatebounds,fallbackrespawnpages andunconsumedasset/fillroles remain specifically unresolved. Negative resultslocalized; no terminalexhaustion. Exact artifact/evidence paths, staticrecipes and preservation: redirect/r2-20261020/handoff.md and review.md. No toolkit issues found.

## Round3 disassembly — pass-003-reviewed

ROM classification: 75.78% (99342 / 131088 bytes classified; 31746 bytes unknown).
L004-L007 ready→verified for all21012supportedbytes (1688Code/865starts,19324Data); independent consumer/root/return/context review, fresh exact closure, animation header checks and script reproduction. See [review](disassembly-r3/review.md), [audit](disassembly-r3/independent-audit.json), [per-bank counts](disassembly-r3/classification-counts.json) and [exact unknown complement](disassembly-r3/unknown-ranges.json). All Static; no new gameplay sequence/coverage. Specific local deferrals and next commentary scope in [handoff](disassembly-r3/handoff.md); no terminal exhaustion. Checkpoint validate/generate/rebuild byte identity passed; preserved prior610regionobjects and commentary/source/name-crosswalk. No toolkit issues found; toolkit-issues.md.


## Redirect r3-20261020 combined ready

Input pass-003-reviewed/current-source commentary-r3. NewL008/L00915226StaticData bytes315proposals,0accepted. Corrected strong prerequisite failures into bounded role unions: state header/result semantics, independently anchored upper geometry payload/246x16nodes, shared256x8lower views, finiteattribute177cells, rooted4bytebitmap276. Connectedattack/auxiliary/spawn/sentinel/fallbackprefix fields included; original failedclaims preserved/narrowed. Complete11307start source audit/primary suffix+graphics review; second intermediate2972nonintegral16/dimensions and untyped suffixes remain scoped, no new runtime or fill claims. Exact405disposition ledger31746Unknown, preservation/mapvalidatepassed. Handoff: [r3](redirect/r3-20261020/handoff.md); no terminal exhaustion. No toolkit issues found.


## Round4 disassembly — pass-004-reviewed

ROM classification: 87.40% (114568 / 131088 bytes classified; 16520 bytes unknown).

L008/L009 ready→verified:315StaticData regions15226bytes, no Code/runtime input additions. Role-only independent anchors, full state/selector semantic consistency and same-format shared unions reviewed against primary consumers. All prior1455 region objects and source/commentary/crosswalk preserved. Exact per-bank counts, residuals, local deferrals and accepted-standard comparison: [review](disassembly-r4/review.md), [counts](disassembly-r4/classification-counts.json), [unknown](disassembly-r4/unknown-ranges.json). Existing103264 impossibility preserved; no whole-gap fill inference, original dimensions or natural event eligibility asserted. Full residual investigation remains redirect work after current-source Data commentary integration. Checkpoint validate/generate/byte-identical rebuild passed. No toolkit issues found; [report](toolkit-issues.md).


## Round5 disassembly — pass-005-reviewed

ROM classification: 89.36% (117138 / 131088 bytes classified; 13950 bytes unknown).

L010 verified17StaticData proposals2570bytes. Independent primary decode/orientation/contracts/context/arithmetic review accepts2525finite second-root cells plus separate35internal minimum-prefix fields,8attribute slots,2action tiles;412suffix remains Unknown. No new Code/starts/runtime routes. Prior1770objects/source/reading/crosswalk unchanged. Exact per-bank/residual/evidence in [review](disassembly-r5/review.md), [counts](disassembly-r5/classification-counts.json), [unknown](disassembly-r5/unknown-ranges.json). Fresh checkpoint validation and byte-identical131088-byte rebuild passed. Next current-source commentary; no terminal exhaustion. No toolkit issues found; [report](toolkit-issues.md).


## Round6 disassembly — L011 verified

ROM classification: 89.37% (117150 / 131088 bytes classified; 13938 bytes unknown).

Independent12root common-index layout acceptance,12StaticData bytes,0Code. Original allocation/reads remain unproved; independent256-coordinate symbolic search mechanics/bounds and competitor assessment in [review](disassembly-r6/review.md). Per-bank/residual counts in [counts](disassembly-r6/classification-counts.json), exact13938Unknown in [complement](disassembly-r6/unknown-ranges.json). All1787prior objects and maintained source preserved. Fresh pass-006-reviewed validate/generate/byte-identical131088-byte rebuild passed. No new gameplay scenario or unchanged replay. Next full current-source commentary then redirect; no terminal exhaustion. No toolkit issues found; [report](toolkit-issues.md).

## Redirect r6-20261020 active
Input pass-006-reviewed/commentary-r6;117150classified/13938Unknown. Research only. Frontier: primary consumer/boundary review of ending/action and scoped residual constraints, exact72-gap ledger refresh; prior reviews reused only on unchanged premises.

## Redirect r6-20261020 — terminal incomplete recommendation

Input pass-006-reviewed/commentary-r6. Classification unchanged117150/131088(89.37%),13938Unknown,exact72gaps. Zero new proposals,accepted bytes,Code,runtime inputs or gameplay routes. Reused unchanged sound reviews; freshly inspected ending threshold8vs6-ID boundaries, two-axis extents vs3-spaced roots, repeated action-attribute tail, animation domains, coordinate sentinel/prefix alternatives, independent uniform suffixes, graphics400 occurrence and audio token/command boundaries. Prior eight-threshold “conflict” narrowed to coupled-eight-record failure; arbitrary unpaired allocation not disproved. Terminal incomplete recommended ONLY for coordinator primary-rationale inspection: [rejection review](redirect/r6-20261020/rejection-review.md), [handoff](redirect/r6-20261020/handoff.md), [exact ledger](redirect/r6-20261020/unknown-ledger.json).

Fresh configured private JSON-RPC api.describe/load/immutable map.validate passed; targeted bank-aware consumer decodes captured, primary byte checks and exact current=internal=pass006 map equality/source hashes passed. No fresh research rebuild; commentary-r6 byte-identical verification preserved by exact hashes. Maps/state/source/ROM/configured toolkit untouched; no commits. No useful justified revised method remains among identified concrete avenues; this is not proof of universal exhaustion or full acceptance. No toolkit issues found; [report](toolkit-issues.md).

## Redirect r6 reopened — graphics representation claim
Coordinator identified narrower backing-role inference using typed neighbor endpoints, coherent tile grammar and exact known duplicate. Prior rejection addressed asset occurrence/use too strongly. Same redirect resumes; prior terminal review preserved. Frontier in redirect/r6-20261020/frontier.json.

## Redirect r6 corrected graphics representation — L012ready

Coordinator critique resumed SAME redirect after terminal recommendation: narrower physical44672..45072 NES2bpp tile-backing representation, separate from natural transfer/original asset/fill purpose. Exact first16tile duplicate43008 and preceding planar stripe continuity; remaining384bytes exactly12two-tile pairs with zero secondplane and symmetric closed8x16 border/notch grammar. Allother15984bank2bytes independently consumed graphics sources; transfer endpoints bound hole. Hardware-only H/V/HV matching across3047consumedtiles supports firsttile only, no broader consumed-match claim. Prior patterned reserve/copy competitor preserves representation role; old asset/use rejection narrowed, historical terminal review preserved.

L012ready400StaticData bytes/2proposals,0accepted; current117150classified/13938Unknown unchanged,fullacceptance13538Unknown. [Current handoff](redirect/r6-20261020/graphics-handoff.md), [corrected primary review](redirect/r6-20261020/graphics-corrected-premise-review.md), [audit](redirect/r6-20261020/graphics-representation-audit.json), [ledger](redirect/r6-20261020/graphics-unknown-ledger.json). Other concrete reviewed avenues retain unchanged scoped premises; no independent useful method deferred. Reproducible audit errors=[]; corrected-commentary source hashes and immutable ROM/map/state/toolkit checks passed. No runtime/map/source edits/new rebuild or commits; independent disassembly next. No toolkit issues found; [report](toolkit-issues.md).


## Round7 disassembly — L012 verified

ROM classification: 89.67% (117550 / 131088 bytes classified; 13538 bytes unknown).

L012 verified400StaticData bytes in2separately accepted regions16+384;0Code/new starts. Independent63descriptor source union/endpoints, exact duplicate and cross-boundary stripe, twelve closed-border/notch planar pairs and concrete format alternatives reviewed. Natural upload, original standalone asset and original fill purpose unresolved. [Review](disassembly-r7/review.md), [audit](disassembly-r7/independent-audit.json), [reproduce](disassembly-r7/audit.py). No new execution or scenario; no unchanged replay.

All1799prior region objects preserved;1801total,22619Code/11307starts,94906Data including16header bytes,25Text. Maintained source/reading/review/crosswalk/assets and ROM/configured release hashes unchanged; [preservation](disassembly-r7/preservation.json). Exact remaining13538Unknown across71gaps: [complement](disassembly-r7/unknown-ranges.json), [counts](disassembly-r7/classification-counts.json). Physical103264 remains Unknown. Other residual reviews unchanged; no terminal claim.

| Bank8 | Code | Data | Text | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 0 | 0 |
| 1 | 0 | 8192 | 0 | 0 |
| 2 | 0 | 8192 | 0 | 0 |
| 3 | 0 | 8192 | 0 | 0 |
| 4 | 0 | 8192 | 0 | 0 |
| 5 | 0 | 8192 | 0 | 0 |
| 6 | 638 | 7510 | 0 | 44 |
| 7 | 0 | 389 | 0 | 7803 |
| 8 | 0 | 7690 | 0 | 502 |
| 9 | 0 | 7976 | 0 | 216 |
| 10 | 3273 | 4268 | 0 | 651 |
| 11 | 5979 | 1527 | 0 | 686 |
| 12 | 4904 | 2816 | 0 | 472 |
| 13 | 1138 | 5065 | 0 | 1989 |
| 14 | 5835 | 1653 | 25 | 679 |
| 15 | 852 | 6844 | 0 | 496 |

Fresh map.validate/save/export passed; internal=current=pass007. Fresh `python3 run.py checkpoint pass-007-reviewed` validate/generate/ca65+ld65 whole131088-byte verification ok=true,byte_identical=true. [Verification](pass-007-reviewed-verification.json). No toolkit/ROM/maintained-source edits or rebuilds/commits. Next full current-source commentary integrates2regions/400bytes and reconciles graphics Unknown prose; preserve11307start/347unit review credit, then coordinator review. Whole-ROM classification incomplete. No toolkit issues found; [report](toolkit-issues.md).


## Redirect r7-20261020
Current71-gap/13538Unknown ledger refresh and graphics-correction analogy audit complete. Closest residual six00/40attribute copy has independently typed neighboring exemplar and code endpoint, but common mask sequence does not discriminate ordinary constants versus OAM occurrence; absent natural selection is not the rejection. Other unchanged primary residual reviews reused with exact scope; historical graphics rejection excluded. No ready lead or justified new method; terminal incomplete recommendation accepted after coordinator inspection. [Review](redirect/r7-20261020/rejection-review.md), [handoff](redirect/r7-20261020/handoff.md). Maps/source/ROM/toolkit preserved; no new execution. No toolkit issues found.


## Round8 disassembly — revised policy, full structural classification

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Revised user policy and attachment independently evaluated. Accepted13538previously Unknown bytes in72regions:13276StaticData/70unlabeled generic regions and262StaticCode/2regions/144starts. All1801prior region objects unchanged;1873total,22881Code/11451starts,108182Data including16header bytes,25Text. Exact Unknown complement empty. No unresolved effective bank contexts in accepted code views; natural entry/use remains unconfirmed for new static routines and many data blocks.

[Structural review](disassembly-r8/review.md), [exact boundary review](disassembly-r8/data-boundary-review.json), [routine audit](disassembly-r8/routine-audit.json), [all11307old-start conflict scan](disassembly-r8/code-conflicts.json), [accepted annotations](disassembly-r8/accepted-regions.json), [counts](disassembly-r8/classification-counts.json), [empty complement](disassembly-r8/unknown-ranges.json), [preservation](disassembly-r8/preservation.json). Generic Data deliberately claims only non-code role; no invented format/consumer/padding/reserve semantics. L013verified252-byte9354alternate terrain updater under[12,13,14,15], independent of prior9350failure;1024threshold-selector and33792loop-state cases checked. L014verified generic non-code bulk and interior field gaps. L015verified10-byteC780signed scaling prefix into existingC78Ahelper; all256signed inputs checked, old suffix preserved. See analysis/leads.json.

| Bank8 | Code | Data | Text | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 0 | 0 |
| 1 | 0 | 8192 | 0 | 0 |
| 2 | 0 | 8192 | 0 | 0 |
| 3 | 0 | 8192 | 0 | 0 |
| 4 | 0 | 8192 | 0 | 0 |
| 5 | 0 | 8192 | 0 | 0 |
| 6 | 638 | 7554 | 0 | 0 |
| 7 | 0 | 8192 | 0 | 0 |
| 8 | 0 | 8192 | 0 | 0 |
| 9 | 0 | 8192 | 0 | 0 |
| 10 | 3273 | 4919 | 0 | 0 |
| 11 | 5979 | 2213 | 0 | 0 |
| 12 | 5156 | 3036 | 0 | 0 |
| 13 | 1138 | 7054 | 0 | 0 |
| 14 | 5845 | 2322 | 25 | 0 |
| 15 | 852 | 7340 | 0 | 0 |

Fresh configured `python3 run.py inspect` passed. Fresh map.validate/save/export passed; internal=current=pass008. Fresh `python3 run.py checkpoint pass-008-reviewed` validated/generates/ca65+ld65 reassembles all131088bytes byte-identically, ok=true/byte_identical=true. [Verification](pass-008-reviewed-verification.json), [checkpoint](../maps/pass-008-reviewed.json), [generated listing](../generated/pass-008-reviewed/game.asm). ROM SHA256115b50b36d4dfb7beaa032f868f1b6f11c8384764c79428fe821c1e23f5ebdc6 unchanged. Existing configured release used, no toolkit/source edits/rebuilds or commits. All24prior maintained commentary source hashes match r7; labels, variables, branch comments, crosswalk and347unit/11307start review credit retained.

Broad gameplay checklist persisted in [scenarios.json](scenarios.json), with [gameplay familiarity](gameplay.md). Supported MainState6destination pipeline visits12/17/24/3A/47: final numeric areas03/07/0D/16/1B, each distinct rendered active gameplay plus move/attack/jump/release inputs. Exact patches0300..031Ftask set,1Ddestination,41step,CA/CBhealth48, clean exported snapshots, budgets and effective banks retained. Area names not inferred; rope/water scenery is not claimed traversal. Full frame and actual accepted fetch evidence saved. Successful status spell: Mind20/selector1/freshA -> Mind15/CD10. Boss arena47visited and two bounded far/close combat attempts emitted weapon/projectiles and fetched attack initializer; boss health05E2remained15, no hit or victory. Close-position patch0512=B0/0502=00 follows clean restore. Do not force indefinitely.

Ending uses established main10/step0 full22-step initializer, then supported timer acceleration from exported clean step8snapshot (0533=FF;2B=00 only signed-delay states; heldStart at15). Reaches main0/frame685 and visible terminal END. This is artificial progression, not natural completion/boss victory. [END](disassembly-r8/ending-terminal.png), [ending exact timer/input log](disassembly-r8/ending-timer-log.json), [spell recipe](disassembly-r8/spell-recipe.json), [boss recipe](disassembly-r8/boss-actions.json), [close attempt](disassembly-r8/boss-close-actions.json). Prior natural routes remain distinct.

All retained complete-paged artificial fetched roots are checked against current Code classifications with no Data conflicts; aggregate_dropped=0. Original long captures contain first1000roots, so all six original patch routes were deterministically repeated for complete pages:3511/3609/3853/3839/4340/2409contextual root records (21561summed records), all zero non-Code conflicts, aggregate_dropped=0 and exact original final CPU/state/RAM equality. No classification correction was needed. [Complete original-route audit](disassembly-r8/original-route-fetch-audit.json), [additional short/boss/spell/ending fetch audit](disassembly-r8/artificial-fetch-audit.json). Detail drops at zero-capacity tracing are expected; aggregate drops are checked separately. Snapshots remain local same-build artifacts; no full-hardware compatibility or natural reachability claim.

Structural classification complete, but full current-source readability/final acceptance remain pending. Next phase commentary must integrate72new map regions into preserved maintained source, comment new144instruction starts with variables/branches and proper CPU aliases, and replace/reconcile all71old Unknown scaffolds/prose with short generic-data uncertainty notes where applicable. Preserve previous347functional reviews/11307starts, labels/symbols/crosswalk; generic Data requires no invented names or meaning. Fresh maintained-source rebuild/read/hash verification and final editorial/archive audit still required. This worker did not run commentary or redirect.

No toolkit issues found; [report](toolkit-issues.md). Only portable NES JSON mapping is distributable; input, snapshots, traces, source/assets stay local.
