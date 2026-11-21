# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 — disassembly (active)

Consumed leads: none (initial leads.json empty). Vector contracts and initial
53 candidates inspected; observed contexts preserve physical bank identities.
First-pass classification batch prepared in `analysis/round1/annotate.py`, grounded
in accepted opcode fetches and conservative control-flow traversal. Repeated
packed data and exact FF suffix inspection recorded in `round1/data-inspection.md`.

Progression attempts: $0401 stage index hypothesis confirmed through stage selector
reads and live RAM. Patching only this field during story exposed different enemy
records but retained stale Round01 layout; therefore it does not prove a coherent
later-stage visit. Patches30,80,112 and111-before-increment retained as rejected
full-initialization recipes, useful partial selector mechanics evidence. Do not
repeat unchanged. Current bounded alternative patches $049C highest-round and
$0401 current-round on actual menu then uses 1P Continue (Select twice), allowing
ordinary initializer control flow. Results pending. $C672 has a plausible stage
initializer contract but register injection is not justified: mainline is normally
in $E90C–$E913 wait loop with stackFF, while interrupt/task dispatcher contract is
unresolved. Need caller/task-state analysis before any PC edit.


## Round 1 final disposition

Current map/checkpoint pass-003: 133132 classified,30724 unknown of163856
(81.249389708036%; rounded81.25%). Code8970,Data91394,Image32768.
See round1-disassembly-handoff.md for authoritative calculation/per-bank counts.
Static direct-call closure consumed connected supported batch to zero gains.
Final additional Continue confirmation succeeded: giant boss arena visibly verified
in round1/confirmedB4.ppm, accepted bank10 roots and exact snapshot retained.
Prior stage-only mixed-state rejection remains limited to those recipes and is
superseded as a progression obstacle by the supported menu route. Ending still
unobserved; boss-state/defeat contracts now concrete redirect questions. No terminal
exhaustion determination made. Toolkit issue NES-001 confirmed, no blocker.

### Round2 disassembly — verified pass-004, incomplete

ROM classification: 92.01% (150757 / 163856 bytes classified; 13099 bytes unknown).
Added17625bytes:11226Code+6399Data from all five ready R1 structural leads.
Totals:Code20196,Data97793,Image32768;155 remaining unknown intervals.
All9643 internal regions have structural evidence. Code provenance:7676Observed,
12520Static bytes; observed does not mean natural (includes prior artificial routes).
New observed subset101instructions/203bytes; two fixed-bank actual context additions
preserved. No new gameplay runs. Reused verified Round01/30/80/boss/twoBAD END/HAPPY END.

| PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
|0|0|8192|8192|0|
|1|0|8192|8192|0|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|0|8192|8192|0|
|6|0|8192|8192|0|
|7|0|8192|8192|0|
|8|934|7252|8186|6|
|9|0|8192|8192|0|
|10|1536|1176|2712|5480|
|11|0|8192|8192|0|
|12|5496|370|5866|2326|
|13|1096|6744|7840|352|
|14|6457|215|6672|1520|
|15|4677|100|4777|3415|

Header16Data and CHR32768Image fully classified, outside PRG bank counts.
Independent review: analysis/round2/review.md, review-decodes.json, data-boundaries.json;
full supported batch accepted after bank/stack/inline/finite-table consumer inspection.
Explicit wrong-bank98662 KIL operand excluded; boss-conditioned bank10 veneer accepted.
Local resume correction preserved additional actual fixed-bank contexts at118096/118099;
no toolkit defect. State saved, maps/current.json exported and map.validate passed.
Checkpoint pass-004 validated/generated/verified byte_identical=true,ok=true;
analysis/pass-004-verification.json, maps/pass-004.json, generated/pass-004/game.asm.
Map SHA256563b559e3c5d2a1a0d676f2f7734997365a4ddd6bd236f11454e305d2a5f2f41.
Input ROM unchanged; SHA256fc521e891d074369d67579fd56617460b9c9949dec67cc4a37d46b4e4bb09d29.

Consumed verified:R1-residual-structure,R1-packed-records,R1-inline-contracts,
R1-finite-dispatch-data,R1-sound-noncode. Produced research:R2-bank-shadow-contract,
R2-task-stack-bank-contract,R2-residual-consumers. Exact residual analysis/round2/unknown.json;
call/bank frontier analysis/round2/frontier.json. FA81 bank shadow, CEE3 nonlocal stack,
unresolved pointer domains and heterogeneous bank10 gaps need further redirect research;
no unsupported data blanket and no terminal exhaustion claim. Connected supported batch
is closed; remaining barriers require these missing invariants, not replayed gameplay.

Existing commentary/source preserved unchanged, still verified for input pass-003 only;
full current pass-004 source commentary is pending. No commentary/redirect phase executed,
no publication/archive/toolkit changes or cleanup/deletion. Toolkit:no new issues found;
existing NES-001 confirmed documentation/API validation bug, explicit buttons-array
workaround, zero blockers. See analysis/toolkit-issues.md.

## Round3 disassembly — pass-005

ROM classification: 99.91% (163714 / 163856 bytes classified; 142 bytes unknown).

Accepted12957newStaticbytes:8371Code4586Data,4158regions. Fresh configured-binary decode4199views; zero decoder/physical/overlap discrepancies. Primary structural review: round3/review.md and review-decodes.json; exact counts round3/summary.json. Conditional FA81 mode3/shadow, eventual CEE3 task resumption, finite task0 initialization, bounded pointer/normal-branch contracts, independent bank10 collision routine and finite PPU copies reviewed. C5FD continuation includes all eight selected lower pairs and restored-pair C611. Reachability remains separate; no new observed execution.

Per physical8KiB PRG bank (classified/unknown): 0:8192/0, 1:8192/0, 2:8192/0, 3:8192/0, 4:8192/0, 5:8192/0, 6:8192/0, 7:8192/0, 8:8186/6, 9:8192/0, 10:8184/8, 11:8192/0, 12:8183/9, 13:8192/0, 14:8163/29, 15:8102/90. Header16/0; CHR32768/0. Provenance bytes: {'Observed': 7676, 'Static': 156038}. Structural kinds: {'Code': 28567, 'Data': 102379, 'Image': 32768}.

All three ready R2 leads verified and consumed; history appended in leads.json. Existing Round01/30/80/boss/BAD/HAPPY progression evidence reused; no unchanged replay. Remaining142Unknown in15intervals: round3/unknown.json and redirect/r2-20261121-a/residual.json. E902 has destructive reset-stack return barrier; other short instruction-like islands lack discriminating entry/boundary/control contracts. No blanket Data fallback. Connected queue exhausted; incomplete, no terminal rejection/completion claim.

map.validate passed; state saved and portable map exported. Fresh maps/pass-005.json and generated/pass-005 verified byte-identical by analysis/pass-005-verification.json. ROM unchanged. Existing pass004 commentary preserved; current pass005 commentary/readability and final editorial audit pending. No other phase/toolkit changes/publication/archive/cleanup. No new toolkit issues found; existing NES-001 and workaround retained in analysis/toolkit-issues.md.

## Round3 redirect complete — ready combined batch

Attempt redirect/r3-20261121-a retains pass005 map/state/source and142Unknown/15ranges. Proposed131Staticbytes (127Code+4Data), no accepted bytes: concrete resetSPFF producer and disjoint destructive-clear/task-seed frames, sibling controller/mask variants, duplicated36-byte sound-reset/disable neighborhood, bounded inline queue ABIs, OAM constant fill, result-mode gate and coordinate clamp. Detailed primary contract evidence and localized competing interpretations:redirect/r3-20261121-a/contracts.md; exact proposals/checks/ledger/handoff there. R3-reset-return andR3-static-variants ready in leads.json. If fully accepted11Unknown/5ranges; actual classification unchanged until independent disassembly.

Coherent connected and independent actionable avenues assessed. Remaining tinyphase-reset pair, isolatedRTS/JMP andFACE-like word stay ambiguous from insufficient independent structural discrimination; no universal actual-caller prerequisite and no forcedData. No terminal exhaustion assessment. Current-source readability remains complete at input pass005, fresh prior byte-identical verify; final editorial audit pending. No new toolkit issues found; existingNES-001 workaround/no blocker retained. No live server/replay/map/source/state change/toolkit/archive/publication/cleanup/deletion/goal completion.

## Round4 disassembly — pass-006

ROM classification: 99.99% (163845 / 163856 bytes classified; 11 bytes unknown).

Accepted131newStaticbytes (127Code+4caller-inlineData),65regions. Independent primary review in round4/review.md; fresh63proposal views plus743accepted-support decodes in round4/review-decodes.json/primary-decodes.json. Zero decoder/physical/overlap discrepancies. Concrete TXSFF reset return frame survives0110..01CF clear and012E/F task seeds; stable controller and mask siblings, duplicated36byte sound neighborhood, known FA04/F922 inline ABI, actual absolute-LDA F4 OAM constant fill, mode-consistent panel selector and paired coordinate clamp supported. Structural classification only, natural entry unresolved; no new observed execution. Prior destructive arbitrary-SP objection localized/corrected, history preserved.

Per physical8KiB PRG bank(classified/unknown): 0:8192/0, 1:8192/0, 2:8192/0, 3:8192/0, 4:8192/0, 5:8192/0, 6:8192/0, 7:8192/0, 8:8192/0, 9:8192/0, 10:8192/0, 11:8192/0, 12:8183/9, 13:8192/0, 14:8190/2, 15:8192/0. Header16/0; CHR32768/0. Provenance bytes: {'Observed': 7676, 'Static': 156169}. Structural kinds: {'Code': 28694, 'Data': 102383, 'Image': 32768}. Exact counts:round4/summary.json.

Both R3 ready leads verified/consumed with appended history. Remaining11Unknown in5ranges:round4/unknown.json. Bank12 8AE1+3/8B0E+2 candidate linked phase-reset,8E24+1 isolatedRTS,9525+3 isolatedJMP; fixedC1E3+2 FACE-like word. Independent entry/extent or code-versus-data discrimination remains absent; no speculative genericData. Connected supported structures already classified, no further easy gain identified. Reuse prior later-level/boss/BAD/HAPPY progression evidence; no unchanged replay. Incomplete ordinary handoff, no terminal-exhaustion or full-readability claim.

map.validate passed; saved analysis/state.json/exported maps/current.json. Fresh pass006 map/generated source verified ok=true,byte_identical=true in analysis/pass-006-verification.json. ROM unchanged. Entire pass005 commentary/source preserved at recordedSHA256; pass006 commentary expansion/final editorial audit pending. No other phase executed, toolkit change/publication/archive/cleanup/deletion/goal completion. No new toolkit issues found; existing NES-001 workaround retained in analysis/toolkit-issues.md.

## Round5 disassembly — pass-007

ROM classification: 99.99% (163847 / 163856 bytes classified; 9 bytes unknown).

Accepted2Static genericData bytes physical115187+2 /fixedC1E3. Independent review of actual precedingC1DFJSR/C1E2RTS boundaries, inlineE5C1 and descriptor00/count02 F992 consumer proves32byte paletteC1E5..C205. CE3byteDEC overlaps it; FA1byte nonterminalNOP requires successor in it. Strongest overlapping-code/detachedNOP alternatives assessed explicitly under accepted nonoverlap model; no hardware impossibility or FACE-pointer meaning asserted. Fresh91configured-release contextual decodes; no new execution/natural reachability. Evidence:analysis/round5/review.md, fresh-decodes.json, accepted-neighbors.json and annotation.json.

Per physical8KiB PRG bank(classified/unknown): 0:8192/0, 1:8192/0, 2:8192/0, 3:8192/0, 4:8192/0, 5:8192/0, 6:8192/0, 7:8192/0, 8:8192/0, 9:8192/0, 10:8192/0, 11:8192/0, 12:8183/9, 13:8192/0, 14:8192/0, 15:8192/0. Header16/0; CHR32768/0. Provenance:{'Observed': 7676, 'Static': 156171}. Structural kinds:{'Code': 28694, 'Data': 102385, 'Image': 32768}. Exact counts:analysis/round5/summary.json.

R4-c1e3-boundary-data verified with appended history. Remaining9Unknown4bank12ranges:101105+3,101150+2 linked potential wholefieldclear,101940+1 isolatedRTS,103733+3 JMP97C9. Accepted family/store/target bounds are acknowledged, but do not independently assign the candidate operations. Reused compact primary rejection review with scoped competitors; no caller mandate, unchanged gameplay replay or unsupported genericData. Connected supported structures already classified. Incomplete ordinary handoff; no terminal-exhaustion or editorial acceptance claim.

map.validate passed, state saved/map exported. Fresh pass007 verified ok=true,byte_identical=true:analysis/pass-007-verification.json. Entire pass006 commentary/source preserved SHA256bf24d353c74443d753aec2b792ebe8c599ad902d1b62cfa815680d7dc90cebe7; pass007 commentary update remains separate. ROM unchanged. No other phase, toolkit modification, live emulator/server, archive, publication, cleanup/deletion or goal completion. No new toolkit issues found; existing NES-001 workaround retained in analysis/toolkit-issues.md.

## Manual fallback disassembly — pass-008

ROM classification: 100.00% (163856 / 163856 bytes classified; 0 bytes unknown), including 9 user-authorized Manual fallback bytes.
Evidence-backed classification remains 99.99% (163847 / 163856); the 9 Manual bytes are not structural proof. This phase does not claim whole-ROM evidence-backed completion or final editorial acceptance.

Input checkpoint: pass-007. Before any annotations, exact internal-map complement was verified as 9 bytes, within the user's <=10-byte exception: physical offsets 101105+3 ($8AE1), 101150+2 ($8B0E), 101940+1 ($8E24), 103733+3 ($9525), all physical PRG bank12 with CPU alias shown for lower mapped slot. These exact ranges alone were annotated generic Data through supported map.annotate with Manual provenance. The explicit user instruction authorized fallback despite unresolved code-versus-data ambiguity, required unknown meaning and preserved history, and prohibited redirect research. Manual confidence1.0 records certain authorization, not inferred role certainty.

Prior ambiguity remains unresolved: linked candidate whole-field phase clear (5 bytes), isolated RTS (1), isolated JMP (3). No semantic label, execution, natural reachability, unused/dead status or independent structural evidence was invented. See ../round5-disassembly-handoff.md and ../redirect/r4-20261121-a for primary historical reasoning; all records preserved. No leads consumed/produced and no research rerun.

Before/after checks prove all existing internal regions identical, exactly four new Manual Data regions totaling9 bytes, and unchanged ROM/commentary source. Supporting artifacts: complement-before.json, input-identities.json, annotations.json, map-validation.json, preservation-check.json and summary.json here. Totals: Code28694, Data102394, Image32768; provenance Observed7676, Static156171, Manual9. All16 PRG banks8192 classified/0 unknown; bank12 consists8183 evidence-backed +9 fallback bytes. Header16 and CHR32768 fully classified.

map.validate passed; analysis/state.json saved and maps/current.json exported. Fresh maps/pass-008.json and generated/pass-008/game.asm rebuild byte-identically: ../pass-008-verification.json (ok=true, byte_identical=true). ROM SHA256 fc521e891d074369d67579fd56617460b9c9949dec67cc4a37d46b4e4bb09d29. Map SHA256 9bf5919f780a425ad55d877612ea96407d65c4ddb1ce910b0d1f7141eab8a89e. All exact output hashes in summary.json.

Next: separate commentary phase must preserve and reconcile existing source with four fallback ranges, explain unknown meaning/manual policy explicitly, refresh reading copy and run fresh verification; then final editorial acceptance. Current source unchanged by this phase; prior commentary credit remains historical. No other phase, toolkit change, archive, publication, cleanup/deletion or goal completion. No new toolkit issues found; existing ../toolkit-issues.md retained (NES-001 and AUDIT-002 workarounds). All internal ROM, state, traces, assets and source stay local; supported ROM-free portable mapping is the only sharing deliverable.
