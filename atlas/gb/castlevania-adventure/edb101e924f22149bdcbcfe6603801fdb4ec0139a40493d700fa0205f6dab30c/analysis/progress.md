# Disassembly progress

## Round 1 — disassembly (in progress)
Input checkpoint: initialized workspace; no classified map. Baseline established with `python3 run.py inspect` and `python3 run.py baseline`; analysis/baseline-verification.json confirms byte_identical=true for all 65536 bytes. Raw baseline is not classification.

Natural DMG startup and title Start inputs exercised effective switch banks 1–3. First 1,000,000 operations completed 91 VBlanks, 561 distinct physical instruction starts; title and accepted Start increased observed starts to 1168. Subsequent right-input gameplay exploration increased discovery candidates further. Contextual discovery inspected; RST $00 is an inline-table dispatcher (POP HL / RST $08 / JP HL), so unobserved fallthrough candidates are not automatically trusted. RPC evidence is local in analysis/runtime/rpc.jsonl and discovery pages. No existing lead IDs consumed.

A local helper paging error queried past terminal offsets; it was corrected. Interrupting that helper ended the local server before a snapshot was exported. This is an orchestration error, not evidence of a toolkit defect; retained RPC logs preserve fetched instruction evidence, and runtime will be replayed before further work. Round is incomplete.

### Saved classification batch 1
Internal map validated and saved: 3,155 / 65,536 classified bytes (4.81%); 62,381 unknown. This includes 2,592 initial observed code bytes, 437 further code bytes (observed/static separately preserved per region), and 126 Data bytes. Static additions are ordinary alternate branches, predominantly bank-2 audio paths, discovered with semantic RST00 no-return profiles. Every added block was decoded and exact end checked before annotation. Main state table, HRAM load image, cartridge header and I/O pairs have specific structural annotations. Required role/evidence metadata present on all regions. No checkpoint yet beyond baseline; current map export saved but gameplay evidence still accumulating.

Normal gameplay Start progression reached STAGE 01 / REST 02 interstitial, then transition to field around frame693; distinct observed starts reached 2023. Long-lived helper termination lost transient later coverage before it could be paged, so standalone bounded sessions now import saved snapshots and save subsequent snapshots before exit. This orchestration limitation is not classified as a toolkit bug. Snapshot analysis/runtime/replay-stage.snapshot.json restores frame353 deterministically. No existing leads consumed; R1-DISPATCH and R1-HRAM-DMA verified.

### Saved classification batch 2
ROM classification: 14.83% (9722 / 65536 bytes classified; 55814 bytes unknown).
Code 8727, Data129, Padding866. Round1 phase remains disassembly. First field confirmed on local screenshot analysis/runtime/field753.png after natural stage1 transition; replayable snapshot field753.snapshot.json saved. 3097 distinct ROM instruction starts during frame353–753 replay; contextual discovery proposed4233 candidates with profile safeguards. Inspected every added block with decoder and exact terminal check; added5698 code bytes, preserving observed versus static provenance. Static evidence remains static, independent of execution coverage. Fill in inspected unused vector/header gaps and bank tails classified separately; last switch-bank bytes01/02/03 are bank identity data, supported by actual bank save/restore routines reading CPU7fff. All metadata has role and evidence; map.validate ok; state/current.sym saved. First combat experiment now holds Right and pulses B, periodically A, in20-frame units, to expose movement, whip, jumping, enemy and scroll paths. Still productive; not stopping yet.

### Saved classification batch 3 and checkpoint
ROM classification: 16.95% (11109 / 65536 bytes classified; 54427 bytes unknown).
Code10046, Data129, Padding866. Combat route frame753–953 holds Right; for each of10 twenty-frame segments presses B for8 frames then releases for12, presses A during segments i%3==1 and releases at8. Sprite moved from opening tree to candle; screenshot combat953.png, persistent snapshot combat953.snapshot.json. Contextual discovery and candidate inspection added1387 code bytes; byte classifications include static reachability, not only runtime coverage.

`python3 run.py checkpoint round-001-disassembly-a` passed map validation, generation, and byte_identical=true verification of65536 bytes. Exact stock map maps/round-001-disassembly-a.sym; source generated/round-001-disassembly-a; report analysis/round-001-disassembly-a-verification.json. Map hash38a62d4f012d9b137040281ab6fc4073bee8cb5f028959357d33277dc0d599f3. Per-bank counts at this checkpoint in analysis/runtime/batch3-counts.json. Continuing one continuous-Right scenario toward first enemy/scroll; whip-heavy movement was slow. No toolkit issues found; helper lifecycle/paging errors were local orchestration errors.

### T001 and corrected movement experiment
The nominal continuous-Right frame953–1193 scenario added only20 bytes but unexpectedly left player still. Concrete experiment demonstrated snapshot restoration loses held input (both named and persistent), despite documented input persistence. Cached c00c=01 after combat snapshot import became00 after2 frames, then explicit Right press restored01. Minimal FF00 row-select repro confirms EE held -> EF released -> EF after either restore (expectedEE). This is recorded as confirmed bug T001 in analysis/toolkit-issues.md. Movement route assumption is withdrawn; runtime observations remain valid. Workaround reasserts buttons after snapshot import. Corrected movement is the next obvious promising alternative, not a repeat of a failed route with unchanged evidence.

### Corrected movement batch
ROM classification: 18.05% (11829 / 65536 bytes classified; 53707 bytes unknown).
After T001 workaround, corrected frame953–1193 movement scrolls the field, earns score50 and encounters first enemy adjacent to player. Saved screenshot/snapshot walkfixed1193;12*20frame runs with explicit Right press and all other buttons released. This added700 code bytes after20 earlier nominal-walk bytes (all actual decode/fetch evidence remains valid). Validated state/current.sym saved. Running short opening-enemy jump/whip encounter next; substantial gains continue, but difficult level progression is deferred when gains diminish. Exact action logs in scenarios/RPC records; summaries preserve detail drops separately from aggregate drops.

## Round 1 — disassembly pass finished, ROM incomplete

ROM classification: 18.57% (12173 / 65536 bytes classified; 53363 bytes unknown).

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 7825 | 166 | 73 | 8064 | 8320 |
| 01 | 1229 | 5 | 67 | 1301 | 15083 |
| 02 | 1972 | 1 | 37 | 2010 | 14374 |
| 03 | 108 | 1 | 689 | 798 | 15586 |


Input checkpoint: initialized workspace (no baseline/store). Output checkpoint: `round-001-disassembly`; phase disassembly finished; commentary/redirect not executed by this worker. Prior coherent checkpoint `round-001-disassembly-a` remains available. Final internal store analysis/state.json; stock map maps/current.sym and exact frozen maps/round-001-disassembly.sym; bank-organized source generated/round-001-disassembly. `python3 run.py checkpoint round-001-disassembly` passed map validation, generation and byte_identical=true for all65536 bytes. Verification report analysis/round-001-disassembly-verification.json; map SHA-256 5b4da10df282060e4f84be2608e8ae9f41b10ddb744602d320418b3a5f542d73. ROM SHA-256 still edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c.

Totals: Code11134, Data173, Padding866; no Image/Text classification yet. All432 regions have role and supporting evidence. Stored provenance bytes: Observed8251, Static3922, Manual0. Independent union of actual fetched ROM instruction starts across saved discovery/RPC evidence:4890 starts covering9144 opcode/operand bytes; later fetches of previously static blocks do not automatically change their conservative store provenance. Execution coverage is distinct from classification; no editorial review completion claimed. Aggregate drops in saved trace summaries were0; detail capacity0 deliberately discarded detail while retaining aggregate/context evidence.

Natural scenarios: startup/logo/title, Start transition, stage1 interstitial and first forest field; Right, B whip and periodic A jump, opening enemy defeat/score80, scrolling, further movement/encounter with visible flying enemy and health reduction; then Start pause/resume input alternative. Later levels/ropes/bosses not reached. Reproducible exact actions and outcomes consolidated in analysis/scenarios.json; local snapshots field753/combat953/walkfixed1193/combat1393/walk1633/pause1653 in analysis/runtime. Reassert desired buttons after every snapshot import (T001). Historical walk1193 without Right reassertion is superseded; it was not sustained-Right progression.

Stopping evidence: first field +5698code bytes, initial combat +1387, corrected Right +700, opening-enemy combat +169, subsequent Right/flying-enemy/damage +6, pause/resume alternative +125; six inspected inline tables +44Data bytes. Gains diminished after productive normal gameplay and obvious alternatives. Remaining bulk graphics/level/audio structures and later gameplay need targeted consumer/domain/route work rather than more blind input repetition. This is the single easy-gain stopping condition, not a claim of exhaustive investigation or whole-ROM completion. Exact complementary unknown ranges analysis/round-001-unknown-ranges.json; metrics analysis/round-001-classification.json.

Lead IDs consumed: none (initial list empty). Produced/outcomes: R1-DISPATCH verified; R1-HRAM-DMA verified; R1-INLINE-TABLES verified; R1-PADDING verified; R1-OPENING-PLAY verified with diminishing gains; R1-UNREASSERTED-WALK superseded by T001 workaround; R1-PAUSE verified experiment. None is a ready request to repeat unchanged play. Deferred: selector domains/handler semantics, bulk asset consumers/extents, later-level route and bank-context semantics. These are inputs for commentary/redirect, not speculative completed classification.

Toolkit findings: one confirmed bug (T001 snapshot held-input restoration), zero suspected bugs, zero missing-feature findings. Workaround effective, no blocker; see [toolkit report](toolkit-issues.md). No toolkit or ROM changes. Local socket-helper paging/lifecycle errors were orchestration errors and did not justify toolkit findings. Compact handoff: [round-001-disassembly-handoff.md](round-001-disassembly-handoff.md).

## Round 1 — commentary finished; redirect pending
Input checkpoint `round-001-disassembly`; output commentary/source with unchanged classification map/state. Selected incremental scope: dispatch, startup/input, banked sound wrappers, HRAM DMA, asset consumers. Twelve compact review units; remaining scaffold and editorial audit pending. No lead IDs consumed/produced. Useful questions for redirect: exact360-byte tilemap at0cce, six-byte asset lists with explicit caller-bank context, and selector-domain/state-writer checks. No exhaustive editorial completion claim.

Preparation and both coherent/final commentary verifications passed byte_identical=true for65536bytes. Reading copy refreshed successfully after final edits. See [commentary handoff](round-001-commentary-handoff.md), [verification](../commentary/verification.json), [commentary progress](../commentary/progress.md). ROM classification remains18.57% (12173classified,53363unknown). Toolkit: T001 confirmed, no new findings and no blocker; [report](toolkit-issues.md). Coordinator next phase: round1 redirect, not executed by commentary worker.

## Round 1 — redirect pass finished (2026-10-17)
Input/output classification checkpoint: `round-001-disassembly`, unchanged. Input commentary: [compact handoff](round-001-commentary-handoff.md), accepted partial review immediately. Exactly one static-first redirect pass; no disassembly/commentary phase or classification edits performed. ROM remains incomplete: **18.57% (12173 / 65536 classified; 53363 unknown)**; per-bank counts remain those of the disassembly checkpoint above. No new execution coverage, gameplay route, snapshots or debugger mutations.

Searched reviewed consumers, uncommented/generated sections in all four commentary source banks, literal table loads, bank-selection wrappers, indirect dispatch and unresolved selectors. Confirmed six literal tile-transfer wrappers and the stage selector at0862..08a0: five pointers0b8d..0b96, selector masked3 or explicit4, wrapper0e36 selecting bank3. Eleven lists all terminate at inspected zero destination words. Their record/payload union is17298 currently unknown bytes, plus10-byte stage pointer table. Overlapping payloads were deduplicated for this count. Bank1 wrapper0e91 does **not** make its sources bank1:12b4..15a3 lies in the fixed bank. Separate20x18 tilemap consumer proves exact360-byte source0cce..0e35; nearby zeros remain outside that claim. These are candidate structural leads, not new classifications.

Persistent [lead registry](leads.json): added `R1-TILE-TRANSFERS` (ready, highest), `R1-FIXED-TITLE-MAP` (ready, high), `R1-STATE-DOMAINS` (deferred), `R1-AUDIO-DATA-DOMAINS` (deferred). Updated IDs: none. All seven historical entries/outcomes retained unchanged. Exact per-record physical extents and effective banks: [transfer candidate evidence](redirect/round-001-transfer-candidates.json). State writers require caller/dataflow bounds; sound six-channel RAM/offset tables offer18 immediate bytes but audio header/stream extents need selector grammar. No unchanged failed gameplay/dispatch lead was reopened.

Verification: `python3 run.py inspect` confirmed original65536-byte SHA256 edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Started configured `run.py serve`, inspected `api.describe`, loaded analysis/state.json and `map.validate` returned ok; bounded structure queries retained in [RPC results](redirect/round-001-api.json). No state save/export/checkpoint needed because map unchanged. Retain exact successful disassembly checkpoint [verification](round-001-disassembly-verification.json) and commentary [verification](../commentary/verification.json), both byte_identical=true65536bytes; those checks were not rerun here. JSON registry and record arithmetic validated by local scripts.

Recommendation: **begin round2 immediately** on the two new ready leads. Expected candidate gain17668 unknown bytes including pointer table, subject to next worker inspection/validation. Remaining unrelated bank bulk, late gameplay, state reachability and audio grammar deferred; this pass is not exhaustive source review. Toolkit: existing one confirmed bug T001, zero suspected bugs/missing features; no new issue/blocker. [Toolkit report](toolkit-issues.md); restore workaround remains explicit button reassertion.

## Round 2 — disassembly pass finished (2026-10-17), ROM incomplete

ROM classification: 46.85% (30702 / 65536 bytes classified; 34834 bytes unknown).

Input phase/checkpoint: disassembly, `round-001-disassembly` (12173 classified). Output phase/checkpoint: disassembly finished, `round-002-disassembly`; commentary/redirect not executed. Preserved prior store/classifications. Exactly one static-first easy-gain pass consumed two new ready leads. Configured release binary unchanged; ROM hash confirmed by `python3 run.py inspect`.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 7878 | 2322 | 73 | 10273 | 6111 |
| 01 | 1229 | 5 | 67 | 1301 | 15083 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 108 | 12289 | 689 | 13086 | 3298 |


Totals Code11187, Data18649, Padding866; Image/Text0. Added18529bytes:18476Data plus53Code. All478/478 regions have role/evidence; provenance Observed8251, Static22451, Manual0. New planar tile payloads are represented as Data with specific tile roles; no new stock image directives required. Classification is semantic/static evidence, separate from execution. No runtime, debugger patches, new snapshots or gameplay routes this pass; retained round1 cumulative evidence4890 fetched starts/9144 instruction bytes. Diminished normal gameplay was not repeated without a new concrete prospect. Gameplay familiarization unchanged: existing sourced routes remain recorded in gameplay.md, but do not contribute classification evidence.

Consumed `R1-TILE-TRANSFERS` verified: independently decoded wrappers/consumer0e47 and tile loop0eae/0ebd, parsed every six-byte record and zero first-word sentinel, checked all positive counts and bank-contained payload extents, physically read every payload in<=4096byte requests. Stage domain masks3 or explicit4 proves five-word0b8d pointer span. All original candidate17308bytes (17298 union+10 pointer table) supported; deduplicated overlap partitioned with precise per-record evidence. Source addresses below4000 remain physical bank0 even wrapper0e91 switches tobank1; higher pointers resolve from explicit effective switch banks2/3 in this4bank MBC1 ROM. The common loop multiplies count by16 and copies unchanged bytes toVRAM; every inspected record's upper selector bits14/15 arezero, so no broader flag behavior asserted.

Consumed `R1-FIXED-TITLE-MAP` verified:0caf..0ccd reads20bytes per18rows from0cce, writes9800 with32cell destination stride; exactly360bytes through0e35, adjacent0e36 independently code. Zero cells inside this exact consumer extent are map data; no nearby zero-padding inference.

Produced `R2-FIXED-INTRO-TILES` verified: existing classified0ec8 loads0ef9 and calls0e36. The eight-byte terminated list has destination01c0,count50,source0f01; establishes800fixed-bank payload bytes0f01..1220 plus8list bytes. Destination9400..971f; switchbank3 does not change fixed sources. Added53static code bytes in exact wrappers0c63..0c88 and0e7a..0e88 (decoded whole instructions ending shared tail jump, same save/select grammar as classified wrappers); no execution/natural reachability claim. Exact evidence [verified rows](round-002/verified-transfers.json), [inspection RPC](round-002/inspection-rpc.json), [annotation parameters/results](round-002/annotations.json).

Single-pass stopping: the two actionable substantial leads and obvious adjacent fixed-source transfer are resolved; bounded consumer call-site review exposes no further new substantial transfer payload. Remaining state/audio leads explicitly deferred without established domains; blind unchanged gameplay already diminished inround1. Stop incomplete under easy-gain rule, without exhaustive analysis of harder structures. Remaining unknown34834bytes with exact ranges [round-002-unknown-ranges.json](round-002-unknown-ranges.json); full metrics [round-002-classification.json](round-002-classification.json). Bulk remaining banks1/2, bank0 level/state tables and bank3 remainder need fresh consumer/grammar evidence; no blanket data claims.

Verification: map.validate returnedok after annotations; map.save analysis/state.json; map.export maps/current.sym. `python3 run.py checkpoint round-002-disassembly` passed stock map validation, generation and byte_identical=true for all65536bytes. Frozen map [round-002-disassembly.sym](../maps/round-002-disassembly.sym), bank-organized source [generated/round-002-disassembly](../generated/round-002-disassembly), report [round-002-disassembly-verification.json](round-002-disassembly-verification.json). Map SHA2567e8fb59747fe2531eff72ea4bdb277a28e82226107a201b661a0f2da6ee198ef; ROM unchanged SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Exact478region count and overlap-free physical union independently checked from internal store after save; raw fallback unknowns excluded.

Toolkit: existing one confirmed bugT001, zero suspected bugs/missing features, no new issues/blockers. No runtime snapshots restored; reassert buttons after future imports. [Persistent toolkit report](toolkit-issues.md). No toolkit orROM modification. Compact [round2 handoff](round-002-disassembly-handoff.md).

## Round 2 — commentary finished; redirect pending
Input/output classification checkpoint `round-002-disassembly`; source output commentary/source. ROM remains 46.85% classified (30702 bytes), 34834 unknown. Full previous commentary preserved under analysis/round-002 before fresh preparation. All five existing reviewed physical records and useful comments/names carried and reverified; new bounded lists/title/common font/intro review added. Concrete redirect dependency: 69-byte addressed tile-run stream1221..1265 consumed by0ad2/caller0ec8, still unclassified. Final verification byte_identical=true65536bytes and reading copy refreshed; one intermediate missing cross-bank alias failed link and was corrected before final pass. Final editorial audit pending; no runtime/map/input/toolkit changes. Existing T001 only, no new issue/blocker. Lead registry unchanged; details and deferred work: [round-002-commentary-handoff.md](round-002-commentary-handoff.md).

## Round 2 — redirect finished (2026-10-17)

Input/output classification checkpoint `round-002-disassembly`, unchanged; partial commentary handoff accepted. Exactly one redirect pass; no other phase executed. ROM remains **46.85% (30702classified/65536;34834unknown)**. Scanned all four available source banks including generic labels, raw/uncommented and pending sections; checked literal tables, missing dispatch paths, bank wrappers, state/level/sprite/audio consumers. Static-first bounded investigation; no gameplay, debugger patches, runtime or new execution coverage. Existing gameplay record preserved.

Added `R2-SCROLL-TRANSITION-HANDLERS` ready: new explicit mask03 at2a86 bounds four words2a8d..2a94 and targets2a95/2b38/2bce/2bdf, with fixed0/switch3 from2754. Contextual discovery budget1800 completed without exhaustion,347instruction candidates662union bytes;400previously unknown bytes all within2a95..2d2b. Exact8table bytes offer408immediate candidate bytes. This is new missing-handler evidence rather than unchanged global-state speculation. Indirect writes stop discovery and need RAM/VRAM register proof before continuation. No candidate accepted merely to create a lead.

Added `R2-ADDRESSED-TILE-STREAMS` ready as supplemental:144unique candidate bytes,69intro and75bank3, parsed from consumer0ad2 grammar with FF terminals and FE destination restarts. Bank3 suffix7cea overlaps7ce2; raw caller before7c97 still needs toolkit decode. Added `R2-LEVEL-TABLE-DOMAINS` deferred:3505/3567/357b outer tables masked03, nested c412 domain and262f bank context unresolved; no bulk extent claim. Updated attempt history of `R1-STATE-DOMAINS` and `R1-AUDIO-DATA-DOMAINS`, both remain deferred. Prior entries/outcomes preserved in [registry](leads.json). Remaining bulk banks1/2 and bank0 level/audio structures lack closed domains/grammar; unexplained raw spans are unknown, not data.

**Proceed to round3 on the new transition-handler lead:**400bounded unknown code bytes plus8table bytes gives immediate substantial structural/behavior gain; tiny144stream bytes alone would not warrant cycling. Recipes and physical/bank contexts in registry. [RPC evidence](redirect/round-002-api.json), [source inventory/stream arithmetic](redirect/round-002-static-evidence.json), [compact handoff](round-002-redirect-handoff.md).

Verification: python3 run.py inspect confirms original65536byte SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Configured run.py serve api.describe/map.load/map.validate passed. Map unchanged, so no save/export/new checkpoint; prior round-002-disassembly and final commentary byte_identical=true65536byte verifications retained, not rerun. Toolkit: existing1confirmed bugT001,0suspected/0missing-feature findings,0new issues/blockers; [persistent report](toolkit-issues.md). No snapshots used; reassert buttons after future imports.

## Round 3 — disassembly pass finished (2026-10-17), ROM incomplete

ROM classification: 47.69% (31254 / 65536 bytes classified; 34282 bytes unknown).

Input checkpoint `round-002-disassembly`; output `round-003-disassembly`; phase disassembly finished, commentary next under coordinator. One bounded static pass preserved existing478regions and added552bytes:400Code,152Data. No runtime, debugger edits, snapshots, inputs or gameplay replay; sourced gameplay record and scenarios preserved. Prior opening-play gains had diminished; new ready static leads enabled this pass. Configured release binary and input ROM unchanged.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8278 | 2399 | 73 | 10750 | 5634 |
| 01 | 1229 | 5 | 67 | 1301 | 15083 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 108 | 12364 | 689 | 13161 | 3223 |

Totals Code11587, Data18801, Padding866, Image/Text0. Required role/evidence annotations490/490regions. Observed8251bytes; Static23003bytes; Manual0. Retained historical execution evidence4890ROM instruction starts/9144instruction bytes, unchanged; static classification is separate from execution coverage.

R2-SCROLL-TRANSITION-HANDLERS verified:2754 selects effective switch3, then2a7e calls2a86; c405 decrement/mask03 proves exact8inline bytes2a8d..2a94 naming2a95/2b38/2bce/2bdf. Verified RST00 semantics consume inline return. Bounded analysis.discover_context budget1800,use_observed=false,fixed0/switch3 completed347candidates/662unionbytes;400currently unknown instruction bytes. Independently decoded every new block and exact instruction ends in [blocks](round-003/blocks.json), [decoder](round-003/full-2a95.json), [discovery](round-003/discovery.json). These handlers manipulate window position ff4a/ff4b, scroll ff42 and tile/block buffers; precise phase meanings remain provisional. Seven partial blocks end at conservative indirect-store traversal stops; no unsupported continuation or mapper crossing added. Existing overlapping code remains untouched. Unresolved stores and tile arrays2e01/2e11 need independent extents/destination evidence.

R2-ADDRESSED-TILE-STREAMS verified:0ad2 reads little-endian destination, emits tile bytes, FE begins another destination, FF returns;0af0 waits for suitable LCD status. Intro0ec8 literal DE1221 yields six runs3/4/18/19/4/3, exact69bytes through1265. Bank3 literal loads in independently decoded7c20..7c96 and classified7cc0/7cf1 name11streams with suffix7cea overlapping7ce2;75unique bytes. All candidate bytes read with rom.read; exact run counts/destinations/terminators independently parsed in [verified streams](round-003/verified-streams.json). Raw callers were inspected as consumer evidence, not automatically accepted as executable from disassembly alone. Adjacent1266 and raw bank3 caller bytes remain unknown. Fixed pointers remain physicalbank0 despite bank switches; upper sources explicitly physicalbank3.

[Annotation requests/results](round-003/annotations.json) and [RPC](round-003/rpc.json) retain provenance and role/evidence. No smaller annotation overlapped an existing region, so no splits required. map.validate returnedok; map.save analysis/state.json; map.export maps/current.sym include_sym=false. `python3 run.py checkpoint round-003-disassembly` passed stock validation, generation and exact65536-byte rebuild: [verification](round-003-disassembly-verification.json), [stock map](../maps/round-003-disassembly.sym), [bank-organized listing](../generated/round-003-disassembly). Map SHA2568e3f9cc5b51f12403c097b95f0d55470877ff388d85957cb75a19ee9c300fc0d; ROM SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Full [metrics](round-003-classification.json), [unknown ranges](round-003-unknown-ranges.json).

Stopping incomplete: both new ready leads resolved. Remaining R2-LEVEL-TABLE-DOMAINS still lacks nested c412 bounds and caller bank evidence; historical global-state/audio leads remain deferred. Conservative indirect-store continuations promise only small extensions absent additional proof. No fresh obvious substantial scenario remains, so no blind repeat or difficult progression. Unknown34282bytes remain, principally banks1/2; raw export fidelity supplies no classification evidence. No new leads produced. Commentary/redirect not executed.

Toolkit summary: existingT001 confirmed snapshot-held-input bug;0new issues,0suspected bugs,0missing features, no blocker. Future snapshot restores must explicitly reassert input. No runtime this pass. [Persistent toolkit report](toolkit-issues.md). Compact [handoff](round-003-disassembly-handoff.md).


## Round 3 — commentary finished; redirect pending
Input/output classification checkpoint round-003-disassembly unchanged: **47.69% (31254/65536;34282 unknown)**. Exactly one bounded intermediate pass. Full previous commentary preserved under round-003/commentary-prior before fresh baseline; carried 22 review records/units and valid names/comments, then reviewed 12 new records covering552bytes. No prior review invalidated; actual untouched work/editorial audit pending. Source names/comments now explain masked four-phase transition, window motion, block/two-bit threshold cache and addressed tile streams. Nested c412/table bounds and raw continuation gaps remain questions for redirect. No leads consumed/produced by this phase; no runtime/input/map/toolkit changes. Final commentary verification byte_identical=true65536bytes and reading copy refreshed. Existing T001 confirmed bug only, no new issue/blocker; [report](toolkit-issues.md). [Compact handoff](round-003-commentary-handoff.md).

## Round 3 — redirect finished (2026-10-17)

Exactly one bounded redirect pass after partial commentary; input/output classification checkpoint `round-003-disassembly` unchanged. ROM remains **47.69% (31254classified/65536;34282unknown)**. Static-only investigation; no other phase, classifications, gameplay/runtime, debugger mutations, snapshots, source/ROM/toolkit changes. Gameplay/scenarios preserved. All four source banks inventoried, including raw/uncommented sections, generic labels, pending dispatch/consumer questions and bank selection.

Materially improved `R2-LEVEL-TABLE-DOMAINS` from deferred to ready/high: do not require the unresolved global c412 bound before investigating known initialized entry0.082e resets c412=0;3505/357b consumers mask stage03.27c2 guards c427<selected width,27f0 assigns column c410,2815 handles five rows c403=0..4;2875 computes layout offset25*floor(column/5)+5*row+column%5. First-entry widths25/30/53/45 support765candidate unknown map-index bytes. Stage2 width53 requires a253byte initial range and four3byte tails; unused final-screen cells remain excluded. Source/helper formulas provide a materially improved concrete recipe, not adjacency guessing. Broader c412 domain,262f effective switch-bank and c400 block payload extents remain unresolved.

Threshold question independently closed at candidate level:3567 selects356f/3573/356f/3577, each four-byte sequence endsFF;2918/2d0d scan until byte<=threshold, so maximum class3 holds for every input byte without knowing block contents.12threshold bytes supplemental only. Total lead gain813unknown bytes=765layouts+24outerpointer+8firstnestedpointer+4width+12threshold. Independent region-union count checked. No candidate annotated during redirect. Exact ranges, bytes/decodes, source inventory and formulas in [static evidence](redirect/round-003-static-evidence.json) and [RPC](redirect/round-003-api.json).

No new lead IDs; append-only histories preserve all15prior entries/outcomes. `R1-STATE-DOMAINS` and `R1-AUDIO-DATA-DOMAINS` rechecked, still deferred with no substantial improved recipe. Bank1 nested7bb9/object structures need domains/grammar; bank2 audio still lacks bulk command/stream bounds; bank0 c406 dispatch requires writers; seven conservative continuations,2e01/2e11 and1266 remain small or unproved. Raw bank3 callers do not prove execution. Do not retry these unchanged merely to cycle.

**Proceed to round4 only on materially improved R2-LEVEL-TABLE-DOMAINS:**813candidate bytes offer immediate substantial map structure;12threshold bytes alone would not justify another round. Complete independent verification in the next disassembly pass and reduce the candidate set if a domain assumption fails. [Registry](leads.json), [compact handoff](round-003-redirect-handoff.md).

Verification: python3 run.py inspect confirms original65536byte SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Configured release run.py serve api.describe/map.load/map.validate passed. Current map hash8e3f9cc5b51f12403c097b95f0d55470877ff388d85957cb75a19ee9c300fc0d matches frozen round003 checkpoint; state/map untouched. Existing round003 disassembly/commentary byte_identical=true65536byte checks retained, not rerun. No new checkpoint or save/export needed with unchanged map. Toolkit: existing1confirmed bugT001,0suspected/0missing-feature findings;0new issues/no blocker. [Persistent report](toolkit-issues.md); reassert all inputs after future restores.

## Round 4 — disassembly pass finished (2026-10-17), ROM incomplete

ROM classification: 48.93% (32067 / 65536 bytes classified; 33469 bytes unknown).

Input checkpoint `round-003-disassembly`; output `round-004-disassembly`; phase disassembly finished, commentary next under coordinator. Exactly one bounded static pass consumed materially improved `R2-LEVEL-TABLE-DOMAINS`; no new IDs. Preserved all490prior regions and metadata, adding813Data bytes in20regions. No runtime, debugger edits, snapshots, inputs or unchanged gameplay replay; sourced gameplay and scenario routes retained. No research needed for this local consumer proof. Configured release binary and ROM unchanged.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8278 | 3212 | 73 | 11563 | 4821 |
| 01 | 1229 | 5 | 67 | 1301 | 15083 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 108 | 12364 | 689 | 13161 | 3223 |

Totals Code11587, Data19614, Padding866, Image/Text0. Required role/evidence annotation coverage510/510regions. Stored provenance Observed8251bytes, Static23816, Manual0; historical actual execution4890ROM instruction starts/9144instruction bytes unchanged. Classification and execution coverage remain separate. Full [metrics](round-004-classification.json), complementary [unknown ranges](round-004-unknown-ranges.json).

Lead verified for c412=0 only: independently decoded082e reset, raw07b2 stage-reset sequence (consumer evidence, not newly classified code), RST08/28 index helpers,27c2 width guard,27f0 assignments,2815 five-row loop,2864 switch1 wrapper,2875 screen-major addressing,2918 and2d0d threshold scans. c02b&3 closes each outer four-word directory3505/3567/357b:24bytes. Nested selector0 supplies four words at350d/3527/351d/3547 and four widths3583=25,3590=30,358b=53,35a0=45. Layout index equals25*floor(column/5)+5*row+column%5 for column0..width-1,row0..4; independently simulated subtraction quotient/remainder and checked exact index-set union765bytes. Exact ranges35b0+125,3bf0+150,3853+253,3952+3,3957+3,395c+3,3961+3,3ede+225. Stage2 deliberately excludes ten unused final-screen cells; explicit post-save check confirms all remain unknown. No adjacent padding or broader nested extents inferred.

Three threshold vectors356f/3573/3577 each endFF at entry3.2918/2d0d compare unsigned, stop on A<=threshold; exhaustive256input check per vector proves class0..3 independently of level block data. Twelve threshold bytes accepted; stages0/2 share356f. All accepted tables/layouts are CPU<4000 physicalbank0; wrapper switch1 does not relocate them. c400 payload bounds and262f/44a7 effective context remain unresolved. Static data proof makes no natural execution/reachability claim for all stages. [Verified domains](round-004/verified-domains.json), [annotations](round-004/annotations.json), [inspection RPC](round-004/inspection-rpc.json), [supplemental](round-004/supplemental-inspection-rpc.json), [mutation/save RPC](round-004/rpc.json), [preservation check](round-004/preservation-check.json).

map.validate returnedok before/after annotations; map.save analysis/state.json; map.export maps/current.sym include_sym=false. `python3 run.py checkpoint round-004-disassembly` passed stock map validation, generation and byte_identical=true for65536bytes. [Verification](round-004-disassembly-verification.json), [stock map](../maps/round-004-disassembly.sym), [bank-organized listing](../generated/round-004-disassembly). Map SHA256db27386b327e14e3fbe336f12c35bfc1082271dff57108f93fbf5c3757c453fb; original ROM SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c confirmed with run.py inspect.

Easy-gain stopping: improved ready recipe fully consumed. Broader c412 domains, bank1 objects, bank2 audio grammar, global states and small indirect-store continuations lack fresh closed substantial evidence. No unchanged diminished gameplay replay or speculative block classification. Stop incomplete with33469unknown bytes; all15lead IDs/history preserved, accepted lead nowverified with explicit partial-scope retry policy. Commentary/redirect not executed by this worker; required phase handoff in [round-004-disassembly-handoff.md](round-004-disassembly-handoff.md).

Toolkit summary: existing1confirmed bugT001,0suspected bugs,0missing-feature findings;0new findings/no blocker. No snapshot used; future restore must reassert held/released buttons. [Persistent toolkit report](toolkit-issues.md). No toolkit orROM edits.

## Round 4 — commentary finished; redirect pending
Input/output classification checkpoint `round-004-disassembly` unchanged: **48.93% (32067/65536 classified;33469 unknown)**. Exactly one bounded intermediate commentary pass. Entire previous commentary archived at round-004/commentary-prior before fresh baseline. Carried34 reviewed physical records/units and useful names/comments; reviewed20 new Data records/813bytes. No prior credit invalidated; threshold-dependent comments revised.54 entries reviewed,569 pending; final editorial audit pending. Names/word references and comments explain first-entry screen-major layouts, widths, nested-zero scope, excluded stage2 cells, and independent0..3 threshold classes. Broader c412 domains, block extents and upper-window bank context remain redirect questions. No lead IDs consumed/produced; no runtime/input/map/ROM/toolkit changes. Carried and final source verifications passed byte_identical=true65536bytes; reading copy refreshed and current baseline equality/scaffold/source locations checked. Toolkit: existing T001 confirmed bug only,0new issues/no blocker; [report](toolkit-issues.md). [Compact handoff](round-004-commentary-handoff.md).

## Round 4 — redirect finished (2026-10-17)

Exactly one bounded redirect pass after partial commentary. Input/output classification checkpoint `round-004-disassembly`, unchanged: **48.93% (32067classified/65536;33469unknown)**. No candidate classification, other phase, source/toolkit/ROM edits or new execution coverage. Source-bank inventory includes pending/raw/generic sections; targeted descriptor/layout, render wrapper, threshold consumers, state, object and audio gaps inspected. Gameplay/scenarios retained.

Added ready/high `R4-FIRST-LAYOUT-BLOCKS`: materially new exact-ID payload recipe, **1016 conservative unknown bytes** (1008payload+8base-directory), not an unchanged global domain retry. Already verified c412=0 layouts give16/17/15/18distinct raw blockIDs.262f stores stage-selected base from bank1:41fe, words7000/7510/7320/77c0; caller context needs independent verification next pass.2754 explicitly selectsbank3 before2815;2864 temporarily selects1 for2875 and restores3 before2838's4x4 copy and28ce's four2918 calls. Exact16byte records therefore lie inbank3, physicalCPU+8000. Full1056byte union is unknown; exclude48bytes for stage3IDs65..67 until raw scrolling versus transition low6 interpretation is checked. No min/max interval, omitted-ID records or later nested-entry extents inferred. [Exact ranges/inventory](redirect/round-004-static-evidence.json), [API](redirect/round-004-api.json).

Natural field753 snapshot imported for inspection only: c400/c4017000,c4120,switch1 outside rendering. Snapshot bank is not tile payload bank. All8buttons explicitly released after import (T001), no CPU run/step, debugger memory/register patches or new execution evidence. Initial bank1 payload hypothesis rejected during source inspection; final artifact/lead uses explicit bank3 restoration chain. Future optional constructed-state function tests are bounded in registry and must remain separate from natural reachability.

Threshold class0..3 packs into unrestricted byte0..255 and does not itself expose a new ROM lookup bound. c412 descriptor writer262f/increment2aaa still lack globalmaximum; seven continuation gaps/2e01/2e11 and1266 lack substantial independent gain. Bank1 objects andbank2audio remain domain/grammar questions. Preserved all15previous IDs/outcomes, added16th; updated histories only for R1-STATE-DOMAINS,R1-AUDIO-DATA-DOMAINS,R2-LEVEL-TABLE-DOMAINS. [Registry](leads.json).

**Proceed to round5 only on new ready R4-FIRST-LAYOUT-BLOCKS**, independently verifying contexts/record boundaries and reducing candidates if an assumption fails. [Compact handoff](round-004-redirect-handoff.md). Verification: run.py inspect original ROM SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c; configured release api.describe/map.load/map.validate passed. Map/state hashes unchanged, map SHA256db27386b327e14e3fbe336f12c35bfc1082271dff57108f93fbf5c3757c453fb matches frozen round004. Retained prior disassembly/commentary byte_identical=true65536byte reports, not rerun. No new checkpoint/save/export with unchanged map. Toolkit: existing1confirmed bugT001,0suspected/0missing-feature findings;0new findings/no blocker. [Persistent toolkit report](toolkit-issues.md).

## Round 5 — disassembly finished (2026-10-17), ROM incomplete

ROM classification: 50.48% (33083 / 65536 bytes classified; 32453 bytes unknown).

Input checkpoint `round-004-disassembly`; output `round-005-disassembly`; exactly one bounded disassembly pass, commentary next under coordinator. Consumed `R4-FIRST-LAYOUT-BLOCKS`, now verified for conservative first-layout scope; no IDs produced. All510prior regions/metadata preserved;64new Data regions add1016bytes. No runtime, snapshots, inputs, debugger changes, new gameplay research or unchanged diminished route replay. Sourced gameplay/scenarios retained; static evidence sufficed.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8278 | 3212 | 73 | 11563 | 4821 |
| 01 | 1229 | 13 | 67 | 1309 | 15075 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 108 | 13372 | 689 | 14169 | 2215 |

Totals Code11587, Data20630, Padding866; Image/Text0. Required role/evidence coverage574/574regions. Provenance Observed8251bytes, Static24832bytes, Manual0. Actual historical execution4890ROM instruction starts/9144instruction bytes unchanged, separate from classification. [Metrics](round-005-classification.json), [unknown ranges](round-005-unknown-ranges.json).

Independent bank/consumer proof: initialization0150 selects bank1;0862 caller retains it across0e36 bank3 transfer wrapper, with interrupt sound wrappers preserving prior bank.262f reads four little-endian words atbank1:41fe via c02b&3/RST08, stores H/L toc400/c401:7000,7510,7320,77c0. This is bank-context-specific static proof; no global reachability assertion for every descriptor.2754 selects3;2815 calls2838 then28ce;2864 temporarily selects1 for2875 then restores3.2875 reads exact first-layout IDs, adds16*rawID to base and returnsDE.2838 reads4rows*4bytes consecutively;28ce resetsDE and calls2918 four times, each reads4bytes. Thus exact payload physical address=CPU+8000. RST08 and28 arithmetic independently decoded. Decoder probes initially starting014b/061d were unaligned exploratory probes, corrected to0150/061c; no code classifications derive from those probes.

Recomputed layout coordinate sets from round004 base/width, checked existing classified cells, exact16/17/15/18ID sets against ROM, excluded stage3 IDs65..67 and read each candidate before annotation. Accepted63records1008bytes plus8byte directory, exactly1016unique new bytes. No min/max spans, missing-ID records, later nested selectors or surrounding bytes inferred. Post-save preservation proves48withheld bytes and10unused stage2 cells remain unknown, all prior metadata unchanged and exact new-byte union equals accepted records. [Records](round-005/verified-records.json), [annotations](round-005/annotations.json), [RPC](round-005/rpc.json), [preservation](round-005/preservation-check.json).

map.validate before/after passed; map.save analysis/state.json; map.export maps/current.sym include_sym=false. Fresh `python3 run.py checkpoint round-005-disassembly` passed map validation, generation, byte_identical=true for65536bytes. [Report](round-005-disassembly-verification.json), [portable stock map](../maps/round-005-disassembly.sym), [listing](../generated/round-005-disassembly). Map SHA2568a9b2fc8be45fecb613ef41c9100f767d94c7bc94311ccc521dd6ea165eece25. run.py inspect confirmed original ROM SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Toolkit/configured release unchanged.

Easy-gain stop: ready conservative recipe fully consumed. Flag-sensitive stage3 IDs, global c412 domains, bank1 objects, bank2 audio and global states lack fresh substantial closed evidence. No unchanged blocked leads retried; no speculative filling or difficult progression. ROM remains incomplete with32453unknown bytes. Commentary/redirect not executed. [Compact handoff](round-005-disassembly-handoff.md).

Toolkit: existing1confirmed bugT001,0suspected/0missing-feature findings;0new findings/no blocker. No restore used; future restores must explicitly reassert all buttons. [Persistent report](toolkit-issues.md).


## Round 5 — commentary finished; redirect pending
Input/output classification checkpoint `round-005-disassembly` unchanged: **50.48% (33083/65536;32453 unknown)**. Exactly one bounded intermediate pass. Full previous commentary preserved at `round-005/commentary-prior`;54 prior physical review records/units and useful comments/names carried,64 new records/1016bytes reviewed. No credit invalidated; bank/base and raw-ID versus masked-ID consumer explanations revised. Stage3 IDs65..67 remain unresolved.118 scaffold entries reviewed/591 pending; final editorial audit pending. No lead IDs consumed/produced, no runtime/input/map/toolkit changes. Byte-identical commentary verification65536bytes passed; reading copy refreshed at handoff. Existing T001 confirmed bug only,0new/no blocker; [toolkit report](toolkit-issues.md). [Compact handoff](round-005-commentary-handoff.md).

## Round 5 — redirect finished (2026-10-17), cycle stops incomplete

Exactly one bounded redirect pass after partial commentary. Input/output checkpoint `round-005-disassembly` unchanged: **50.48% (33083/65536 classified;32453 unknown)**. No candidate classification, source/map/state/ROM/toolkit edits, runtime, snapshots, inputs or new execution coverage. No other phase executed.

**Stop the cycle incomplete; another round is not warranted.** No new or materially improved ready lead offering immediate substantial gain. Preserved all16 IDs and outcomes; added none, updated attempt histories only for `R1-STATE-DOMAINS`, `R1-AUDIO-DATA-DOMAINS`, `R2-LEVEL-TABLE-DOMAINS`, `R4-FIRST-LAYOUT-BLOCKS`. [Registry](leads.json).

Searched all4 source banks including raw/uncommented sections, generic labels and numeric table references; inspected selector reset/threshold/record writers, stage parameter loader, scrolling/transition paths, bank1 object consumers and bank2 audio consumers. [Inventory and findings](redirect/round-005-static-evidence.json). Scrolling2875 uses full byte and screen-major coordinates; transition2c82 masks3f and uses5*c410+c403, with2aaa incrementing c412 before rendering. Already classified nested0 scrolling cells do not bound transition cells or prove shared applicability. Raw stage3 IDs65..67 remain48unknown bytes at physicalfbd0..fbff, bank3 CPU7bd0..7bff. Resolving these alone is not substantial. No omitted-ID/minmax filling.

Larger remaining gaps: c412 descriptor/writer domain (262f bank1:44a7, c02a writers0968/1e98), nested bank1 object directory7bb9 and sound4916 command/stream grammar. No fresh closed substantial extents. Seven transition continuations/2e01/2e11 and1266 remain prior small or grammar-dependent opportunities, not reissued ready leads. Replay of diminished gameplay lacks a materially improved route. Current unknown ranges in [round005 report](round-005-unknown-ranges.json). Pending commentary remains118reviewed/591pending; final editorial audit not complete.

Verification: `python3 run.py inspect` confirmed original65536byte ROM SHA256 edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Configured release `api.describe`, `map.load`, `map.validate` passed ([RPC](redirect/round-005-api.json)). Current map SHA2568a9b2fc8be45fecb613ef41c9100f767d94c7bc94311ccc521dd6ea165eece25 equals frozen round005. State SHA2568e63b4b2b9e067a38315cd5786b40caa7131b2b62275cd25d72f9d7cf482325a unchanged. Existing disassembly/commentary byte_identical=true65536byte reports retained; not rerun. No new checkpoint/save/export for unchanged classification.

Toolkit: existing1confirmed bugT001,0suspected/0missing-feature findings;0new findings/no blocker. No restore used; future restores must reassert inputs. [Persistent toolkit report](toolkit-issues.md).

## Coordinator final handoff — five rounds finished

Sequential disassembly/commentary/redirect phases completed through round 5. Redirect found no new or materially improved ready leads offering immediate substantial gain; stop incomplete per goal.md. ROM classification: 50.48% (33083 / 65536 bytes classified; 32453 bytes unknown). Coordinator independently checked original ROM SHA-256, current.sym hash equality with round-005 checkpoint, byte_identical=true verification report, per-bank metrics and all three final phase handoffs. All checks passed. Whole-ROM classification and final editorial audit remain incomplete; goal is not marked complete. Final commentary has 118 reviewed/591 pending records. Existing toolkit T001 only, workaround available, no blocker. See round-005-redirect-handoff.md, round-005-classification.json and toolkit-issues.md.

## Workflow update — automatic bounded investigation (2026-10-17)
Active goal.md, workflow.md and disassembly/redirect prompts updated from the
toolkit templates. The prior easy-gain stop remains a valid historical outcome;
no bounded investigation pass has yet run. On next coordinator resume, schedule
investigation mode from round-005-disassembly before considering termination.
Prioritize one further level/substage descriptor and its consumers; independent
fallback is one known audio command/stream. Follow recorded hypothesis/runtime
budgets, preserve lead history and distinguish artificial states from natural play.
No ROM, map, analysis state, source or verification artifacts changed. Classification
remains 33083/65536 (50.48%), with32453unknown bytes. No new execution or rebuild
verification claimed by this prompt migration.

## Workflow update — research in every redirect (2026-10-17)
Supersedes the automatic bounded-investigation migration note above. Active prompts
now put thorough but reasonable research in every redirect, with no one-off allowance
or fixed hypothesis/experiment quota. Resume redirect from round-005-disassembly and
its commentary handoff. Investigate credible level/substage descriptor and audio
avenues while useful evidence accumulates; abandon rabbit holes and slim chances.
A concrete useful lead can justify another round without a substantial byte estimate.
Preserve prior attempts and record new evidence/methods before reopening stalled work.
No investigation has run as part of this update. Classification remains33083/65536
(50.48%),32453unknown; ROM/map/state/source and verification artifacts unchanged.

## Round 5 — resumed redirect finished (2026-10-17)

Resumed exactly the redirect phase from `round-005-disassembly`/round005 commentary under the revised policy. Historical redirect findings and all16 prior lead IDs preserved; prior stop is superseded by new useful bounded evidence, not erased. **Classification unchanged:50.48% (33083/65536 classified;32453 unknown).** No classification, map/state/source/ROM/toolkit edits or checkpoint/export. Existing recovery point and commentary remain intact.

New ready leads `R5-BOUNDED-RESPAWN-DESCRIPTORS` and `R5-TRANSITION-RECORD-SLICES` supply concrete next-disassembly work.0968's strict threshold scan maps explicitly bounded stage/c412 inputs to c02a43..50;262f bank1:44a7 consumes five-byte records and reloads nested c412. Fourteen constructed samples independently returned expected selectors and descriptors; stage2 input1 maps47→nested0. Exact descriptor-backed layout/width/pointer ranges contain1251unknown bytes, including nine additional nested slices; no global selector domain, natural reachability or adjacency filling asserted. Bank1 upper-window layouts4023/40a0/41cc must not be mistaken for bank0.0384 shifts width by5 (width*32), distinct from the layout's five-row enumeration. Thirteen independently bounded1e98/1fab lists provide205unknown pointer/record bytes including explicit two-byte high-E0 sentinels, preserving the different state-specific meanings of their fifth fields. [Research/evidence](redirect/round-005-resumed-evidence.json), [cases](redirect/round-005-resumed-cases.json), [record lists](redirect/round-005-resumed-transition-records.json), [registry](leads.json).

Runtime was artificial function testing only: fresh DMG emulator, effective switch1, writes to2180/c02b/c412, RAM return sentinelc100 on stackdff0, explicit pc0968 then262f; maximum150/200 one-operation steps per invocation, each timeout1000ms. All28calls returned;1810operations total,3752RPC requests,0errors. Exact patches/call/state/replies and replay script retained. No snapshots, input presses/releases or natural gameplay; default fresh-start inputs released. Debugger patches establish neither natural reachability nor execution of record/payload targets. No runtime map/history was saved. [RPC](redirect/round-005-resumed-api.json), [replay script](redirect/round-005-resumed-probe.py). Static audio fallback inspected4712/4916 header flag handling, but further stream grammar deferred once useful grounded level recipes established; no new audio claim or unchanged retry.

Verification: configured release api.describe/map.load/map.validate passed before/after; all candidate roles read directly from local ROM and no candidate overlaps Code. Hashes unchanged: ROMedb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c; state8e63b4b2b9e067a38315cd5786b40caa7131b2b62275cd25d72f9d7cf482325a; map8a9b2fc8be45fecb613ef41c9100f767d94c7bc94311ccc521dd6ea165eece25 equals frozenround005. Existing disassembly/commentary byte_identical=true65536byte verification retained, not rerun. No classification checkpoint needed for unchanged maps. Proceed to next disassembly on two ready recipes; final editorial audit remains pending. [Resumed handoff](round-005-resumed-redirect-handoff.md).

Toolkit: existing1confirmed bugT001,0suspected bugs/0missing features;0new findings/no blocker. No restoration used; reassert all buttons on future restores. [Persistent report](toolkit-issues.md).

## Round 6 — disassembly finished (2026-10-17), ROM incomplete

ROM classification: 54.63% (35803 / 65536 bytes classified; 29733 bytes unknown).

Input `round-005-disassembly`; output `round-006-disassembly`. Exactly one disassembly pass, no commentary/redirect. Both resumed ready IDs `R5-BOUNDED-RESPAWN-DESCRIPTORS` and `R5-TRANSITION-RECORD-SLICES` verified with independent decoded consumer/bank proofs and ROM reconstruction. Added2720Static Data bytes in99regions:1251bounded threshold/descriptor/layout,205transition directory/record,1264downstream exact-ID payload. Existing574regions/metadata and all lead history preserved. No global domain closure prerequisite; no min/max filling.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8278 | 4240 | 73 | 12591 | 3793 |
| 01 | 1229 | 441 | 67 | 1737 | 14647 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 108 | 14636 | 689 | 15433 | 951 |

Totals Code11587, Data23350, Padding866, Image/Text0. Role/evidence673/673regions. Stored provenance Observed8251/Static27552/Manual0bytes. Historical actual fetch coverage unchanged4890ROM instruction starts/9144instruction bytes, separate from classification. No emulator, inputs/releases, snapshots, debugger mutations or fresh execution this pass. Read and retained sourced gameplay/scenario routes; bounded static evidence sufficed, no unchanged diminished gameplay replay. [Metrics](round-006-classification.json), [unknown ranges](round-006-unknown-ranges.json).

0968 strict CP scan independently reconstructed14stage/inputintervals restrictedc412<99, selecting43..50.262f bank1:44a7 five-byte records reload nestedc412; stage2input1→47→nested0 distinction preserved.27c2width guard,2815five rows,2864bank1 wrapper and2875formula independently checked. Exact width*5cells exclude incomplete-screen padding; upper-window layouts are physicalbank1. Thirteen unique descriptor-backed pairs independently parsed through4206 nested directories:5byte rows,2byte high-E0termination;1e98 fifth field writesc02a only on success,1fab interpretation differs. No gate or natural later-stage reachability claimed. Downstream exact raw-ID records use bank3 base+16*ID after2754/2864bank restoration,2838four-by-four and28ce/2918consumer extent; added1264unknown bytes. Stage3raw IDs>=64 withheld;48bytesfbd0..fbff and incomplete-screen cells remain unknown. [Verified slices](round-006/verified.json), [inspection/mutation RPC](round-006/rpc.json), [annotations](round-006/annotations.json), [preservation](round-006/preservation-check.json).

map.validate passed before/after; map.save analysis/state.json; map.export maps/current.sym include_sym=false. Fresh `python3 run.py checkpoint round-006-disassembly` passed stock validation/generation/byte_identical=true65536bytes; mapSHA2565e2843a4a1ac36bc01451ec94416c00c6774ec89d2815c645f3e6b2f9a8a2d02 independently equalcurrent.sym. [Report](round-006-disassembly-verification.json), [stock map](../maps/round-006-disassembly.sym), [listing](../generated/round-006-disassembly). OriginalROM SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c unchanged; configured toolkit binary unchanged.

Orchestration correction: initial checkpoint ran before asynchronous classification finished saving and captured oldround005map. Retained as superseded `round-006-presave-recovery`, with corresponding report paths corrected. Required round006 checkpoint subsequently produced after process completion and independently hash checked. Original round005 recovery unchanged. This is a local invocation race, not a toolkit defect.

Easy-gain stop: both ready recipes and direct exact-ID downstream payload extension consumed. Further fifth-field descriptor targets are promising bounded research for redirect, but not unbounded recursive global closure; no unsupported bytes accepted. Stage3flag applicability, other nested lists, bank1object and bank2audio grammar unresolved; prior gameplay lacks new route. Handoff[round-006-disassembly-handoff.md](round-006-disassembly-handoff.md). Final editorial audit pending.

Toolkit: existing1confirmed bugT001,0suspected/0missing-feature findings;0new findings/no blocker. No restore used; future restores reassert all intended button states. [Persistent report](toolkit-issues.md). No ROM or toolkit changes.

## Round 006 — intermediate commentary

Input/output classification checkpoint `round-006-disassembly` unchanged: 54.63% classified, 29733 unknown bytes. Previous commentary preserved in `analysis/round-006/commentary-prior` before fresh preparation. Carried 118 physical review records with checked anchors and reverified source; reviewed all 99 new Data records (2720 bytes). No credit invalidated. Reviewed 217 scaffold entries; 641 remain pending, final editorial audit pending.

Updated strict threshold selection, descriptor selector versus reloaded nested index, upper-window bank-1 layouts, exact scrolling payloads, and the different transition fifth-field meanings. Existing masked transition indexing remains distinct from raw-ID scrolling. Replaced the stale first-layout-only limitation; prior names and comments retained where evidence agrees. No runtime, map, classification, ROM or toolkit changes. A missing carried transition label was detected by the first rebuild and restored at the unchanged physical routine before successful verification.

Redirect: inspect additional descriptor selectors from transition fifth fields with bank/context and exact extent proof. Stage-3 flagged IDs, natural gate success, global selectors, bank-1 objects and bank-2 audio remain deferred. See [compact review](round-006/commentary-review.json), [verified evidence](round-006/verified.json), and [handoff](round-006-commentary-handoff.md). Byte-identical verification of all 65536 bytes passed; reading copy refreshed at handoff. Existing T001 confirmed bug only; no new findings/no blocker. [Toolkit report](toolkit-issues.md).

## Round 006 — redirect complete; next disassembly warranted

Input checkpoint `round-006-disassembly` and compact commentary handoff; output checkpoint unchanged. Classification **54.63% (35803/65536),29733 unknown**. Exactly one redirect phase; no classification/map/source edits. Preserved all previous lead outcomes and recovery checkpoints. Added ready **R6-TRANSITION-DESCRIPTOR-LAYOUTS** and **R6-TRANSITION-DOWNSTREAM-SLICES** with605/734currently unknown candidate bytes (1339unique), backed by finite stage-specific transition selector fields,262f descriptors, exact width/coordinate sets, raw-ID bank3 blocks and sentinel-led transition lists. No substantial-gain/global-domain prerequisite. [Handoff](round-006-redirect-handoff.md), [evidence](redirect/round-006/evidence.json), [leads](leads.json).

Nineteen artificial262f calls: effective fixed0/switch1, explicit selector/stage writes, safe RAM stack returnc100,200-operation cap, all returned94..98operations and matchedc412/nested plusc40c-c40d=width*32. Clean snapshot is session-local, recreated by [script](redirect/round-006/research.py); all8buttons explicitly released after restore(T001). Detailed [RPC](redirect/round-006/rpc.json); reproducible setup linked from scenarios.json. No natural gameplay gate success or later-stage reachability inferred. Bank1 upper layouts andbank3 payload restoration retained; stage3 rawIDs>=64 withheld. New list fifth values require independent future descriptor checks. Generic/uncommented direct callees and pending uncertainties inspected where relevant; unrelated unchanged stalls not repeated.

Verification tested: run.py inspect original65536-byte SHA256; configured release api.describe/map.load/map.validate before/after passed; ROM/state/current.sym hashes unchanged (evidence artifact). Existing round006 disassembly/commentary byte-identical reports retained, not rerun. Unknown complement unchanged. Next phase independently verifies/classifies exact lead slices and makes a new checkpoint. Toolkit existing1confirmedT001,0suspected/0missing features,0new/no blocker; [report](toolkit-issues.md). ROM incomplete.

## Round 007 — disassembly complete, ROM incomplete

Input checkpoint `round-006-disassembly`; compact redirect handoff consumed. Exactly one disassembly phase; no commentary or redirect performed.

ROM classification: 56.67% (37142 / 65536 bytes classified; 28394 bytes unknown).

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8278 | 4591 | 73 | 12942 | 3442 |
| 01 | 1229 | 853 | 67 | 2149 | 14235 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 108 | 15212 | 689 | 16009 | 375 |

Verified R6-TRANSITION-DESCRIPTOR-LAYOUTS and R6-TRANSITION-DOWNSTREAM-SLICES independently, accepting1339unique previously unknown Data bytes in109regions:605descriptor/layout bytes and734downstream bytes (576exact raw-ID block bytes,158transition pointer/record/terminal bytes). Exact role breakdown in [preservation check](round-007evidence/preservation-check.json); [independent reconstruction](round-007evidence/verified.json), [replayable static verification](round-007evidence/verify.py), [RPC](round-007evidence/rpc.json). All673prior regions/metadata and other internal fields preserved exactly, with original recovery artifacts untouched. Lead history retained and outcomes appended.

Nineteen stage/selector cases were independently reconstructed from previously classified transition fifth fields. Inspected consumer decoding and ROM candidate slices before annotations. 262f uses effective fixed0/switch1 descriptor words and five-byte records; c412 is reloaded nested index, separate from descriptor selector.27c2/2875 prove five rows and column<width; enumeration by screen/row/valid-column independently matches exact prior candidate cells. Upper-window layouts physicalbank1, lower fixedbank0. 2754 bank3 and2864 temporarybank1/restore3 establish exact sixteen-byte raw-ID tile/class reads. Stage3rawIDs>=64 withheld (64unique payload bytes in this scope, none added). 1e98 ordinary gates retain conditional fifth-field c02a interpretation; 1fab alternate semantics not conflated. Transition lists parsed by five-byte stride plus explicit high-E0 two-byte termination; no adjacency fill. No target execution, natural gate success, later-stage reachability, global selector closure or runtime execution claimed.

Static verification only; no new gameplay or input scenario. Existing natural routes and T001 workaround preserved. Actual execution evidence remains4890saved observed instruction starts/9144instruction bytes, distinct from classification. Store provenance Observed8251,Static28891,Manual0bytes. Totals Code11587,Data24689,Padding866;782/782regions carry role/evidence;Image/Text0. Required structural metadata complete for classified slices; final editorial review pending.

Sequential finalization: map.validate passed after all annotations, then map.save analysis/state.json, map.export maps/current.sym(include_sym=false); server finished before fresh `python3 run.py checkpoint round-007-disassembly`. Stock map validation, generation and byte-identical65536-byte verification passed. [Report](round-007-disassembly-verification.json), [stock map](../maps/round-007-disassembly.sym), [generated bank source](../generated/round-007-disassembly). Frozen map equals current.sym; mapSHA25632174aa027b953ad8e7265c55c100f2d1404b7b8a3daf4032954e5ede69e1160. Immutable ROMSHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c independently confirmed by inspect and preservation check. [Unknown complement](round-007-unknown-ranges.json), [metrics](round-007-classification.json).

Easy-gain stop: both ready bounded recipes verified in full. Additional fifth-field descriptors in these new transition lists remain useful independent extensions (including52,54,55,60,61), requiring their own stage/consumer/context verification; no recursive global-domain claim made. Stage3flag applicability, natural gate success, raw continuation code, bank1object/bank2audio grammar and late gameplay remain deferred; unchanged failed routes not repeated. [Phase handoff](round-007-disassembly-handoff.md).

Toolkit: existing1confirmed bugT001,0suspected/0missing-feature findings,0new/no blocker. No snapshot restoration/runtime or toolkit modifications; configured release unchanged. [Persistent report](toolkit-issues.md).

## Round 007 — intermediate commentary

Input/output classification checkpoint `round-007-disassembly` unchanged: 56.67% classified (37142/65536), 28394 unknown bytes. Entire previous commentary preserved in `analysis/round-007/commentary-prior` before fresh preparation. Carried 217 reviewed physical records with unchanged metadata and relocated source anchors; reviewed all 109 new Data records (1339 bytes). No credit invalidated. Current scaffold: 326 reviewed, 622 pending; final editorial audit pending.

Checked the affected descriptor loader, list gates, layout resolver, bank wrapper and tile/class consumers. Comments now explain shared nested layouts, the descriptor's actual object fields, exact five-row width limits and the broader withheld stage-3 raw-ID category. Ordinary gated fifth-field descriptor writes remain separate from alternate object-field semantics and masked transition indexing. New records retain descriptive structural names. Prior labels/comments carried into fresh assembly; one unlabeled transition-row anchor required explicit restoration after the first link caught the omission. No instruction or data changes.

Redirect: newly reviewed lists expose further fifth-field selectors, including 52, 54, 55, 60 and 61. Independently check each source stage and descriptor reload, then exact width-limited cells, raw-ID blocks and sentinel-led lists. Natural gate success, stage-3 flagged IDs, global selector closure, raw continuations, bank-1 object grammar and bank-2 audio remain deferred. [Compact review](round-007/commentary-review.json), [evidence](round-007evidence/verified.json), [handoff](round-007-commentary-handoff.md).

Final commentary verify passed byte_identical=true for all 65536 bytes; reading copy refreshed. Frozen phase1 state and stock map match the unchanged current files. No runtime or gameplay route added. Toolkit existing 1 confirmed bug T001, 0 suspected/0 missing features; 0 new/no blocker. [Persistent report](toolkit-issues.md). No ROM, map, classification or toolkit changes.

## Round 007 — redirect complete; next disassembly warranted

Input `round-007-disassembly` and compact commentary handoff; checkpoint unchanged. Classification **56.67% (37142/65536),28394unknown**. Exactly one redirect phase; no classification/map/source changes. Added ready R7-TRANSITION-DESCRIPTOR-LAYOUTS (336unknown candidates), R7-TRANSITION-DOWNSTREAM-SLICES (125), R7-CONTINUE-HANDLERS (153Code candidates);614unique potential bytes. Preserve prior leads/recovery. [Compact handoff](round-007-redirect-handoff.md), [evidence](redirect/round-007/evidence.json), [registry](leads.json).

26 independently reconstructed stage/selector cases from classified transition rows; all bounded constructed262f tests passed94/96/98operations (200cap), fixed0/switch1, matching c412/width*32. Source-list nested differs from descriptor reload:52→6,54→4,55→2,60→8,61→6. Exact five-row/valid-column layout cells and sentinel-led lists; stage3rawIDs>=64 withheld. Independent raw wrappers06f1..0712 selectbank3 then call7c20/7c3f; decoded handlerfc20..fc96 ends exactly at verified streamfc97. Four artificial calls returned988/19/279/278operations, single-step10000cap, exact patches/input releases/logs retained. No natural reachability or successful ordinary gates inferred. First bulk caller probe overshot RAMsentinel and hit IllegalInstruction; retained evidence and corrected per-operation return detection, local orchestration error only.

Verification: inspect original65536-byte ROMSHA256; configured release api.describe/map.load/map.validate passed before/after. Protected ROM/state/current.sym hashes unchanged. Prior round007 disassembly/commentary byte-identical reports retained, not rerun; no new classification checkpoint. End research with actionable independent ready leads; no global closure/substantial-byte prerequisite. Remaining unresolved flagged IDs/object/audio grammars and natural gameplay routes recorded, unchanged failed avenues not repeated. Toolkit existing1confirmedT001,0suspected/0missing features,0new/no blocker; [report](toolkit-issues.md). Final editorial audit pending.


## Round 008 — disassembly complete, ROM incomplete

Input `round-007-disassembly`; output `round-008-disassembly`. Exactly one disassembly phase; no commentary/redirect performed.

ROM classification: 57.62% (37760 / 65536 bytes classified; 27776 bytes unknown).

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8316 | 4737 | 73 | 13126 | 3258 |
| 01 | 1229 | 1136 | 67 | 2432 | 13952 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 227 | 15244 | 689 | 16160 | 224 |

Consumed/verified R7-TRANSITION-DESCRIPTOR-LAYOUTS (336new Data bytes), R7-TRANSITION-DOWNSTREAM-SLICES (125Data:32raw-ID block bytes plus93transition pointer/record/terminal bytes), R7-CONTINUE-HANDLERS (153candidate Code bytes plus4-byte directly referenced06ed tail). Added618unique bytes/68regions; preserved782prior regions/metadata and unrelated internal fields exactly. Lead history retained, no IDs produced. [Preservation](round-008evidence/preservation-check.json), [independent reconstruction and exact slices](round-008evidence/verified.json), [replayable verification](round-008evidence/verify.py), [RPC](round-008evidence/rpc.json).

Independently reconstructed26stage/selector cases from classified transition fifth fields. Source-list nested and262f-loaded nested remain distinct:52→6,54→4,55→2,60→8,61→6. Reinspected consumer instructions, effective fixed0/switch1 descriptor/layout context, width-guarded five rows, exact screen-major cells including incomplete-screen exclusions.2754 selects3 and2864 temporarily selects1/restores3; exact16-byte raw-ID blocks read as tile/class data. Stage3IDs>=64 withheld. Lists use exact five-byte stride and explicit high-E0 two-byte terminal. No natural gate success, later-stage reachability or global domain claim.

Independent continue-wrapper replay: PC06f1/c00d0 andPC0702/c00d0/4/8,c6010; after session-local clean restore all8buttons explicitly released(T001),2180=1,stackdff0=[00,c1],SPdff0; step1/state untilPCc100 with10000operation cap. Returned988/19/279/278operations and fixed0/switch1. Exact [script](round-008evidence/raw-probe.py), [RPC](round-008evidence/raw-probe-rpc.json), [results](round-008evidence/raw-probe-tests.json), scenarios.json. Only actual instruction fetches receive Observed provenance:49new starts/114bytes;43new Code bytes Static. Physicalfc20..fc96 exactly endsJP0ad2 before knownfc97 stream; literal stream pointers independently match classified Data. Untaken confirmation fallthrough andJP06ed supported statically;06ed XOR A/JP049c independently decoded. Pointer targets never counted as fetched. This pass83distinct constructed ROM fetched starts reported separately from historical4890starts/9144instruction bytes, without an uncomputed cumulative union. Detailed trace capacity0; complete bounded aggregate coverage responses retained.

Totals Code11744,Data25150,Padding866;Image/Text0.850/850regions have required role/evidence. Provenance Observed8365,Static29395,Manual0. [Metrics](round-008-classification.json), [exact unknown complement](round-008-unknown-ranges.json). Existing gameplay familiarity/natural routes preserved; no new natural route or external research. Diminished normal-play routes and unchanged failed avenues were not repeated; all useful ready bounded recipes resolved, direct06ed extension included. Further list fifth fields need independent stage/loader proof; flagged IDs,natural gates,bank1objects,bank2audio/global dispatch and late gameplay remain deferred. Final commentary audit pending.

Sequential finalization: map.validate passed, map.save analysis/state.json and map.export maps/current.sym(include_sym=false) completed; server exited before fresh `python3 run.py checkpoint round-008-disassembly`. Stock validation,generation and byte_identical65536-byte verification passed. [Report](round-008-disassembly-verification.json), [stock map](../maps/round-008-disassembly.sym), [bank listing](../generated/round-008-disassembly), [handoff](round-008-disassembly-handoff.md). Frozen stock equals current; mapSHA25610c920f0dd52c5bb59faca7cf4414eb645f0294843294b65bdb4ca0299ff39d5. run.py inspect and preservation independently confirm original ROMSHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Prior recovery artifacts preserved.

Toolkit: existing1confirmedT001,0suspected/0missing features;0new/no blocker. Configured release unchanged; no toolkit/ROM edits. Local verification assertions corrected caller-target physical mapping and exposed direct06ed tail before any disk save; orchestration errors only. [Persistent report](toolkit-issues.md).

## Round 008 — intermediate commentary complete

Input/output checkpoint `round-008-disassembly` unchanged. ROM classification **57.62% (37760/65536), 27776 unknown bytes**. Entire prior commentary preserved in `analysis/round-008/commentary-prior` before fresh preparation. Carried all 326 reviewed physical records after metadata/boundary/source-anchor checks, reviewed all 68 new records (618 bytes:461 Data,157 Code). No review credit invalidated. Scaffold:394 reviewed,607 pending; final editorial audit pending. No unchanged unknown sweep.

Explained descriptor fields, width-limited five-row cells, conditional transition rows and exact raw-ID blocks. Checked affected reload/gate/layout/bank/tile/class consumers; source-list nested indices remain distinct from reloaded nested indices (52→6,54→4,55→2,60→8,61→6). Stage3 IDs >=64 remain withheld. Continue routines now have functional names and concise prose: bank-preserving wrappers, prompt drawing, answer selection, confirmation reset and shared decline tail. Exact prompt stream addresses use known stream symbols. Replaced stale fc97 comment about unclassified callers. Checked state/substate setters and stream decoder; no new classification inferred. Preserved the old ScrollTransitionRow label explicitly at its checked unchanged anchor before linking.

Evidence remains scoped:114 newly fetched Code bytes under artificial calls;43 Code bytes static, including06ed and confirmation reset. No natural continue/gate/later-stage reachability claimed; no runtime or gameplay route added. Redirect may follow newly exposed stage-specific fifth-field selectors only after independent loader/context proof, or concrete continue state callers to test natural entry/confirmation. Global selector closure, stage3 flags, remaining raw continuations, bank1 objects, bank2 audio and late gameplay remain deferred. [Compact review](round-008/commentary-review.json), [evidence](round-008evidence/verified.json), [handoff](round-008-commentary-handoff.md).

Final commentary rebuild passed byte_identical=true for all65536 bytes; reading copy refreshed. Frozen map/state remain identical to current. No map/classification/ROM/toolkit edits or final audit. Toolkit existing1confirmed T001,0suspected/0missing features;0new/no blocker. [Report](toolkit-issues.md).

## Round008 — redirect complete; next disassembly warranted

Input round-008-disassembly and compact commentary handoff;checkpoint unchanged. ROM classification **57.62% (37760/65536),27776unknown**. Exactly one redirect;no map/source/classification/ROM/toolkit edits. Added ready R8-TRANSITION-EXACT-SLICES334unknown bytes and R8-AUDIO-EXACT-HEADERS74,408unique potential Data bytes. Historical leads/recovery preserved;R1-AUDIO-DATA-DOMAINS attempt history updated with materially improved bounded method. [Handoff](round-008-redirect-handoff.md), [registry](leads.json).

33source-stage/selector cases independently reconstructed from classified transition rows;all262f tests returned within200single-operation cap. New selectors53→nested5,62→7,63→9,64→10. Exact five-row/valid-column screen-major cells,16-byte raw-ID blocks and five-byte lists/high-E0 terminals only;stage3IDs>=64withheld. Parent list nested differs from reloaded descriptor nested. No successful ordinary gates,natural reachability or global closure inferred. [Evidence and replay](redirect/round-008/evidence.json).

Independent pending/generic audio source inspection established bounded selected command headers despite unresolved global command/stream domains. Literal5/6/7/21/49 and masked-stage table40/48;4712 reads priority then six-bit mask,4763 consumes a word per set bit. Seven constructed tests returned112/108/108/112/264/334/264operations under1000cap,expected command/priority/pointer RAM prefixes verified. Exact pointers/headers plus six-channel4672/4689tables yield74unknown bytes. No stream bytes inferred from target values. All8buttons released after every clean snapshot restore(T001);exact patches,banks,input/budgets in scripts/RPC/scenarios. No new natural gameplay. [Audio evidence](redirect/round-008/audio-evidence.json).

Verification: configured release api.describe,map.load,map.validate passed before/after both sessions;protected ROM/state/current.sym SHA256 unchanged. Prior byte-identical65536-byte disassembly/commentary reports retained,not rerun;no classification checkpoint. [Verification](redirect/round-008/verification.json). Remaining unknown counts and394reviewed/607pending unchanged;final editorial audit pending.

Toolkit existing1confirmedT001,0suspected/0missing features,0new/no blocker. No toolkit modifications. [Persistent report](toolkit-issues.md).


## Round 009 — disassembly finished, ROM incomplete

ROM classification: 58.24% (38168 / 65536 bytes classified; 27368 bytes unknown).

Input round-008-disassembly; output round-009-disassembly. Independently verified R8-TRANSITION-EXACT-SLICES (334Data) and R8-AUDIO-EXACT-HEADERS (74Data). Added408Static Data bytes in61regions. All850prior regions/metadata and unrelated internal fields preserved exactly;911/911regions have role/evidence. [Proof](round-009evidence/verified.json), [RPC](round-009evidence/rpc.json), [preservation](round-009evidence/preservation-check.json).

33source-stage/selector cases rederived from classified transition fifth fields, loader262f, exact directories and five-row width-limited screen-major indexing. Parent list nested differs from loaded nested;53→5,62→7,63→9,64→10. Upper layouts/lists bank1, lower fixed0;2754 selects3 and2864 restores3 after temporary1. Exact16-byte raw-ID payloads only, stage3IDs>=64withheld. Five-byte records stop before explicit high-E0 two-byte terminal; incomplete-screen gaps remain unknown. New records expose selectors56/57/65 for later independent source/descriptor/context investigation, not recursively accepted here.

Audio fixed00bd selects switch2. Literal command sources include bank2:46a3 command5 via4706; fixed3471command6,21b7command7,042ecommand21,04e3command49 via00bd. Masked stage source0a0e/0a1e yields40/48. 4033 indexes table4916 by command-1;4712 reads priority/mask,4763 consumes2bytes per selected low-six bit even if priority rejected: header extent2+2*popcount(mask&63). Selected commands5/6/7/21/40/48/49 only. 405b..40cc bounds c244=0..5,40a6 word consumer proves4672table12bytes;467e proves4689table6bytes. Stream pointer targets establish no stream extent or execution. Static proof only; no emulator/runtime, natural sound execution, gate success or new gameplay route claimed. Existing diminished routes were not repeated without new evidence.

Per-bank Code/Data/Padding/Unknown:00=8316/4827/73/3168;01=1229/1332/67/13756;02=1972/4107/37/10268;03=227/15292/689/176. TotalsCode11744/Data25558/Padding866;Observed8365/Static29803. No new fetched instruction starts. Historical natural and constructed execution figures remain separate; no uncomputed cumulative union. [Metrics](round-009-classification.json), [exact unknown complement](round-009-unknown-ranges.json).

Preservation audit caught a local script map.load path using round007 rather than frozen round008 input. Earlier provisional58.24%/checkpoint claim withdrawn before handoff. Superseded byte-faithful but classification-incomplete output retained as round-009-input-recovery with truthful report; original850-region input retained. Corrected script reloaded frozen input, repeated exact408byte additions and verified all850regions/metadata plus unrelated fields before fresh final checkpoint. This is local orchestration, not toolkit defect. Final map.validate/save/export finished sequentially before `python3 run.py checkpoint round-009-disassembly`; stock validation/generation and byte_identical65536bytes passed. [Final verification](round-009-disassembly-verification.json), [superseded recovery](round-009-input-recovery-verification.json). Current.sym equals frozen final map; hash1c4b75b43ac85fe6e79db0cfa002ac57b7c390445cb4e88fefa76f8bfd07516d. ROM unchanged; configured release unchanged.

Bounded ready recipes resolved; no commentary/redirect performed. Remaining selectors require independent proof;stage3flags,natural gates/continue,bank1objects,bank2stream grammar,global domains and later gameplay remain unresolved. Final editorial audit pending. Toolkit existing1confirmedT001,0suspected/0missing features,0new/no blocker; [report](toolkit-issues.md). [Compact handoff](round-009-disassembly-handoff.md).

## Round009 — bounded intermediate commentary complete

Corrected input `round-009-disassembly`, stock hash `1c4b75b43ac85fe6e79db0cfa002ac57b7c390445cb4e88fefa76f8bfd07516d`; superseded input-recovery was not used. Entire old commentary preserved in analysis/round-009/commentary-prior before fresh preparation. All394 prior reviewed physical records carried with exact boundaries/metadata and checked relocated source anchors. Reviewed61 new Static Data records/408bytes, including74audio bytes.455reviewed/605pending; no invalidated credit; final editorial audit pending. [Review](round-009/commentary-review.json), [preservation](round-009/commentary-preservation.json).

Explained descriptor fields, five-row width-limited layouts, five-byte transition rows/high-E0 terminals and exact sixteen-byte raw-ID payloads. Checked affected reload/gate/layout/tile/class consumers; parent-list nested differs from reloaded nested (53→5,62→7,63→9,64→10). Upper layout window uses bank1; payload consumer restores3; stage3rawIDs>=64 remain withheld. New selectors56/57/65 require independent source-stage/descriptor/context proof.

Named audio indexed-word reader, register-offset lookup, command loader/register-preserving wrapper and selected-stream installer. Fixed00bd selects bank2 and now calls the known wrapper symbol. Inspected4033,405b..40cc,467e,4712,4763 and selected command sources: bank2:46a3literal5;fixed3471literal6,21b7literal7,042eliteral21,04e3literal49;masked-stage0a0e/0a1e selects40/48. Six logical channels have $25-byte state records; logical channels can share hardware register groups. Header is priority+mask+one word per selected low-six bit. Priority rejection still consumes the pointer; target values alone establish no stream extent. Selected directory entries do not close the command domain. No runtime/natural sound execution inferred.

Prior labels/comments preserved, including explicit ScrollTransitionRow anchor exception; repetitive call/jump/rst differences resolved consistently. Two coherent commentary rebuilds passed byte_identical=true for65536bytes; reading copy refreshed. Frozen map/state equal current. No classification/map/ROM/toolkit edits, gameplay experiment or final audit. ROM remains58.24%classified/27368unknown. [Handoff](round-009-commentary-handoff.md), [evidence](round-009evidence/verified.json).

Toolkit existing1confirmedT001,0suspected/0missing features;0new/no blocker. [Report](toolkit-issues.md).

## Round009 — redirect complete; ready bounded work, ROM incomplete

Input corrected round-009-disassembly and commentary handoff; no superseded recovery used. ROM remains58.24%(38168/65536classified;27368unknown),455reviewed/605pending. No map/state/source/ROM/toolkit edits. Three ready IDs:R9-TRANSITION-EXACT-SLICES exact95unknownData;R9-AUDIO-BOUNDED-STREAMS exact80unknownData;R9-AUDIO-INLINE-DISPATCH-CORRECTION18StaticCode→Data+12unknownData,plus8adjacentStaticCode requiringindependentproof/retraction(conservative netgain4). Actual internalCode overlaps inspected,not exportfallback. ExactIDs/splits,requiredreviewinvalidation andObservedneighbors inregistry. Static source/consumerproof and3constructed262f calls98operations(cap200);31directaudioeventcalls for5/7/21 terminate,2000cap/event,100eventcap. Command6finiteFB/FE grammar provenstatically;no repetitive runtime/naturalreachabilityclaim. Tracecapacity0 deliberatelydropsdetail,aggregate_dropped0,completecoveragepaged. Bothmap.validatepassed;protectedhashesunchanged;existingbyte-identical65536checkpointunchanged. No unchangedstalledreplay/globalclosureclaim. [Handoff](round-009-redirect-handoff.md),[evidence](redirect/round-009/evidence.json),[verification](redirect/round-009/verification.json). Toolkit existingT001only,0new/0suspected/0missingfeatures,no blocker;[report](toolkit-issues.md).

## Round 010 — disassembly finished, ROM incomplete

ROM classification: 58.51% (38347 / 65536 bytes classified; 27189 bytes unknown).

Input corrected round-009-disassembly (911 regions); output round-010-disassembly. Exactly one disassembly pass consumed R9-TRANSITION-EXACT-SLICES, R9-AUDIO-BOUNDED-STREAMS and R9-AUDIO-INLINE-DISPATCH-CORRECTION. Independent consumer and bank checks accept 95 transition bytes and 80 selected audio stream bytes. Stage 1 selectors 56/57 load nested 11/12; stage 3 selector 65 loads nested 8; all widths are 5. Five-row screen-major enumeration, explicit high-E0 list terminals and restored bank 3 raw-ID consumers bound each slice. Stage 3 IDs >=64 remain withheld. Commands 5/6/7/21 select channel 0/1/1/0 and exact stream lengths 11/30/17/22. Independent finite grammar/control interpretation reaches FF; command 6 terminates after 2875 control steps with repeat counts 253 and 8. No runtime, new fetched roots, natural reachability or pointer-target execution claim.

Corrected internal Static Code overlaps: physical 81af..81b8 (five words), 8269..827a (nine words), and 82ad..82ae (selected EA00 entry zero). Exactly 18 Code bytes become Data and 12 unknown bytes become Data. Physical 82af..82b6 eight unsupported Static Code bytes return to Unknown; pointer shape alone is insufficient. Region 89 split to six-byte dispatcher prefix, region 90 removed before replacement, region 91 split to real nine-byte 42b7 handler. Exact handler decoding and prior artificial EA00 fetched evidence are independent of table fallthrough; Static provenance retained. All Observed neighbors preserved. Original affected records/metadata frozen, fresh roles/evidence explain justified changes. [Proof](round-010evidence/verified.json), [inspection](round-010evidence/inspection-rpc.json), [mutation RPC](round-010evidence/rpc.json).

Exact change audit: 187 new classified bytes, 18 Code→Data, eight retracted, net +179. All 908 unaffected prior regions and metadata, plus unrelated internal fields, match frozen input exactly. All 928 current regions have role/evidence. Totals Code 11718 / Data 25763 / Padding 866. Per-bank Code/Data/Padding/Unknown: 00=8316/4883/73/3112; 01=1229/1371/67/13717; 02=1946/4217/37/10184; 03=227/15292/689/176. [Preservation](round-010evidence/preservation-check.json), [metrics](round-010-classification.json), [unknown complement](round-010-unknown-ranges.json). Execution coverage remains historical and separate; no cumulative union was computed.

Map validate/save/export completed sequentially before fresh checkpoint. Stock validation and generation passed; fresh verification confirms byte_identical=true for all 65536 bytes. [Verification](round-010-disassembly-verification.json). Current.sym equals checkpoint map, hash b8a7d76a399a52bce66980895006c5c932f40f379f73f263bf79f312756677c8. Input ROM retains original SHA-256 edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c; configured binary unchanged. No alternative build or toolkit edit.

Affected old Code/fallthrough commentary credit is explicitly invalidated in [record](round-010evidence/commentary-invalidation.json). Prior reviewed-range lists do not intersect affected scopes; 455 unaffected reviewed credits persist, while historical 605 pending must be recomputed against the new checkpoint. No commentary phase or final editorial audit performed. New selectors 58/59, nested audio domains and retracted pointer hypotheses are separate unresolved leads; stage 3 flags, bank 1 objects, music streams, global domains and natural gates/later progression remain open. Existing diminished gameplay routes were not replayed without a materially improved route. Toolkit existing one confirmed T001, zero suspected/missing features, zero new findings, no blocker; [report](toolkit-issues.md). [Compact handoff](round-010-disassembly-handoff.md).

Final internal provenance totals: Observed 8365 bytes and Static 29982 bytes. These are classification provenance counts, not a computed whole-history execution-coverage union.

## Round 010 — bounded intermediate commentary complete

Input round-010-disassembly, stock hash b8a7d76a399a52bce66980895006c5c932f40f379f73f263bf79f312756677c8. Entire prior commentary preserved under analysis/round-010/commentary-prior before fresh preparation. Carried all 455 unaffected physical reviewed records with equal metadata and 910 checked source anchors. Reviewed 21 new/split records: 476 reviewed, 602 pending; final editorial audit pending. [Review](round-010/commentary-review.json), [preservation](round-010/commentary-preservation.json).

Explicitly invalidated prior Code/fallthrough interpretations in physical 81a9..81b8, 8269..827a and 82ad..82bf; no prior reviewed record intersects them. Bank-2 CPU403b pops the inline return and jumps through an indexed word. The FB–FF and E7–EF words are data; EA00 proves only the first nested word. Eight retracted bytes 82af..82b6 remain unknown raw bytes, without instruction labels. Independently supported dispatcher prefix and 42b7 handler remain code. Reviewed marker/repeat/call/return/stop consumers and parameter/nested dispatch; prior command-loader/header review is independent and remains valid.

Reviewed selected transition additions with stage context: 56/57/65 reload nested 11/12/8, width five; descriptor fields, screen-major five-row layout, list rows and high-E0 terminals explained. New 58/59 targets still need independent proof. Stage3 IDs>=64 remain withheld. Selected audio commands5/6/7/21 have bounded streams11/30/17/22 bytes; FB records repeat start, FE compares the incrementing counter and resumes at the marker until count reached. Command6 repeats its two ten-event groups253 and8 times; static finite interpreter2875 control steps is not emulator execution or elapsed sound timing. Other stream/selector domains remain open.

Two coherent source verifications passed byte_identical=true for65536bytes; reading-after refreshed. [Verification](round-010/commentary-verification.json). Frozen state/map equal current; no map/classification/ROM/toolkit change, runtime, other phase or final audit. ROM remains58.51%classified,27189unknown. [Handoff](round-010-commentary-handoff.md).

Toolkit existing one confirmedT001, zero suspected/missing features, zero new findings/no blocker. [Report](toolkit-issues.md).

## Round010 redirect complete

Input round-010-disassembly / compact commentary handoff. Two ready leads produced: R10-TRANSITION-58-59 (51Unknown candidatebytes) and R10-AUDIO49-STREAMS-EA12 (111Unknown candidatebytes),total162distinct. Stage1 source-backed58/59→nested12/13,width5;two artificial262f tests98operations each. Command49 three finite streams35/29/37bytes terminate after13/13/17artificial eventcalls. Their EA16/2a/21/20 independently establish nested selectors1/2:fourUnknown inline bytes82af..82b2,plus6selected table bytes8a9f/8aa9/8ab1.82b3..82b6 staysUnknown.42cbhandler alreadyCode,16instructions independently decoded/fetched. No natural gate/audio reachability/globaldomain claim. New66 remains separately unverified.

No map/classification/source/ROM/toolkit edits;58.51%classified38347/65536,27189Unknown,476reviewed/602pending unchanged. map.validateok;input/state/current.sym/configuredrelease hashes equal before/after/final. Exact tests/negative scope/[findings](redirect/round-010/findings.md),[evidence](redirect/round-010/evidence.json),[handoff](round-010-redirect-handoff.md). No generated checkpoint required;existing verified round010 output retained. Ready for next disassembly only. Toolkit existing1confirmedT001,0suspected/missingfeatures,0new/no blocker;explicit8buttonreleases applied afterclean session-local restores;[report](toolkit-issues.md).

## Round 011 — disassembly finished, ROM incomplete

ROM classification: 58.76% (38509 / 65536 bytes classified; 27027 bytes unknown).

Input round-010-disassembly; output round-011-disassembly. Exactly one pass consumed R10-TRANSITION-58-59 and R10-AUDIO49-STREAMS-EA12. Added 162 Static Data bytes in 13 regions: 51 transition bytes, 101 finite audio stream bytes, four reclaimed inline dispatch bytes and six selected parameter-word bytes. All 928 prior region objects, metadata and unrelated internal fields match frozen input exactly; all 941 current regions have role/evidence. No code correction or new Code claim. Recovery/history artifacts preserved. [Preservation](round-011evidence/preservation-check.json), [independent proof](round-011evidence/verified.json), [inspection](round-011evidence/inspection-rpc.json).

Stage1 classified parent rows43dd/43e4 source58/59 through1e98;262f reloads nested12/13,width5. Exact five-row screen-major cells, selected descriptor/layout/width/list pointers, five-byte rows and high-E0 terminals independently checked. Bank3 raw-ID payloads already classified; no redundant regions added. New selector66 is an unresolved hypothesis. No natural gate/later-stage reachability claim.

Command49 header independently selects channels2/3/4, finite stream lengths35/29/37 with FF endpoints. Aligned EA16/2a/21/20 parameters establish nested1/2 inline words82af..82b2. 82b3..82b6 remain Unknown. Independent decoding caught a redirect inference error: EA16 selects42c0, which ends JP41a2, so it does not read8aa9. EA20 selects42cb with low nibble0; DEC/ADD wraps toFE before base4a9f addition, reading CPU4b9d,physical8b9d. Accepted exact words8a9f/8ab1/8b9d;8aa9..8aaa remain Unknown. Prior evidence/lead history retained with correction appended. No target data extent inferred, including non-pointer valueFB51 at8b9d. 42cb is already Code and unchanged.

Four constructed42a5 calls with parameters16/21/2a/20, fixed0/switch2, DEc300,SPdff0, stopPC41a2, cap100 operations, returned22/31/31/32operations. Exact ROM Read events confirm inline words and selected parameter bytes; detail/aggregate drops0. Clean session-local restore and explicit eight-button releases used(T001). These tests corroborate local consumer behavior only, not natural sound execution or gameplay progression. [Read verification](round-011evidence/word-read-verification.json), [replay](round-011evidence/word-probe.py), [RPC](round-011evidence/word-probe-rpc.json). Whole-history execution coverage was not recomputed; classification provenance is Observed8365/Static30144 bytes, distinct from runtime coverage.

Totals Code11718/Data25925/Padding866. Per-bank Code/Data/Padding/Unknown: 00=8316/4911/73/3084;01=1229/1394/67/13694;02=1946/4328/37/10073;03=227/15292/689/176. [Metrics](round-011-classification.json), [unknown complement](round-011-unknown-ranges.json). No new natural route or external gameplay research; unchanged diminished routes not repeated. Ready finite slices exhausted; further selector66,music40/48 calls/loops,stage3 flags,bank1 objects and natural gates remain for fresh investigation. Commentary/redirect not executed and prior review credit remains historical pending reconciliation.

Map validate/save/export completed sequentially before fresh checkpoint. Stock validation/generation and fresh byte_identical=true for65536bytes passed. [Verification](round-011-disassembly-verification.json), [stock map](../maps/round-011-disassembly.sym), [bank listing](../generated/round-011-disassembly). Current.sym equals frozen map SHA256bb22398c95f1691d6a8d7806cef67d0dd73390e4580807d2810f37bd6d3c1fcb. Original ROM SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c and configured release unchanged. No toolkit changes or alternative build.

Toolkit existing one confirmed T001, zero suspected/missing features, zero new findings/no blocker. Incorrect initial local trace-space filter yielded empty detail, corrected to unfiltered Read capture before claims; not toolkit defect. [Report](toolkit-issues.md). [Compact handoff](round-011-disassembly-handoff.md).

## Round 011 — bounded intermediate commentary complete

Reviewed round-011-disassembly, stock hash bb22398c95f1691d6a8d7806cef67d0dd73390e4580807d2810f37bd6d3c1fcb. Full prior commentary preserved under analysis/round-011/commentary-prior before fresh preparation. Carried 475 exact physical/metadata reviewed records with 945 checked source anchors. Replaced one prior eight-byte unknown review with four supported Data bytes and four Unknown bytes. Also revised the earlier EA00-only dispatch explanation and reviewed the existing negative/word handlers. Final inventory: 490 reviewed, 599 pending; final audit pending. [Review](round-011/commentary-review.json), [preservation](round-011/commentary-preservation.json).

EA16 selects the negative low-nibble setting handler, which jumps to 41A2; there is no 42CB fallthrough and no 8AA9 word read. EA21/EA2A/EA20 read physical 8A9F/8AB1/8B9D. EA20 uses the eight-bit wrapped FE index before adding the 4A9F base. Four direct-read tests corroborate these local mechanics; they do not establish natural reachability. Only 82AF..82B2 are newly supported dispatch words; 82B3..82B6 and 8AA9 remain Unknown. Existing 42CB code was named, not reclassified.

Command 49 channel 3/2/4 stream slices are respectively 29/35/37 bytes, each through FF, with no call/repeat branch altering the finite bounds. Static consumer review supports this grammar, not elapsed sound timing or natural execution. Stage-1 selectors 58/59 reload nested 12/13 and width five. New nested-13 layout and list data are explained; its final selector 66 remains an independent source-stage/descriptor-context question. Parameter-word targets and intervening words remain unresolved. Stage-3 flagged IDs, bank-1 object records, music 40/48 call/loop consumers and natural gates remain deferred.

Two coherent verification runs passed byte_identical=true for all 65536 bytes; reading-after refreshed at handoff. Frozen map/state equal current; no phase-1/toolkit edits or final audit. [Final verification](round-011/commentary-verification.json), [evidence](round-011evidence/verified.json), [direct reads](round-011evidence/word-read-verification.json).

Toolkit: existing one confirmed T001, zero suspected/missing features, zero new findings and no blocker. [Report](toolkit-issues.md).

### Round011 redirect — ready structural graphs
Input round-011-disassembly / round-011 commentary compact handoff. Exactly one redirect completed; no classification/map/source/ROM/toolkit changes.58.76%classified38509/65536,27027unknown unchanged; latest verified checkpoint and commentary remain authoritative.

Ready R11-TRANSITION-66 independently sources stage1 nested13 row43eb selector66, nested14 width10; seven unknown descriptor/directory bytes. Constructed262f returns98operations/c41214/width320, cap200. Downstream list selectors30/31 already known, so chain stops with no repeated work.

Ready R11-MUSIC40-48-STATEFUL-GRAPHS follows actual single-slot FD/FC, FB marker, finite FE and absolute FEFF state, yielding seven exact cyclic sparse event graphs and571 union unknown stream bytes; EA28 independently adds selected word8aad two bytes. Seven representative constructed control tests match exact state transitions, cap100 each. Closed structural state graphs do not prove natural execution, timing or finite playback. Do not classify pointer min/max gaps, targets or EA entries3/4. Total future candidate580bytes; independent disassembly review required. Remaining global command/selector domains, bank1objects, stage3 flagged IDs and natural gates deferred without unchanged stall retries.

[Findings](redirect/round-011/findings.md),[evidence/replay/RPC](redirect/round-011/evidence.json),[leads](leads.json),[handoff](round-011-redirect-handoff.md). map.validate passed; protected ROM/state/current.sym/configuredrelease hashes unchanged. No new checkpoint needed. Toolkit existing1confirmedT001,0suspected/0missingfeatures,0new/no blocker;[report](toolkit-issues.md). All8buttons released after each clean session-local snapshot restore.

## Round 012 — one disassembly pass finished; ROM incomplete

ROM classification: 59.65% (39089 / 65536 bytes classified; 26447 bytes unknown).

Input round-011-disassembly and round011 redirect handoff; output round-012-disassembly. Consumed R11-TRANSITION-66 and R11-MUSIC40-48-STATEFUL-GRAPHS. Independently recomputed exact sparse event/edge sets and repeated decoder states for seven source-selected music graphs. Added 580 Static Data bytes in five regions: seven descriptor bytes, 571 stream bytes and exact EA28 word8aad..8aae. All 941 prior region objects, metadata and unrelated internal fields preserved exactly; all 946 current regions carry role/evidence. No existing classification changed. History/recovery and previous commentary remain preserved. [Proof](round-012evidence/verified.json), [raw inspection and decode](round-012evidence/inspection-rpc.json), [preservation](round-012evidence/preservation-check.json).

Stage1 nested13 row43eb fifth field66 selects directory452b and five-byte descriptor4675: nested14,width10. Downstream layout/list/payload bytes already classified; only the exact seven new bytes accepted. The source table at fixed0:0a1e is decimal40,40,40,48; c02b&3 selects command, wrapper00bd selects switch2,4712 indexes4916 by command-1 and4763 initializes selected channels. Header mask and packed pointer sequences independently checked. Audio physical addresses are CPU+4000 with fixed0/switch2.

Music40 channels2/3/4 have67/169/111 unique event starts and246/190/240 states; music48 channels2/3/4/5 have23/25/33/14 events and23/25/119/15 states. Exact sparse union, events, edges and cycle-entry states match independent enumeration; overlapping channel graphs counted once. FD overwrites the single return slot; FC resumes it without popping. FB changes marker without resetting counter. FE FF consumes absolute target; other FE increments eight-bit c106, repeats marker unless equal operand, then clears counter and skips operand. Actual source bytes and boundaries inspected before annotation. Initial return/marker zero are never read before assignment on these paths;4763 clears counter. Exact repeated state proves structural cyclic closure, with no finite-duration, natural playback or global music domain claim. No min/max span padding inferred.

Seven independently replayed isolated FB/FE/FD/FC controls returned23/32/30/31/31/23/32 operations under100-operation caps, including FF-to00 counter wrap. Setup restores clean session-local snapshot, explicitly releases all eight buttons(T001), selects bank2 by2180 write, patchesc106 andc11b..c11e from before-state, setsPC41a3/DEevent/SPdff0, and steps one operation to next41a3. [Exact before/after values](round-012evidence/control-tests.json), [RPC](round-012evidence/rpc.json), [replay](round-012evidence/verify.py). These are artificial local consumer tests; full graph remains static. Whole-history execution coverage not recomputed; no new Code or Observed classification.

Aligned EA28 selects exact physical8aad word4ac5 through42cb eight-bit doubled decrement. Destination4ac5 has no inferred extent/classification. Prior EA20 correction8b9d and42c0 endingJP41a2 preserved;8aa9..8aaa and82b3..82b6 remain Unknown. No new EA high-nibble domain inferred.

Totals Code11718/Data26505/Padding866. Per-bank Code/Data/Padding/Unknown:00=8316/4911/73/3084;01=1229/1401/67/13687;02=1946/4901/37/9500;03=227/15292/689/176. [Metrics](round-012-classification.json), [exact unknown complement](round-012-unknown-ranges.json). Required annotations946/946. Classification is distinct from execution coverage and pending commentary review. Stage3 flagged IDs, bank1 object data, other music domains, natural gate success and later-stage progression remain open. No materially improved natural route was available; diminished prior routes not repeated. Ready leads verified; no unchanged stalled hypothesis reopened or other phase executed.

Sequential map.validate, map.save toanalysis/state.json and stock map.export tomaps/current.sym completed before fresh checkpoint. Stock validation/generation passed and fresh byte_identical=true covers65536bytes. [Verification](round-012-disassembly-verification.json), [portable stock map](../maps/round-012-disassembly.sym), [bank-organized listing](../generated/round-012-disassembly). Current map equals checkpoint hash8bea2d22ce7218162a61bcee8f4ca1a7b130e0c95f6238a9e81c9597e886142c. Original ROM identity and configured release unchanged; no toolkit changes or alternate binary.

Toolkit report retains one confirmed T001, zero suspected bugs, zero missing features, zero new findings and no blocker. Explicit input releases used after every restore. [Report](toolkit-issues.md). [Compact handoff](round-012-disassembly-handoff.md). Next coordinator phase is commentary; no final editorial audit performed here.

## Round 012 — bounded intermediate commentary complete

Input round-012-disassembly, map 8bea2d22ce7218162a61bcee8f4ca1a7b130e0c95f6238a9e81c9597e886142c. Full prior commentary preserved in analysis/round-012/commentary-prior before fresh preparation. All 490 reviewed physical ranges and metadata carried, with 980 checked source anchors and every prior label retained. Five new regions reviewed: 495 reviewed, 600 pending; editorial audit pending. [Review](round-012/commentary-review.json), [preservation](round-012/commentary-preservation.json).

Selector66 now resolves from stage1 nested13 row43eb to directory452b and descriptor4675, reloading nested14 width ten. Its downstream layout/list consumers were checked without reopening unrelated prior reviews. Music40/48 exact event unions and state closure were read with the source command table, bank wrapper, header mask, packed pointers and channel initializer. FD overwrites one return slot; FC reads it without popping. FB preserves the counter; counted FE increments eight bits including wrap, repeats the marker until equality and then clears the counter. FE FF reads an absolute target. Comments explain these mechanics and the shared cyclic event data. Seven local artificial controls corroborate mechanics only; the graphs establish neither finite playback nor natural reachability. [Static graph evidence](round-012evidence/verified.json), [controls](round-012evidence/control-tests.json).

EA28 selects physical8aad word4ac5; no target extent is inferred. EA20 remains8b9d. Physical8aa9 and82b3..82b6 remain Unknown. Affected consumers and prior corrections were checked; no classification or map changes. Other music domains, parameter targets, bank1 object records, stage3 flagged IDs, natural gate success and later progression remain deferred for redirect. No new natural experiment route was established.

Two coherent source rebuilds passed byte_identical=true for all65536 bytes; reading-after refreshed at handoff. Frozen map/state equal current. Local carry exceptions for repeated operand renames and two source-only labels were resolved and all names rechecked; no toolkit defect. [Final verification](round-012/commentary-verification.json). Partial review only; no final audit or other phase executed.

Toolkit: existing one confirmed T001, zero suspected bugs/missing features, zero new findings and no blocker; no emulator/snapshot or toolkit edits. [Report](toolkit-issues.md).


## Round012 redirect — ready exact audio slices
Input round-012-disassembly and round-012-commentary-handoff. Produced ready R12-STAGE-MUSIC37-41-42-43 (3613unknown candidate bytes) and R12-LITERAL-AUDIO20-28 (47), exact union3660. Prior lead/pass history preserved against redirect/round-012/input-leads.json. No classification/map/source/toolkit changes:39,089/65,536classified(59.65%),26,447unknown;495reviewed/600pending. Perbank(code/data/padding/unknown):0=8316/4911/73/3084;1=1229/1401/67/13687;2=1946/4901/37/9500;3=227/15292/689/176.

Source05aa table05bc has conditionalstage0..3 outputs37/42/41/43, NO localmask;0a0e locally masks&3 yielding40/48. Both full finite source sets inspected. Existing Code literalbank1 sources5971/7af7 yield28/20 through bank2 wrapper00bd. Exact stateful sparse music graphs and finite20/28streams establish candidate roles; independent31graph event/edge/state-count verification passed. Fresh actualEA3x/4x events reopen four formerly unsupported dispatch bytes82b3..82b6 with explicit new evidence; prior retraction history retained. EA20=8b9d,EA16doesnotfallthrough,8aa9unselected. Selected words only, no target/minmax fill.

Eleven bounded constructed tests passed:20/28loaders112each,37/41/42/43loaders334each;37oldpriority0/1accepted334each,2rejected118;EA34/42probes71/25. Caps1000/100,clean session snapshots/all8buttonsreleased(T001),fixed0/switch2. Artificial execution does not prove natural reachability/playback or finite music duration. Initial slow full-journal-per-operation harness deliberately interrupted/preserved and optimized to use step-returned state; same tests replayed successfully. Local orchestration correction,not toolkit issue. map.validateok;ROM/state/current.sym/configuredrelease hashes unchanged. Existing round012disassembly and commentary65536byte-identical verification retained; no new checkpoint warranted unchanged classification. [Findings](redirect/round-012/findings.md),[verification](redirect/round-012/verification.json),[handoff](round-012-redirect-handoff.md). Toolkit existing1confirmedT001,0suspected/0missingfeatures,0new/no blocker;[report](toolkit-issues.md).

## Round 013 — one disassembly pass finished; ROM incomplete

ROM classification: 65.23% (42749 / 65536 bytes classified; 22787 bytes unknown).

Input round-012-disassembly and round012 redirect handoff; output round-013-disassembly. Verified R12-STAGE-MUSIC37-41-42-43 (3613 bytes) and R12-LITERAL-AUDIO20-28 (47 bytes), adding 3660 Static Data bytes in 18 regions. All 946 prior region objects, metadata and unrelated internal fields are preserved exactly; all 964 current regions carry role/evidence. Recovery and retraction history remain on disk. [Independent proof](round-013evidence/verified.json), [consumer/byte inspection](round-013evidence/inspection-rpc.json), [preservation](round-013evidence/preservation-check.json).

Actual source05aa reads table05bc values37/42/41/43 with no local mask. This slice is explicitly conditional on stage0..3, consistent with stage initialization and masked progression; it does not bound every possible c02b writer. Source0a0e masks c02b&3 locally and selects40/48 at0a1e. Existing Code bank1:5971 submits decimal28 and7af7 submits20. Fixed00bd saves caller bank via7fff, selects2 via2180, calls4706, then restores caller bank. Bank2 physical offsets equal CPU+4000. 4712 uses4916+2*(command-1); priority and low-six-bit mask precede packed selected-channel pointers. 4763 consumes a pointer before unsigned priority comparison: equal/greater new priority accepted, lower rejected while maintaining alignment. Exact selected headers/directories only; no global command directory extent inferred.

Independent reconstruction starts all18selected channel graphs from their actual packed pointers, with single return/marker slots and zero counter. FD overwrites return; FC resumes it; FB sets marker without clearing count. FE FF consumes an absolute target, other FE increments eight-bit count then either repeats marker or clears count and advances. E0 widths are4/3/4/4/4/2 by channel, E7..EF consume one parameter. Return/marker are assigned before use. Exact event bytes, event boundaries, edges, state counts and stop/cycle closure match redirect evidence; shared events are deduplicated. Additions comprise8source bytes,12directory bytes,48header bytes,4inline dispatch bytes,2selected-word bytes and3586event/operand bytes. Sparse consumed bytes alone are classified, with no min/max fill. Command20/28 FF endpoints are structural; stage graphs have exact state cycles. No natural execution, playback duration or global selector claim follows.

Fresh aligned EA30/34/38/40/42 events establish the previously missing high-nibble3/4 domain, justifying82b3..82b6 as inline words selecting existing Code42e7/42f3. Prior round010 retraction was correct for its evidence and is preserved; these new source events reopen that narrow slice. Handler42e7 sets envelope through c111/45e6;42f3 toggles c105 bit1. Neither changes parser-control slots. Exact EA27 selected word8aab..8aac is new; other selected words remain preserved. EA20 still wraps to8b9d, EA16 endsJP41a2 and does not fall through,8aa9..8aaa remain Unknown. Selected values prove no target extent.

This pass is static: no emulator, snapshot, input route or new external gameplay research. Prior11bounded artificial loader/priority/EA tests remain linked in [redirect evidence](redirect/round-012/evidence.json) and [replay](redirect/round-012/research.py), with exact patches, caps and all-eight-button releases(T001); they were not rerun. Existing sourced controls/routes and diminished natural progression history are preserved; no materially improved normal route emerged and unchanged stalled hypotheses were not repeated. Classification provenance remains separate from execution coverage: Observed8365/Static34384 bytes; whole-history fetch coverage was not recomputed and no new Code was added.

Totals Code11718/Data30165/Padding866. Per-bank Code/Data/Padding/Unknown:00=8316/4919/73/3076;01=1229/1401/67/13687;02=1946/8553/37/5848;03=227/15292/689/176. [Metrics](round-013-classification.json), [exact unknown complement](round-013-unknown-ranges.json). Unbounded object-driven sound sources, apparent literal patterns in Unknown source, stage3 flags, bank1 objects, natural gates and later gameplay remain unresolved. Remaining ready lead recipes are consumed; no commentary/redirect performed in this worker. Prior review credit needs reconciliation for new bytes; final editorial acceptance is pending.

Sequential map.validate, map.save to analysis/state.json and map.export to maps/current.sym completed before the fresh checkpoint. Stock validation/generation passed; fresh byte_identical=true covers65536bytes. [Verification](round-013-disassembly-verification.json), [portable stock map](../maps/round-013-disassembly.sym), [bank-organized assembly](../generated/round-013-disassembly). Current.sym equals the checkpoint map SHA2563bff0d75c9f7ef54fbb739ed6c741708d8b9bded0e172a63451bd30fc0149751. ROM and configured release hashes match the redirect input. No toolkit edits or alternate build.

Toolkit report retains one confirmed T001, zero suspected bugs, zero missing features, zero new findings and no blocker. No runtime/snapshot use in this pass. [Report](toolkit-issues.md). [Compact handoff](round-013-disassembly-handoff.md). Next coordinator phase is commentary; ROM remains incomplete.

## Round 013 — bounded intermediate commentary complete

Input round-013-disassembly, map SHA256 3bff0d75c9f7ef54fbb739ed6c741708d8b9bded0e172a63451bd30fc0149751. Preserved the full previous commentary in analysis/round-013/commentary-prior before fresh preparation. Carried 494 unchanged reviewed ranges with exact phase1 metadata, prior names and comments and 988 checked source anchors. The remaining prior review, Unknown82b3..82b6, is superseded by fresh Data evidence; its earlier retraction and valid-at-the-time review remain preserved. Eighteen new ranges reviewed: 512 reviewed, 606 pending. Editorial audit remains pending. [Review](round-013/commentary-review.json), [preservation](round-013/commentary-preservation.json).

Reviewed conditional05aa stage0..3 selection separately from0a0e local &3; named stage tables, literal20/28 entries, packed headers and shared event blocks. Checked actual source pointers, six-channel masks and unsigned priority: selected pointer consumption precedes comparison, equal/higher priority accepted. The eighteen exact graphs have sparse event/operand boundaries, single return and marker slots, and channel-dependent E0 widths. Literal FF endpoints and stage state cycles describe structure only. No finite playback, natural reachability or global command-domain claim follows. [Proof](round-013evidence/verified.json).

Fresh EA30/34/38/40/42 events support82b3..82b6 inline handler words. Named envelope and channel-flag handlers; neither alters parser control slots. EA27 selects8aab; EA20 still wraps to8b9d, EA16 ends without fallthrough, and8aa9..8aaa remain Unknown. No selected word establishes target extent. Unbounded object-driven sound commands, unknown source literals, bank1 object records, stage3 flags, natural gates and later gameplay remain deferred. Existing gameplay research and routes are preserved; this pass used no runtime or new external research.

Three coherent/final edited-source rebuilds passed byte_identical=true for all65536 bytes; reading-after refreshed after the final build. Frozen map/state equal current; no classification, map or toolkit edits. Carry exceptions for repeated operands and source-only labels were resolved, including restored EA20 comments. Local prepare initially refused the existing directory; the preserved directory was moved aside and fresh preparation succeeded. These were local workflow matters. Partial review only; no final audit or other phase performed. [Verification](round-013/commentary-verification.json).

Toolkit retains one confirmed T001, zero suspected bugs/missing features, zero new findings and no blocker. No emulator/snapshot or toolkit edits. [Report](toolkit-issues.md).

## Round013 — redirect complete, actionable bounded objects

Input round-013-disassembly and compact commentary handoff; classification remains42,749/65,536(65.23%),Unknown22,787; commentary512reviewed/606pending and final audit pending. Added ready lead `R13-OBJECT-7BB9-SELECTED-LISTS`:40 existing descriptor-backed conditional stage/nested contexts select138 distinct five-byte placement records, exact selected directory words and E000 two-byte coordinate boundaries.858currently Unknown bank1 bytes have sparse structural evidence. No classification/map/source changes. [Findings](redirect/round-013/findings.md),[evidence](redirect/round-013/evidence.json),[leads](leads.json).

Code5679/570b/575a use bank1 directory7bb9 with c02b then c412.56b4 scans stride5, bounded coordinate-difference comparison; E000 is a negative coordinate boundary under tested nonnegative positions, not an unconditional opcode.56e1 reads type/slot;312e copies coordinate and parameter into object fields. stage1/nested12 has nonmonotonic0070,0038: preserved without sorting/global order or natural record-selection inference. Exact source artifacts retained per context; no closed stage/nested domain is claimed.

52 final bounded artificial tests passed:40 directory calls PC5679→5684 cap100 each, actual27/29operations;7 isolated record+1 spawn calls56a5→RAMc300 cap500, actual199/199/199/209/282/199/331;5 scansDE1000→c300 cap2000,actual307/117/385/347/41, HL exact terminal. Bank1/SPdff0 and clean session-local restore with explicit all8button releases(T001). Every spawn asserts active80,type,coordinate and parameter+18; these artificial states prove local consumers, not ordinary play. Equality tests did not spawn; retained negative logs. Local initial monotonic and parameter-offset assertions were corrected with full retry logs retained; no toolkit defect. [Replay](redirect/round-013/probe.py),[tests](redirect/round-013/tests.json),[RPC](redirect/round-013/probe-rpc.json). No new natural route/external research; unchanged gameplay/global-domain stalls not repeated.

Other commented/generic/raw source questions remain honestly deferred: apparent066f/06d6 audio literal boundaries, stage3flagged IDs, sound-source object contexts and global domains.063a's literal state8/9 writer gives a possible future narrower raw0729/0806 method, but no extra ready claim without prelude/dispatch verification. Ready bounded object data is a useful next disassembly batch; no exhaustive further exploration prerequisite.

map.load/map.validate passed. ROM,state,current.sym and configured release hashes unchanged; no alternate build/toolkit edits, no redundant source checkpoint. Existing round-013-disassembly fidelity remains byte_identical65536. [Redirect verification](redirect/round-013/verification.json). Toolkit retains1confirmedT001,0suspected/0missingfeatures,0new/no blocker;[report](toolkit-issues.md). [Compact redirect handoff](round-013-redirect-handoff.md). Next coordinator phase: round014 disassembly consuming the new ready lead. ROM remains incomplete.

## Round 014 — one disassembly pass finished; ROM incomplete

ROM classification: 66.54% (43607 / 65536 bytes classified; 21929 bytes unknown).

Input round-013-disassembly and round-013-redirect-handoff; output round-014-disassembly. Verified R13-OBJECT-7BB9-SELECTED-LISTS, adding 858 Static Data bytes in 69 regions. All 964 prior region objects, metadata and unrelated internal fields match frozen input exactly; all 1033 current regions carry role/evidence. Historical corrections, retractions and recovery artifacts remain preserved. [Independent reconstruction](round-014evidence/verified.json), [actual consumer/byte inspection](round-014evidence/rpc.json), [preservation](round-014evidence/preservation-check.json).

Independently recovered all 40 conditional stage/nested contexts from preserved descriptor cases and checked all 98 selector/descriptor source links against actual 44a7 directory words and descriptor first bytes. Loader262f writes that first byte to c412;082e independently resets stage/nested zero. Bank1 CPU addresses equal physical offsets under fixed0/switch1. Source5679 uses7bb9+2*stage then selected-table+2*c412 via RST08. Exact sparse consumption comprises8 stage-directory bytes,80 selected nested-word bytes,690 bytes in138 distinct five-byte records and80 bytes in40 coordinate boundaries. No min/max gap fill, global stage/nested domain or pointer-target code inference.

56b4 subtracts DE from the little-endian coordinate, records unsigned borrow in c3c8, conditionally negates signed BC via0365 and compares magnitude with10. Negative differences outside the window advanceHL by5; positive differences outside it stop. Equality leaves c3c8 zero, so56a5 does not spawn; historical successful constructed tests use coordinate+1. E000 is a comparison boundary for nonnegative DE<2000, not an unconditional sentinel opcode. Exhaustively checked all8192 values in that stated domain: no unsigned borrow and magnitude>=10. No conclusion is extended beyond this condition. Stage1/nested12 at7ed7 retains coordinates0070 then0038; records are not sorted and no natural selection of the second row is claimed.

56e1 reads record+3 as type and+4 as slot, computes c600+32*slot and skips active slots.312e copies record coordinate to object+11/12, parameter at record+2 to object+18, type at record+3 to object+1 and initializes active80. Exact field roles follow consumers, without speculative enemy names. Historical52 constructed calls remain corroboration of local consumers:40 directory caps100(27/29 operations),7 isolated coordinate+1 spawns caps500(199..331),5 DE1000 scans caps2000(41..385). Equality negative tests and parameter-offset correction history remain linked in [redirect findings](redirect/round-013/findings.md), [replay](redirect/round-013/probe.py) and scenarios.json. They were not rerun; no emulator, snapshot, new external research or natural route in this static pass. Existing sourced gameplay and diminished normal-route history are preserved. No materially improved natural route emerged, and unchanged stalled tests were not repeated.

Totals Code11718/Data31023/Padding866. Per-bank Code/Data/Padding/Unknown:00=8316/4919/73/3076;01=1229/2259/67/12829;02=1946/8553/37/5848;03=227/15292/689/176. [Metrics](round-014-classification.json), [exact unknown complement](round-014-unknown-ranges.json). Classification provenance is Observed8365/Static35242 bytes, separate from whole-history fetch coverage, which was not recomputed. No new Code/Observed claim. Stage3 flags, other bank1 objects, source tables1b7b/2134/2f7e, unresolved audio boundaries/dynamic45d6, global selector domains, natural gates and later-stage routes remain unresolved. Ready lead consumed; no other phase performed. Commentary review and final editorial acceptance remain pending.

Sequential map.validate, map.save to analysis/state.json and map.export to maps/current.sym completed before fresh checkpoint. Stock validation and generation passed; byte_identical=true covers all65536bytes. [Verification](round-014-disassembly-verification.json), [portable stock map](../maps/round-014-disassembly.sym), [bank-organized assembly](../generated/round-014-disassembly). Map SHA256 fc21ab998b2e4486ddffa920002ee52ad493ef2c7453e77e4f6f413103200c5f matches current.sym. Input ROM and configured release hashes remain unchanged; no toolkit edits or alternate binary. [Final verification](round-014evidence/final-verification.json).

Toolkit retains one confirmed T001, zero suspected bugs, zero missing features, zero new findings and no blocker. Static-only pass; historical input restoration workaround retained. [Report](toolkit-issues.md). Next coordinator phase: commentary. [Compact handoff](round-014-disassembly-handoff.md).

## Round 014 — intermediate commentary complete

Input round-014-disassembly, stock map SHA256 fc21ab998b2e4486ddffa920002ee52ad493ef2c7453e77e4f6f413103200c5f. Preserved the entire prior commentary in analysis/round-014/commentary-prior before fresh preparation. Carried all 512 reviewed ranges with exact phase-1 metadata, 1,024 checked source anchors, and prior names and comments preserved. Reviewed the 69 new regions (858 Static Data bytes): 581 reviewed, 612 pending. [Compact review and affected source anchors](round-014/commentary-review.json).

Reviewed the bank-1 stage/nested directory, 40 descriptor-backed contexts and 138 five-byte placement records against the static proof and consumer instructions. Added descriptive names and concise schema comments. Reassessed the three lookup consumers, distance scan, slot selection and object initializer: equality does not spawn; record type and slot select c600+32*slot; coordinate goes to object+11/12, parameter to +18 and type to +1. E000 is a coordinate comparison boundary only for nonnegative DE below 2000. Stage 1/nested 12 retains coordinates 0070 then 0038 without a sorting or natural second-row selection claim. Selected pointer values do not prove execution or a global stage/nested domain. Historical artificial tests remain historical; no new runtime, research or gameplay route was used. [Static proof](round-014evidence/verified.json), [constructed consumer findings](redirect/round-013/findings.md).

Remaining object slices, natural selection of the nonmonotonic second row, stage-3 flags, source tables 1b7b/2134/2f7e, audio instruction boundaries and dynamic 45d6 remain unresolved. No new evidence warrants repeating unchanged stalled experiments. Classification remains incomplete: 21,929 Unknown bytes, separate from editorial review. Final audit remains pending.

Coherent and final source rebuilds both passed byte_identical=true for all 65,536 bytes. The reading copy was refreshed at handoff. Frozen map/state equal current; no map, classification or toolkit edits. Repeated operand substitutions were restored explicitly, unchanged banks carried exactly, and every prior label/comment checked. [Final source verification](round-014/commentary-verification.json). Partial review only; no audit or other phase.

Toolkit retains one confirmed bug (T001), zero suspected bugs or missing features, zero new findings and no blocker. No emulator or snapshots used. [Persistent report](toolkit-issues.md).

## Round014 redirect — ready independent bank2 persistent placements

Input round-014-disassembly / round-014-commentary-handoff. Current classification unchanged43,607/65,536=66.54%,21,929Unknown; editorial581reviewed/612pending, final audit pending. Added ready R14-PERSISTENT-PLACEMENTS-663F; no previous leads changed. Fresh3079 explicit bank2 wrapper/308a stage&3/c412 lookup,30a2 unsigned coordinate loop and30d7/30ee/312e consumers establish40 conditional selected contexts,155 distinct five-byte rows and943 Unknown candidate bytes. Field+4 indexes CBxx flags; four-slot c580 pool, runtime type2, originaltype+1b,index+1c distinguish these from bank1 fixed-slot rows. Exact sparse roles/source cases in [evidence](redirect/round-014/evidence.json). E000 is an unsigned upper comparison boundary only under stated lower/upper bounds; stage1/nested15 coordinates0088,0058 remain nonmonotonic. No natural selection/global domain claim.

51 final bounded artificial tests passed:40 directory28ops cap100,6spawn171/172ops cap500,5terminal254/86/338/562/30ops cap3000. All fixed0/switch2,SPdff0 RAMreturnc300, clean session-local restore and all8buttons released(T001). Exact patches,budgets,requests,setup retry history preserved in [findings](redirect/round-014/findings.md),[replay](redirect/round-014/research.py),[tests](redirect/round-014/tests.json). Two local setup mistakes retained: omitted HL directory load and omitted E=index overwritten into c3cf; corrected from decoded source contracts. Neither toolkit bug nor natural gameplay evidence.

Map.validate passed; protected ROM/state/map/configuredrelease hashes unchanged; existing checkpoint remains byte_identical65,536. No map, classification, source or toolkit edits/new checkpoint. [Verification](redirect/round-014/verification.json),[handoff](round-014-redirect-handoff.md),[leads](leads.json). Independent concrete batch warrants next disassembly phase; other raw state/audio sources,globaldomains,stage3flags,dynamic45d6,natural later gameplay remain unresolved without repeated stalled tests. Toolkit existing1confirmedT001,0suspected/0missingfeatures,0new,no blocker;[report](toolkit-issues.md).

## Round 015 — one disassembly pass finished; ROM incomplete

ROM classification: 67.98% (44550 / 65536 bytes classified; 20986 bytes unknown).

Input round-014-disassembly and round-014-redirect-handoff; output round-015-disassembly. Verified R14-PERSISTENT-PLACEMENTS-663F with943 new Static Data bytes in68 regions. All1033 prior region objects/metadata and unrelated internal fields preserved exactly;1101/1101 current regions carry role/evidence. Historical corrections, retractions and recovery artifacts remain untouched. [Independent proof](round-015evidence/verified.json), [consumer and ROM inspection](round-015evidence/rpc.json), [preservation](round-015evidence/preservation-check.json).

Independently reconstructed40 accepted conditional stage/nested contexts from prior verified descriptor contexts, rechecked98 selector/descriptor links against44a7 ROM words and descriptor first bytes;262f writes first byte to c412 and082e resets initial stage/nested zero.3079 explicitly selectsbank2 through2180 and restores prior bank.308a gates c009&7==1,3095 selects663f directory using stage&3 then c412 through RST08. Physical bank2 aliases CPU=physical-4000 with fixed0/switch2. Exact sparse additions comprise8stage-directory bytes,80nested-word bytes,775bytes in155 distinct five-byte rows and80bytes in40 E000 coordinate boundaries. No gaps inferred from ranges or export fallback.

30a2 uses RST20 unsigned HL-DE subtraction: skip coordinate<lower, stop coordinate>=upper, otherwise check flag/spawn then advance5. E000 is an upper comparison boundary only when lower and upper<=E000, checked for all57345 unsigned bound values in that domain; it is not an unconditional sentinel and bank1 signed-distance grammar does not apply. Stage1/nested15 retains coordinates0088 then0058 without sorting or natural second-row selection claim.30d7 uses field+4 as CBxx persistence flag index.30ee searches four32-byte slots at c580;312e copies coordinate to+11/12,parameter+2 to+18 and original type+3 to+1.30ee then writes runtime type2, preserves original type at+1b and flag index at+1c, and marks CBxx flag80. No global original-type/index domains or sprite/enemy names inferred.

Historical51 bounded artificial tests corroborate local consumers only:40directory28ops cap100,6isolatedspawns171/172ops cap500,5scans30..562ops cap3000. Exact RAM/register patches, all8button releases after restore(T001), budgets and corrected setup retries remain in [redirect replay](redirect/round-014/research.py), [tests](redirect/round-014/tests.json), [findings](redirect/round-014/findings.md) and scenarios.json. These were not rerun; no emulator/snapshot/new external research/natural route this static pass. Existing sourced gameplay routes and diminished natural-route results retained. No materially improved ordinary route emerged; unchanged stalled leads were not repeated.

Totals Code11718/Data31966/Padding866. Per-bank Code/Data/Padding/Unknown:00=8316/4919/73/3076;01=1229/2259/67/12829;02=1946/9496/37/4905;03=227/15292/689/176. [Metrics](round-015-classification.json), [exact unknown complement](round-015-unknown-ranges.json). Provenance Observed8365/Static36185 bytes; execution coverage remains a separate historical metric, not recomputed. No new observed/code claim. Other raw main-state/audio boundaries,1b7b/2134/2f7e selectors,dynamic45d6,stage3flags,global domains and natural later gameplay remain unresolved. No other phase or worker; commentary/final editorial review remain pending.

Sequential map.validate, map.save analysis/state.json and map.export maps/current.sym completed before fresh checkpoint. Stock validation/generation passed; byte_identical=true for65536bytes. [Verification](round-015-disassembly-verification.json), [portable stock map](../maps/round-015-disassembly.sym), [bank-organized assembly](../generated/round-015-disassembly). Current/checkpoint map SHA256564c50144aced8cc93555119f375ce7654bff05935610a854dafe40e8f438d0b. ROM and configured release unchanged; no toolkit changes or alternate binary. [Final verification](round-015evidence/final-verification.json).

Toolkit retains one confirmed T001, zero suspected bugs/missing features, zero new findings and no blocker. [Report](toolkit-issues.md). [Compact handoff](round-015-disassembly-handoff.md). Next coordinator phase: commentary.

## Round 015 — intermediate commentary complete

Input round-015-disassembly, stock map SHA256 564c50144aced8cc93555119f375ce7654bff05935610a854dafe40e8f438d0b. Preserved the entire prior commentary in analysis/round-015/commentary-prior before fresh preparation. Carried all 581 prior reviewed ranges with exact phase-1 metadata, 1,162 checked source anchors and all prior labels/comments. No review credit invalidated. Reviewed 68 new regions containing 943 Static Data bytes: 649 reviewed, 619 pending; editorial audit remains pending. [Review and dependencies](round-015/commentary-review.json), [static proof](round-015evidence/verified.json).

Reviewed the bank-2 stage/nested directory and 155 five-byte records in forty descriptor-backed contexts. Named the directory, records and affected consumers. The final record field indexes CBxx flags, unlike the bank-1 slot field. The consumer allocates from four c580 slots, sets runtime type 2 and saves original type at +1b and flag index at +1c. RST20 compares unsigned: skip below lower, stop at or above upper. E000 is a stopping bound under lower/upper <= E000, not an unconditional sentinel. Stage 1/nested 15 retains 0088 then 0058. No sorting, sprite, global type/index domain or natural second-row selection claim follows.

Affected dependencies 3079/308a/30a2/30d7/30ee and shared initializer 312e were checked against the source; bank-1 signed-distance interpretation remains separate. Natural selection of the nonmonotonic second row, remaining object slices, stage-3 flags, source tables 1b7b/2134/2f7e, dynamic 45d6, audio instruction boundaries and later gameplay remain deferred. Existing gameplay research and replay routes preserved; no new runtime or external research used. ROM remains incomplete with 20,986 Unknown bytes; source review is partial.

Coherent and final rebuilds passed byte_identical=true for all 65,536 bytes, and the reading copy was refreshed at handoff. Frozen map/state equal current; no classification, map or toolkit edits. Carry exceptions were repeated operands and stable comment/label anchors, restored and checked; unchanged banks carried exactly before the targeted consumer edits. No final audit or other phase performed. [Verification](round-015/commentary-verification.json).

Toolkit retains one confirmed bug T001, zero suspected bugs/missing features, zero new findings and no blocker. No emulator/snapshot use. [Persistent report](toolkit-issues.md).

## Round015 — redirect complete, ready finite downstream batch

Input round-015-disassembly and compact commentary; classification unchanged67.98%(44,550/65,536),20,986Unknown;649reviewed/619pending. Investigated accepted placement-value sets against33a8 type dispatch, persistent2f0a/2f45 conversion and16fe/160c/1684 real sprite consumer. Added ready `R15-FINITE-OBJECT-DISPATCH-SPRITES` (237Data bytes) and `R15-FINITE-INITIAL-STATE-WRAPPERS` (102Code+36Data bytes). Combined375 currentUnknown bytes; finite sparse cases only, no global type/state/domain, natural later-stage selection or reached-body execution claim. [Findings/recipes](redirect/round-015/findings.md),[roles](redirect/round-015/evidence.json),[verified ranges](redirect/round-015/verification.json),[leads](leads.json).

51 final dispatch/conversion/initializer cases(max199ops),44 renderer cases(max163ops),22 state0 wrapper cases(26ops) passed. Wrapper Instruction/Read traces have zero drops; three wrapper instructions fetched and first selected inline word read, bodies merely reached. SpriteFE/FF/count0 grammar closes22 nodes with sparse pointed component payloads; no adjacency inference. Runtime2 original+1b converts before sprite selection;2f7e reads+9 and remains a separate unresolved domain. Bank1 animation pointers do not transfer to coincident bank2:46d9. Initial50-case successful version expanded to51 with runtime2 word; no failed runtime tests. Local scratch base/DE and final verification schema-key corrections retained in[retry record](redirect/round-015/local-retry.md);not toolkit issues.

map.validateok=true. Protected ROM,state,map,commentedsource and configuredrelease hashes unchanged after all tests; no map.save/export/checkpoint because this redirect leaves classification untouched. Existing round015 checkpoint and commentary rebuild remain byte-identical65,536. [Verification](redirect/round-015/verification.json),[handoff](round-015-redirect-handoff.md). All patches/inputs/caps/logs persisted; snapshots session-local and explicitly release all8buttons after restore(T001). No new natural route or external claim. Other raw handler forms/bodies,main/audio boundaries,1b7b/2134/2f7e,dynamic45d6,stage3flags and natural nonmonotonic row consumption remain unresolved. Stop this pass with useful bounded leads; coordinator may proceed to next disassembly. Existing1confirmedT001,0new/0suspected/0missing features;no blocker,[toolkit report](toolkit-issues.md).

## Round 016 — one disassembly pass finished; ROM incomplete

ROM classification: 68.55% (44925 / 65536 bytes classified; 20611 bytes unknown).

Input round-015-disassembly, round-015 redirect and existing commentary history; output round-016-disassembly. Both ready R15-FINITE-OBJECT-DISPATCH-SPRITES and R15-FINITE-INITIAL-STATE-WRAPPERS verified. Added375 bytes in115 regions:102 Observed Code,273 Static Data. All1101 prior region objects/metadata and unrelated internal fields preserved exactly;1216/1216 current regions have role/evidence. Prior names, reviews, history, retractions and recovery files retained. [Independent proof](round-016evidence/verified.json), [inspection/mutation journal](round-016evidence/mutation-rpc.json), [preservation](round-016evidence/preservation-check.json).

Reconstructed29 accepted bank1 placement types plus persistent runtime2 from prior exact context rows;312e retains bank1 runtime type while30ee separately stores original+1b and sets runtime2.33a8/RST00 selects thirty exact words at33ae+2*type,60 new Data bytes. Accepted original0..8 convert through2f45 to0..6:7 maps0/2/5 by c51c,8 maps2/3 by tick low nibble<13.2f0a overwrites+1b with conversion, then selects seven2f77 animation bytes.2f7e reads separate+9; no selector domain inferred. State0 animation13 and initializer0 plus selected14/15/16/17/58/73/74 give nine bank1:46d9 pointer words. Fixed0/switch1; bank1 physical=CPU. Coincident bank2:46d9 is excluded.

Independent finite grammar walk closes22 nodes. FE contains count/control,duration/reserved and payload pointer; FF contains jump pointer; count0 is stop. Payload count/duration precede sparse x/y/tile components: odd tile inherits flags, even tile adds explicit attribute XOR flags.16fe reads duration even for animation0; RST30 offsets destination, not source. Only exact roles classified; skipped/reserved fields retain structural record roles, no intervening gaps. Pointer/frame/payload slice adds170 Data bytes, plus7 converted selectors and60 type words=237. Consumer decoding and every added exact range inspected before annotation.

Twenty-two historical state0 cases cover20 unique6-byte wrappers. Independent trace audit checks actual bytes3e02cddc32c7; physical/CPU/bank aliases; fetched entry,entry+2,entry+5; first inline word's two Read events; zero detail/aggregate drops;26operations/cap100. First-state target bodies were reached but NOT fetched. Only17 unknown wrapper prefixes102bytes and18 first words36bytes added. No global inline-table extent or body execution inferred. Historical51 dispatch/conversion/initializer plus44 renderer tests corroborate static data; no new emulator/snapshot or natural route this pass. Exact setup, budgets, inputs and local retry history preserved in [redirect findings](redirect/round-015/findings.md) and scenarios.json. Ordinary gameplay previously diminished; no materially improved route identified, unchanged stalled leads not repeated.

Totals Code11820/Data32239/Padding866. Per-bank Code/Data/Padding/Unknown:00=8316/4986/73/3009;01=1331/2465/67/12521;02=1946/9496/37/4905;03=227/15292/689/176. [Metrics](round-016-classification.json), [exact unknown complement](round-016-unknown-ranges.json). Provenance Observed8467/Static36458 bytes; execution coverage remains separate historical evidence, not recomputed. Reached first-state bodies and other direct handler forms remain useful contextual inspection candidates only. Raw main-state/audio boundaries,selectors1b7b/2134/2f7e,dynamic45d6,stage3flags,global domains and natural nonmonotonic-row/later-stage reachability remain unresolved. No other phase or worker performed; final editorial acceptance pending.

Sequential map.validate, map.save analysis/state.json and stock map.export maps/current.sym completed before fresh checkpoint. Validation/generation passed and byte_identical=true for65536 bytes. [Checkpoint verification](round-016-disassembly-verification.json), [portable stock map](../maps/round-016-disassembly.sym), [bank-organized listing](../generated/round-016-disassembly). Map SHA2569e20470536766a3cdc9cd5db06d6f2b9c04d7fc1e7f5cd11dcc497f453be20a6. ROM/configured release/commented source unchanged; no toolkit edits or substitution. [Final verification](round-016evidence/final-verification.json).

Toolkit: existing1confirmedT001,0suspected/0missingfeatures,0new/no blocker. [Persistent report](toolkit-issues.md). [Compact handoff](round-016-disassembly-handoff.md). Next coordinator phase commentary.

## Round 016 — intermediate commentary complete

Input round-016-disassembly, map SHA256 9e20470536766a3cdc9cd5db06d6f2b9c04d7fc1e7f5cd11dcc497f453be20a6. Entire prior commentary preserved under analysis/round-016/commentary-prior before fresh preparation. Carried all 649 prior reviewed ranges with exact phase1 metadata, all prior names/comments, dependencies and 1,298 remapped source anchors; no review credit invalidated. Repeated operand substitutions and placement-tail insertion exceptions were restored against stable source anchors. Reviewed 115 new regions (102 Observed Code bytes, 273 Static Data bytes): 764 reviewed, 655 pending; editorial audit pending. [Compact review](round-016/commentary-review.json), [proof](round-016evidence/verified.json).

Reviewed thirty finite type-dispatch words, persistent conversion and seven animation selectors, nine bank-1 animation pointers, 22 closed FE/FF/count-zero frame nodes and sparse sprite payloads. Named affected type, animation and renderer consumers. Converted type retained at +1b differs from the separate +9 selector used by 2f7e. FE selects a separate payload; FF redirects the frame stream; zero stops. Even tile bytes supply attributes XOR object flags; odd tiles reuse attributes. Bank-2 coincident 46d9 is excluded. These slices establish neither a global type/index domain nor natural selection.

Initial-state wrapper traces establish three fetched entry instructions and reads of the first inline word. The reached body was NOT fetched. No global inline-table extent or body execution is claimed. For redirect: inspect the selected first-state bodies with their effective bank context before classifying them; establish later state indices separately. Other handler forms, dynamic 45d6, source tables 1b7b/2134/2f7e, audio boundaries, stage-3 flags and natural later/nonmonotonic selections remain deferred. Existing gameplay familiarity and replay routes preserved; no new runtime or external research used. ROM incomplete: 20,611 Unknown bytes.

Two coherent/final source rebuilds passed byte_identical=true for all 65,536 bytes; reading-after refreshed. Frozen map/state equal current. [Final verification](round-016/commentary-verification.json). No map, classification or toolkit edits; no final audit or other phase. Partial review only.

Toolkit: existing one confirmed bug T001, zero suspected bugs/missing features, zero new findings and no blocker. No emulator/snapshot use. [Persistent report](toolkit-issues.md).

## Round016 redirect — ready finite first-state consumer batch

Input checkpoint round-016-disassembly; commentary handoff accepted immediately (764reviewed/655pending). Classification unchanged44,925/65,536=68.55%, Unknown20,611. Independently inspected finite reached first-state targets and helpers, then18 bounded wrapper-to-return tests used correctc600 bank1 slots (persistent2c580), playerc500 and safe return stack. Actual bodies now fetched; state+2 becomes1 in78..377ops/cap1000,SPdff2; no state1-body/global-domain claim. Separate Instruction/Read/Write traces have0 detailed/aggregate drops. Type3's temporary switch2 audio context audited by physical/CPU alias rather than blanketbank1 assumption.

571 actual fetched starts independently decoded;437 Unknown Code bytes in17 sparse intervals. Seven exact10-byte32b2 initializer records and one8-byte31d8 record plus ten observed literal animation IDs yield354 sparse Unknown Data bytes,26 closed frame nodes/payloads.52 renderer tests pass OAM/pointer contracts(max423ops/cap1000). Total791 candidate bytes, not classified in redirect. Ready IDs R16-FETCHED-FIRST-STATE-BODIES and R16-FIRST-STATE-INITIALIZERS-SPRITES; prior leads/outcomes preserved. Local summary-key/stdlib-name/context-audit mistakes corrected, original retry retained; no toolkit defect.

map.validate ok=true; protected ROM,state,current.sym,configuredrelease and all4 commented source hashes unchanged after all tests. Existing round016 checkpoint/commentary byte-identical65536 verification retained; no new unchanged-map checkpoint. No toolkit/source/map/classification edits, no other phase. [Findings and exact setup/limitations](redirect/round-016/findings.md),[verification](redirect/round-016/verification.json),[handoff](round-016-redirect-handoff.md),[leads](leads.json). Deferred types25/33/45/46 and state1 bodies require separate finite contracts; no unchanged gameplay/global stalls. ExistingT001 remains only confirmed bug;0new/0suspected/0missing features,no blocker;[report](toolkit-issues.md).

## Round017 — first-state bodies and sparse initializer/sprite structures

ROM classification: 69.76% (45716 / 65536 bytes classified; 19820 bytes unknown).

Input round-016-disassembly and round016 redirect handoff; output round-017-disassembly. One disassembly pass complete, ROM incomplete. Both ready leads R16-FETCHED-FIRST-STATE-BODIES and R16-FIRST-STATE-INITIALIZERS-SPRITES are verified: 437 Code plus 354 Data bytes added, 105 new regions. All 1216 prior region objects and metadata, unrelated store fields, recovery history, ROM, configured release and commented source remain unchanged. All 1321 regions have role and supporting evidence.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8316 | 4986 | 73 | 13375 | 3009 |
| 01 | 1768 | 2819 | 67 | 4654 | 11730 |
| 02 | 1946 | 9496 | 37 | 11479 | 4905 |
| 03 | 227 | 15292 | 689 | 16208 | 176 |

Independent analysis.structure/analysis.decode inspection of all 571 actual fetched starts retained physical/CPU aliases and effective context; type3 temporarily fetches physical8706 as CPU4706 in bank2 audio before restoring bank1. Historical18 constructed wrapper-to-return tests retain separate Instruction/Read/Write arrays, zero detailed/aggregate drops,78..377operations/cap1000, returnSPdff2 and state+2=1. No new runtime or snapshot used this pass. Execution evidence is historical and finite, distinct from classification: writing state1 proves no state1-body execution. Code added only actual fetched instruction extents, not static reachability or reached targets.

Independently inspected32b2/31d8 confirms exact4+4+2 and4+4 initializer copies, respectively; all78 record bytes have actual CPU Read coverage. Ten literal16fe entry selectors resolve exact bank1 pointer words and26 closed frame nodes. Sparse FE indirect payloads, FF jumps, count0 stops and component count/duration/Y/X/tile/optional attributes are independently reconstructed; 52 historical renderer cases cover every node at timers1/2, max423operations/cap1000. Grammar data provenance is Static, confidence0.97; fetched code Observed, confidence0.97. Sparse roles exclude adjacent gaps and do not infer whole actor/state/animation domains.

Sequential map.validate(ok=true), map.save analysis/state.json and map.export maps/current.sym completed before fresh checkpoint. python3 run.py checkpoint round-017-disassembly validated, generated bank-organized source and rebuilt all65536bytes byte-identically; map SHA256 db63b6031c819c689a80c4848d634b0606e1b69ee4dc83c7a35ec9501caf3edf. [Verification](round-017-disassembly-verification.json), [independent proof](round-017evidence/verified.json), [preservation/final audit](round-017evidence/final-verification.json), [unknown intervals](round-017-unknown-ranges.json). Stock map maps/round-017-disassembly.sym; local source generated/round-017-disassembly.

Stop at verified finite easy-gain slice: no unchanged gameplay/70-case replay or recursive actor closure. Types25 predecessor,33 companion allocation,45/46 variant parser, stage flags, dynamic tables, state1 bodies and later natural selection remain deferred with their earlier hypotheses. Types13/14 direct fields do not establish sprite pointers;26/27 prove only observed no-match companion scan. Existing sourced routes and T001 workaround preserved; no new external research or natural gameplay claim.

Toolkit: existing one confirmed T001; zero new, suspected bugs or missing features and no blocker. No toolkit changes/rebuild/substitution. [Persistent report](toolkit-issues.md).

## Round 017 — intermediate commentary complete

ROM classification remains 69.76% (45,716 / 65,536 bytes classified; 19,820 unknown). Input/output structural checkpoint round-017-disassembly; map SHA256 db63b6031c819c689a80c4848d634b0606e1b69ee4dc83c7a35ec9501caf3edf. Entire prior commentary preserved under analysis/round-017/commentary-prior before fresh preparation; interrupted working copy retained separately. Carried 764 prior reviews with exact phase1 metadata and 1,528 checked source anchors. Prior symbols/comments retained; two affected wrapper statements now distinguish historical wrapper-only tests from the finite round17 body fetches. No review credit invalidated. Reviewed all 105 new regions (437 Observed Code, 354 Static Data bytes): 869 reviewed, 692 pending; editorial audit pending. [Compact review](round-017/commentary-review.json).

Named first-state initialization, occupancy, companion scanning and coordinate helpers. Reviewed exact seven ten-byte 32b2 records (4+4+2 copy) and one eight-byte 31d8 record (4+4 copy), literal bank1 animation words and sparse closed FE/FF/zero frame/payload structures. These are finite constructed consumer observations and static format interpretations. Type3 submits sound $16 through fixed00bd, temporarily fetching bank2 CPU4706/physical8706 before restoring bank1. State0-to1 writes prove no state1-body execution. Types13/14 direct fields do not establish sprite pointers;26/27 only exercise no-match companion scanning; type31 only its zero-coordinate path; type40 occupancy/RNG values do not define their domains.

For redirect, separately establish finite predecessor/slot contracts for type25, companion allocation and downstream parser contracts for33, and variant parser contracts for45/46 before executing or expanding them. State1 bodies, dynamic45d6, +9 sound selectors, source tables1b7b/2134/2f7e, stage3 flags, audio boundaries and later/nonmonotonic selection remain deferred. [Historical exact setup and limitations](redirect/round-016/findings.md), [independent structural evidence](round-017evidence/verified.json). No new runtime, external gameplay research or natural route claim.

Carried-source and final edited-source rebuilds passed byte_identical=true for all65,536 bytes; reading-after refreshed. Frozen phase1 map/state equal current artifacts. [Final verification](round-017/commentary-verification.json). No map/classification/toolkit changes, no final audit or other phase. Partial review only.

Toolkit: existing one confirmed T001, zero suspected bugs/missing features, zero new findings and no blocker. Carry exceptions were local source anchoring matters; no toolkit edits. [Persistent report](toolkit-issues.md).

## Readable-source requirement restored (2026-10-17)
User clarified that fully commented, labeled assembly with variable and branch names
is a required input to the porting agent. Active commentary/coordinator/redirect
instructions now require incremental work on older readability gaps and final
source-quality assessment separate from classification/audit. Prior review credit
does not exempt concrete naming/comment defects. No ROM, map, source, review status
or classification changed; paused phase remains unchanged. Next commentary must
include shared RAM symbols and older unexplained helpers alongside its new scope.

## Commentary catch-up — 2026-10-17 (paused goal unchanged)

Commentary only on round-017 verified source. Classification remains 69.76% (45,716/65,536); 19,820 unknown bytes. No map/classification edits, runtime, research, input sequences or new execution claims. Existing gameplay records and replay routes remain unchanged. No redirect or disassembly pass started.

Added shared.inc with 13 established WRAM symbols and the HRAM DMA entry. Replaced their literal LD operands consistently across bank files. Named 54 existing routine/branch labels, including interrupt stubs, frame-loop waits, word dispatch carry paths, state/substate setters, arithmetic helpers, palette/video initialization and LCD synchronization. Added 22 functional explanations. Examples: NextRandomByte now explains the accumulator/frame-byte mixer and preserved registers; CompareHLWithDE documents unsigned carry; WaitForNextHBlank explains its two-mode-0 sequence and running-LCD requirement. Existing finite actor/sprite research remains intact in assembly; no natural selection inferred. Source-only names do not alter stock symbols.

Backlog advanced in previously reviewed and older classified code; prior scaffold/status credit preserved, source anchors relocated, six compact helper units added. This is a useful shared-infrastructure improvement, not readable completion. Remaining gaps include the early input/pause flow around 0266/02eb (stack-unwinding behavior needs prose), generic main-state bodies, further video/BCD helpers, actor field offsets and bank-2 channel state. Scratch c3c8/c3c9 remains numeric because it has multiple uses. RNG distribution/consumer meaning and peripheral intent of shared interrupt tails remain explicitly unresolved.

Next catch-up scope: explain and name the 0266/02eb pause/input paths, remaining 0463..082e state bodies and their established delay/selector variables; then carry descriptor/layout field names and actor field constants from existing research into operands. Do not infer new domains or classify gaps.

Verification: commentary.py verify passed ok=true, byte_identical=true for all 65,536 bytes; source hashes include shared.inc. commentary.py read refreshes reading-after.asm at handoff. No final editorial audit requested. Exact backup, symbol manifest, protected hashes and verification are in analysis/commentary-catchup-2026-10-17/. ROM, current map, internal state, frozen baselines and configured release are unchanged.

Toolkit: existing T001 is the sole confirmed bug; zero new/suspected/missing-feature findings, no blocker. No emulator/snapshot/toolkit changes. Report: analysis/toolkit-issues.md.

## Commentary catch-up 002 — active readability goal

Previous goal turn made verified source progress. Continued only commentary on round-017-disassembly; structural classification remains 45,716/65,536 (69.76%), 19,820 unknown bytes. No redirect/disassembly, runtime, input sequence, research, map or toolkit edits. Protected ROM/map/state/frozen-baseline/release hashes match the pre-catch-up values.

Replaced 42 additional generic routine/branch labels and seven established WRAM operands consistently across banks. Added 33 explanations covering early-state Start/Select handling, pause stack unwinding and audio lock/bank restoration, intro/countdown/prompt states, stage-setup persistence reset, progress initialization, gameplay update branches and local bank-2 arithmetic/copy helpers. Shared state delay is a wrapping counter; stage/descriptor/nested-layout selectors remain distinct. Raw source bytes and unsupported selector domains stay unresolved. The pause gate requires an outer CALL continuation and deliberately discards the gameplay return when paused; this behavior is now stated beside the code rather than hidden behind generic labels.

Reviewed units were added without changing scaffold boundaries or prior statuses. Source references were relocated using exact renamed label anchors and bounded diff alignment. Prior review credit retained. Readability acceptance remains incomplete: unresolved actor field constants and generic branches, sound channel state, older scrolling/video/BCD helpers, data structure prose and unexplained unknown intervals still need coverage. Mechanical indicator counts in preservation-and-backlog.json are navigation aids, not semantic acceptance evidence.

Next scope: older fixed-bank HUD/BCD helpers 038c..0433 and 0b03..0c37, then scrolling/layout variables and actor field offsets using existing research. Continue through bank-1 state bodies and bank-2 channel machinery afterward; final review must address every scaffold entry/unit and source-quality gaps, including earlier reviewed ranges. Do not substitute a green build or renamed-label count for that assessment.

Verification: commentary.py verify ok=true, byte_identical=true for all 65,536 bytes; commentary.py read refreshed reading-after.asm. Final audit remains pending until full editorial coverage. Exact source backup, changes, units and protected checks: analysis/commentary-catchup-002/. Existing sourced gameplay routes remain unchanged.

Toolkit: existing T001 only; zero new/suspected/missing-feature findings, no blocker. See analysis/toolkit-issues.md. The readability goal remains active; no overall completion claim.

## Commentary catch-up 003 — timer, score and HUD contracts

Previous turn was verified source progress; current goal remains full readability acceptance. Scope: older fixed-bank timer/HUD routines 038c..0433, tile-map helpers 08f1/094f, scoring/digit routines 0b03..0b78 and 31f6..326f, related callers and numeric shared operands. Added 39 functional routine/branch names, ten RAM symbols and 17 purpose/contract comments. Existing source labels and source locations were updated consistently across bank files; prior review credit/scaffold preserved, with compact units for the reviewed entry fragments.

Established mechanics now in assembly: BCD seconds borrow to 59, minute decrement, 32-game-frame cadence, alternate-mode gate and current-bank timeout target; shared score selection by mode bits 0/2, decimal carry through byte 2 followed by binary byte-3 carry; changed-byte score threshold comparison; reverse-byte/high-low digit order, optional leading-zero suppression, always-visible final digit, Y/X scratch interpretation and OAM cursor. Four score bytes must not be described uniformly as packed BCD. Timer names express the rollover mechanics, not a calibrated wall-clock rate. The raw score-award tail remains unclassified and its wider reward behavior unresolved. Scene tile identities and bank-inherited callbacks remain qualified.

Classification unchanged: 45,716/65,536 (69.76%), 19,820 unknown bytes. No map/state/ROM/toolkit/runtime/input/research changes; no redirect or disassembly phase. All protected hashes match the initial catch-up snapshot. Shared variables now replace numeric LD operands consistently wherever those absolute addresses occur, without converting raw byte emissions or changing classification.

Verification: commentary.py verify ok=true and byte_identical=true for all 65,536 bytes; read refreshed reading-after.asm. Final acceptance audit remains pending. Exact edited-source verification, backup, change manifest and preservation check: analysis/commentary-catchup-003/. Toolkit: T001 remains the only confirmed issue; zero new/suspected/missing-feature findings and no blocker. See analysis/toolkit-issues.md.

Agent assessment: the shared timer/scoring/HUD infrastructure is materially more readable, but the whole listing still fails full commentary acceptance. Next: descriptor/layout and transition RAM roles (c400/c401 byte order, c40c/d width-derived coordinate bound, object/player coordinate fields); remove the stale first-width-only comment using existing verified selector records. Then actor field constants and older bank-1/2 state machinery. Unexplained bulk/unknown intervals and pending inventory entries still require explicit review; a build alone cannot satisfy that gate.

## Commentary catch-up 004 — layout, camera and transition contracts (phase interrupted)

Previous goal turn was progress. Continued the same commentary phase across existing classified source. Added 68 functional routine/branch names, 22 established RAM symbols and 18 explanations covering descriptor/block address byte order, player world-X clamping and camera derivation, queued versus active updates, five-block column completion, seven-column priming, screen-major layout indexing, packed class rows and transition initialization/window motion. No classification, map, ROM, runtime/input/gameplay research or toolkit changes; classification remains 45,716/65,536 (69.76%), Unknown 19,820.

Camera-result bytes c3d8/c3d9 are reused by bank1's three-cell class scan; caller checks caught that aliasing before final naming. Source uses wScreenXOrScanCount / wScrollXOrScanMask and describes the alternate context, rather than claiming persistent camera variables. Major/cross indices swap row/column roles between scrolling and transitions; shared names preserve that distinction. Block base/destination words are high-first, while world X and layout width are low-first. Corrected stale first-entry-only layout/width comments with the existing sparse verified domains; gaps still do not prove table lengths or natural reachability.

Review scaffold and statuses preserved, source anchors relocated and 18 compact reviewed helper fragments added. Unintended newline normalization of hardware.inc during a broad text operation was restored from the original include before final verification; original hash 491941c0e77f6dbf539124835a8108122eab1a342bafebd4582776ab0500374a restored. This was a local editing matter, not a toolkit issue. Full protected artifact checks pass in analysis/commentary-catchup-004/preservation.json.

Verification: final commentary.py verify ok=true, byte_identical=true across 65,536 bytes; commentary.py read refreshed reading-after.asm. Exact backup/change manifest and verification remain in analysis/commentary-catchup-004/. No final audit or overall success claim: the entire current-source requirement is not yet met. Commentary continues before any redirect/disassembly.

Remaining full scope is recorded by exact physical ranges in analysis/commentary-remaining-inventory.json (all pending scaffold entries, including unknown boundaries); previously reviewed records also remain subject to concrete readability repairs. Immediate next work: missing bare main-state entry labels (04ad,056a,05a5,061c,062b), collision-class extraction around2937..2a12 and player/actor pointer accessors2352/32dc. Then bank1 actor initializers/state machines and bank2 channel/stream machinery, remaining data and unknown boundary notes. Final acceptance must reconcile units/scaffold/source locations, resolve naming/comment gaps throughout all four banks, record an agent source-quality assessment and pass commentary.py audit plus a fresh byte-identical rebuild/read refresh. Green builds are fidelity evidence only.

Toolkit: T001 remains the sole confirmed bug; zero new/suspected/missing-feature findings and no blocker. See analysis/toolkit-issues.md. Goal remains active; this is a saved interruption of commentary, not a successful phase handoff.

## Commentary catch-up 005 — actor field contracts and bare state entries (phase interrupted)

Previous turn made verified progress. Continued the same full-source commentary phase; maps/classification unchanged at 45,716/65,536 (69.76%), Unknown 19,820. No runtime, gameplay/input sequence, new research, redirect or disassembly. ROM/current map/internal state/frozen comparison artifacts/configured release hashes remain unchanged.

Named 29 older accessor/initializer/collision routine and branch labels and five previously bare state entry points: title04ad, stage-scene056a, setup-delay05a5, playable-scene061c and gameplay062b. Added four actor-pointer RAM symbols and nine shared actor field/size constants; directly passed known actor offsets now use symbols across bank files. Named primary versus current actor pointers rather than assuming every pointer is the player. Added 18 helper contracts plus five state descriptions.

Accessor2352/32dc uses low-byte-only offset addition, so the page constraint is explicit. Initializers32b2 and31d8 explain exact4+4+2 and4+4 copies and animation A preservation. Collision probe2937 explains signed horizontal offsets, cache row/column wrap, field rotation and scratch outputs;29d2 separately documents its eight-bit horizontal tilemap calculation. Deactivation3199/319e was checked instruction-by-instruction: it clears active/type, puts sprite X at e0 and clears animation; it does NOT clear sprite Y or the whole slot. This corrected the first draft before final verification. Existing selector/domain/runtime uncertainty remains preserved.

Reconciled previously completed source review with the inventory: 52 fully inspected Code records received review credit, including older helper fragments and now-commented bare entries inside longer regions. This was not a blanket status reset or a label-count acceptance rule. 921 scaffold records reviewed,640 remain pending;23 compact reviewed helper/entry units added. Exact credited physical records are in analysis/commentary-catchup-005/preservation-and-review.json. Full remaining ranges refreshed in analysis/commentary-remaining-inventory.json. Final editorial acceptance remains pending, including concrete defects in already-reviewed material.

Verification: final commentary.py verify ok=true,byte_identical=true for all65,536bytes; read refreshed reading-after.asm. Original hardware.inc hash retained. No final audit or overall goal-completion claim. Backup, changes, preservation and exact final verification are in analysis/commentary-catchup-005/.

Next full-source work: actor render/update frame loop15a4..16fe and persistent actor update32ec..33a8, then older physics/collision blocks around22a9..2352 and bank1 first-state consumers. Continue bank2 channel/stream code, remaining generic branch/routine labels, data prose, and unknown-range boundary notes afterward. Before final acceptance, reconcile every region/unit/source reference, explicitly assess source quality across all four banks, and run the final audit/fresh rebuild/read refresh. No redirect until current-source readability is complete. Goal active; saved interruption only.

Toolkit: existing T001 only; zero new/suspected/missing-feature findings,no blocker. Deactivation draft correction was local commentary review, not a toolkit bug. Persistent report: analysis/toolkit-issues.md.

## Commentary catch-up006 — renderer, motion and actor update (phase interrupted)

Previous goal turn was progress. Continued the same whole-source commentary phase. Named60 older render/update/motion labels, removed19 redundant generic aliases beside descriptive labels, added one actor-loop RAM symbol and four frame/flash field constants, and added/expanded contracts for20 helpers plus the renderer/component/animation routines. No ROM/map/classification/toolkit/runtime/research/input changes; classification stays45,716/65,536(69.76%),Unknown19,820. All protected initial hashes remain unchanged.

Renderer15a4..16fe now explains23 slots(c500..c7c0), HUD gates, X-byte hiding, inherited-bank count/FE/FF grammar, timer-zero underflow, shared-payload versus stream-pointer ownership, flip origins and odd/even tile attribute behavior. Motion2277..2352 explains horizontal acceleration/24-bit position versus vertical velocity clamping/sign reversal and16-bit fraction/Y; raw continuations/literals remain explicitly excluded from Code claims. Actor update32ec..33a8 explains18slots(c580..c7a0), flash parity mapped to sprite palette bit4, type-before-motion order, state9 culling bypass and type2/type19 retirement exceptions. Unknown physics/game purposes stay mechanical rather than invented.

Review metadata repaired: all source references are now relative to commentary/source, matching the final audit's path resolution. Old source/bank paths would have resolved to source/source/bank and failed. This was local metadata format debt, not a toolkit defect.1,943 existing/new references were checked before the additional complete-routine fragments were credited. After manual source review,28 Code scaffold records received credit across exact named routine ranges;20 new helper units plus compact per-region routine fragments preserve the immutable scaffold.949records reviewed;612pending. Prior sound reviews retained. Exact fragments/spans and protected checks: analysis/commentary-catchup-006/preservation-and-review.json. Future edit scripts must accept bank_000.asm:line references (without the former source/ prefix) when relocating anchors.

Final source verification: commentary.py verify ok=true,byte_identical=true for65,536bytes; commentary.py read refreshed reading-after.asm. Original hardware.inc remains byte-for-byte unchanged. No final audit or overall acceptance claim. The full-source requirement is still incomplete; saved interruption only, no redirect/disassembly.

Next: previously classified fixed-bank collision/actor blocks21bd..2277,2352..262f, and player input/state code172c..21bd; remaining bank1 first-state/actor helpers and bank2 audio paths follow. Also reconcile older vector/startup review fragments, pending data/padding/unknown-boundary notes, and source anchors. Full exact pending ranges remain in analysis/commentary-remaining-inventory.json. Final completion requires every scaffold/unit reviewed, consistent variables/fields/constants, no unexplained meaningful code/data blocks, explicit uncertainty at unknown boundaries, an agent full-source quality assessment and final commentary.py audit/fresh verification/read refresh.

Toolkit: existing T001 only; zero new/suspected/missing-feature findings,no blocker. No emulator/snapshot use or toolkit edits. Persistent report analysis/toolkit-issues.md. Goal remains active.

## Commentary catch-up007 — player frame and collision helpers (phase interrupted)

Previous turn made verified progress. Added42 functional labels, six player/HUD/previous-X/countdown symbols, three shape/motion field constants and19 functional comments. Scope:172c..1895 player initialization/frame dispatch/window checks,21bd score selector,2219..2277 sprite box test and2376..2522 terrain probes. The box test is eight-bit screen X/Y distance with actor eligibility and summed shape extents; packed terrain probes instead sample shape-selected offset pairs, use maxima and a special empty-first-probe rule, and route inherited-bank callbacks. Raw table bounds, collision class game meanings and callback semantics remain qualified. No new runtime, research, gameplay route or bank/domain proof.

32 manually inspected Code records credited across the exact routine spans;981records reviewed,580pending. Old scaffold/metadata retained except deliberate review/source updates. Shared motion offsets and known actor strides use field/size constants in reviewed contexts. All source paths validated relative to commentary/source; editor uses that convention for future passes. Exact source backups, changes, credits, checked references and protected hashes: analysis/commentary-catchup-007/.

Classification/ROM/map/state/toolkit unchanged:45,716/65,536(69.76%),Unknown19,820. Final commentary.py verify byte_identical=true65,536bytes and read refresh passed; original hardware include preserved. No final audit or overall acceptance claim. Same full-source commentary phase remains active, with exact pending ranges in analysis/commentary-remaining-inventory.json.

Next: player ground/air/action states18a0..1cb3 and pre-update projectile/collision1cb3..21bd; actor timer+5 is reused for player action timing, so replace the globally over-specific ACTOR_FLASH_TIMER name with a contextual/general timer symbol. Then remaining bank1 and bank2 machinery plus data/unknown boundary notes and older review gaps. No redirect until current-source readability is complete.

Toolkit: existingT001 only; zero new/suspected/missing-feature findings,no blocker. No toolkit edits or runtime. See analysis/toolkit-issues.md.

## Commentary catch-up008 — ground/air/action player states (phase interrupted)

Continued immediately after catch-up007 in the same active commentary phase. Named45 older labels plus four bare routine entries at1908/191c/1b8b/1b98. Added30 functional comments plus four bare-entry explanations, and ACTOR_SUBSTATE. Replaced over-specific ACTOR_FLASH_TIMER with ACTOR_TIMER_05 across source: the same field is an ordinary-actor flash counter and a player-action countdown. These meanings are now contextual rather than a globally misleading name.

Reviewed18a0..1cb3 known Code: ground substate dispatch/idle frame-timer compensation; held Left precedence and facing; fixed-point jump velocity seeding; upper/lateral/support class gates; landing and tile-row alignment; state/substate reset; held Down versus fresh A/B edges; helpers that discard or replace return addresses; saved prior state and timed action substates; partial variant/aux-slot initialization. Input bit5 supports the action/attack interpretation, but raw weapon/table selector domains and unclassified tails remain explicit. Frame timing/game physics units are not invented. The helper at1e6f and action-contact2031 still need their own mechanical review.

35 Code scaffold records credited after full source review of these fragments;1016reviewed/545pending.34compact units added, exact physical window and credits in analysis/commentary-catchup-008/preservation-and-review.json. All source references checked in bounds, immutable scaffold preserved; no reset of older review credit. Exact full pending ranges refreshed in analysis/commentary-remaining-inventory.json. Prior reviewed ranges remain subject to concrete defects at final acceptance.

ROM/map/state/frozen artifacts/configured release remain unchanged. Classification45,716/65,536(69.76%),Unknown19,820; no runtime/research/gameplay/input/new bank-domain claim. Final commentary.py verify ok=true,byte_identical=true across65,536bytes; read refreshed reading-after.asm. Source backup, change manifest and final verification in analysis/commentary-catchup-008/. No final audit, redirect or disassembly; goal stays active and current-source acceptance is incomplete.

Next exact Code scope:1cb3..21bd projectile pre-update, contact/damage and remaining player movement helpers; then2e31..316d object helpers and remaining bank1/bank2 code. Reconcile old vector/startup/fixed-bank helper fragments, remaining Data/Padding/Unknown boundary notes, generic aliases and numeric contextual scratch roles throughout all four banks. Final audit requires every record/unit/source location reviewed, explicit source-quality acceptance plus fresh byte-identical verify/read, separate from incomplete ROM classification. Saved interruption only.

Toolkit: existingT001 only;zero new/suspected/missing-feature findings,no blocker. Timer-name correction was local semantic review, not a toolkit defect. No toolkit changes; report analysis/toolkit-issues.md.

## Commentary catch-up009 — auxiliary contact and transition gates (phase interrupted)

Previous turn made progress; same full-source commentary phase continued. Added51 functional labels, six actor/variant RAM symbols and17 helper contracts. Scope1cb3..1d98 auxiliary action slot;1e6f..2031 ordinary/alternate transition list gates and partial coordinate snap;2031..21bd contact box, target field19 subtraction and effect conversion.33 Code records reviewed;1049reviewed/512pending at this batch. No classification/map/ROM/toolkit/runtime/research/input changes. All protected hashes unchanged.

Source distinguishes the two transition row interpretations, distance/range tests, unchecked source/variant lookups and inherited-bank callbacks. Auxiliary contact stack review corrected the draft: the two POPs remove the immediate CALL return and a saved target pointer, retaining the outer return. Effect D is described as field+3, not invented motion units. Target+19 behaves like a hit budget in this consumer but is not globally called health. Raw suffixes/table domains remain unresolved.

Final byte-identical65536 verify/read refresh passed; saved exact batch verification in commentary-catchup-009/verification.json. No final audit/redirect or successful phase handoff; current-source goal incomplete. Continued directly into the remaining persistent/object helpers in catch-up010. ExistingT001 only;zero new/suspected/missing-feature toolkit findings,no blocker. Report analysis/toolkit-issues.md.

## Commentary catch-up010 — persistent lifecycle/effects and selector correction (phase interrupted)

Continued immediately in the same commentary phase. Added40 functional routine/branch names plus two bare persistent-state entry labels,21 helper comments and four extra reviewed-entry units. Reviewed2e31 prefix,2ea7..3012 persistent lifecycle/contact,3034 variant upgrade,3079..31ee allocation/motion helpers and344f..3505 effect/screen helpers.30 Code records credited;1079reviewed/482pending. Source references validated and immutable scaffold preserved. Exact semantic windows, backups, changes and protected checks: analysis/commentary-catchup-010/.

Material correction: normal2fc9 contact reads actor+1b, not a separate base+9 field. ReadCurrentActorField0b ->INC L(+0c) ->ADD6(+12) ->ADD9(+1b), with no intervening pointer reload. Exact original bytes and static derivation are in commentary-catchup-010/contact-selector-correction.json. Fixed source comments and recorded correction in commentary/uncertainties.md. Historical constructed conversion tests did not execute this consumer and remain valid. Frozen phase1 evidence/history retains its old wording, superseded editorially without altering maps/classification. No new global selector/table domain, natural reachability or execution claim.

Other useful contracts now explicit: CP7 flags at conversion entry; original/converted type sharing+1b; persistent camera interval(lower,upper] differs from placement scan[lower,upper); conditional CBxx flag clearing; effect sounds compare pre-decrement lifetime; camera sprite-X mapping versus right-edge prefix; placement initialization uses incoming bank2 transiently before state0 later installs bank1 animation, with no bank2 sprite-data inference. Placement offset prose now uses consistent hex field offsets.

Final commentary.py verify ok=true,byte_identical=true for65,536bytes; read refreshed reading-after.asm. ROM/current.sym/internal state/frozen comparison artifacts/configured release unchanged;classification45,716/65,536(69.76%),Unknown19,820. No runtime/input/gameplay research or other phase. No final audit or complete claim. Same goal active; saved interruption only.

Next: finish fixed-bank residual helpers0862..0ef9 and reconcile older vector/startup/timer/layout fragment review credit, including regions that straddle already-reviewed routine boundaries. Then bank1/2/3 classified code and remaining Data/Padding/Unknown boundary notes. Exact pending ranges: analysis/commentary-remaining-inventory.json. Previously reviewed records remain subject to source-quality repairs. Final acceptance requires all source meanings/uncertainties/names/fields coherent, every record/unit/source location reviewed, explicit agent quality assessment and final audit/fresh rebuild/read refresh. No redirect while current-source readability remains incomplete.

Toolkit: existingT001 only;zero new/suspected/missing-feature findings,no blocker. The selector error was historical ROM-analysis pointer arithmetic, corrected through source review; not a toolkit defect. No toolkit changes. Persistent report analysis/toolkit-issues.md.

## Commentary catch-up011 — fixed-bank residuals and ledger reconciliation (phase interrupted)

Previous turn made progress; same whole-source commentary phase continued. Named18 remaining generated fixed-bank labels, added five contracts plus unsigned-DE/entry/tile-copy/count comments, and15 CPU table-base constants. bank_000.asm now has zero code_/Call_/Jump_/jr_ label definitions. Known table operands use descriptive bases, with explicit bank1 and unresolved extent qualifications; no raw backing bytes were promoted. This naming result alone is not full source-quality acceptance.

Reviewed stage setup/tile overlay selection,21 actor-retirement slots, vertical-scene reset/clamp prefix, addressed tile-run branches/title row copy, tile count/copy and odd-frame intro-window carry. Explicit edge cases: clamp prefix preserves values below10 rather than saturating; BC tile count expands modulo65536 and zero byte count copies65536; extracted transfer flags are restored into A but unused by CopyTileBytes. Alternate HUD3270 only initializes occupancy/Y in its known prefix; raw selector/draw suffix remains unknown.

Reconciled74 older fixed-bank Code records whose full mechanics were inspected in catchups001..011, including cross-routine fragments. All fixed-bank Code scaffold records now have reviewed source/unit entries; this retains concrete final readability repair obligations (contextual scratch operands, source/data boundary notes and final consistency). Compiled labels from the pinned RGBDS linker provide exact address/source anchors, not semantic evidence. The supplementary symbol link equals the original ROM byte-for-byte. Its .sym/.gb remain local and are not the portable map/deliverable. Source-location bounds and unchanged scaffold/protected hashes pass;1153records reviewed/408pending. Evidence: analysis/commentary-catchup-011/preservation-and-review.json.

Final commentary.py verify ok=true,byte_identical=true65536bytes; read refreshed reading-after.asm. ROM/maps/internal state/frozen baselines/configured release unchanged. Classification45,716/65,536(69.76%),Unknown19,820; no runtime, gameplay/input/research, disassembly or redirect. No final audit/goal acceptance claim. Exact backup/changes/verification in commentary-catchup-011/.

Next: remaining classified Code in bank1(46pending records),bank2(62),bank3(8); use source/verified research to explain every entry and field, preserving finite versus natural reachability. Also pending Data/Padding/Unknown boundaries and known numeric context/scratch gaps throughout the source. Exact pending ranges remain in analysis/commentary-remaining-inventory.json. Full acceptance requires every region/unit and source location reviewed plus explicit source-quality approval and final audit/fresh rebuild/read refresh. Same commentary phase active; saved interruption only.

Toolkit: existingT001 only;zero new/suspected/missing-feature findings,no blocker. No toolkit edits/build substitution. Supplementary compiled symbols are local source bookkeeping, not toolkit changes. Report analysis/toolkit-issues.md.

## Commentary catch-up012 — bank3 continue/display/high-score Code (phase interrupted)

Continued immediately after011. Named12 bank3 generated labels, added four established RAM symbols plus context-specific c601 aliases, five display/score contracts and rewritten continue-control comments. Source now uses named high-score, REST, displayed-stage and continue-count fields; c601 is explicitly shared by continue choice and column-priming count. Known shared stream suffixes use symbols+offsets. bank0/bank3 have zero generated code_/Call_/Jump_/jr_ definitions.

Bank3 Code now explains fresh Up/Down/A ordering, Down precedence, retry state/reset effects and the unguarded continue decrement; stage/REST packed-byte display; eight score nibbles without decimal validation; unsigned high-byte-first score comparison; flag-preserving sixteen-bit pointer stepping and increasing-address four-byte copy. No normalized-BCD assumption was made for the score high byte. Eight pending Code records credited;1161reviewed/400pending. Earlier continue review retained, source anchors relocated and all references checked. Full scaffold remains unchanged.

Final commentary.py verify ok=true,byte_identical=true65,536bytes; read refreshed reading-after.asm. Exact source backup/change/verification and protected checks in analysis/commentary-catchup-012/. ROM/maps/state/frozen artifacts/configured release unchanged;classification45,716/65,536(69.76%),Unknown19,820. No emulator/research/input/disassembly/redirect or final audit. Same full-source goal active, not accepted.

Next: bank1's46 pending Code records and bank2's62, then pending39 Data/13 Padding/240 Unknown scaffold entries and concrete source-quality gaps in prior reviewed material. Those raw unknown regions need honest boundaries/uncertainty, not new classification or speculative decoding. Also reconcile contextual scratch names and common MBC/bank-ID/OAM operands throughout. Exact pending ranges in analysis/commentary-remaining-inventory.json. Finish only after all meaningful source blocks/fields/comments and inventory units are reviewed, explicit source-quality assessment, final audit and fresh byte-identical rebuild/read refresh. Saved interruption only; no handoff to redirect.

Toolkit: existingT001 only;zero new/suspected/missing-feature findings,no blocker. No toolkit changes or alternative release. See analysis/toolkit-issues.md.

## Commentary catch-up013 — remaining bank1 Code (phase interrupted)

Previous turn was progress; same whole-source commentary phase continued. Named67 generated routine/branch labels, added two bare state entry labels and four slot/player RAM aliases, wrote36 helper contracts and two bare-entry explanations. Removed34 redundant R16/R17 analysis-code aliases only where adjacent descriptive labels exist; callers now use the descriptive symbols. bank1 now has no generated code_/Call_/Jump_/jr_/R*_code_ definitions. Source pointer/field operands retain supported context and raw extents.

Reviewed all46 pending bank1 Code records: placement frame/transition gates and signed-near-coordinate scans; slot strides/strict equality behavior and prime pass; type4 initialization/countdown (33ae+2*4=5935 checked against exact ROM bytes); cycling child spawner; unmasked stage setup/update prefixes; terrain turn/facing; carrier movement prefixes; three-cell class2 cache/tile clearing; player contact scan, eligibility, stack bypass, amount scaling and budget/recoil prefixes. Every accepted fragment is described mechanically even when enemy/weapon meaning remains unresolved. Existing finite first-state observations remain finite and do not establish natural selection or later state closure.

Useful edge contracts: spawner+1c can wrap FF->0 rather than globally cycle1..4; positive child-coordinate branch uses ADD0 for the high byte and does not propagate low-byte carry; direct child allocation builds a four-byte record into a preselected free slot; c3d0..d7 is preserved by the class-cell clear but c3c8..cf is not; the first player hit discards its immediate scan return; recoil's jump initializer advances the reset substate to1. Initial naming/type and final substate wording were checked before final source verification. No behavior fixes or new classification followed.

All bank1 Code scaffold records now reviewed, with compact units, exact pinned-link label anchors and checked source locations.1207records reviewed/354pending. This is review progress, not full-source acceptance: contextual scratch/constants and Data/Unknown boundary notes plus final source consistency remain obligations even in earlier reviewed records. Exact windows/credits/protected checks and source backups: analysis/commentary-catchup-013/.

Final commentary.py verify ok=true,byte_identical=true65,536bytes; read refreshed reading-after.asm. Supplementary pinned-symbol link equals the ROM and stays local. ROM/current.sym/internal state/frozen artifacts/configured release unchanged;classification45,716/65,536(69.76%),Unknown19,820. No runtime/research/input/disassembly/redirect or final audit. Same goal active; saved interruption only.

Next: bank2's62 pending Code records(audio frame/channel/event machinery), then39 Data/13 Padding/240 Unknown records. Finish contextual scratch/MBC/bank-ID/OAM names and replace redundant data-address aliases with their already-established descriptive symbols, preserving sound material. Final acceptance requires all source/unit/reference/inventory checks and explicit source-quality assessment plus final audit/fresh rebuild/read refresh. Exact pending ranges: analysis/commentary-remaining-inventory.json. No redirect while current-source readability remains incomplete.

Toolkit: existingT001 only;zero new/suspected/missing-feature findings,no blocker. No runtime/toolkit edits or alternate release. Local type-word/recoil wording checks were source review, not toolkit defects. Report analysis/toolkit-issues.md.

## Commentary catch-up014 — full classified audio Code (phase interrupted, 2026-10-18)

Previous turn made verified progress. Reviewed bank2 audio Code through CPU481d, adding114 functional routine/branch names, three bare entry labels and60 audio working/global RAM aliases. Added49 contracts plus expanded parser/output/install documentation, five bank2 ROM-base constants and explicit hardware register operands. Bank0 wrappers now call the known bank2 init/tick/mute/restore symbols. All62 pending bank2 Code records credited;1269reviewed/292pending. All classified Code scaffold records across four banks now reviewed, while full-source quality/inventory acceptance remains pending.

Source documents six37-byte saved records copied into/out of a shared C100 work area; pause and half-rate/tempo gates that differ across logical channels; command0/priority/header install, routing side effects before priority rejection and partial field reset; note/rest durations and gates; single saved event return, FB marker/FE counter, separate pitch sequence controls; signed/modulo pitch delta and sticky negative flag; tone/noise/wave duty/envelope distinctions; shadowed register writes/arbitration; wave RAM copy; exact reset ranges. Reserved work offsets19/24 (hex) remain unresolved. The pause wrapper marker overlaps copied work offset24 rather than an independent persistent latch. The reset ends C24B, so half-rate phase C24D is retained. Timing names are driver ticks, not calibrated playback durations.

Editorial corrections carried into assembly: EA3x writes duty/wave parameter through register1 or wave RAM, NOT the envelope register2; EA4x changes routing-swap flag1. Updated the previously reviewed four-byte EA handler-word editorial summary while preserving frozen phase1 evidence. Pitch FE checks equality before special FF handling; FF returns without clearing an old delta; a clear sign input does not clear the old negative flag. ReadIndexedAudioWord doubles modulo256 rather than preserving the ninth bit. Reset's HeaderOldLicenseeCode operand was a coincidental $014b byte count, now AUDIO_RESET_COPY_BYTES. No raw handler/table gaps or wider selector/channel combinations were classified from these interpretations.

Source/ledger/scaffold/compiled-address checks pass. A reconciliation check initially found a Code record spanning a semantic-window boundary (862a..8672); the same already-inspected combined window was recorded, without changing the record/classification. Exact source backups, changes, manual windows, credits, checked references and protected hashes: analysis/commentary-catchup-014/. Supplementary symbol link equals original65536bytes and remains local.

Final commentary.py verify ok=true,byte_identical=true65,536bytes; read refreshed reading-after.asm. ROM/current.sym/state/frozen artifacts/configured release unchanged;classification45,716/65,536(69.76%),Unknown19,820. No emulator/research/input/disassembly/redirect or final audit. Same source-readability goal active; saved interruption only.

Next: remaining39 Data/13 Padding/240 Unknown scaffold records and concrete earlier-review source gaps. Add exact boundary/role/uncertainty notes without reclassifying unknown bytes; check mnemonic-form fill/unknowns honestly rather than treating emission as evidence. Finish context-specific scratch aliases, common MBC/bank-ID/OAM constants and redundant descriptive data aliases, then explicitly assess every source block/field/prose and reconcile all references/units. Final acceptance requires the full inventory, source-quality assessment and final audit/fresh rebuild/read refresh, separate from incomplete ROM classification. Exact pending ranges in analysis/commentary-remaining-inventory.json.

Toolkit: existingT001 only;zero new/suspected/missing-feature findings,no blocker. Header-count naming, duty interpretation and reconciliation boundary matters were editorial/local, not toolkit defects. No toolkit edits/rebuild/substitution. Persistent report analysis/toolkit-issues.md.

## Commentary catch-up015 — data/padding/unknown boundary review (phase interrupted)

Previous turn made progress. Reviewed the remaining39 Data,13 Padding and240 Unknown scaffold records without changing a single classification or map byte. All1561 records now have reviewed summaries/source locations/units. This is inventory coverage only: source-quality repairs and final acceptance remain pending.

Built a pinned-RGBDS label-only source index on a private copy, proving byte-identical65536 output and exact source-line boundaries. All292 pending start/end boundaries align with emitting lines. Added precise physical/CPU/effective-bank notes and labels for every unknown gap, explicitly denying Data/execution evidence from raw emission. Known consumer bases and already-classified handler words give conditional mechanical context; target pointers are not execution proof. The eight FF bytes at unused restart38 were mnemonics inside an existing Padding region; now emitted as fill, retaining bytes and classification. No unknown instructions were decoded or promoted.

Data review explains header fields, ten named I/O pairs including written IE terminator, six symbolic inline handler tables and their unproved wider selector domains, per-bank identity bytes, and exact planar tile slice counts/row format/overlapping-transfer boundaries without invented artwork identities. Inline DB-to-DW edits match existing proven words and compiled targets; no pointer-only Code inference. Header/checksum bytes retained without rgbfix. Padding notes distinguish vector alignment from bank tails.

Both maintained-source verify/read and after-edit label-only index rebuild passed byte-identical65536. All240 unknown labels match physical offsets.3453source references, positive unit ranges, immutable1561-region/phase1 metadata and protected ROM/map/state/frozen/release hashes checked. Unknown remains19,820bytes;classification45,716/65,536(69.76%). Exact index/scripts/notes/backups/checks/verification: analysis/commentary-catchup-015/. A first local metadata assertion expected phase1 on Unknown scaffold entries; corrected to preserve its optional absence. This was local validation schema handling, not toolkit behavior.

Next source-quality work: contextual scratch aliases and common MBC/bank-ID/OAM/fixed fields, redundant data-address aliases, numeric field/count consistency and prose contradictions even in already-reviewed ranges.386numeric absolute RAM LD operands remain across bank0/1, mostly reused scratch roles; bank2/3 have none. Final acceptance requires explicit agent review of these gaps plus final audit/fresh rebuild/read refresh. Updated remaining inventory has no pending regions but lists this source-quality backlog. Same full-source goal active; no final audit, goal completion, redirect or disassembly claimed.

Toolkit: existingT001 only;zero new/suspected/missing-feature findings,no blocker. No runtime/research/input or toolkit changes; configured release unchanged. Report analysis/toolkit-issues.md.

## Commentary catch-up016 — current-source editorial acceptance (2026-10-18)

Completed the existing classified-source readability scope across all four banks, reusing the preserved routine/structure reviews from catch-up001–015 and resolving their final consistency backlog. This closes commentary for the current source only. The main goal remains paused; no redirect, emulator experiment, research scenario/input, map annotation, classification change or new disassembly pass was performed.

Shared WRAM now distinguishes persistent state from contextual scratch lifetimes: input/HUD/render, placement, tile/class probes, contact extents, terrain results and transition coordinates. Added common mapper, bank-ID, OAM, stack and understood fixed-field names. All four banks have zero raw absolute C000..DFFF bracket operands and zero numeric direct CALL/JP/JR targets. Remaining numeric actor fields are context-dependent offsets with mechanical explanations, not claims of globally shared meaning. ROMX symbols remain CPU addresses subject to the documented inherited-bank contract.

Removed204 redundant address-only data aliases, named33 previously unpaired data entries, then replaced the remaining368 round-prefixed aliases with structural names or their co-located descriptive names. Named the synthetic early-state input continuation at0266 and six established pointer consumers, retaining Unknown backing/classification where applicable. The source explains interrupt preservation/reentrancy, state dispatch, common copy/arithmetic/display helpers, actor/player/terrain/contact/transition behavior, placement formats, audio parser/register/record behavior and tile/layout formats. Corrected stale timer glyph, single-screen nested13 layout and descriptor reload prose. Current uncertainty notes explicitly supersede historical pending-commentary counts.

Agent editorial assessment: all currently classified routines, meaningful branches and data structures now have useful functional/mechanical explanations, labels and consistent known variable/field symbols. Sound prior reviews were retained rather than cosmetically rewritten. Unknown game purpose remains stated near its consumer. Every Unknown gap has physical/CPU/bank boundary notes; raw emission and pointer targets remain neither classification nor execution evidence. This assessment is separate from automated checks and from incomplete ROM classification.

Final checks: configured-toolkit commentary.py audit passed ok=true, byte_identical=true for65,536bytes, SHA256 edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Audit accepted1561regions/1643units and refreshed reading-after.asm. Private pinned-RGBDS label-only index also rebuilt identically with10456emitting lines spanning the full ROM. Checked3453source references, all unit ranges, exact Unknown label addresses and0266 continuation. Original ROM, maps/current.sym, analysis/state.json, frozen baseline/state and configured release hashes remain unchanged;1561-region scaffold and frozen phase1 metadata unchanged. Classification remains45,716/65,536bytes(69.76%);Unknown19,820bytes. Source metrics and evidence: analysis/commentary-catchup-016/preservation-and-review.json; exact source hashes and final build: audit.json/verification.json there and in commentary/.

Remaining limitations: structural Unknown19,820bytes; unchecked global selector/command domains; unresolved actor-type field purposes; inherited upper-bank contexts; natural later-state/transition/audio reachability; individual artwork identities. These are explained uncertainties, not a current-source naming/comment backlog. No remaining current classified-source catch-up scope. The next catch-up should cover newly accepted classification or concrete new evidence that changes existing comments, with the same byte-identity/map-preservation checks. Stop here before redirect or further disassembly.

Toolkit: existingT001(snapshot held-input restoration) only;zero new/suspected/missing-feature findings,no blocker. No toolkit edits, rebuild, substitution or runtime/snapshot use. Persistent report: analysis/toolkit-issues.md.

## Round017 — redirect complete (2026-10-18)

Input/output structural checkpoint round-017-disassembly; maps/current.sym SHA256 db63b6031c819c689a80c4848d634b0606e1b69ee4dc83c7a35ec9501caf3edf. Authoritative current-source gate catch-up016 accepted1561regions/1643units; independently matched all seven source hashes to commentary/audit.json. Appended historical round017-commentary handoff with this gate update, preserving its partial/interrupted history.

Investigated supported deferred25/33/45/46 as a connected parent->initializer->parser->actual child state->bounds/class/tilemap->animation batch.56 accepted constructed actor calls (4initializer,31parser/initialchild,18post-commandchild,3bounds), max770operations/caps1000or1500, all c300/SPdff2, zero detailed/aggregate drops.22 independent record/full-pool assertions passed.26 renderer cases cover13nodes*timer1/2,max839/cap1000; exact OAM and pointer assertions passed. This is finite artificial-state evidence, not natural placement/gameplay or global allocator/selector closure. Existing sourced gameplay routes preserved.

Added ready R17-CONNECTED-PARENT-CHILD-HANDLERS (**972 actual-fetched Observed Code bytes**, independently decoded), R17-FINITE-CHILD-STRUCTURES-SPRITES (**375 Static Data bytes**,22 sparse ranges). Both belong in one next disassembly batch (1347bytes). Code graph577starts/972bytes and exact roles in analysis/redirect/round-017/verification.json/graph.json; no candidate annotated. Type33 directory neighbors and speed gaps remain Unknown. Two preliminary forced-current-FF variant tests excluded as outside produced state contract, retained in out-of-contract-tests.json; preceding valid rows prove FF. Local14-component renderer observation length corrected40->80; final26 pass.034d draft corrected to ExchangeHLAndDE, not RNG. Prior preexisting round017 redirect inspection artifacts preserved.

ROM still **45,716/65,536 classified (69.76%),19,820 Unknown**, separate from fidelity. map.validate ok=true; final-audit.json confirms source/ROM/state/map/release hashes unchanged and commentary audit still matches. Existing byte-identical65536 structural/commentary checks retained; no new checkpoint/rebuild for unchanged map/source. Current phase redirect finished, not interrupted; next disassembly validates both new leads together. Compact handoff analysis/round-017-redirect-handoff.md; exact evidence, recipes, remaining missing prerequisites and diminishing-prospect deferrals in analysis/redirect/round-017/findings.md. All previous lead IDs/history retained.

Toolkit: existing confirmedT001 only;zero new/suspected/missing-feature findings,no blocker. No toolkit edits/rebuild/substitution. See analysis/toolkit-issues.md.

## Round018 — connected finite parent/child batch; disassembly pass finished

ROM classification: 71.81% (47063 / 65536 bytes classified; 18473 bytes unknown).

Input round-017-disassembly, accepted commentary catch-up016 and complete redirect017; output round-018-disassembly. Both R17-CONNECTED-PARENT-CHILD-HANDLERS and R17-FINITE-CHILD-STRUCTURES-SPRITES verified together. Added1347bytes:972Observed Code in8 intervals,375Static Data in22 sparse intervals;69 role-specific new map regions. All1321 prior region objects/metadata and unrelated internal fields preserved exactly;1390/1390 resulting regions have role/evidence. No explicit Unknown regions existed to split: additions occupy unrepresented gaps; neighboring old regions remain exact. Prior source/commentary/retractions/history retained.

Independent verification decoded every historical fetched start through the configured server, checked actual opcode bytes and per-event physical/CPU/bank aliases, zero detailed/aggregate drops and SPdff2 returns; matched the saved exact code/role proof. Rewalked literal animation IDs0/56/61/76/77 through13 closed FE/FF/zero nodes and sparse payloads, independently derived finite counts2/3/4, six/three-byte records, four/five-byte commands and single-byteFF, dispatch/pointer words, all four class masks and only speed0/4. All972newCode bytes are fetched instructions, including bounds/nonlocal-POP and class/tilemap continuations; inline dispatch barriers77be..77c2 and78d1..78d7 remain Data. Sentinel padding and intervening speed/directory gaps remain Unknown. Historical56 constructed actor calls(max770,caps1000/1500),22 independent full-pool/record assertions and26 renderer cases(max839/cap1000) reused without unchanged replay. Pointer values/reached targets are not execution evidence; debugger construction does not prove natural selection/timing or general allocator safety. Excluded forced-current-FF tests remain excluded.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8316 | 4986 | 73 | 13375 | 3009 |
| 01 | 2740 | 3194 | 67 | 6001 | 10383 |
| 02 | 1946 | 9496 | 37 | 11479 | 4905 |
| 03 | 227 | 15292 | 689 | 16208 | 176 |

Totals Code13229/Data32968/Padding866; execution coverage is separate historical finite evidence. [Metrics](round-018-classification.json), [unknown complement](round-018-unknown-ranges.json), [independent proof](round-018evidence/verified.json), [inspection and mutation](round-018evidence/mutation-rpc.json), [preservation](round-018evidence/preservation-check.json). Sequential map.validate(ok), map.save analysis/state.json, stock map.export maps/current.sym then fresh python3 run.py checkpoint round-018-disassembly passed validation/generation/byte_identical=true for65536bytes. Map SHA256c6d732d75b1a374d1c85e6f9ff9f205f9bfdf47e10389aefcdcf6f98d39cabff. [Verification](round-018-disassembly-verification.json); stock map maps/round-018-disassembly.sym; local bank-organized source generated/round-018-disassembly.

Supported connected continuations are included through parent parsers, child49states0/1 and child37states0/1/2, bounds and class/tilemap consumer. No additional obvious supported unknown interval emerged: neighboring type33 variants and speed entries lack selected source domains; live allocator pressure/natural scheduling requires scene invariants; older dynamic45d6/1b7b/2134/2f7e, stage3flags, later/nonmonotonic selections and audio8aa9 remain distinct missing prerequisites. Redirect017 already investigated these reasonably; no improved natural route or new gameplay condition justified repeating stalled scenarios. Existing sourced gameplay/scenario routes and exact artificial patches remain valid, no new runtime/snapshot/research. One disassembly pass complete; ROM incomplete. Next coordinator phase commentary, preserving accepted source and completing new-region readability; no commentary/redirect performed here.

Toolkit: existing one confirmedT001; zero new/suspected/missing-feature findings, no blocker; no toolkit edits/rebuild/substitution. [Persistent report](toolkit-issues.md).


## Round018 commentary — complete current-source gate (2026-10-18)

Input/output structural checkpoint round-018-disassembly unchanged; map c6d732d75b1a374d1c85e6f9ff9f205f9bfdf47e10389aefcdcf6f98d39cabff.
ROM classification71.81%(47063/65536),Unknown18473 remains separate from readability.
Preserved entire prior accepted commentary at analysis/round-018/commentary-prior
before fresh preparation. Reused1547exact range/kind reviews and all useful source,
replaced14affected old Unknown extents, reviewed91new/split records. Current accepted
scope1638regions/1720units;3607source locations reconciled by verified physical index.
Banks0/2/3 exactly equal accepted prior files; all current scaffold/phase1 annotations
unchanged. New actor25/33/45/46 parents, child37/49 states, bounds/nonlocal stack path,
class/tile writer and sparse initialization/commands/sprite data have62contracts,
113functional alias replacements and context-specific field/scratch/constants.
Affected wrappers/mask consumers updated. No current classified-source naming/comment
backlog; honest mechanical uncertainty remains for gameplay ownership/natural
scheduling, full-pool allocation safety, global domains and artwork identities.

Agent assessment and exact preservation/inventory evidence:
[review](round-018/review-assessment.json). Final commentary.py audit passed ok=true,
byte_identical=true65536,1638regions/1720units, reading-after.asm refreshed;
[audit](round-018/audit.json),[verification](round-018/verification.json).
ROM/map/state/configured release hashes unchanged; no emulator/input/ROM/map/toolkit
or classification changes. Full current-source acceptance does not complete the ROM.

New concrete evidence correction: type33 stream7a04 row speeds0,0,0,0,4,3,2,1 and
saved actual child read pairs prove the prior redirect017 'only0/4' prose wrong.
Six step bytes7a7a..7a7f remain Unknown in this map; exact rows/reads/values/consumer
and bounded reconciliation recipe saved as [follow-up](round-018/speed-step-followup.json).
This is a local evidence omission, not a toolkit defect or new natural route.
No disassembly or redirect was performed. Next sequential phase redirect should
reconcile this omission and investigate other useful continuations; no unchanged
failed route replay is prescribed. [Full accepted handoff](round-018-commentary-handoff.md).

Toolkit:existingT001 only;zero new/suspected/missing-feature findings,no blocker.
No toolkit edit/rebuild/substitution or snapshot use. [Report](toolkit-issues.md).

## Round018 redirect — COMPLETE; coherent next batch ready

Input/output structural checkpoint round-018-disassembly, map
c6d732d75b1a374d1c85e6f9ff9f205f9bfdf47e10389aefcdcf6f98d39cabff;
classification remains47,063/65,536 (71.81%),18,473Unknown. Current-source
commentary gate independently matched all seven source hashes and accepted
1638regions/1720units. No classification/map/source/toolkit changes.

Independently corrected redirect017's speed-domain omission: finite source rows
and historical actualReads support speeds0..4; six7a7a..7a7f bytes were omitted.
Preserved historical verified375-byte lead and appended explicit correction.
Did not stop at this6-byte lead: followed accepted26/27/31 state1 through
companion copying, bounded five-slot type7 allocation, all three child states,
periodic16-byte sequence, literal animation28 and audio26 finite stream. Also
verified literal persistent-motion31ee8-byte copy from reviewed source.

Ready leads R18-CLASS-CHILD-SPEED123 (6Data),
R18-COMPANION-SPAWN-TYPE7 (657Code+180Data),
R18-PERSISTENT-MOTION-SEED (8Data):851bytes total,657Code(478Observed,
179Static)+194Data. No byte classified in redirect. Static control-flow roots,
inline barriers, indirect-store contracts and per-start provenance retained;
no requirement/claim every branch fetched.7constructed actor calls (cap1800,
max1146),20renderer cases(cap1000),16audio calls(loadercap1000/events500,
max461); actor/audio zero detailed/aggregate drops. All30unknown sound26
bytes actually read; slot/header/parser outputs match. All restores release8buttons.
Original reused renderer oracle reset odd-tile attributes incorrectly; corrected
against reviewed emitter's reuse contract; failed RPC/tests retained separately.
No natural scheduling, later-stage, global-domain, ownership or audio timing claim.

Map.validate passed; protected ROM/state/map/release hashes match round018
protected.json and all accepted sources match commentary audit. Existing
byte-identical65536 checkpoint/commentary checks remain valid for unchanged
artifacts; no redundant new checkpoint claimed. Finite connected investigation
complete, no productive frontier interrupted. Other actor families and natural
ownership/timing/global directories remain independent future questions; exact
deferral reasons in findings. Next coordinator phase: disassembly019 with all
three ready leads; classification and current-source readability gates remain.

[Handoff](round-018-redirect-handoff.md),[leads](leads.json),
[findings](redirect/round-018/findings.md),
[exact ranges/roles](redirect/round-018/verification.json),
[final audit](redirect/round-018/final-audit.json).
Toolkit:existing confirmedT001 only;zero new/suspected/missing-feature findings,
no blocker;no toolkit edits/rebuild/substitution.[Report](toolkit-issues.md).

## Round019 — disassembly complete; ROM incomplete

ROM classification: 73.11% (47914 / 65536 bytes classified; 17622 bytes unknown).
Input round-018-disassembly; output round-019-disassembly. Verified all three ready
R18 leads: R18-CLASS-CHILD-SPEED123, R18-COMPANION-SPAWN-TYPE7,
R18-PERSISTENT-MOTION-SEED. Added851bytes:657Code(478Observed/179Static),194StaticData.
Independent configured-toolkit decode/structure equality across353 instruction starts,
ROM-based sparse animation/sequence/speed/motion/audio role reconstruction and
historical zero-drop actor Instruction/child Read audit agree with redirect proposals.
Seven actor calls,20renderer cases,16audio calls reused; no new runtime/snapshot or
natural route. Exact recipes remain redirect/round-018/probe.py,consumer.py,audio-probe.py
and scenarios.json. Static code is not reported executed. Preserve736f..7374 inline
barrier; actor fixed0/switch1, bank2 audio CPU=physical-4000. Speed1/2/3 finite
row domain is corrected; no speed5/adjacent selector inference. Animation28 closes
8 payload frames plus jump; audio26 closes20events/24stream bytes toFF. Data roles
and exact additions: [independent proof](round-019evidence/verified.json),
[inspection RPC](round-019evidence/inspection-rpc.json).

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|8316|4996|73|13385|2999|
|01|3397|3348|67|6812|9572|
|02|1946|9526|37|11509|4875|
|03|227|15292|689|16208|176|

Totals Code13886/Data33162/Padding866. All1456regions have role/evidence metadata;
all1390prior region objects/metadata and unrelated fields preserved exactly. Existing
commentary/source unchanged: no readability credit claimed for66new structural
regions. Independent execution metric for this batch478bytes only; historical
execution coverage remains separate. Unknown ranges: round-019-unknown-ranges.json.
Sequential map.validate/map.save/map.export passed; fresh `python3 run.py checkpoint
round-019-disassembly` passed validation,generation and byte_identical=true65536.
Stock map maps/current.sym and maps/round-019-disassembly.sym SHA256
8ffcd2909a343dc6767c8ab4a28a59eba6f242fca7d1608151d4b3047c2d0042.
Generated bank source generated/round-019-disassembly; report
round-019-disassembly-verification.json; [final audit](round-019evidence/final-verification.json).
ROM/configured release and all accepted commentary source files unchanged.

Easy gains in this connected batch are exhausted: remaining parent/state transitions
reach existing helpers/known nodes; wider7fa4 selector domain needs other writers,
natural companion ownership/contact/allocation needs scene prerequisites. Prior
unchanged stalled natural routes were not replayed. Independent actor families
are redirect questions, not unsupported adjacency gains. Next phase commentary:
carry accepted018 source and finish readability for these new regions, then redirect
may investigate other actor wrappers' later states with concrete writer prerequisites.
No commentary or redirect executed in this worker.
Toolkit: existing confirmedT001 only;zero new/suspected/missing-feature findings,
no blocker,no edits/build/substitution. [Report](toolkit-issues.md).


## Round019 — commentary complete, ROM incomplete
Classification remains73.11%(47914/65536),17622Unknown; input/output structural
checkpoint round-019-disassembly, current map/state unchanged. Entire prior accepted
commentary preserved before fresh preparation;1622exact reviews carried,80new/split
records reviewed. All1702regions/1784units accepted with zero readability backlog.
Agent assessment covers exact new companion/spawn/type7/animation28/audio26/seed
windows and affected prior wrappers/speed consumers; historical speed correction
retained and newly classified steps1..3 reconciled. Unknown natural ownership,
scheduling/contact/timing and global selectors remain explicitly mechanical.
Fresh final audit passed byte_identical=true65536,editorial_audit=true and reading
copy refreshed. No new runtime/snapshot/input/disassembly/redirect. Protected
ROM/state/map/configured release hashes unchanged. Toolkit existingT001 only;
zero new findings,no blocker; report[toolkit-issues.md](toolkit-issues.md).
[Compact handoff](round-019-commentary-handoff.md),[audit](round-019/audit.json),
[source-quality assessment](round-019/review-assessment.json),
[preservation](round-019/final-preservation.json). Next phase redirect.

## Round019 redirect — complete, ready connected type6/type8 batch
Input/output structural checkpoint round-019-disassembly unchanged. Commentary019
gate accepted1702regions/1784units; zero current readability backlog. Classification
unchanged **73.11% (47914/65536;17622Unknown)**. No map/source/ROM/toolkit edits.

NewR19-TYPE6-TYPE8-LATER-STATES andR19-TYPE6-TYPE8-ASSETS ready: source-produced
state1 from accepted6/8 initializers connects finite6 states1..4 and8 states1..8,
shared terrain/countdown/facing helpers,animation35/36/41/42 andsound12. Exact
proposal495bytes=401Code(377Observed/24Static)+94Data,11sparse Data/5Code ranges.
Static graph208instruction starts/456bytes preserves excluded inline/raw boundaries.
34bounded actor calls(max923/cap1800),22renderer cases(cap1000),6audio calls
(max234;caps1000/500),18output assertions;zero actor/audio detailed/aggregate
drops. Artificial terrain cache00/55 and fields establish local contracts only;
no natural timing/placement/ownership/artwork/audio reachability inference.

Fresh map.validate passes and protected ROM/state/map/configured release plus
accepted019source hashes equal. Accepted019 byte-identical65536/editorial audit
remains applicable;no unchanged-source checkpoint replay. Adjacent5c2b/random
setup needs its own caller prerequisite;other actor machines independent. Selected
connected graph/assets closed,no productive interruption. Coordinator next:
round020disassembly,validate/classify these two ready leads together.
[Handoff](round-019-redirect-handoff.md),[evidence](redirect/round-019/findings.md),
[verification](redirect/round-019/verification.json),[final audit](redirect/round-019/final-audit.json),
[leads](leads.json). Toolkit existing confirmedT001 only;0new/suspected/missing
features,no blocker;explicit8button releases used after all restores.
[Report](toolkit-issues.md).


## Round020 — disassembly complete; ROM incomplete

ROM classification: 73.87% (48409 / 65536 bytes classified; 17127 bytes unknown).
Input round-019-disassembly; output round-020-disassembly. Verified both ready
R19-TYPE6-TYPE8-LATER-STATES and R19-TYPE6-TYPE8-ASSETS. Added495bytes:
401Code(377Observed/24Static),94StaticData. Independently checked configured-toolkit
decode/structure equality for208 starts, graph successor closure, current-actor WRAM
contracts for21 indirect stores, exact state words and finite animation35/36/41/42
roles. Sound12 directory/header/channel1 E0 width and FF stop independently reconstructed;
actual bank2 loader/parser Reads cover all14new audio bytes. Exact sparse roles and
proof: [verified](round-020evidence/verified.json),[RPC](round-020evidence/inspection-rpc.json).

Reused34 actor calls(cap1800,max923),22renderer cases and6audio calls,18output
contracts;historical actor/audio detailed and aggregate drops0. No new runtime,
snapshot or natural gameplay route. T001 explicit8button release recipes remain
in redirect/round-019/probe.py,consumer.py,audio-probe.py and scenarios.json.
Physical actor=CPU bank1; sound wrapper maps bank2 with CPU=physical-4000.
Static24bytes remain Static; executed377bytes is a batch metric,not total coverage.
Artificial terrain caches/contact/marker/timer fields prove local mechanics only.

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|8316|4996|73|13385|2999|
|01|3798|3428|67|7293|9091|
|02|1946|9540|37|11523|4861|
|03|227|15292|689|16208|176|

Totals14287Code/33256Data/866Padding;1489regions all have role/evidence.
All1456prior region objects/metadata and unrelated fields preserved exactly;
33new structural regions await commentary. Existing accepted019commentary source
unchanged;1702regions/1784units prior review credit must be carried. No new source
readability credit claimed. Unknown complement: round-020-unknown-ranges.json.
Sequential map.validate/save/export passed; fresh `python3 run.py checkpoint
round-020-disassembly` validated, generated and verified byte_identical=true65536.
Stock maps/current.sym and maps/round-020-disassembly.sym hash
b82a9709f56fa8649eceab6fe60c4eff45a4c8e8b470e77d1ba37fa6eca31a2c.
[Verification](round-020-disassembly-verification.json),
[final audit](round-020evidence/final-verification.json),
[preservation](round-020evidence/preservation-check.json).
Configured release,ROM and accepted source hashes unchanged.

Connected easy gains exhausted: type6state1..4/type8state1..8 domains and selected
assets close through known helpers. Inspected adjacent5c2b continuation/random
inline table: no selected state/call enters it, so no adjacency classification.
Other actor wrappers require separate source-produced state-domain investigation.
Natural later-state/contact/timing and old stage3flag questions lack a materially
new route prerequisite; prior failed routes not repeated. Redirect may investigate
another accepted wrapper's later states after commentary. No productive connected
work interrupted. Next phase commentary: carry accepted019source/review credit,
explain33new regions with consistent symbols, fresh byte-identical audit and reading
refresh; no commentary or redirect performed here.

Toolkit: existing confirmedT001 only;zero new/suspected/missing-feature findings,
no blocker,no toolkit modifications/build/substitution. Local verification helper
schema/alias corrections were orchestration errors,not toolkit defects.
[Persistent report](toolkit-issues.md).

## Round020 — commentary complete; ROM incomplete
Input/output structural checkpoint round-020-disassembly. ROM classification:
73.87% (48409/65536bytes classified;17127Unknown), unchanged. Full accepted019 tree
archived under analysis/round-020/commentary-prior before preparation. Carried1686
exact scaffold reviews/useful prose by physical range and dependencies; finished42
new/split records,1728regions/1810units reviewed.3787 source references reconciled;
zero raw WRAM operands/numeric direct control targets/round-prefixed definitions.
Agent assessed whole current-source readability using accepted scope, exact new
code/data windows and affected dependencies; zero naming/comment backlog/deferred
range. Source explains type6/type8 finite terrain/latch/marker/countdown/motion
machines, shared helpers, sparse animation35/36/41/42 additions and sound12.
Historical prose that type8 clears+07 is contradicted by exact instructions: it
leaves+07 untouched and writes magnitude0200 at+08/+09. No structural correction
or natural gameplay claim follows. All Unknown extents have explicit boundary notes.
Natural timing/selection/artwork/global domains remain explained uncertainties.

Fresh `python3 /root/romatlas/scripts/commentary.py verify /root/castlevania-adventure`
passed ok=true,byte_identical=true for65536bytes; `read` refreshed reading-after.asm.
No final archive audit command invoked; current-scope review completeness and
fidelity checks are separate from final acceptance. Frozen phase1 scaffold/state/map
and archived accepted019source hashes match. No runtime,input,snapshot,disassembly,
redirect or toolkit modification. [Assessment](round-020/review-assessment.json),
[verification](round-020/verification.json),[preservation](round-020/final-preservation.json),
[handoff](round-020-commentary-handoff.md). Next phase redirect may investigate other
accepted wrappers with new source-produced state evidence; no commentary work is
interrupted. Toolkit:existing confirmedT001 only;zero new/suspected/missing-feature
findings,no blocker. [Persistent report](toolkit-issues.md).

## Round020 — redirect in progress (same-phase research frontier)
Input/output structural checkpoint remains round-020-disassembly; commentary gate
round-020-commentary-handoff.md accepted. Classification unchanged:48409/65536
(73.87%),17127Unknown. No map/source/ROM/release edits. Static investigation has
followed occupancy40 and sound3 finite later states, type4reset, proximity38/39,
spawner subtraction, direct accepted converters41/42/43 and connected7/15 behavior,
type10->9 and type32 state1. Current provisional evidence:902UnknownCode plus
551UnknownData (not classified) in redirect/round-020/verification.json; graph,
source consumers and finite animation/audio grammar retained. Sixteen initial actor
calls,60initial renderer cases,81audio calls passed; expanded tests ongoing.
Research continues in redirect;13/14/16/53 later-state graphs and other concrete
accepted raw consumers remain actionable, not deferred as independent. Do not
advance to disassembly from this checkpoint until redirect continuation resolves
or reasonably closes these avenues. Exact durable frontier/handoff follows.
Existing toolkit T001 only;all8buttons explicitly released after every restoration.

### Round020 redirect — context handoff, IN PROGRESS
Structural checkpoint remains round-020-disassembly; classification73.87%
(48409/65536 classified;17127Unknown), no map/state/source/ROM/release mutation.
Supported coherent proposal3395bytes:2292Code(1971Observed/321Static)+1103Data.
Sixtyactor calls(max1585/cap1800),152renderer cases(max475/cap1000),117audio calls
(max465/cap1000) passed;171new audio Data bytes all actual Reads. Every actor/audio
trace has0detail/aggregate drops. Source-produced state words, retained inline/RNG
barriers, finite motion/animation/audio graphs and per-instruction provenance saved.
Ready added IDs:R20-ACCEPTED-ACTOR-STATE-GRAPHS,R20-ACCEPTED-ACTOR-LITERAL-ASSETS;
no prior lead history lost. All protected hashes match;map.validateok. Existing
round-020-disassembly byte-identical65536 verification retained; no new checkpoint
for unchanged classification. [Audit](redirect/round-020/research-checkpoint.json),
[findings](redirect/round-020/findings.md),[leads](leads.json),[scenarios](scenarios.json).

This is a context rollover within redirect, NOT phase completion/exhaustion.
Next worker resumes SAME redirect with alternate-stage622b/6237 callers/stage/mode
writers and selected handlers, finite type-sound2134 caller filters for new produced
12/15/17/18, and rawE9 consumer8293. Then reassess any remaining explicit promising
avenues; do not begin disassembly merely because the first combined batch is ready.
Exact frontier and negative limitations:[handoff](round-020-redirect-handoff.md).
Toolkit existing confirmedT001 only,zero new/suspected/missingfeatures,no blocker;
no toolkit modifications. [Report](toolkit-issues.md).

### Round020 redirect continuation — active supplemental batch
Still SAME redirect; classification48409/65536=73.87%,17127Unknown unchanged.
Original3395-byte proposal preserved; supplemental conditional alternate-stage,
type20/21/22, E9/pitch and finite contact-consumer proof raises combined supported
proposal to7801Unknown bytes (4510Code+3291Data).36alternate and11parameter calls
saved; all successful calls zero detail/aggregate drops. Prior unsupported pitch
cursor5 sample explicitly excluded; source-domain type20 states7/8 words excluded.
Persistent state2 CP7/converter now proves normal+1b0..6, enabling seven contact
handlers/sounds and mode1 writer3072 without original global placement assumption.
Current exact research/frontier in redirect/round-020/findings.md and supplemental
verification. Remaining asset/consumer checks and lead packaging continue; do not
schedule next phase from this active checkpoint. No structural edits. Toolkit
existingT001 only,0new/no blocker; analysis/toolkit-issues.md retained.

### Round020 redirect — audited same-phase continuation checkpoint
Supported accumulated proposal12351Unknown bytes=5935Code(3427Observed/2508Static)+
6416Data. Original3395 preserved; new supplemental3643Code/5313Data. Classification
unchanged73.87%(48409/65536),17127Unknown.36alternate calls(max2927/cap6000),
198supplement renderer(max905/cap1000),11parameter calls(cursor5excluded),1EEcall
and reusable7contact calls audited with zero trace drops. Protected ROM/state/map/
configuredrelease/commentary hashes match,map.validateok; existing65536byte-identical
checkpoint unchanged. No source/map/state/ROM mutation or new checkpoint.
Added ready portions:R20-ALTERNATE-COMPLETION-CODE,R20-ALTERNATE-COMPLETION-ASSETS;
all earlier lead history preserved. Finite stage setup/update,type20/21/22, normalized
persistent0..6contact, stage-clear/death/ending0..14 and selected audio/assets retained.

Phase remains IN PROGRESS: final fixed-table reconciliation exposed runtime17
33d0->61ac omitted from old final graph. Type16 produces17 on BOTH variants;
5d7d/5e86 are actual18/19 converters, despite historical direct test names17/18.
Fetch evidence remains valid; runtime names corrected. Resume same redirect at61ac
and finite source-produced states before final avenue reassessment. Exact frontier:
round-020-redirect-handoff.md; audit:redirect/round-020/continuation-checkpoint.json.
Toolkit existingconfirmedT001 only,0new/suspected/missingfeatures,no blocker; report
analysis/toolkit-issues.md. No toolkit changes/rebuild/substitution.

### Round020 redirect — runtime17/player/scroll same-phase checkpoint
Still SAME redirect, not exhaustion. Classified48409/65536=73.87%,17127Unknown
unchanged. Audited proposal14685Unknown=7357Code(4201Observed/3156Static)+7328Data;
original3395 preserved, supplement5065Code/6225Data. No map/state/current.sym/ROM/
commentary-source/configuredrelease changes; protected hashes match,map.validateok;
existing65536byte-identical checkpoint retained unchanged. Exact union/provenance:
[verification](redirect/round-020/supplement-verification.json),
[audit](redirect/round-020/continuation-checkpoint.json).

Closed actualruntime17 four-state graph and preserved historicalactual18/19 test
correction; followed carrier/fixed/player/action/aux/death/HUD/window/scroll/camera
continuations, finite shape/box/timer/palette/prompt/frame/audio data and missing
ending mapperwrapper1719. CALL1a54nonlocalreturn excludes1a57..1a6e. New10type17/
14fixed/12player/1wrapper constructed calls pass, max1434/cap1800,zero trace drops;
76new renderer cases,274supplement total(max905/cap1000),152original preserved.
All8buttons explicitly released after every restore/import(T001). No new natural
route or natural timing/ownership/transition/ending claim.

New final source-context avenue remains productive: accepted type38 placementstage2/
nested0 writesC4051 and transitioninit incrementsC412->1. Existing40contexts omit2/1;
inspected351f->3966,width358c53,4220->42c0. No bytes proposed from this context yet.
NewleadR20-TYPE38-NESTED1-CONTEXT investigating; all prior ready leads/history retained
and updated to union14685. Resume SAME redirect here, not disassembly. Exact compact
[handoff](round-020-redirect-handoff.md), [findings](redirect/round-020/findings.md),
[leads](leads.json),[scenarios](scenarios.json). Toolkit existingconfirmedT001 only,
zero new/suspected/missingfeatures,no blocker; [report](toolkit-issues.md). No toolkit
modifications/build/substitution.

### Round020 redirect — COMPLETE ready combined batch

Redirect020 source-context continuation and concrete final avenue reassessment are
complete; next authorized phase is disassembly of the combined ready leads. Structural
checkpoint remains round-020-disassembly,48409/65536classified=73.87%,17127Unknown;
no map/state/current.sym/source/ROM/configuredrelease changes. Original3395proposal/
history preserved. Final audited union15126Unknown=7369Code(4269Observed/3100Static)+
7757Data; supplement5077Code/6654Data. Protected hashes match,map.validateok;
existing65536byte-identical verification unchanged, no redundant checkpoint.

New R20-TYPE38-NESTED1-CONTEXT ready: source stage2/nested0 type38->C4051 increments
nested1; exact counter domain47..0/cross0..4 yields240layoutbytes3966..3a55,
17alternate-transition rows,18phase2 placement rows,exact directory words and12byte
5731..573c adjustment suffix. Adds429Data+12ObservedCode. Width358c53 and last25cells
withheld: normal scroll queue bypassed until nested increments2. Selected bank3
blocks repeat known records; placementtypes32/8 already closed; persistent list empty.
No further actionable downstream graph appears.16newcalls,max489/cap1800,allzero
detail/aggregate drops,explicit8button releases after every import(T001). All prior
test histories reused without unchanged replay. No new natural route claimed.

Final triage preserves actual missing prerequisites: nested1width/tail need normal
scroll/descriptor source; stage3flags need raw-ID semantics; dynamic45d6 needs a
bounded source selector; uncalled0376/1266/2fbd/34af/0a58 lack inspected new callers;
failed natural routes/global domains have no changed method. No arbitrary sweep or
independent-scope deferral. Full final rationale:[findings](redirect/round-020/findings.md),
[exactunion](redirect/round-020/supplement-verification.json),
[audit](redirect/round-020/continuation-checkpoint.json),[leads](leads.json),
[handoff](round-020-redirect-handoff.md). Remaining structural17127Unknown persists;
15126are proposals only, not classification. Toolkit existingconfirmedT001only,
0new/0suspected/0missingfeatures,no blocker;[report](toolkit-issues.md).

## Round021 disassembly — supported combined batch; ROM incomplete

ROM classification: 96.95% (63535 / 65536 bytes classified; 2001 bytes unknown).

Added 15126 unique previously Unknown bytes: 7369 Code (4269 Observed, 3100 Static)
and 7757 Data. All five ready R20 lead IDs are verified; their overlapping ranges
are counted once. Exact per-lead retained histories: [leads](leads.json).
Freshly decoded 3625 saved instruction starts with CPU/bank contexts and recomputed
both ROM-driven sparse recipes exactly. Independently audited actual saved Instruction
events, zero detail/aggregate drops and safe RAM return sentinels; reached addresses
and pointer targets alone were not promoted to Observed. Source-state contracts,
local WRAM/VRAM pointer setup and finite consumer roles remain explicit.
[Independent proof](round-021evidence/verified.json), [RPC](round-021evidence/inspection-rpc.json),
[source evidence](redirect/round-020/findings.md). Confidence .97 for new regions.

Supported connected graphs include accepted later actor states/direct converters,
actual runtime17 (historical5d7d/5e86 names corrected to18/19), alternate stage actors,
normalized persistent contact0..6, completion/death/ending0..14, player/action/auxiliary
window/terrain/scroll/camera/HUD continuations and the1719 bank-restoring animation
wrapper. Sparse data covers source-produced words, copied seeds, finite shapes/boxes,
frames/audio/credit/palette/timer/tile structures and conditional stage2/nested1
transition and phase2 placement records. Labels/roles describe mechanical support;
full semantic source commentary is the next separate phase.

Preserved barriers: CALL1a54 discards its immediate return through1a6f;1a57..1a6e
stays Unknown. Type20state7/8 words stay Unknown. Nested1 transition major0..47
and cross0..4 supports exactly240cells3966..3a55; width358c and last25cells3a56..3a6e
are withheld because normal scrolling is bypassed until nested2. Type56 has exactly
two states; AE78..79 remains dual-use Code and waveform copy8523..8532 remains Code.
Wrapper1719 uses bank2 before1720, bank1 at1723..1728 and restores2 before172b.
Conditional HUD/timer/source-state bounds are local lifecycle contracts, not global
invariants or natural reachability. Unsupported parameter cursor5 remains excluded.

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|10076|5676|73|15825|559|
|01|8405|6953|67|15425|959|
|02|2948|13092|37|16077|307|
|03|227|15292|689|16208|176|

Totals21656 Code/41013 Data/866 Padding;2565 regions all have role and evidence.
All1489 prior region objects/metadata and unrelated fields remain byte-for-byte
equivalent in internal JSON;1076 new regions added. No old classification was
split/retracted. [Preservation](round-021evidence/preservation-check.json).
Unknown complement: [exact ranges](round-021-unknown-ranges.json). Execution coverage
is separate:4269 new Code bytes have actual historical instruction fetch evidence;
3100 new Code bytes are static. No whole-ROM execution percentage is claimed.

No new runtime, snapshot, input route or external gameplay research was needed.
Reused historical60actor/36alternate/6contact/11parameter/1EE/10type17/14fixed/
12player/1wrapper/16nested1 case records, original152 and supplemental274 renderer
cases, and source-selected audio records. Seven contact histories include a retry;
only six final contact-tests records are counted above. Exact patches, bank contexts,
button sequences and caps remain in [scenarios](scenarios.json) and redirect020 scripts.
All historical restores explicitly release all8buttons(T001). Artificial observations
prove local mechanics; natural later-stage/contact/ending/timing/ownership/visibility
remain unproved. Prior natural routes and unchanged failures were not replayed.

Connected easy gains are exhausted: no uninspected external Unknown callee remains
in the supported graphs; nested1 phase2 suffix closes its only new code continuation,
placementtypes32/8 are already closed, persistent list is empty and bank3 payloads
repeat known records. Width/tail needs a descriptor or normal-queue producer at
nested1. Stage3 flagged IDs still lack a decoding rule, dynamic45d6 a bounded source
selector, and uncalled0376/1266/2fbd/34af/0a58 a new inspected caller. Failed natural
routes/global domains have no changed prerequisite or useful new method. These
missing facts justify deferral; no arbitrary Unknown sweep or unchanged replay.

Sequential map.validate, map.save analysis/state.json and map.export maps/current.sym
(include_sym=false) passed before fresh `python3 run.py checkpoint round-021-disassembly`.
Stock map validation, generation and rebuild returned ok=true,byte_identical=true
for all65536 bytes. [Verification](round-021-disassembly-verification.json),
[final audit](round-021evidence/final-verification.json). Stock map hash
102123cf64822812ad5f340581918b70dd16a72a18a280390acbf110fd2b6081.
ROM/configured release/workspace config/accepted020 commentary hashes unchanged.
No toolkit edits/build/substitution.

Next phase commentary must preserve accepted020 source and1728region/1810unit review
credit by physical ranges/dependencies, explain all1076 new structural regions with
consistent routine/branch/field/constant symbols, reconcile affected older prose,
finish the entire current-source readability backlog, verify byte identity and
refresh the reading copy. Accepted source remains intact; no commentary or redirect
was run here. ROM remains incomplete until every byte meets structural and final
readability acceptance. Toolkit existing confirmedT001 only;zero new/suspected/
missing-feature findings,no blocker. [Persistent report](toolkit-issues.md).


## Round021 commentary — interrupted same-phase checkpoint

The entire accepted020 tree was archived under analysis/round-021/commentary-prior
and hashed before fresh preparation. 1728 prior scaffold regions/1810 units remain
archived;1569 exact unchanged scaffold records carried with source references
remapped through physical addresses.159 changed old Unknown extents produced1147
new/split records. Do not run prepare again or copy fresh assembly over source.

First semantic batch reviews15 new full scaffold regions and several partial units:
actual runtime17 states0..3, timer/mode/parent-slot window; corrected direct runtime18/19
identities;1719 animation wrapper's saved caller bank/bank1/restore; exact240cell
nested1 transition layout and bounded alternate transition/phase2 placement lists.
Rows now follow their five-byte structure. Runtime17 fields use shared symbols.
Added dual-use notes for8523..8532 Code/waveform-copy and AE78..79 Code/initializer.

Important correction: CALL1a54 is CD6F1A, targeting LD A,0D at1a6f, AFTER POP DE
at1a6e. It proceeds through the ordinary state/substate setter RET. Prior assertion
that this call discards its own return is contradicted by literal instructions.
Source now explains the distinction and retains1a57..1a6d as Unknown. No map/state
change made. Bounded continuation decode/caller-stack validation is a concrete
structural follow-up after commentary. See barrier-correction.json.

Fresh configured-release verify after both coherent source groups returned
ok=true,byte_identical=true for65536bytes. Source-index label-only copy rebuilt
identically. Read refreshed reading-after.asm. Final archive audit NOT invoked.
Frozen phase1-state/baseline equal current state/map; archived prior hashes match.

REMAIN IN COMMENTARY.1132 scaffold records/15413bytes remain pending, including
new Unknown boundaries; exact physical ranges/source locations are in
analysis/round-021/remaining-review.json. These numbers include sparse/subdivided
source, not only new classified bytes. Current-source readability is incomplete.
All other R21 actor/alternate/player/action/window/scroll/camera/HUD/audio/ending
graphs and assets need meaningful names, branch/RAM/field/constants and prose.
Review old dependencies too: wrapper and table comments claiming later states/raw
continuations are unsupported now need reconciliation (e.g.578f/58ab/5935/5d1d/
5d92/5e9b/5ff8/62cb/6ff2/702c/706c/7423,622b/6237,2d2c; main completion/ending).
Their prior review credit is not proof that these changed dependencies are current.
Review raw RAM/control operands and generic R21/obsolete Unclassified aliases across
remaining classified spans. Preserve useful old prose. Do not mark completion from
a script, byte-identity or this focused batch. Save refreshed physical source index
when continuing; do not rerun review-checkpoint.py because carried references have
already been remapped to current source.

Evidence: analysis/redirect/round-020/findings.md and supplement.py, supplement-
verification.json, combined-code-provenance.json, nested1.py and original verification;
analysis/round-021evidence/verified.json. No runtime/new inputs/research performed.
No natural reachability, timing, artwork or global HUD/timer-domain claim.
Toolkit:existing confirmed T001 only;zero new/suspected/missing features,no blocker;
no toolkit edits/build/substitution. Local carry aliases and obsolete nonlocal
interpretation are source/inference corrections,not toolkit defects.

### Round021 commentary continuation — main-state and ending checkpoint
Source death/continue and stage-completion graphs, early-title aliases, and bank2 ending state0..14/credits/reserved actors/rectangles/tile patches reviewed.113 meaningful routine/branch/structure contracts,46 newly reviewed full scaffold records;1086records/13409bytes still pending exactly in round-021/remaining-review.json. Corrected final ending input from historical 'Start' to held A: literal c00c AND10 at6dff. This is a source inference correction, not a new runtime observation. Unknown bytes and AE78..79 dual-use Code retained. No map/state/classification changes; ROM classification96.95%(63535/65536),2001Unknown.

Configured-release commentary verify ok=true,byte_identical=true65536bytes after coherent batches; read refreshed and label-only ending-index rebuilt identically. Source refs and current-index refreshed; protected hashes verified unchanged. No runtime/research/new scenario or toolkit edits. Existing T001 only,zero new/suspected/missing-feature findings,no blocker; analysis/toolkit-issues.md. REMAIN IN COMMENTARY; older actor/player/action/scroll/audio/HUD/asset backlog still prevents redirect.

### Round021 commentary continuation — same-phase checkpoint after window groups
Completed readable transition2ae9..2d0c, auxiliary-window2d33..2e30 and window-motion2e31..2ea6 graphs, including affected older prefix prose. Added41 meaningful branch/routine/structure contracts and reviewed21new full scaffold regions(622bytes). Then reviewed window controller24, triggers38/39, carrier32 and carrier arithmetic suffixes:23contracts,10new full scaffold regions(227bytes). Across this context:77new full regions/2853pending bytes resolved, plus affected prior dependency comments/symbols. The first group previously resolved46records/2004bytes. Current pending1055records/12560bytes; exact physical ranges/source refs in round-021/remaining-review.json. Current-index/source-index.json is copied from fresh window-actors-index and all prior review refs remapped sequentially by physical emission addresses. Do not rerun non-idempotent checkpoint/edit scripts.

Semantic limits: nested1 transition remains48rows*5cells; width358c/tail3a56..6e withheld. Window mode1 oscillates; mode2 terminates on its nonnegative SCY boundary. CA03 is a window-direction byte also read for type25 animation; natural ownership/timing unproved. Carrier helper74ff genuinely discards its immediate return; this does not revive the contradicted1a54→1a6f nonlocal-return claim. Final ending handler tests held A(c00c&10), not Start. Unknown raw regions retain Unknown; map/state/classification unchanged96.95%(63535/65536),2001Unknown.

Fresh configured-release commentary verify/read passed; ok=true,byte_identical=true65536bytes. Exact latest report round-021/continuation-verification.json; label-only window-actors-index rebuilt identically. ROM/map/state/workspace/configured-release hashes match partial-review protected hashes; archived prior untouched. No runtime/research/newscenario/toolkit edits; existing T001 only,zero new/suspected/missing features,no blocker. Remaining actor/alternate/player/action/camera/HUD/audio/assets source is pending; current phase COMMENTARY, no redirect or final audit/acceptance.

### Round021 actor continuation (commentary still in progress)

Reviewed the complete local machines at578f..599e and5d1d..5ff7, shared decrement664e..6654, allocated-actor initializer6e6f..6e97 and allocator/accessors6f92..6ff1. Added explicit five/four/five-state handler words, entry-position counts, asymmetric player-X trigger distances, STAT-gated3x3 background erasure, screen-edge/Y gates, three-slot conversion and companion reservation/allocation/cooldown. Eight terrain probes now have signed offset pairs and direction overwrite order. Introduced contextual actor-work and allocation/probe scratch symbols. Static contracts explain these paths; no new runtime or natural-placement claims.

Corrected inherited converter prose: runtime18/type13 and runtime19/type14 access field1, add14 then9, reaching15 then1e. They write field1e=ff and skip its usual clear; field1d is untouched. Previous first-batch claims of1d=ff/retained1e were wrong. Current source and review units carry the correction; historical artifacts remain historical.

This continuation reviewed30 new full scaffold records (977 formerly pending bytes), with85 source contracts including already-reviewed dependencies. Pending1025 records/11583bytes; exact ranges analysis/round-021/remaining-review.json. Current physical references remapped from the prior CURRENT index to companion-actors-index, then promoted. Configured release verify ok=true/byte_identical=true65536bytes; label-only index independently rebuilt identical; reading copy refreshed. All protected hashes remain unchanged. Classification96.95%(63535/65536),2001Unknown unchanged.

Toolkit: existing confirmedT001 only; no new/suspected issues or missing features, no blocker. See analysis/toolkit-issues.md. No classification, runtime, research or toolkit edits. Continue ONLY commentary; actor16/alternate/player/action/camera/HUD/audio/assets backlog remains. No audit/redirect acceptance.

Round021 continuation: runtime16 full machine5ff8..61ab plus alternate dispatch/stage0 runtime11 machine622b..653f and stage1 counter initializer6811..681f now reviewed. Added41full scaffold records/1023pending bytes in these groups. Pending984records/10560bytes; current-index matches alternate-stage-zero-index. Source explains parent17 link/delay and variant-latch interaction; signed wrapped HUD arithmetic; unchecked setup stage indices; primary phase/follower cleanup; pointer save/restore and placement; six runtime11 states and missing allocation-failure guards. External stage2/3 handler names establish table association only; bodies still pending. Fresh configured verification byte_identical=true; indexed label copy identical and reading copy refreshed. Classification/protected inputs unchanged. ExistingT001 only, no new toolkit findings (analysis/toolkit-issues.md). Same commentary phase remains incomplete.

### Round021 actor/alternate continuation checkpoint — still incomplete

Additional stage1 updater6820..697a reviewed13full records/287pending bytes: exact80 occupancy counting,20spawn160call sequence, five-slot overwrite without occupancy guard, four motion seeds and all-zero completion. Final continuation aggregate84new full records/2287previously pending bytes,178source contracts; round021 total176new full scaffold reviews. Exact pending971records/10273bytes (analysis/round-021/remaining-review.json). Current-index promoted from actor-continuation-final-index; review references remapped from prior CURRENT index and contextual bank1 units corrected. Protected hashes matched for ROM/state/map/workspace/configured release. Source comments distinguish type13/14 side-selector1e from runtime17 parent link1e.

Final configured commentary verify ok=true,byte_identical=true65536bytes; label-only indexed copy identical, reading-after.asm refreshed. Exact verification saved to analysis/round-021/continuation-verification.json and partial-review.json. Initial actor-trio verification failed on an undefined provisional R21_code_6eef name; replacing it with the actual existing AdvanceOrTurnActorOnTerrain target resolved the link, with all subsequent verification runs passing. This was an analysis naming error, not a toolkit defect.

No runtime/research/classification changes. Classification96.95%(63535/65536);2001Unknown unchanged. Commentary remains IN PROGRESS; exact same-phase frontier is analysis/round-021-commentary-handoff.md, next coherent groups6540..6810,697b..6b69,stage3 plus broad player/action/camera/HUD/audio/assets backlog. No redirect or final audit/archive acceptance. Existing confirmedT001 only; no new/suspected/missing-feature toolkit issues, no blocker. Persistent report: analysis/toolkit-issues.md.

### Round021 commentary runtime20/stage2 checkpoint — remains in commentary

Reviewed6540..6810 except previously reviewed664e..6654: ten-state runtime20 dispatch, stage2 setup, two weighted random tables, terrain/width/facing gates, sprite centering and nine sparse action seeds.53 contracts;47 newly reviewed full scaffold records/691 formerly pending bytes. Sparse loader destinations are2,6,8,9,0d,0f,10,15,1c: final bytes are sprite flags/countdown, with field1e set separately by the side-sensitive branch. Current source correctly explains this arithmetic. States7/8 alias existing handlers but natural entry remains unresolved. Exact pending924records/9582bytes in round-021/remaining-review.json; current-index now stage-two-runtime-index. Fresh configured verification byte_identical=true65536, label-only indexed rebuild identical, reading refreshed; protected hashes matched. Classification unchanged96.95%(63535/65536),2001Unknown. No runtime/research/toolkit changes; existing confirmedT001 only,no new/suspected/missing features,no blocker (analysis/toolkit-issues.md). Next same-phase group runtime21 697b..6b55,stage3 dispatcher6b56 onward; broad readability backlog remains.

Round021 runtime21 action machine697b..6b55 reviewed35contracts/33new full records/448pending bytes. Six-state dispatch, entry/repeat/retirement delays, wrapped sprite-distance selection, eleven motion seeds and sparse loader now explained. Normal6b3b retirement increments C431: stage1 initial20 is a replenishable quota, not a proven total20 spawns; current earlier updater/review clarified. Corrected runtime20 delayed choice weights to2state5/4state9/2state1. Current pending891records/9134bytes; current-index=runtime-twenty-one-final-index, fresh configured verify and label-only rebuild byte-identical65536,reading refreshed/protected hashes matched. No runtime/research/classification/toolkit changes; existingT001 only,no new finding/no blocker. Stage3 remains next; current phase commentary incomplete.

Round021 stage3 initial graph6b56..6d94 reviewed26contracts/24newfull records/569pending bytes: seven-state dispatcher, four source-bounded placement positions, alternating animation/count phases, parity-selected four-child volleys with independently checked allocation failures, eight ten-byte seeds and next-phase setup. Late state5/6 tails remain pending.867records/8565bytes pending; current-index=stage-three-primary-index; fresh verify/index rebuild byte-identical65536,reading refreshed,protected hashes matched. Two undefined provisional RAM names caught at linking were replaced with existing symbols before successful verification; local naming issue,not toolkit defect. No classification/runtime/research/toolkit changes. ExistingT001 only,no new finding/no blocker; continue same commentary phase.

Round021 stage3 late tails6d95..6e6e reviewed13contracts/8newfull records/218pending bytes. Exact moving-pass gate preserves A/HL changes: helper6ddb returns preincrement1d in A/HL at1d,so later comparisons are not independent X tests; with no matched gate final[HL] is spriteX,so X7 takes the state6 tail and writesX0/Y32. Natural fallback occurrence unproved.6df8 OR A checks predecrement1c,so initial32 fires on call33 when zero wraps255. Child allocations create two runtime23 then preferred runtime9 C620/random branch or ordinary third runtime23; earlier children survive failure. Fresh verify/index rebuild byte-identical65536,reading/current-index refreshed/protected hashes matched. Pending859records/8347bytes. Classification96.95%(63535/65536),2001Unknown unchanged. No runtime/research/toolkit changes; existingT001 only,no new finding/no blocker. Continue ONLY commentary.

### Round021 alternate/actor9 commentary same-phase checkpoint

Actor9 full5c2b..5d1c plus primary-Y helper6f6a..6f91 reviewed20contracts/14newfull records/274pending bytes. Zero/nonzero state gate,60/20/160call delay fields,upper-terrain gate after four motion choices,finite random table/four motion seeds and primary-facing branch explained. Low-byte comparison in6f6a cannot break sprite-Y ties because its JR C has zero displacement. No new runtime/natural-entry claim. Updated uncertainties' stale historical completion claim to current incomplete status.

This context reviewed126newfull scaffold records/2200previously pending bytes across five groups,147meaningful contracts. Round021 total302newfull reviews;2138units inventoried. Current pending845records/8073bytes exactly in round-021/remaining-review.json. Current-index now runtime-nine-index; all source refs remapped from the prior CURRENT physical index after each batch. Fresh configured verify ok=true/byte_identical=true65536; independent label-only index rebuild identical,reading-after refreshed. continuation-verification.json/partial-review.json updated,protected ROM/map/state/workspace/release hashes match. No prepare/carry/regeneration/historical scripts rerun.

Classification remains96.95%(63535/65536),2001Unknown; readability remains incomplete. Continue SAME COMMENTARY phase through player/action/scroll/camera/HUD/audio/assets and remaining older dependency gaps. No final audit/archive/redirect acceptance. Toolkit:existing confirmedT001 only,zero new/suspected/missing features,no blocker; analysis/toolkit-issues.md. No toolkit/runtime/research/classification changes. Local provisional symbols and initial insertion-address typo corrected before final successful verification; no toolkit defect.

### Round021 player action/climbing/camera continuation — commentary remains incomplete
Reviewed24new full scaffold records/623pending bytes in player Down state1a77..1ada,action restoration1b22,sound1b7b,auxiliary setup1bc2..1c32/store1c66; climbing/facing1d98..1e6e,transition entry1e89,snap-camera1f3a..1faa,alternateYclamp2025,HUD328a..32aa.39sourcecontracts,prior dependency prose repaired without raw-tail claims. Camera reconciliation walks one world pixel per iteration through clamp/camera/column queue; convergence requires target/clamp compatibility and no iteration cap exists. Climbing/Down identities remain tentative; no natural reachability claim. Current pending821records/7450bytes,exact round-021/remaining-review.json,current-index=player-climb-camera-index. Fresh configured verify and independent label-index rebuild byte-identical65536;reading refreshed,protected hashes matched. No runtime/research/classification changes:96.95%(63535/65536),2001Unknown. Existing confirmedT001 only,no new/suspected/missing-feature toolkit findings,no blocker(analysis/toolkit-issues.md). Remain ONLY commentary,not redirect/final acceptance.

### Round021 broad player/sprite/shape continuation — remains in commentary
This context reviewed730new full scaffold records/4308pending bytes. Player Down/action/climbing/camera and fixed contact/pickup/ending-wrapper continuations have explicit branch/RAM/field semantics; finite animation assets resolve616records2675bytes across339semantic structures. Source gives exact FE duration/continuation vs FF unused-byte/redirect semantics, payload counts and signed component geometry,tile and explicit-XOR/reused attribute behavior. Shapes now describe exact first/probe-half XY pairs and unsigned contact-box extents. Direct41/42 convert to runtime7 with explicit motion/origin fields; hazard tail sets gateff/animation8. Six scaffold records spanning already explained adjacent actor groups were inspected across their entire extent and reviewed without duplicating good prose.
Current pending115records/3765bytes,exact round-021/remaining-review.json,current-index=actor-residuals-index. Remaining classified source is bank2 audio code/data; Unknown boundary records remain pending separately. Source readability is not complete. Fresh configured verification ok=true,byte_identical=true65536;independent label-index copy identical,reading refreshed;protected hashes match. ROM classification96.95%(63535/65536),2001Unknown unchanged. No runtime/research/newinput/classification/toolkit changes. Existing confirmedT001 only,no new/suspected/missing-feature findings,no blocker;analysis/toolkit-issues.md. Continue ONLY commentary,not redirect/final audit/archive acceptance.

### Round021 commentary current-source completion
Completed the entire remaining845record/8073byte readability backlog in this context,including player/action/climbing/camera/HUD,finite sprite/shape structures,audio parameters/frequency/wave/pitch/directory/header/event graphs and50Unknown-boundary records. All2716scaffolds/2854meaningful units reviewed;pending0. Actual semantic scope review repaired18generic state-word aliases,rawRAM/mapper/OAM operands,primary hit-budget vs HUD field19 symbols and stale5731raw-suffix prose. Raw-tail notes at still-Unknown23xx/7659 were checked and preserved. Detailed manual assessment:round-021/readability-assessment.md;exact report:round-021/commentary-source-completion.json. Inventory and fidelity are supporting checks,not semantic proof.

Fresh configured commentary verify ok=true/byte_identical=true65536;label-only indexed rebuild identical;reading refreshed. Current-index=scope-final-index and current review refs remapped physically;protected hashes match. Classification unchanged96.95%(63535/65536),2001Unknown. No runtime/research/newinput/classification/toolkit changes. Existing confirmedT001 only,no new/suspected/missing-feature findings,no blocker;analysis/toolkit-issues.md. Current-source commentary complete;whole-ROM classification incomplete and final editorial/archive acceptance still pending. No final audit run.

## Round021 redirect — combined396-byte ready batch; ROM incomplete

Current-source gate2716records/2854units,pending0 accepted. Source-led redirect
completed connected and independent avenue reassessment; three new readyIDs in
[leads](leads.json). Classification remains63535/65536=96.95%,2001Unknown;
checkpoint round-021-disassembly/fidelity65536bytes retained. No map/state/source/
ROM/configuration/configuredrelease mutation and no fresh checkpoint this phase.

[Exact findings](redirect/round-021/findings.md),[handoff](round-021-redirect-handoff.md),
[ranges/roles/contexts/18frames](redirect/round-021/verified.json),
[protection/verification](redirect/round-021/final-verification.json).
Proposed396Unknown=56Code(32newObserved/24Static)+340Data. CorrectedCALL1a54 enters
1a6f AFTERPOP1a6e,returns1a57;23byte jump/motion tail endswith separatePOP/RET.
Leftwarddirection falls through9byte1962tail. Other24Codebytes close contact,
acceleration,clamp,classmask/tileclear and pitchrepeat glue. Two9byte runtime20
seeds,sixground/air actionbytes,runtime1word and298spritebytes(animations1/3/4/7/60)
add324Data. Source-preserving incomingcontactcontexts add16Data (12amountbytes,
shape6/12boxpairs);incomingbit5 differs from auxiliarybit6. Converters41/42->7,
43->15,allocatedchild17 supply exactconditionaldomains. Type25 effectE=3c proves60.

New2climbingA calls215ops/cap600,C302/SPdff4;9contacts494ops/cap1200,C300/SPdff2;
zero trace detail/aggregate drops. Actualfetches prove32Code,not the24staticbytes.
[Scenarios](scenarios.json) and localrecipes retain exactpatches/inputrelease/banks.
After every snapshotimport releaseall8buttons(T001). FirstrawC00C/C00D attempt
failed; retainednegative log,correct sampledC418/C41A succeeds. Constructedoverlap/
inputcache tests prove no naturalroute,timing,visibility or globaldomains.

Finaltriage inspected12classifiedcontrolfrontiers,8unknownimmediatepointerroots,
50commentaryunknownboundarycontracts and saveduncertainties/frontiers. Every
productivegraph/downstreamasset closes. RSTgaps needactualselectorwriters; further
contactslots lacksourcecompatibleflags/types or repeatknownmechanics. Nested1
width/tail,stage3flags,dynamic45d6,olduncalledhelpers,naturalfailedroutes and sparse
assets retain explicit prerequisites in findings.md;no unchangedreplay/unknownsweep.
Nextdisassembly validates combinedbatch;nextcommentary corrects eightlegacy
TypeSevenContactSprite headercomments: secondpayloadbyte05 ignored,notshape5.
Sourcepreserved thisphase. Toolkit existingconfirmedT001 only,0new/0suspected/
0missingfeatures,no blocker;[report](toolkit-issues.md). No toolkitedit/build/substitution.

## Round022 disassembly — three supported leads verified; ROM incomplete

ROM classification: 97.55% (63931 / 65536 bytes classified; 1605 bytes unknown).

Input checkpoint round-021-disassembly and completed021 commentary2716records/2854units
(pending0). Output fresh round-022-disassembly; only disassembly ran. Added396unique
previouslyUnknown bytes:56Code(32Observed/24Static)+340Data,89newregions. Exact
all2565prior region objects/metadata/unrelated fields preserved;2654current regions
all have role/evidence. [Preservation](round-022evidence/preservation-check.json).
No baseline import or reclassification; accepted021commentary source unchanged.

Consumed three ready IDs,now verified with exact histories retained in[leads](leads.json):
R21-ORDINARY-CONTINUATIONS(56Code),R21-SOURCE-SELECTED-ASSETS(324Data),
R21-CONDITIONAL-INCOMING-CONTACT(16Data). Classification is their exact union,not
range hulls or an unchecked lead sum. [Recomputed recipe](round-022evidence/recompute.py),
[contexts/decodes/roles](round-022evidence/verified.json),[independent audit](round-022evidence/audit.json),
[mutation RPC](round-022evidence/mutation-rpc.json). New confidence .97.

Independent fresh decode/structure inspected every instruction start with CPU aliases
and effective fixed0/switch1 or bank2 context; the ROM-driven recipe was recomputed
in a separate output directory and matched redirect021 exactly. Independent component
walking closed18frame nodes for source-selected animations1/3/4/7/60:298newData bytes.
Additional Data: two9byte sparse runtime20 seeds, six saved-ground/air actionbytes,
runtime1dispatchword, twelve source-compatible incoming-contact amountbytes and
shape6/12boxpairs. No neighboring selector/pointer/state or graph gap was inferred.
Data provenance remains Static; actual saved Reads separately corroborate17newData
bytes in the combined tests,including13incoming-contact bytes.

Historical actualInstruction events were audited against ROM opcode values,physical
banks,CPU aliases and safe return sentinels;32Code bytes have actual fetch support.
Two climbing-A calls215ops/cap600 returnC302/SPdff4; nine source-output-preserving
overlaps494ops/cap1200 returnC300/SPdff2;all detail/aggregate drops0. Exact sampled
C418/C41A inputs,bank/actor/player/stack patches, persistent snapshot and explicit
all8buttonrelease recipe remain in[scenarios](scenarios.json),redirect/round-021/
probe.py/contact.py and tests.json/contact-tests.json. No replay was necessary;
failed rawC00C/C00D experiment remains negative evidence only. No new natural route,
external research,runtime,input or snapshot. Execution support is separate from
classification:24newCode bytes remain statically justified ordinary continuations.
No whole-ROM execution percentage or natural timing/visibility/contact claim.

Code roles:1962+9 clears facing flag20;1a57+23 initializes jump/motion after ordinary
CALL1a54->1a6f and finallyPOP1a6c discards CheckA's caller;20ae+5 submits sound12;
22a1+6 propagates velocity subtraction borrow;2316+2 clamps vertical highbyte7;
75cb+3 sets carry before existingclassmask logic;7659+1 returns from selectedtileclear;
physical8564+7 CPU4564/effectivebank2 clears repeatcount,advances pitchcursor twice,
loops4506. The skippedPOP1a6e is DISTINCT from finalPOP1a6c. Incomingcontactrequires
shape bit5,auxiliarycontactbit6;converters41/42->runtime7,43->15,child17 supply
postconversion amount indices. Type25 effect E=3c@2192 establishesanimation60;
workdelay3c@5c3d is unrelated. Payload secondbyte05 ignored,never actorshape5.

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|10121|5688|73|15882|502|
|01|8409|7281|67|15757|627|
|02|2955|13092|37|16084|300|
|03|227|15292|689|16208|176|

Totals21712Code/41353Data/866Padding. Exact remaining complement:
[unknown ranges](round-022-unknown-ranges.json). Unknown counts502/627/300/176.
Classification and fidelity gates are separate. Sequential map.validate,map.save
analysis/state.json,map.export maps/current.sym(include_sym=false) passed before
fresh `python3 run.py checkpoint round-022-disassembly`: stock map validation,
generation and rebuild ok=true,byte_identical=true for all65536bytes.
[Verification](round-022-disassembly-verification.json),[final audit](round-022evidence/final-verification.json).
Maphash879ee77c6e8220af0bc278ca0eecd7dd484df2e33333a46187ced78c7ee50c78.
ROM,workspaceconfig,configuredrelease and accepted021source hashes unchanged;
no toolkit edits/build/substitution.

Easy gains exhausted under changed-evidence policy: inspected connected routines
and accepted downstream structures close; redirect021's source-frontier audit found
no additional productive continuation. Ordinary gameplay's preserved failed routes
have no changed prerequisite/method or credible new substantial gain; unchanged
replay would repeat negative history. RST words require selectorproducers;runtime0/
20state0/7/8/21state0 needactual occupied/statewriters;remaining contactslots need
compatible source type/bit5/shape rather than arbitrary patches. Nested1width358c/
layouttail3a56..6e needdescriptor/normalqueueproducer;stage3flagIDs needdecode rule;
dynamic45d6 needsboundedselector;uncalledhelpers neednewcaller;audio/graphics
neighbors lacksource-selected consumers. No arbitraryunknown sweep. Supporting
negative/source-domain reasoning retained in[redirect findings](redirect/round-021/findings.md).

Next ONLY commentary: carry accepted021source and all2716records/2854units prior
review credit by physical dependencies. Complete readability for89newregions and
affectedolderprose,including newlyclosed unknown-boundary notes and eightlegacy
TypeSevenContactSprite1..8Header descriptions: replace'shape5'with ignoredheaderbyte.
Preserve endingheldA,converter18/19field1eFF,replenished21quota,stage3A/HL/preDEC
anddualuse findings. Finish entire current-source scope,verify byteidentity,refresh
reading;no partialrepair gate. Full-ROM finalacceptance still requires1605Unknown
resolved plus finaleditorial audit. Toolkit existingconfirmedT001 only;zero new/
suspected/missingfeatures,no blocker;[persistent report](toolkit-issues.md).

## Round022 commentary — complete current-source readability
Input/output structural checkpoint round-022-disassembly remains unchanged:
63931/65536classified (97.55%),1605Unknown. Entire accepted021 commentary archived
before fresh prepare. Carried2688exactphysicalreviews;completed100new/splitrecords
and affected old notes:2788scaffolds/2926units reviewed,pending0. Scope includes
allclassifiedsource with validpriorcredit,not89additionsalone. Corrected eight
TypeSevenContactSprite headers' misleading shape5 claim to ignoredsecondbyte05;
explained climbing-A ordinaryreturn/finalPOP,otherordinarytails,seeds,state0/1tables,
runtime1word,18animationnodes/payloads,incomingpostconversiontypes/shape6/12pairs.
Oldraw-boundaryprose and correspondingreviews reconciled. Exactassessment,
confidence/limits and contracts:[scope](round-022/readability-assessment.md).
No new gameplayinputs/runtime/research;priorconstructed215/494opcases remain
conditional evidence only;originalscenariosequences unchanged. Unknownranges
remainround-022-unknown-ranges.json;no classificationchange. Fresh configured
commentary verifyok=true,byte_identical=true65536bytes;independentfinal-index matches
andreading-afterrefreshed. ProtectedROM/state/map/workspace/release andfrozenbaseline
match. Entireaccepted021archive hashes verified. [Completion](round-022/commentary-source-completion.json),
[handoff](round-022-commentary-handoff.md). Finaleditorial/archiveaudit pending;noauditrun.
ToolkitexistingoneconfirmedT001,zero new/suspected/missingfeatures,noblocker;
[report](toolkit-issues.md). No toolkitedits/build/substitution;only stock.sym distributable.

## Round022 redirect — completed reassessment; no actionable new lead

ROM classification remains97.55% (63931/65536),1605Unknown; perbank502/627/300/176.
Input/output checkpoint round-022-disassembly unchanged; current readability complete
2788records/2926units,pending0. Fresh bounded scan11266classified starts finds only
4knownRSTinline-tablefrontiers, no ordinary missingcallee/fallthrough; immediate
Unknown roots8->4,all unchanged conditional bases. Current102changedcontracts,
source boundaries/new seeds/assets/converters/contact and older layout/audio/state
prerequisites reassessed. No new/improved ready lead; passes entry added with empty
added/updated/ready arrays, all prior outcomes retained. [Findings and actual missing
prerequisites](redirect/round-022/findings.md),[compact handoff](round-022-redirect-handoff.md).

Fresh configuredrelease map.validate ok=true; all protectedROM/state/map/config/
release plus7commentarysourcehashes/readingcopy match. [Verification](redirect/round-022/verification.json).
Accepted022disassembly and freshcommentary byteidentical65536 checks remain valid;
no new checkpoint,generation,archiveaudit,map/source edits or reclassification.
No runtime/input/snapshot/external research; unchanged failednaturalroutes not replayed.
Existing scenarios and explicitall8buttonreassertion T001 workaround retained.
Remaining unknown complement round-022-unknown-ranges.json unchanged. Stopworkflow
incomplete until new selector/producer/caller facts or improved justified method;
this is not wholeROM completion. Toolkit existing1confirmedT001,zero new/suspected/
missingfeatures,no blocker; [report](toolkit-issues.md). No toolkit edit/build/substitution.
### Post-round022 continuation revalidation
Previous turn completed a substantive redirect reassessment; this continuation
revalidated the resulting evidence impasse without repeating exhausted analysis.
Current state still has63931classified/1605Unknown and zero ready leads. All seven
accepted commentary source hashes, reading-copy hash and original ROM hash match
the verified completion report. No changed selector/producer/caller fact or useful
new method was found. Per goal.md, do not start an unchanged cycle. This is the
second consecutive goal turn with the same no-actionable-evidence condition;
goal remains active under the blocked-audit threshold. Exact check:
analysis/redirect/round-022/continuation-revalidation.json. No map/source/runtime
or toolkit changes; existing T001 only, no new toolkit findings.


### Third consecutive evidence-impasse audit
Current state again confirms63931classified/1605Unknown, zero ready leads, unchanged latest redirect pass, and exact verified source/ROM/reading hashes. Same missing selector/producer/caller facts have persisted across three consecutive goal turns. No useful changed prerequisite or concrete improved method is available under goal.md; unchanged cycles are prohibited. Blocked-audit threshold met; goal marked blocked pending new evidence or external-state change. No map/source/runtime/toolkit modifications. Existing T001 only, no new toolkit findings; analysis/toolkit-issues.md. Exact audit: analysis/redirect/round-022/continuation-revalidation.json.

## Final-investigation prompt migration (2026-10-18)

Added prompts/final-investigation.txt and coordinator/workflow scheduling for stalled
near-complete ROMs. Round022 has 1605/65536 unknown bytes and meets both thresholds.
Next resume should run final investigation after confirming the existing commentary
handoff; the earlier no-new-cycle instruction does not preclude this new method.
No investigation was run and no ROM, maps, state, source, classification, review
credit or verification artifacts changed. Historical handoffs remain unchanged.

## Final investigation FI022-20261018-v1 — completed structural reassessment (2026-10-18)

Input/output structural checkpoint round-022-disassembly unchanged; mapSHA256
879ee77c6e8220af0bc278ca0eecd7dd484df2e33333a46187ced78c7ee50c78.
Independent full-physical complement:63931/65536classified=97.55096435546875%,
1605Unknown in134ranges; perbank502/627/300/176. Existing current-source commentary
complete2788records/2926units,pending0. Exact input hashes and all-range bank/CPU/
neighbor/hypothesis/history/missing-fact dispositions saved under
[final-investigation](final-investigation/handoff.md),[ledger](final-investigation/range-ledger.json).

Materially changed method completed: isolated utility algebra/stack/code-boundary
proof;128contextualinstructionstarts and128directedges audited; closed FE/FF sprite
graphs with parity-counted exactpayloads; explicit0..5channelmaskbound; six unselected
five-byte record-format probes; layoutwidth/pointer crosscheck; frequencyencoding
versus unbounded index; scoped selector predecessor proof; remaininggraphic/seed/
command/isolatedRET boundary audit. Supported311byte sparse proposal union:
198StaticCode+113StaticData; zero newObserved. Proposals do not mutate classification.
Ready leads FI22-STATIC-UTILITY-CONTRACTS and FI22-CLOSED-ANIMATION-AND-CHANNEL-MASK
appended preserving all earlier IDs/outcomes. If fully accepted,1294Unknown remain;
no whole-ROM completion claim. Unselected sparse-table extents/RET/pattern-only
assets remain withheld with exact missingfacts. [Detailed findings](final-investigation/findings.md),
[manifest](final-investigation/proposal-manifest.json),[negative and preservation checks](final-investigation/independent-checks.json).

No emulator/input/snapshot/debugger mutation,external research or gameplay route
added; historical scenario sequences remain unchanged. Configuredrelease map.load/
map.validatepassed;15protectedROM/state/map/workspace/release/current-sourcehashes
match input. No map/source/toolkit edits,save/export,newcheckpoint,generation,rebuild
or finalarchiveaudit. Existing accepted022disassembly/commentary byte-identical65536
reports remain fidelity evidence only. Next phase disassembly validates exactproposals,
then fullcurrent-source commentary,then redirect; no otherphase launched here.

Attempt status completed; exact checkpointidentity and exhaustedmethods in
[completion](final-investigation/completion.json). Do not automatically repeat on
unchanged evidence; changedround alone insufficient. New evidence/materially improved
concrete method must address recorded barriers. Existing confirmedT001 only;zero
new/suspected/missingfeatures,no blocker;[toolkit report](toolkit-issues.md).

### Final investigation same-phase revision — preliminary 311 superseded; final 969

The preliminary completion above was superseded before phase delivery. A late direct
consumer audit distinguished transition code1eef..1ef2's explicit high-E0 terminal
from placement coordinate comparisons. Four independently rooted terminated objects
propagate their fifth-byte descriptor links into28 finite contexts. Exact descriptor,
layout, placement and raw-ID closure adds472 Static Data bytes. New source types34/35
and literal child37 connect existing handlers to count5/count2 setups and bounded FF
command streams, adding109 Data bytes. Independent sound4/25 header/channel1/FFstream
proof adds36; scoped note-initialized pitch FB/FEFF graphs and EA29word add41.

Final disjoint proposal union is969 bytes:198 Static Code +771 Static Data, zero new
Observed. The current map remains63931 classified /1605 Unknown; if all proposals are
accepted,636 Unknown remain (banks302/103/119/112). Every original134range now has
exact proposed/residual slices and a disposition. Initial311completion/handoff are
preserved under explicitly superseded filenames. Final[handoff](final-investigation/handoff.md),
[manifest](final-investigation/proposal-manifest.json),[ledger](final-investigation/range-ledger.json),
[completion identity/methods](final-investigation/completion.json) are authoritative.

No runtime/input/snapshot/research/map/source/toolkit changes. All15 protected hashes
still match the captured input. Existing byte-identical reports retained, no new
checkpoint/rebuild or final archive audit. Remaining source/extent barriers are explicit;
all productive new object continuations closed through existing code/assets. Four ready
leads preserve every old outcome. Next only independent disassembly validation, then
ALL current-source commentary and redirect. Do not repeat the final investigation
unchanged; new evidence or a concrete improved method must address recorded barriers.
Existing confirmed T001 only, zero new findings or blockers;[report](toolkit-issues.md).


## Round023 — disassembly complete, ROM incomplete

ROM classification: 99.03% (64900 / 65536 bytes classified; 636 bytes unknown).

Input round-022-disassembly plus authoritative final-investigation FI022 final969
manifest (preliminary311 superseded). Output round-023-disassembly. Accepted exact
60-span969-byte union:198StaticCode and771StaticData,zero newlyObserved. Independent
configured-release decode/structure checks cover128starts/128edges and full bank/CPU
aliases; five ROM grammar recipes recomputed into this pass's artifacts match the
saved proofs exactly. Reviewed arithmetic/carry, stack/return, saved-target accessor,
palette/serial contracts; FE/FF animation graphs and parity-counted payloads; explicit
0..5 NR51 channel bound; explicit high-E0 transition termination and fifth-field
propagation through28descriptor-output contexts; coordinate-bounded placements;
source34/35 setup counts5/2,FFcommand closure and literal child37; noise mask02
headers/FFstop; pitch note-initialized count0/cursor0 FEFF loops. Exact sparse extents
only, no arbitrary alias/table/placement expansion. All four FI22 ready leads verified.

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|10278|5731|73|16082|302|
|01|8413|7801|67|16281|103|
|02|2992|13236|37|16265|119|
|03|227|15356|689|16272|112|

Total Code21910/Data42124/Padding866. All2714regions have role/evidence metadata;
all2654prior regions and metadata plus unrelated state fields preserved exactly.
Observed-provenance Code remains15032bytes; static classification does not increase
execution coverage. Accepted022commentary source and reading copy remain unchanged;
2788records/2926units pending0 credit is prior-source credit, not a new-source review.

Caveats:2fbd uses saved contact-targetc3d2/c3d3,not currentactorc416/c417. Palette
index3 touches c016;B0 wraps the final loop. Serial software handshake writer and
working feature unproved. Animation29/37,sound4/25 natural selection unproved.
PitchFEFF closure assumes ordinary timed-note count0 initialization,not inherited255.
No hardware compatibility,timing,dead-code or natural gameplay claim.

No emulator/input/snapshot/debugger patch/external research or new normal route.
Reused valid evidence; unchanged failed routes not repeated. No concrete improved
natural route or further supported connected extension remains: types34/35/37 close
through accepted handlers/assets; new noise streams introduce no further selectors;
pitch graphs close within scoped initialization. Residual methods/barriers remain
in final-investigation/range-ledger.json. Exact636-byte complement saved to
[unknown ranges](round-023-unknown-ranges.json),matching FI projected complement.
Isolated RETs,unbounded frequency/alias tables,unsupported selector amounts,weak
fixed records,stage2ordinary-width and stage3flagged-ID questions remainUnknown.

Sequential configured-release map.validate/save analysis/state.json/export
maps/current.sym include_sym=false passed. Fresh `python3 run.py checkpoint
round-023-disassembly` passed stock map validation,generation and byte_identical=true
for all65536bytes; mapSHA256
712619d00b78c4033a2b9a8fa4ce9c012538d91f7af323d5a932ecade09c9a0e.
[Verification](round-023-disassembly-verification.json),
[independent proof](round-023evidence/audit.json),
[preservation](round-023evidence/preservation-check.json).
ProtectedROM,configuredrelease,configuration and accepted022source hashes match.
Initial annotation attempt used unsupported capitalized provenance and was rejected
before persistent mutation; lowercase retry passed. Local API usage correction,
not a toolkit defect. No toolkit edit/build/substitution or commit.

Next phase commentary: complete ALL current classified source, preserve sound prior
credit, explain/name the60new regions and update old source claims narrowed to type33/
selector0 or formerly unclassified slices. Preserve all caveats above and existing
dual-use code waveform interpretation. Then redirect; this worker launched no other
phase. Remaining unknown bytes prevent completion regardless of byte fidelity.
Toolkit:existing one confirmedT001,zero suspected/missingfeatures/new findings,
no blocker. [Persistent report](toolkit-issues.md).

## Round023 — commentary complete, ROM incomplete

ROM classification: 99.03% (64900 / 65536 bytes classified; 636 bytes unknown).
Input/output structural checkpoint round-023-disassembly unchanged. Entire accepted022
commentary archived before fresh prepare in round-023/commentary-prior; archive
manifest equals preparation-original.2732 exact physical records retained valid
review credit;66 new/split records reviewed.2798records/2993units,124 current local
contracts,zero pending records/bytes. Complete current-source semantic assessment
uses accepted scope,changed dependencies and specific old defects; no readability
range deferred. [Assessment](round-023/readability-assessment.md).

New algebra/compare/exchange,state9,palette,serial,savedcontacttarget,carrier and
EB/EC/ED skip routines explained and named. Palette index3 reaches sharedc016 and
B0 final rotate loop wraps256. Serial DEC/JR Z loops only on zero; SC-busy early
return/softwareflag polling do not prove a working protocol or handshake writer.
Shared class-parent type33/selector0-only prose now covers types33/34/35 selectors
0/1/2,count4/5/2 fixedchild37 slots and bounded commands. New animation29/37 FE/FF
objects,payload parity,NR51 channel1 mask77,sound4/25 noise E0/FF objects and pitch
7/9/10 scoped markerloops explained. Three older ignored05 packedheader claims
corrected; animation37 reuses existing payload with oddBD attributes retained.
EA27/EA2A old raw/extent claims corrected. Three older address-only branch aliases
named. Explicit high-E0 transitions distinguished from coordinate-bound placements;
new descriptors/layout/rawblocks explain fields,domains and bank context.

Fresh configured-release commentary verify byte_identical=true for65536bytes;
independent label-only source index matches65536bytes; reading refreshed. All old
review references remapped physically. No direct numeric memory operands,generic
round/branch aliases or Unknown labels on classified bytes. ROM/map/state/config/
configuredrelease hashes protected; fresh frozen state/map equal current. Final
editorial/archive acceptance remains pending; no final audit invoked.
[Completion/verification](round-023/commentary-source-completion.json),
[index](round-023/final-index/source-index.json),[archive](round-023/prior-manifest.json).

No runtime/input/snapshot/externalresearch/classification/toolkit changes. Existing
natural reachability,timing,artwork/serial purpose and pitchcount255 uncertainty
preserved,along with dual-use Code/wave data. Local script assertions and existing
animation37 payload alias reuse corrected before successful source build; local
orchestration matters,not toolkit defects. Next redirect,with no newly proved
natural route and no automatic repeat of unchanged FI022. ExistingT001 only,
zero new/suspected/missingfeatures,no blocker;[toolkit report](toolkit-issues.md).

## Round023 — redirect complete; ROM incomplete

ROM classification99.03% (64900/65536),636Unknown in84ranges; perbank302/103/119/112.
Input/output structural checkpoint round-023-disassembly unchanged. Current-source
commentary2798records/2993units pending0 retained; final editorial/archive acceptance
pending. New concrete editorial debt found: AudioRoutingMaskChannelZero has stale
"Unclassified channel1" sentence although mask77 is accepted immediately below.
Preservedsource; remove/update in authorized editorial work then reverify/refresh.

Fresh configured-release128newCode-start contextual checks have no Unknown direct
successor. Reviewed full utility/serial/contact contracts, explicitE0 transition
and ordinary resolver difference,28contextdescriptor closure, parent34/35 child37
setup/command/movement and speed0..4 closure, separate noise4/25 finite graphs and
pitch7/9/10 count0initialized FEFF loops. No new structural producer/extent proof;
all accepted proof-role bytes known. Reassessed independent remaining selector,
layout,frequency,ending,graphics and natural-route barriers; no improved justified
method and no unchanged failed route replay/arbitrary Unknown or RAM sweep.
[Detailed evidence/missing facts](redirect/round-023/findings.md),
[exact reviewed source](redirect/round-023/reviewed-source.txt),
[closure](redirect/round-023/connected-closure.json).

No new/materially improved ready structural leads. leads.json preserves all prior
entries and appends sixattempt histories plus empty round023redirect result.
No productive research frontier interrupted; this completes only redirect.
Threshold636<=16384 and636/65536<5% ordinarily eligible for finalinvestigation,
but completed FI022-20261018-v1 final969 already covers these residual barriers;
its969proposals accepted and no changed method here warrants automatic repeat.
Changedcheckpoint/round alone insufficient. Do not run unchanged disassembly cycle.

Fresh map.load/map.validate ok=true; all13protectedROM/state/map/workspace/release/
source/reading hashes unchanged; accepted commentary source hashes match.
No map/source mutations, save/export,new checkpoint/rebuild, emulator/input/snapshot,
externalresearch/toolkitchanges. Existing accepted65536byte-identical disassembly and
commentary reports retained as fidelity evidence; no fresh rebuild claim.
[Verification](redirect/round-023/verification.json),
[compact handoff](round-023-redirect-handoff.md).
Toolkit existing1confirmedT001,0new/0suspected/0missingfeatures,no blocker;
[report](toolkit-issues.md). Future restores reassert8buttons. Only stock.sym portable.


### Round023 commentary follow-up — routing-mask defect closed

Corrected the stale channel1-unclassified sentence at bank_002.asm:589 to identify
adjacent ChannelOneRoutingPreserveMask; updated its region/unit review summaries.
Reviewed the six-entry table and its merge/swap consumer comments for consistency.
No line counts, operands, labels, physical ranges or review status changed. Prior
whole-current-source readability assessment remains valid with this concrete defect
closed; 2798 records/2993 units reviewed, zero pending readability records/bytes.
Fresh configured-release verify passes byte_identical=true for all 65536 bytes;
reading-after.asm refreshed. Current physical index and review references remain
valid without remapping; its derived indexed-source comment was synchronized.
ROM/state/map/workspace/release, frozen baseline and prior archives protected.
[Follow-up evidence](round-023/editorial-followup-verification.json).
No structural/runtime/toolkit work or final audit performed. Classification remains
64900/65536 (99.03%),636 Unknown; final editorial/archive acceptance remains pending.
Existing T001 only, no new/suspected/missing-feature issue or blocker;
[toolkit report](toolkit-issues.md). No automatic FI022 repetition warranted.

### Post-round023 continuation revalidation
The previous goal turn made substantive progress:969Static bytes classified, commentary completed and fresh redirect exhausted new supported continuations. Current revalidation confirms64900classified/636Unknown, zero ready leads and exact current verified source/ROM hashes. No new selector/producer/extent fact or concrete improved method addresses FI022 residual barriers; completed FI022 must not repeat unchanged. This is the second consecutive goal turn ending at this same evidence impasse in the resumed run. Goal remains active pending blocked-audit threshold. No map/source/runtime/toolkit edits. Existing T001 only; no new findings (analysis/toolkit-issues.md). Exact audit:analysis/redirect/round-023/continuation-revalidation.json.

### Third consecutive post-round023 evidence-impasse audit
Current authoritative state again has64900classified/636Unknown and no ready lead; latest redirect unchanged. Verified commentary source and original ROM hashes still match. Prior editorial defect is closed per round-023-commentary-followup-handoff.md despite the historical redirect pass retaining its original pending note. No changed source-selector/producer/extent fact or useful concrete method addresses completed FI022 residual barriers. Three consecutive goal turns now satisfy the blocked threshold in this resumed run; goal marked blocked, not complete. No map/source/runtime/toolkit changes. Existing T001 only, no new toolkit findings; analysis/toolkit-issues.md. Exact audit:analysis/redirect/round-023/continuation-revalidation.json.

## Iterative final-investigation migration (2026-10-18)

Coordinator/workflow/final-investigation instructions now support repeated residual
investigation using previous handoffs, method planning and preserved attempt history.
Round023 and FI022 remain evidence inputs, not an automatic prohibition on developing
a materially different method. Next resume plans alternatives from those residual
barriers; unchanged failed tests remain excluded. No investigation was launched and
no ROM/map/state/source/review or verification artifact changed.

## FI023-20261018-residual-v2 — completed iterative residual investigation

Re-read current iterative prompt/workflow/goal and latest023 redirect/commentary follow-up. Original top-level FI022 final969 handoff copied intact to analysis/final-investigation/legacy-FI022-final969/handoff.md before latest-index conversion; all original proof/ledger artifacts retained unchanged. Unique attempt evidence: analysis/final-investigation/FI023-20261018-residual-v2/.

Exact internal-map complement recomputed:64900/65536 classified (99.029541015625%),636 Unknown/84 ranges, banks302/103/119/112. Every range has current context/neighbors/history/disposition. Proposed29 Static Data, accepted0: physicalac42..ac43 andad1a..ad34, fixed0/switch2 CPU6c42 and6d1a. Producer recurrence proves credit index8 is drawn at phase1 BEFORE phase17 cp8 exit, then erased later in state9. Exact count3/FF runs9/2/6 tile bytes terminate at independent Codead35. Prior supplement.py used range(8); source/map discrepancy triggered instruction-based revalidation. Abstract recurrence is static only, no runtime/natural claim. If accepted,607 Unknown remain,82 ranges, banks302/103/90/112. Current2798records/2993units pending0 readability retained; final editorial/archive acceptance pending.

Continued independent methods after first proposal: six ending-initializer literal bank/source/count copies cover no residual; whole-shape-container partition matches boundaries but count4 at indices15/16 yields side counter1 and odd-parity subtract2 loop, preventing full terrain-container certificate; field-only readers remain possible.11-bit frequency values do not certify array allocation length. Exact method changes, discriminators/results and remaining prospects in method-plan.md/method-outcomes.md; proofs and ledger retained. No arbitrary selector/RAM/whole-ROM sweep or unchanged failed route. New lead FI23-ENDING-CREDIT-BLOCK8 ready with exact recipe/history. Next disassembly, ALL-current-source commentary, redirect, then iterative residual investigation; not terminal exhaustion or whole-ROM completion.

Configured-release map.load/map.validate passed;207 targeted contextual instruction checks, protected ROM/map/state/source/reading/workspace/release hashes unchanged. No map/source mutation/save/export/new checkpoint/generation/rebuild; existing65536-byte-identical accepted reports remain fidelity evidence. No emulator/input/snapshot/debugger/external research or new gameplay route. Existing toolkit confirmed T001 only,zero new/suspected/missing-feature findings/no blocker; analysis/toolkit-issues.md. Evidence/identity/completion and compact handoff are in the attempt directory; latest index analysis/final-investigation/handoff.md. Only stock.sym distributable.

## Round024 disassembly — FI23 ending credit block8 accepted

ROM classification: 99.07% (64929 / 65536 bytes classified; 607 bytes unknown).

Input round-023-disassembly and accepted023commentary2798records/2993units pending0;
output round-024-disassembly; phase complete, next commentary. Consumed
FI23-ENDING-CREDIT-BLOCK8: verified exact29StaticData, zeroObserved, two new regions.
Independent configured-release207 contextual responses match; literal branch review
and fresh abstract byte recurrence draw0..8/erase0..7, state6 exit after4130 calls.
Index8 draws at phase1 before phase17 cp8 exit; states7/8 preserve it, state9 phase36
erases it. Word physicalac42/CPU6c42 -> CPU6d1a bank2; count3 addressed FF runs
9/2/6tiles to9b45/9b89/9bc7 terminate exactlyphysicalad35/CPU6d35 acceptedCode.
Prior range(8) extraction was a processing limit, not a supported selector domain.
[Independent audit](round-024evidence/audit.json),[API checks](round-024evidence/verification-rpc.json).

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|10278|5731|73|16082|302|
|01|8413|7801|67|16281|103|
|02|2992|13265|37|16294|90|
|03|227|15356|689|16272|112|

TotalCode21910/Data42153/Padding866; all2716regions have role/evidence. All2714prior
objects/metadata and unrelated state preserved exactly. ObservedCode15032bytes
unchanged; structural classification is separate from execution coverage. Exact
607-byte complement in82ranges matches FI023 projected residuals:
[Unknown ranges](round-024-unknown-ranges.json). Accepted023source and reading copy,
ROM, workspace configuration and configuredrelease hashes preserved.
[Preservation](round-024evidence/preservation-check.json).

Sequential map.validate/save analysis/state.json/export maps/current.sym
include_sym=false passed. Fresh `python3 run.py checkpoint round-024-disassembly`
passed validation, stock generation and byte_identical=true for all65536bytes.
MapSHA2568055aa2c6f8a255518ee939a27a063d649adee45e517660fc1519d78a9d48b95.
[Verification](round-024-disassembly-verification.json). Rebuild fidelity does not
classify remaining unknown bytes; ROM remains incomplete.

No runtime/input/snapshot/debugger patch/external research or new natural route.
Reused valid saved evidence; unchanged failed gameplay not replayed. Connected
credit graph closes at accepted renderer/eraser and directVRAM tile IDs; no further
command/actor/copy selector. Six literal ending initializer copies cover no residual
byte. Shape terminal count4 has the recorded odd side-counter obstruction; 11-bit
frequency values do not prove allocation length. No further supported extension
without materially improved residual method; retain FI023 fullledger/history.

Next commentary must complete ALL current-source readability while preserving sound
2798record/2993unit credit, carrying two new regions and correcting prior descriptions
that state6 draws only0..7 or advances before drawing8. Also resolve the recorded stale
channel1-unclassified sentence in AudioRoutingMaskChannelZero. No source editing here.
Then redirect and iterative final investigation while eligible607Unknown remain;
this pass is not a terminal handoff. No other phase/goaltools/toolkit edits/builds.
Toolkit: existing confirmedT001 only, zero new/suspected/missing-feature findings,
no blocker. [Persistent report](toolkit-issues.md).

## Round024 commentary — complete current-source readability

Input/output structural checkpoint round-024-disassembly unchanged. Classification
64929/65536=99.07%,607Unknown/82ranges. Entire accepted023 commentary (including
channel1 follow-up) archived before fresh configured preparation. All2798records/
2994units reviewed,zero pending;2796same-kind records and2993prior units retain valid
physical credit. Two29byte credit extents independently reviewed; added pointer unit.
State6 index8 phase1 draw-before-phase17-exit and state9 phase36 later erase corrected
in source/directory/reviews. Count3 addressed9/2/6FF runs end exactly at ad35Code.
Natural ending/artwork meaning unproved; no new runtime,input,research or scenarios.

Fresh configured-release verify rebuilt byte-identically65536, reading/index refreshed,
6040source references remapped; emitted statements identical to accepted023.
ROM/map/state/configuration/release/frozen baseline and prior archive protected.
[Assessment](round-024/readability-assessment.md),[verification/protection](round-024/commentary-source-completion.json),
[handoff](round-024-commentary-handoff.md). No remaining readability ranges; final
editorial/archive acceptance remains reserved,no audit run. Commentary complete;
coordinator may launch redirect, then iterative FI using current residual method
history. No other phase/goaltools run by worker. Toolkit existing confirmedT001 only,
zero new/suspected/missing-feature findings,no blocker;[report](toolkit-issues.md).

## Round024 redirect — complete; iterative residual planning next

Input/output round-024-disassembly unchanged; current-source readability2798records/
2994units,zero pending.64929/65536classified=99.07379150390625%,607Unknown/82ranges;
bankUnknown302/103/90/112. Fresh available-source frontier search saved508matches;
affected credit directory/renderer/eraser and raw shape/ordinarylayout/frequency/
bank3 gaps inspected against FI023 history. Ninth block creates only literalVRAM
writes, no actor/audio/bank/copy selectors; adjacent isolatedC9 lacks caller.
No new/materially improved ready leads or interrupted productive redirect frontier.
Prior attempts preserved; threelead histories appended,empty result recorded.
[Findings/missingfacts](redirect/round-024/findings.md),
[exact82ranges](redirect/round-024/unknown-ranges.json),
[compacthandoff](round-024-redirect-handoff.md).

Next coordinator phase MUST be iterative final-investigation method planning:
607<=16384 and607/65536=0.0092620849609375<=0.05. FI023 accepted29bytes is not a
terminal stop. Compare concrete alternative methods against FI023/FI022 barriers;
do not rerun unchanged recurrence/copy/shape/frequency proofs or failed routes.
No empty disassembly/commentary cycle,otherphase or goaltools run here.

Fresh run.py inspect originalROM identity and configured-release api.describe/
map.load/map.validate pass; protectedROM/state/map/workspace/release/source/reading
hashes and accepted commentary source hashes match. No map/source save/export,
checkpoint,rebuild,runtime,input,snapshot,debugger,externalresearch/toolkit change.
Retained accepted65536byte-identical reports remain fidelity evidence; no fresh
rebuild claim. [Verification](redirect/round-024/verification.json). Existing
confirmedT001 only,zero new/suspected/missingfeatures/no blocker;
[toolkitreport](toolkit-issues.md). Only stock.sym distributable; ROM incomplete.

## FI024-20261018-contract-domains-v1 — completed26byte Static Data proposal

Input/output round-024-disassembly unchanged:64929/65536classified=99.07379150390625%,607Unknown/82ranges,banks302/103/90/112. Accepted0,proposed26StaticData;projected581Unknown/85ranges,banks276/103/90/112. Current readability2798records/2994units,pending0 retained.

M4/M5 exact screen-major contract corrects inherited FI022/FI023 contiguous-width53*5 arithmetic. Root3966 consumes265 sparse cells within275byte envelope; last consumed3a76,ten interleaved holes excluded. Coindexed independently rooted five-entry stage2 width/layout arrays and explicit column<53,cross<5 guards support conditional width358c plus25cells. Natural ordinarynested1 remains unproved;known transition increments to2. Allfull-byte rawID downstream bank3blocks alreadyclassified. M1 separates bounded count4ROMcopy from later nonterminating scratch loop but supplies no new shape ownership;no shape proposal. Destination-budget and universal shared-cardinality alternatives underdetermined.

[Handoff](final-investigation/FI024-20261018-contract-domains-v1/handoff.md),[manifest/proof](final-investigation/FI024-20261018-contract-domains-v1/screen-major-layout-proof.json),[everyrange ledger](final-investigation/FI024-20261018-contract-domains-v1/range-ledger.json),[methods/outcomes](final-investigation/FI024-20261018-contract-domains-v1/method-outcomes.md),[completion](final-investigation/FI024-20261018-contract-domains-v1/completion.json). LeadFI24-SCREEN-MAJOR-NESTED1ready;priorlead/FIhistory preserved. Completed attempt not terminal:next independentdisassembly,ALLcurrent-source commentary,redirect,iterativeFI. Do not repeat unchanged prior methods or contiguous-end error.

Configuredrelease map.load/map.validate and385targeted structure lengths pass;protectedROM/map/state/source/reading/configuration/release hashes unchanged. No runtime,input,snapshot,debugger,externalresearch,map/source/toolkit edits,save/export,generation,newcheckpoint/rebuild. Existing65536byte-identical fidelity reports retained. Existing confirmedT001only,zero new/suspected/missing-feature findings,no blocker;[toolkitreport](toolkit-issues.md). Only stock.symdistributable;ROMincomplete.

## Round025 disassembly — conditional screen-major nested1 accepted

ROM classification: 99.11% (64955 / 65536 bytes classified; 581 bytes unknown).

Input round-024-disassembly and accepted024commentary2798records/2994units,pending0.
Output round-025-disassembly; disassembly complete, next commentary. Consumed
FI24-SCREEN-MAJOR-NESTED1 verified:26StaticData,zeroObserved,six new regions:
358c+1,3a56+13,3a65+3,3a6a+3,3a6f+3,3a74+3. Independent literal repeated-subtraction
and repeated-addition simulates all five contexts, rather than copying a contiguous
extent assumption. Queue uses stage&3/nested width, column<53; activates cross0;
consumer completes after cross5. Resolver uses identical stage/nested globals and
25*(column//5)+5*row+column%5. Stage2 directory roots351d/358b, independently rooted
stage1 starts3527/3590, and all five pointer/width pairs establish the specific
conditional structure. Widths53/53/53/10/10 give exact five screen envelopes;
accepted ordinary nested0/2 corroborate sparse consumption. This is structural
ownership through consumer contracts, not adjacency or a maximum observed selector.

Nested1 has265cells within275bytes;25new cells pluswidth358c accepted. Ten holes
3a63..64,3a68..69,3a6d..6e,3a72..73,3a77..78 remain Unknown. Source fixedbank0;
resolver temporarily switch1, wrapper restores3 for full-byte rawID payloads
CPU7320+16*ID,physicalf320+16*ID. All18rawID payloads alreadyclassified; no new
connected payload frontier. Transition mask3f does not apply to ordinary resolver.
Natural ordinarynested1 selection remains unproved; known transition increments1
to2 before ordinary queue. No runtime/input/snapshot/externalresearch or unchanged
failed gameplay replay. [Independent proof](round-025evidence/audit.json),
[385context responses and sequential mutation](round-025evidence/rpc.json).

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|10278|5757|73|16108|276|
|01|8413|7801|67|16281|103|
|02|2992|13265|37|16294|90|
|03|227|15356|689|16272|112|

TotalsCode21910/Data42179/Padding866;2722regions. All2716prior objects/metadata
and unrelatedstate preserved exactly; accepted024source/reading,ROM,release and
configuration hashes unchanged. ObservedCode15032 unchanged. Exact581Unknown
in85ranges matches FI024 projected residuals; [complement](round-025-unknown-ranges.json),
[preservation](round-025evidence/preservation-check.json).

Sequential map.validate/save analysis/state.json/export maps/current.sym with
include_sym=false passed before fresh `python3 run.py checkpoint round-025-disassembly`.
Stock validation/generation/verification passed byte_identical=true for65536bytes;
mapSHA2569e884470ae4551c0b8e50e0a8ad1d14db8807d5c6779eea4f8f5886d712c624f.
[Checkpoint](round-025-disassembly-verification.json). ROM incomplete; fidelity
and classification remain separate gates. Local verifier initially named next
independently rooted directory stage3 instead of actualstage1 and compared the
residual wrapper object to a bare list; corrected locally, not toolkit defects.

Commentary next must complete ALLcurrent-source readability, preserving sound
2798records/2994units credit; carry sixnewregions and update sparse-screen geometry,
width and conditional reachability descriptions. No commentary/source edits here.
Then redirect and iterative final investigation while581Unknown remain eligible.
FI024 residual ledger/methodoutcomes remain starting points; shape copy termination
alone lacks ownership, destination tilemap budgets do not prove ROM container extent,
and unrelated indexed tables do not inherit domains. No extra supported extension
or unchanged runtime test is deferred from this connected contract.
Toolkit: existing confirmedT001 only;zero new/suspected/missing-feature findings,
no blocker. [Persistent report](toolkit-issues.md). No toolkit edits/build/substitution.

## Round025 current-source readability complete

Classification64955/65536 (99.11%),581Unknown remains incomplete. Entireaccepted024
commentary archived intact before fresh preparation; retained2994units, now2807
records/3000units reviewed,pending0. Six new width/sparse-cell structures named.
Corrected conditional ordinary nested1 geometry:265consumed cells in275-byte envelope,
53valid columns within55; ten interleaved holes remainUnknown. Natural ordinary path
unproved; known transition increments1to2 before queue. All affected dependencies
assessed; useful prior prose/credit preserved and physical source references remapped.
Fresh configured verify and independent label-only index byte-identical65536; reading
refreshed,ROM/map/state/configuration/release/archive hashes protected. No current
readability gap or deferral. Final editorial/archive audit pending,not run.
[Assessment](round-025/readability-assessment.md),
[handoff](round-025-commentary-handoff.md). Existing confirmedT001 only,
zero new/suspected/missing-feature findings,no blocker;[report](toolkit-issues.md).

## Round025 redirect — corrected-geometry residual closure

Input/output round-025-disassembly unchanged:64955/65536classified=99.11346435546875%,
581Unknown/85ranges,banks276/103/90/112; readability2807records/3000units,pending0.
Accepted26-byte sparse layout closes in classified full-ID bank3 payloads. New
changed-condition check applies actual screen-major formula across45 coindexed
rooted entries:zero consumed Unknown source cells; ten screen holes excluded.
No natural ordinarynested1 selection inferred. No new/reopened ready lead; verified
FI24-SCREEN-MAJOR-NESTED1 residual closure appended and allprior history retained.
[Findings](redirect/round-025/findings.md),[geometry](redirect/round-025/rooted-layout-geometry.json),
[handoff](round-025-redirect-handoff.md),[leads](leads.json).

Shape ownership/field-root,distinct selector-producer and independent ROMsource-end
barriers remain; no changed prerequisite for unchanged route/actor/selector/copy
retries. Next coordinator phase MUST be iterative final-investigation method planning:
581<=16384 and581/65536=0.0088653564453125<=0.05. Compare alternate methods with
FI024/FI023/FI022 history;no terminal exhaustion assertion or emptyclassificationcycle.
No other phase/goaltools run.

Fresh run.py inspect and configured-release api.describe/map.load/map.validate pass;
protectedROM/state/map/configuration/release/source/reading hashes match,accepted
commentary source hashes match. [Verification](redirect/round-025/verification.json).
No classification/source/map save/export/checkpoint/rebuild/runtime/input/snapshot/
debugger/externalresearch/toolkit change. Retained65536byte-identical reports remain
fidelity evidence;no fresh rebuild claim. Toolkit existing confirmedT001 only;
zero new/suspected/missing-feature findings,no blocker;[report](toolkit-issues.md).
Only stock.sym distributable; final editorial/archive acceptance pending;ROMincomplete.

## FI025-20261018-exhaustion-v1 — TERMINAL method exhaustion; incomplete

Current phase final-investigation completed against round-025-disassembly,
mapSHA2569e884470ae4551c0b8e50e0a8ad1d14db8807d5c6779eea4f8f5886d712c624f.
Recomputed64955/65536classified=99.11346435546875%;581Unknown/85ranges,
banks276/103/90/112. Accepted0/proposed0; earlier29credit/26layout accepted work
is preserved, not reproposed. Current-source2807records/3000units,pending0 intact.

Actively evaluated A1..A7: common-domain versus safety inference; SAME-shape
joint directory/record/box allocation and field-only slices; frequency
normalization/construction versus numerical pattern; source section/bank bounds
versus reusable metatile allocation and ending transfer identity; rounded screen
envelope versus consumed geometry and descriptor grammar; self-delimiting
parser/seed/alias objects; isolated return routine contracts. Each records prior
outcome, independent alternative, discriminator and stop rule. Source evidence
leaves those alternatives non-discriminating; no credible concrete untried method,
new proposal or productive interrupted frontier remains. TERMINAL research
exhaustion on this checkpoint, not ROM completion. Do not repeat unchanged
planning or launch an empty normal cycle. Resume only with new evidence,
materially improved concrete method or explicit reassessment request.
[Method matrix](final-investigation/FI025-20261018-exhaustion-v1/method-plan.md),
[outcomes](final-investigation/FI025-20261018-exhaustion-v1/method-outcomes.md),
[exact85range ledger](final-investigation/FI025-20261018-exhaustion-v1/range-ledger.md),
[handoff](final-investigation/FI025-20261018-exhaustion-v1/handoff.md).

Fresh run.py inspect,168 targeted contextual structure verifications and
map.load/map.validate pass.91 protectedROM/map/state/configuration/release/source/
reading/prior-evidence file hashes match; no previous completed evidence edited.
[Verification](final-investigation/FI025-20261018-exhaustion-v1/verification.json),
[completion](final-investigation/FI025-20261018-exhaustion-v1/completion.json).
No new runtime/input/snapshot/debugger/route/research or map/source/release/toolkit
change,save/export,checkpoint or rebuild. No artificial experiment masquerades as
execution. Existing65536-byte-identical disassembly/commentary reports retained;
no fresh rebuild claim. Final editorial/archive acceptance remains pending;
classification and fidelity remain distinct. Only stock.sym distributable.
Existing confirmedT001 only;zero new/suspected/missing-feature findings,no blocker;
[toolkit report](toolkit-issues.md). No restoration; future all8inputreassertions remain.


## Archive preparation and spelling correction — 2026-10-18

Renamed workspace and original local ROM filename to castlevania-adventure; corrected all internal UTF-8 spelling/path references. ROM bytes and assembly unchanged. Historical checkpoint hashes were not rewritten to imply new verification of historical text. Reused complete round025 readability reviews for final editorial acceptance; 581 Unknown bytes remain. See commentary/editorial-audit.md (from analysis: ../commentary/editorial-audit.md). Archive validation follows; no new toolkit issues found.


## Archive restoration and current-prompt preparation — 2026-10-26

Restored the archived Round 25 source exactly with the matching local ROM; archive restoration passed exact_source=true and byte_identical=true. Prepared current toolkit Game Boy goal/workflow/evidence policy and disassembly/commentary/redirect prompts. Retained archived maps, internal state, analysis/lead history, exact edited source and completed editorial review credit. Original archive metadata and documents are preserved under archive-origin/. A fresh byte-identical frozen baseline was generated from the restored map.

Classification remains 64955/65536 bytes (99.113464%); 581 Unknown bytes across 85 ranges. Round 25 readability and archived final editorial acceptance are complete. Next phase: Round 26 disassembly under prompts/disassembly.txt. This explicit continuation setup authorizes reassessment under the latest evidence policy, including practical role inspection and bounded generic-data fallback; preserve sound findings and do not repeat unchanged exhausted experiments. Preparation has not started gameplay, research or classification.

Historical terminal method-exhaustion findings remain preserved. Reevaluate their premises under current policy rather than treating archived completion or byte fidelity as classification proof. Missing scratch evidence/checkpoints/traces/snapshots were excluded by archive policy; recover needed premises from available source/map/records. The historical T001 input-restoration workaround remains documented in analysis/toolkit-issues.md; reassert all eight inputs after snapshot restoration unless independently verified fixed.

## Round026 disassembly — generic non-code reassessment (in progress)

Input restored Round25:64955/65536 classified,581 Unknown/85ranges. Explicit
current-policy reassessment separates generic non-code classification from original
allocation, selector coverage, semantics and reachability. Inspected every physical
hole, neighboring accepted regions, associated consumers and historical scoped
rejections. Individual rationale/bounds/limits are retained in
[ledger](round-026/classification-ledger.json), with [physical context](round-026/context.txt).
Fresh decode audit of11394 accepted-code instructions found no ordinary fallthrough
into Unknown. Its one conservatively bank-expanded target to bank3 f505 is actually
the bank1 player-hazard callback7505; no supported bank3 entry is established.
[Audit](round-026/conflict-audit.json). Five isolated C9 bytes remain ambiguous.

Accepted576bytes/80regions using configured-release map.annotate, Static generic
Data with per-region explanatory metadata; prior classifications untouched.
[Annotation results](round-026/annotation-results.json). Saved state/exported map;
portable map hash unchanged because raw fallback emitted the same directives before.
Fresh [checkpoint verification](round-026-disassembly-verification.json) byte-identical.
ROM classification: 99.99% (65531 / 65536 bytes classified; 5 bytes unknown).

Fresh broad gameplay ongoing. Initial unsynchronized stage patches are rejected as
stage-initialization evidence; snapshot was inside active setup. Corrected replay
stops at DispatchGameFrame0433 before state writes, and verifies resultant state,
descriptor, frame and execution. Initial attempt preserved, not counted as visits.
[Corrected replay](round-026/safe-gameplay.py). Phase not yet handed off.

## Round026 disassembly — completed phase handoff

ROM classification: 99.99% (65531 / 65536 bytes classified; 5 bytes unknown).
Per-bank code/data/padding/unknown:bank0 10278/6029/73/4;
bank1 8413/7904/67/0;bank2 2992/13354/37/1;bank3 227/15468/689/0.
All80newgenericData regions carry role/evidence/comment metadata and Static provenance.
Existing observed/static classifications retained;annotation and execution coverage
are separate. Five isolatedC9bytes remainUnknown; wholeROM incomplete.

Fresh initialized gameplay visits now include stages1/2/3/4,distinct field/cavern/
spikes/masonry scenes;source-supported completion/shake/scroll shortcuts reach
actual ending and visible STAFF credits. Captured2816unique physical ROM opcode
addresses in selected traced segments;zero newData conflict/Unknown fetch.
Boss defeat/natural late-stage reachability/final held-A screen remainunclaimed.
[Checklist](scenarios.json),[runtime audit](round-026/runtime-audit.json).
Rejected setup attempts preserved;shorter synchronized replays supersede them.

Configured-release map.save/export/validate and fresh checkpoint
round-026-disassembly byte-identical65536bytes;[verification](round-026-disassembly-verification.json).
[Counts/hashes](round-026/classification-counts.json). No ROM/toolkit/source changes.
ConsumedR26-GENERIC-NONCODE-REASSESSMENT verified;producedR26-ISOLATED-RET-RESIDUAL
deferred,notready. Separate commentary next: reconcile80genericData/sourceUnknown
claims,assess allcurrent source readability,reuse reviewcredit,refresh reading copy
and verify. [Compact handoff](round-026-disassembly-handoff.md).
ExistingconfirmedT001only;zero new/suspected/missing-feature findings,no blocker;
[toolkit report](toolkit-issues.md). No finalacceptance/archive phase launched.
Final post-runtime load/validate/conflicts/protected-hash audit passed:
[final verification](round-026/final-verification.json). Disk14Gavailable,
round026evidence17MiB. Phasecomplete;nextseparatecommentary.

## Round026 commentary — completed current-source readability

All2807records/3007actualrestoredunits reviewed,6045source references checked;
zero pending readability records/bytes or deferred ranges. Historical3000unit phase
count predates seven restored editorial units; all actualinputunits retained.
Entire preceding commentary archived before fresh baseline preparation; carried
source preserved. All80newgenericData ranges/576bytes locally explained,stale
Unclassified labels renamed,affected lookup/layout/audio/child/state dependencies
assessed. Corrected broader table/domain claims without inventing schema/semantics;
reconciled older review-only Unknown claims as concrete backlog defects.

Fresh configuredverify and independent index assembly byte-identical65536; reading
refreshed; emitted statements unchanged except consistent labels. ProtectedROM,
state,map,config,release/prior hashes match. Classification65531/65536,fiveUnknown
C9 bytes remainsincomplete. [Assessment](round-026/commentary/readability-assessment.md),
[checks](round-026/commentary/verification.json),[handoff](round-026-commentary-handoff.md).
Next separatephase redirect; no redirect/finalaudit/archive acceptance launched.
ExistingconfirmedT001only;zero new/suspected/missing-feature issues,no blocker;
[toolkitreport](toolkit-issues.md). All internal assets/evidence/source remainlocal.

## Round026 redirect — corrected accepted-directory return contract

Input remains round-026-disassembly:65531/65536 classified,5Unknown(singleC9 at
0728,2f09,3420,34af,ad6a). Current-source readability gate complete. No map/state,
ROM/source/toolkit/runtime mutation. New unique attempt
[assessment](redirect/R26-20261027-dispatch-contracts-v1/assessment.md) preserves
changed premise, primary byte/table/source evidence and per-residual scoped review.

The prior no-contract conclusion omitted accepted GameStateHandlers word7 at045d,
which selects0728. Its ten-entry established directory and independently traced
CALL/RST/POP/JP stack contract support a one-byte empty main-state7 handler.
Produced readyR26-MAINSTATE7-EMPTY-HANDLER:proposed1byte StaticCode,accepted0;
natural state7 production/execution unclaimed. Independent disassembly review next.
Other4 remain ambiguous after same-standard consumer/stack/bank inspection;
3420's33ae pointer cell is genericData with no independent runtime0 membership
proof, not equivalent to accepted main-state directory. No remaining concrete
productive continuation identified; no arbitrary calls or unchanged gameplay replay.
Broad stage1..4/ending/credits evidence reused; boss/final-held-A gaps remain explicit,
with no identified link to residuals. No terminal exhaustion/finalacceptance claim.
ExistingconfirmedT001 only;zero new toolkit findings,no blocker. Disk14GiBfree.
[Compact handoff](round-026-redirect-handoff.md);[leads](leads.json).

## Round027 disassembly — completed single pass

ROM classification: 99.99% (65532 / 65536 bytes classified; 4 bytes unknown).
Per-bank Code/Data/Padding/Unknown:bank0 10279/6029/73/3;
bank1 8413/7904/67/0;bank2 2992/13354/37/1;bank3 227/15468/689/0.
Accepted R26-MAINSTATE7-EMPTY-HANDLER:one Static Code byte0728 after independent
accepted ten-word directory, exact raw pointer and CALL/RST/POP/JP stack review.
No Observed or natural selector7 claim. [Assessment](round-027/assessment.md),
[primary review](round-027/primary-review.json),[counts](round-027/result.json).
Prior blanket no-contract conclusion corrected only for0728;remaining2f09,3420,
34af,ad6a(bank2CPU6d6a) remain structurally ambiguous. Reused sound scoped review;
no arbitrary call or unchanged gameplay replay. Broad Round026 stage1..4/ending/
credits evidence reused;boss/final-held-A/natural late-game claims remain absent.

Configured-release map.load/annotate/save/export/validate/conflicts and fresh
round-027-disassembly checkpoint byte-identical65536bytes pass;
[verification](round-027-disassembly-verification.json),[final audit](round-027/final-verification.json).
ROM/config/release/commentary source unchanged;previous checkpoint/state/map preserved.
ExistingconfirmedT001 only;zero new toolkit findings,no blocker;[report](toolkit-issues.md).
WholeROM incomplete;single disassembly pass complete,next separate commentary.
[Compact handoff](round-027-disassembly-handoff.md). No commentary/redirect/archive
acceptance launched. Internal evidence/source/assets stay local. Disk14GiB free.

## Round027 commentary — complete current-source readability

All2807records/3007units reviewed,6045source refs checked,zero pending/deferred
readability ranges. Preserved complete previous commentary with manifest; fresh
current-state baseline and carried authoritative source. Static0728 is now named
EmptyMainState7Handler with RET, symbolic directory word, direct outer-return
stack explanation and affected state/credit dependencies assessed. No new natural
selector7/fetch claim. Four residualUnknown bytes unchanged;65532/65536classified.
Fresh configured-release verification plus independent indexed assembly both
byte-identical65536bytes; full physical scaffold/state agreement checked and reading
copy refreshed. [Phase evidence](round-027/commentary/verification.json),
[assessment](round-027/commentary/readability-assessment.md),
[compact handoff](round-027-commentary-handoff.md). ExistingconfirmedT001 only;
zero new toolkit findings/no blocker. No toolkit/ROM/map/state/runtime changes;
no redirect/finalaudit/archive acceptance launched. Internal artifacts local.

## Round027 redirect — ready inline-origin runtime0 proposal

Corrected independent origin reasoning: RST00 at33ad explicitly anchors33ae;
zero-index3420 pointer plus inherited outer CALL stack supports one-byte empty
handler without a complete directory or naturally produced selector. Current entry0
pair remains Static genericData pending independent role review. Proposed1byte,
accepted0;4Unknown unchanged. All3 accepted terminalJP/JR+RET analogs independently
entered and Observed; none establishes detached epilogue convention for other3.
[Assessment](redirect/R27-20261027-origin-and-epilogue-v1/assessment.md),
[compact handoff](round-027-redirect-handoff.md),[ready lead](leads.json).
No ROM/map/state/source/runtime/toolkit edits. ExistingconfirmedT001 only;zero new
issues,no blocker. No terminal exhaustion/final acceptance requested.

## Round028 disassembly — inline-origin runtime0 handler verified

ROM classification: 100.00% (65533 / 65536 bytes classified; 3 bytes unknown).
Rounded100.00% does not establish completion. Accepted3420 length1 StaticCode from
independent RST33ad/base33ae/word0/outerCALL stack/exact-boundary review. No natural
occupiedtype0 or Observed claim;33ae genericData pair unchanged, independently supported
pointer0 interpretation documented without whole-table expansion.
Per-bank Code/Data/Padding/Unknown:0 10280/6029/73/2;1 8413/7904/67/0;
2 2992/13354/37/1;3 227/15468/689/0. Execution coverage unchanged/separate.
Remaining2f09,34af,ad6a(bank2CPU6d6a) ambiguous under reused scoped Round027 primary
review; no detached epilogue convention or changed productive test. Broad stage1..4/
ending/STAFF routes reused; explicit boss/finalheldA/natural progression limits retained.
[Assessment](round-028/assessment.md),[primary review](round-028/primary-review.json),
[counts](round-028/result.json),[handoff](round-028-disassembly-handoff.md).
Fresh configured-release save/export/validation/conflicts and checkpoint pass,
byte-identical65536bytes;[verification](round-028-disassembly-verification.json),
[protected audit](round-028/final-verification.json). ROM/config/release/all commentary
hashes match; source unchanged. Prior map/state/history retained. ExistingconfirmedT001
only;zero new toolkit findings,no blocker;[report](toolkit-issues.md). Disk14GiB free;
no bulk scratch. One disassembly phase complete; wholeROM incomplete; separate commentary
next. No commentary/redirect/finalaudit/archive acceptance launched. Internal artifacts local.

## Round028 commentary — current-source readability complete

All2807 records/3007 units retain valid review credit;6045 source references checked,
zero pending/deferred readability ranges. Prior commentary preserved in
round-028/commentary/prior with manifest; fresh frozen map/state baseline prepared.
EmptyRuntimeTypeZeroHandler names3420; source explains type0 local word33ae and outer
CALL continuation3343 without a full directory or natural selection/execution claim.
Caller/field reader/dispatcher/axis and adjacent ending wrapper dependencies assessed.
[Assessment](round-028/commentary/readability-assessment.md),
[checks](round-028/commentary/verification.json). Fresh configured-release verify and
independent physical-index RGBDS reconstruction byte-identical65536bytes; reading refreshed.
ROM/config/release protected hashes match. Classified65533/65536;3Unknown remain
2f09,34af,ad6a(bank2CPU6d6a). WholeROM incomplete; no readability backlog.
Existing confirmedT001 only;zero new toolkit issues,no blocker;[report](toolkit-issues.md).
No ROM/map/state/toolkit edits,redirect/finalaudit/archive acceptance. Internal artifacts local.

## Round028 redirect — terminal rejection review for coordinator

65533/65536 classified; exact3Unknown2f09,34af,ad6a(bank2CPU6d6a).
Proposed0/accepted0; no map/state/ROM/source/toolkit/runtime changes. Retrieved
R27 primary source/analog audit and R26 exact word audit; scoped each negative
and compared combined constraints with accepted0728/3420 contracts. No useful
revised inference/test remains; no speculative code or genericData proposal.
[Per-byte review](redirect/R28-20261027-terminal-rejection-v1/rejection-review.md),
[fresh exact complement/protected hash audit](redirect/R28-20261027-terminal-rejection-v1/final-audit.json).
Broad artificial stages1..4/ending/STAFF evidence retained;boss/finalheldA unvisited
limits explicit, no exhaustive gameplay claim. Readability complete; existing
validated/conflict-free checkpoint and byte-identical source verification retained,
with final protected/source hash equality checked. ExistingconfirmedT001 only;
zero new toolkit issues,no blocker. Disk14GiB free,no bulk scratch. Recommendation
is incomplete pending coordinator review; no final acceptance/archive performed.

## Coordinator review — Round028 incomplete termination, 2026-10-27

Inspected scoped rejection reasoning and primary source return chains; no unsupported
caller prerequisite or overbroad negative result remains. Three isolated C9 bytes
at physical2f09/34af/ad6a remain materially ambiguous, not generic-data candidates.
No ready lead or concrete useful revised test remains. Independently checked all18
recorded protected/product hashes and direct rebuilt-ROM equality: pass.
65533/65536 classified=99.99542236328125%;3Unknown. Current-source readability
complete, fresh byte-identical checks retained; final acceptance/archive pending.
Workflow terminates incomplete, not whole-ROM completion. Preserve accepted work
and sound rejection review; no unchanged automatic research cycle.
[Coordinator audit](round-028-coordinator-review.md),
[checkpoint](round-028-disassembly-handoff.md),
[commentary](round-028-commentary-handoff.md),
[redirect](round-028-redirect-handoff.md),[leads](leads.json).
