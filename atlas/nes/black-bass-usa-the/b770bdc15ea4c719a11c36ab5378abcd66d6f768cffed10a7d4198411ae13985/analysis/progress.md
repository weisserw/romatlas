# Disassembly progress

## Coordinator log (goal.md orchestration)

ROM: UxROM (mapper 2), 128 KiB PRG (8 x 16 KiB banks), CHR-RAM, vertical mirroring.

| Round | Phase | Status | Input checkpoint | Output checkpoint | Handoff | Notes |
|---|---|---|---|---|---|---|
| 1 | disassembly | done (easy-gain exhaustion) | none (fresh) | pass-001 (byte-identical) | analysis/handoffs/round-001-disassembly.md | 98.90% (129652/131088; 1436 unknown, all bank 7); leads L-001..L-005 produced |
| 1 | commentary | done (all classified source covered) | pass-001 | commentary-001 (byte-identical; commentary/source/game.asm verified) | analysis/handoffs/round-001-commentary.md | 271 labels; leads L-006, L-007 produced; L-001/2/4 updated; final editorial audit reserved |
| 1 | redirect | done (attempt r001-a7k2; non-terminal) | commentary-001 | (no map change) | analysis/handoffs/round-001-redirect.md | ready leads L-008 (+62), L-001 (+1360), L-002 (+8, 6 weak optional), L-006 (labels only); L-007 verified, L-004 partly; late game/ending reached |
| 2 | disassembly | done | commentary-001 | pass-002 (byte-identical) | analysis/handoffs/round-002-disassembly.md | 100.00% (131088/131088; 0 unknown); L-008, L-001, L-002 (6 weak bytes as code at conf 0.6-0.7), L-006 accepted; 81 new/changed regions need commentary |
| 2 | commentary | done (all classified source covered; editorial_audit pending) | pass-002 | commentary-002 (byte-identical) | analysis/handoffs/round-002-commentary.md | round-1 commentary preserved in analysis/round-1/; L-009 produced (readability only) |
| 2 | redirect | done (attempt r002-q5m8) | commentary-002 | (no map change) | analysis/handoffs/round-002-redirect.md | no ready leads; weak bytes upheld; L-009 answered; readability findings R-01..R-14; recommends final acceptance |
| 2 | final acceptance | done | commentary-002 | final-001 (byte-identical; map validates) | analysis/handoffs/final-acceptance.md | R-01..R-14 applied; ~330 generic labels renamed; commentary.py verify + audit pass; editorial_audit=reviewed; TI-006 logged |

Current: TERMINAL HANDOFF (orchestration complete, 2026-11-20).

### Terminal coordinator summary
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
  Lowest-confidence claims retained with evidence: $C040 (code, 0.7) and lone rts at $D76C/$E24A/$E30F (code, 0.6); 1296 B bank-7 static code with no known caller (reachability unknown, not claimed dead).
- Final map checkpoint: final-001 (maps/final-001.json, analysis/final-001-verification.json: byte_identical=true).
- Commentary: commentary/source/game.asm verified byte-identical; commentary.py audit passed; editorial audit record commentary/editorial-audit.md; residual semantic uncertainties in commentary/uncertainties.md.
- Leads: analysis/leads.json (no open ready leads). Gameplay coverage: analysis/scenarios.json (natural unaided catch/ceremony S-017 deferred with reason).
- Toolkit issues: analysis/toolkit-issues.md (TI-001..TI-006; toolkit not modified).
- Archive document selection: archive-options.json (no archive executed; not authorized here).


## Round 1 — disassembly pass (2026-11-20)

ROM classification: 98.90% (129652 / 131088 bytes classified; 1436 bytes unknown).

Checkpoint: pass-001 (maps/pass-001.json, generated/pass-001,
analysis/pass-001-verification.json) — map.validate ok, generate ok, verify
byte_identical=true. Baseline: maps/baseline.json / generated/baseline verified (raw only).
Internal store: analysis/state.json (686 regions); export: maps/current.json.

| Bank (16K) | Code observed | Code static | Data | Unknown |
|---|---|---|---|---|
| header | 0 | 0 | 16 | 0 |
| 0 (sound engine + lake map data) | 965 | 227 | 15192 | 0 |
| 1 (San Lake data) | 0 | 0 | 16384 | 0 |
| 2 (lake data) | 0 | 0 | 16384 | 0 |
| 3 (lake data) | 0 | 0 | 16384 | 0 |
| 4 (Lake Amada data) | 0 | 0 | 16384 | 0 |
| 5 (tile data) | 0 | 0 | 16384 | 0 |
| 6 (game logic / screens / text) | 3422 | 2285 | 10677 | 0 |
| 7 fixed $C000 | 6711 | 4601 | 3636 | 1436 |

Evidence: observed opcode fetches (5114 unique ROM offsets) from scenarios S-001..S-009
(analysis/scenarios.json); static recursive descent with explicit bank contracts; inline-arg
and display-list consumer contracts; jump tables; observed data reads in banks 1 and 5;
uniform fills and ASCII signature text. Details: analysis/investigation.md.
Execution coverage is separate: 11098 code bytes observed, 7113 static-only.

Leads produced: L-001 (unreferenced fixed-bank routines, 1436 B), L-002 (stray bytes),
L-003 (data semantics), L-004 (gameplay milestones: fish catch, password, ceremony, ending),
L-005 (static-only code observation). Status: incomplete; pass ended at easy-gain exhaustion.
Toolkit: 5 findings (2 confirmed bugs, 2 missing features, 1 suspected doc gap) — see
analysis/toolkit-issues.md. Handoff: analysis/handoffs/round-001-disassembly.md.

## Round 1 — commentary (2026-11-20)

Input: pass-001 (byte-identical).  Output checkpoint: commentary-001
(maps/commentary-001.json, generated/commentary-001, analysis/commentary-001-verification.json:
map.validate ok, generate ok, verify byte_identical=true).  Commented source:
commentary/source/game.asm, verified with scripts/commentary.py verify (byte_identical=true,
commentary/verification.json); reading copy commentary/reading-after.asm refreshed.

Classification unchanged: 129652/131088 classified, 1436 unknown (bank 7); the map changed
only by 271 region labels (routine/data names at region starts) and a comment suffix.

Scope covered: ALL classified source — 8256 instructions in banks 0/6/7, all data blocks
(banks 0-7 + header) and notes for all 24 unknown blocks.  ~409 global routine/data names,
~300 RAM symbols, hardware/bank/object/button constants, block headers for essentially every
routine, inline comments on the complex routines, inline arguments and display lists
re-emitted as .word/.byte with text-derived names (DL_*, Txt_*).  review.json: 711 scaffold
entries reviewed, 944 units; editorial_audit left pending (final acceptance).

Readability: good for engine/library, NMI/VRAM queue, sound, menus/screens, standing code,
records and the fishing state machines; fight/fish-AI internals are described mechanically
with tentative names.  Weak spots: auto local branch labels (@skip/@loop) in long routines,
numeric song/sfx ids, PPU address immediates in screen code, large generic data blocks.

Findings: debug name "HBMAX " (DebugHud); standing-code format decoded; SessionEvent 1 =
lure lost; bank 5 is mapped during fishing and holds fish/lure tables (L-006); strike
conditions documented (L-004 notes); unknown fixed-bank blocks look like unused library
helpers (L-001 notes).  Leads produced: L-006, L-007; updated: L-001, L-002, L-004.
Toolkit: no new toolkit issues found this phase (see analysis/toolkit-issues.md).
Handoff: analysis/handoffs/round-001-commentary.md.

## Round 1 — redirect (attempt r001-a7k2, 2026-11-20)

Input checkpoint: commentary-001 (maps/commentary-001.json, byte-identical; classification =
pass-001: 129652/131088, 1436 unknown in bank 7). No map/source/state changes (research only).
Records: analysis/redirect/r001-a7k2/ (bank7-unknown-ledger.md, bank5-tables.txt,
observed-static.txt, tools/). Handoff: analysis/handoffs/round-001-redirect.md.

Methods: bank-7 linear decode + contract review of all 24 unknown blocks; reference search
(jsr/jmp, LE words, split immediates, rts-trick words); bounded table index-domain proofs;
offline standing-code model (validated on the game's own code); runtime (run.py serve,
untraced navigation, capacity-0 traced segments) with RAM aids for strike/fight/ranking.

Outcomes:
- New consumer reference: $C38E/$C39B Expand1bppList lists (62 B) -> L-008.
- Proposed structure for the rest: 1296 B Static code (library routines, no known caller),
  64 B bounded tables, 8 B generic data ($CD strays); 6 B weak residue ($C040, 3 lone rts).
- Gameplay: valid generated code (Day 11) -> day 12 -> catch (strike/fight/landing with aids)
  -> record -> ending text "THE FISHING TOURNAMENT OF THIS YEAR IS ALL OVER" -> title;
  winner ceremony scene with RankPoints=1. Traced segments fetched 1426 instruction starts in
  previously static-only code (artificial state); no fetch in unknown bytes.
- Bank-5 table extents for L-006 (fish tile sets chain $8FC5-$A9B4).
Leads consumed/updated: L-001, L-002 (ready), L-006 (ready), L-007 (verified), L-004 (partially
verified), L-005 (notes). Produced: L-008 (ready). Ready for next disassembly: L-008, L-001,
L-002, L-006 (expected up to +1436 B -> 100% if all accepted; >=1430 B without weak residue).
Recommendation: another disassembly/commentary cycle. Toolkit: no new toolkit issues found
(TI-004 trace slowdown reconfirmed: ~4 fps traced).

## Round 2 — disassembly pass (2026-11-20)

Input: commentary-001 (byte-identical). Internal store restored with map.load (no reinit);
all existing regions, labels and commentary metadata preserved (region 42 TileDataBank5
narrowed to $8000-$8207 with its label kept). Output checkpoint: pass-002
(maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json):
map.validate ok, generate ok, verify byte_identical=true (map sha256 69249d40...0550b4).
Internal store: analysis/state.json (767 regions); export: maps/current.json.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Bank (16K) | Code observed | Code static | Data | Unknown |
|---|---|---|---|---|
| header | 0 | 0 | 16 | 0 |
| 0 | 965 | 227 | 15192 | 0 |
| 1 | 0 | 0 | 16384 | 0 |
| 2 | 0 | 0 | 16384 | 0 |
| 3 | 0 | 0 | 16384 | 0 |
| 4 | 0 | 0 | 16384 | 0 |
| 5 (split into 31 regions) | 0 | 0 | 16384 | 0 |
| 6 | 3422 | 2285 | 10677 | 0 |
| 7 fixed $C000 | 6711 | 5903 | 3770 | 0 |

Change vs pass-001: bank 7 +1302 Static code (1296 L-001 library routines, 3 $C040, 3 lone
rts), +134 data (62 L-008 lists, 64 L-001 tables, 8 $CD strays). Execution coverage
unchanged (no new runs): 11098 code bytes observed; new code is Static, "no known caller".

Lead outcomes: L-008 consumed/accepted (+62). L-001 consumed/accepted (+1360).
L-002 consumed/accepted (+8 generic data; weak residue accepted on evidence: $C040 code
conf 0.7, $D76C/$E24A/$E30F rts conf 0.6). L-006 consumed/accepted (bank-5 split, 0 byte
change; extra extents from Expand1bppList/CopyTilesInline arguments). None rejected.
Verification per lead: analysis/investigation.md (Round 2 section).

Gameplay: no new scenarios run. Justification: no unknown bytes remained after the leads, and
the late game/ending was already reached in redirect (S-012..S-015); new runs would only
change execution coverage, not classification.

Status: incomplete — classification is complete, but the 81 new/changed regions (50 bank 7, 31 bank 5) lack
commentary-phase source coverage, and the final editorial audit is pending. Weakest claims
for review: $C040 and the three lone rts bytes; generic bank-5 remainders.
Toolkit: no new toolkit issues found (existing TI-001..TI-005 unchanged; see
analysis/toolkit-issues.md). map.annotate requires an explicit `id` (0 for new regions)
inside `region`, as documented in nes.md. Handoff: analysis/handoffs/round-002-disassembly.md.

## Round 2 — commentary (2026-11-20)

Input: pass-002 (map changed vs the round-1 commentary baseline: 81 new/changed regions).
Previous commentary preserved intact under analysis/round-1/commentary/ (moved, not
copied over); fresh baseline prepared with `commentary.py prepare` (before/ from pass-002,
byte-identical). Names/comments/review carried by physical address through
commentary/tools/annotate.py + spec.txt; only the 22 notes for formerly-unknown blocks
and the single-block bank-5 note were invalidated and rewritten.

Output checkpoint: commentary-002 (maps/commentary-002.json, generated/commentary-002,
analysis/commentary-002-verification.json: validate ok, generate ok, byte_identical=true,
map sha256 530ccd61...87). Map vs pass-002: classification identical (767 regions, 0
structural diffs); +16 region-start labels from commentary names (FishK0-3Frames,
FishSharedSprites, LureFrames, LureMotionTables, SplashFrames, TileMapAB35, DataB858,
Exp1bppUploadTiles/BuildTile/CopyPlane/ZeroPlane, PaletteBrighten/Darken).
Commented source: commentary/source/game.asm, `commentary.py verify` byte_identical=true
(commentary/verification.json); reading copy commentary/reading-after.asm refreshed.

ROM classification unchanged: 100.00% (131088/131088). Source readability: ALL classified
source covered (no remaining ranges). review.json 767/767 entries reviewed, 1013 units;
editorial_audit pending (final acceptance).

Covered: all 50 new bank-7 regions (named routines with headers, branch labels, RAM/
constant symbols; Static/no-known-caller wording, never dead/unused), 31 bank-5 regions
(notes + structured pointer/record tables), backlog: MUS_*/SFX_* constants at all sound
call sites, ~300 generic branch labels renamed in the 32 longest routines, BTN masks,
pointer immediates; fixed round-1 review.json source-line drift (tool bug in line
counting).

Findings: FishSharedSprites ($AC4D-$AE72) = fish sprite lists shared by all kinds; the
DrawObject probe branch is reachable only via DrawObjectFlagged; tile records for size
classes 4/0 of kinds 1-3 are empty; MMC1 library init details. Lead L-009 added (bank-5
semantic residue, readability only). Toolkit: no new toolkit issues found (TI-001..TI-005
unchanged; analysis/toolkit-issues.md). Handoff: analysis/handoffs/round-002-commentary.md.

## Round 2 — redirect (2026-11-20, attempt r002-q5m8)

Input: commentary-002 (131088/131088 classified, 0 unknown). Research/lead records only.
Records: analysis/redirect/r002-q5m8/. Handoff: analysis/handoffs/round-002-redirect.md.
- Weak claims ($C040 jmp ($FFFC); lone rts $D76C/$E24A/$E30F; no-caller library code)
  re-examined: no contrary evidence, keep as code. Jump-slot group alignment; 13 of 16
  terminal-following rts are live epilogues.
- L-009 answered: DataB858 = cast animation + aim cursor sprite data (PowerCursorObj/
  CastAnimObj records); TileMapAB35 = duplicate of LandingBoxTiles. Readability only.
- Ranking/ceremony rules decoded; two reversed comments found; prize texts captured
  (winner, 3RD, 5TH). Song 1 = silence. CastPhase 5 = bounce.
- Gameplay: Lake More, Japan Lake (generated codes), LOOK AT MY RECORD, HBMAX HUD
  visited (S-015/S-016 updated; S-017 natural catch deferred).
Leads: none ready; updated L-002, L-004, L-005, L-009. 14 readability findings for the
final editorial pass. Recommendation: proceed to final acceptance. Toolkit: no new issues.

## Final acceptance — editorial audit (2026-11-20)

Input: commentary-002.  Output checkpoint: final-001 (maps/final-001.json,
generated/final-001, analysis/final-001-verification.json: validate ok, byte_identical=true,
map sha256 1ea41d786c8ce22da17d3892acd1959f9a688c35c54087aed143120fedfd435d).
Classification is unchanged: 767 regions with identical offset, length, kind, provenance
and confidence; 131088/131088 bytes (Code 19513, Data 111575, Unknown 0).  Map change:
42 region-start labels only.  map.annotate dropped comment/evidence on a label-only edit
(TI-006); this was restored and redone with explicit fields, and a diff confirmed only
labels changed.
- Applied R-01..R-14.  The reversed PlayerBeatsRivals/CompareTotals logic was fixed
  (PlayerBehindRivals), RankPoints is now ParticipantRank, and the sprite order is
  attributes, dX, dY, tile.  Renamed: CastAnimFrames/PowerCursorFrames (was DataB858),
  LandingBoxTilesCopy (was TileMapAB35), MUS_STOP and CastPhase5Bounce.  The DrawGreeting
  order is fixed.  The $C040/lone-rts notes and uncertainties.md are updated.
- Applied the known editorial issues: about 330 generic labels renamed (56 short-routine
  @loop remain); NT_ADDR name-table constants; character literals; symbolic
  CopyTilesInline sources with their banks; address-based data names replaced; MUS_/SFX_
  prose; headers for 50 routines and notes for about 70 data tables; review summaries
  regenerated.
- New consistency fixes: SndEnvelope1-9 are renumbered to their SndEnvelopeTable entries,
  and the lake map display lists (DL_LakeMap0-3) now carry picture, palette, attribute and
  terrain labels.
- Verification: `commentary.py verify` byte_identical=true.  `commentary.py audit` ok
  (767 regions, 1048 units, editorial_audit reviewed).  The reading copy is refreshed.
- L-009 closed (readability applied).  Toolkit: TI-006 added
  (analysis/toolkit-issues.md).  Report: commentary/editorial-audit.md.  Handoff:
  analysis/handoffs/final-acceptance.md.

## Archive repair (2026-11-20)
113 quoted `.byte "..."` operands in commentary/source/game.asm respelled as equivalent `$XX` lists
so archive.py can reference them from the baseline; byte-identical rebuild verified; probe archive ok.
Details: commentary/archive-repair.md. Backup: analysis/archive-repair-backups/.
