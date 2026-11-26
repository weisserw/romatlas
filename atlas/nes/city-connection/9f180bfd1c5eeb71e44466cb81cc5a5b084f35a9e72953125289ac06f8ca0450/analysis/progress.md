# Disassembly progress

Round 1 disassembly active. Baseline maps/baseline.json and generated/baseline verified byte-identical (analysis/baseline-verification.json). ROM SHA unchanged. Game title observed: City Connection, 1985 Nintendo/Jaleco NTSC CNROM image.

Validated vectors: NMI CPU $838D/physical 925; reset $D000/20496; IRQ $842E/1086. Bounded static traversal (max_instructions 30000, no observed edges) finds 4233 candidate instructions /8764 summed bytes, one overlapping byte at physical 2674 requiring review; indirect jump barrier physical 638. Evidence analysis/disassembly/pass-001/static-candidates.json and static-summary.json. These are candidates, not yet classifications.

Startup traced 120 frames with no input. Persistent startup.snapshot and startup.ppm saved in analysis/disassembly/pass-001; trace summary startup-trace.json. Server session closed; next session must emulator.start, snapshot.import, and reassert buttons. No map state saved yet. Next inspect game start, broad progression contracts and review candidate conflict before annotating. No toolkit issue confirmed. Baseline raw export is not classification.

Intermediate checkpoint pass-001 verified byte-identical. Classified 41,577 /65,552 bytes (63.43%), unknown23,975 before subsequent expansion. Static code8,755 bytes in38 runs; CHR32,768 image; header16/vectors6/OAM template20/dispatch12 data. Runtime corrected NES input to buttons array, after invalid documented shape silently released everything (NES-001). Proper Start enters level initialization; 8*65536 delay loop at CPU $D201 explains early transition frames. Current active progress-game.py session continues bounded level play; expand.py follows return-address dispatch targets and uniform noncode runs. No terminal exhaustion claim.

## Pass 001 final handoff — incomplete, ready for commentary/redirect

ROM classification: 99.19% (65022 / 65552 bytes classified; 530 bytes unknown).

Final internal state analysis/state.json; portable map maps/current.json; verified frozen checkpoint maps/pass-001-final.json, generated/pass-001-final, analysis/pass-001-final-verification.json. Byte-identical=true, ok=true; map SHA25642fbe24d52edf33f94a70cefa07d77ade66950c769fe284a1e4fec359592757c. Input SHA256 remains workspace identity; no toolkit or ROM changes.

Counts (physical8KiB banks; Data includes generic noncode and exact uniform runs):

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header |0|16|0|
| PRG0 |5500|2287|405|
| PRG1 |1398|6794|0|
| PRG2 |2258|5809|125|
| PRG3 |25|8167|0|
| CHR0 |0|8192|0|
| CHR1 |0|8192|0|
| CHR2 |0|8192|0|
| CHR3 |0|8192|0|

Total Code9181, Data23073, Image32768. Every classified region has structural evidence; readable-source editorial work has not run. Runtime coverage separate:1697natural contextual instruction roots (New York),1796artificial late-scenario roots; zero aggregate drops, retained detail0by design. All observed fetches fall within accepted static Code; no claim every classified instruction executed.

Natural New York driving/jump/oil visited; artificial bonus and Tokyo visibly visited and exercised. Evidence/recipes analysis/scenarios.json, gameplay.md and disassembly/pass-001. Tokyo screenshot late-stage.ppm; source loop contract $D216 wraps stage6to0 and increments$A1, runtime terminal/loop remains deferred. Disassembly followed substantial vector/static, connected dispatch and generic noncode easy gains. Remaining530bytes in4mixed/code-contract ranges require targeted redirect, with concrete D001..D004 recipes in leads.json. This is a nonterminal handoff; do not interpret as exhaustive investigation or full completion.

Toolkit report analysis/toolkit-issues.md:1confirmed documentation/API mismatch NES-001,0suspected bugs,0missing features, no blockers. No toolkit fixes. All servers closed; snapshots exported for replay. Handoff analysis/disassembly/pass-001/handoff.md.

## Coordinator phase transition — round 1 commentary

Disassembly handoff accepted as nonterminal at maps/pass-001-final.json; byte-identical verification inspected. Classification percentage printed to user before commentary: 99.19% (65022 / 65552; 530 unknown). No input leads consumed; D001–D004 produced for later redirect review. Separate worker round01_disassembly finished with servers closed. Separate sequential worker round01_commentary now owns current-source readability; input handoff analysis/disassembly/pass-001/handoff.md. Redirect must wait for full readability scope and fresh verification. Archive document-selection contract inspected in /root/romatlas/scripts/archive.py: automatic FINAL paths plus explicit additional final Markdown, explicit omissions, special exclusions, and source/before traversal exclusions; final archive-options.json inventory remains pending. No archive execution authorized.

## Round 1 commentary completed

Full current-source readability complete: 95 reviewed scaffold regions, 298 functional/structure units, all 39 Code entries explained, all Data/Image/Unknown boundary notes. Fresh commentary/verification.json ok=true/byte_identical=true; reading-after.asm refreshed. Local maintained source commentary/source/game.asm + symbols.inc, compact handoff analysis/commentary/round-001/handoff.md and result.json. Classification/map/state unchanged: 65022/65552 (99.19%), 530 unknown; map SHA25642fbe24d52edf33f94a70cefa07d77ade66950c769fe284a1e4fec359592757c. No source frontier remains. Final editorial audit remains pending; no audit run.

Commentary produced concrete consumer-derived questions C001 (pitch table's first twelve bytes currently Code, physical9403..9415) and C002 (first road pointer byte currently Code, physical1620..1621), retained unchanged for redirect alongside D001..D004. Details commentary/redirect-questions.md; limitations commentary/uncertainties.md. No runtime/classification/redirect/toolkit/archive/publication/cleanup actions. Input ROM and frozen artifacts unchanged; outputs local.

## Coordinator phase transition — round 1 redirect

Commentary handoff analysis/commentary/round-001/handoff.md inspected and accepted for current-source readability: all95regions,39Code regions,298reviewed units; no remaining frontier. Fresh commentary/verification.json byte_identical=true and named source/reading identities recorded. Map/state unchanged at pass-001-final. Separate worker round01_commentary complete; separate sequential round01_redirect now owns research. Input D001–D004 and commentary questions C001/C002; no classification change or terminal decision yet. Final editorial audit and final archive-options inventory remain pending.

## Round 1 redirect completed — combined ready batch

Unique research analysis/redirect/round-001-a/handoff.md. All D001–D004 reviewed with consumer/shared-helper/encoded-hook contracts; proposals cover530unknown bytes (429Code/101Data). C001/C002 newly ready for13-byte Code→Data correction after exact always-taken branch proofs. analysis/leads.json preserves history; proposals/residual-ledger/primary-checks/incoming audit and identity recheck durable. Accepted0newbytes; maps/state/source/ROM unchanged at42fbe24d...; current65022/65552,530unknown. Independent disassembly review required.

Broad artificial visits now cover later stages1..4 plus supported stage5completion→0loop with initialized first-city rendering and controls. Scenarios/gameplay updated; no distinct ending shown. All1879saved artificial contextual roots lie within existing Code. Title counter commentary premise corrected statically (shared epilogue does call IncrementFrameCounter); natural90frame wait timed out, no attract visit. No new toolkit issue, no archive/publication/cleanup. Servers closed;2.6GiB free. No terminal exhaustion claim; next review/classify batch then commentary/redirect.

## Coordinator phase transition — round 2 disassembly

Round1 redirect handoff analysis/redirect/round-001-a/handoff.md inspected: combined D001–D004 ready for all530unknown bytes; C001/C002 boundary corrections ready. Connected and independent avenues packaged, broad city/stage-loop best-effort evidence recorded, no terminal exhaustion claimed. Separate redirect worker completed and servers closed. Separate round02_disassembly now independently reviews batch from pass-001-final; commentary remains preserved for next phase. No new classifications accepted by coordinator. Final audit/document inventory remain pending.

## Pass 002 final — structural pass finished, editorial completion pending

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

Accepted D001–D004: all530 previously unknown bytes (429Code/101Data). Accepted C001/C002:13Code→Data correction. No rejected proposals. Independent reasoning and primary checks: analysis/disassembly/pass-002/review.md, independent-decodes.json and annotations.json; prior redirect evidence preserved. Total Code9597, Data23187, Image32768; every byte structurally classified.

| Bank | Code | Data | Image | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 0 | 0 |
| PRG0 | 5803 | 2389 | 0 | 0 |
| PRG1 | 1386 | 6806 | 0 | 0 |
| PRG2 | 2383 | 5809 | 0 | 0 |
| PRG3 | 25 | 8167 | 0 | 0 |
| CHR0 | 0 | 0 | 8192 | 0 |
| CHR1 | 0 | 0 | 8192 | 0 |
| CHR2 | 0 | 0 | 8192 | 0 |
| CHR3 | 0 | 0 | 8192 | 0 |


Fresh checkpoint maps/pass-002-final.json / generated/pass-002-final, analysis/pass-002-final-verification.json: ok=true, byte_identical=true. Current/frozen map SHA256 58d2d1fc77ed1a0c5ee0b835c8df8a5a734ed773152332e814ff6b5fbb3a94d8. Internal state saved and map.validate passed. Input identity verified unchanged. No ROM/toolkit modification. All servers closed.

Execution coverage unchanged: no new fetch claim or replay; representative natural/earlier artificial gameplay plus redirect's later city/stage-loop recipes remain in scenarios.json and redirect/round-001-a. Static new routines have unresolved external/natural reachability. No additional gameplay question justified rerunning them. No unsupported downstream continuation remains in this supported batch.

Readability/editorial status: prior round1 commentary tree and handoff preserved untouched; map changes require next complete current-source commentary (including title-frame shared counter correction). Final editorial audit pending; this structural100% is not a terminal completion claim. Next coordinator phases commentary then redirect/audit under their own prompts.

Toolkit report analysis/toolkit-issues.md: existing1confirmed documentation/API mismatch NES-001,0suspected bugs,0missing features,0blockers. No new toolkit issues found. Workaround honored. Disk2.6GiB free, scratch36KiB.

## Coordinator phase transition — round 2 commentary

Pass002 independent review accepted D001–D004,C001/C002; handoff and byte-identical final verification inspected. ROM classification printed to user before commentary:100.00% (65552 / 65552;0unknown). Current checkpoint pass-002-final map SHA58d2d1fc77ed1a0c5ee0b835c8df8a5a734ed773152332e814ff6b5fbb3a94d8. Separate disassembly worker completed, servers closed. Separate round02_commentary instructed to preserve entire prior commentary tree before reprepare, carry valid review and finish updated full readability. Structural coverage100% does not yet satisfy final editorial/workflow gates. Redirect then final audit/verification/document inventory remain pending.

## Coordinator phase transition — round 2 redirect

Round2 commentary handoff inspected/accepted:104regions,47Code,315reviewed units, no source frontier; fresh verification and reading identities recorded. Prior entire commentary tree preserved analysis/round-2/commentary. Current map/state58d2d1fc..., structural65552/65552,0unknown. Separate commentary worker finished; separate round02_redirect now inspects updated source and remaining concrete structural questions. Final editorial acceptance/audit and complete archive document inventory pending.

## Coordinator transition — final editorial acceptance

Round2 redirect handoff/findings inspected: exact empty complement, no actionable structural leads, valid semantic/structural distinction, prior decisive generic/branch evidence reviewed. No terminal-incomplete rejection review applies to zero residuals. Separate redirect worker complete; separate final_acceptance worker now handles targeted final editorial review, warranted fixes, documented audit/fresh byte-identical verification+reading and map/store validation. No empty extra cycle. Final coordinator completion audit and refreshed archive-options Markdown inventory still pending; no archive execution.

## Terminal coordinator handoff — workflow complete

ROM classification:100.00% (65552 / 65552 bytes classified;0unknown). Separate final acceptance passed targeted editorial review, fresh byte-identical audit/read refresh and configured map/store validation. Current identities/contiguous evidence-backed coverage/verified leads/phase handoffs independently checked in analysis/coordinator-completion-audit.json. Terminal handoff analysis/terminal-handoff.md; current checkpoint pass-002-final, commentary/audit.json and analysis/final-acceptance/result.json. Archive-options final inventory prepared separately; no archive/publication/cleanup/toolkit modification. Classification/editorial completion is evidence-based and distinct from orchestration status.
