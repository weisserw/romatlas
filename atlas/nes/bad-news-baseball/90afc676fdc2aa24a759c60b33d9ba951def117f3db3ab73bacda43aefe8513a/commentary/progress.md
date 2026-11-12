# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round 001 current-source review milestone

Inspected the entire 10,683-line frozen source in physical order. Added 174 subsystem/routine explanations spanning all 7,938 classified instructions, 1,058 routine/branch symbols, shared hardware/RAM equates and local notes for every raw interval. Full 693-entry scaffold now records reviewed scope, physical ranges, inherited effective bank views and source locations. Specific explanations cover screen decoding, attribute quadrants, presentation script commands and $11-byte objects, audio records/commands, menus and two-byte roster records, password packing/checksum, match transitions, fixed-point ball movement, controller debounce, NMI/PPU queue, MMC1 wrappers and record readers. Local names preserve unknown address identity where scratch fields are reused.

First coherent annotated-source verification passed byte-identically (source hash 1066c148e61d31627263f0fe9fafeb5bc5731891c201ec48dcdc789386e2cd6f). Final review is still checking the new names and prose for concrete inconsistencies; fresh handoff verification and reading-copy refresh remain. Phase1 classification is unchanged and incomplete. Editorial/archive acceptance remains pending.

## Round 001 commentary handoff gate

Completed full current-source readability. No pending classified-source ranges remain. All 693 scaffold entries are reviewed; 502 Code records contain compact local units and preserve their phase1 bank views. Added 62 named CPU lookup bases, explicit record-buffer layout and concrete helper input/output/calling-convention notes during the consistency pass. Hardware and understood RAM operands use symbols while explicit a:/z: encoding widths are preserved. Address-based work names remain where storage is shared or purpose is not established. No structural map/state changes were made.

Fresh final verification passed: commentary/verification.json reports byte_identical=true and source SHA256 ec5bcb5c5d31a2b7d07ea1b255e2f055f5d95dffb4414be908cb5e3a7814a49d. commentary/reading-after.asm was refreshed after that final source. The frozen before source, baseline map/state and all earlier analysis checkpoints remain intact. Final editorial_audit is deliberately pending; no archive acceptance or whole-ROM completion is claimed. Unresolved meanings and practical redirect checks are in uncertainties.md. No toolkit issues found; analysis/toolkit-issues.md remains unchanged.

Historical construction/refinement scripts are retained in analysis/scratch/round001-commentary-*.py. They document this first baseline transformation; do not rerun them against an already annotated source, since they expect their corresponding input revision.


## Round 002 active commentary

Input checkpoint pass-002-final, map SHA256 24ef23895954dee3a4262e48055c86620455cd656c94853167e745e879f4c5d1. Current classified scope 244184/262160 bytes (93.14%): Code27963, Data85149, Image131072. Unknown17976. Previous commentary preserved in analysis/round-002/commentary, including frozen comparison artifacts and reviewed source ec5bcb5c5d31a2b7d07ea1b255e2f055f5d95dffb4414be908cb5e3a7814a49d.

Active, incomplete: reconcile all 812 added regions (387 Code,425 Data;83074 bytes), carry unchanged physical-region prose/review credit, refresh affected dependencies and remaining unknown notes. Full current-source readability gate, fresh verification and reading-copy refresh remain pending. No handoff readiness is claimed. Corrected 0x16253..0x16263 is four lower/upper LE16 pairs, never BMI/BRK.


Round002 readability reconciliation milestone: all 1480 current scaffold entries (889 Code,443 Data/Image,148 Unknown) and1213 meaningful units now have reviews;520 unchanged classified regions retain prior prose/credit. All387 fresh code fragments and425 fresh data regions inspected;154 semantic scope notes plus26 helper notes added. Fresh byte-identical verification passed after carry and after the first semantic refinement. A final named-return boundary refinement exposed a branch-target mismatch at0x1a3b9; its four branches were corrected to the renderer return, preserving the separate indirect dispatcher entry. Final verification, source-reference reconciliation and reading refresh follow this concrete correction. No ranges remain deliberately deferred for readability.


## Round 002 current-source readability complete

All244184 current classified bytes reviewed (Code27963,Data85149,Image131072), plus honest boundary notes for148 unknown intervals/17976 bytes. All1480 current scaffold entries and1213 meaningful units reviewed; no current readability backlog. Retained520 unchanged classified regions and their useful prose/credit, reconciled812 additions (387Code,425Data). Source has23348 lines and2136 unique code/data labels;154 new semantic scope notes plus26 helper notes. Examples include incremental camera tile/attribute strips; signed vector lookup; command-specific presentation argument widths; audio tokens/DMC/envelope limitations; nine actor versus four field records; computer nearest-actor/bounds predicates; pitcher side-switch persistence; ball collision/camera motion and field/result markers. Corrected table at0x16253..0x16263 is four explicit lower/upper word pairs.

Fresh final commentary/verification.json: ok=true,byte_identical=true. Source SHA256 862952e794ae020694d09b1e815afe1275602e6dd0035682e63193efb061fc52. Reading-after.asm refreshed after that verification. Scoped evidence: analysis/round-002/commentary-review-checks.json, editorial-scopes.json, helper-notes.json and new-code-review.txt. All source locations use game.asm relative to source/. Source mapping.json now matches the expanded map.

Previous commentary remains intact under analysis/round-002/commentary. Existing before/ and reading-before.asm remain historical round001 comparison; current frozen comparison is before-round002/ and reading-before-round002.asm, declared by manifest.json. Current baseline.json/phase1-state.json match pass-002-final; prior copies are additionally retained with .round001 suffixes. These are local internal artifacts, never portable sharing deliverables.

Updated uncertainties.md retires resolved old script/table and side-switch questions while retaining precise envelope, selector, record-property, bank-context and residual unknown limitations. No unreviewed readability ranges remain. Whole-ROM classification and final editorial/archive acceptance remain incomplete; editorial_audit stays pending. No final audit, redirect, archive, publication, toolkit edit, input change or live server was performed by this worker.

Historical round002 transformation scripts in analysis/scratch/round002-commentary-*.py are revision-specific, non-idempotent investigation artifacts; do not rerun them over current source. Final scope/source truth is commentary/source/game.asm plus commentary-review-checks.json; helper boundary corrections are reflected there.


## Round 003 active commentary

Checkpoint pass-003-final-connected, map SHA256118c926b99e13dddf0dfae69066f2084c12a9ca8ce3c502441d741f3e487a14c. Current classification261974/262160 (99.93%):36386 Code,94516 Data,131072 Image;186 Unknown in10 intervals. Entire preceding commentary preserved under analysis/round-003/commentary (source862952e794ae020694d09b1e815afe1275602e6dd0035682e63193efb061fc52). New frozen comparison before-round003; prior comparisons retained.

Active reconciliation of200 new regions (117 Code,83 Data), carrying all unchanged physical prose and credit. Full current-source readability review and fresh verification/reading refresh pending; no successful handoff yet. Independent structural evidence analysis/disassembly/round003/review.md and associated contracts supplies bank/stack/table context. No toolkit/input changes or live server.


Round003 refinement milestone: all117 new code fragments and83 data regions reconciled with147 functional scope notes. Preserved prior physical prose; repaired13 stale gap assertions and six carried-target branch names. Added220 bank-specific CPU lookup aliases, indirect record offsets and consistent actor height/vertical-velocity fields; rendered65 established scalar/word tables directly in assembly. Ten residual intervals retain question-specific local notes. Intermediate verification passed byte-identically after the expansion and both consistency batches. Concrete final checks remaining: source-reference/scaffold reconciliation, exact unknown complement, uniqueness and source partition, final fresh verify and reading refresh. Full editorial/archive audit remains pending.


## Round 003 current-source readability complete

All261974 classified bytes (36386 Code,94516 Data,131072 Image) and ten exact residual boundary notes are reconciled. All1542 scaffold entries and1439 meaningful units reviewed,1006 Code regions. Preserved1332 prior classified physical regions and useful prose/credit; added117 Code/83 Data regions. No pending current-source readability entries or deliberately deferred ranges remain.147 functional scope notes cover arithmetic/quadrants/division, roster setup/match UI, signed actor/field integration, dispatch/nonlocal stack transfers, banked readers, text and numeric data.220 CPU lookup aliases retain physical identity;65 established tables are directly rendered. Source has29200 lines and2695 unique labels.

Concrete consistency fixes include13 stale gap assertions, six carried-target branch names, actor height/vertical velocity fields, separate relative field offsets, Scratch04/09 versus same-valued count constants, team-name versus player-reader distinction, delayed-state/marker-initializer entry boundaries, and the carried dispatch unit off-by-one. Exact residual186 bytes/10 intervals stay Unknown.

Fresh final verification: ok=true,byte_identical=true, source SHA2562a68c97f32366331429ff8ab04ba251a52d8627829d8ed61fcd5f67052a95430. Reading-after.asm refreshed afterward. Scope evidence analysis/round-003/commentary-review-checks.json checks the current scaffold, exact source partition/instruction lengths, all review/source references, label uniqueness, unchanged prior region metadata and map identity. Current source mapping.json/baseline/state match pass-003-final-connected. Entire prior commentary remains a local snapshot at analysis/round-003/commentary; before/, before-round002 and new before-round003 comparisons are preserved.

Current uncertainties.md replaces stale barrier counts and resolved questions with localized structural/semantic limits. Coordinator owns exact-residual redirect; no terminal exhaustion or whole-ROM completion claim. Final editorial_audit remains pending, with no audit/archive/publish/cleanup/toolkit edit/input change or live server. No scripts/archive.py invocation occurred. All ROM/internal evidence/source/assets remain local. Historical round003 scripts are revision-specific and must not be replayed over annotated source.


## Round 004 active commentary

Checkpoint pass-004-final; map SHA25620b933e17d8f3032291cb4ca229fd6da613cee1a1975e18d4d2fff4df73f6dc6. Prior full commentary preserved under analysis/round-004/commentary. Reconciling180 new bytes (171Code,9Data) by physical range; retaining valid prior prose/review credit. Exact residual six bytes in10530..10534 and19E99..19E9B remain unknown. All current readability, dependency checks and fresh verification/reading refresh pending; final editorial/archive acceptance remains separate.


## Round 004 current-source readability complete

Reconciled all180 new bytes in eight regions (five Code/171bytes, three Data/9bytes), preserving1532 prior classified physical regions, useful prose and valid review credit. All1542 scaffold entries and1447 meaningful units reviewed; no current readability backlog. Full current inventory checked; all17861 instruction lengths, physical data/incbin identities, source locations, unique labels and exact scaffold partition validated. Repaired two stale source boundary assertions and five affected inventory summaries.

Source explains8701 local returning/sprite tails separately from8727 handshake exit; CDA4 nested special exits retain A378 and savedY under actor-state0..11; D4DB sequential PLA/RTS has an exact nonstandard stack-directed transfer, unlike balanced readers. Added descriptive operand aliases and rendered three consumer-backed suffix lookups; sharedB5B2 RTS remains Code/scalar. Natural invocation, complete selectors and original grouping remain uncertain. Six unknown bytes retain honest local notes:10530..10534 and19E99..19E9B.

Fresh final verify ok=true,byte_identical=true; source SHA256976b55a104e228767cb35b3633c9b18c98951bcda749474b1fc5e9790dc65123. reading-after.asm refreshed. Current baseline/state/source mapping match pass-004-final, map SHA25620b933e17d8f3032291cb4ca229fd6da613cee1a1975e18d4d2fff4df73f6dc6; comparison before-round004 declared in manifest. Previous full commentary remains analysis/round-004/commentary. Scoped checks analysis/round-004/commentary-review-checks.json. Classification INCOMPLETE262154/262160 despite rounded100.00%. Final editorial_audit remains pending; no final audit/archive/publication/cleanup/toolkit/input edits or emulator run. Disk6.4GiB free, scratch bounded. No live server or competing writer; coordinator owns redirect and terminal assessment.


## Round 005 current-source readability complete

Checkpoint pass-005-final-reviewed, map SHA256 a839d376085c7bc579e260056f5bf4c81fceb3316622389bfcfa63d9f3cb4b02. Preserved the entire previous commentary under analysis/round-005/commentary before reconciliation. Current baseline/state/source mapping match that checkpoint; before-round005 and reading-before-round005.asm provide the new frozen comparison. Earlier comparisons and all useful prose, metadata and review credit survive.

All262156 classified bytes (36557 Code,94527 Data,131072 Image),1542 scaffold entries and1448 meaningful units reviewed; no current-source readability backlog or deferred ranges. Carried1540 unchanged classified regions and reconciled the two new Data cells19E99..19E9B. Full physical inventory, review/source references, unique labels, exact complement, all prior instruction/data directives and physical incbin identities checked.17861 instructions remain unchanged except one equivalent symbolic operand;2706 labels and29342 source lines.

TrajectoryContactSelectorRemapSuffix maps conditional contact selectors3/4 to4/3 before direction-offset indexing under negative Work04E6. TrajectorySelectorRemapBase names the indexed base; preceding executable LDA/RTS bytes remain Code. Source explains the conditional domain, effective slots[12,13,14,15] and original grouping uncertainty. Updated the remaining audio note with dispatcher, helper analogy, period-origin and different-bank alias constraints. Exact four-byte residual10530..10534 remains Unknown; natural invocation and complete selectors remain unclaimed. No additional semantic research prerequisite or readability gap remains.

Fresh final commentary verification ok=true,byte_identical=true; source SHA25649b40995602d089aab96a790c07f628db23414a50301fc7a68da93b1780b9414. reading-after.asm refreshed after verification. Scoped evidence analysis/round-005/commentary-review-checks.json and primary analysis/disassembly/round005/review.md plus analysis/redirect/round004-a/review.md. Classification remains INCOMPLETE262156/262160 despite rounded100.00%; final editorial_audit stays pending. No final audit, redirect, archive execution, publication, cleanup/deletion, toolkit/input edits or emulator run. Coordinator retains archive-options.json ownership and terminal assessment. This worker stops writing at handoff.


## Final editorial acceptance

Targeted final review reuses the valid completed full-source gate, with no blanket reread. Corrected a generic period-prefix note and stale neighboring-classification assertion in the audio/DMC area; added SemitonePeriodStoragePrefix with mechanical consumer/extent explanation. All assembly instructions/directives and map/input bytes remain unchanged. Preserved pre-editorial source/review/uncertainties/verification under analysis/round-005/pre-editorial. Refreshed shifted locations and matching inventory summaries. No known naming/comment backlog remains.

commentary/editorial-audit.md records scope, findings, limits and accepted uncertainty. review.json editorial_audit is reviewed; all1542 scaffold entries and1449 meaningful units reviewed. Fresh commentary.py audit passed ok=true,byte_identical=true,editorial_audit=true using the configured toolkit; it also refreshed verification.json and reading-after.asm. Source SHA256ea8947206254f1a3596a253aa370d67b18e924249794dc33aeff0d1c6a1de01c; map SHA256a839d376085c7bc579e260056f5bf4c81fceb3316622389bfcfa63d9f3cb4b02.2707 unique labels,29349 source lines. Exact classification remains INCOMPLETE262156/262160, four Unknown audio bytes10530..10534. Source editorial acceptance does not establish whole-ROM completion; coordinator owns terminal workflow and any archive decision. No toolkit/input/map edit, archive execution, publication, cleanup/deletion or emulator run. Compact final handoff analysis/handoffs/final-editorial.md.
