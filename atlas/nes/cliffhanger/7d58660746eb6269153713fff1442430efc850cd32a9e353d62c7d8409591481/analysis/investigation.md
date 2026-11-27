# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 disassembly — completed pass, incomplete classification

ROM classification: 64.48% (169049 / 262160 bytes classified; 93111 bytes unknown).

Round 001 disassembly completed as one substantive easy-gain pass. Outcome: **incomplete ROM classification**. Next phase: full current-source commentary, then redirect investigation. This is not a terminal exhaustion decision.

Verified checkpoints: baseline (raw fidelity only), round-001-startup, and final **round-001-disassembly**. Final portable mapping maps/round-001-disassembly.json; working analysis/state.json and maps/current.json; source generated/round-001-disassembly/game.asm; verification analysis/round-001-disassembly-verification.json confirms ok=true, byte_identical=true. Input SHA256 unchanged, 7d58660746eb6269153713fff1442430efc850cd32a9e353d62c7d8409591481. Toolkit configured binary used exclusively; no toolkit edit/archive/publication/cleanup.

Classification evidence: header and physical section sizes from parsed iNES; CHR 131088..262160 is separate pattern-image storage; vectors physical131082..131088 encode C000/FF97/C14A in fixed bank15. Explicit bank-aware opcode views from validated roots, actual contextual instruction fetches, and connected static paths, preserving mapper barriers. Inspect contexts.json, contexts-shortcuts.json, contexts-mechanics.json and discovery/final-discovery JSON files. No conflicting candidate instruction starts were accepted. 91 constant-data runs (24,924 bytes) have exact boundaries, neighboring bytes and rationale in uniform-data-evidence.json. Nearby references/connected code checked; no semantic table/fill claim made from uniformity. Raw output gaps remain unknown.

Execution and annotations: 13,031 code bytes, 156,018 data/image/header/vector bytes; 616 evidence-bearing regions. Union of preserved contextual files contains 5,336 distinct ROM opcode starts; execution coverage counts opcode starts, separate from structural classified bytes. Trace mechanics summary reports 7,274 total unique instruction addresses including non-ROM execution, 7,446 contextual roots, aggregate_dropped=0. Zero-detail tracing deliberately drops event details; exact accepted-fetch/context aggregates persist. RAM instruction fetches do not establish any ROM upload origin. Full routine labels, RAM variables and current-source commentary are still pending; byte identity is not readability completion.

Gameplay: natural Sony/copyright/title and attract/demo cabin reached. **Controller correction NES-001 supersedes earlier attempted-control claims:** {button,pressed} shared-protocol requests silently had no effect; actual NES API is {port:1,buttons:[...]} and buttons:[] releases. correct-inputs.py, intro-controls.py and mechanics.py use valid API. A valid title Start press opens helicopter story; eight Start10/release20 pulses,120-frame settling, and Right10/Right+A10/Right+B10/Up+B10 produce player-started level1 movement/climb from ground to cabin upper-left (natural-play-before/after.png). No artificial patch used in this route.

Artificial progression: saved clean gameplay snapshot, restored for each target; patch RAM0404=target-1,0708=2 (CPU fixed bank14 stage-status handler),240 untraced frames plus10 traced frames. Stage4 snowy forest visible,0404=4/0708=0; stage14 cliff hanging visible,14/0; stage8 Level Completed interstitial,8/2; terminal15 Gabe/Jessie mountain/kiss dialogue,15/2. These establish artificial state observations, not natural late-game reachability. Valid mechanics retest in mechanics.py: forest route returns to title after a fall/death; cliff route displays ladder/cliff platform and reduced lives (before/after captures). Stage8 gameplay itself remains unconfirmed. Start30/release120 and controls in stage8/ending stay in presentation states. Ending dialogue visited, staff credits not confirmed. Boss-specific combat not confirmed; needs stage/actor setup contract rather than blind RAM edits. See gameplay.md and scenarios.json.

Lead outcomes: no initial ready leads (original leads.json empty). Verified reset/vector connected code; verified title input contract after correcting shared-schema attempt; verified stage0404/status0708 progression and terminal selector12; verified bounded constant runs as generic non-code. Initial ineffective input method superseded, not silently discarded. No structural rejection inherited. Stage8/credits continuation remains a redirect question, not proof of unreachable/dead content.

Unresolved and concrete next tasks: (1) Commentary explain current fixed-bank main loop, input/presentation $EDA3/$EDF4, mapper helper $D58E, and stage consumer $0404/$0708, preserving exact aliases. (2) Redirect inspect $ECCC presentation selector records and $CC55/$CC83 setup plus fixed-bank CPU $CE12/$CE21 stage tables; find stage8/credits/boss setup entry contracts. (3) Banks8..11 have repeated 1024-byte constant-field/record patterns and exact annotated lanes, promising static non-code extension; inspect consumers/neighboring references before interpreting complete records. (4) Sound bank6 connected code uses zero-page FA..FC and RAM07xx state; inspect dispatch/indirect target barriers. Exact residual complement unknown-ranges.json (179 ranges), per-bank counts bank-counts.json. Do not replay completed experiments without changed question; continue connected investigations rather than stopping at a first pointer.

Toolkit report: **one confirmed bug**, NES-001 input API documentation/validation mismatch, workaround valid buttons lists, no blocker. analysis/toolkit-issues.md has reproduction, commit and correction impact. Long initial server/step exited SIGTERM143 for uncertain external reason; preserved logs/snapshot and recovered bounded startup traces, no toolkit causality claimed. NES discovery implementation uses max_instructions (not shared budget); scripts use actual supported parameter. Disk at last check ~3.6GiB free; selected captures/snapshots local, no deletion.

| Physical bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 0 | 0 | 1252 | 6940 |
| PRG 1 | 0 | 1496 | 6696 |
| PRG 2 | 0 | 1606 | 6586 |
| PRG 3 | 0 | 865 | 7327 |
| PRG 4 | 0 | 120 | 8072 |
| PRG 5 | 0 | 2546 | 5646 |
| PRG 6 | 1567 | 383 | 6242 |
| PRG 7 | 0 | 3294 | 4898 |
| PRG 8 | 0 | 4031 | 4161 |
| PRG 9 | 0 | 3319 | 4873 |
| PRG 10 | 0 | 3808 | 4384 |
| PRG 11 | 0 | 2053 | 6139 |
| PRG 12 | 2650 | 1 | 5541 |
| PRG 13 | 1103 | 86 | 7003 |
| PRG 14 | 4380 | 0 | 3812 |
| PRG 15 | 3331 | 70 | 4791 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

## Round001 redirect, attempt round001-a

Supported combined static batch ready: geometry15 linked containers/endpoints; presentation descriptors/nametables/text/palettes; all actor/player/special handler entry domains with bounded call-summary closure; counted world/actor/profile data; six sound handlers and bounded generic sound storage.83231 proposed currently unknown bytes,0 accepted in this phase. Exact positive/negative evidence, improved caller-summary method and localized remaining prerequisites in analysis/redirect/round001-a/frontier.md, contract-review.md and analysis/handoffs/round-001-redirect.md. Historical observations preserved. No terminal exhaustion assessment.


## Round 002 disassembly — fresh verified checkpoint

Active phase/checkpoint: **round002 disassembly complete — round-002-disassembly**. Outcome: **incomplete**,56 unknown gaps; not a terminal exhaustion decision. Five ready redirect leads independently verified and consumed. Added89717 bytes:83231 combined proposed unknown bytes (72971Data/10260Code),6448 further bounded residual Data and38 direct-control-edge Code. Final23329 Code/235437 other structurally classified bytes. All additions Static; no new natural reachability or observed-coverage claim. All616 prior annotations and their IDs/labels/comments/evidence/views exactly preserved against maps/round-001-commentary.json.

ROM classification: 98.71% (258766 / 262160 bytes classified; 3394 bytes unknown).

Independent evidence: analysis/round-002/audit-summary.json, audit-rpc.jsonl and review.md;6707 exact bank-aware instruction views redecoded in78 bounded batches;15 geometry chain unions and descriptor/text/animation/world record formats independently reconstructed. No code/data/start conflicts. Stage14 accepts82 encoded columns without asserting nominal96 completeness. Generic data retains unresolved subformats after practical consumer/reference/pattern inspection. Extra exact intervals/rationale in extra-data-review.json; connected2 direct-edge roots yield20instructions/38bytes, zero diagnostics.

Verification: map.validate ok; working analysis/state.json and maps/current.json saved. Fresh frozen maps/round-002-disassembly.json and generated/round-002-disassembly/game.asm; analysis/round-002-disassembly-verification.json reports ok=true/byte_identical=true. ROMSHA unchanged7d58660746eb6269153713fff1442430efc850cd32a9e353d62c7d8409591481; mapSHA176ba275693b0dc750fb67202ed95d9bee6515bcb06b68af0226133381b2563a. Commentary/source unchanged: readability approval remains input169049bytes;89717 new bytes require commentary, final editorial audit pending. Fidelity alone is not readability.

Gameplay: reuse natural level1 controls and artificial stage4/8/14/terminal15, stage8 mechanics and credits; scenarios.json recipes/results preserved. No unchanged replay, no ROM patch, no new artificial-state claim. Boss combat remains attempted/unconfirmed.

Next phase commentary, then evidence-informed redirect if coordinator selects it. Compact handoff analysis/handoffs/round-002-disassembly.md; authoritative residual frontier analysis/round-002/frontier.md, exact gaps unknown-ranges.json, per-bank totals bank-counts.json and consumed outcomes lead-outcomes.json. Concrete questions: F26B now has balanced PPU-only preservation contract for fixed-bank text/presentation continuations; CA00 setup entry; sound alternate branches;9324 callback origin/991D pointer/D80F matching record. No exhaustive recursive closure or terminal rejection claimed.

Toolkit report: existing confirmed NES-001 remains nonblocking via complete buttons lists; **no new toolkit issues found**. Preserve [toolkit issue report](toolkit-issues.md). Configured binary only; no toolkit edits/archive/publish/deletion.

| Physical bank | Code | Data/image/header | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 0 | 0 | 8192 | 0 |
| PRG 1 | 0 | 8192 | 0 |
| PRG 2 | 0 | 8192 | 0 |
| PRG 3 | 0 | 8176 | 16 |
| PRG 4 | 0 | 8192 | 0 |
| PRG 5 | 0 | 8192 | 0 |
| PRG 6 | 1587 | 6569 | 36 |
| PRG 7 | 0 | 8192 | 0 |
| PRG 8 | 0 | 8192 | 0 |
| PRG 9 | 0 | 8192 | 0 |
| PRG 10 | 0 | 8192 | 0 |
| PRG 11 | 0 | 8192 | 0 |
| PRG 12 | 6327 | 632 | 1233 |
| PRG 13 | 7000 | 980 | 212 |
| PRG 14 | 4843 | 2204 | 1145 |
| PRG 15 | 3572 | 3868 | 752 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

## Round002 redirect
Attempt analysis/redirect/round002-a combines suppliedcallback/RAM domains, mapper restoration and boundedindependentsemanticcontracts. Ready3305Code+73Data, map unchanged. Detailedmethods/results andpriorbarriercorrections incontracts.md; exactresidualledger includesall56currentgaps. Fourprojectedtinyfragments remain ambiguous; no terminalclaim. Improvedbosscontact testno callbackfetch; seeboss-result.md. Next independentdisassemblyreview thencommentary/redirect.


## Round003 disassembly — verified structural batch, incomplete

ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).

Added 3378 bytes (3305 Code, 73 Data), all Static. Exact independent review: [review](round-003/review.md), [audit](round-003/audit-summary.json), [decoded views](round-003/exact-decodes.json). All prior regions, comments, labels, evidence and views preserved exactly; ROM SHA unchanged. Two R002 ready leads verified and consumed, history retained in leads.json and [outcomes](round-003/lead-outcomes.json). No new runtime/natural selection claim. No repeat broad gameplay: saved natural level1/artificial later stages/ending evidence remains valid, boss combat remains unconfirmed.

Fresh checkpoint round-003-disassembly; map.validate successful and analysis/round-003-disassembly-verification.json confirms byte-identical rebuild. Current state/current map saved. Structural completion remains incomplete: four gaps physical100415+1 ($882F),108048+6 ($A600),108281+5 ($A6E9),111386+4 ($B30A); bounded alternatives in prior residual-review.md and new review. No speculative fragment classification or terminal exhaustion claim. Current commentary was approved for258766bytes;3378 newly classified bytes require full current-source commentary and final editorial remains separate.

No new toolkit issues found. Existing confirmed NES-001 is nonblocking with complete-button-list workaround; [toolkit report](toolkit-issues.md). No toolkit edits, archive, publication, deletion or ROM/asset downloads. Next: coordinator commentary, then redirect as useful.

| Physical bank | Code | Other classified | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 0 | 0 | 8192 | 0 |
| PRG 1 | 0 | 8192 | 0 |
| PRG 2 | 0 | 8192 | 0 |
| PRG 3 | 0 | 8192 | 0 |
| PRG 4 | 0 | 8192 | 0 |
| PRG 5 | 0 | 8192 | 0 |
| PRG 6 | 1623 | 6569 | 0 |
| PRG 7 | 0 | 8192 | 0 |
| PRG 8 | 0 | 8192 | 0 |
| PRG 9 | 0 | 8192 | 0 |
| PRG 10 | 0 | 8192 | 0 |
| PRG 11 | 0 | 8192 | 0 |
| PRG 12 | 7514 | 677 | 1 |
| PRG 13 | 7197 | 980 | 15 |
| PRG 14 | 5988 | 2204 | 0 |
| PRG 15 | 4312 | 3880 | 0 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |


## Round004 disassembly — verified wrapper, incomplete

ROM classification: 100.00% (262150 / 262160 bytes classified; 10 bytes unknown).

Added6 Static Code bytes at physical108048 CPU$A600 bank13 under[12,13,14,15]. Independent primary review accepted complementary animation producer/consumer and ordinary return/stack contracts plus exact terminal bounds; no runtime/natural reachability claim. R003-redir-animation-wrapper verified and consumed. All7347 prior regions preserved exactly. [Review](round-004/review.md), [outcome](round-004/lead-outcomes.json), [exact counts](round-004/counts.json).

State/current map saved; fresh checkpoint round-004-disassembly validates and rebuilds byte-identically (analysis/round-004-disassembly-verification.json). Remaining100415+1,108281+5,111386+4 are materially ambiguous code/data, with retained bounded reviews; no forced classification. Connected paths already classified. Existing progression/scenarios reused, no redundant gameplay. Full current-source commentary and final editorial remain separate. No terminal exhaustion claim.

No new toolkit issues found; existing nonblocking [NES-001](toolkit-issues.md) retained. No toolkit edits/archive/publication/deletion.

| Physical bank | Code | Other classified | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 0 | 0 | 8192 | 0 |
| PRG 1 | 0 | 8192 | 0 |
| PRG 2 | 0 | 8192 | 0 |
| PRG 3 | 0 | 8192 | 0 |
| PRG 4 | 0 | 8192 | 0 |
| PRG 5 | 0 | 8192 | 0 |
| PRG 6 | 1623 | 6569 | 0 |
| PRG 7 | 0 | 8192 | 0 |
| PRG 8 | 0 | 8192 | 0 |
| PRG 9 | 0 | 8192 | 0 |
| PRG 10 | 0 | 8192 | 0 |
| PRG 11 | 0 | 8192 | 0 |
| PRG 12 | 7514 | 677 | 1 |
| PRG 13 | 7203 | 980 | 9 |
| PRG 14 | 5988 | 2204 | 0 |
| PRG 15 | 4312 | 3880 | 0 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

Round004 redirect: reviewed strongest residual claims against primary bytes/current source and prior scoped negatives in analysis/redirect/round004-a/review.md. Three tiny fragments remain code-versus-data ambiguous; A600 combined-contract inference already accepted. No useful changed inference/test remains; terminal incomplete recommendation requires coordinator primary-evidence inspection. Exact complement and gameplay/shortcut review saved with round004 handoff. No interruption frontier, map changes or arbitrary replay.

## Explicit user-directed residual fallback — disassembly phase

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Exact internal complement was verified before mutation: 10 bytes only at physical 100415+1 (PRG12 CPU $882F), 108281+5 (PRG13 CPU $A6E9), 111386+4 (PRG13 CPU $B30A), effective PRG [12,13,14,15]. Accepted exactly these as generic Data of unknown meaning with Manual provenance under explicit user authorization. This is a user-directed classification convention, NOT new proof of a non-code structural role. Prior code/data ambiguity, possible TXA bridge, INC/INC/RTS helper and timer-clear entry remain unresolved; no new execution or natural reachability claim. Prior bounded references, neighbors, decode and candidate reviews are retained in [round004 residual review](redirect/round004-a/review.md) and its linked primary artifacts. Confidence 1 records certainty of the manual convention, not empirical proof.

All 7348 existing regions preserved exactly. Now 26640 Code, 235520 other classified (including 10 Manual fallback bytes), zero unknown; 262150 bytes retain prior evidence-backed classification. [Before/after and per-bank counts](manual-residual-fallback/counts.json), [RPC audit](manual-residual-fallback/rpc.jsonl). map.validate passed; analysis/state.json saved and maps/current.json exported. Fresh checkpoint manual-residual-fallback validates and rebuilds byte-identically: [verification](manual-residual-fallback-verification.json). ROM SHA unchanged. No new research, gameplay, toolkit edits, deletion, archive or publication. Full current-source commentary rebase and final editorial remain separate pending phases.

No new toolkit issues found; existing confirmed nonblocking NES-001 and complete-buttons workaround retained in [toolkit issues](toolkit-issues.md). Disk inspected before batch: 3.3 GiB free; no exploratory scratch created.

| Physical bank | Code | Other classified | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 0 | 0 | 8192 | 0 |
| PRG 1 | 0 | 8192 | 0 |
| PRG 2 | 0 | 8192 | 0 |
| PRG 3 | 0 | 8192 | 0 |
| PRG 4 | 0 | 8192 | 0 |
| PRG 5 | 0 | 8192 | 0 |
| PRG 6 | 1623 | 6569 | 0 |
| PRG 7 | 0 | 8192 | 0 |
| PRG 8 | 0 | 8192 | 0 |
| PRG 9 | 0 | 8192 | 0 |
| PRG 10 | 0 | 8192 | 0 |
| PRG 11 | 0 | 8192 | 0 |
| PRG 12 | 7514 | 678 | 0 |
| PRG 13 | 7203 | 989 | 0 |
| PRG 14 | 5988 | 2204 | 0 |
| PRG 15 | 4312 | 3880 | 0 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |
