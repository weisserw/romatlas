# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly in progress
Baseline created and verified: analysis/baseline-verification.json, maps/baseline.json,
generated/baseline. Configured release binary used; ROM identity unchanged.
CNROM mapper 3, fixed PRG banks [0,1,2,3]; 32 KiB PRG + 32 KiB CHR.
Vectors reset $8004 / NMI $8057 / IRQ $80F2 validated at physical $800A..$800F.
Bounded static traversal produced 5,804 instruction hypotheses; reviewing inline-data
barriers before annotation. Runtime title identified Arkista's Ring. Durable snapshots
startup.snapshot.json and start.snapshot.json; no regions classified yet. Initial
button/pressed API syntax silently ignored on NES; corrected to buttons array.
Current frontier: enter game and obtain stage RAM contract, review static candidates.

Static review found 5,951 nonoverlapping instruction candidates after excluding
inline dispatch data. Annotation/save is running. Generic CHR image classification
and hardware header/vectors committed through map.annotate. Runtime naturally
visited stage 1; artificial initialized stages 8/16/24/30/31/32 are in flight using
$03D0 plus $8552 stage transition contract and accelerated banner timer $20/$21.
NES-001 input API mismatch recorded in analysis/toolkit-issues.md; workaround active.


## Round 001 — disassembly completed, incomplete ROM

ROM classification: 89.86% (58908 / 65552 bytes classified; 6644 bytes unknown).

State/map: analysis/state.json and maps/current.json. Fresh checkpoint
[pass-001](../generated/pass-001/game.asm), map maps/pass-001.json, verification
analysis/pass-001-verification.json: ok=true, byte_identical=true. Input SHA unchanged.
Configured binary built from toolkit commit 51e2abfc3cb8cc0250bf2859dd08d36aaecc1a77; no toolkit edits.

|Bank|Code|Data|Image|Unknown|
|---|---:|---:|---:|---:|
|header|0|16|0|0|
|PRG-0|7469|614|0|109|
|PRG-1|3122|53|0|5017|
|PRG-2|2358|4316|0|1518|
|PRG-3|0|8192|0|0|
|CHR-0|0|0|8192|0|
|CHR-1|0|0|8192|0|
|CHR-2|0|0|8192|0|
|CHR-3|0|0|8192|0|

Unique physical accounting, all annotated regions have evidence; 6422 regions,
4 labels. Source readability/editorial review remains commentary work.
Code/data classification is separate from execution: merged saved scenarios contain
3,498 unique ROM opcode starts; broad artificial stage trace alone 3,106 starts,
0 aggregate drops, 2,652,814 detail events dropped by deliberate zero detail capacity.
Coverage initial pages were bounded; this is a lower bound, not whole-game coverage.
Observed and artificial provenance retained separately from Static structure.

Natural title/start reached stage1. Corrected artificial stage initialization visited
stages8/24/31/32 and executed right/up+shoot; fourth-loop transition showed ending
text and later Christine status. See scenarios.json, gameplay.md and stage screenshots.
First shortcut failed from retained $19 sprite-zero wait mode; corrected using $19=0
as established by stage-exit prologue. No ROM patches.

Accepted broad static vector flow, two inline audio dispatch tables, seven-way split
dispatch and connected code; generic non-code asset blocks after bounded consumer/
pattern inspection. Exact unknown complement: round001-accounting.json. Ready next
work is entity/event selector graphs and small disconnected routine contracts in
leads.json. Productive basic progression was attempted broadly; repeated sightseeing
now gives small gains. No terminal exhaustion claim: redirect investigation remains
promising, especially 5,017 unknown bytes in PRG1.

Toolkit: one confirmed API validation/documentation mismatch, no blockers; see
[toolkit-issues.md](toolkit-issues.md), NES-001. No suspected bugs or missing features
reported. Baseline and pass checkpoint both rebuilt byte-identically.

## Coordinator boundary — round 001 commentary active

Disassembly worker completed; coordinator inspected its compact handoff, exact counts
and pass-001 byte-identical verification. Required classification percentage printed
to user before commentary: 89.86% (58908 / 65552; 6644 unknown).
Separate worker round1_commentary now owns current-source readability from pass-001.
Redirect must wait for all-classified-source readability, refreshed reading copy and
fresh byte-identical verification. No terminal assessment yet. Baseline statements
above are historical, superseded by the completed disassembly entry.

Previous goal turn made progress (verified baseline and durable archive contract note);
this continuation also verified the live disassembly worker and accepted its completed
phase evidence before advancing sequentially. Archive contract inspected; final
archive-options.json will be written only against terminal document inventory.

## Round 001 — commentary active checkpoint
Prepared commentary frozen baseline with initial byte-identical verification. Three
source batches now explain CPU $8004..$92FA (physical $14..$130B), including reset/NMI,
title/menu, stage setup, status/health, entity initialization and map decoding.
All three batches verified byte-identically; latest source hash recorded in
commentary-frontier.md. review.json preserves reviewed mechanics credit with final
branch/work-byte consistency pending. Next substantive range CPU $93E7 onward.
All later classified source, data/image/unknown boundaries, consistency sweep,
final fresh verification and refreshed reading copy remain pending. Phase remains
commentary; no redirect/disassembly started. Toolkit NES-001 unchanged, no new findings.

Verified batches 004..006 extend functional review through CPU $B80D (physical
$381D), including full player movement/attacks/projectiles, inventory item effects,
entity lifecycle/type dispatch, sprite animation and secondary entity handling.
Latest SHA 7091b37e8532ac06ffcf60b5f1b45bdb26290fc2f1f74d60bb9de394f04b21b0;
verification byte_identical=true. Remaining substantive code begins $B863; also
full boundary/data review and branch/scratch consistency remain. Phase active.

Batches 007/008 complete functional comments through all classified code ($D035).
Global sweep added 544 branch-only symbols and 111 shared hardware/RAM aliases;
all remaining classified direct call targets have explanatory entry comments.
Sweep rebuilt byte-identically (083e069a01e213e8d3989e174a78fb0731d467a0356783afd5dd7d729ff67d1b).
Data/image and all 32 unknown gaps now have source format/boundary/uncertainty notes,
reusing prior bounded pattern review. Final record consistency and reading refresh pending.

## Round 001 — commentary completed

Completed current-source readability across all classified code/data/image and every
unknown boundary. 419 compact functional explanations and 544 named mechanical joins;
consistent hardware/RAM/shared scratch/primary-secondary record/table operands.
No structural reclassification or map/state edits. Scope/readability is separate from
89.86% classification and remaining 6,644 unknown bytes. No known readability gaps.
Final commentary verification ok=true, byte_identical=true; source SHA256
656fb64587d11fae78f4fd79de3d0c1dd1ceac60349483c80372704bc7960172.
Reading-after refreshed. Frozen before/baseline retained. Handoff:
[commentary-round-001-handoff.md](commentary-round-001-handoff.md); review/scope records
in commentary/review.json and commentary-round-001-scope.json. Final editorial/archive
acceptance remains pending and no audit was invoked. Toolkit NES-001 unchanged,
no new findings or blocker. No redirect/disassembly started in this phase.

## Coordinator boundary — round 001 redirect active

Accepted commentary-round-001-handoff.md after inspecting scope, verification and
source samples. Source SHA 656fb64587d11fae78f4fd79de3d0c1dd1ceac60349483c80372704bc7960172
matches fresh byte-identical verification; reading copy refreshed. Full current-source
readability complete; final editorial acceptance remains pending. Separate sequential
round1_redirect worker now investigates existing entity/small-routine leads and new
localized commentary questions. No classification or terminal completion claim.
Previous goal turn was a verified wait with new final-source batch evidence; this
turn accepted completed commentary and advanced the coordinator workflow.

## Redirect round 001 active checkpoint — static contracts
Unique attempt analysis/redirect/r001-static-contracts/frontier.md and read-only RPC/decoder artifacts persisted. Map/state/source unchanged (58908 classified,6644 unknown). Coherent dispatch graphs and independent small routines under review; candidate discovery alone not accepted classification. See frontier for exact continuation, alignment rejection and remaining data/selector questions. No terminal assessment.

Redirect001 static investigation checkpoint: connected primary/secondary lifecycle,
all19 effect continuations, event callbacks and independent small PPU/collision/
animation helpers reviewed; proposed5463 code +1179 data bytes=6642. Research-only
proposal and per-range rationale: analysis/redirect/r001-static-contracts/proposal.json
and findings.md. No map mutation; accepted unknown remains6644. If all accepted,
residual would be two loneRTS bytes2991/44B2. Boundary audit found no proposed
instruction overlap or direct control edges into proposedData. Inventory static
producers include6/7; reads can overlap existing code8C3C conditionally.25FC mask
preserved as actual zp read. Toolkit: existing NES-001 only, no new issue/blocker.

## Coordinator boundary — round 002 disassembly active

Redirect001 completed coherent connected and independent batch; coordinator inspected
redirect-round-001-handoff.md. Combined ready leads propose 6642 bytes, not accepted
classification. Separate round2_disassembly now independently reviews exact contracts,
bytes/context and boundaries before a new pass-002 checkpoint. Current classification
remains 89.86%, 6644 unknown. Prior turn verified live redirect wait; current turn
accepted durable ready handoff and advanced workflow. Commentary preservation and
next full readability pass required. No terminal exhaustion claim.

## Round 002 — disassembly completed, incomplete ROM

ROM classification: 100.00% (65550 / 65552 bytes classified; 2 bytes unknown).

Rounded percentage is 99.996948987% before rounding and does not establish completion.
Independently reviewed/accepted combined R001-entity-dispatch, R001-small-routines,
R001-static-tables: 5463 Code +1179 Data=6642 new Static bytes. No Observed or natural
reachability claim. Exact raw-data equality, fresh explicit-context decode2719
instructions, overlap/complement and direct-control boundary checks persisted in
[review](disassembly-round-002/review.md), fresh-decode.json and independent-audit.json.
Minor proposal wording corrected:9B6C00/04/02/84 repeats are four bytes, not triples;
no role/boundary rejection. All old classifications preserved. All9160 regions have
evidence. Fixed CNROM [0,1,2,3]; no new execution coverage. Reused valid broad
round001 stages/ending routes; unchanged gameplay would not discriminate loneRTS roles.

|Bank|Code|Data|Image|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|header|0|16|0|16|0|
|PRG-0|7538|654|0|8192|0|
|PRG-1|7331|860|0|8191|1|
|PRG-2|3543|4648|0|8191|1|
|PRG-3|0|8192|0|8192|0|
|CHR-0|0|0|8192|8192|0|
|CHR-1|0|0|8192|8192|0|
|CHR-2|0|0|8192|8192|0|
|CHR-3|0|0|8192|8192|0|

Exact unique physical complement in round002-accounting.json:0x2991/CPU A981 and
0x44B2/CPU C4A2, each one byte60. Competing spare return vs data/padding roles remain
materially ambiguous; no proven specific entry/consumer. C47C..C4A1 is genericData
from repeated pairs, not a claimed proven pointer table. B98689 20 is emitted
exactly `.byte $89,$20 ; nop #$20` in fresh source. Prior localized residual review
remains applicable; no speculative acceptance and no terminal redirect exhaustion.

map.validate succeeded; analysis/state.json, maps/current.json and maps/pass-002.json
SHA256 feba0c385e2b7e2ec6e964e25af6f0b1f8121850804176f0c1bc6ebf4a2ba424.
Fresh [pass-002 source](../generated/pass-002/game.asm),
[verification](pass-002-verification.json):ok=true,byte_identical=true.
Input ROM SHA unchanged; no ROM/toolkit edits. Prior commentary/source/review and
pass-001 are preserved. Fresh generated source SHA
686229c69be8ee6b15117a8ee8755133cc60bc6e0d58a740b54203e000714d95;
full current-source commentary must now incorporate new classified material and
preserve prior functional review before redirect. No commentary/redirect run here.

Toolkit: no new issues; existing confirmed NES-001 input schema mismatch remains
nonblocking using buttons array. [toolkit-issues.md](toolkit-issues.md) preserved.
No suspected bugs/missing features newly found. Two rejected annotate requests
were local schema mistakes corrected with id=0 creation sentinel; no state changes
from rejected calls. Disk8.4GiB free at handoff, no cleanup/deletion performed.

## Coordinator boundary — round 002 commentary active

Inspected disassembly-round-002-handoff and pass-002 fresh byte-identical verification.
Printed required classification before commentary: 100.00% (65550 /65552;2unknown).
Rounded percentage is not ROM completion. Separate round2_commentary now integrates
new classified source with preserved prior commentary/review. Worker must also adapt
per-Code-region units to supported final-audit schema, retaining review credit.
Final editorial acceptance and redirect rejection review remain pending. Previous turn
was verified live wait; current turn accepted completed pass-002 and advanced phase.

Round002 commentary active: baseline preserved and physical incremental merge verified byte-identically. See commentary/progress.md and commentary-round-002-frontier.md for exact integration frontier.

### Coordinator-verified commentary002 integration checkpoint

Live round2_commentary worker retained; prior turn was a verified wait, no restart.
Prior commentary tree exists at analysis/round-002/commentary-round-001; current
integrated source SHA8326868944741b3ca0de084a2475bd027b357a3ef87bdb04bac60644f7961c97
matches commentary/verification.json byte-identical result. This proves integration
fidelity, not completed new readability. New functional comments/symbols and review
carryover/schema adaptation are still pending. Keep same phase; do not advance.

Round002 commentary complete for all current source; final fresh byte-identical verify and reading refresh passed. Source SHA6755f9046216be3d82b229be25aea37dbad7e976a3e3142c04ec2ea641879b73. All9162 scaffold entries reviewed;9118 Code regions have valid units;419 prior compact reviews retained. Final editorial audit pending. See commentary-round-002-handoff.md. Classification65550/65552 remains incomplete.

## Coordinator boundary — round 002 redirect active

Accepted commentary-round-002-handoff after current hash/verification and integrity
inspection; all9118 Code regions have units, reading copy current. Full current-source
readability complete at6755f9046216be3d82b229be25aea37dbad7e976a3e3142c04ec2ea641879b73.
Separate round2_redirect now investigates exact two-byte residual and any credible
remaining structural avenues, with evidence-policy rejection review required before
terminal incomplete handoff. Final editorial acceptance remains pending.
Previous turn verified live commentary wait; current turn accepted finished gate and
advanced sequential workflow. No ROM classification/goal completion claim.

## Round002 redirect — residual role review complete, incomplete ROM

Pass002/current SHA feba0c385e2b7e2ec6e964e25af6f0b1f8121850804176f0c1bc6ebf4a2ba424; full current-source readability already complete. Fresh current-map operand/boundary and target-word audit confirms unknown physical2991/A981 and44B2/C4A2, each60. Reviewed independent contracts and limited negative scopes; no supported role proposal or discriminating next test. Exact rejection review and primary evidence: analysis/redirect/r002-residual-contract-review/rejection-review.md, primary-audit.json, audit.py. Added blocked residual leads, no ready leads. Gameplay/semantic continuations triaged against latest accepted graph; saved late-stage/ending evidence reused without replay. Coordinator must inspect reasoning before accepting terminal incomplete. No map/state/source/ROM/toolkit changes, no final editorial audit. No new toolkit issue; NES-001 unchanged/nonblocking.

## Coordinator rejection review accepted; final editorial acceptance active

Inspected compact redirect002 rejection reasoning, decisive primary audit and both
neighboring decoded ranges. See coordinator-rejection-review.md for scoped acceptance:
no corrected inference or useful discriminating method remains on current premises.
Terminal structural outcome is incomplete65550/65552, not whole-ROM complete.
Separate final_editorial_acceptance worker now performs targeted final editorial
checks and supported audit/verification, reusing valid prior review. Archive options
and requirement-by-requirement completion audit remain coordinator work.

## Terminal coordinator handoff — orchestration achieved, ROM incomplete

Two sequential phase rounds and final editorial acceptance complete. Coordinator
accepted justified terminal incompleteness after decisive rejection review; no ready
lead or useful revised method remains on current premises. No unfinished worker or
pending audit remains. Reopening criteria/history preserved, not a blocked live task.
ROM classification: 100.00% (65550 / 65552 bytes classified; 2 bytes unknown).
Exact unknown physical2991/A981 and44B2/C4A2 retained. Rounded100.00% is not complete.
Current checkpoint pass-002; current-source readability/editorial acceptance COMPLETE.
Supported commentary/audit.json: ok=true,byte_identical=true,editorial_audit=true;
current source6755f9046216be3d82b229be25aea37dbad7e976a3e3142c04ec2ea641879b73.
Coordinator fresh map/store validation and unique physical accounting passed in
coordinator-final-verification.json. Final handoff: terminal-handoff.md; requirement
completion audit: coordinator-completion-audit.md. Archive contract inspected and
archive-options.json finalized:34 Markdown,21 additional selected,3 duplicate setup
copies omitted,0 unaccounted; archive-document-selection.json records exact inventory.
All final results/handoffs and historical conclusions preserved. No archive execution,
workspace deletion, cleanup, toolkit edits or publication authorized/performed.
Previous turn made progress on document selection/editorial evidence; this turn
completed authoritative final verification and terminal preparation. Orchestration
goal may complete; whole-ROM classification must remain INCOMPLETE.
