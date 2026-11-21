# Round 1 disassembly handoff

## Scope
Fresh workspace: baseline created and verified, then one easy-gain disassembly pass for
Break Time - The National Pool Tour (FCI 1992, MMC1, 128 KiB PRG, 32 KiB CHR). Methods were
observed code from traced debugger gameplay, a bounded static extension, structural data,
observed ROM data reads and an ASCII text scan. No RAM or register edits were used.

## Checkpoint
- pass-001: maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json.
  The checkpoint verified with ok=true and byte_identical=true.
- analysis/state.json is saved and maps/current.json is exported. map.validate returned ok.

ROM classification: 65.78% (107792 / 163856 bytes classified; 56064 bytes unknown).

The status is incomplete.

## Per-bank counts (code / data+image / text / padding / unknown)
header 0/16/0/0/0; prg00 5292/4/294/0/2602; prg01 0/2304/0/1716/4172; prg02 3/1747/0/0/6442;
prg03 53/1221/0/1351/5567; prg04 3/1216/0/0/6973; prg05 53/8/0/771/7360;
prg06 6570/0/148/0/1474; prg07 3216/93/0/771/4112; prg08 1723/0/241/4389/1839;
prg09 0/8/0/8184/0; prg10 5229/85/966/0/1912; prg11 6025/8/514/1378/267;
prg12 956/730/0/0/6506; prg13 0/29/0/1716/6447; prg14 3815/0/38/3948/391;
prg15 0/16/0/8176/0; chr 0/32768/0/0/0. The same table is in analysis/progress.md.

## Findings
- Bank model: reset goes to $8000 in bank 0, which runs `INC $FFF9 / JMP $C000`. Every even
  8 KiB bank starts with this stub followed by a JMP entry table. Fixed bank 14 sits at
  $C000. Contexts observed: 16 KiB fixed-last mode [x,x+1,14,15] for x = 0, 6, 8, 10, 12;
  32 KiB mode [0,1,2,3] during the title; [0,1,4,5] during the eight-ball title; [0,1,0,1].
- Code is 32,938 bytes: 18,986 Observed and 13,952 Static (closure rules are in
  analysis/investigation.md).
- Structural data: header, CHR (Image), vectors and the $FFF9 reset-trick byte, the
  odd-bank trailers, zero padding, and the bounded bank-0 dispatch table $82E4.
- 76 runs (7,227 bytes) are classified as Observed data from captured ROM reads, with
  dummy-read filtering. 140 ASCII text spans (2,201 bytes) are classified as Text.
- Controller routine is at $C4BD. Held buttons are at $56/$57 and new presses at $58/$59.
- Text includes the tour ending message (bank 0, physical about 6889) and staff credits
  (bank 10, physical about 87126-87725).

## Scenarios exercised
SC-001 to SC-007 and SC-011 are in analysis/scenarios.json:
- Visited: title, main menu, Nine Ball, Rotation, 14-1, and the Tour start in New York.
- Eight Ball was attempted (the rack screen was reached).
- Practice was attempted, but its coverage was lost when the server process hit its 2 h limit.
- WATCH mode and the later tour cities and ending were deferred.
- Routes, inputs and the coverage/read aggregates are in traces/round-1/ (tools in
  traces/round-1/tools; the scripts expect a scratch/ directory).

## Unresolved questions
- Formats and loaders for the graphics-like data in banks 2-5 and the sound-like data in
  banks 12-13.
- Bounds of the jump tables reached through JMP (ind).
- String pointer tables and terminators.
- Extents of the tables that the observed data-read runs come from.
- The rejected bank-0 static closure from $83A7.
- A possible uncalled code block at $82E8.

## Deferred work
- RAM shortcut to the later tour cities and the ending (L-003).
- Practice, Trick Shot and WATCH coverage (L-006).
- Data-format work in L-004, L-005, L-008 and L-009.

## Leads
- Consumed: none (no leads existed).
- Produced: L-001 to L-009 (analysis/leads.json).

## Toolkit issues
There are 3 missing-feature findings and no blockers (analysis/toolkit-issues.md):
- TK-001: Read trace events do not mark dummy reads.
- TK-002: Tracing slows execution about 10x and has no coverage-only mode.
- TK-003: Read streams overflow and there is no aggregated data-read API.

## Pass end
The pass ended at easy-gain exhaustion; it was not interrupted.
