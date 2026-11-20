# Unknown-complement ledger — redirect r02-a (input checkpoint pass-002, 1,044 unknown bytes, 25 ranges)

Source: maps/current.json = maps/pass-002.json (sha256 c6d84b24...). Ranges: analysis/evidence/round-02/residual-unknown.txt.
Dispositions: P = proposal (lead, kind); R = residual (no supported proposal). Proposal detail: proposals.json.

| Range (phys) | Bank/CPU | Bytes | Disposition |
|---|---|---|---|
| 0x0012A-0x0012F | 0 $811A | 6 | P L-020 Code (MapWin_ShowPortrait twin) |
| 0x01D53-0x01D94 | 0 $9D43 | 66 | P L-020 Code (bite chance roll, operands = adjacent Data) |
| 0x021C9-0x0221D | 1 $A1B9 | 85 | P L-019 Data $A1B9 (AND #$3F table end) + Code $A1BA-$A20D (+ Data->Code $A20E-$A21B, dummy-read artefact) |
| 0x023FD-0x0252B | 1 $A3ED | 303 | P L-019 Data $A3ED-$A3F4 (FishProbeDY 8-15), Code $A3F5-$A4C4, Data $A4C5-$A4DB, Code $A4DC-$A51B |
| 0x0252D-0x0254C | 1 $A51D | 32 | P L-019 Code (+ Data->Code $A51C, $A53D-$A53F) |
| 0x0258D-0x025ED | 1 $A57D | 97 | P L-019 Code (+ Data->Code $A560-$A57C) |
| 0x0267C-0x0268A | 1 $A66C | 15 | P L-019 Code (falls into observed $A67B) |
| 0x027E6-0x02810 | 1 $A7D6 | 43 | P L-019 Code (+ Data->Code $A801, dummy-read artefact) |
| 0x02812-0x0282C | 1 $A802 | 27 | P L-019 Code |
| 0x028C1-0x028ED | 1 $A8B1 | 45 | P L-019 Code |
| 0x02DC6 | 1 $ADB6 | 1 | R lone RTS after `jmp $EDA3` (see rejection review) |
| 0x02F0F-0x02F33 | 1 $AEFF | 37 | P L-019 Code |
| 0x032FA | 1 $B2EA | 1 | R lone RTS after `jmp $B4D7` |
| 0x03353 | 1 $B343 | 1 | R lone RTS after `jmp $B3C5` |
| 0x03434-0x03485 | 1 $B424 | 82 | P L-019 Code |
| 0x047B4-0x047BC | 2 $87A4 | 9 | P L-020 Code |
| 0x18A82-0x18A87 | 12 $8A72 | 6 | P L-020 Code (script native test shape) |
| 0x1A6AB-0x1A6B2 | 13 $A69B | 8 | P L-020 Data (script bytecode subroutine) |
| 0x1C095-0x1C09D | 14 $C085 | 9 | P L-020 Code |
| 0x1CA08-0x1CA38 | 14 $C9F8 | 49 | P L-020 Code |
| 0x1CAC5-0x1CB04 | 14 $CAB5 | 64 | P L-020 Code |
| 0x1DDB3-0x1DDBE | 14 $DDA3 | 12 | P L-020 Code |
| 0x1DF86-0x1DFAA | 14 $DF76 | 37 | P L-020 Code |
| 0x1E0FF-0x1E106 | 15 $E0EF | 8 | P L-020 Data $E0EF-$E0F0 (signature 15-16) + Code $E0F1-$E0F5; R $E0F6 lone NOP |
| 0x1E1C3 | 15 $E1B3 | 1 | R lone NOP between RTS $E1B2 and IRQ handler $E1B4 |

Totals: proposed Unknown->Code 997, Unknown->Data 42, Data->Code 48 (rule-R/W Data). Residual after proposals: 5 bytes
(0x02DC6, 0x032FA, 0x03353, 0x1E106, 0x1E1C3).
