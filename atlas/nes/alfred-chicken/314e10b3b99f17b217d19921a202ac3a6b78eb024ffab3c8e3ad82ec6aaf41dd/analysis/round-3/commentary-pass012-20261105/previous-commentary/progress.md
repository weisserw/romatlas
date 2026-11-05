# Round 2 current-source commentary complete — pass-007

Full prior version preserved before preparation at `analysis/round-2/commentary-pass007-20261105/previous-commentary/`; identity-manifest.json covers every file. All1021 prior classified ranges retain exact offset/length/kind/views. Carryover preserved names, prose, symbols and review credit; dependencies remain valid because prior targets and mapped views are unchanged. New baseline was prepared with the configured release workflow, never generated over maintained source. Frozen before/ remains unchanged.

Readability covers all31439code bytes,70531data bytes,13text bytes and16header bytes; unknown29105bytes retain honest boundary notes. New4226code bytes were read in physical order in compact-new-code.txt and explained through135 additional routine/control contracts, with139 contiguous code review units. New53600data bytes have accepted format/boundary explanations;399 specific data structures have descriptive physical backing labels and prose. Generic non-code slices retain bounded pattern/consumer uncertainty. All3648 scaffold records remain intact, reviewed for current-source readability;856 meaningful units include carried credit. Final editorial audit remains pending. No remaining classified-source readability ranges.

Names and symbols preserve exact instruction widths, constants and effective PRG contexts. Added aliases identify fixed actor slots and local encounter counters. Code explanations cover synthetic platform returns, allocation failure unwinding, line-motion error state, channel-specific bytecode grammar, bounded resource tables and nonreturning stack-reset transitions. Physical backing labels are explicitly distinguished from CPU addresses. Existing reviewed code was retained and numeric direct transfers into new targets now use context-resolved symbols.

Examples:
- `SetVariantPpuDestination`: source/game.asm:3335.
- `DispatchOscillatingPlatform`: source/game.asm:11409.
- `BuildActorLineMotion`: source/game.asm:15095.
- `QueueEncounterDialogueLine`: source/game.asm:6701.
- `RestoreSavedPlayerEntry`: source/game.asm:33331.
- `CopyLongDecompressBackReference`: source/game.asm:35747.

Fresh final verification succeeds: ok=true,byte_identical=true; ROM SHA256314e10b3b99f17b217d19921a202ac3a6b78eb024ffab3c8e3ad82ec6aaf41dd. Exact source hashes in verification.json and immutable final-verification.json. reading-after.asm refreshed after the final edit. Final identity check confirms input ROM/current map/internal state and entire preserved prior commentary unchanged. No map/classification changes or runtime replay.

Remaining meanings/questions are in uncertainties.md and this phase's handoff.md: natural actor/flight selector domains, indirect stack contracts, graphics helper boundaries, behavior/persistence prefixes and exact audio semantics remain research questions. Classification remains101983/131088(77.80%); this is commentary completion only, not full-ROM or final editorial acceptance. Initial pending-ranges.json is a preserved preparation frontier, superseded by this completed status and final-scope.json.

No new toolkit issues found. Existing confirmed nonblocking NES-001 remains in analysis/toolkit-issues.md. No toolkit edits, redirect/disassembly work, archive, cleanup/deletion, publishing or subagents performed.

## Round 2 commentary corrective supplement — complete

Coordinator gate reopened this same phase after the preliminary handoff: shared numeric lookup had assigned value-constant names to RAM operands and used immediate values as RAM roles in generated labels. The earlier completed packet remains historical and untouched; this supplement supersedes its readability gate. Correction snapshot and complete audit evidence: `analysis/round-2/commentary-pass007-correction-20261105/`.

Audited all14854 instructions by original addressing class, including carried source:7599memory operands,2636immediates,4619control/implied instructions. Corrected7 RAM$08 references from BUTTON_START to work_byte and10 RAM$04 references from PPU_INCREMENT_VERTICAL to scratch_ptr. These are scratch storage addresses, not controller/PPU bit values. Context notes explain their decoder, clipping-width and sprite-loop roles. No actual RAM$10 operand was involved in the coordinator's example; original decoded bytes prove RAM$08. Exact values/storage classes and bank-specific table aliases pass independent final validation with zero mismatches.

Corrected402 new generated immediate-site labels so immediate bytes no longer imply RAM identities; corrected17 related storage-derived labels. Prior immediate destination labels were checked against their construction: they describe actual following stores rather than naming the numeric immediate as RAM. Shared old symbolic immediate masks remain in immediate consumers. Removed122 repeated per-instruction structural provenance comments for platform/decompressor/persistence slices, retaining functional explanations once and all original structural evidence unchanged in maps. Inventory source locations refreshed; all3648 scaffold entries and856 units remain intact and reviewed for current-source readability, with final editorial audit pending.

Fresh final verify succeeds byte_identical=true,ok=true after all source edits; reading-after refreshed. Source SHA25617c7b649e9615f1f1e1ab9fe846c7bab80f59a6250f62e68455971f7345610d3; symbols SHA2568421360d7de39b052ca53736c3a8c1800abeb4a0b9f66a006a114dae3940a05c. Exact final hashes, review and verification copied into the corrective packet. Input ROM/current map/internal state identities unchanged. No classified readability gaps remain after this corrective audit. Classification remains101983/131088(77.80%),29105unknown; no structural inference changed.

No new toolkit issues found; defect was in the local commentary edit recipe, not the configured toolkit. Existing nonblocking NES-001 retained. No redirect, final audit, toolkit modification, cleanup/deletion, archive or publication.
