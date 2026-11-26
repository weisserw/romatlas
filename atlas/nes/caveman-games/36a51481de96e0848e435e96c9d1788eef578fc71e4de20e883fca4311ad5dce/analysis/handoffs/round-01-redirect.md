# Round 1 — redirect handoff (attempt r01-a1)

**Recommendation: (a)** Run another disassembly → commentary → redirect round on the ready batch below. If the
disassembly phase accepts these proposals, every one of the 2318 unknown bytes gets a classification.

- Scope: all open leads L-001..L-011, the exact unknown complement, and gameplay milestone S14 (natural
  finishes of Saber Race and Dino Race). Research record: analysis/redirect/r01-a1/notes.md (evidence for each
  lead). Scripts, outputs and snapshots are in the same directory.
- Checkpoint: pass-001b (maps/pass-001b.json == maps/current.json; analysis/state.json). Not modified.
  Classification is unchanged at 99.12% (259842/262160; 2318 unknown). No toolkit change and no publish.
- Unknown ledger (exact, every range has a disposition): analysis/redirect/r01-a1/unknown_ledger.json.

## Findings
1. **L-001 (bank 3 $A60C-$AEF8, 2254 B) is not code of this cartridge.** It is a stale, corrupted copy of a
   6502 program assembled for $86EB-$8FD7 and placed here shifted by +$1F21.
   - At that shift, 47 of 66 JSR/JMP targets land exactly after an RTS byte. The next best shift gets 17.
     Real bank-3 code peaks at shift 0 (l001_reloc.txt, reloc_control.txt).
   - Bank 3 and the fixed bank never reference the range (l001_refs_bank3_7.txt is empty), and it has no
     observed fetch.
   - Run in place, its calls would land mid-routine in Dino Race code. Its variables $89D0-$89DA would become
     MMC1 register writes.
   - It polls a development port at $7F00/$7F01.
   - Some bytes are replaced by a copy of the preceding byte (e.g. file 0xE7E7 `8D D9 D9`).
   - Proposed role: generic Data.
2. **L-008/L-009/L-010 cover the other 64 unknown bytes.** Each proposal rests on consumer, extent or contract
   evidence (details under Ready leads).
3. **L-007: bank-7 bytes currently classified as Data are code.**
   - The IRQ split handlers follow from IrqHandler's RTS-dispatch table: the 7 words at $C072 plus 1 give
     $C081/$C08D/$C091/$C0A5. Each ends in JMP $C036, and $C036 falls into the established $C048 epilogue.
   - $F26A is byte-identical to the QueueSfx routine established in banks 0-6.
   - $C62F is byte-identical to observed code in bank 0.
   - $C61A loads X and falls into the observed wait loop at $C61C.
   - All of these are static only: IrqHandler has never run, because CLI only executes in bank 1, whose IRQ
     vector is a bare RTI.
4. **Gameplay (L-006 → verified; S14 visited).** Controls read from the code:
   - Dino Race: tap A for speed; press B to jump when $0478 = 0 and $0476 < $20.
   - Saber Race: alternate A every 2 frames; press B shortly before kind-1 obstacles.

   Both events finished naturally in Practice and showed their results screens. Snapshots:
   dinorace_results.json, saberrace_results.json. The runs added 537 new instruction starts, all inside
   existing Static Code regions; no fetch landed outside Code. The 11 L-004 roots were still not executed.

## Ready leads (for disassembly; recipes are also in analysis/leads.json)

| ID | Physical range (file) / context | Action | Effect |
|---|---|---|---|
| L-001 (high) | 0xE61C-0xEF08, bank 3 $A60C-$AEF8, ctx [6,7,14,15] | One Data region (the inner Data pieces may be merged), Static, conf about 0.85, label e.g. DR_StaleForeignImage, with the notes.md evidence text. Re-check with `python3 analysis/redirect/r01-a1/l001_reloc.py`, `reloc_control.py`, `refscan.py A60C AEF9 3,7` (static, seconds). | 2254 B classified |
| L-010 (high) | 0x1558C-0x15597, bank 5 $957C-$9587 | Data: 0x1558C-0x15591 is the CL_Bracket6Pairs tail; 0x15592-0x15597 is CL_Bracket6Places (6 = NumPlayers in the 6-player bracket; ends at CL_PairFromWinners $9588). | 12 B |
| L-008 (high) | 0x95F6-0x9623, bank 2 $95E6-$9613, ctx [4,5,14,15] | Data 0x95F6-0x95F7 (FS_MarkerXBase, read by FS_DrawMarker). Code 0x95F8-0x9623, Static, unreferenced. Entry contract: PHA; JMP (the meter value is on the stack). Semantic check: 12-byte PpuBuf header and 12 iterations with one put each. Fallback: leave the 44 B Unknown. | 46 B |
| L-009 (medium) | 0xD189-0xD18E, bank 3 $9179 | Code, Static, unreferenced: LDX #0 / JSR / INX, then falls into DR_SetVelocity. Same idiom as the observed LatchBothPads. | 6 B |
| L-007 (medium) | bank 7: 0x1C046-0x1C057; split the Data region at 0x1C082 (76 B) into words 0x1C082-0x1C08F + Code 0x1C090-0x1C0CD; 0x1F27A-0x1F287; 0x1C63F-0x1C644; 0x1C62A-0x1C62B; 0x1C601-0x1C609 | Data→Code, Static. Keep $CEDB-$CEE7 as Data (a lone RTS plus dev-port STA stubs). | about 140 B fidelity fix |
| L-011 (low) | 0x14CCD-0x14CD7 (bank 5 $8CBD), 0x1AE1C-0x1AE25 (bank 6 $AE0C) | Optional Data→unreferenced Static Code. Lone RTS bytes and the $8984 self-jump stay Data. | fidelity only |
| L-012 (low) | Static Code regions with new fetches (cov_session2.json) | Merge the coverage and upgrade provenance or execution records. | provenance only |

After the batch, run map.validate, map.save, map.export, then a new checkpoint (e.g. pass-002). Then run full
current-source commentary on the changed ranges: banks 2/3/5/7 plus the L-001 remark.

## Non-ready / deferred (with reasons)
- **L-004** (11 static roots, 717 B, already classified): reachability only. The natural event finishes did not
  execute them. The untried route is 2-player mode (S15 deferred). No unknown byte or contested range depends
  on it; try it only if the reachability claim is wanted.
- **L-005** (specific roles inside generic data): semantic refinement of already-classified data with no
  completion effect. No consumer question blocks a proposal.
- **L-002, L-003**: superseded by L-008, L-009 and L-010.
- **L-006**: verified (controls known; both events finished).
- Ending: final placings were reached earlier via the $0659 shortcut (S10). Clubbing, Mate Toss and Dino Vault
  finished naturally in round 1, and now Dino Race and Saber Race have too. A full natural Start-the-Games run
  was not repeated: it would add only intermediate-result paths that were already visited.

## Rejection review
Not required: this is not a terminal recommendation.

## Artifacts
analysis/redirect/r01-a1/{notes.md, unknown_ledger.json, l001_reloc.txt, reloc_control.txt, l001_refs_bank3_7.txt,
cov_dinorace.json, cov_session2.json, dinorace_results.json, saberrace_results.json, frames/, leads.before.json, *.py};
analysis/leads.json (L-001..L-012; pass record), analysis/scenarios.json (S14 visited, S15 deferred),
analysis/investigation.md, analysis/progress.md (Pass records). Toolkit: no new issues found.
No background processes are left running (the bridge was stopped).
