# Commentary progress

Round 1 commentary completed across all current classified source: 9,323 Code bytes and 137,385 Data/Text bytes, plus honest boundary/uncertainty notes for 246,524 unknown bytes. Frozen before/, baseline.json and phase1-state.json preserved. Source remains local and independently buildable.

Reviewed all 6,075 baseline source lines. Added 216 routine/handler contracts and compact units, 351 control-flow labels and consistent RAM, hardware, table and field aliases. Explained menu/movement bytecode, animation records, graphics commands, resource headers, sprite groups, 24-bit jobs, controller accumulation, text commands, sound sequencing/packets/instruments/APU shadows, MMC1/NMI/reset, title/window responses and speech selection/descriptor mechanics. Final consistency review corrected the character-translation setter ($C73B) versus string-printer ($C740) boundary and startup description. Review scaffold retains all 417 physical entries; all have reviewed source locations and summaries.

[Uncertainties](uncertainties.md) records unresolved meanings and concrete bounded redirect questions. No current classified-source readability gaps remain. Whole-ROM classification remains 37.31% (146708/393232 bytes), unchanged by commentary.

Fresh commentary.py verify passed byte-identical with ROM SHA256 f442bde3a19e6070fd55a7f025d1fbf463a43a960a62f4c44d6fed19fcd9c6d1; verification.json records source hashes. commentary.py read refreshed reading-after.asm after the final source edit. Final editorial_audit remains pending; no final audit invoked. No toolkit issues found.
