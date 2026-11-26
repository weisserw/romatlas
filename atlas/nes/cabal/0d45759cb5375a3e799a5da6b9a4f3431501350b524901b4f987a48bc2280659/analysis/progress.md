# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 disassembly in progress
Inspection and verified byte-identical raw baseline established (maps/baseline.json, generated/baseline). Mapper7, 131088 total bytes, 131072 PRG, no CHR ROM. Persistent session uses analysis/scratch/rpc-files.py and request/response files; no map annotations yet. Initial startup ran40 frames before bounded execution timeout, 745 execution addresses recorded. Continuing gameplay/static vectors. ROM unchanged.

### Round001 intermediate checkpoint
Recovered execution using short self-contained JSON-RPC subprocess batches and durable snapshot imports/exports, after an unpersisted long-lived server exited143. No toolkit defect established. maps/pass-001-initial.json and generated/pass-001-initial verified byte-identical:7379classified bytes (7339code,40header/vectors). Natural title→Level1-1 battlefield route recorded in gameplay.md and round001 snapshots; firing/aiming/rolling attempted, currently still early battlefield. RAM08 stage-index hypothesis was patched16 before loading; resulting frame remains Level1 village, so this attempt does not demonstrate later-level entry. Investigating correct progression contract rather than repeating long inputs.

Generic non-code bodies after inspected code prefixes are now annotated Static, with256byte sample evidence, repeated packed-record/bitplane/compression patterns and PPU decompression consumers; known bank tail code excluded and existing code preserved. Current interim internal counts111763/131088classified,19325unknown (85.26%), before merging combat observations; not yet final checkpoint. Classification roles are generic data only, not claimed precise asset formats. See analysis/evidence/round001/data-inspection.json. Later levels/bosses/ending remain unvisited and require a supported RAM shortcut; no terminal exhaustion claim.

## Round001 disassembly completed — incomplete ROM

ROM classification: 89.56% (117404 / 131088 bytes classified; 13684 bytes unknown).

Internal state analysis/state.json; portable map maps/current.json; frozen checkpoint maps/pass-001.json; local source generated/pass-001; verification analysis/pass-001-verification.json. map.validate passed; generation and whole-file byte-identical reconstruction passed. Original ROM SHA256 unchanged. Code12980bytes; Data104424bytes (includes16header bytes,24vector bytes and104384generic data bytes).162unknown intervals. Detailed complement and counts: analysis/evidence/round001/classification-counts.json.

| Physical8KiB PRG bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 4965 | 0 | 3227 |
| 1 | 2844 | 3072 | 2276 |
| 2 | 0 | 8192 | 0 |
| 3 | 42 | 7926 | 224 |
| 4 | 1006 | 6144 | 1042 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 61 | 7926 | 205 |
| 8 | 3276 | 2048 | 2868 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 42 | 7926 | 224 |
| 12 | 702 | 4096 | 3394 |
| 13 | 0 | 8192 | 0 |
| 14 | 0 | 8192 | 0 |
| 15 | 42 | 7926 | 224 |

Header16bytes is outside PRG bank counts, classified Data. No CHR ROM. Physical offsets include header; mapper7 banks are effective8KiB identities, with32KiB contexts[0..3],[4..7],[8..11],[12..15]. No RAM execution upload origins attributed to ROM. Annotation evidence covers currently classified regions; semantic/editorial completion deferred to commentary. Observed-annotated ROM bytes9695 is a lower bound, not total instruction coverage or a reachability percentage; static code remains Static. Full execution roots/hits/scenario trace summaries are local round001 evidence. Zero-capacity traces retain aggregates; detail drops are intentional, aggregate_dropped=0 in captured summaries. Bounded timeouts retained partial progress and do not establish completion.

### Evidence and decisions
- Validated banked vectors and reset/NMI/IRQ stubs; discovered static branches and actual instruction fetches in explicit effective contexts. Inspected candidates and conflicts before annotation. Final bounded closure follows directJSR callee entries in unchanged bank context; ordinary return propagation remains unproved.1433bytes gained in this closure, with no overlapping instruction conflicts.
- Generic data intervals[0x3410,0x7F00),[0x8810,0xFF00),[0x11810,0x17F00),[0x19010,0x1FF00). Initial pattern samples were strengthened by full-byte scanning, every256byte chunk retained in data-full-inspection.json, and manual review of all16 maximal long official-decode runs in data-suspicious-decodes.json. These are repetitive record/load/compare/address patterns or bitplane/flag runs, not coherent routine contracts. Checked all available discovered control/data references and existing observed/static code; none conflicts within these intervals. PPU RLE/streaming consumers inspected near physical0x8400 and banked prefix loaders. Generic structural role only; full asset layouts/original table boundaries not claimed. Exact common tail code/data vicinity after eachxx7F00 remains separate unknown/code/vector slices.
- Gameplay/scenario records: analysis/gameplay.md, analysis/scenarios.json. Natural title and village display visited. Firing, aiming, roll inputs and later Start attempts recorded but active mechanics not demonstrated. RAMD2/D3 plus derived08 contract established by A456 and actual values; direct08 patch rejected because overwritten. Artificial later scenery, final5-4 display, and illustrated ending observed. Scene0 trials are explicitly invalid natural scene selectors and map to prior finalscene; do not count as natural later-level reachability. Boss-specific PC817C attempt and300frame delay still show wait/intro, so boss actor execution remains unresolved. Ending entryPC8196/SP255 with RAMD2=5,D3=4,08=38 rendered soldier/flag/text; highscore terminal not yet confirmed.

### Frontier and reason for handoff
This single easy-gain pass ends with verified structural progress and specific investigation questions; it is not a terminal exhaustion claim. Long input repetitions and first selector edits ceased providing new understanding. Further progression needs the mainloop Start/NMI wait and boss initialization contract, rather than more unchanged play. Prefix mixed code/tables, BRK bank dispatcher/selector entries, common bank-tail tables and asset boundary refinements remain useful redirect work. Full current-source commentary precedes redirect. No existing leads were supplied initially; new actionable leads are in analysis/leads.json. No toolkit modifications, publication, deletion, archival or workspace-wide commit performed.

Toolkit findings: no toolkit issues found. See analysis/toolkit-issues.md. Unexpected server exit143 and sandbox Unix-socket restriction were worked around with bounded subprocess batches and durable local snapshots; no toolkit defect established. Scratch80KiB; evidence about60MiB, disk about3.2GiB free; no files pruned.

Final consistency check:5018unique actual ROM instruction-fetch start offsets across persisted scenario contexts; zero fetch starts overlap classified Data. Evidence: analysis/evidence/round001/fetch-data-conflict-check.json. Actual final disk measurements: scratch132KiB, retained round001evidence83MiB,3.2GiB free; prior approximate scratch/evidence estimate superseded. No cleanup performed.

## Round001 commentary in progress
Prepared commentary from verified pass-001; frozen before/baseline retained. Full current-source naming and mechanical explanation underway; no review completion claimed. Source frontier: all 12980 code bytes plus data/unknown notes.

### Commentary source batch saved
Complete compact instruction inventory reviewed; 366 mechanical routine/control-unit explanations applied across all four AxROM images. Examples: audio token interpreter and four-channel RAM arrays; NMI/controller semaphore; actor parallel-array sorting and clipped metasprites; signed RLE copies; two-bit attribute merging; packed collision rectangles. Review scaffold preserved. Verification and source-boundary quality review pending; phase remains commentary.

### Round001 commentary completed — current-source readability
All 12980code bytes/6404instruction records and104424data bytes reviewed; all162unknown intervals have honest boundary notes.366routine/control units and6575preserved scaffold entries record current readability. Source contains descriptive physical-bank entries/branches, hardware/RAM/table operands and mechanical explanations; remaining state/actor identities are documented as unresolved. Full source scope finished, not an interrupted batch.

Fresh commentary/verification.json passes byte-identically; commentary/reading-after.asm refreshed. Map and internal-state baselines unchanged. analysis/round001-commentary-assessment.json and analysis/handoffs/round-001-commentary.md record scope/next questions. Classification unchanged:117404/131088classified,13684unknown. Final editorial audit pending; no whole-ROM completion claim. No toolkit issues found, no toolkit changes, deletion, publication or archive. About3.2GiBfree; scratch632KiB. Next phase belongs to coordinator: redirect, not initiated here.

Round001 redirect active, attempt analysis/redirect/round001-a/progress.md. Input pass-001; accepted classification unchanged 117404/131088 (89.56%), 13684 unknown/162 intervals. Investigating banked BRK/dispatch, post-write/return static continuations, data counterexample $E05A, and corrected progression wait premise. No output checkpoint or completed redirect handoff yet.

### Coordinator resume checkpoint 2026-11-26
Separate worker `/root/redirect_resume` is running round001-a redirect; inspect its live status before launching any replacement. Corrected bank1 directory extent and 13-entry transfer bounds are recorded in attempt contracts/progress. Reviewed graph proposes 6412 bytes (21 already-classified correction), not yet accepted; current classification remains117404/131088 (89.56%),13684unknown. Later-stage sprite0-hit barrier research and residual proposals remain active. Coordinator re-inspected archive.py document contract and wrote provisional archive-options.json selecting all current unrecognized Markdown, with no disposable scratch Markdown identified. Refresh after new phase output; no archive/cleanup executed.

### Round001-a redirect research completed
Resumed existing attempt; combined proposals ready for independent disassembly.
See analysis/handoffs/round-001-redirect.md and analysis/redirect/round001-a/contracts.md.
Accepted pass-001 unchanged:117404 classified,13684 unknown/162 intervals. Proposed
6411 unknown Code,7273 unknown Data,21-byte accepted-Data correction; zero accepted
changes in redirect. These are hypotheses requiring per-contract review, especially
JSR bank returns and two-byte990C routine prefix. Natural Start gate resolved;
four later-scene timing/initialization routes remain unsuccessful after two targeted
revised methods. Exact negative evidence/scenarios retained. No toolkit issues found;
no toolkit edits, deletion, archive or publication. Next phase disassembly, not a
terminal incomplete assessment or whole-ROM completion claim.

## Round002 disassembly completed — incomplete ROM

ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).

Rounded100.00% is not completion. Exact percentage99.99847430733553%. Code19410/Data111676; header16 counted separately from PRG banks. Sole unknown interval physical[6428,6430), CPU B0$990C..$990D, effective slots[0,1,2,3]. Independent review retained this ambiguity, not a byte quota. Full counts/complement and zero new gameplay fetch/Data conflicts: analysis/evidence/round002/classification-counts.json.

Final checkpoint **pass-002-reviewed**, maps/pass-002-reviewed.json, generated/pass-002-reviewed, analysis/pass-002-reviewed-verification.json. map.validate and byte-identical rebuild passed; state/current/frozen map byte-identical. MapSHA25668110f6fa29cc7e0d4fe1be187839d50c4b5d7151374d15cee17a354aa2785cc. ROMSHA256 unchanged0d45759cb5375a3e799a5da6b9a4f3431501350b524901b4f987a48bc2280659. Preliminary pass-002 was inadvertently checkpointed before annotation completed; preserved as117404-classified historical artifact and explicitly superseded by pass-002-reviewed. No overwrite/deletion.

| Physical8KiB PRG bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 7388 | 802 | 2 |
| 1 | 4299 | 3893 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 97 | 8095 | 0 |
| 4 | 1460 | 6732 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 77 | 8115 | 0 |
| 8 | 4762 | 3430 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 77 | 8115 | 0 |
| 12 | 1173 | 7019 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 0 | 8192 | 0 |
| 15 | 77 | 8115 | 0 |

Accepted connected proposals after fresh exact decoding, bank/return/stack and consumer review:6412graph Code bytes (6391 previously unknown plus21-byte E05A correction),18tailJMPbytes,7273Data bytes. Net13682 newly classified. R001-brk-dispatch and R001-data-prefix-tails consumed/verified with localized exception. Preserved old genericData evidence in split E05A flanks and all historical research. All additions Static; no fabricated Observed/natural selection. See analysis/evidence/round002/review.md, return-audit.json, cursor-primary.json and accepted.json. Return fixed point is a structural returning-path check under entry/wait preconditions, not proof all runtime states terminate.

Two-byte990C claim remains Unknown: live-Y shared tail and matching LDYA4 are compatible with a prefix but do not independently discriminate its entry boundary from constants. No natural-caller requirement imposed. R002-cursor-prefix-990C records exact scope and reopening conditions. Whole graph was not accepted merely on proposal arithmetic.

Reused natural unpaused snapshot with release/right-fire/up-roll/left-fire, four requests10frames each,1500000operations/8000ms; all hit timeout with retained partial progress.2972contextual roots, no new unknown fetches or Data conflict. No40-frame completion or later combat claim. Later stage/boss/terminal milestones remain attempted/deferred under prior targeted negatives; full render/actor-initialization prerequisite still missing. No unchanged RAM sweeps or failed route repeats. Scenarios/gameplay records updated.

Next phase full current-source commentary, then redirect review of isolated ambiguity and stopping rationale. This disassembly handoff is not terminal exhaustion or editorial completion. No toolkit issues found; analysis/toolkit-issues.md preserved. No toolkit edits, archive, publication, deletion or workspace commit.

## Round002 commentary completed — current-source readability

All19410Codebytes/9559instructions and111676Databytes have current mechanical explanation; sole Unknown[6428,6430) explicitly documented. Entire previous commentary preserved at analysis/round-002/commentary-round001 before new baseline.366prior unit starts retained,50summaries corrected;594meaningful units overall (521code,58data,15fallback),9629scaffold entries. High-score/name editor, auxiliary line trajectories, full audio commands, packed boss records, animation channels, BRK+1/mapper restore, E05A and transfer tables explained. No readability ranges pending.

Fresh commentary/verification.json passes byte-identically; reading-after.asm refreshed. ROM/map/state/frozen baseline unchanged. Scope assessment analysis/round002-commentary-assessment.json; handoff analysis/handoffs/round-002-commentary.md. Classification stays131086/131088 exact99.99847430733553%,two Unknownbytes. Final editorial audit reserved for coordinator. New semantic question:8F87 compares RAMB0 with itself before ordering six score digits; no speculative patch. Existing990C structural ambiguity/later-stage initialization negatives retained. No toolkit issues found; no toolkit changes, deletion, archive, publication, runtime research or commit.

## Round002-a redirect completed — combined proposal ready
Accepted pass-002-reviewed unchanged131086/131088 exact99.99847430733553%,2Unknownphysical[6428,6430),B0$990C..D. New independently anchored98BC..98C5 cursor-update family plus exact terminal boundaries supports revised2-byteStaticCode proposal; independent review pending. Prior localized rejection preserved; no global cursor invariant or actual caller claim. Producer counterexamples explicitly retained. See analysis/handoffs/round-002-redirect.md and analysis/redirect/round002-a/contracts.md/family-primary.json/family-contract.json.

8F87 actual four bounded artificial rank cases resolved mechanical adjustment for equal/distinct ranks; no ROM patch. New supported819C post-ending continuation rendered BEST5PLAYERS terminal scoreboard from existing ending snapshot,three120000operation runs all complete; artificial terminal milestone added to scenarios/gameplay. Natural finalboss victory unresolved. Prior later-scene primary negatives reviewed, no unchanged retry. Leads preserve all old outcomes, new/revised findings packaged together. No classification/source/ROM/toolkit mutations, deletion/archive/publication/commit; no toolkit issues found. Next independent disassembly review, not terminal exhaustion.

## Round003 disassembly completed — structural classification complete, final gates pending

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Accepted exactly2bytes StaticCode at physical[6428,6430),B0$990C..D,slots[0,1,2,3]. Independent combined review corrects prior missing-family premise using accepted98BC cursor family, exact terminal-JMP/INY boundaries and actor-loop continuation. Constants competitor considered explicitly; legal decoding/tail compatibility alone were not sufficient. Conditional cursor<count equivalence only; override counterexample preserved. Actual entry/natural reachability unresolved. All previous annotations/history/commentary preserved. Evidence: analysis/evidence/round003/review.md and fresh primary.json. R002-cursor-prefix-990C verified; other lead outcomes unchanged.

Code19412/Data111676, including16header bytes separately from physical PRG banks. Exact unknown complement empty; classification-counts.json and unknown-complement.json retained in analysis/evidence/round003.

| Physical8KiB PRG bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 7390 | 802 | 0 |
| 1 | 4299 | 3893 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 97 | 8095 | 0 |
| 4 | 1460 | 6732 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 77 | 8115 | 0 |
| 8 | 4762 | 3430 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 77 | 8115 | 0 |
| 12 | 1173 | 7019 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 0 | 8192 | 0 |
| 15 | 77 | 8115 | 0 |

Fresh checkpoint pass-003: maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json. Map validation and generated byte-identical rebuild passed; state/current/frozen map identical. ROM SHA256 unchanged. Map SHA256 6568e1188ec98ee68f3c5110286d1b938688e9eaa3267e31238272aacf40557a. No new runtime observations or execution-coverage claims; prior progression positives/negatives reused without unchanged later-scene retries.

Structural annotation coverage100%; execution coverage remains separate. Existing current-source commentary unchanged, new two-byte prefix still awaits current-source/final editorial integration. Final editorial audit and its final byte-fidelity gate remain pending despite this checkpoint's successful byte-identical verification; no full workflow/ROM completion claim. Next coordinator phase: current-source editorial review. No toolkit issues found (analysis/toolkit-issues.md), no blockers. No toolkit edits, commentary phase, redirect, archive/deletion/publication or commit performed.

## Round003-a redirect — structural closure review

Input pass-003 after complete current-source commentary. Independently recomputed exact
physical complement:131088/131088 classified,19412Code/111676Data,0Unknown. Empty lead
result appended preserving all histories. No proposals/accepted changes. Durable
closure/rejection review at analysis/redirect/round003-a/review.md and compact handoff
analysis/handoffs/round-003-redirect.md. Retrieved prior decisive cursor/score evidence,
later-scene primary traces and exact bypass scripts, artificial ending/terminal records;
no new structural contradiction or materially improved useful test emerged. Natural
later boss/ending route remains unresolved; no unchanged experiment repeated. Final
editorial and final fidelity gates remain pending for coordinator. No runtime/map/source/
state/toolkit changes, deletion/archive/publication/commit; no toolkit issues found.

## Terminal coordinator handoff — workflow complete

Round003 final editorial acceptance completed after redirect closure. Input/output checkpoint pass-003 remains unchanged. ROM classification:100.00%(131088/131088bytesclassified;0Unknown),Code19412/Data111676. Final supported commentary/audit.json and verification.json pass byte-identically with refreshed reading-after.asm;9629scaffold regions,595top-level meaningful units,9560embedded unit references. Coordinator independently validated physical coverage/bank views/protected hashes and19784source references; analysis/current-audit.json and coordinator-structural-checks.json record exact scope. Final source hashes are recorded in audit.json.

Lead R002-cursor-prefix-990C verified in round003; no new/ready structural leads. R001-progression-wait remains blocked and resumable for natural later combat/boss/ending; current recipe corrects solved NMI/Start premise and links negative timing/initialization tests. Artificial terminal scoreboard visited; historical scenario supersession made explicit. No gameplay/semantic/hardware completeness claim.

Final handoff analysis/handoffs/terminal.md; requirement audit analysis/coordinator-audit.md; final editorial analysis/handoffs/final-editorial.md and commentary/editorial-audit.md. archive-options.json selects every additional final/history Markdown and omits only6identical historical instruction copies; archive-document-audit.json confirms zero unaccounted documents. No archive/cleanup/deletion/toolkit changes/publication/commit performed. No toolkit issues found. This terminal state is independently verified structural/editorial completion, not success inferred from orchestration status.
