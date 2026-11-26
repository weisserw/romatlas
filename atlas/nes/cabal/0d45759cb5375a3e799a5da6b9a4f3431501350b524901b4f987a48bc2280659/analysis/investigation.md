# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

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

## Round001-a redirect resumed
Research checkpoint remains pass-001 (117404 classified,13684 unknown,162 intervals).
See analysis/redirect/round001-a/contracts.md and handoff.md. Corrected bank1 service
count to9 and removed false B3 offset-table fallthrough. Connected static graph plus
consumer/pattern boundary review proposes6411 previously unknown Code and7273 Data
bytes; also21-byte E05A correction from accepted generic Data to Code. These are
research proposals only, no map/state/source changes or coverage gain accepted.
Natural Start interlude gate resolved. Four later scenery states received targeted
sprite0 and stack-preserving synchronization bypass probes; sustained combat/boss
mechanics remain unconfirmed. Repeating Start/NMI patches or coordinate sweeps lacks
new support. New constraint: exact split-wait loop, not stopped interrupts. Evidence
and negative recipes preserved. Independent disassembly must review each proposal,
particularly two-byte990C prefix and callee-return/bank contracts; no blanket graph
acceptance or whole-ROM completion claim.

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
