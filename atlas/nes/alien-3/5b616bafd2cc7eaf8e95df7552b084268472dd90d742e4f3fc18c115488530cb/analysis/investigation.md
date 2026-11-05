# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round1 disassembly — active evidence batches

Baseline exact rebuild confirmed. Header/CHR/vector roles are independently delimited.
Reset/NMI/IRQ/observed-edge contextual discovery yielded coherent connected routines:
RAM/PPU/APU initialization, mapper helper C008, bank8 audio command interpreter,
bank10 title/options, fixed-bank NMI/IRQ handlers, bank13 gameplay/player logic,
bank12/0 sprite renderer. Static role is separate from natural execution coverage.
All candidate instruction decodes saved; sample blocks show coherent routines and
no unofficial instructions/BRK paths in accepted new batch. Control-flow and
bank contracts, rather than valid decoding alone, establish code.

Uniform data review: 184 exact zero runs >=32 initially proposed, flanking bytes
saved. One368-byte bank15 suffix conflicts with actual rooted vector trampoline
candidates and remains unknown;183 runs23072 bytes accepted as generic non-code.
Not claiming semantic fill purpose. Patterned domain review inspects level/tile
record rows across banks0..7, with independent code boundary9C00 in each even bank
and physical bank endpoints for odd banks. Existing zero runs retained; remaining
non-code slices accepted generically after practical nearby-reference/render-code
inspection. Precise schemas, source layout and consumers remain unresolved.

Pause shortcut: bank13 CPU B3DF reads ten expected masks B437..B440 via X=$A9;
CPX#10 establishes extent. Code has5-frame debounce and pause entry10-frame debounce.
Two-frame taps and a patch during pause debounce failed; settled pause snapshot plus
8held/8released succeeds. $A8 is zero-based stage, incremented by transition;
fixed code1E291 compares12 and branches into ending path. Exact artificial recipe:
import pause-shortcut.snapshot.json, release buttons, patch RAM168 two bytes
[target-1,9], B held8/released8 then12frames, wait360 for initialized destination,
trace representative gameplay actions. No ROM backing writes. Natural control
established only after correcting buttons-array API (NES-001); earlier play was
attract mode, retained and corrected in scenarios.

Ending shortcut correction: the pause cheat unconditionally increments $A8 and jumps
E077, bypassing normal terminal CMP#12 at E281; stage12 therefore initializes invalid
level records (purple screen/text loop, RAM pointer A0/A1 becomes5254). This does
not reject the ordinary ending. An artificial entry at normal completed-level tail
E267 (physical1E277), stackFF, $A8=11 on settled pause snapshot was attempted;
p24 disables IRQ and stalled $52 wait. Retried p20; still pause-state wait; final
bounded correction clears pause$64 and frame gate$52 before continuing. All attempts
remain artificial; natural ending remains unresolved until actual screen confirmed.

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

## Round1 redirect — r1-20261105-a completed productively

Combined all ready commentary leads plus source missing callees/renderer copies,
local-table residual triage and narrowed vector-tail zero inference. No map edits.
[Handoff](round1-redirect-handoff.md) and
[decisive findings/contracts](redirect/r1-20261105-a/findings.md).
Accepted counts218781/43379 unchanged; proposed43371=15227code+28144data, prospective8
unresolved short stubs. Not terminal; independent disassembly review then current
commentary/fresh redirect must establish actual accepted residuals. Invalid exploratory
roots explicitly excluded by corrected graphs; all evidence retained. Leaks of CPU
alias references across banks are triage only and corrected by named bank contracts.

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

## R2 redirect r2-20261105-a

Accepted pass-003, full current-source readability. Fresh independent local control/stack and matched37-state player-domain review; no proposals. Four residuals8bytes remain materially code/data ambiguous. Detailed individual candidate/counterexample/competing-interpretation review and exact primary extracts: analysis/redirect/r2-20261105-a/rejection-review.md, primary-review.json, source-evidence.txt; exact unchanged complement/hash ledger residual-ledger.json. No live-caller prerequisite imposed. Proposed empty helpers/tail trampolines still lack discriminatory independent structural evidence; generic data has no positive pattern/container evidence. No arbitrary execution or unchanged replay. Coordinator must inspect stopping reasoning.
