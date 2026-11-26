# Redirect r2-q1 (round 2) - classification-quality review and commentary questions

Input checkpoint: pass-002-final (maps/pass-002-final.json, byte_identical; 100.00% classified, 0 unknown).
Map/state/source/ROM/toolkit untouched. Server: `run.py serve` via analysis/tools/rpcd.py daemon with
ROMATLAS_SCRATCH=/tmp/ra-2707-r2q1; stopped by its own PID at the end.

## Tools (this dir)
- xref.py -> xref-lowconf.json: for every Code region with confidence < 0.75 (56 regions, 878 B), all static
  references landing inside: absolute/indexed operands of all classified instructions (bank-aware; bank-7 code
  referencing $8000-$BFFF tried in every bank and marked "guess"), branch targets, lda/ldx/ldy #imm pairs within
  4 instructions (both byte orders), and every unaligned 16-bit word in Data regions.
- idxscan.py -> idxscan.json: indexed (abs,X / abs,Y) reads whose base lies in Data and whose 256-byte reach
  covers a low-confidence region; each consumer's index domain then checked by hand.
- datacheck.py -> datacheck.json: jsr/jmp/branch targets from classified code into Data/Padding, observed
  instruction starts (all saved observed-*.json) inside Data/Padding, and any operand into bank-3 $8A76-$8AFF.
- src.py: print commentary source around a physical offset (commentary/tools/linemap.json).
- run_q.py: bounded runtime experiments (credits, base, names, expr, unlock); outputs below.
- review-families.json: per-family verdicts with the decisive evidence.

## Static review results (no classification correction found)
- xref: no data-reader operand (absolute or indexed base) lands inside any low-confidence code region. Hits are
  only (a) branches/jumps from inside the same fragments, (b) the expected calls ($FB38 jsr $9CF7; five bank-3
  jsr $F9A9), (c) coincidences: immediate pairs that are bank-2 message pointers (e.g. MSG_FOUL_NO_RAIL = $97B7 is
  read with bank 2 mapped by Far_RunMenu, not bank 3), random words inside graphics/bitmap data.
- idxscan: 16 indexed reads could in principle reach low-confidence code; all have bounded index domains:
  ScoreboardLayout tables $977D/$9790 (38 bytes, max value 6) -> $97A3 word table max offset 13 -> last byte $97B0,
  directly before $97B1; PaletteTable $D97F (7 words; bank-6 script 'pal' args 0..6) ends 34 bytes before $D9B0;
  OpponentPortraitTable (11 entries by TournamentRound), PreviewPanelColors, GameTypeNameScripts,
  RandomMessageTable (index &3), Reset signature (x 0..4) all stop far short.
- datacheck: 0 control-flow targets into Data/Padding, 0 observed instruction starts in Data/Padding, 0 operands
  into $8A76-$8AFF. Linear decode of the 40 Data regions (<4 KiB) in banks 2/3/5/7: only $9790 decodes without
  unofficial opcodes and it is a consumer-anchored GameType table.
- Stack contract at $DE1C: VramBufFlushNow ($DE1D) pushes two bytes and pulls three; both branch-in users pha
  first, so a direct-call entry needs one preceding pha - exactly $DE1C. Independent support for that fragment.
- $8A76-$8AFF: 138 bytes = header (16, 8) + 128 tiles + 8 attribute bytes (4x2 attribute cells for 16x8 tiles):
  well-formed tile-rectangle shape, still unreferenced. Keep generic data.
- Metadata staleness (not structural): region 989 $F9A9-$FA3E (DrawBigNumber) has 5 static callers and 74/77
  starts observed in observed-intermission-artificial.json, but evidence says "without a known caller";
  regions 974/1112 bank-5 $8A3D-$8A48 are the fall-through of AimAtCalledBall after a returning
  jsr SelectBank5_Temp at $8A3A (region 436, 0.9), so statically reached, but evidence says no caller.
- (c) Observed provenance: all 558 Observed code regions (23,556 B) have every instruction start in the saved
  traces. 31 regions are observed only in artificial-state traces and all say so; wording nit: S14 evidence says
  "then natural interview/menus", which describes unpatched execution after an artificial shortcut, not natural
  reachability; oppmsg evidence says "natural reachability is likely" (judgement, labelled artificial).

## Runtime results (bounded; scratch /tmp/ra-2707-r2q1, removed)
- base: emulator.start, 1340 frames (snapshot boot1340), S8 route to manual-break aiming (snapshot aim0). No edits.
- credits (credits-entry-state.json; natural boot, no edits): Instruction trace limited to one byte at bank-3
  $830E / $831A, then cycle-stepped to the entry. $830E entered at frame 532 with Tmp08=0, DstPtr=$0000;
  $831A at frame 790 with Tmp08=0, DstPtr=$CD43. Both credits scripts (bank 2 $86CF, $8776) start, after the
  header and position op, with menu opcode $07 which itself sets DstPtr/MenuTileMap = $CD43 (NameCharMap) and
  MenuTileOffset = $80; so the entry values are irrelevant: credits use NameCharMap with tile offset $80
  (page 2's DstPtr=$CD43 is the leftover of page 1's op 07).
- names (names-reads.json): ROM Read trace limited to bank-7 $C86D-$D0C3 (no code in range, so every event is a
  data read); boot1340 -> aim0 menus (natural, frames 1340-2740) then the S14 ending shortcut (artificial,
  to frame 3906). 1,871 reads at 290 offsets. NameCharMap ($CD43) indices observed: $20-$6A (entries
  $CD63-$CDAD; $CD63-$CD65 and $CD84-$CD9D dominate = space/punctuation and A-Z). Reads in $CD2F-$CD62 follow
  the letter-0..25 pattern of BigFontTop/BigFontBottom (26 bytes each). So the 891-byte "BigFontBottom" block
  starts with the real 26-byte BigFontBottom ($CD49-$CD62) and NameCharMap's used body begins exactly at $CD63.
  Other reads: AsciiTileTable-$20 indices $20..$4E1 i.e. the 1218-byte block also holds more tables
  (runs $C86D-$C8A6 text map, $CC00-$CC2E, $CCAF-$CD35). WindowFrameTable record 0 (bank 3 $8000, +2/+3) holds
  the pointer $CD43: NameCharMap is the character map of window frame style 0 (RunMenu also loads #$43/#$CD
  literally into DstPtr on its full-screen paths).
- expr (expr-ram.json): the $DD=2 MatchExitCode shortcut leaves the match without EndOfMatch (screens: FLIP FOR
  BREAK next match / ELIMINATED), so no portrait expression capture. Static answer instead: PortraitFrame 1 is
  set before "YOU WIN THE MATCH. YOUR OPPONENT STALKS OFF" and PickRandomComment's "YOU HAVE WON THE GAME.
  YOUR OPPONENT CONGRATULATES YOU / THINKS YOU ARE LUCKY / VOWS REVENGE" (opponent lost); PortraitFrame 2 before
  "YOUR OPPONENT RAN THE TABLE" (twice) and "YOUR OPPONENT WINS THE MATCH" ($9CE4) (opponent won). So 1 =
  opponent-loses reaction, 2 = opponent-wins reaction; the pictures themselves are not visually confirmed.
  Also: RandomMessageTable entry 0 ("SO, YOU THINK YOU'RE GOOD...") is never chosen by PickRandomComment
  (index = Random & 3, 0 replaced by 1).
- unlock (unlock-ram.json, unlock-worldchamp-welcome.png; ARTIFICIAL $EE=1 written at the mode menu): Tournament
  start shows WELCOME regional, then TournamentRound=5 and "WELCOME TO THE BCA WORLD CHAMPIONSHIP TOURNAMENT"
  - confirms the WorldChampUnlock consumer in StartTournament. Natural Speed Pool clear not attempted
  (needs clearing a rack in < 141 time units; manual shot play is impractical here).

## Other commentary findings (static)
- DelayThenEnding/WaitFramesThenEnding/EndingScreenLoop are misnamed: WaitFramesThenEnding runs GameFrame for
  EndDelay frames, then jsr SelectBank3 / jmp $B9AB which is a bare RTS - returning to the caller of
  DelayThenEnding (ShowMessage uses it with 120 frames when a menu is open). Nothing "ending" happens. The 21 bytes
  after that RTS ($B9AC-$B9C0, region 972) are a disabled 16-bit WaitForNmi countdown (dead code, Static 0.7).
- RunMenu $2000/$2001 reads: on hardware these return the PPU I/O latch (last value written to any PPU register
  or read via $2002/$2007), not CPU open bus; value depends on preceding PPU traffic. Not traced (commentary only).
- Scoreboard wheel meaning: not investigated (needs a natural intermission; no structural impact).

## Deferred / not pursued
- Natural opponent-message and intermission routes (reachability only): need a natural lost rack/won game; the
  manual shot contract requires calls in 8-ball; low value now that classification is complete.
- Visual check of portrait frames 1/2: needs a natural game win/loss or an EndOfMatch entry with an
  understood contract.
