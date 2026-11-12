# Round003 disassembly handoff

Status: exactly one substantive pass complete, classification incomplete; commentary then redirect next. No terminal exhaustion claim. No live server or competing writer.

ROM classification: 99.93% (261974 / 262160 bytes classified; 186 bytes unknown).

Final checkpoint pass-003-final-connected. maps/current.json SHA256118c926b99e13dddf0dfae69066f2084c12a9ca8ce3c502441d741f3e487a14c; analysis/state.json saved. Fresh maps/pass-003-final-connected.json, generated/pass-003-final-connected and analysis/pass-003-final-connected-verification.json validate/rebuild byte-identically. Earlier same-pass checkpoints retained, final authoritative. ROM unchanged90afc676fdc2aa24a759c60b33d9ba951def117f3db3ab73bacda43aefe8513a.

Accepted all R008/R00917429 bytes (8074Code/9355Data), plus361 connected bytes (349Code/12Data):17790 new Static bytes total8423Code/9367Data. Totals36386Code/94516Data/131072Image;1532regions. Prior region metadata/IDs preserved. Exact bank-counts.json and unknown.json in analysis/disassembly/round003; per-bank residual PRG08=4,PRG12=174,PRG13=6,PRG14=2, all others0. Header16Data and all CHR classified unchanged.

Independent review: analysis/disassembly/round003/review.md; checks.json8046 fresh contextual decodes/6725 CFG steps across113callee summaries/107 explicit dependency contracts/43balanced tails; bank-checks.json5bank/16wrapper proofs. Reviewed primary C024 stack removal, nested dispatch, finite916B selectors, MMC1 serial context, bounded field/audio RAM pointers, arithmetic consumers,27vector rows/ratio tail,27PPU destinations/33text records, and82non-code containers. Nonlocal PLA/PLA/RTS exits remain distinct;16253..16263 remains accepted arithmetic Data. Static structure needs no known live caller or natural selector; no new reachability claim.

Independent continuation beyond original ceiling:18C7C..18CAC48-byte direction-word update,18D1A..18D4A48-byte four-record position updater,18C70..18C7C12-byte read lookup; then C0BD exact JMP D49D reuses new bank-return proof to establish18CAC..18D1A110-byte actor-position update, and independent clock/result body18BE1..18C70143bytes has finite RAM/control and B7EC/C05D/C081 contracts. extra-proofs.json/connected-proofs.json preserve exact decodes. Active input position0..7 preconditions explicit; no arbitrary corrupt-position clamp claim.

Residual186bytes/10half-open intervals:
-10530..10534 (4): threeINY/RTS, no independent argument-width/entry contract or discriminating data pattern.
-18559..18595 (60),186BC..18711 (85): coherent presentation fragments; existing rooted ordinary-return method did not establish localized entry/stack transfer. Inspect exact tail8701 and service contracts; independent structural alternatives remain open, no universal balanced-return/live-caller prerequisite.
-18AE0..18AEC (12),18BD5..18BE1 (12): short caller/prologue groups around nonlocal field/actor dispatch; now-supported downstream bodies should narrow stack-transfer research.
-19A56..19A59 (3),1A8E9..1A8EB (2): entry/flags fragments uncertain.
-19E99..19E9B (2),1B5C3..1B5C7 (4): tiny scalar/return-like ambiguity after bounded consumer/neighbor inspection.
-1D4EB..1D4ED (2): extra-PHA property wrapper differs from balanced reader wrappers; require exact result/stack transfer contract.

This pass reasonably followed connected structure without exhaustive residual closure. Redirect should inspect those localized contracts and assess independent routine fragments instead of inheriting whole-gap rejection; no terminal rejection review has been claimed. Reuse prior broad side-switch/AllStar/Special/girls/credits evidence and saved routes; no unchanged gameplay replay occurred.

Current commentary/source/game.asm unchanged SHA256862952e794ae020694d09b1e815afe1275602e6dd0035682e63193efb061fc52; all prose and valid review credit preserved. Full current-map source expansion/readability review is assigned to separate commentary worker. Never rerun historical prose transformations blindly.

No toolkit issues found (analysis/toolkit-issues.md). Disk6.5GiB free; scratch292KiB. All ROM/internal evidence/source/assets local. No toolkit edits/input changes/archive/publication/cleanup.
