# Disassembly progress

## Phase-2 complete — batches 011–012, 2026-10-13

Independent whole-game semantic/editorial review complete: 161 reviewed entries,
1077 reviewed units, 32768 reviewed bytes, 0 pending. Completed effect/noise,
pulse2/wave music and all audio data. Quirks and investigated limits are recorded
close to source and in ../commentary/uncertainties.md. No original game opcode
or behavior was fixed. No new runtime input sequence or fetch coverage claimed.

Human editorial report: ../commentary/editorial-audit.md. Required fresh
`commentary.py audit /root/alleyway` exit 0: ok=true, byte_identical=true,
reviewed_regions=161, reviewed_units=1077, editorial_audit=true. It rebuilt
without RGBFIX changes and refreshed ../commentary/reading-after.asm. Results:
../commentary/audit.json and ../commentary/verification.json. All unit starts/
source labels/code boundaries agree, every code byte has unit coverage, and no
generated label, numeric transfer or uninventoried source label remains.
Auxiliary JR Z width/count was corrected to 4845 starts; prior phase-2 4883
reports are superseded, with incoming-grid-reference conclusion unchanged.

All scaffold identity/evidence fields and frozen/current map/state hashes match
preparation. Frozen reading/source match and original ROM remains SHA-256
ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2.
Phase-1 structural counts remain:

| Bank | Code | Data | Image | Padding | Unknown | Total |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 4550 | 11563 | 0 | 240 | 31 | 16384 |
| 1 | 4953 | 2887 | 6144 | 2400 | 0 | 16384 |

Classification is separate from execution. Original traces/replay sequences are
preserved; fixed bank0/bank1 CPU contexts remain resolved. Phase 1 is still
incomplete on its 31-byte Unknown; reviewed ambiguity completes phase 2 only.
Toolkit unchanged; T001 is the sole existing nonblocking missing feature, no new
issue (toolkit-issues.md). Source/assets/ROM/traces remain local. Details in
../commentary/progress.md.

## Phase-2 commentary batches 008–010 — 2026-10-13

Reviewed all non-audio source/data, bonus timer/results, UI, animation/multisprite
records, three graphics blocks, request veneers and fixed trampolines/padding.
Phase-2 inventory:120 reviewed /41 pending entries,572 reviewed units;28033
reviewed /4735 pending bytes. Remaining entries are the audio engine/tables at
physical/CPU6800-7a7f, fixed bank1. Bank0's31-byte unknown remains Unknown in
phase-1 state and its ambiguity remains explicitly reviewed, not relabeled.

Corrected speed-display interpretation after reading RET4a29 and detached4a2a
writer. Investigated unreferenced40-row empty and12-row singleton-brick grids;
all33 stage descriptors and4883 classified-code instruction starts yield no
incoming n16 operands into them. Graphics inspection plus exact ROM-slice checks
support tile meanings/aliases; command streams parse exactly to their bounds.
All new confidence/effects/limits are recorded in ../commentary/progress.md and
../commentary/uncertainties.md. No new runtime input sequence, snapshot or fetch
coverage claim; these batches are static semantic and local-asset review.

Fresh verify exit0, byte_identical=true; reading refresh exit0. All572 unit starts
match linker symbols, source pointers match named labels, ranges remain in ROM,
and all161 scaffold identity/evidence records match frozen state. Original ROM,
frozen/current map/state hashes and frozen reading/source identity are preserved.
Full phase-2 editorial completion remains active; audio and final audit pending.
No new toolkit issue; existing T001 remains nonblocking (toolkit-issues.md).
See ../commentary/verification.json and ../commentary/progress.md.

## Phase-2 commentary batches 006–007 — 2026-10-13

Completed static review/commentary of bank-0 raster scrolling, ball collision,
signed 8.8 integration and paddle/serve control, with all 25 velocity tiers,
impact maps and scroll-hit threshold data. Phase-2 inventory is 82 reviewed /
79 pending entries, 367 reviewed units; scaffold coverage is 15504 reviewed /
17264 pending bytes, exactly 32768 total. Reviewed source reports concrete
bonus pass-through versus normal/solid reflection, array/record units and bounds,
speed/angle fields, sprite composition and shared clamp differences. Corrected
FFC5 interpretation from relaunch delay to scroll-threshold index. Investigated
trailing-Y/detached alignment and unused angle-tail intent limits are recorded
in ../commentary/uncertainties.md. All new evidence is static; no runtime route,
input sequence or execution-coverage claim was added.

Fresh commentary verify passed exit 0, byte_identical=true; reading refresh exit
0. All 367 unit starts agree with linker symbols and source pointers resolve to
their named labels. All 161 scaffold identity/evidence records and frozen/current
phase-1 map/state hashes match preparation; original ROM unchanged, frozen
reading copy matches before/. No toolkit modification or new issue; T001 remains
nonblocking (toolkit-issues.md). Full phase-2 semantic/editorial completion remains
active work. See ../commentary/progress.md and ../commentary/verification.json.

## Phase-2 commentary batches 004–005 — 2026-10-13

Consumer-backed semantic/editorial review now covers 70 / 161 scaffold entries
(11176 / 32768 bytes) with 230 reviewed units; 91 entries / 21592 bytes remain
pending. Reviewed state dispatch/title/demo/stage/launch/loss/completion flow,
all 24 referenced stage layouts, brick records and extra-ball thresholds.
Source uses named state/layout pointers and explicit record/row formats while
remaining byte-identical. Corrected the sequence helper interpretation after
observing that its returned A supplies demo stage selection. Partial reviewed
loader/count/extra-ball and game-over/pause/audio units do not mark their larger
physics/audio regions complete. Remaining scrolling/rendering findings are
recorded for continuation in ../commentary/progress.md.

Fresh commentary verify passed exit 0, byte_identical=true; reading copy refresh
passed exit 0. All 230 unit starts match linker symbols and source pointers
match named labels. All 161 scaffold identity/evidence records are preserved;
original ROM, frozen/current phase-1 map/state hashes match the manifest, and
reading-before matches before/. Source notices remain intact. The 31-byte
phase-1 unknown interval is unchanged. No runtime experiment or additional
execution coverage claimed. Whole-game phase-2 review/editorial audit is still
unfinished. Existing T001 is nonblocking; no new toolkit issue encountered.
See ../commentary/verification.json, ../commentary/progress.md and
toolkit-issues.md.

## Phase-2 commentary batch 001 — 2026-10-13

Started static semantic reading of bank-0 physical $0000-$0301 in the independent
commentary/source tree. Added comments explaining reset clear bounds, vector fill,
VBlank/STAT/serial service, frame waits, LCD synchronization and VRAM command
format/modes. Cross-read serial consumers and the $7ff0 audio trampoline; replaced
the numeric call with VBlankAudioTrampoline. No runtime experiment or new execution
claim: all new interpretation is static. No phase-1 map/state/checkpoint or input
ROM changes. The unresolved physical $0426-$0444 interval (31 bytes) remains
unknown as recorded in the phase-1 investigation ledger.

Fresh commentary verification passed (exit 0, byte_identical=true, original ROM
SHA-256 ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2);
reading copy refreshed (exit 0). Phase-2 inventory is still 0 reviewed / 161
pending while unit boundaries, descriptive symbols and dependencies are completed.
See ../commentary/progress.md and ../commentary/source/bank_000.asm.
Existing toolkit T001 is a nonblocking missing feature; no new issue encountered.
See toolkit-issues.md. The whole-game semantic review and editorial audit remain
active work; phase-1 completeness is not claimed.

## Pass 007 — blocked audit

Previous goal turn: **progress**, from the corrected multiplier label, strengthened table evidence and new range-filtered data-read experiment. This turn revalidated the same evidence-specific blocker; it did not classify new bytes and is not a verified wait. No workspace analysis process is running.

Fresh API checks on the authoritative current state: map.load and map.validate passed; analysis.unknown returned exactly one gap, offset 1062, length 31; disasm.verify against generated/pass-006 returned `ok=true, byte_identical=true`. Input and portable-map hashes match pass-006-verification.json. Final save/export also passed, recorded in [pass-007-final-results.json](pass-007-final-results.json). The current verified checkpoint remains [pass-006](pass-006-verification.json); no unchanged checkpoint is presented as new classification progress.

| Physical bank | code | data | image | padding | text | unknown | total |
|---|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4550 | 11563 | 0 | 240 | 0 | 31 | 16384 |
| 1 | 4953 | 2887 | 6144 | 2400 | 0 | 0 | 16384 |
| total | 9503 | 14450 | 6144 | 2640 | 0 | 31 | 32768 |

All 160 classified regions have role/evidence metadata. Execution evidence remains 2,310 ROM opcode starts and 8 HRAM starts, separately reported; no new runtime scenario was run. ROM bank contexts remain fixed bank 0 / switch bank 1 with no unresolved bank translation. Current dependency recheck confirms no decoded n16/a16 reference into 0426-0445; the sole IndirectJump remains the established state dispatcher at 054b. The existing 257-frame targeted probe has no errors, no matching read/fetch events and no detail/aggregate drops. Those facts do not establish the original structural role of the zeros.

Blocked audit: the same missing evidence has persisted through the pass-005, pass-006 and current goal turns. Local control-flow, pointer, consumer, neighbor/format and targeted runtime methods are documented in [investigation.md](investigation.md); the new external lesson lead was checked in pass 006 and provided no applicable artifact. No concrete unblocked method now distinguishes dormant NOP code, reserved zeros and unused zero data. Further classification would require guessing. Reopening requires an applicable source/symbol description or a newly evidenced natural caller/access; this is not a toolkit blocker or universal unknowability claim. Whole-ROM completion remains false, and the goal is being marked **blocked**, never complete.

Toolkit findings unchanged: T001 metadata-authoring API limitation, with a working supported-storage workaround; no confirmed bugs and no new toolkit findings. [Toolkit report](toolkit-issues.md). Input and toolkit code unchanged; source/assets remain local.

## Pass 006 — continuation revalidation, annotation corrections and targeted reads

Previous goal turn: **progress**, because it classified regions, saved/exported the map and produced verified listings. This continuation also made progress: corrected an erroneous routine label, strengthened two table annotations and added previously missing data-read evidence. The whole-ROM goal remains incomplete and active.

Authoritative checkpoint: [pass-006 verification](pass-006-verification.json), [current portable map](../maps/current.sym), [bank 0 listing](../generated/pass-006/bank_000.asm), [bank 1 listing](../generated/pass-006/bank_001.asm). Internal and stock maps validate; byte-identical rebuild passed. Current map SHA256 is `18d33cc539fdf5739ab7036e38fbc67118ca06b1427de4cf7a64c9ec0ac464dd`; rebuilt/input SHA256 remains `ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2`. Input unchanged. Counts rederived from the current internal store:

| Physical bank | code | data | image | padding | text | unknown | total |
|---|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4550 | 11563 | 0 | 240 | 0 | 31 | 16384 |
| 1 | 4953 | 2887 | 6144 | 2400 | 0 | 0 | 16384 |
| total | 9503 | 14450 | 6144 | 2640 | 0 | 31 | 32768 |

All 160 classified regions retain role/evidence metadata internally and in annotations.json, with matching metadata verified after load/save. All 38 labels export and rebuild. Structural roles now have stronger evidence at `1b71-1b7d` (consumer `19e2-19f6` clamps its index to 0..3 and multiplies by three) and `4551-455d` (three selectors, followed by three three-tile records, accessed at `4554+3*selector`). Both confidence values were raised to .99 through map.annotate after inspection. `MultiplyBByE` was incorrectly labeled at `0453`, which is a RETI; it is now at the actual helper entry `0454`. Region extents/kinds did not change. Evidence: pass-006-inspect-results.json, pass-006-edit-results.json and pass-006-metadata-results.json. Toolkit code unchanged.

New targeted experiment: [zero-probe-results.json](zero-probe-results.json), replay via `python3 analysis/frame_scenario.py zero-probe`. Hypothesis: the zero block is read as joypad/reset-related ROM data even though it was never fetched as code. Trace configured immediately after reset with `enabled=true`, capacity 1000, space cartridge-rom, physical range `[1062,1093)`, kinds Instruction/Fetch/Read. Exact stage-play route was replayed, followed by Select+Start held two frames and released for two frames. Every call completed without error. Final state: 257 frames, 4,009,009 operations, 19,257,508 PPU ticks, state byte 2 after reset to title setup. Retained/query events 0, dropped detail 0, aggregate drops 0. This establishes no read/fetch access **on that route**, not all possible play. ROM opcode coverage is 2,310 distinct starts plus 8 HRAM starts, reported separately from classification.

A newly located historical ASMSchool page mentions a planned Alleyway disassembly but its HTML has no linked Alleyway lesson/source. The Super Mario Land link leads to a different game's notes and does not identify this reservation. Both accessible HTTP pages were inspected; no ROM/archive/assets were downloaded and no external claim was used to classify local bytes. [Investigation ledger](investigation.md) records this eliminated lead and the remaining evidence problem.

Unknown remains exactly `0426-0445` (31 zero bytes): dormant NOP code, reserved fill and unused zero data remain competing structures. Raw emission, legal decoding and the no-access trace do not discriminate them. Current dependency review found no new caller, pointer consumer, bank context or natural input route targeting the interval. Reopening requires an applicable source/symbol description or new naturally sourced access/reference. The latest checkpoint is a fidelity result, not whole-ROM success. Goal not marked complete or paused.

Toolkit findings unchanged: one missing API capability, T001 metadata authoring, with a verified supported-storage workaround; no confirmed bugs and no toolkit blocker. [Toolkit report](toolkit-issues.md).

## Pass 001 — 2026-10-13

Baseline and pass-001 both rebuilt byte-identically. [Verification](pass-001-verification.json) records ROM SHA256 ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2 and exported map SHA256. `python3 run.py inspect` confirms 32768 bytes, ROM-only controller, two physical banks, valid header checksum. Header title ALLEY WAY, destination 1, version 0; these fields do not independently establish release provenance.

| Physical bank | code | data | image | padding | text | unknown | total |
|---|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4396 | 108 | 0 | 0 | 0 | 11880 | 16384 |
| 1 | 4458 | 0 | 6144 | 0 | 0 | 5782 | 16384 |

56 nonoverlapping regions have role/evidence annotations in [annotations.json](annotations.json); ROMAtlas's region schema does not support textual annotations. Internal map validated, saved to state.json, exported to ../maps/current.sym with include_sym=false; stock validation and checkpoint rebuild succeeded. Unknown gaps remain unknown internally, despite raw-byte emission in generated source. No classification completeness claim.

Evidence: [static-results.json](static-results.json) retains all five discovery pages (4488 candidates; budget 10000, not exhausted, 1900 references). [static-decoded.json](static-decoded.json) inspects every candidate: 8842 distinct bytes, no overlaps or decoder warnings. Roots are reset/interrupt vectors, semantic frame-wait continuation at 0226, sixteen dispatch targets read from 054c-056b, and VBlank's call at 7ff0. CPU aliases equal physical offsets in this ROM-only cartridge; fixed bank 0, switch bank 1 throughout. Static roots prove structural code, not execution.

Observed coverage: [collect-results.json](collect-results.json), 190 ROM opcode addresses, no aggregate drops; 352597 dropped detail events were expected with capacity zero. Decoded lengths provide code extent; observed provenance is used only for fetched opcode addresses. 141000 operations, 1088736 PPU ticks, ending PC 0226; only one frame. Inputs occurred largely during initialization and therefore do not confirm title/gameplay/pause outcomes. [collect.py](collect.py) contains exact operation budgets, presses and releases; snapshots were not used. Initial 1000000 and 100000 operation calls with detail tracing timed out after 10000 ms; their partial results remain in initial-results.json and were not used as completed replay sequences. Successful replay reduced calls to 10000/3000 operations with capacity zero.

Semantics: 0150 waits for VBlank before disabling LCD, initializes memory and peripherals; 0381-03b4 copies three 0800-byte 2bpp tile blocks from 4b75/5375/5b75 to VRAM 8000/8800/9000. 0199-01a4 copies the 12-byte DMA routine at 03b5 to HRAM FF80; VBlank calls it at 01fe. 0221 clears FFA0 and waits until VBlank sets it, supporting the continuation after HALT. 053c doubles FFA4, reads a little-endian pointer at 054c and jumps to it; sixteen aligned entries end at first handler 056c, with final three pointing to RET 022c. State-index bounds remain an investigation item.

Unresolved: 17662 bytes in 31 gaps, including large level/layout and sound candidates. Full unknown ranges are retained in classification-results.json. Investigation paths remain open; goal active. Next: parse ROM data consumers and their exact lengths, audit dispatch index bounds, complete startup before issuing gameplay inputs, inspect detached/unreferenced code and filler boundaries.

Toolkit findings: one missing feature (role/evidence annotation storage); no confirmed toolkit bugs. [Report](toolkit-issues.md). Toolkit was not changed.

## Pass 003 — consumer-backed data and detached routines

Added 61 regions; 117 total, all with role/evidence records in annotations.json. [Verification](pass-003-verification.json): internal map validation, stock map validation and byte-identical rebuild passed; exported map SHA256 b294436caa0efdd73ac46a245adf7a2c225123ef90b482a8b3ac96a07d2e421d. Pass-002 also verifies but contains the unchanged pass-001 map: an initial edit batch was rejected for crossing physical bank 4000 and was not saved. The batch was replayed with explicit bank splitting; pass-003 is the new authoritative checkpoint. This was documented validation behavior, not a toolkit bug.

| Physical bank | code | data | image | padding | text | unknown | total |
|---|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4550 | 11563 | 0 | 0 | 0 | 271 | 16384 |
| 1 | 4951 | 1172 | 6144 | 0 | 0 | 4117 | 16384 |

Evidence: detached-results.json inspects coherent standalone helpers (absolute difference, interrupt mask, sign-directed motion wrapper, bounded speed increment, sound request test selectors and sound flag setters). detached-static-results.json traverses those semantic roots plus prior roots; detached-static-decoded.json contains 4830 legal nonoverlapping instruction starts. Added 647 code bytes without execution claims. Known runtime coverage remains 190 ROM instruction addresses.

classify_data.py preserves reproducible consumer/format checks: stage table at 1be1 has 33 three-byte records; 24 distinct pointed layouts span 1c44-3e44, all 14-cell rows, cell values 0..4 and FF sentinel. 3e44-4075 is an unreferenced 40-row grid with the same format (560 cells plus FF), split at physical bank 4000. 4075 holds twelve scrolling-pattern pointers followed by sixteen 20-byte columns; last four are structurally matched but their use is unconfirmed. 11ee has 25 velocity pointers to 76-byte tables ending at 198c; detached 0f6e bounds the speed index at 26 and wraps to 3. 1aaf/1ade/1b0d/1b33 are fixed-length copied command streams, confirmed by destination/mode/count grammar and terminator. 41cd-42b3,42b3-4333,4a3c-4a66 parse as terminated VRAM commands through 02c1. 4a8b has thirteen big-endian sprite pointers, each selecting a 16-byte four-sprite composition; payloads end exactly at 4b75. Palette, scoring, brick and paddle lookup regions have consumer-specific evidence in annotations.json. Confidence ranges .85-.95 where repeated layout supports unused tail records, without claiming those were fetched.

Remaining 4388 unknown bytes in 15 ranges: 0003-0040,0043-0048,004b-0050,0053-0058,005b-0060,0061-0100,0426-0445,4333-43dc,44f5-450d,641a-6800,694b-694c,6f17-6f18,6fac-7050,7531-7ff0,7ff7-8000. Next: sound record/sequence consumers, motion lookup 44f5, fill/empty-area ambiguity, and runtime replay after initialization. State-index bounds still need audit. Concrete paths remain open; no exhaustion claim.

Toolkit findings unchanged: T001 missing internal annotation metadata; no confirmed bugs and no blocker. [Report](toolkit-issues.md).

## Pass 004 — sound structures and later startup replay

Added 34 regions, 151 total with role/evidence annotations. [Verification](pass-004-verification.json): internal/stock map validation and byte-identical rebuild passed, map SHA256 8a43be4e8eea22a3819d49f487acd928764fe29f8140e1652f4f6a32cc30813f. Counts:

| Physical bank | code | data | image | padding | text | unknown | total |
|---|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4550 | 11563 | 0 | 0 | 0 | 271 | 16384 |
| 1 | 4953 | 2887 | 6144 | 0 | 0 | 2400 | 16384 |

classify_sound.py checks every voice byte between 75e3 and 7a6f against the observed static consumer grammar: >=80 is duration control indexed by low7 bits, 01 rests, 00 stops, 7f loops, other bytes select notes. 24 terminated/looped sequences include extra contiguous streams whose callers are not established. Note indices fit parallel 67-byte high/low frequency tables; all duration indices fit 44-byte duration table. 6f9d copies five consecutive register values from presets; 7073 copies four values from 704c. 745f copies exactly 16 waveform bytes from 7a6f into wave RAM. 44f5 is now supported by explicit 24-step movement loops. 4333-43dc matches twelve 14-cell rows and FF sentinel, structurally classified as an unused grid. Two trailing RET bytes have semantic unused-stub evidence, without execution claims. Confidence is lower for unreferenced repeated structures; usage remains distinct from structural classification.

Runtime [gameplay-results.json](gameplay-results.json): deterministic completed 311000 operations, ten frames, 809 unique fetched opcode addresses across ROM and HRAM, zero aggregate drops. 387411 detail drops expected at capacity zero. All ROM opcode addresses fall within prior code classifications. Replay: collect.py gameplay performs 22 calls of 10000 operations before Start/A/right/B/left/Start/Start each held for 3000 and released for 10000; all timeouts 10000ms and no errors. Startup initializes successfully and VBlank/DMA execute; buttons held for less than a full frame may not be sampled. Title/gameplay/pause outcome confirmation remains open. HRAM alias fetches establish copied DMA execution separately from ROM opcode coverage; do not count them as new ROM bytes.

Remaining 2671 bytes: FF gaps 0003-0040,0043-0048,004b-0050,0053-0058,005b-0060,0061-0100,641a-6800,7a7f-7ff0,7ff7-8000 and zero gap0426-0445. Next: audit references and fill boundaries, distinguish zero reservation from dormant NOP code, use frame-budgeted input holds and inspect game state/VRAM. Goal active; concrete paths remain.

Toolkit findings unchanged, T001 missing feature; no confirmed bugs. [Report](toolkit-issues.md).

## Pass 005 — final current checkpoint and unresolved zero reservation

Outcome: **incomplete**, with32737/32768 physical bytes classified. Whole-ROM goal remains active; it has not been marked complete. [Verification](pass-005-verification.json) proves a byte-identical rebuild with internal and stock map validation. The rebuilt ROM SHA256 matches input exactly; input unchanged. [Portable map](../maps/current.sym), [bank000 listing](../generated/pass-005/bank_000.asm), [bank001 listing](../generated/pass-005/bank_001.asm). Map/checkpoint SHA2567577f072b2f003a142ab39fd8ac2cbed2b5df6acaf364b0b0909a211d6bbb1cf.

| Physical bank | code | data | image | padding | text | unknown | total |
|---|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4550 | 11563 | 0 | 240 | 0 | 31 | 16384 |
| 1 | 4953 | 2887 | 6144 | 2400 | 0 | 0 | 16384 |
| total | 9503 | 14450 | 6144 | 2640 | 0 | 31 | 32768 |

All160 classified regions have structural role and supporting evidence in annotations.json **and** internal state.json region_metadata, with matching stock directives.38 semantic labels survive load/save/export and rebuild. Generic metadata was inspected in toolkit source and tested in metadata-results.json; the map.annotate API limitation is worked around without toolkit changes. Required annotations cover classified regions only; the remaining31 bytes deliberately have no asserted structural role. Internal map has no overlaps; unknown gap is exactly0426-0445. No unresolved ROM bank contexts: fixed bank0, switch bank1, CPU aliases equal physical offsets in their respective windows. FF regions are now padding: explicit hardware-slot/section/tail boundaries, exact erased-byte runs, no incoming decoded control references/RST instructions, and surrounding bounded data consumers. See classify_fill.py.

Runtime [stage-play-results.json](stage-play-results.json) completes3937951 operations,18916780 PPU ticks,253 frames.2307 unique ROM opcode starts and8 HRAM starts observed; no aggregate drops.1740446 detail events dropped by deliberate zero-capacity tracing. All fetched ROM starts fall in prior code classifications. Reproducible frame_scenario.py stage-play route confirms title-to-stage entry, launch, paddle movement and pause/resume (see gameplay.md). No full hardware compatibility or100% execution claim.

Remaining evidence problem:31 zero bytes0426-0444 followed by the separately classified RET0445. Byte decoding yields31 NOPs then RET; this is consistent with a dormant delay/no-op stub, zero-filled reserved space before a return stub, or an unused zero table. Legal decoding, lack of runtime fetches and lack of pointers do not choose between those interpretations. [Audit results](audit-results.json) and [investigation ledger](investigation.md) record attempted methods and exclusions. No raw-gap-to-data conversion was used internally. Portable export's .data for this gap is fidelity only.

State-index audit: the sole IndirectJump is054b. FFA4 writes in all4830 decoded instructions are immediate0..12 or B selected4/9 at0814-0820; reset clears HRAM. No absolute FFA4 write or direct HL base atFFA4 exists. All16 handler table targets were traversed, including unused RET targets. This resolves the identified dispatch traversal context; it is not a proof against arbitrary hardware corruption.

Completion audit: identity/input invariant passed; internal map passed; stock export and both bank listings passed; byte equality passed; all160 classified regions have required annotations. Whole-ROM classification fails on31 bytes. Local static/semantic/pointer/neighbor methods for that interval do not establish intended structure; no known input route distinguishes dormant stub from reserved bytes. A source/symbol artifact identifying this reservation or a newly evidenced natural access would reopen the entry. No toolkit blocker is demonstrated. This is an incomplete handoff under the investigation contract, never whole-ROM success; goal tooling has no incomplete status, so goal remains active without an unauthorized pause.

[Machine-readable current audit](current-audit.json) records exact per-bank counts,160 annotation records,38 labels, checkpoint hashes,2307 ROM/8 HRAM fetched starts, and all32 little-endian/10 big-endian raw pointer hypotheses into the unresolved interval. Rebuilt hash and input hash were directly compared and match; current.sym equals the pass005 map byte-for-byte. Replay routes and actual frame/operation outcomes are retained in scenarios.json.

Toolkit findings: T001 API metadata authoring gap, now worked around via supported generic storage; no confirmed bugs and no blocker. [Report](toolkit-issues.md). No toolkit code changed; no ROMs downloaded or workspace committed. Input, traces, state, assembly and assets remain local; only stock .sym is distributable.


## Phase-2 commentary batches 002–003 — 2026-10-13

Completed static semantic review of physical bank-0 $0000-$053b, including
vectors/header, shared frame/LCD/VRAM helpers, detached rectangle writer,
graphics/OAM loaders, joypad edge filter, serial position/button interface,
arithmetic/decimal helpers and foreground loop/reset gate. This records
37 reviewed / 124 pending scaffold entries and 100 reviewed units in the
independent commentary inventory. Added descriptive symbols/HRAM constants
with byte-preserving operands. No new runtime trace or execution claim.

Editorially reviewed the 31-byte $0426-$0444 ambiguity using the existing static,
pointer and zero-probe evidence. Source still emits raw zeros; phase-1 unknown
kind, state, maps and checkpoints are unchanged. Specific investigated limits
are recorded in ../commentary/uncertainties.md. Verification/read after each
applied edit group passed; final results are in ../commentary/progress.md.
Whole-game phase-2 review remains unfinished. Existing T001 is nonblocking;
no new toolkit issue encountered (toolkit-issues.md).

Final batch checks: fresh verify exit 0, byte_identical=true; reading refresh
exit 0. All 100 unit starts match RGBDS linker symbols and every source pointer
matches its named label. Inventory identity/evidence matches the frozen scaffold
for all 161 entries. Original ROM and frozen/current phase-1 map/state hashes
match preparation manifest; frozen reading copy matches the frozen source.
The whole-source editorial audit remains pending. See
../commentary/verification.json and ../commentary/progress.md.
