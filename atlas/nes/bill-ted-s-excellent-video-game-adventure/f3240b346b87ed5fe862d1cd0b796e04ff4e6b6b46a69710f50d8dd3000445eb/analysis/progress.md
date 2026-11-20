# Terminal status — coordinator workflow complete

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

All three sequential phase rounds and targeted final editorial acceptance are finished. Current checkpoint round3-structural-verified; current-source audit and fresh byte-identical verification pass. Independent coordinator checks pass for exact coverage, all reviewed source/unit links, reading-copy equality, final identities, store and portable-map validation. See [terminal handoff](final-handoff.md), [coordinator verification](final-coordinator-verification.json), and [archive document audit](archive-document-audit.json). Archive options select final evidence/handoffs and omit only documented duplicates/process copies; no archive, cleanup, publication, toolkit edits or ROM changes occurred. Semantic/gameplay limitations remain explicit in the handoff. Historical entries below are preserved and may describe superseded checkpoints.

## Preserved workflow history

# Round 1 — commentary complete; redirect next

Full current-source readability complete. Coordinator source-location correction applied: all924 region/unit links now resolve relative to commentary/source/ as game.asm:N; local generator fixed and locations checked. [Commentary handoff](round1-commentary-handoff.md) and [inventory](round1-commentary/readability-report.json) cover all790 scaffold regions,631 Code fragments,134 functional units, all classified data/image and all unknown-boundary notes. Fresh [verification](round1-commentary-verification.json) is byte-identical; commentary/reading-after.asm refreshed. Frozen map/state/ROM unchanged. No final editorial audit claimed; structural classification remains80.25% (210380/262160),51780 unknown.

Meaningful routine/block/branch, hardware/RAM/field and constant symbols now explain current mechanics, including bank-specific CPU aliases and course RAM overlays. Source uncertainties and corrected eight-cell $B6DE consumer domain are in commentary/uncertainties.md; old disassembly evidence preserved. No pending readability ranges. Next: redirect from existing concrete structural/gameplay avenues. No toolkit edits, publication, archive or deletion.

## Preserved commentary running checkpoint

# Round 1 — commentary running

Prepared commentary/ with frozen verified baseline and independent source. First two source block annotation batches through physical $03642 rebuild byte-identically; exact remaining work in commentary/progress.md. Full current-source readability is in progress; disassembly checkpoint unchanged. Do not advance to redirect until source review completes.

# Disassembly progress

Round 1 disassembly **finished incomplete**. One substantive pass completed; next phase is full current-source commentary, then redirect. No toolkit changes, archive, deletion or publication. Input hash unchanged.

ROM classification: 80.25% (210380 / 262160 bytes classified; 51780 bytes unknown).

Structural totals: 14938 Code, 64370 Data, 131072 Image. 652 regions, all with evidence. Header16 bytes classified. Exact unknown complement: [unknown-ranges.json](round1/unknown-ranges.json). Execution is separate: 5607 physical instruction starts / 11673 distinct instruction bytes, 6858 contextual roots, zero aggregate overflow; retained evidence after timer/debugger edits is artificial. Zero retained-fetch/non-code conflicts after final correction. Zero-detail tracing intentionally dropped detail events; see [trace summary](round1/final-trace-summary.json).

Checkpoint: [round1-verified](round1-verified-verification.json), map maps/round1-verified.json, generated/round1-verified/game.asm. Validation passed and whole image rebuilt byte-identically. Map SHA256 cffd7961a9cbed9b7d3fbd260b9f671aba003a84bd1e1234b9edf08d6d963f55. Current internal state analysis/state.json and portable map maps/current.json match the verified checkpoint. Baseline and round1-initial remain sound historical milestones. **round1-static and round1-final are superseded**: one generic bank annotation covered a renderer tail; corrected Data bound $179DF, code from observed $179E0 onward, exact remaining gaps retained Unknown. [Correction review](round1/data-inspection.md), [conflict audit](round1/corrected-conflict-review.json).

Gameplay: copyright/narrative, outdoor booth and Rufus dialogues, phonebook, keypad, accepted level6 password5555864, late-stage outdoor movement/jump, Circuits of Time launch/down movement, and artificial terminal congratulations/Game Over screen. Ending entry $8150 verified through NMI handler patch; $8149 trial rejected because $CC38 resets stage. Exact routes, snapshots and status in [scenarios](scenarios.json), research in [gameplay](gameplay.md). No natural full-game or full hardware compatibility claim.

Lead results: initial leads empty; boot timer, phone controls/password, renderer mixed-bank correction and ending entry verified. Constant-true BNE fallthrough into $B6B6 rejected as code; its eight-byte data suffix proved by indexed consumer. Compressed text stream, remaining fixed-bank script handlers and bank7 audio/data boundaries remain concrete redirect avenues; no terminal exhaustion claim. Compact handoff: [round1-disassembly-handoff.md](round1-disassembly-handoff.md).

Toolkit: **no toolkit issues found**; [persistent report](toolkit-issues.md). Initial non-PTY server exit143 lost transient aggregates; durable snapshot resumed in same pass, then PTY server and per-route exports used. Scratch76KiB, selected round1 evidence120MiB, disk4.2GiB free at handoff.

| Bank | Code | Data/Image | Classified | Unknown |
|---|---:|---:|---:|---:|
| PRG00 | 2681 | 0 | 2681 | 5511 |
| PRG01 | 6162 | 98 | 6260 | 1932 |
| PRG02 | 2869 | 0 | 2869 | 5323 |
| PRG03 | 718 | 55 | 773 | 7419 |
| PRG04 | 0 | 8192 | 8192 | 0 |
| PRG05 | 20 | 8172 | 8192 | 0 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 20 | 8172 | 8192 | 0 |
| PRG08 | 0 | 8192 | 8192 | 0 |
| PRG09 | 20 | 8172 | 8192 | 0 |
| PRG10 | 0 | 8192 | 8192 | 0 |
| PRG11 | 1119 | 6677 | 7796 | 396 |
| PRG12 | 0 | 0 | 0 | 8192 |
| PRG13 | 20 | 12 | 32 | 8160 |
| PRG14 | 359 | 45 | 404 | 7788 |
| PRG15 | 950 | 183 | 1133 | 7059 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

## Preserved running history

# Disassembly progress

Round 1 disassembly running. Baseline has verified byte-identically; classification is in progress.

## Round 1 — disassembly running

Baseline established and byte-identical (analysis/baseline-verification.json). Configured release binary in use. Static vector inspection and bounded runtime familiarization underway; no inherited leads.

Initial checkpoint: [round1-initial verification](round1-initial-verification.json), byte identical. 134231 bytes classified, including 3143 code, 16 header, 131072 CHR; 127929 unknown. First server exit143 interrupted runtime aggregates; restored durable copyright snapshot; PTY replacement and per-route exports now used. Static and broad gameplay investigation continues in the same pass.


## Redirect r1-20261120-a running

Input round1-verified, map SHA cffd7961a9cbed9b7d3fbd260b9f671aba003a84bd1e1234b9edf08d6d963f55. Current-source commentary gate complete. Research only; map/source/state/ROM unchanged. First target: bank6/7 nibble streams, independent payload bounds and mixed bank7 endpoint. Next: inspect pointer/header domains and packed glyph consumer, then script dispatcher, renderer gaps and course/world transition. Corrected layout lookup domain Y=8..1, eight suffix cells; old nine-cell claim is not inherited.

## Round1 redirect r1-20261120-a ready

Investigated text, renderer, script/interaction/course, audio and source numeric-helper continuations. Combined45469 unique proposed bytes (6940 Code/38529 Data),152 disjoint intervals; zero accepted bytes in redirect, current51780 unknown unchanged. Exact static Code endpoints pass; no current-coverage/type conflicts. Course completion flag shortcut reached rendered outdoor arrival and20-frame movement/jump, yielding1401 unknown accepted-fetch bytes from artificial state. Input/map/state/source/symbol hashes match unchanged checkpoint. Handoff: [redirect combined batch](redirect/r1-20261120-a/handoff.md); no terminal exhaustion claim. Independent disassembly next, then current-source commentary and redirect. No toolkit issues found; no toolkit/map/source changes.


## Round2 disassembly — verified, incomplete

ROM classification: 97.94% (256754 / 262160 bytes classified; 5406 bytes unknown).

Accepted46374 new bytes:7845 Code and38529 Data, including1401 artificial-state accepted-fetch bytes; static/reachability claims remain separate. Independently verified redirect152 intervals/45469 bytes plus412 direct continuation instructions/905 bytes in22 intervals. Role review: [batch](disassembly/r2-20261120-a/review.md), [connected paths](disassembly/r2-20261120-a/continuation-review.md). All existing annotations retained. Prior commentary/source untouched.

Final checkpoint [round2-connected-verified](round2-connected-verified-verification.json), valid internal/portable map, generated fresh source byte-identical. Map/state SHA 0b9376ea99ce94d527af13752afb501080df20e5d4300ab1257da5ccb9509718. Earlier round2-verified checkpoint preserved. Header16 separately Data. Annotation evidence complete for classified regions; current generated source readability awaits next full commentary pass, prior commentary gate applies to earlier source only. No final editorial audit.

Execution evidence reused saved663 contextual roots/1401 unknown accepted-fetch bytes, all fresh immutable decodes match, no Data contradiction. No new emulator route needed; prior outdoor/course/arrival movement and artificial ending already sampled. Further historical worlds/horse/canoe require initialized actor/form/position contracts; no unchanged long startup replay. Aggregate coverage remains separate from classification and session-local.

All six ready lead IDs verified for proposed scope, with history preserved. Additional connected lower-bank movement/action/coordinate paths followed through known returns/consumers. Remaining5406 bytes across67 intervals in [exact complement](disassembly/r2-20261120-a/unknown-complement.json). Concrete next questions: selected-form callback$06D8 origin$C7D0/Y; scene/actor headers before$C680; compact neighbor/direction arrays referenced by newly classified$93FD..$9634 actor logic; phone/scene remaining mixed gaps and supported late-world initialization. These are retained for redirect, not blanket rejections or terminal exhaustion.

No toolkit issues found; [report](toolkit-issues.md). No ROM/toolkit changes, deletion, archive or publication.

| Bank | Code | Data/Image | Classified | Unknown |
|---|---:|---:|---:|---:|
| PRG00 | 6411 | 96 | 6507 | 1685 |
| PRG01 | 7701 | 237 | 7938 | 254 |
| PRG02 | 3905 | 850 | 4755 | 3437 |
| PRG03 | 1006 | 7156 | 8162 | 30 |
| PRG04 | 0 | 8192 | 8192 | 0 |
| PRG05 | 20 | 8172 | 8192 | 0 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 20 | 8172 | 8192 | 0 |
| PRG08 | 0 | 8192 | 8192 | 0 |
| PRG09 | 20 | 8172 | 8192 | 0 |
| PRG10 | 0 | 8192 | 8192 | 0 |
| PRG11 | 1465 | 6727 | 8192 | 0 |
| PRG12 | 0 | 8192 | 8192 | 0 |
| PRG13 | 20 | 8172 | 8192 | 0 |
| PRG14 | 1228 | 6964 | 8192 | 0 |
| PRG15 | 987 | 7205 | 8192 | 0 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

## Round2 redirect r2-20261120-a — running
Current input round2-connected-verified, map/state SHA0b9376ea99ce94d527af13752afb501080df20e5d4300ab1257da5ccb9509718;256754 classified/5406 unknown. Current-source readability gate complete. Research begins with direction/probe table domains and numeric scene/phone helper contracts, then selected forms and mixed scene headers. Durable frontier: analysis/redirect/r2-20261120-a/frontier.md. No map/source/ROM/toolkit changes.

## Round2 redirect r2-20261120-a — combined ready batch
Input/current checkpoint round2-connected-verified; map/state SHA 0b9376ea99ce94d527af13752afb501080df20e5d4300ab1257da5ccb9509718 unchanged. Classification256754/262160, Unknown5406 in67intervals; accepted0 here. Proposed5406=3230Code+2176Data in87disjointintervals;1479exactinstructions, no known overlap or type conflict, exact complement coverage. If all independently accepted, hypothetical Unknown0; not current acceptance.

[Handoff](redirect/r2-20261120-a/handoff.md), [claims](redirect/r2-20261120-a/proposal-claims.json), [validation](redirect/r2-20261120-a/package-validation.json), [current residual ledger](redirect/r2-20261120-a/unknown-ledger.json). Finite direction/form/actor/phone graphs followed, seven literal screens and seven six-byte scene headers/command streams parsed to independent endpoints; scene helperC23C and phoneD358/D40E resolved. Data fallback localized with positive patterns, boundaries and consumer checks. Explicit D62B untaken-fallthrough correction retained. No arbitrary pointer roots.

New artificial tests: initialized form5/7 transitions with input; initialized distinct scene3 with input/jump. First NMI-slot shortcut blackened/disarmed future NMI; source-based second-slot correction succeeded, with bounded initial timeout22frames then16completed frames.98contextual candidate starts/76uniquephysical,zero proposedData conflicts,zeroaggregate drops. [Gameplay findings](redirect/r2-20261120-a/gameplay-findings.md). No unchanged startup/ending replay or natural reachability assertion.

Stable leads R1-FIXED-HELPERS and R1-SCRIPT-DISPATCH ready with new scopes/evidence; R1-WORLD-PROGRESSION history appended. All prior evidence/checkpoints preserved. Ready for independent disassembly; no terminal exhaustion or final editorial audit. ROM/source/symbol hashes unchanged. No toolkit issues found; no toolkit/map/source changes, archive, deletion or publication. Disk4.0GiB free; scratch92KiB.


## Round3 disassembly — structural classification verified; editorial gates pending

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Accepted5406 new bytes=3230Code+2176Data in87intervals after independent role/ABI/bank/boundary review,1479fresh configured-binary instructions. Internal totals26013Code+105075Data+131072Image, including16headerData. All old regions unchanged by ID/content; every classified region has evidence. Exact unknown complement empty. [Independent review](disassembly/r3-20261120-a/review.md), [decodes](disassembly/r3-20261120-a/independent-decodes.json), [schema/domain checks](disassembly/r3-20261120-a/independent-data-checks.json), [counts](disassembly/r3-20261120-a/summary.json).

GenericC220..C22B andE253..E254 accepted only as localized auxiliary Data after neighboring patterns/consumers/code-edge checks, no invented complete schema or valid-address claim. Standalone8087 restore andA11E symmetricPPU subtraction have complete mechanical entry contracts. A305 writer contract explicitly narrowed: serial latch ready,bit7clear,controlbits2..3=10 preserves lowerPRG0/1 through fifthMMC1write/RTS; arbitraryA mapping continuity not asserted. ThreeC8BB words only;A50A4LSRCode;D62B literal Data protected by provably takenD629branch. No unsupported callback roots or data-crossing continuation accepted.

Reused saved artificial forms5/7 and initialized scene3 evidence:98contextual/76uniquephysical unknown instruction starts independently decoded under actual effective contexts;zero proposedData conflicts. All newly accepted provenance remainsStatic; actual/artificial fetch evidence and natural reachability remain separately scoped. Prior course/late-stage/ending visits and failed/ corrected NMI-slot recipes retained; no startup/ending replay. Full vehicle mechanics/every historical world remain gameplay/semantic limits, not unresolved structural intervals.

Fresh [round3-structural-verified](round3-structural-verified-verification.json) validates internal/portable maps and generated source byte-identically. Map/state SHAa06ecdcf40250a4e2edb00b18d9da89712b2d38f9935ab1ba46a59dda7ebff4f. ROM unchanged. Prior commentary/source/history preserved; fresh current-source readability, redirect and final editorial/verification gates remain, so this is NOT full-ROM final acceptance or goal completion. No unresolved connected classification continuation remains in this batch. No toolkit issues found; [persistent report](toolkit-issues.md). No toolkit edits, archive, deletion or publication. Disk about4GiB free,scratch92KiB.

| Bank | Code | Data/Image | Classified | Unknown |
|---|---:|---:|---:|---:|
| PRG00 | 7881 | 311 | 8192 | 0 |
| PRG01 | 7919 | 273 | 8192 | 0 |
| PRG02 | 5447 | 2745 | 8192 | 0 |
| PRG03 | 1006 | 7186 | 8192 | 0 |
| PRG04 | 0 | 8192 | 8192 | 0 |
| PRG05 | 20 | 8172 | 8192 | 0 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 20 | 8172 | 8192 | 0 |
| PRG08 | 0 | 8192 | 8192 | 0 |
| PRG09 | 20 | 8172 | 8192 | 0 |
| PRG10 | 0 | 8192 | 8192 | 0 |
| PRG11 | 1465 | 6727 | 8192 | 0 |
| PRG12 | 0 | 8192 | 8192 | 0 |
| PRG13 | 20 | 8172 | 8192 | 0 |
| PRG14 | 1228 | 6964 | 8192 | 0 |
| PRG15 | 987 | 7205 | 8192 | 0 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

## Round3 redirect r3-20261120-a

Completed current-source targeted consistency and independent writer review from round3-structural-verified. [Handoff](redirect/r3-20261120-a/handoff.md), [findings](redirect/r3-20261120-a/findings.md), [exact complement/identities](redirect/r3-20261120-a/unknown-complement.json), [editorial issues](redirect/r3-20261120-a/editorial-issues.json).913regions/262160classified;26013Code+105075Data+131072Image;0Unknown, no overlap. Source/map/state/ROM unchanged. No structural proposal/ready lead;0proposed/0accepted bytes. Existing verified leads retain history with new source-consistency observations.

Found stale saved-form structural-frontier prose at game.asm:1950 despite accepted callback closure;0372 mislabeled second mailbox at4898/18053. Known explicit0372 producers use01/02 -> scanner16/17; full writer domain remains unproved. Research leaves exact repairs for final editorial worker, with no empty disassembly cycle. No structural contradiction; final acceptance still pending independent editorial audit and fresh fidelity checks after edits. Honest grammar/resource/natural-use/full-vehicle limits remain. Reused prior routes/reviews; no unchanged replay, speculative callback roots, archive/deletion/publication. No toolkit issues found; [report](toolkit-issues.md). Disk~3.9GiB free; no new bulk scratch.

## Final editorial acceptance final-20261120-a

Separate sequential final worker accepted current-source editorial quality after targeted changed-evidence/dependency checks, reusing sound full readability review. [Handoff](editorial/final-20261120-a/handoff.md), [editorial findings](editorial/final-20261120-a/editorial-audit.md), [machine completion](editorial/final-20261120-a/completion.json), [audit](editorial/final-20261120-a/audit.json), [fresh verification](editorial/final-20261120-a/verification.json). Three redirect defects corrected; explicit audio producer constraint retained with limits; five stale unknown-neighbor notes corrected. Nine comment lines only; symbols/instructions/bounds unchanged.913regions/368uniqueunits/892records/1805stableanchors;262160classified/0Unknown (100.00%). No pending readability or structural contradiction. Supported final audit and fresh byte-identical rebuild passed; reading refreshed and independently matched final source. Source4682856804fc946661b8169aa1e5b7d2b44f7c7b543e699c809b7ba5feaa89b7; map/statea06ecdcf40250a4e2edb00b18d9da89712b2d38f9935ab1ba46a59dda7ebff4f unchanged. Honest grammar/schema/producer/natural-use/gameplay limits retained. No toolkit/ROM/map/state edits, deletion/publication/archive execution or goal completion. Exact next frontier: coordinator independent final acceptance and archive-options. Disk~3.9GiBfree,scratch92KiB.
