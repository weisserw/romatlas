# Disassembly progress

Round 1, disassembly active. Verified baseline maps/baseline.json, generated/baseline,
analysis/baseline-verification.json (byte_identical=true). Configured release binary only.
ROM is Alien 3 (NES), confirmed startup copyright screen, 262160 bytes, MMC3.
Persistent session tracing zero detail capacity; bounded startup reached frame 190.
Discovery has 545 candidate instructions from reset/observed contexts; decoded listing
in analysis/round1/startup-listing.txt pending annotation review. CHR/header/vectors
classification saved. 537 inspected code instructions also annotated (540 total regions, 132272 classified bytes); no gameplay reached yet. Startup title transition continues.
Current frontier: reach title menu, use sourced NES pause level-skip sequence if valid,
inspect level transition RAM and collect broad progression before final discovery.
Baseline is fidelity only. Phase incomplete; do not advance to commentary yet.

Correction: button/pressed shape was ignored by NES API; earlier level1 is attract demo, not controlled play. Actual buttons array now used. See NES-001/002 in toolkit-issues.md. Discovery budget corrected to max_instructions, 5095 candidates available.
Correct input schema entered actual options menu at frame2642 (actual-menu.ppm).
Start selection now activated; collecting controlled gameplay and pause skip.

Round1 current saved structural map: 164984 / 262160 bytes (909 regions), including
10808 code bytes (537 individual startup regions plus185 connected regions), 183 exact
uniform zero runs after flanking-byte and code-conflict review. Full discovery5095
context candidates, 4663 new unique instructions reviewed and 185 contiguous code
regions accepted. Cheat table10 bytes structurally proved by finite comparator.
Corrected-input natural pause skip succeeded after8-frame held/released inputs;
zero-based stage increment0->1 confirmed. Broad artificial stage shortcuts ongoing.
First patch attempt on just-paused snapshot occurred during10-frame debounce and
failed to transition; retry uses settled pause-shortcut.snapshot.json. Artificial
$A8 writes alone are not counted as a stage visit.

## Round1 disassembly completed — incomplete ROM

ROM classification: 83.45% (218781 / 262160 bytes classified; 43379 bytes unknown).

Final saved state analysis/state.json; portable map maps/current.json, frozen checkpoint
maps/pass-001.json; readable-source generation generated/pass-001/game.asm. Validation
and byte-identical rebuild passed: analysis/pass-001-verification.json, map SHA256
932fc29234faed98beda8535de8a1a6c78a431b49ad20ee46f7ce6c68afaba01.
Input SHA256 unchanged. Header16 and CHR131072 classified separately from PRG.
Code15111, Data203670;1539 structural regions, all have evidence. Unknown complement
analysis/round1/unknown-final.json. No full-ROM or current-source readability completion
claimed; commentary phase follows. Baseline independently verified once.

| PRG8KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|---:|
| 0 | 364 | 7300 | 7664 | 528 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 364 | 7300 | 7664 | 528 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 364 | 7300 | 7664 | 528 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 364 | 7300 | 7664 | 528 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 1026 | 52 | 1078 | 7114 |
| 9 | 0 | 4658 | 4658 | 3534 |
| 10 | 2220 | 64 | 2284 | 5908 |
| 11 | 0 | 5429 | 5429 | 2763 |
| 12 | 709 | 280 | 989 | 7203 |
| 13 | 3156 | 84 | 3240 | 4952 |
| 14 | 2445 | 41 | 2486 | 5706 |
| 15 | 4099 | 6 | 4105 | 4087 |


Evidence: contextual connected instruction listings and immutable byte-pattern reviews
in analysis/round1; accepted code includes reset/interrupt setup, mapper helpers, audio
interpreter, title/options, gameplay/player/renderer and ending text/IRQ code. Generic
data uses repeated non-instruction record content after nearby-code/reference inspection;
exact subformat/consumer semantics remain unresolved. No fetch/data conflicts across
11240 final context keys (analysis/round1/fetch-data-audit.json; last ending contexts
additions also checked by map barriers).

Execution is separate:7943 unique physical opcode starts,11240 contextual roots,12755
context edges; aggregate_dropped=0. Detail capacity0 intentionally retains no events;
detail dropped112505816 does not mean aggregate loss. trace-final.json carries counts.
Early mistaken button/pressed routes were natural attract mode and corrected. Actual
buttons-array controlled options/level1 then natural pause skip0->1 confirmed. Artificial
stage2 first guardian (visible boss),8 later guardian area,9 late normal,11 final guardian
area visited with movement/fire/jump; later boss contacts unproven. Ending confirmed
visually after ordinary completion-tail artificial contract; WELL DONE bonus then Ripley
escape (ending-after-bonus.ppm, ending-confirmed.snapshot.json). Exact routes, failed
patch/debounce alternatives and replay snapshots: analysis/scenarios.json and gameplay.md.

Stopping rationale: one substantial initial pass completed: corrected normal inputs,
representative broad progression and ending, observed roots plus supported connected
static batches, pattern data and duplicate-renderer closure. Remaining banked dispatch,
sound/data extents and object handlers need targeted redirect research; current pass is
not terminal exhaustion and no terminal rejection review is asserted. Ready R1-L03/04/05
and gameplay researchR1-L07 recorded in leads.json; ready ranges are starting points,
not byte ceilings. No prior external leads existed. R1-L01 andL06 verified; invalid
cheat-to-stage12 R1-L02 superseded by normal-tail ending recipe.

Toolkit findings:2 confirmed documentation/API mismatch bugs (NES input schema and
discovery budget parameter),0 suspected bugs,0 missing features; neither blocks work.
See analysis/toolkit-issues.md. Toolkit unmodified; configured binary retained.
Disk:9.1GiB free; exploratory scratch24KiB; cited evidence/snapshots retained locally.
Next phase: full current classified-source commentary, then redirect the recorded
concrete questions. Round1 disassembly checkpoint pass-001 is ready.

## Round 1 commentary active

Prepared frozen commentary baseline from pass-001; prepare verified byte-identically.
Inventory: 1762 physical scaffold entries (1182 Code,357 Data,223 Unknown).
Full compact classified instruction listing reviewed by physical bank, including
four duplicate level renderers, audio interpreter, screen commands, actor renderer,
fixed collision/movement/scroll/NMI helpers. Applying functional entry/block notes,
bank-specific symbols and common hardware/field aliases with source-generation
helper analysis/scratch/build-commentary.py. Concrete completed draft notes cover
level renderers, bank8 audio, bank10 title/options/completion/IRQ/resource loading,
and bank12 actor/OAM rendering. Fixed gameplay/interrupt source edits, metadata and
fresh verification remain pending; no successful commentary handoff yet.

## Round 1 commentary completed — current source readable, ROM incomplete

Full pass-001 current-source readability is complete:15111 code bytes,203670 data
bytes;43379 unknown bytes retain explicit physical-boundary uncertainty notes.
All1762 scaffold entries reviewed (1182 Code,357 Data,223 Unknown). All code bytes
covered by423 meaningful code units across213 physical groups; four additional data
units explain header,CHR planes,vector words and pause shortcut. Source has973
bank-qualified routine/branch labels and296 field/address/table aliases.
Functional explanations include audio streams/modulation, screen resources/commands,
actor frame/OAM clipping, player/inventory/collision, scrolling/map staging and IRQ/NMI.
Full scope, examples and unresolved meanings:analysis/round1-commentary-assessment.md.

Fresh final commentary/verification.json:byte_identical=true, input SHA256 unchanged.
Source SHA256 game.asm=b6709cd2a01060ee9a6d14d6d5c165a899cfad4491c7d6ed58d2780114adcc54;
symbols.inc=fc52fee86c38c3b0a7b2a73b9d785b80e135a109eacaae747e7aab89cc32c037.
Reading copy commentary/reading-after.asm refreshed. Inventory/source-link/functional
coverage checks:analysis/round1/commentary-readability-check.json. Frozen phase1 state
matches analysis/state.json; structural maps/checkpoint pass-001 unchanged.
No pending classified-source readability backlog; final editorial_audit stays pending
for later editorial/archive acceptance. Whole-ROM classification remains incomplete.

R1-L03/04/05/07 are concrete redirect work, not deferred commentary:screen finite
mode domain, actor type/state handlers, audio table/stream extent, guardian exits
and completion continuation. Commentary uncertainties and compact handoff link them.
Toolkit report unchanged:2 confirmed doc/API bugs,0 suspected,0 missing features;
no blockers or newly found toolkit issue. No toolkit edits, archive, deletion or
publication performed. Cited evidence/source and editing helper retained locally.

## Round1 redirect — active attempt r1-20261105-a

Input checkpoint pass-001, commentary current-source handoff complete. State/maps/source
unchanged; 218781 classified /43379 unknown. Research only in
analysis/redirect/r1-20261105-a. Read redirect/evidence policy and configured NES APIs.
Static groundwork: screen word dispatcher CPU8027..803D has11 entries and actual
fixed callers select0..10; hypothetical connected CFG1509 instructions. Actor
frame/behavior roots have15 adjacent type entries; nested state candidates and4478
instruction graph captured. Audio500-instruction consumer graph reaches841E RTS,
followed by frequency/pointer/instrument/stream-pattern non-code domain. Completion
return E27F increments stage, compares12, calls modes4 and6 then restarts inventory.
Graphs explicitly retain hypothetical call-return propagation; mapper/nonlocal stack
contracts are being reviewed before proposals. Active frontier: validate screen
resource/command tables; actor state array endpoints against frame counterparts;
audio bounded table/stream representation; guardian exit/progression continuation.
No map edits or natural execution claims. Disk9GiB free; research small, no cleanup needed.

Redirect milestone: counterpart actor state domains all agree;74 frame sequences,
155 referenced frame records6672 bytes and4 same-format gap records coverbank12
8000..9B10. All replacement-state words fit their type domains. All11 screen
resource tile streams and sprite streams parse within consumer bounds. Audio
code terminates841E before its separate frequency/pointer/stream data domain.
Inspected additional source missing callees, inactive actor initializers, full
actor/level renderer families, and mapper-write continuations while preserving
fixed slots13/14/15; mapper pair helper remains explicit boundary.
Current proposed-only batch39024 bytes(14669 code/24355 data), no accepted changes;
zero existing-data/code overlaps. Prospective residual4355 bytes is now being
triaged for obvious local constants/tables before handoff; not a terminal assessment.
Source exit experiment from final guardian: patch player X/Y0400/0208 in source
window03F8/0200, unpause,12frames reachescompletion fade,372frames visually WELL DONE;
1092/1824frames continueescape/ending. Separate source-proven ending cursor shortcut
sets terminal flag from0to1 in12frames and displays HIGH SCORE/FINAL SCORE612frames
later. No PC edit, no boss defeat or natural reachability claim. Reusable artifacts
in attempt directory including exit-test.json, ending-continuation.json,
ending-terminal.json, selected captures and exit-after372.snapshot.json.

Redirect packaging milestone — proposed43371 bytes (15227 code/28144 data),7227
new instruction starts, prospective8-byte residual. Corrected-root authoritative
versions:source-prefix-closures-v2,screen-irq-v3; original root attempts remain
history and are explicitly EXCLUDED in findings.md. Actor exploratory slot12
behavior view normalized to supported representative gameplay slot0; raw bank12
KIL is excluded, and alias-only reference triage never establishes physical consumers.
Package checks exact unknown intersection, existing data conflicts, cloned byte
identity and bytewise overlapping instruction starts:0conflicts. Accepted state
SHA932fc29234faed98beda8535de8a1a6c78a431b49ad20ee46f7ce6c68afaba01
and ROM SHA unchanged. Current accepted counts still218781/43379, no map writes.
Narrowed prior368-zero rejection toFE85..FFF4(367bytes):FE84 remains operand of
accepted vector JMP; narrowed suffix has independent boundaries/no code conflict.
Residual8bytes:bank14 D0ED3/D4E51/D58B3/DABE1, unreferenced short JMP/RTS stubs;
no speculative annotation. Productive handoff to independent disassembly, not terminal.
Full contracts, invalid predecessor scope, runtime outcomes and deferral reasons in
analysis/redirect/r1-20261105-a/findings.md; preparing compact lead/handoff update.

Round1 redirect handoff complete:analysis/round1-redirect-handoff.md; findings and
manifest inanalysis/redirect/r1-20261105-a. Final proposed43371(15227Code/28144Data),
prospective8 residual. Accepted unchanged218781classified/43379unknown. Independent
packaging assertions:sum lengths=union43371, disjoint accepted state, no duplicate
proposal bytes, no instruction overlaps/data conflicts, renderer clones identical,
state/ROM hashes unchanged. Final disk9GiB free; attempt19MiB, scratch344KiB, no
superseded bulk scratch cleanup needed. All bounded runtime subprocesses exited.
Leads updatedR1-L03/04/05/06/07; addedR1-L08/09;3 new artificial progression scenarios.
Next frontier: independent disassembly review with authoritative corrected graph
versions/contracts, classify accepted proposal slices, verify and checkpoint; then
full current-source commentary/fresh redirect. No terminal/exhaustion assessment.
Toolkit summary:0new issues, existing2confirmed documentation/API bugs,0suspected,
0missingfeatures, noblockers;analysis/toolkit-issues.md. No toolkit/source/map edits.

## Round2 disassembly — active, input pass-001

Independent review active on exact r1-20261105-a proposals, not blanket graph adoption.
Current accepted checkpoint remains pass-001 (218781 classified/43379 unknown).
Frontier: bank/control/stack contracts and exact candidate boundaries; corrected
source-prefix-closures-v2 and screen-irq-v3 only. Full proposal decoder/bank/overlap
checks in analysis/round2; static structural review and data consumer checks follow.
Prior gameplay scenarios include representative early/later/guardian/bonus/ending/final
score states; reuse supported evidence rather than repeating unchanged scenarios.
Disk9.0GiB free, scratch344KiB; no files removed. Baseline remains verified, not rerun.

Round2 review checkpoint: all43371 proposed bytes structurally accepted (15227Code,
28144Data), allStatic; decoder/bank and independent actor/screen record audits passed.
Review:analysis/round2/review.md. Four residual fragments remain8bytes. Full renderer
instruction copies match; broader exploratory full1KiB comparison correctly fails
on variable stage initialization tails and is not a classification premise.
Annotation RPC initially rejected missing/nonzero new IDs; rejected operations did
not change saved state. A shell sequencing mistake nevertheless created pass-002
from unchanged pass-001 (same map SHA932fc292...). Preserve this verified duplicate
as history; actual round2 accepted checkpoint will be pass-003. Corrected id0 inserts
are in progress. No toolkit modification or prior evidence deletion.

## Round2 disassembly completed — pass-003, incomplete

ROM classification: 100.00% (262152 / 262160 bytes classified; 8 bytes unknown).

Exact percentage99.99694842844065%; rounded100.00% does not establish completion.
30338 Code +231814 Data; header16Data and CHR131072Data included in whole-file total.
Accepted43371 previously unknown bytes (15227Code/28144Data), allStatic, in293
annotations grouping only contiguous compatible slices. Every old annotation unchanged.
Required structural evidence is attached to every new region; full current-source
readability/commentary needs updating after this pass and is not claimed here.

| PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 633 | 7559 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 633 | 7559 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 633 | 7559 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 633 | 7559 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 1051 | 7141 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 4878 | 3314 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 979 | 7213 | 8192 | 0 |
| 13 | 6219 | 1973 | 8192 | 0 |
| 14 | 7545 | 639 | 8184 | 8 |
| 15 | 7134 | 1058 | 8192 | 0 |

Exact complement and counts: [round2/accounting.json](round2/accounting.json).
Residuals (physical half-open):1D0FD..1D100,1D4F5..1D4F6,1D59B..1D59E,
1DACE..1DACF; all bank14, CPU D0ED/D4E5/D58B/DABE. Isolated JMP CF65/RTS
encodings remain materially ambiguous; no short-opcode guess or dead-code claim.

Evidence: [independent review](round2/review.md), [decode audit](round2/decode-audit.json),
[data audit](round2/data-audit.json), [acceptance](round2/acceptance.json), and
redirect r1-20261105-a primary contracts/proposals. All7227 candidate instructions
redecoded with exact bank mapping; all159 sprite records,74 animation sequences,
15 matched actor domains,11 screen resource domains independently checked.
R1-L03/04/05/06/07/08/09 verified within these structural scopes. No proposal rejected;
invalid predecessor roots/alias-only references/exploratory bank12 target excluded.
Broader full1KiB renderer identity check failed on stage-dependent initialization
tails; selected instruction slices match and tails independently show record data.
Bounded inherited progression evidence reused, including early/later levels, guardians,
bonus, ending and final-score. Execution coverage unchanged; new code remainsStatic.
No natural guardian defeat or natural ending reachability claimed. No unchanged
scenario repetition warranted for this structural batch.

map.validate passed; state/map/checkpoint hashes match. [pass-003 verification](pass-003-verification.json)
reports byte_identical=true; generated/pass-003 is the actual round2 source.
pass-002 is a preserved verified duplicate of pass-001 accidentally created after
rejected annotation-ID requests; it is not the round2 gain checkpoint.
No previous source/checkpoint/evidence overwritten or removed.

Stopping: ready connected structural batch and ordinary static continuations reviewed;
four short fragments need fresh redirect reasoning against accepted neighbors. This
is an easy-gain handoff, not terminal exhaustion; no terminal rejection review claimed.
Next: full current-source commentary, then fresh redirect on exact8-byte complement.
Toolkit:0new findings; existing2confirmed documentation/API bugs,0suspected,
0missingfeatures, no blockers; [toolkit-issues.md](toolkit-issues.md). No toolkit edits.
ROM SHA remains5b616bafd2cc7eaf8e95df7552b084268472dd90d742e4f3fc18c115488530cb.

Round2 final handoff:analysis/round2-disassembly-handoff.md. Final storage9.0GiB free;
analysis/scratch344KiB, round2 audit artifacts small/local; no cleanup or archive.
All bounded server subprocesses exited. Next active frontier: full current-source
commentary on pass-003 then fresh redirect R2-L01.

## Round2 commentary active — pass-003

Prior commentary preserved intact at analysis/round-2/prior-commentary. Fresh baseline
will use actual pass-003/map SHA4e8db6a0; preserve valid physical-range review credit.
Scope30338Code/231814Data,8unknown; new15227Code/28144Data require full readability.
Active frontier: scaffold preparation and carry-forward, then screen/actor/player/fixed
new code and data-format notes. Same commentary phase until full scope complete;
final editorial audit reserved. No structural changes, deletion or publication.

Round2 commentary drafting milestone: all7227 new opcode starts inspected in bounded
per-bank compact listings (analysis/round2/commentary-bank*.txt); cloned renderer
semantics reviewed once with distinct physical identities. Functional draft notes
cover screen menus/bonus/final-score/IRQ, all actor behavior/initializer families,
player reaction/weapon/support/motion paths, fixed progression and damage dispatch.
Current frontier: apply names/aliases, carry previous physical reviews, document
audio/frame/animation/screen/column data formats, check full inventory and rebuild.
No source-readability success yet; pending work remains within commentary.

## Round2 commentary completed — full current-source readability

Actual pass-003:30338Code/231814Data now readable;8unknown unchanged/explained.
All1836 scaffold entries reviewed;719 functional code units cover every code byte,
69 data units,1881 routine/branch labels+68 data labels,351 aliases. Prior1539
classified physical reviews reused; entire previous phase preserved under
analysis/round-2/prior-commentary. All new7227 opcode starts and data domains
reviewed using independent contracts; no remaining naming/comment backlog.
Updated stale continuation statements and known pointer/digit/scratch aliases.

Fresh commentary verification byte_identical=true; reading-after refreshed.
Source SHA0604078407c3a8c9d994026b2b13cd8b2bb37e03c5086bf46b013aa2eec52f1d;
symbols SHA907fb4aad7a9578ba95c113ccbcf22cd90a0c5a3056632f697e9c9fcc74f8054.
Source links/coverage/maps/hashes:analysis/round2/commentary-readability-check.json.
Full assessment:analysis/round2-commentary-assessment.md; current uncertainties
commentary/uncertainties.md. Final editorial audit pending and not run.

ROM remains INCOMPLETE262152/262160,exact99.99694842844065%,rounded100.00%.
Four bank14 unknowns unchanged:1D0FD..1D100,1D4F5..1D4F6,1D59B..1D59E,
1DACE..1DACF. Next active frontier:fresh redirectR2-L01 against now-described
neighboring handlers/control contracts, not terminal exhaustion. No new execution
or natural reachability claim. Toolkit0new,existing2confirmed doc/API mismatches,
0suspected/0missingfeatures,no blockers;analysis/toolkit-issues.md. No toolkit,ROM
or map change, publication, archive, deletion or cleanup of cited/history artifacts.

## Round2 redirect active — r2-20261105-a

Input actual pass-003/map SHA4e8db6a04eb66e103f84903691c4c75900553daf826f7edf62392b2e1be2082a; full current-source readability gate satisfied. Frontier: independent four-residual assessment using accepted walking/arc/weapon control and stack contracts, finite player domains, primary prior graphs. No-reference judgment is not inherited as veto. Storage9.0GiB free; no cleanup or ROM/map/source/toolkit changes.

Round2 redirect assessment ready for coordinator: r2-20261105-a. Fresh37-word
player-table ROM read and15 primary graph extractions confirm local predecessor
closures and known dispatch domains; none establishes the four isolated fragments.
Individual trampoline/empty-helper/redundant-epilogue/generic-data claims reviewed
in analysis/redirect/r2-20261105-a/rejection-review.md with primary artifacts.
No proposal or ready revised test; R2-L01 attempt appended, current sufficiency
rejected without asserting deadness/non-code. Exact8-byte complement retained,
map/state/ROM/source unchanged. Reuse valid progression/ending/final-score evidence;
no gratuitous replay. Terminal incomplete recommendation requires coordinator review.
Toolkit0new,existing2confirmed doc/API bugs,0suspected/0missingfeatures,no blockers.
No final editorial audit, archive, publication, deletion or toolkit changes.

## Final editorial acceptance complete — pass-003, ROM incomplete

Reused full1836 scaffold/719code/69data reviews. Named player motion index/timer
and overhead launch offsets; removed65 duplicate consecutive comments, updated
current uncertainty/status notes and retained line identities. Preserved pre-final
source/review/check/verification/reading at analysis/final-editorial-before.
Fresh configured-binary audit passed byte_identical=true; reading copy refreshed.
Scope/source-link/hash checks refreshed against final source. ROM/map/state unchanged;
8unknown unchanged. Final detail:analysis/final-editorial-handoff.md and
commentary/editorial-audit.md. Toolkit0new,existing2confirmed doc/API issues,
0suspected/0missingfeatures,no blockers. No archive,publication,cleanup,deletion
or toolkit modifications. Coordinator retains archive ownership.

## Coordinator terminal handoff — justified incomplete

Workflow finished after two sequential disassembly/commentary/redirect rounds and
final editorial acceptance. Coordinator inspected the terminal rejection review and
decisive primary ROM/instruction/player-table evidence; no useful corrected inference
or supported next test remains. Exact8-byte ambiguity retained, not a blocked task
and not complete ROM classification. Reopening requires supported structural evidence.

ROM classification: 100.00% (262152 / 262160 bytes classified; 8 bytes unknown).
Exact99.99694842844065%. Final structural checkpoint pass-003; current source fully
reviewed and final audit byte-identical. 30338Code/231814Data. Unknown bank14 physical
half-open1D0FD..1D100,1D4F5..1D4F6,1D59B..1D59E,1DACE..1DACF.

Final artifacts: analysis/terminal-handoff.md, analysis/coordinator-audit.md,
analysis/current-audit.json, analysis/completion-requirements.json,
commentary/editorial-audit.md, commentary/audit.json, commentary/reading-after.asm.
archive-options.json explicitly selects additional final/history documents and
omits only duplicate instruction templates; analysis/archive-document-audit.json
records complete inventory with zero unaccounted Markdown. All artifacts ready for
a separately authorized archive; no archive execution/cleanup/publication occurred.
Input ROM/toolkit unchanged. Toolkit2confirmed documentation/API issues, no blockers.
Orchestration completion does not mark ROM classification complete.
