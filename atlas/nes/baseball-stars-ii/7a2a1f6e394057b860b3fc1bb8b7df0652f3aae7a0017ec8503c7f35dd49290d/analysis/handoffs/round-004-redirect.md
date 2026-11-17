# Round 4 redirect handoff (attempt analysis/redirect/r4-redirect-a) - COMPLETE

Recommendation: **terminal incomplete**. No supported proposal. Residual: 8 unknown bytes in 4 runs.

## Scope and checkpoint
- Input: maps/pass-004.json = maps/current.json = analysis/state.json (sha256 077d58cd3516...), 393,224 / 393,232
  classified; residual list analysis/pass-004-remaining.json. Round-4 commentary handoff (complete) accepted as input.
- Changed only research/lead records (leads.json, progress.md, this file, r4-redirect-a/). ROM (7a2a1f6e...), map, state,
  source and configured binary untouched (hashes re-checked). Emulator ran on a /tmp store copy (deleted); daemon stopped.
- Proposed bytes: 0. Accepted bytes this round: 0.

## Residuals (exact)
| Run | Physical | Bank / CPU (prg_banks) | Bytes | Neighbours |
|---|---|---|---|---|
| R1 | 0x0040EA-0x0040EB | b2 $80DA-$80DB ([2,3,30,31]) | 60 60 | after RTS $80D9 (end of X/Y->VRAM-address routine $80B0); before Code $80DC |
| R2 | 0x005998-0x00599B | b2 $9988-$998B | A9 80 A9 00 | after $9987 RTS ($C316 table target); before $998C ($C316 table target) |
| R3 | 0x005B58 | b2 $9B48 | 60 | after EditedLineupPtrs (6 words, $9B3C-$9B47); before Observed $9B49 |
| R4 | 0x012175 | b9 $A165 | 60 | after RTS $A164; before Observed $A166 |

## Coordinator questions - results
1. **Stale-copy / identity evidence** (copies3.rerun.txt, fuzzy4.py/json).
   - r3 copies3.py did cover these exact windows (exact matches with 0/2/4/8 bytes of left/right context; k<=2
     near-copies of runs >=4 bytes). Re-run: only hits are R1's `xx 60 60 60` inside bank-20 generic data, and the
     generic 1-2 mismatch hits for the common `A9 xx A9 xx` pattern (R2).
   - New: a fuzzy search (3-gram diagonal voting, 48 bytes + run + 48 bytes, all PRG).
     - One real copy: the R1 routine $80B0-$80D9 is duplicated as classified C06 in bank 3 $B0B3-$B0DD. That copy
       ends on a single RTS, followed directly by ASCII menu strings (D11). So the routine needs one RTS, and the extra
       `60 60` at R1 is not part of the routine's copied body. Whether it is padding or a stub is not settled.
     - R2, R3 and R4 have no related context anywhere else; the best alignments match 15-19 of ~100 bytes, which is
       noise.
   - Result: no stale-copy identity for any run. No copy assigns a role.
2. **Padding conventions** (census in this attempt, results summarised in leads.json L021 attempt redirect-004).
   - About 50 classified standalone `$60` regions exist. Every one has a basis: recursive descent from a branch, JMP,
     $C316 or far-call target; an observed fetch; or (0x03CF54) an indexed-read consumer.
   - Of the 6 RTS;RTS pairs, the 4 classified second RTSes are a branch target, a $C316 target ($9987, and $B4F6 via
     the table at b3 $B40D) or Observed. The other two pairs are R1 and R4.
   - Runs of 3 or more `$60` occur only inside consumer-read or generic data tables.
   - Uniform fill between regions is `$00` (e.g. 0x010A78+1432, 0x017BF2+478), never `$60`.
   - No byte anywhere was accepted as `$60` padding, or as unreferenced lone-RTS code. Leaving the residue unknown is
     therefore consistent with earlier passes.
   - C01/C02 (accepted round 4) differ: each had a consumer-bounded start, a non-trivial contract and a sibling or
     semantic link. The residue runs have none of these.
   - The ROM-wide evidence also gives no support to a `$60`-fill data reading.
3. **Runtime check** (readprobe4.py, readprobe4.json, readprobe4.log). Not tried before: the earlier rounds checked
   instruction starts only, not data reads.
   - Setup: Read+Instruction trace on [run-1, run end) for each run. 10 saved snapshots (mainmenu, mainmenu_madeteam,
     league_made, league_watchq, p1_fielding, stadpick_cpu, league6x1_made, cpuB_17, vs2p_errors_on_fielding,
     league_champion), 120 frames each, buttons released, no RAM or register writes.
   - Result: 0 data reads and 0 executions of any residue byte.
   - Positive control: the probe caught and correctly classified the 6502 RTS dummy read of $A165 after RTS $A164
     executed (p1_fielding).
   - The negative result is weak evidence, as expected; more sweeps have a reduced prospect.

## Rejection review (updated; supersedes nothing - r3 review preserved)
| Run | Claim tested | Decisive evidence | What it rules out | Competing reading | Disposition |
|---|---|---|---|---|---|
| R1 | code (two RTS stubs) | refs3.json no ref; bank-3 copy C06 ends at single RTS; readprobe 0 reads/fetches | referenced entry; copied routine tail; observed data consumer | non-$00 filler / removed-stub residue; data has no consumer either | unknown: unreferenced return opcodes alone lack structure (policy) |
| R2 | code `LDA #$80 / LDA #$00` (dead, overwritten by `LDA $0531` at $998C) | $C316 table [$9987,$998C] exact targets; refs3 none; no copy; 0 reads | entry via known dispatch; table continuation | data pair vs dead edit residue; `LDA #$80` matches the local `LDA #$80/STA $96` idiom but the fragment has no store/contract, so the sibling is shape-only | unknown: no discriminating role evidence |
| R3 | code RTS after table | 7th word would be $A960 outside the $7Fxx domain; no ref; no copy; 0 reads | table continuation; referenced entry | lone filler vs stray RTS | unknown |
| R4 | code RTS after RTS | refs3 none; no copy; only an RTS dummy read observed | referenced entry; data consumer in sampled states | filler vs stray RTS | unknown |

Independent constraints considered together: decode, bounds, references, table domains, the RTS idiom, ROM-wide
padding practice, copy identity and data reads. None of them discriminates code from data for any run. Concrete revised
test: none with a credible prospect. A new discriminator would need one of these: a reference (including an RTS-trick
push), a genuine data read or fetch in a state not yet sampled, or a non-trivial contract.

## Leads
- L021: stays blocked; a redirect-004 attempt was appended and the recipe updated.
- L006: verified (unchanged).
- No new IDs. Pass record redirect-004 added.

## Deferred / remaining
- Final editorial audit and verification remain required for acceptance with the 8-byte residue.
- Possible future runtime: data-read tracing over new natural states (e.g. full VS games, team editing) only if they
  become available for other reasons. A dedicated sweep has a slim prospect.

## Artifacts (analysis/redirect/r4-redirect-a/)
ctx4.py, copies3.rerun.txt, fuzzy4.py/json, readprobe4.py/json/log, readprobe4.smoke30f.json, rdaemon4.py, rr4.py,
leads.before.json, STATUS.txt. Disk: 4.9G free; scratch unchanged.
