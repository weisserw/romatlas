# Round 3 — disassembly handoff (pass-004)

## Scope
Single disassembly pass resuming from pass-003 (analysis/state.json, byte_identical). Reviewed and applied the two
ready redirect leads (L-023, L-024). Static only; no emulator runs (gameplay already broad; nothing unknown needed runtime).
Scripts/logs: analysis/pass-004/ (apply.py with built-in assertions, apply.log, verify.log, gapcons_all.py/.log, stats.log,
state.pre-pass-004.json). Leads backup: analysis/leads.pre-disassembly-r3.json.

## Checkpoint and verification
- Checkpoint **pass-004**: maps/pass-004.json, generated/pass-004, analysis/pass-004-verification.json:
  ok true, byte_identical true, map_sha256 119004fa6a25debf62003c9d25b35c8da7fe2278ad4cfbe4e064968257d55690
  (identical to analysis/state.json and maps/current.json). map.validate ok.

## Counts
ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

| Space | Code | Data | Text | Padding | Unknown |
|---|---|---|---|---|---|
| iNES header | 0 | 16 | 0 | 0 | 0 |
| PRG 16K bank 0 | 11733 | 4369 | 111 | 171 | 0 |
| PRG 16K bank 1 | 1425 | 12855 | 0 | 2104 | 0 |
| PRG 16K bank 2 | 1578 | 14752 | 0 | 54 | 0 |
| PRG 16K bank 3 | 1578 | 14752 | 0 | 54 | 0 |
| PRG 16K bank 4 | 1578 | 11231 | 0 | 3575 | 0 |
| PRG 16K bank 5 | 192 | 12637 | 0 | 3555 | 0 |
| PRG 16K bank 6 | 5118 | 6404 | 0 | 4862 | 0 |
| PRG 16K bank 7 | 1970 | 9322 | 5 | 5087 | 0 |
| CHR ROM | 0 | 131072 | 0 | 0 | 0 |
| **Total** | **25172** | **217410** | **116** | **19462** | **0** |

Execution coverage is a separate metric and unchanged by this pass.

## Leads
- **L-023 consumed (accepted)**: physical 0x1AC8 (b0 $9AB8, $60, prg_banks [0,1,14,15]) -> Code, Static, conf 0.6,
  label UpdateBoyStop_OrphanRts, region id 2581. Structural claim only; reachability: none known (unreferenced).
  Reviewer reasoning (evidence-policy.md): a lone unreferenced RTS is insufficient on its own, but here it is combined with
  (a) the doubled-RTS idiom found 16 other times in code, every second RTS referenced (branch/JMP/JSR or word table);
  (b) no consumer, operand, word or #$9A immediate addresses it, never read/fetched while neighbours are hit thousands of
  times; (c) no fill convention ($FF-only padding, odd address); (d) new all-bank survey: of 25 small non-code gaps
  enclosed by code, 24 are consumer-read and the 25th (b7 $CBB7) is visibly non-instruction ASCII, so an unread
  opcode-valued datum has no analogue. Competing reading (unread 1-byte datum) is logically possible but unsupported.
  Revisit if any read/consumer of $9AB8 is found.
- **L-024 consumed**: 25 bank-6 regions / 203 B Padding -> Data (Static 0.9, ids kept, not merged): 24 x 8 B at
  template+$3A/+$50 of templates $8E4D,$8EBB,$8F29,$8F97,$9005,$9073,$90E1,$914F,$91BD,$922B,$9299,$9307 (all ObjSetTemplates
  targets; whole 110-byte copy loop b6 $8937) and $A0B3-$A0BD inside TypeBounces ($A0A6-$A0C4, read by ObjectsHitFloor).
  Script asserted exactly 25 Padding regions in those ranges, each all-$FF at the expected offset.
- Rejected: none. Unchanged: L-006, L-008 (ready, verification/reachability-only, no unknown bytes), L-009 partial.

## Remaining unknowns
None (0 bytes). Classification complete at the structural level; overall completion still requires commentary of the
changed regions and the final editorial audit/byte-identical verification of the commentary source.

## Notes for commentary (changed regions)
- b0 $9AB8: replace the "only unclassified byte" note (commentary/source/game.asm ~line 4174 and header line 15) with
  UpdateBoyStop_OrphanRts: second RTS after UpdateBoyStop_Done ($9AB7), unreferenced, structural Code (Static 0.6),
  no reachability claim; cite the doubled-RTS idiom.
- b6 template +$3A/+$50 runs (24 regions $8E87..$9357) and $A0B3-$A0BD: now Data; update the "Phase 1 classified these
  as padding" note at $A0B3 and the template notes (field 5/7 = object slots 3-10 default $FF values).
- New observed paths from round-2 redirect (rocket flight, post-ending restart) are provenance notes only.

## Toolkit findings
No new toolkit issues found this pass; TK-001 (NES emulator.input parameters), TK-002 (MMC1 static discovery/budget),
TK-003 (tracing slowdown) stand — see analysis/toolkit-issues.md.
