# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.


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

Detailed primary artifacts: round001-final-static-candidates.json, round001-review-decoded.json, round001-split-dispatch-decode.json, round001-runtime-accepted.json; full request/reply ledger round001-rpc.jsonl. Runtime merge found zero classification conflicts.

## Redirect001 — combined static contracts/tables proposal
See redirect-round-001-handoff.md and redirect/r001-static-contracts/findings.md.
Connected primary/secondary/effect/event graphs and independent helper/data
avenues investigated;5463Code+1179Data=6642 proposed,zero accepted. Exact map
complement6644 remains; hypothesis after acceptance leaves loneRTS bytes2991/44B2.
No new reachability claims; generic non-code allowed after consumer/boundary review.
Inventory6/7 static producers and actual25FC mask investigated mechanically.
Ready leads updated with durable recipes; localized residual review preserves
uncertainty without propagating missing-caller prerequisites to coherent code/data.
No terminal exhaustion or ROM-completion claim. Existing NES-001 only, no blocker.

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

## Round002 redirect — residual role review complete, incomplete ROM

Pass002/current SHA feba0c385e2b7e2ec6e964e25af6f0b1f8121850804176f0c1bc6ebf4a2ba424; full current-source readability already complete. Fresh current-map operand/boundary and target-word audit confirms unknown physical2991/A981 and44B2/C4A2, each60. Reviewed independent contracts and limited negative scopes; no supported role proposal or discriminating next test. Exact rejection review and primary evidence: analysis/redirect/r002-residual-contract-review/rejection-review.md, primary-audit.json, audit.py. Added blocked residual leads, no ready leads. Gameplay/semantic continuations triaged against latest accepted graph; saved late-stage/ending evidence reused without replay. Coordinator must inspect reasoning before accepting terminal incomplete. No map/state/source/ROM/toolkit changes, no final editorial audit. No new toolkit issue; NES-001 unchanged/nonblocking.
