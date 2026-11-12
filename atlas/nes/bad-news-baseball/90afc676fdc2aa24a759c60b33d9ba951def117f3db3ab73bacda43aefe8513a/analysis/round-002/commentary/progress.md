# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round 001 current-source review milestone

Inspected the entire 10,683-line frozen source in physical order. Added 174 subsystem/routine explanations spanning all 7,938 classified instructions, 1,058 routine/branch symbols, shared hardware/RAM equates and local notes for every raw interval. Full 693-entry scaffold now records reviewed scope, physical ranges, inherited effective bank views and source locations. Specific explanations cover screen decoding, attribute quadrants, presentation script commands and $11-byte objects, audio records/commands, menus and two-byte roster records, password packing/checksum, match transitions, fixed-point ball movement, controller debounce, NMI/PPU queue, MMC1 wrappers and record readers. Local names preserve unknown address identity where scratch fields are reused.

First coherent annotated-source verification passed byte-identically (source hash 1066c148e61d31627263f0fe9fafeb5bc5731891c201ec48dcdc789386e2cd6f). Final review is still checking the new names and prose for concrete inconsistencies; fresh handoff verification and reading-copy refresh remain. Phase1 classification is unchanged and incomplete. Editorial/archive acceptance remains pending.

## Round 001 commentary handoff gate

Completed full current-source readability. No pending classified-source ranges remain. All 693 scaffold entries are reviewed; 502 Code records contain compact local units and preserve their phase1 bank views. Added 62 named CPU lookup bases, explicit record-buffer layout and concrete helper input/output/calling-convention notes during the consistency pass. Hardware and understood RAM operands use symbols while explicit a:/z: encoding widths are preserved. Address-based work names remain where storage is shared or purpose is not established. No structural map/state changes were made.

Fresh final verification passed: commentary/verification.json reports byte_identical=true and source SHA256 ec5bcb5c5d31a2b7d07ea1b255e2f055f5d95dffb4414be908cb5e3a7814a49d. commentary/reading-after.asm was refreshed after that final source. The frozen before source, baseline map/state and all earlier analysis checkpoints remain intact. Final editorial_audit is deliberately pending; no archive acceptance or whole-ROM completion is claimed. Unresolved meanings and practical redirect checks are in uncertainties.md. No toolkit issues found; analysis/toolkit-issues.md remains unchanged.

Historical construction/refinement scripts are retained in analysis/scratch/round001-commentary-*.py. They document this first baseline transformation; do not rerun them against an already annotated source, since they expect their corresponding input revision.
