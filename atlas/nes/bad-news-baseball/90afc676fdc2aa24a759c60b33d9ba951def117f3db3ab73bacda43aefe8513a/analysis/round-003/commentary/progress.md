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
