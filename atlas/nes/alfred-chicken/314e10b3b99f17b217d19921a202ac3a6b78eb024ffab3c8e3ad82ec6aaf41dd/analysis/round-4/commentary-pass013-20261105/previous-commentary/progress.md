# Round 3 current-source commentary complete — pass-012

Preserved the entire correctedpass007 commentary before preparation at analysis/round-3/commentary-pass012-20261105/previous-commentary. identity-manifest.json covers all26 files; final checks show zero mismatches. All3394 prior classified map regions retain exact physical bounds/kinds/views, so valid prose, symbols and review credit were carried by physical match. Fresh pass012 source was independently prepared and verified before edits; frozen before/ remains intact.

Readability now covers all45066code and78209data/text/header bytes. New13627code bytes were read in compact-new-code.txt and explained with248 routine/control contracts;203 new data/text slices receive bounded format/role/uncertainty notes, including118 meaningful structures/text units. All3979 scaffold records remain intact and reviewed for current-source readability, with1222 units. Remaining classified readability ranges: none. Initial pending-ranges.json is preserved as the preparation frontier, superseded by final-scope.json. Final editorial audit remains pending;7813unknown bytes remain structurally unresolved.

Addressing-class audit checked all21035instructions:10567memory,4233immediate,6235control/implied; zero value/storage-class/bank-table mismatches. Immediate values never generated RAM-role labels. Memory$04/$08 retain scratch_ptr/work_byte rather than PPU/controller constants.44 new bank-specific CPU data aliases were checked across50 uses; physical Backing labels remain distinct from CPU aliases. Names preserve exact values and widths. Carried dependencies were rechecked through current contexts and byte-identical assembly.

Comments explain title glyph/PPU upload and numeric conversion, projectile orbit/profile/contact mechanics, actor response/line-motion/descriptor families, player recovery/slope/contact paths, actor81's saved-coordinate substates and reset setup. Conditional no-hit/hit unwinding, dispatcher state3 expiry, callback bank/stack restoration, bounded decoder output and TickAudio phase5 premises remain explicit. These do not assert arbitrary-input safety or natural reachability.

Examples:
- `ClearInternalRamExceptStack`: `commentary/source/game.asm:247`.
- `DrawTitleAttributesAndNumbers`: `commentary/source/game.asm:1016`.
- `Convert24BitValueToSixDigits`: `commentary/source/game.asm:6911`.
- `StepActorControlWordStream`: `commentary/source/game.asm:16322`.
- `UpdateActor81TimedSubstates`: `commentary/source/game.asm:27764`.
- `ResetAndInitializeTitle`: `commentary/source/game.asm:35220`.
- `ResolvePlayerLandingAndTileResponse`: `commentary/source/game.asm:42354`.

Fresh final verification after source edits: ok=true,byte_identical=true. reading-after.asm refreshed. Source SHA25621c25bc9114aa3a00826b5b6de745e600a9a0b7edb86d5f5465f113af4677c90; symbols SHA256edfffb2341c8a8c853dc337944392e6dd87c264dea4379fca067b2b87ea852c1. final-identities.json confirms input ROM, map/state and configured release binary unchanged, frozen map/state match current, and prior version intact. Final scope/review/verification are preserved in this phase packet. No classification changes or runtime replay.

Unresolved meanings and bounded next checks are in commentary/uncertainties.md: scene callbacks C9D7/C9C2 and fixed tails, residual projectile/contact code, bank9 text command grammar, mixed resource endpoints and physical1C2CC aliases. Generic noncode claims do not invent asset schemas; literal text does not validate adjacent commands. Classification remains123275/131088(94.04%),7813unknown; this is current-source commentary completion, not full-ROM or final editorial acceptance.

No new toolkit issues found. Existing nonblocking NES-001 and NES-002 remain in analysis/toolkit-issues.md. No toolkit edits, disassembly/redirect work, final audit, archive, cleanup/deletion, publication or subagents.

## Round 3 commentary corrective supplement — complete

Coordinator review found concrete naming gaps in the six-digit converter output and actor81 table entries. Resumed the same commentary phase. Preserved the complete preliminary current commentary in analysis/round-3/commentary-pass012-correction-20261105/preliminary-commentary with a per-file identity manifest; the original complete packet remains historical. Correction does not change classification.

Named decimal_digit_buffer at RAM$038C..$0391 consistently across all five absolute-indexed converter/queued/direct-PPU references; exact widths and bytes retained. Actor81 table bytes96B9C0B9D0B9 at physical179A0 independently confirm entries179A6/179D0/179E0 under[10,11,14,15]. Named their frame countdown, seeded hold and frame-restoration/retirement roles locally. Checked analogous packed-motion, line-motion, boundary-motion and guarded behavior-response entries, plus accepted actor/projectile role roots; added descriptive labels at sixteen additional established entry addresses, without treating invalid behavior selector0 or repeated suffix pointers as new selector proof. Specialized saved-coordinate, stack-unwind and bank contracts remain intact.

Refreshed all inventory source locations. All3979 scaffold records and1241units remain reviewed for current-source readability; final editorial audit pending. No remaining classified readability ranges. Fresh final rebuild succeeds ok=true,byte_identical=true and reading-after.asm is refreshed. Full21035-instruction addressing audit again passes with zero mismatches. Source SHA25637f29e235c8cb9bcf54f9c4c0c0ee03943b3c0c04fff3d2dd32d6e12f84adb83; symbols SHA256386ef08f13d7eaf564ce165a5a45e204008bf87c768b1b7eeab24d496937f934. Immutable input/map/state/configured binary identities match the disassembly handoff; preserved preliminary version identity checks pass.

Concrete source changes and proofs are in correct.py,entries.py,entry-audit.json,new-entry-units.json,scope.json and final-addressing-validation.json in the corrective packet. Current examples use Actor81CountDownFrames,Actor81HoldBeforeReturnFrames,Actor81RestoreFramesOrRetire and decimal_digit_buffer. Existing commentary/uncertainties.md remains the redirect question list; no semantic research question was replaced by fabricated certainty.

Classification remains123275/131088(94.04%),7813unknown. No new toolkit issues found; existing nonblocking NES-001/NES-002 remain in analysis/toolkit-issues.md. No toolkit changes, disassembly, redirect, final editorial audit, cleanup/deletion, archive or publication.
