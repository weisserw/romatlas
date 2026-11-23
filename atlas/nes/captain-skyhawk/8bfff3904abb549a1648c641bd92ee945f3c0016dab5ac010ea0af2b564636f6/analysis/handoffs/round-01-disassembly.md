# Round 1 disassembly handoff (COMPLETE pass; ROM classification incomplete)

- Scope: first pass on fresh workspace (Captain Skyhawk, NES mapper 7/AxROM, 128 KiB PRG,
  CHR-RAM). Baseline created; static AxROM-aware discovery + traced gameplay; whole-ROM
  code/data separation.
- Checkpoint: pass-001 (maps/pass-001.json, generated/pass-001).
- Verification: analysis/pass-001-verification.json - ok true, byte_identical true,
  map_sha256 b253e7de...ee41. map.validate ok; analysis/state.json and maps/current.json saved.
- ROM classification: 99.88% (130936 / 131088 bytes classified; 152 bytes unknown).

## Per-bank counts (classified / unknown bytes)
| Space | Code | Data | Unknown |
|---|---|---|---|
| iNES header (16) | 0 | 16 | 0 |
| bank0 (PRG 0-3, file $00010-$0800F) | 21125 | 11569 | 74 |
| bank1 (PRG 4-7, file $08010-$1000F) | 1365 | 31403 | 0 |
| bank2 (PRG 8-11, file $10010-$1800F) | 3891 | 28871 | 6 |
| bank3 (PRG 12-15, file $18010-$2000F) | 8704 | 23992 | 72 |

## Findings (details: analysis/investigation.md)
- Bank 0 = main bank (reset/NMI/frame loop); banks 1-3 are entered only through the
  far-call stub $FFD2 (jmp ($0018) into per-bank 3-byte JMP tables) and the NMI stub.
- Frame loop: `ldy/ldx/jmp $AE10` sets NMI resume vector $20/$21; NMI does jmp ($0020).
- Static code reached 35,085 bytes with zero invalid/BRK/overlap; all 9,478 observed ROM
  instruction starts agree with static boundaries.
- Data split at code-referenced addresses; specific roles only with extent evidence.
- RAM $0669 = mission index (patch at briefing to k-1 reaches mission k); $02 = lives.

## Leads
- Consumed: none (fresh workspace).
- Produced: L001 (all 152 unknown bytes: unreferenced code-like fragments), L002 (RAM routine
  images stored as Data), L003 (progression: mission completion, docking/space, shop, ending),
  L004 (bank-2 code mostly unobserved), L005 (data semantics for commentary).

## Unresolved questions
- Are the L001 fragments dead code or reached through a computed path not yet found?
- Mission-completion RAM contract (needed for shop/space/ending visits).
- ROM origin of other observed RAM PCs (e.g. $06F0) - copied or built by stores?

## Deferred work
- Scenarios S07-S09 (completion, shop, ending) deferred; see analysis/scenarios.json.
- Labels/commentary for code runs (only structural labels assigned).

## Artifacts
- analysis/scenarios.json, analysis/leads.json, analysis/gameplay.md, analysis/toolkit-issues.md
  (TK-001 confirmed bug, TK-002 missing feature).
- analysis/evidence/round-01/: tools/ (trav.py traversal, datamap.py data rules, apply.py
  annotation, stats.py counts; scripts reference scratch/ snapshot paths - snapshots now in
  snapshots/), coverage.json.gz, camp1-3 logs, frames/, snapshots/ (same-build only).
- scratch/ holds disposable intermediate output only.
