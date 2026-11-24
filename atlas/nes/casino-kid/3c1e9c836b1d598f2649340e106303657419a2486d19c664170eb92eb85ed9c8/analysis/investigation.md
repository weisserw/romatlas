# Investigation ledger

## Round 1 disassembly (2026-11-23)

Method summary (reproducible from workspace tools, all local):
- tools/walk.py: conservative static walker over the iNES image (UxROM: 16 KiB banks 0-6
  switchable at $8000, bank 7 fixed at $C000). Roots: vectors (NMI $E5FA, RESET $C000,
  IRQ $E608), fixed API jump table $C019-$C112 (JMP stubs), pushed-return targets
  ($E63F, bank 6 $A15D/$A366), observed opcode offsets, and inline jump-table entries.
  Follows branches, JMP abs, JSR targets and JSR fallthrough; stops at RTS/RTI/JMP (ind),
  BRK/unofficial opcodes (none hit after contracts below), window crossings, overlaps.
  $8000 targets from fixed-bank code resolve only through `lda #n / sta $14 / jsr $DEC7`
  (DEC7 writes $14&7 to the bus-conflict table at $DED4,x = UxROM bank select).
- Callee contracts (JSR fallthrough rules):
  - return-address consumers (pla before push / tsx) stop the fallthrough, except recognised
    "pla;pla;...;clc;adc #N;...;pha;pha" idiom -> N inline bytes;
  - $D608 / $D64A: 3 inline bytes (count, pointer) - confirmed by 18 observed call sites
    (instruction after the 3 bytes executed, inline bytes never fetched);
  - routines beginning `jsr $D806`: 5 inline bytes ($D806 adds 5 to the outer return address);
  - bank 3 $90F2 (2), $9110 (4); bank 4 $965E/$95D7/$969C/$9626/$8B1C (2);
  - $E463 (via $C06A): inline word table indexed by $1D, handler RTS returns to caller's caller;
    extent anchored where the first entry targets the byte after the table (or $E63F pushed
    as return address at $E61C-$E621 for the NMI table at $E625);
  - $E47B (via $C06D): same, but table length = immediate stored to $19 before the call; the
    handler returns to the table end (pushed pointer + 2*$19). Counts: bank 3 $8DD9 (9),
    bank 6 $9122 (4), fixed $ECB6 (36).
- Observed coverage: traced boot/title/intro/casino, overworld walking + talks, password
  screen, debug KING path, poker vs King, ending (traces/coverage.json; scenarios S1-S7).
  Before this replay, observed opcode starts were all inside static instructions (0 conflicts).
- Data: every remaining gap was inspected with (a) code-likeness scan (official-opcode runs
  ending RTS/RTI/JMP with calls into established routines: only $E557/$E573 hit, now code),
  (b) count of JSR/JMP bytes targeting established fixed-bank routine entries inside data
  (1 random hit in 10.5 KiB of bank 0, none elsewhere; code in banks 3-6 calls the $C0xx API
  constantly), (c) unofficial-opcode density (banks 0-2 ~36%), (d) observed ROM reads in
  banks 0 and 2 during dialogue loading. Remaining gaps classified generic Data (Static).
- Specific non-code structures: iNES header, CPU vectors, vector copies at $BFFA in banks 0-6
  ($C010/$C013/$C016 -> JMP stubs), ASCII title "CASINO KID" at $FFE0, uniform zero runs,
  inline parameter bytes, inline jump tables, bank-select table $DED4 (part of generic data).

Rejected/limited attempts:
- Toolkit analysis.discover_context: stops at every JSR on mapper 2 (TK-002); not used for classification.
- Artificial PC=$C185 at the frame wait (bank 4 game entry) showed a table screen without
  opponent setup and stalled: rejected as a route (missing opponent-setup contract).
- Walking/talking to find card opponents: two NPCs were non-opponents; deferred (lead L1).

Remaining uncertainties (not unknown bytes, but weaker claims):
- 569 Static code regions are inferred by the walker; misapplied inline-skip contracts would
  misplace small Data/Code splits (lead L3). Unreferenced code fragments listed in lead L7.
- Generic data meaning (banks 0-2 graphics, fixed-bank tables, music data in bank 6) unknown.

## Round 1 redirect (attempt r1-a1, 2026-11-24; checkpoint pass-002, no map change)

Full record: analysis/redirect/r1-a1/notes.md (sections A-K), replay logs rr.log, dbg.log,
pk.log, pwchain.log; coverage-all.json; handoff analysis/handoffs/round-1-redirect.md.

- L1 resolved: opponent selection is the floor talk check (seat table $98C5 / people ids
  $98E3; ids $18-$1F blackjack). Position patch ($0559/$055A/$AE) next to seat $18 reached
  blackjack; 13 traced rounds incl. dealer-broke exit and progress recording. Supersedes the
  rejected PC=$C185 attempt (that one lacked the encounter/opponent setup; the new method uses
  the game's own talk -> encounter -> SelectOpponentLevel -> RunBlackjack path).
- Runtime confirmation: 4438 new opcode starts (bank 3 1659 of 1862 static starts), 0 off
  map instruction boundaries, 0 in non-Code; 276 inline-callee sites return at S+3+N (L3).
- Boundary audit: the only inconsistency in the whole map is $CFF6 (L8, 1 byte -> Code).
- Data structure tiling with exact extent proofs: cheat/preset passwords (bank 5 $93DF,
  bank 6 $87DA), poker tables (bank 4 $9F06-$BABF), blackjack tables (bank 3 $913A-$9B87)
  (L13, L14). Poker category 8 = "ROYAL STRAIGHT" (L9: not a bug).
- Gameplay visited: blackjack, poker vs regular gambler JOSEPH, debug monitor (memory
  viewer/editor, music 1-10, SFX 0-35), GAME OVER via BANKRUPT password, status-screen password
  encode + SAVED GAME decode round trip (scenarios S8, S10-S13).
- Negative/limited: FINAL HAND option not completed (timing); L4/L7 still no references;
  traced emulation ~10x slower than untraced (TK-004).

## Round 2 disassembly (pass-003)

- L8: $CFF6 decoded independently (1-byte PHA; only control-flow targets outside Code in the map
  were JMP/BNE $CFF6 per targetaudit). Annotated Code/Static by extending the mkmap EXTRA_CODE
  fragment to $CFE6-$CFF6. Reachability of the fragment remains unresolved (no caller).
- L2: offline re-check of redirect coverage against pass-002 instruction starts: 11050 starts,
  0 off boundaries, 0 in non-Code. Merged into traces/coverage.json (backup
  traces/coverage.pre-round2.json). Walker roots from new coverage did not change any boundary.
- L13/L14: re-checked extents with direct operand scan (lda $913A/$913B,y; $91D4,y; $9F57,x;
  $A064,x) and the MulMulti/AddMulti base immediates in BjSetup/LoadOpponentRecord/PrintHandName/
  ShowPokerMessage; pointer tables' min targets / max $FF string ends tile exactly. Hand-name
  strings end at $A408. Spans $9177-$91D3 (bank 3), $9F72-$A037 and $A408-$A596 (bank 4) are
  $FE/$FF-controlled text without an identified consumer: unlabelled Data.
- Toolkit note (not a bug): data regions reject execution views ("execution views belong to code
  regions only"); data labels are carried without views.
