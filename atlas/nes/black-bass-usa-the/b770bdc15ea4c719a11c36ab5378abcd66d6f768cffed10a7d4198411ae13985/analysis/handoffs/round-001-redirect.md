# Round 1 redirect handoff (attempt r001-a7k2)

Status: complete (phase finished; not terminal). Input checkpoint: commentary-001
(maps/commentary-001.json; classification = pass-001, 98.90%, 1436 unknown, all bank 7).
No map, source, state or ROM changes were made; research/lead/scenario records only.

## Scope
- Whole unknown complement (24 bank-7 blocks) re-decoded with contract and reference checks.
- Late-game route: standing-code generator, day 12, fish strike/fight/catch, ceremony, ending.
- Bank-5 fishing table extents (L-006).
Records: analysis/redirect/r001-a7k2/ (bank7-unknown-ledger.md, bank5-tables.txt,
observed-static.txt, tools/*.py). Lead file: analysis/leads.json; scenarios S-012..S-016.

## Findings
1. NEW REFERENCE: $C38E and $C39B are Expand1bppList record lists, pointed to by split
   immediates in classified code at $C1C2/$C24A; $FF terminators at $C39A/$C3CB end flush
   with the block (L-008, 62 B). Pass-001 searched LE words only.
2. All other unknown bytes resolve structurally (ledger table): 1296 B of complete
   rts-terminated library routines that call classified helpers and reuse live RAM
   protocols (e.g. $C37B = byte copy of UploadPalettes; $C8F0 = DrawObject alt entry;
   $F264 = UploadAttr sibling for $2B40; $F845 = MMC1-style BankSelect; $F7DB-$F842 MMC1
   init/serial writers), plus three bounded tables (64 B: $F2F8 32-entry, $F3BB 8-direction
   dx/dy, $F4B5 d-pad->direction using the same numbering). No caller exists for any code
   entry (jsr/jmp, LE words, split immediates, rts-trick words checked) -> Static code with
   "no known caller"; never Observed.
3. $CD stray bytes (8 B at $F333, $F6AD-$F6AF, $F6FD, $F754, $F843-$F844) recur only after
   terminal instructions; non-code -> generic data. Weak residue: $C040 jmp ($FFFC) (3 B),
   dead rts $D76C/$E24A/$E30F (3 B).
4. Standing code fully modelled (tools/standing_code.py, validated on the game's own code).
   Code LIUBBEPZECTRON4A (Day 11) accepted -> day 12 -> ending text "THE FISHING TOURNAMENT
   OF THIS YEAR IS ALL OVER" -> title. Fish strike, fight, landing and catch sheet reached
   with documented RAM aids; winner ceremony scene reached with RankPoints=1 (artificial).
5. Bank 5: fish tile sets chain contiguously $8FC5-$A9B4; pointer table extents listed (L-006).

## Ready leads for next disassembly (expected byte effect)
- L-008 (high): $C38E-$C3CB data lists, +62 B.
- L-001 (high): code/tables per ledger, +1360 B (1296 code, 64 table).
- L-002 (medium): CD bytes as generic data +8 B; optional $C040 code +3 B; dead rts +3 B only
  if reviewer accepts the epilogue argument.
- L-006 (low, 0 unknown bytes): split bank-5 data region into labelled tables.
If all accepted: 1436/1436 unknown bytes resolved (100% classification); otherwise at least
1430 B with the 6 weak bytes left unknown.

## Unresolved questions
- Natural reachability of the library routines (none found; probably linked but unused).
- $D76C/$E24A/$E30F single rts and $C040: code vs data inherently weak for 1-3 byte objects.
- Bank-5 frame-table/motion-script internals; $A9B5-$AB5C role.
- Prize text ("YOU ARE WINNER IN A CLASS"/"2ND PRIZE") not captured on screen.

## Deferred avenues (reasons)
- Lake More / Japan Lake, LOOK AT MY RECORD, HBMAX debug HUD, 2nd prize: same classified
  code paths; no connection to unknown bytes; cheap to add later from the S-012 route.
- Natural (unaided) bass catch / multi-day play: slow manual play; milestones already
  visited with aids; no classification payoff.
- Searching callers for library routines beyond the four reference methods: no further
  method (no jump tables/vectors in RAM point there; banks 1-4 hits are data coincidences).

## Runtime coverage
Traced artificial segments fetched 1426 instruction starts inside previously Static-only code
(analysis/redirect/r001-a7k2/observed-static.txt); none inside unknown bytes.

## Recommendation
Run another disassembly -> commentary cycle with L-008, L-001, L-002, L-006; then a short
redirect/final audit. Toolkit: TI-004 (trace slowdown) reconfirmed; no new toolkit issues found.
