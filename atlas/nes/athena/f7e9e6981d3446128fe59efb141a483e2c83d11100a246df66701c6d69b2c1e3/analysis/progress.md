# Round 1 — disassembly pass complete, ROM incomplete

ROM classification: 32.52% (42631 / 131088 bytes classified; 88457 bytes unknown).

6483 Code bytes, 36148 Data bytes, 381 annotated regions. Header contributes 16 classified bytes outside PRG banks. Each classified region has evidence; semantic/editorial readability remains for commentary. Execution and structural coverage are separate. Observed decode retains 2878 distinct physical instruction starts; final trace summary reports 3602 unique instruction identities and 5435 contextual roots, no aggregate drops. Detail capacity0 intentionally drops all detail. Tracing was disabled during later-stage initialization acceleration, then enabled during 30-frame input exercises; do not infer fetch evidence during disabled intervals.

| Physical 8KiB PRG bank | Code | Data | Unknown |
| --- | ---: | ---: | ---: |
| 0 | 0 | 1217 | 6975 |
| 1 | 385 | 355 | 7452 |
| 2 | 0 | 450 | 7742 |
| 3 | 0 | 195 | 7997 |
| 4 | 0 | 0 | 8192 |
| 5 | 294 | 1141 | 6757 |
| 6 | 532 | 0 | 7660 |
| 7 | 704 | 0 | 7488 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 0 | 0 | 8192 |
| 13 | 0 | 0 | 8192 |
| 14 | 3292 | 0 | 4900 |
| 15 | 1276 | 6 | 6910 |

Verified baseline: [baseline report](baseline-verification.json). Final checkpoint: [verification](round01-disassembly-final-verification.json), maps/round01-disassembly-final.json, generated/round01-disassembly-final/game.asm. map.validate passed; whole-file byte_identical=true. Internal resumable state analysis/state.json and maps/current.json match final checkpoint. Earlier round01-disassembly checkpoint preserved.

Evidence: round01/contexts.json and observed-decode.json (accepted fetch roots + bank-aware decode), static-first.json (validated C008/C0CA vector graph), static-connected.json (conservative successors from observed contracts), call-candidates.json/call-accepted.json (69 additional bytes from direct JSR entry contracts). Calls/mapper writes stop static continuation; unobserved code remains Static. data-inspection.json records exact non-code intervals. Header/vectors, 32 maximal uniform runs and two graphics backing banks (32KiB) classified Data; precise graphics segmentation remains unclaimed. Tile/bitplane patterns across both banks and E5D5/E5DD bank selection plus transfer consumers support generic non-code classification.

Gameplay: title at natural frame234; Start held2 frames/released then120 frames, Right/A/B180 frames, Right/B180 frames reached tile scene at frame716. Natural active player/boss/ending not confirmed. Clean snapshot saved. One-shot health/time patches $95=$0F/$92=$3B followed Right/A/B; timeout retained partial progress frame879, capture saved. Later-stage selector $9C derived from graphics/setup/config consumers. Stages2/4/6/7 attempted with PC=$E73C scene initialization, clean restore each. Distinct graphics observed for2/4/6; stage7 returned forest-like scene, no ending. Player remains absent and $94 remains0: initialized scene alone does not prove playable stage visit. Each artificial attempt recorded in scenarios.json; snapshots retained.

Lead outcome: R01-PROGRESSION is ready for redirect: understand concurrent task/player spawn contract ($F200..F273, $94 initialization mode, C2B2/C2F2 scheduling) and use a supported complete scene/player transition. PC=$E73C alone is insufficient. Do not repeat long identical input runs. Graphics/data consumers and connected unknown code also warrant static investigation after commentary; no claim of terminal exhaustion. No preexisting leads to verify.

Toolkit: [toolkit-issues.md](toolkit-issues.md), no toolkit issues found. Bounded timeout/operation-budget stops are supported behavior; state inspected afterward. ROM hash enforced by run.py. Free disk8.5GiB, scratch28KiB at handoff; no cleanup performed.

## Round 1 commentary — in progress

Prepared commentary from round01-disassembly-final with the supported commentary.py script; preparation rebuilt byte-identically. Frozen baseline/state and before/ are preserved. Reviewed current assembly across all classified code families: title/HUD, object/OAM expansion, reset/NMI/transfer ring, cooperative task scheduler, six-voice audio, column/metatile/attribute streaming, mapper wrappers, scene setup and primary-object transition. Editing frontier: apply consistent names/operand constants and functional explanations to commentary/source/game.asm; populate physical review records, explain raw gaps/data, then fresh verify/read. No readability completion claimed yet. Concrete unresolved progression contract remains R01-PROGRESSION; mode-zero primary task follows entry/exit animation rather than demonstrating playable controls. Working edit script is /tmp/comment_round1.py; semantic descriptions and symbols are staged there, not yet applied. No classification change.

## Round 1 commentary — complete current-source readability

All 6,483 classified code bytes (3,149 emitted instructions), 36,148 classified data bytes and explanatory boundaries for 88,457 unknown bytes reviewed; 514/514 scaffold spans reviewed. Source now has 213 functional units and 222 used RAM/hardware/table aliases, consistent functional branch names and source-local explanations. Full current-source scope complete; no remaining commentary ranges. Structural classification remains 42,631/131,088 (32.52%), unchanged. Frozen baseline map/state equal current map/state. Final editorial_audit remains pending; no final audit/archive/publication.

Fresh supported commentary.py verify passed byte_identical=true (commentary/verification.json, source game.asm SHA256 c88c5f27be166e18c9de57fb8d6d54d34252674ff8d1c1378ce4922fd68409b4, constants.inc 2749bd852f02b084ea9a68e4125de874be6dbc50ecb5433b4efdf661d56d91fb); reading-after.asm refreshed after final edits. Input SHA256 still workspace identity. Durable source commentary/source, inventory review.json, uncertainty notes commentary/uncertainties.md; editing recipe analysis/round01-commentary-edit.py retained. Handoff analysis/round01-commentary-handoff.md. Next phase: redirect, beginning with supported primary-object/task initialization rather than repeating failed scene-only entry. Other connected structural opportunities are described in the uncertainty notes. No toolkit issues found. Disk 8.5GiB free, scratch44KiB; no deletion/cleanup.

## Round 1 redirect — active r01-connected

Current phase redirect, input round01-disassembly-final after completed commentary. Exact classification unchanged42631/131088, residual88457. Research/frontier: analysis/redirect/r01-connected/frontier.md. Old liveRPC51699 is gone; configured binary restarted via run.py serve without map/state/source edits. Task04 entryE460 stack47 and nonzero94 contract resolves the missing full scene/player initialization. Artificial complete stage0 now visibly playable at1694 (stage0-advanced.png and snapshot); prior absence was partly lengthy EFF7 reveal, supported by saved stack F0B2->E860. Representative later-stage complete-task visits are active, followed by bounded control capture. Audio command finite16 table/handlers, sprite optional/skip continuations, scheduler sleep paths are independent concrete structural findings under review. Exact unknown ledger saved; bank-split disposition completion pending before combined handoff. No archive/publication/toolkit change or terminal claim.

Redirect milestone: representative complete initialized stage2 andstage6 visits succeeded with activeplayer; selected captures/runtime/context evidence in analysis/redirect/r01-connected. Stage4 first process endedSIGTERM with no result and is resumed, not rejected. Static fixed connected graph proposes9843 unknown codebytes subject to independent control/stack/boundary review. Independent audio-format inspection now supports8608 unknown backingdatabytes under effective[4,5,14,15] via55 consistent header/sequence roots, consumer grammar, first-origin boundary and followingA1A0 code; no classification performed. Detailed active frontier and correction of an exploratory bank6/audio-context assumption recorded in frontier.md.

Redirect milestone: stage4 ice scene also visited successfully using complete task04 initialization plus traced controls; stage6 labyrinth actor/projectile capture preserved. Stage13 first subprocess interruptedSIGTERM before artifacts, restarted with authoritative terminal-state handling. Audio full55-root grammar check completed without escapes; dedicated stage/task table extent evidence saved. See appended milestones in analysis/redirect/r01-connected/frontier.md for current frontier (older startup paragraphs are history).

Redirect milestone: stage13 active gameplay confirmed. Directpage0B final-area patch did not reachboss/ending; grounded bitmap analysis identifies stage13 bossselector37 onpage5, and supported camera/worldpage5+spawn-counter reset experiment is active. Material existing-map correction ready:64bytes013E30..013E70 (CPUbank4BE20..BE60) are actual fetched code in visit6 despite prior whole32KiB GraphicsBacking Data. Preserve adjacent patternedData; revise only localized64-byte role and prior overbroad rationale. Frontier.md contains source/caller/context/boundary evidence and explicit table-domain limit.

Redirect latest: gameplay exploration complete through artificial composite final boss and post-boss terminal throne scene; scenarios updated. Current combined proposal84677 new bytes (25260Code,59417Data) plus64-byte local existingData-to-Code correction; accepted42631/88457 unchanged. Proposal checks show13291 unique instruction starts with no overlapping interpretations, mismatches, unofficials or code/data conflicts. Exact bank-split current unknown ledger sums88457. Lead writer has persisted12 ready IDs; final counts/backing additions and compact handoff are being finalized. Last substantive check follows already identified banked helper bodies (notably bank3B4C1) left untraced during setup; exact frontier in analysis/redirect/r01-connected/frontier.md. No terminal-exhaustion claim.

## Round 1 redirect — complete, combined batch ready

Handoff: analysis/redirect/r01-connected/handoff.md. Persistent analysis/leads.json now contains15 ready IDs with exact ranges/contracts, recipes, evidence and preserved outcomes. Accepted map remains42631 classified /88457 unknown; map/state/source/ROM hashes unchanged. Combined proposal88430 new bytes (26470Code,61960Data), plus64-byte local existingData-to-Code correction at013E30..013E70, would leave27 unknown bytes if fully accepted. These are proposed, not accepted counts.

Connected research covered complete task04/player initialization, active stages0/2/4/6/13, main/secondary actor dispatch and shared helpers, composite boss37 and an artificial post-boss terminal throne scene. Credits/natural defeat remain unverified. Audio/sprite container grammars, scene/task tables, bank-aware terrain/animation/motion/palette/text backing and generic patterned local data are batched for independent review. Ordinary static calling/field/stack contracts are separated from actual reachability. All supported banked helper bodies were followed; the draft BF70 data endpoint was localized/corrected before any map edit.

proposal-checks.json:14243 unique instruction starts/31849 code bytes, zero overlapping interpretations, mismatches, unofficials or proposal code/data conflicts. unknown-ledger.json is the exact current88457-byte complement with physical bank/CPU context, neighbors, proposed dispositions and nine unresolved27-byte fragments detailed in deferred-avenues.md. final-record-checks.json confirms all15 lead evidence links and unchanged immutable hashes. Current-source commentary remains complete for the old classification; new accepted work will require full current-source commentary. Final editorial/archive remains pending. No toolkit/source/map edit, archive/publication or cleanup/deletion. Disk8.4GiB free, scratch44KiB, selected durable redirect evidence94MiB. Next phase: independent disassembly review of the combined batch.

## Round 2 disassembly — active

Input round01-disassembly-final. Independent structural review in analysis/round02/independent-review.md and fresh primary-decode.json/.txt supports connected15-lead batch, including localized graphics correction. Primary consumers/stack/field/API contracts and exact boundaries reviewed; parser/decode legality is a consistency check. Source evidence typo84AD corrected locally to actual84BD without rewriting history. Applying supported annotations; nine27-byte residuals remain unresolved. Existing commentary/source untouched. Reusing representative progression and post-boss evidence; no unchanged replay justified. Current saved state still42631/88457 until accepted batch save.


## Round 2 disassembly — complete pass, ROM incomplete

ROM classification: 99.98% (131061 / 131088 bytes classified; 27 bytes unknown).

33017 Code bytes, 98044 Data bytes, including16-byte header outside PRG banks. Unique internal-map structural accounting; explicitUnknown and raw export fallback excluded. Accepted88430 new bytes (26470Code/61960Data), plus64-byte existingData-to-Code correction. No double counting. Per-bank exact counts:

| Physical 8KiB PRG bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 448 | 7744 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 898 | 7294 | 8192 | 0 |
| 6 | 633 | 7559 | 8192 | 0 |
| 7 | 1582 | 6606 | 8188 | 4 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 64 | 8128 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 7004 | 1181 | 8185 | 7 |
| 13 | 7077 | 1108 | 8185 | 7 |
| 14 | 7357 | 830 | 8187 | 5 |
| 15 | 7954 | 234 | 8188 | 4 |

Independent primary consumer/stack/field/bank review: [review](round02/independent-review.md), [fresh decode](round02/primary-decode.json), [exact code-boundary checks](round02/fresh-boundary-checks.json). All15 ready leads verified in leads.json with preserved historical attempts; no unsupported fragments forced into code. Graphics region314 split through supported map.annotate ID edit and new remainder annotations; left label preserved, obsolete whole-bank no-code rationale localized, historical checkpoint unchanged. Small evidence typo84AD localized to actual84BD. Specific known data/record roles and meaningful labels retained; generic patterned Data has practical bounded inspection and does not assert full semantics.

Final verified checkpoint **round02-disassembly-reviewed-final**: [verification](round02-disassembly-reviewed-final-verification.json), maps/round02-disassembly-reviewed-final.json and generated/round02-disassembly-reviewed-final/game.asm. map.validate passed and whole-file byte_identical=true; maps/current.json equals checkpoint map. analysis/state.json is supported saved internal state. Earlier round02-disassembly-final checkpoint remains preserved as pre-metadata-review history. Exact counts, physical residuals and hashes: round02/classification.json.

Residual27 bytes in9 intervals:00F9C3..00F9C7,018232..018235,01944E..019451,01967F..019680,01BE04..01BE0B,01C490..01C493,01D4AF..01D4B1,01E609..01E60A,01F2D1..01F2D4. NOP/fill, lone transfer/return and short literal/entry alternatives remain structurally unresolved. Reviewed rejection premises against accepted routine contracts; accepted helpers have independent field/API/family evidence, residual fragments do not. Exact alternatives and changed-evidence conditions in redirect/r01-connected/deferred-avenues.md; next redirect may review these narrow residuals after commentary. This is a single productive pass handoff, not a final whole-ROM exhaustion/completion claim.

Gameplay reused complete initialized active stage0/2/4/6/13, composite37 encounter, post-boss terminal throne scene and durable routes/snapshots in scenarios.json. Reviewed ending/credits obstacle and F2C4..F2DD pin/yield contract. No useful changed runtime condition identified; no unchanged long replay performed. Natural defeat/credits remain unverified; artificial observations and untraced setup retain their limits.

Prior commentary/source game.asm/constants.inc hashes unchanged. Full current-source commentary is now pending for new classification; final editorial/archive gates pending. No archive,cleanup,deletion,publishing or toolkit edit. [toolkit-issues.md](toolkit-issues.md): no toolkit issues found. Latest disk check8.4GiB free, scratch44KiB. Compact next-phase handoff: round02-disassembly-handoff.md.

## Round 2 commentary active

Entire previous commentary preserved at analysis/round-2/commentary-round01-preserved before supported fresh preparation. Exact physical code/data spans carried with original generated-label aliases; fresh byte-identical verification passed. See commentary/progress.md and analysis/round-2/carry-report.json. Current-source readability remains in progress; exact outstanding scaffold is commentary/review.json status=pending plus carried dependency/source-location checks. No redirect handoff yet.

Round2 commentary milestone: lower-bank28 plus actor1/actor2 source batches reviewed through physical $019819 with purpose/branch labels and shared constants. Fresh byte-identical verification passed after each. Exact outstanding intervals: analysis/round-2/commentary-frontier.json; pending actor/fixed/data/dependency work remains in commentary, no redirect handoff.

## Round 2 commentary — resumed after terminal worker error

The same commentary worker returned an authoritative terminal max_output_tokens error after prolonged live-status waits. Coordinator resumed that phase through followup_task rather than reinitializing. Worker confirms actor3 edits and byte-identical verification completed, while the oversized actor4 payload never executed; no tool/session remained active. Pending frontier is being reconciled and work continues in smaller saved/verified batches. This is not a readability-complete handoff or terminal exhaustion. Coordinator read-only map/ROM/count checks remain in coordinator-map-validation.json.

Actor3 and actor4a/4b applied and byte-identically verified; substantive actor review now through physical $01B2E6. Earlier progress frontier was superseded; no active tool/session. Pending scaffold: 236 total / 88 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

## Round 2 commentary — fresh-context continuation after second terminal error

The resumed round02_commentary worker returned a second authoritative max_output_tokens terminal error. No new durable batch beyond actor4b is present. Coordinator launched round02_commentary_cont with only the commentary prompt and compact checkpoint/frontier, preserving the same phase and all prior work. Current game.asm verified hash ed2cee2e08fcd9044fbcecd699c6a37faeaca0900b46ec25aacddfa5e427bfc8; exact pending inventory commentary-frontier.json (236 entries/88 Code, plus consistency/dependency/location work). Source must not be reprepared or applied scripts replayed. Fresh context uses small immediately executed edit batches. This context rollover is not phase completion, terminal exhaustion or a blocker.

Actor5a applied through physical $01B577; selector reservation and triplet sampler reviewed. Pending scaffold: 232 total / 84 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Actor5b applied through physical $01B827; shared direction, target and fixed-point screen-motion math reviewed. Pending scaffold: 231 total / 83 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Actor6 applied through physical $01BB6B; motion scripts and contact/hit responses reviewed. Pending scaffold: 230 total / 82 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Actor7a applied through physical $01BE04; contact windows and camera-edge placement reviewed; two missing actor6 entry labels corrected. Pending scaffold: 229 total / 81 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Actor bank substantive code complete through physical $01C00B; terrain placement searches and distance/contact helpers reviewed. Pending scaffold: 228 total / 80 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed1 applied: task delay/state APIs and title companion lifecycle reviewed. Pending scaffold: 223 total / 75 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed2 applied: audio bank restoration, queue clearing, stream handlers and bank6 service task reviewed. Pending scaffold: 219 total / 71 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed3 applied: stage metatile replacement, triggered object records and attribute-cache updates reviewed. Pending scaffold: 216 total / 68 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed4 applied: object and edge/current terrain-cache lookups reviewed. Pending scaffold: 214 total / 66 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed5 applied: layout lookup, flag clearing, byte negation and attribute-mode entries reviewed. Pending scaffold: 209 total / 61 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed6 applied: short mode setters/getters and split-entry boundaries reviewed. Pending scaffold: 204 total / 56 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed7 applied: reference helpers and auxiliary record-family selection reviewed. Pending scaffold: 202 total / 54 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed8 applied: terrain contact rules, layout classes, row offsets and flagged motion reversal reviewed. Pending scaffold: 201 total / 53 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed9 applied: upper terrain contact, primary/secondary stage lists and vertical cell-crossing tests reviewed. Pending scaffold: 197 total / 49 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed10 applied: shape-selected terrain probes, side/upper contact resolution and saved-position helpers reviewed. Pending scaffold: 195 total / 47 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed11 applied: contact-shape offset table consumer, terrain modification dispatch and stage threshold comparison reviewed. Pending scaffold: 194 total / 46 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed12 applied: packed-cache mutation, one-shot protection, edge snaps and direction offsets reviewed. Pending scaffold: 193 total / 45 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed13 applied: vertical crossing resolution, state40 phase reversal and upper terrain/layout snaps reviewed. Pending scaffold: 192 total / 44 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed14 applied: vertical delta, packed terrain-half selection and auxiliary records/frame sequences reviewed. Pending scaffold: 188 total / 40 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed15 applied: pause lifecycle, decimal score addition, reward thresholds and best-score update reviewed. Pending scaffold: 187 total / 39 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed16 applied: camera-page bounds, alternate coordinate subtraction entries and flag0 getter reviewed. Pending scaffold: 182 total / 34 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed17 applied: individual object flag APIs and auxiliary-object task dispatch reviewed. Pending scaffold: 177 total / 29 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed18 applied: player-attached offset streams and moving auxiliary sprite variants reviewed. Pending scaffold: 176 total / 28 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed19 applied: moving auxiliaries, terminal screen paths, fixed deltas and flashing sequence update reviewed. Pending scaffold: 174 total / 26 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed20 applied: counter exhaustion, graphics/audio wrappers, stage type remap and health dispatch reviewed. Pending scaffold: 170 total / 22 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed21 applied: scene-table counter initialization and graphics/health/palette/animation bank wrappers reviewed; two scene display wrappers retain explicit unresolved roles. Pending scaffold: 169 total / 21 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed22 applied: saved-scene reset continuation and split bank0 scene-step entries reviewed. Pending scaffold: 164 total / 16 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed23 applied: indexed scene uploads, initialization counters and stage-title display continuation reviewed. Pending scaffold: 161 total / 13 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed24 applied: full gameplay population task, grid/proximity/stream/type-bitfield spawn paths and maximum-health display reviewed. Pending scaffold: 160 total / 12 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed25 applied: camera spawn stream, population/slot allocation, grid cursor advance and saved-loadout initialization reviewed. Pending scaffold: 155 total / 7 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed26 applied: cache preload and transition tile walk, next-page reference, scene audio wait and scripted descent reviewed. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player1 source annotations applied for physical127700 CPUF2C4 through CPUF3B9; large spanning scaffold deliberately remains pending until all later blocks reviewed. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player2 source annotations applied through CPUF572; special terrain motion and jump initiation/recovery reviewed. Spanning region remains pending. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player3 source annotations applied through CPUF7D1; extra sprites and normal/special-health auxiliary attack animation families reviewed. Spanning region remains pending. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player3 duplicate internal label corrected (equipped special-health attack sequence12); fresh verification now passes. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Correction to preceding note: first label rename still failed because its branch retained the earlier duplicate name. Branch target now corrected; actual fresh byte-identical verification passed with source hash1de047579307fce1aad8a8cd7ca36567c2d9d77c2952760c9d10007408d2f179. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player4 source annotations applied through CPUFAA8; grounded/submode animation, terrain upgrade and selector-specific attack dispatch reviewed. Spanning region remains pending. Pending scaffold: 152 total / 4 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Player5 applied through CPUFB95; large player-state region now substantively reviewed across all five saved source batches, including removal animation. Three added code regions remain. Pending scaffold: 151 total / 3 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Camera1 source annotations applied through CPUFD58; scene/cache transitions and scripted exit reviewed. Spanning camera region remains pending. Pending scaffold: 151 total / 3 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Camera2 applied through CPUFEC1; camera mode scroll/clamps and initial stream population reviewed. Two added code regions remain. Pending scaffold: 150 total / 2 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Fixed27 applied: final added code reviewed through physical $01FFD2. All added Code scaffold now reviewed; full data/unknown source and naming/dependency/location checks still pending. Pending scaffold: 148 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Existing terrain backing source notes reviewed against preserved candidate samples and stage/scroll consumers; retained useful prose and explicit schema uncertainty for27 pending spans. Pending scaffold: 121 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Four audio/sprite backing containers now have readable format/consumer and overlap-boundary notes plus descriptive aliases; fresh byte-identical verification passed. Pending scaffold: 117 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

First five residual unknown ranges now have local boundary/competing-role notes based on preserved independent structural assessment; classification unchanged. Pending scaffold: 112 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

All nine residual unknown intervals now explained locally without decoding/reclassification; fresh byte-identical verification passed. Pending scaffold: 108 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

Context-pressure interruption after complete added-code substantive review,31 data-container/terrain reviews and all9 unknown notes. Exact continuation saved in analysis/round-2/commentary-continuation-code-complete.md;108 Data plus full consistency/location gate remain. Not a handoff. Pending scaffold: 108 total / 0 code; exact intervals in analysis/round-2/commentary-frontier.json. Carried dependency/source-location and branch consistency checks remain pending. Same commentary phase, no completion handoff.

## Round 2 commentary — data and consistency continuation

round02_commentary_cont stopped cleanly for documented context pressure after sustained verified batches. All added code and nine unknown notes are substantively covered; 108 Data spans and explicit branch/field/alias/dependency/source-location checks remain. Compact exact handoff: round-2/commentary-continuation-code-complete.md. No live process remains from that context. Fresh worker round02_commentary_finish continues the SAME phase from current source, without preparation/replay. Readability gate remains open; no redirect/final acceptance yet.

## Round 2 commentary — current-source readability complete

The same interrupted phase has now completed all 108 remaining Data reviews and the full branch/field/wrapper/table, carried-dependency and source-location consistency work. Source retains useful prior prose and honest limits, with no known current-source readability gap. All 698 regions / 473 meaningful units reviewed; exact scaffold frontier empty. Fresh supported byte-identical verify and refreshed reading-after passed. Map/state/ROM unchanged. Substantive bank-six wrapper and fixed-bank consumer corrections concern commentary, without invalidating accepted classification/views. Handoff: round02-commentary-handoff.md; exact validation/hashes: round-2/commentary-consistency-final.json. Final editorial/archive acceptance remains pending; coordinator owns that gate. No final audit, toolkit change, archive, cleanup, deletion or publication performed.

## Coordinator gate — round 2 commentary accepted, redirect active

Inspected round02-commentary-handoff.md full scope/readability assessment and current uncertainty notes. Independent source/reading hashes, empty exact frontier and current-vs-frozen map/state checks all pass; coordinator-commentary-gate.json records evidence. Current-source readability accepted for all 698 regions/473 units; final editorial acceptance remains pending. Separate sequential worker round02_redirect now investigates current structural opportunities and the nine residual intervals under evidence-policy.md. Classification remains 99.98%, 131061/131088 classified with27unknown; no terminal decision yet.


## Round 3 disassembly — pass complete, ROM incomplete

ROM classification: 99.99% (131073 / 131088 bytes classified; 15 bytes unknown).

33022 Code, 98051 Data including16-byte header; accepted5Code+7Data. Unique internal-map accounting excludes Unknown/raw export fallback.

| Physical 8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 448 | 7744 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 898 | 7294 | 8192 | 0 |
| 6 | 633 | 7559 | 8192 | 0 |
| 7 | 1582 | 6606 | 8188 | 4 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 64 | 8128 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 7004 | 1181 | 8185 | 7 |
| 13 | 7077 | 1115 | 8192 | 0 |
| 14 | 7362 | 830 | 8192 | 0 |
| 15 | 7954 | 234 | 8188 | 4 |

Three R02 leads independently verified; historical attempts preserved in leads.json. Primary acceptance and localized residual decisions: [independent-review](round03/independent-review.md), [fresh decode](round03/fresh-primary-decode.json), [exact counts/ranges](round03/classification.json). Title return fallthrough, stage tile generic backing and current-cache default prefix accepted Static. D49F expressly compared to accepted D955/DA4C/DF95 field/count specialization standards; actual caller is a separate unresolved claim. No complete14-entry stage-table schema asserted. Supported connected downstream paths already classified.

Saved analysis/state.json, exported maps/current.json, map.validate ok. Fresh checkpoint round03-disassembly-reviewed-final passed whole-file byte-identical verification; [verification](round03-disassembly-reviewed-final-verification.json). Maintained commentary/source and ROM hashes unchanged. New map SHA25655a0ad55c4785fc44585933422a031a3e6e284ee2df6c8588dcc13d152cf9ceb.

Residual15 bytes in6 exact half-open physical ranges:00F9C3..00F9C7(4),018232..018235(3),01944E..019451(3),01967F..019680(1),01E609..01E60A(1),01F2D1..01F2D4(3). NOP/fill, spare thunk/encoded bytes, return/literal and clearflag alternate-entry ambiguities remain localized. Existing primary rejection review reused with its precise scope, no invented universal natural-fetch prerequisite. Further independent entry/field-family/non-code-envelope evidence may reopen them; this is not terminal exhaustion.

Existing broad stage0/2/4/6/13, boss and post-boss routes and ending limitation reviewed/reused; no useful changed condition warranted unchanged replay or arbitrary tiny-fragment execution. Current-source commentary integration for12new bytes pending; prior full round2 prose/review must be carried, not rewritten unchanged. Final editorial/archive acceptance pending. No archive, deletion, cleanup, publication or toolkit edits. [Toolkit issues](toolkit-issues.md): no toolkit issues found. Disk8.4GiB free; scratch44KiB. Compact handoff: round03-disassembly-handoff.md.

## Coordinator gate — round 3 commentary accepted, redirect active

Inspected final scope/readability handoff and precise source diff; independent source/reading hashes and frozen map/state checks pass (coordinator-round03-commentary-gate.json). Full current readability accepted with698regions/476units; valid prior review preserved. Separate round03_redirect now reviews residual15bytes/sixintervals and any credible remaining structural methods. No terminal stopping rationale accepted yet; final editorial/archive readiness and document accounting remain pending.


## Round 4 disassembly — pass complete, ROM incomplete

ROM classification: 99.99% (131076 / 131088 bytes classified; 12 bytes unknown).

33025 Code,98051 Data including16-byte header. Accepted3Code Static bytes at physical01944E..019451 / CPU943E with effective[12,13,14,15]. Independent combined encoded-boundary and guard/deactivation semantic/normal-return contract review supports the thunk. Guard excludesB2; no179-word table, natural selection or Observed claim. Existing downstream bodies already classified. Lead outcome/history preserved.

| Physical8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 448 | 7744 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 898 | 7294 | 8192 | 0 |
| 6 | 633 | 7559 | 8192 | 0 |
| 7 | 1582 | 6606 | 8188 | 4 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 64 | 8128 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 7007 | 1181 | 8188 | 4 |
| 13 | 7077 | 1115 | 8192 | 0 |
| 14 | 7362 | 830 | 8192 | 0 |
| 15 | 7954 | 234 | 8188 | 4 |

Primary evidence: [independent review](round04/independent-review.md), [fresh configured decode](round04/fresh-primary-decode.json), [exact counts/ranges](round04/classification.json). Remaining five half-open intervals:00F9C3..00F9C7(4EA),018232..018235(3EA),01967F..019680(RTS),01E609..01E60A(RTS),01F2D1..01F2D4(JSRE108). Precise primary residual review carried from redirect/r03-rejected-selector-contract/rejection-review.md; no newly supported connected unknown bytes. Existing broad gameplay, boss/postboss and final yield-loop evidence reused; no useful changed runtime condition justified unchanged replay or forced fragments. Natural credits remain unverified. This is not terminal exhaustion.

Saved state and exported map validate; fresh round04-disassembly-reviewed-final whole-file byte-identical verification passed. Map SHA25660778e26c4705c5eeb146960415dfa7538253107508bfbcd1c811bc36d60e59d. Maintained commentary/source, reading copy, constants and ROM unchanged (round04/input-integrity.json). Next commentary carries accepted round3 full698-region/476-unit review and prose for unchanged spans and integrates these3new bytes, replacing one obsolete Unknown note with a static thunk explanation. Final editorial/archive gates remain pending. No toolkit/source edits, archive, cleanup/deletion or publication. [Toolkit issues](toolkit-issues.md): no toolkit issues found. Disk8.3GiB free; compact evidence only. Handoff: round04-disassembly-handoff.md.

## Coordinator gate — round 4 commentary accepted

Inspected full scope/readability handoff and precise diff; independent current-source/reading hashes and frozen map/state identity checks pass (coordinator-round04-commentary-gate.json). Readability accepted for698regions/477units, includingnewStaticthunkandboundaryworddependency withnaturalB2selectionunclaimed. Classification remains99.99%,131076/131088classified/12unknown. Required nextredirect assesses actual remainingmethods using soundlocalizedreviewcredit; no terminaldecision accepted yet. Finaleditorialaudit/documentaccountingremainpending.

## Coordinator terminal rejection review — returned narrow generic-Data question

Inspected round04 terminal reasoning and decisive primary artifacts;438sourceexcerptlines/currentROMresidualbytesmatch,closedspawn/uploader/playercontractschecked. Terminalstopnotaccepted: EA-runreviewrequestsadditionalnoncodeenvelopedespiterepeatedpattern+closedpredecessor+independentlyboundedfollowingData. Same round04_redirect resumed todiscriminate genuineCodealternativefromoriginalNOPpaddingemission undergenericDatapolicy. See coordinator-terminal-review.md forprecisechallenge/otherlocalizedconstraints. No sourcemapclassificationchanges; editorialgatepending.


## Round 5 disassembly — pass complete, ROM INCOMPLETE

ROM classification: 100.00% (131083 / 131088 bytes classified; 5 bytes unknown).

The100.00% value is rounded; five Unknown bytes prevent whole-ROM completion.33025Code/98058Data including16-byte header. Accepted R04-ISOLATED-EA-GENERIC-DATA: exactly7Static generic Data bytes at00F9C3..00F9C7 and018232..018235, contexts[6,7,14,15]/[12,13,14,15]. Combined uniform pattern, closed predecessor and independently bounded Data successor review resolves the prior extra-envelope premise. HUD unofficial successor and explicit five-byte record/RTI-stack competitor independently checked. No schema/fill-purpose/natural reachability claim or neighbor extent change. All prior reviews retained; leads attempt history appended.

| Physical8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 448 | 7744 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 898 | 7294 | 8192 | 0 |
| 6 | 633 | 7559 | 8192 | 0 |
| 7 | 1582 | 6610 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 64 | 8128 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 7007 | 1184 | 8191 | 1 |
| 13 | 7077 | 1115 | 8192 | 0 |
| 14 | 7362 | 830 | 8192 | 0 |
| 15 | 7954 | 234 | 8188 | 4 |

Exact residual half-open physical ranges:01967F..019680(1),01E609..01E60A(1),01F2D1..01F2D4(3). Singleton empty-return and coherent F2C1 alternate-code ambiguities remain; no new independent constraint. See [independent review](round05/independent-review.md), [fresh primary decode](round05/fresh-primary-decode.json), [exact classification](round05/classification.json). Saved API state/export validated; fresh round05-disassembly-reviewed-final rebuild verified byte-identically. Map SHA2565b7c245080286a9f36fb101b8822c56a6c3dbdf95024187ac5aba640840e3c3f. Maintained full round4 commentary/source/constants/reading and inputROM hashes unchanged (round05/output-integrity.json).

Broad initialized stage0/2/4/6/13,boss and artificial postboss evidence reused; no unchanged replay/arbitrary-PC execution. Natural defeat/credits remain unverified. Next commentary carries valid full round4 review and integrates only7new Data bytes plus boundary dependencies, removing two obsolete Unknown notes. One productive pass, not terminal exhaustion; editorial/archive acceptance pending. No toolkit/source edits, archive, cleanup/deletion or publication. [Toolkit issues](toolkit-issues.md): no toolkit issues found. Disk8.3GiB free; scratch44KiB. Handoff:round05-disassembly-handoff.md.

## Coordinator gate — round 5 commentary accepted

Inspected source diff limitedtotwogenericDatanotes,fullscopeassessmentandindependentcurrenthash/frozenmap/statechecks;allpass (coordinator-round05-commentary-gate.json). Fullreadability698regions/477units accepted; three residualnotesremainunchangedandhonest. Requiredredirectround5nowassessesremainingmethodsandterminalrationaleusingvalidreviewcredit. Classification131083/131088,5unknown;100.00%isroundedandROMINCOMPLETE. Finaleditorialacceptance/documentselectionpending.

## Coordinator terminal structural assessment accepted — INCOMPLETE

Accepted round05 redirect terminalrationale afterprimaryreasoninginspection,334currentexcerptlinecomparisons/ROMbytechecks andcomparisonwithcorrectedgenericDataandacceptedsmallCodecontracts. coordinator-terminal-review.md/.json recordexactscope. All20leadsverified,zeroready;no productiveinterruptedmethod. Classification100.00%ROUNDED(131083/131088;5Unknown);ROMINCOMPLETE. Finaltargetededitorialacceptanceandsupportedsourceaudit nowauthorizedaspreparationonly; noarchive/cleanup/publication. Finaldocumentselection/completionauditstillrequired.

## Terminal handoff — coordinator workflow finished, ROM incomplete

ROM classification: 100.00% (131083 / 131088 bytes classified; 5 bytes unknown).

100.00% is rounded. The remaining half-open physical intervals are 01967F..019680,01E609..01E60A,01F2D1..01F2D4. Terminal INCOMPLETE is justified by primary rejection review; the EA overconstraint was corrected through independent acceptance, and no useful remaining proposal or interrupted frontier survives. All 20 leads are verified.

Final editorial acceptance passed the supported audit byte-identically: 698 regions / 669 unit records, including 477 retained meaningful units and 192 split-fragment records. Source d094f9544c017aff015f4e498f2ce2b42a85f3f357aebeb71cb6ff31a59a11fc, constants: 3dbd69c2a6e52732e793843266402817a1bacdb1d436203c3da562f659dd46b8; map 5b7c245080286a9f36fb101b8822c56a6c3dbdf95024187ac5aba640840e3c3f. Independent final-verification.json and final-map-validation.json confirm current identities/inventory/context/counts. Progression evidence remains reusable; original early deferral now explicitly marked superseded, without erasing its reason.

Final handoff:final-handoff.md. Coordinator reasoning:coordinator-terminal-review.md; full requirement audit:coordinator-audit.md and completion-audit.json. Root archive-options.json explicitly selects final/history documents and classifies only two intermediate duplicate copies for omission; originals remain local. No archive, cleanup/deletion, publication or toolkit edits. This closes orchestration/preparation, not whole-ROM classification. Historical pending-gate reports are superseded by the final editorial/coordinator artifacts, not erased.
