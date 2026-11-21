# Investigation ledger

Follow workflow.md's completion and investigation contract.

## Round 1 disassembly (pass-001)

### Established conventions (static evidence, fixed bank)
- MMC1 control written $0E by $C104/$C16F/$C1C2 serial writes: 16 KiB PRG switch at $8000, last 16 KiB (8K banks 14,15) fixed at $C000.
- $C14E SelectPrgBank / $C000 (jmp $C1B9) NMI-safe variant: X = 16 KiB bank -> PRG register ($E000 serial), $0100 = current bank. Effective 8K banks = [2X, 2X+1].
- $C852 FarCall: `JSR $C852` + 3 inline bytes (bank, (target-1) lo/hi); pushes thunk $C897 (restores caller bank from stack) and RTS-jumps to target. 95 far-call sites; banks used 0,2,4,5.
- $C58A LoadChrCompressed (X bank, ($20) src, ($22) PPU dest, Y tiles; per plane: mask byte + popcount bytes); $C5AC raw 16-byte tiles.
- Sound driver: fixed $D800 jumps into bank 2 ($8000/$806D/$80A7); music data bank in $0202 (observed 3 and 4), driver bank $0204=2.
- Each 16K bank 0-6 ends with an identical MMC1 power-on stub at $FFBC (+39-byte RAM payload, vectors -> $FFBC).

### Method
- tools/trav.py: bank-aware recursive descent (official opcodes only; BRK/unofficial/JMP(ind) stop a path) from vectors,
  far-call targets, 8 verified dispatch tables (tools/tables_spec.py), 6 hand-reviewed unreferenced routines, and observed
  contexts (analysis/evidence/observed-contexts-r1.json, 12646 contextual roots from traced windows).
- Data: far-call triples, dispatch tables/records, CHR chains (exact chunk tiling between consecutive known pointers:
  bank 1 $8000-$BDB0 20 blocks; bank 6 $8000-$A245 8 blocks + 36 raw tiles + 255-tile block), uniform $FF runs,
  gaps with >=35% unofficial/BRK opcodes in linear decode and no clean >=12-instruction island (reviewed islands
  were $55/$AA tile patterns and music bytes), small gaps read as data at their first byte by classified code,
  13 hand-reviewed text/palette/pointer gaps (tools/manual_data.py).
- Every region applied through map.annotate (toolkit re-decodes code under the explicit prg_banks view).

### Lead outcomes
- No leads existed at start (round 1). Produced L-001..L-007 (analysis/leads.json).

### Rejected / not claimed
- Reachability is not claimed for Static code (416 regions); EXTRA_ROOTS routines have no known caller.
- Generic data regions claim only non-code role; table layouts/meanings are not claimed except CHR chains and dispatch tables.
- Residual caveat: generic-data gaps may still hide short (<12 instruction) unreferenced routines (L-002).

### Remaining unknowns (31 bytes)
0x15518+5, 0x160c1+5, 0x178f9+13, 0x179d7+2, 0x1b037+6 — no consumer/structure found (L-001).

## Round 2 disassembly (pass-002)

Classification: 100.00% (131088 / 131088; 0 unknown). Scripts: analysis/pass-002/apply_leads.py, provenance_l009.py, apply_l009.py.

### Lead outcomes
- L-001 verified (structure: consumer contracts DrawMetasprite count+4n, DrawMetatileList 0-terminated, QueueVramString X=2; bounds = next code;
  reachability: natural reads for $9508/$A0B1/$B9C7, artificial for $B8E9/$B8EF). $B027 residue: generic data, no specific role claimed.
- L-008 verified (structure: RunBossScript bound cmp #$0D; targets decode; $97A2/$97A4 E6 1C fallthrough into BossActionMove; BossBounceOffsets
  terminator at $97A1). Only $9684 references bytes $967E-$9685 in bank 0 (word search) -> words 13-15 unselectable, generic data.
  Reachability of entries 6/7 not observed and not claimed.
- L-004 verified: DMC sample role for $E000-$FF60 union (table re-derived from ROM, natural dmc_dma reads); 5 gaps generic.
- L-010 verified: record domains World 0-5 x Stage 0-7 give the 144/96-byte extents; $815B separate 13-byte ItemChrIndex.
- L-009 verified: provenance split by instruction-run coverage; natural vs artificial kept distinct in evidence text.
  Coverage sources: round-1 observed contexts (artificial flag) + redirect windows (poweron/demo/bonus natural; battle/gameover/ending1/credits artificial).

### Not claimed / open
- Reachability of remaining 9526 B Static code (incl. $C149,$C21B,$C8DA,$C92A,$A94A, K22 $93E5, BossActionMoveSpeed2/3).
- Generic data meanings (music/level data etc.) — commentary semantics only.
