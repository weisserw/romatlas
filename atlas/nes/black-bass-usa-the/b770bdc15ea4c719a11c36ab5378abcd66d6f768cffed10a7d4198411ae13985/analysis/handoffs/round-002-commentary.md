# Round 2 commentary handoff

Status: complete for the current classified source (not final editorial acceptance).

Checkpoint: commentary-002 — maps/commentary-002.json, generated/commentary-002,
analysis/commentary-002-verification.json (validate ok, byte_identical=true, map sha256
530ccd61edff7d63433c8d8e32505b89e695d9d9b1b2104cc086051abd5bb787).
Commented source: commentary/source/game.asm — `commentary.py verify` byte_identical=true
(commentary/verification.json); reading copy commentary/reading-after.asm refreshed.
Map vs pass-002: identical classification (767 regions); +16 region-start labels only.
Previous commentary: analysis/round-1/commentary/ (complete, untouched).

Scope: ALL classified source (ROM 100% classified). New regions: 50 bank-7 (library
routines with no known caller, tables $F2F8/$F3BB/$F4B5, Expand1bppList record lists
$C38E/$C39B, $CD filler, $C040 and three lone rts) and 31 bank-5 regions. Backlog:
song/effect constants, branch labels in the 32 routines that had >= 5 generic labels,
BTN masks, pointer immediates, wording ("unused" -> "no writer found"), ScoreRetrieveActions
$FF comment corrected (entries are skipped, not terminators). Remaining ranges: none.

Readability: new library routines have purpose headers, named branches, symbolic work
bytes and explicit "Static code, no known caller" (reachability unknown); bank 5 is now
navigable (pointer tables resolve to labels; record tables as rows). Engine/game code
quality as in round 1. review.json 767/767 reviewed, 1013 units; editorial_audit pending.

Method/tools: commentary/tools/annotate.py (rebuilds source/ from before/ + spec.txt,
byte-neutral; ADDRDUMP=<file> dumps address/label listing), build_review.py. Tool fixes:
"; round-N" comment parsing, correct 1-based source lines (round-1 review.json locations
had drifted after the first multi-line display list), repeatable spec-local labels.

Findings: FishSharedSprites $AC4D-$AE72 holds DrawObject sprite lists used by all four
fish kinds (pointer walk of the frame tables); ProbeMode is set only by DrawObjectFlagged,
so DrawObject/PlaceSprite probe branches are reachable only through that no-caller entry;
FishTileSetRecords give kinds 1-3 empty records for size classes 4 and 0 (kind 3 class 0
would read tile data); Mmc1Init writes CHR0 twice and its inc $FFF9 reset idiom would not
set bit 7 with this ROM's $00 byte; WaitSecondsTimer's counter is never decremented here.

Unresolved questions: consumers of TileMapAB35 ($AB35) and DataB858 ($B858); per-frame
extents in the frame blocks and the last LureMotionTables extent; identity of songs 1/
effects 1 (MUS_SCREEN_END/SFX_QUIET are guesses); on-screen meaning of retrieve actions
and fight reactions (round-1 items in commentary/uncertainties.md).

Known editorial issues for final acceptance:
- ~490 generic @skip/@loop/@return labels remain in shorter routines (<= 4 per routine,
  e.g. MoveFish, ShowDepthValue, DecodeMapRow, NmiAttrRowUpload, DrawHudMeter).
- Numeric PPU-address immediates in screen/HUD code (e.g. $2428, $2105, $2124 targets)
  and character constants (#$5A etc.) in name/code entry are not symbolized.
- CopyTilesInline inline argument words print raw source addresses ($8208, $AB5D, ...)
  instead of bank-5 labels; bank context is per call site.
- Address-based data names kept where roles are unresolved (Tiles1bpp_*/Tiles2bpp_*,
  TileMapAB35, DataB858) and large generic blocks in banks 0-4 and 6 stay single incbins.
- Some prose still cites sound numbers ("sound 7", "effect 5") next to the new constants.
- review.json unit summaries are generated from block headers (truncated at 400 chars).

Leads: L-009 added (bank-5 semantic residue; readability only, 0 bytes). No structural
redirect questions arose; weakest structural claims remain $C040 and the lone rts bytes.
Toolkit: no new toolkit issues found (analysis/toolkit-issues.md unchanged).
Servers stopped; scratch cleaned.
