# Round 1 — disassembly phase handoff

- Scope: one easy-gain disassembly pass from baseline (no prior regions/leads). Game: Bases Loaded 3
  (Jaleco 1991), MMC3, PRG 32 x 8 KiB, CHR 128 KiB. PRG mode keeps bank 30 at $C000, 31 at $E000.
- Input checkpoint: baseline. Output checkpoint: **pass-001** (maps/pass-001.json,
  generated/pass-001, analysis/pass-001-verification.json).
- Verification: map.validate ok; `run.py checkpoint pass-001` ok, byte_identical true
  (map sha256 24574786…3faa).

ROM classification: 61.07% (240154 / 393232 bytes classified; 153078 bytes unknown).

Kinds: Code 33,004 (Observed 26,459 / Static 6,545), Data 141,575, Padding 65,575. 839 regions.

## Per-bank classified / unknown (bytes)

| Area | Classified | Unknown | Notes |
|---|---|---|---|
| header | 16 | 0 | iNES header |
| PRG00 | 1 | 8191 | BankId |
| PRG01 | 1552 | 6640 | padding tail |
| PRG02 | 1725 | 6467 | code 1724 |
| PRG03 | 1791 | 6401 | padding |
| PRG04 | 1 | 8191 | |
| PRG05 | 1616 | 6576 | padding |
| PRG06 | 1 | 8191 | |
| PRG07 | 1422 | 6770 | padding |
| PRG08 | 1168 | 7024 | code 1167 |
| PRG09 | 2775 | 5417 | padding |
| PRG10-13, PRG15 | 8192 each | 0 | $FF fill (+BankId in 10, 12) |
| PRG14 | 6563 | 1629 | code 367 (1P intro text), padding |
| PRG16 | 1152 | 7040 | code 1119, jump table |
| PRG17 | 190 | 8002 | |
| PRG18 | 156 | 8036 | |
| PRG19 | 205 | 7987 | |
| PRG20 | 4603 | 3589 | code 4578 |
| PRG21 | 1482 | 6710 | code |
| PRG22 | 3351 | 4841 | code |
| PRG23 | 3670 | 4522 | code 1401, padding 2269 |
| PRG24 | 240 | 7952 | |
| PRG25 | 1265 | 6927 | |
| PRG26 | 6077 | 2115 | code 6064 |
| PRG27 | 1545 | 6647 | |
| PRG28 | 5751 | 2441 | code 5688 |
| PRG29 | 3981 | 4211 | padding |
| PRG30 | 8192 | 0 | DPCM samples $C000-$DFFF |
| PRG31 | 7631 | 561 | code 4612, DPCM $E000-$E7FF, tables, vectors |
| CHR | 131072 | 0 | 16 x 8 KiB CHR banks |

## Findings
- Vectors: RESET $FF15, NMI $E921, IRQ $E9FE (bank 31). Jump engines $F292/$ED2C (bank 31), $86B5 (bank 16).
- MMC3 writes via $5E mode bits; far-call helper $EF42–$EFA2 maps banks Y,Y+1 at $8000/$A000 and
  reads per-bank tables at $8001; even banks start with their own bank index.
- DPCM sample system fully bounded ($F552 player, $F59D map, $F5BD record table).
- RAM: $037C half-inning counter; $68-$77 warm-boot signature RAM.
- Details: analysis/investigation.md; gameplay notes analysis/gameplay.md.

## Scenarios exercised (analysis/scenarios.json)
S-001 boot/title; S-002 full WATCH game to GAME OVER; S-003 1P intro/team/order swap; S-004 1P
batting/pitching; S-005 extra innings via $037C; S-006 game-end attempt (rating screen not captured);
S-007 EDIT mode. Deferred: S-008 2P / later levels / ending.

## Leads
- Consumed: none (none existed). Produced: L-001 (jump tables + handlers), L-002 (far-call bank
  tables), L-003 (bulk data banks), L-004 (rejected closures), L-005 (rating screen/levels RAM),
  L-006 (2P / EDIT functions).

## Unresolved / deferred
- 25 jump tables and unexecuted handlers; banks with no observed code; score/out RAM contracts for
  the game-end shortcut; 2P and later levels; DMC DMA read confirmation of samples (optional).
- Artifacts: context dumps traces/round-001/*.json.gz; same-build snapshots
  traces/round-001/snapshots/*.snap.gz; helper scripts analysis/tools/round-001/ (hard-coded
  scratch paths /tmp/ra67 — edit before reuse; emulator.input needs `buttons` arrays).

## Toolkit issues (analysis/toolkit-issues.md)
- TK-001 confirmed bug: NES emulator.input silently ignores documented button/pressed params.
- TK-002 missing feature: NES static discovery stops at every JSR/indexed store on MMC3.
- TK-003 missing feature: trace aggregates slow emulation ~12x; no coverage-only mode.
No blockers.

Status: pass complete; ROM incomplete.
