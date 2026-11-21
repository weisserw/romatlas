# Investigation ledger

## Round 1 disassembly (2026-11-21), checkpoint pass-001

Game: Break Time - The National Pool Tour (FCI 1992), MMC1, 8 x 16 KiB PRG (16 x 8 KiB
physical banks 0-15), 32 KiB CHR. Bank model observed: power-on [0,1,14,15]; reset
vector $8000 -> bank 0 stub `INC $FFF9 / JMP $C000` (every even 8 KiB bank starts with the
same stub followed by a JMP entry table at $8006, $8009, ...); fixed bank 14 at $C000 starts
with the same stub then `JMP $C0B7`. Contexts observed: [0,1,14,15], [6,7,14,15],
[8,9,14,15], [10,11,14,15], [12,13,14,15] (16 KiB fixed-last mode), [0,1,2,3] (32 KiB mode
during title; NMI $FAB0 in bank 3), [0,1,4,5] (eight-ball title), [0,1,0,1].
Controller routine: $C4BD (bank 14) -> held $56/$57, new presses $58/$59.

### Method and evidence

1. Observed code: aggregated contextual instruction roots from traced scenarios
   SC-001..SC-007 (analysis/scenarios.json), merged in traces/round-1/cov.json
   (11,737 unique contextual roots). Each executed opcode offset gives an instruction
   with CPU address and effective prg_banks; contiguous instructions merged into Code
   regions with provenance Observed (18,986 bytes).
2. Static extension (traces/round-1/tools/extend.py): from each observed instruction's
   unobserved successors (branch targets, JMP abs, JSR target, JSR fallthrough), a closure
   in the same effective context. Rules: stop at RTS/RTI/BRK/JMP(ind); after a store to
   >= $8000 or a JSR from fixed-bank code, switchable slots become unknown; JSR fallthrough
   not followed if the callee was observed never returning to its fallthrough or begins
   with PLA/TSX/TXS within 6 instructions; whole closure rejected if it contains BRK or an
   unofficial opcode, overlaps an observed instruction or another static instruction,
   crosses a window, uses a non-indexed absolute operand < $0100, or references $0800-$1FFF,
   $2008-$3FFF, $4020-$5FFF. Accepted 13,952 bytes Static (confidence 0.85). Mnemonic
   distribution of static code matches observed code (lda/sta/jsr/bne dominant).
   One fixed-bank-origin chain (from $C2B1 BEQ, bank 14) enters banks 10/11 using the
   only observed context [10,11] of that routine (no mapper write between).
   Rejected closure: origin physical 978 ($83A7, from BNE at $83A5) reaches opcode $0F
   at 4152 (lead L-001).
   Note: toolkit analysis.discover stops at every JSR on MMC1 (documented), so it was not
   used for bulk discovery; all regions were validated by map.annotate decoding.
3. Structural data (Static): iNES header; CHR ROM (Image); fixed-bank vectors
   ($FFFA NMI=$C571, RESET=$8000, IRQ=$C5EE) and $FFF8/$FFF9 (operand of the INC $FFF9
   MMC1 reset trick); 8-byte trailers at the end of every odd 8 KiB bank (bit-7 byte +
   vector-format words); 8 bytes $FFF0-$FFF7 (generic data, role unknown); uniform $00 runs
   (Padding for bank-tail runs, Data for two 85-byte interior runs in banks 7 and 10 that
   end exactly at observed code); bank-0 dispatch table $82E4 (2 words, bounded by
   CMP #$02/BCC at $80ED, entry $8103 observed).
4. Observed data reads: ROM Read events streamed for range [8208,49168) and
   [98320,114704) along the nine-ball route and [32784,49168) along the eight-ball route
   (traces/round-1/reads_*.json, readruns.json). Filters: drop page-cross dummy reads
   (read followed on the next cycle by +$100), drop the byte after a 1-byte opcode, drop
   bytes classified as code, drop isolated single-byte reads. 76 runs annotated Data
   (Observed, confidence 0.9) covering 7,227 bytes; extents are read bytes only.
5. Text (Static): bounded scan of unknown bytes for uppercase ASCII words ($0D line
   breaks); 140 spans, 2,201 bytes (menus, tour city/opponent profiles, prompts, ending
   message, staff credits). Graphics-bank false positives ("UUUU...") and alphabet tile
   sequences excluded. Terminators/pointer tables not claimed (L-008).

### Reachability vs structure

Observed regions = executed in debugger scenarios (natural inputs, no RAM edits in round 1).
Static regions = structural inference only; reachability not observed. No region was
labelled dead/unused.

### Remaining unknowns (56,064 bytes)

Largest: banks 2-5 graphics-like data (~26.3 KiB unknown; L-004), banks 12-13 (~12.9 KiB;
likely sound data, L-005), bank 7 (4.1 KiB incl. word tables at $B1EF..), bank 1 (4.2 KiB,
tables near observed reads), bank 0 (2.6 KiB: palettes at 7563.., tables, $82E8 code
candidate L-007), banks 6/8/10 tables (1.5/1.8/1.9 KiB), jump tables (L-002).

### Lead outcomes

No leads existed before round 1. Produced L-001..L-009 (analysis/leads.json).

## Round 2 disassembly (2026-11-21), checkpoint pass-002

Input pass-001 + redirect r1-a1 proposal. Tools: traces/round-2/tools (apply_r2.py applies
reviewed proposal entries to unknown sub-intervals only; apply_resid.py the residual ledger;
codecheck.py the code-structure test). All new regions are Static unless noted.

### Proposal review (accepted / rejected bytes per lead)

| Lead | Accepted | Rejected | Review evidence |
|---|---|---|---|
| L-010 DPCM | 26,330 data | 0 | Re-derived VoicePlayer1 ($FA7C): A*4 index into $C009 table, $4012/$4013 writes; recomputed tiling ($C040..$FA41 bank 2/3, last sample ends at $FCC0 bank 4/5); $AA fill uniform; $C025-$C03B residue is a byte copy of bank-14 slot bytes, truncated mid-slot; PlayVoiceSample disables NMI and the voice-bank NMI/IRQ vector is an RTI, so no slot copy is used while mapped -> data. |
| L-005 sound | 12,945 (12,903 data + 42 code) | 0 | Parser is consumer-driven; only 52 range bytes are not parser-covered (single $C5 stop bytes, 3 NotePeriodTable words, bank-13 tail interleave). Cmd-7 handler $82EC code. Bank 13 $B744-$B943 copy of bank 1 accepted as residue data. |
| L-015 screens | 4,817 data | 0 | Re-ran chain: palettes/metatile sets (multiples of 5)/self-delimiting screens tile 7563..14676; found a second 32-byte palette after $A206 (palette-valued, 8790 start of metatiles) and the 1-byte $AD28 gap; non-code either way. |
| L-014 bank 7 | 2,822 data | 0 | Scripts/tables tile to $B2E1; $B2E2-$BCF4 identical to bank 5. Optional $BCC0-$BCF4 (copy of VoicePlayer2+RTI) accepted as residue data (conf 0.7): copy ends exactly where bank-5 code ends, nothing in bank 7 references it, and as bank-7 code its LDA $C009,X would read the fixed-bank slot table. |
| L-001/L-007/L-011 bank 0 | 908 code + 43 data (CharToTileTable) | 0 | Closure re-run: only pruned edge is BNE $9028 at $9025 after LDA #$00 (Z=1, never taken); no illegal/BRK/overlap/misalignment, no stack-manipulating callees. Table index domain 0..$2A from CMP guards. |
| L-011 slots/stubs | 320 code (+ bank 8 $8966 200, bank 14 165) | 0 | Every slot target is an existing or newly closed code start; reset stubs identical to the executed bank-0 stub. |
| L-012 bank 6 | 241 (182 code + 59 data) | 0 | Closure clean; tables bounded by next handler. |
| L-013 | included above (16 code) | 0 | clean. |

### Residual ledger (L-016 / L-008 / L-009): 7,450 bytes classified, 23 left unknown

- Consumer-anchored tables (42 ranges, 4,765 bytes): generic data; each range is read by an
  indexed load in classified code (anchor listed in the region evidence) and fails code decoding
  (illegal opcode/BRK/misalignment). Exact table layouts not claimed. 58489..58501 split: 12-byte
  table + clean routine $A475 (code).
- String terminators/headers (L-008): 1,295 bytes data (`$00`, `col,row` headers, short strings);
  85771 ($8EFB) split: 5-byte code-shaped prefix left unknown, 2-byte header data.
- Non-code gaps: 590 bytes data (square table $AA66 512 B, text boxes/headers 78 B).
- Adjoining runs (L-009): 62 bytes data.
- Code: 16 ranges, 738 bytes. Test (codecheck.py): whole range decodes with official opcodes, no
  BRK, internal branch targets on decoded boundaries, all exits to classified instruction starts,
  ends with RTS/JMP or falls into a classified instruction start, and every data operand is a
  variable also used by classified code in the same 16K bank. No direct reference was found
  (entry presumably through unresolved indirect dispatch); reachability unknown, not claimed dead.
  65837 ($811D bank 8) is a JSR fallthrough of classified code.
- Left unknown (23 bytes, 9 fragments): phys 4964 (JMP $935D), 6173 (RTS), 51444 (LDA #$FF/RTS),
  84584/84607 (JMP chain $8A58->$8A6F->$8A74), 85771 (LDA #$80/STA $A3/RTS), 117501 (RTS),
  117621 (duplicate JMP $CB8F), 118162 (RTS). All sit right after an unconditional JMP/RTS with no
  reference; they look like unreachable structured-macro epilogues, but an unreferenced RTS/JMP
  alone lacks sufficient structure under evidence-policy.md. None executed in round-2 traces.

### Gameplay evidence (round 2)

- SC-014 Trick Shot: menu lists 7 trick shots (consistent with 7 TrickShotScripts, L-014);
  shot 1 played (traced).
- SC-015 National Pool Tour with the in-game A+B+SELECT match-result shortcut (MainLoop_TourCheck $C1F9, JMP MainLoop_MatchOver at $C21E;
  natural joypad input, no RAM edits): New York -> Milwaukee -> Atlanta -> Los Angeles -> Las
  Vegas -> EndingSequence (victory text) by frame ~15,000. Traced: the first stage-won transition
  (frames 3541..3850) and the ending start (14565..15493).
- Coverage: 5,044 contextual roots; all fall in Code (none in Data/Text/unknown), including 203
  roots in the round-2 bank-0 closure ($8E39 stage-won screen) and 2 in new slot code, confirming
  their code role in natural play. 23 Static regions fully executed were re-annotated Observed.

## Round 3 (disassembly, checkpoint pass-003)

### L-020 — accepted (12 bytes, Static code)
Independent review (traces/round-3/tools/around.py re-decodes classified neighbours from the map;
apply_r3.py runs analysis.decode in the neighbour bank context). For each fragment: the preceding
instruction is an unconditional JMP closing a THEN block; a classified forward branch targets the
fragment end (ELSE label); the fragment is exactly one JMP abs whose target is the first
instruction after the ELSE block (ENDIF):
- 4964 bank 0 $9354: BEQ $9357 at $934E; JMP $935D (RTS after PLA/SEC/SBC/BNE else block).
- 84584 bank 10 $8A58: BMI $8A5B at $8A40; JMP $8A6F (end of the 20-byte else block = next fragment).
- 84607 bank 10 $8A6F: BNE $8A72 at $8A38 (encloses the inner IF); JMP $8A74 (RTS after INC $29).
- 117621 bank 14 $CB65: BNE $CB68 at $CB56; JMP $CB8F (else-block end; sibling live JMP ENDIF at
  $CB51 targets the same address, and $CB6F/$CB78 show the same idiom in this routine).
PRG word scan (address and address-1): only hits are 84585 (the operand of fragment 84584, nested
chain) and DPCM bytes 47527/63911 (coincidental). Annotated Code, provenance Static, confidence
0.8, views in the neighbours' bank context. Reachability: none known; not Observed, not claimed unused.

### L-019 — verified (metadata split, 0 net bytes)
Region 88856..89517 (Data) split: 88856+32 (16 X/Y-range byte pairs, no reader found),
88888+624 (RackLayouts $9B28, 13 x 48, base from ADC #<$9B28/#>$9B28 in InitRackBalls, ends at
$9D98), 89512+5 (empty string at col 26,row 22 + $00, then $15 $15 header of "WHOEVER GETS").

### L-017 — still blocked (11 bytes)
Not retried; the r2-a1 rejection review (analysis/handoffs/round-2-redirect.md §2) was re-read and
still applies: no reference (word scan re-run this round: only coincidental 536 and 67504), and the
"RTS after loop JMP" generator rule is falsified by bank-14 $CBC7. Remaining unknown:
6173+1 (bank 0 $980D), 51444+3 (bank 6 $88E4), 85771+5 (bank 10 $8EFB), 117501+1 (bank 14 $CAED),
118162+1 (bank 14 $CD82).

### Gameplay
Not exercised: scenarios SC-014..SC-016 already cover tour-to-ending, passwords and trick shot; no
remaining unknown byte is tied to an unvisited game state, so no concrete gain was likely.
