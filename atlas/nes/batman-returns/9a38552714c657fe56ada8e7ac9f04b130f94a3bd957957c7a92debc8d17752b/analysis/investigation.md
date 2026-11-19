# Investigation ledger

Follow workflow.md's completion and investigation contract.

## Round 1 disassembly (checkpoint pass-001)

Game: Batman Returns (Konami, 1993), NES MMC3, PRG mode 0 throughout all observed contexts
(banks 14/15 fixed at $C000/$E000; switchable windows always loaded as even/odd pairs:
0/1, 2/3, 4/5, 6/7, 8/9, 10/11, 12/13 - even bank at $8000, odd at $A000).

### Reviewed regions and evidence

| Range (phys) | Classification | Evidence | Reachability |
| --- | --- | --- | --- |
| 0x0000-0x000F | Data `INesHeader` | iNES header | n/a |
| 0x20010-0x6000F (CHR) | Image `ChrRom` | iNES CHR ROM size/offset | PPU reads |
| Code, 1055 regions, 35,119 bytes | Code | 21,584 B observed fetches (natural boot/title/intro/1-1 and artificial $3E stage tour); 13,535 B static recursive descent | Static bytes not executed in traces |
| Banks 1,2,3,8 (whole) | generic Data | no fetches across all scenarios, no static flow, illegal/BRK density 0.43-0.57; every low-density 32-byte window inspected = tables/tiles/pointers; intro data reads observed in bank 1 ($B46E-$B666) | data reads only in bank 1-3 during intro |
| Bank 9 except $BE85-$BF4D code | generic Data | density 0.40+, pointer/sprite tables in low windows | - |
| 38 inline jump tables after JSR JumpEngine ($C294) / SoundJumpEngine ($8BA9 bank 0) | Data (pointer tables), 251 entries | engine pops return address and does JMP (zp); extent inferred conservatively (see static-tables.json) | entries reached partly observed |
| 0x1FDDF-0x2000F bank 15 tail | fill data, `KonamiIdBlock` ("RD053", "BATMAN RETURNS"), `CpuVectors` | ASCII, vectors decode to observed code | - |
| other gaps (6 ranges >=128 B, zero low-density windows, preceded by RTS), uniform runs >=16 B | generic Data | density / uniformity | - |

Key routines (labels): Reset $FBCC, NMI $FC75, IRQ $CF29 (bank 14), JumpEngine $C294,
SoundJumpEngine $8BA9 (bank 0), SetPrgPairFromStack $D156 (pulls caller-pushed bank, writes MMC3 R6/R7 = Y, Y+1).

Method: analysis/tools-round1/sa2.py (local recursive descent; roots = observed contextual
roots normalised to own-window bank + fixed 14/15, vectors, jump-table entries; JSR fallthrough
only when observed or callee stack-balanced; paths into illegal/BRK opcodes or overlapping decodes pruned
backward over must-edges). Observed contexts/edges: traces/round1/.

### Remaining unknowns (52,061 bytes)

Mostly unreached code plus interleaved tables in banks 0,4,5,6,7,10,11,12,13,14,15 (see per-bank
counts in progress.md). Large gaps with low illegal density (code-like) are unreached code, not data;
they were deliberately left unknown. See leads L001-L005 in analysis/leads.json.

### Lead outcomes this pass

- No leads existed at start. Produced L001-L006 (ready: L001-L004).
- Rejected/held: classifying code-bank gaps as data by density alone (they contain unreached code);
  $3E values 0x16/0x17 (invalid stage, garbled screen).

## Round 2 disassembly (checkpoint pass-002)

Result: ROM classification: 100.00% (393223 / 393232 bytes classified; 9 bytes unknown). Incomplete (9 B).

### Reviewed regions and evidence

| Range / batch | Classification | Structural evidence | Reachability |
| --- | --- | --- | --- |
| 0x15854-0x15861 (L010) | Code (Static) | setter stubs; $984B is JSR target of IntroStep $1A; sibling stub pattern | not traced |
| L001/L007/L008/L009/L011/L012 descent (25,619 B) | Code (Static, 0.85) | r01-a bank-aware descent rerun on refreshed map: pair kept across JSR, MapBanks/BankPop modelled, JumpEngine and local jmp(zp) tables parsed; 0 illegal/overlap/data conflicts | 428 fetches confirmed in Round-2 probes; rest static only |
| 154 jump tables + $DE19/$910E/$CE4C/$CE6C (1,918 B) | Data | consumer dispatchers; extents end at entry target / reached instruction / explicit bound | - |
| JSR fall-through holes (400 B) | Code (Static) | callees return (dispatchers ending in handlers' RTS; $D156 re-pushes return address); clean decode to next instruction boundary | not traced |
| Bank 0 $8EB0-$9FFF + 3 small tables (L004) | Data | SndHeader/Envelope/Vibrato tables consumer-anchored; table targets lie in adjacent blocks | - |
| Banks 12/13 residual (L003) | generic Data | no flow; density 0.29-0.71; low-density windows only word lists/tile runs/records/text | - |
| $C314, bank-ID bytes, $D58F/$DDC0, bank-11 $A043 (L013-L016) | Data | consumer reads, bounds, value conventions | - |
| bank-7 $B649/$B763 tables extended, handler $B70D | Data / Code | full tables end exactly at entry-0 target; entries shared state code | not traced |
| Residual holes (4,179 B) | Data (consumer-read, non-instruction, bank-end trailers) / Code (coherent unreferenced routines, 0.7) | gapscan.py categories; coherent.py checks (clean decode to gap end, terminal end, branch/JSR targets on instruction starts, operands RAM/IO or classified) | routines: no caller found, reachability unknown (L018) |

Always-taken branches: $832A (bne after lda #$17) and $C8C9/$C8CD (bne after lda #imm) — fall-through bytes are data.

### Remaining unknowns (9 bytes)

Isolated single instructions after terminal instructions with no reference: RTS at $8733 (b4), $B1AE, $B792 (b5),
$8CBA (b10), $BBB6 (b11), $FD4A (b15); JMP $FBCC at $C1D2 (b14). Left unknown per evidence policy (an unreferenced return
opcode lacks structure). Lead L017.

### Lead outcomes this pass

Verified: L001, L003, L004, L007, L008, L009, L010, L011, L012, L013, L014, L015, L016. Superseded (unchanged): L002, L005.
Open: L006 (natural ending: final boss reached, natural defeat not achieved; forced defeat flags reach GameMode 7 / $B7B9),
L017 (9 B), L018 (reachability of unreferenced Round-2 routines). Rejected/blocked: none.

### Uncertainties

- Static code reachability (40 KB Static) mostly unconfirmed by traces; structural evidence only.
- Unreferenced coherent routines (442 B, confidence 0.7) could in principle be data that decodes cleanly; no contradiction found.
- Generic data layouts (banks 12/13 scripts, bank-0 sequences) not decoded in detail.

## Round 3 disassembly (pass-003) - L017 review

Claim reviewed: the 9 residual bytes are assembled instructions (Code, Static); no reachability claimed.
Decisive prior evidence (r02-a): zero control/pointer/operand references and zero data reads in the otherwise fully
classified ROM - this rules out *reachable* code and *consumed* data, but is symmetric and does not by itself discriminate
code from data. Discriminating constraint sought this pass: structure enclosing the byte (analysis/tools-round3/l017_span.py).

| Range | Bank/CPU | Decode | Independent structure | Decision |
| --- | --- | --- | --- | --- |
| 0x08743 | b4 $8733 | RTS | inside branch spans 0x08727->0x08744, 0x08719->0x08760, 0x08747/0x0875E->0x08729 | Code Static 0.6 |
| 0x0B1BE | b5 $B1AE | RTS | inside beq 0x0B1B0->0x0B1BF; twin of 0x0B7A2 routine tail | Code Static 0.6 |
| 0x0B7A2 | b5 $B792 | RTS | inside beq 0x0B794->0x0B7A3; twin of 0x0B1BE | Code Static 0.6 |
| 0x1C1E2-4 | b14 $C1D2 | JMP $FBCC | full 3-byte instruction, operand = validated reset entry, exact gap between Code regions | Code Static 0.6 |
| 0x14CCA | b10 $8CBA | RTS | none (no span, no twin) | unknown |
| 0x17BC6 | b11 $BBB6 | RTS | none | unknown |
| 0x1FD5A | b15 $FD4A | RTS (after RTS) | none | unknown |

Strongest competing interpretation for the rejected three: an unconsumed stray byte (no positive support, but the code
reading has only the bare opcode + generic JMP->RTS idiom, which evidence-policy.md names insufficient). No further test
exists without a reference, fetch, consumer or twin; they are the terminal residual. Reachability of all 9: none found.
