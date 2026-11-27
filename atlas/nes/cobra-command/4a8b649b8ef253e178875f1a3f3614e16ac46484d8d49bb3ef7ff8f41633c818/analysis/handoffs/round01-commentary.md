# Round 1 commentary handoff — current-source readability complete

ROM classification remains incomplete: 234161 / 262160 bytes (89.32%); Code 21382, non-code 212779, Unknown 27999. No structural annotations, maps, input or prior evidence were changed in this phase.

## Scope and assessment

Completed readability across all currently classified source, including the initial backlog: 952 Code regions, 58 Data regions and one 131072-byte CHR Image. All 213 Unknown scaffold intervals have explicit local notes explaining opaque raw inclusion without inventing code/data roles. Every meaningful recovered routine/control block/data consumer has purpose/mechanical descriptions, consistent names where established, and ordinary uncertainty where game semantics remain unresolved. Descriptive routine/branch aliases coexist with frozen physical generator labels. Hardware, player/projectile parallel arrays, bank selectors, camera/patch state and menu/sound scratch are named; scratch aliases are scoped to their owning banks/modes.

Reviewed inventory: all 1224 scaffold entries, 514 compact meaningful source units, 1131 clipped unit associations attached to scaffold Code entries. No remaining classified readability ranges: `analysis/commentary-round01/remaining.json` is `[]`; scope/identity check: `analysis/commentary-round01/scope-check.json`. Final editorial audit remains pending and was not run. This assessment is current-source readability, not whole-ROM classification or final acceptance.

Maintained local source: `commentary/source/game.asm`; refreshed reading copy: `commentary/reading-after.asm`. Frozen `before/`, `reading-before.asm`, `baseline.json` and `phase1-state.json` preserved. No regeneration over maintained source.

## Checkpoint and fidelity

Classification checkpoint **pass-001**, unchanged map SHA256 `30a0e9f8324b21c1d9582312c4fde1da220b1d0f3600ed103cc5bb694765d69a` (`maps/current.json` and frozen `maps/pass-001.json`). Original ROM SHA256 unchanged `4a8b649b8ef253e178875f1a3f3614e16ac46484d8d49bb3ef7ff8f41633c818`.

Fresh supported commentary verification: `commentary/verification.json`, ok=true, byte_identical=true. Maintained source SHA256 `21c78f4cea6618ea976b733b87b3a66b35a473f8dfe435749cb3ca7403774378`. Reading copy refreshed after the final source edits. Source still reconstructs the entire input exactly, including unknown ranges, without claiming those ranges classified.

## Examples

- Reset/NMI/main loop, camera save/restore and MMC1 serial writes: source `ResetHardware`, `NmiHandler`, `SaveSceneForEquipment`, `WriteMmc1Prg`. Explicit lower-bank changes and upper fixed-slot context are described.
- Metatile/attribute algorithms: `ExpandAndUploadPatchMetatile`, `MergeMetatileAttribute`, `BuildTileRowBuffer`, `RedrawCameraViewport`. Packed two-bit palette selection, quadrant masks and nametable shadow layout are explained.
- Sound driver: `DispatchSoundStreamOpcode`, `SoundCommandEndSecondaryChannel`, counted stream loops and per-channel stream return slots. Nonlocal CPU-return unwinding is documented rather than treated as a leaf-call contract.
- Equipment/text: `RestoreScratchAndLeaveEquipment`, `DispatchEquipmentPpuOperation`, `ReadEquipmentTextToken`, `FillEquipmentPanelTiles`. Mode-local scratch reuse and raw counted OAM format are distinct from gameplay composites.
- Player/weapon/ending: `UpdatePlayerGameplay`, primary/secondary movement and impact routines, `UpdatePlayerReachAndActorProximity`, `RunEndingTextAndCredits`. Static mechanics are described; tentative rescue semantics and artificial ending reachability remain explicit.

## Corrections and actionable redirect questions

Published R1-C001..R1-C004 in `analysis/leads.json`; prior R1-L001..L004 preserved, none consumed or rejected. Supporting limitations: `commentary/uncertainties.md`.

1. **R1-C001 structural correction:** physical `$1cedd..$1cee1`, CPU `$cecd..$ced1`, is a four-byte sprite-zero Y/tile/attribute/X descriptor consumed by `$ceb4`; phase1 merged it into Code. Source explicitly describes its generated mnemonic encoding as data; independent redirect review should split exactly these four bytes. No map mutation performed here.
2. **R1-C002 semantic correction:** physical `$1ec83..$1ecb3`, CPU `$ec73/$ec83/$ec93`, comprises three scene sound-command tables, not CHR selectors. Direct consumer `$ec58` selects a table and passes entries through `$ec2f` to sound bank six `$8003`. Source/review corrected; historical map evidence retained for later reconciliation. Data bounds unchanged.
3. **R1-C003 refined reach/rescue lead:** source `$a660..$a93d` gives actor-seven state/position fields, exact horizontal/vertical thresholds, reach flags and actor pending values. Recover the proper actor initialization/spawn contract before a targeted alignment experiment. This materially improves R1-L004; do not repeat the unsuccessful boss flag patch.
4. **R1-C004 wrapper postconditions:** `$ec2f` saves/restores the PRG selector through sound command acceptance; `$ebd9/$ebde` update shadows; `$ec13` writes hardware; `$fce4` deliberately exits in bank four. Independently prove the narrow sound scratch/selector-domain contract, then resume supported caller continuations for R1-L003. Do not assume arbitrary mapper callees preserve context.

No deferred classified readability work. Deferred meanings include exact generic-data schemas, song/item names, player terrain/hit-state game labels, complete indirect-selector domains, natural rescue/boss routes and unknown handler/stream/text gaps. These are localized in source and do not weaken the classification evidence or create fictional continuations.

## Toolkit and retention

One existing missing feature NES-MF-001, zero confirmed/suspected bugs, no blockers; `analysis/toolkit-issues.md`. No new toolkit issues found in commentary, no toolkit edits. No archive, cleanup or publishing. All source/assets/evidence stay local. Free disk approximately 3.6 GiB; owned commentary notes/scripts approximately 0.5 MiB. Completed batch scripts are one-shot edit provenance; do not rerun over maintained source. Resume any later commentary from the current source/review credit, freezing the entire prior commentary before a changed-baseline preparation as required by the phase prompt.
