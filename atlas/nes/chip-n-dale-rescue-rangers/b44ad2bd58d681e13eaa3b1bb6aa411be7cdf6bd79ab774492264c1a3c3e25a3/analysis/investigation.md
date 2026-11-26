# Investigation ledger

## Round 1 disassembly pass 1 (2026-11-26) - checkpoint pass-001-final

Game: Disney's Chip 'n Dale Rescue Rangers (Capcom), MMC1, 128 KiB PRG + 128 KiB CHR.
Evidence/tool copies: analysis/evidence/pass-001/ (coverage, contexts, edges, ROM-read
offsets, traversal output, scratch scripts in tools/). Snapshots: analysis/snapshots/.

### Method
1. Baseline (raw) created and verified (maps/baseline.json).
2. Observed coverage from traced scenarios S01-S08 (analysis/scenarios.json), exported
   from `analysis.coverage/contexts/edges` and merged offline (TK-002 forced frequent
   dump + `trace.clear`). All observed contexts are canonical: 16 KiB bank k at $8000 ->
   prg_banks [2k,2k+1,14,15]; fixed 16 KiB bank 7 at $C000 -> [-,-,14,15].
3. Local recursive descent (toolkit static discovery is unusable on MMC1, TK-003) from
   all observed opcodes plus verified handler tables (L-001, L-002): branch/JMP/JSR
   successors in canonical context, straight-line blocks accepted only if every
   instruction is an official opcode, no BRK, no overlap with other instruction
   boundaries; no return continuation after inline-data dispatchers (callee pops return
   address) or after `JSR $FF53` (16 KiB bank switch) unless the selected bank equals the
   caller's bank. Result: 13,437 Observed + 15,831 Static code bytes.
4. Data:
   - Header (16), CHR ROM (131,072; PPU-only space), 8x vector tables and 8x 18-byte
     pre-vector gaps: Static Data.
   - Observed ROM data-read clusters (trace stream, Read events in rom space, per-frame
     files) merged across <=64-byte unread spans, only inside non-code gaps, single
     isolated reads and reads directly after code (dummy-read risk) excluded: ~19.6 KiB.
   - Generic non-instruction data: 64-byte blocks whose maximum linear-decode run
     (bytes decodable before an undefined opcode/BRK, from any start offset) is < 48,
     merged runs >= 128 bytes, no branch/JSR/JMP target from classified code inside:
     75.4 KiB, confidence 0.8. Calibration: every 64-byte block inside observed code has
     max run >= 65.
   - Uniform runs >= 32 bytes: Padding (1,117 bytes).
5. Reset stubs at $xFE0 of 16 KiB banks 0-6: Static code (identical to the real reset
   stub; view [-,-,2b,2b+1] at $FFE0).

### Lead outcomes
- L-001 verified: object action handler table bank 6 $80A7/$810C (101 entries).
- L-002 verified: player action handler table fixed $C926/$C933 (13 entries).
- L-006 verified: replicated reset stubs.
- L-003, L-004, L-005 ready for redirect (see analysis/leads.json).

### Remaining unknowns (6,303 bytes, 121 ranges; analysis/evidence/pass-001/unknown-ranges.json)
Mostly code-like (long decode runs) unexecuted ranges in bank 6 (object handlers
reachable only through not-yet-found tables), bank 2 (map/cutscene helpers, e.g. $8C01
region was partly resolved after S08), fixed bank ($FB01+320, scattered gaps) and small
gaps between data blocks. They were not classified because neither observed fetches nor
a static path/table reaches them and their decode runs prevent the generic-data test.

### Known caveats
- Trace `Read` events also contain opcode-fetch duplicates (filtered by excluding code
  bytes) and possibly 6502 dummy reads; clusters with one read were dropped.
- Generic data meaning is undetermined; boundaries of read clusters are approximate
  within classified data (they do not claim table extents).
- Static code is structural inference; natural reachability of the 52 unexecuted
  object handlers and 4 unexecuted player-action handlers is not claimed.

## Round 2 disassembly pass 2 (checkpoint pass-002)

Consumed redirect r1-a1 ledger (analysis/redirect/r1-a1/unknown-ledger.json, 135 rows).
- Verified with toolkit decode (contexts from ledger) and a whole-map successor scan
  (analysis/evidence/pass-002/check.json): all 24 candidate code ranges tile exactly, use only
  official opcodes, end terminally or fall into classified code, and all their targets are
  instruction starts; no new data range is a control-flow target. Table words: MovingItem
  (7), Pickup (5), SoundCommand (8), EffectParam (5), MusicCommand (25), EnvelopePhase (5)
  all point to instruction starts.
- Entry evidence: L-009 JSR $8900/$A410 after LDA #$02/JSR $FF53 at fixed $C7A7/$C8B0;
  L-007/L-010 via split handler tables; L-008 via inline jump tables. L-011/L-012 have no
  caller (structure-only, Static 0.75; reachability unknown, not claimed dead).
- Accepted with reasoning: $803F (bank 4) as 1-byte generic data - any instruction there
  overlaps observed starts $8040/$8041 and the preceding RTS ends the routine.
- Kept unknown: fixed $F62C (phys 0x1F63C) - legal RTS after JMP $F721; no reference to
  $F62B/$F62C anywhere in PRG (byte-pair scan), so code-vs-filler stays materially ambiguous.
- Labels: commentary labels reused at their physical offsets (regions split at label starts);
  new names for previously unnamed code (SelectPauseScreen, LevelClearTally*, SelectSubScreen,
  MovingItemHandler23/24/25/27, PickupHandler1/3/4, DemoInputRecorder, etc.).

## Round 3 disassembly pass 3 (checkpoint pass-003)

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
Per-bank: every PRG bank 8192/8192, CHR 131072/131072, header 16/16 (table in progress.md).
Execution coverage 13864/31384 code bytes Observed; annotation coverage 1031/1031 regions with evidence.

- L-015 review (evidence-policy "unreferenced return opcode" caution considered): the claim rests on
  independent constraints beyond the legal RTS decode: (1) $F721 PlayerHitSolidSide has one PRG reference
  (raw `21 F7` scan: only 0x1F63A, the JMP operand) and writes C (cmp #$31 / cpy #$10 / clc; SolidSideOverlap
  result read only after its own setting) before any read, so the preceding `clc` is dead; (2) PlayerBump's
  carry contract (`sec ; rts` yes-path, callers `jsr PlayerBump ; bcc`) makes `clc ; rts` the natural no-path
  epilogue; (3) negative data checks from r2-a1 (no word/split/immediate reference, AimDirectionTable bounded
  to $F586-$F5A3, odd address, Observed code on both sides); (4) no Padding/filler region uses $60 as a fill
  value (pass-003 check). Strongest competitor: 1-byte filler; rejected as weaker. Annotated Code Static 0.6,
  region 1811 PlayerBump_DeadRts; reachability not claimed (not fetched in any trace). Revisit if a reference
  to $F62B/$F62C is ever found.
- L-016: 14 regions (ids 931,1661,1010,1307,1349,1363,1401,1424,1435,1445,1459,1561,1573,1575; 230 B) re-decoded
  with toolkit analysis.decode in their saved contexts; every instruction start present in r2-a1 twop-cov.json
  (natural 2P title-menu route, no RAM edits). Provenance -> Observed. 6 partially fetched regions left Static
  (optional split not done; provenance only).
