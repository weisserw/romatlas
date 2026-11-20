# Round 2 disassembly handoff

Input: commentary-001. Output checkpoint: pass-002 (maps/pass-002.json, generated/pass-002,
analysis/pass-002-verification.json): map.validate ok, generate ok, byte_identical=true.
State: analysis/state.json (767 regions, loaded not reinitialized); maps/current.json exported.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
Per-bank counts: analysis/progress.md (Round 2 table). Bank 7: 6711 observed code,
5903 static code, 3770 data.

Leads consumed (all accepted, none rejected): L-008 (+62 data lists), L-001 (+1296 static
code, +64 tables), L-002 (+8 $CD generic data; $C040 code conf 0.7; lone rts $D76C/$E24A/
$E30F code conf 0.6), L-006 (bank 5 split into 31 regions, extra tile-source extents).
Details: analysis/investigation.md "Round 2 disassembly"; leads.json attempts.

New/changed regions needing commentary: bank 7 $C040, $C37B-$C3CB, $C8F0, $D76C, $DB90,
$DF57, $E24A, $E30F, $ECB3-$ECC2, $ED08, $EDD2-$EE78, $EF32-$EFE9, $F264, $F2B3-$F317,
$F333-$F431, $F467-$F4C4 (gaps), $F4E7, $F545-$F5FF, $F6AD, $F6FD, $F754, $F76A, $F7DB-$F853;
bank 5 $8000-$BFFF (new labels: Tiles2bpp_*, Tiles1bpp_*, FishK*SizeFramePtrs,
FishTileSetRecords, FishTileSets, LureActionProfiles, Exp1bppList* in bank 7).
Commentary should replace "unclassified (L-001/L-002)" notes in game.asm.

Weakest claims for review: $C040 and three lone rts (single-opcode); bank-5 generic remainders.
Reachability: new bank-7 code is Static with no known caller; never claimed Observed/dead.
Gameplay: no new scenarios (no unknown bytes left; late game reached in r001-a7k2).
Status: incomplete (commentary coverage of new regions and final editorial audit pending).
Toolkit: no new toolkit issues found; see analysis/toolkit-issues.md (TI-001..TI-005).
