# Round 3 redirect handoff (attempt analysis/redirect/r3-redirect-a) - COMPLETE

Recommendation: **next: disassembly round 4**. One combined batch is ready: L021 (C01 and C02) and L025 (F01).

## Scope and checkpoint
- Input checkpoint: maps/pass-003.json, which equals maps/current.json and analysis/state.json (sha256 c36cb8af12e9...).
  - Classification is 99.99%, with 25 unknown bytes in 6 runs (analysis/pass-003-remaining.json).
  - The round-3 commentary handoff (complete) was accepted as input.
- This attempt continued the interrupted r3-redirect-a from its saved frontier.
  - It did not create a new baseline.
  - It did not replay the completed scans (fidelity3, refs3, rtsidiom, islands3, copies3) or the league probes.
- Research and lead records only were changed. The ROM (sha 7a2a1f6e...), map, state, source and configured binary are
  untouched.
  - The emulator ran on a /tmp copy of the store, which is now deleted.
  - The daemon was stopped at handoff.
- Verification:
  - analysis/state.json, maps/pass-003.json and maps/current.json still hash c36cb8af.
  - The proposal sanity checks in write_props.py pass: every proposed Code byte is currently unknown, and the split
    lengths add up to 245.

## Proposals (analysis/redirect/r3-redirect-a/proposals.json; reproduce with `python3 write_props.py` from the workspace root)
| Item | Lead | Range (physical) | Bank / CPU | Bytes | Change |
|---|---|---|---|---|---|
| C01 | L021 | 0x03C0D0-0x03C0D4 | b30 $C0C0-$C0C4 | 5 | unknown -> Code/Static |
| C02 | L021 | 0x03F6D2-0x03F6DD | b31 $F6C2-$F6CD | 12 | unknown -> Code/Static |
| F01 | L025 | 0x004C62-0x004C63, 0x004CB0-0x004CB1 | b2 $8C52, $8CA0 (prg_banks [2,3,30,31]) | 4 | Code -> Data (inline words); region 0x004C3A+245 split 40/2/76/2/125 |

- **Proposed: 17 of 25 unknown bytes**, all Code/Static, plus a 4-byte Code->Data correction.
- **Residual: 8 bytes in 4 runs.**
- Reachability: no proposal item has a known caller or an observed fetch. None of them is claimed unused.

**F01 (inline-parameter fidelity fix)**
- InlineParamCallA ($C6D6) and InlineParamCallB ($C6FC) are Code/Observed. Each one:
  - pulls the return address (which points at JSR+2);
  - reads (ret)+1 and (ret)+2 into Temp04/05 (A) or Temp0C/0D (B);
  - resumes at ret+3 through `SEC / TYA(Y=2) / ADC` and `JMP ($0006)`.
- So $8C52-$8C53 (word $2204, a VRAM address) and $8CA0-$8CA1 (word $00BA, the digit buffer $BA-$BD filled at
  $8C86-$8C9B) are never executed. The resume points $8C54 and $8CA2 decode cleanly.
- ROM-wide check (fidelity3.json):
  - These are the only 2 of 388 JSR $C6D6/$C6FC sites whose following bytes are Code.
  - Every other inline-argument callee family ($C316, $C271, FarCall Y=4/8) is consistent.
  - 0 branch/JMP/JSR targets land in Data, unknown bytes or mid-instruction.

**C01 (LoadChrBankTemp, `LDA $22 / STA $22 / RTS`, confidence 0.65)**
- Start bound: RandomTable is extent-bounded by its consumer. NextRandom does `AND #$3F` and indexes the 64 bytes
  $C080-$C0BF, all with values 0..7. $C0C0 ($A5) lies outside that extent and that value domain.
- End bound: the RTS sits right before classified $C0C5.
- Sibling shape: the opening `LDA zp / STA same zp` is the same as in the adjacent classified entries $C0C5 (zp $23)
  and $C0FD (zp $24), which feed SetBgChrSet/SetSprChrSet. $22 is ChrBankTemp, which the same routines use.
- Contract: it returns A = ChrBankTemp with N/Z set.
- Data alternative: there is no consumer and no table domain fits. islands3.py shows that every small (<=8 byte) Data
  island sandwiched between code in this ROM has a consumer.

**C02 (Temp0C/0D += 18, confidence 0.6)**
- Start bound: the 6-word table $F6B6 is read by $F659-$F66A with X = Temp01*2 and Temp01 < 6 (CMP #6 at $F63E), so it
  ends at $F6C1.
- End bound: RTS at $F6CD, followed by the exact duplicate D63 ($F6CE-$F6D9).
- Semantic link: the constant $12 = 18 equals:
  - the stride of the $F6B6 destination records ($68A7, $68B9, ... $6901);
  - the per-record loop count (CMP #$12 at $F6A5) in the same routine.
- The high-byte step is defective: `ADC $0D` runs with A still holding the new low byte. This is evidence of unused or
  buggy code, not of data.
- Read as words, the bytes give $A918/$6512/$850C/$650C/$850D/$600D. These fit neither adjacent pointer domain, and
  the "third 12-byte table" alternative is refuted by content.

The changed inference relative to the disassembly-003 hold: no new runtime evidence was needed. What is new is that
independent constraints were combined: a consumer-bounded start, a classified end, and sibling or stride semantics.
Before, each run had been judged only as an "effectless/odd fragment with no reference". The disassembly reviewer may
still keep C01/C02 unknown.

## Gameplay (L006; analysis/scenarios.json S26-S27)
- S26 (natural input, no RAM writes, from scratch/snaps/league6_after_game1.json): NEW LEAGUE with 6 teams and 1 game
  per pairing. Snapshot: scratch/snaps/league6x1_made.json.
- S27: all 15 pairings were simulated through the standings grid (l6loop.py). The final standings were LS 4-1 .800 down
  to OK 1-4.
  - This reached GameMode 41 (league end) and the LEAGUE CHAMPION trophy and celebration for LA SUPERSTARS.
  - Pressing A returns to the main menu. The finished league then offers only SEE STANDINGS / NEW LEAGUE.
  - No separate credits sequence was observed.
  - Snapshot: scratch/snaps/league6x1_final_game_end.json.
- Trace aggregates for the session: 6,961 starts (r3-redirect-a/r3-observed.json).
  - 0 starts in any unknown byte, in Data, or in the L025 inline words.
  - 21 Static Code regions gained observed starts, all on statically decoded instruction boundaries
    (r3-static-observed.json). Examples: RecordLeagueResult, StandingsLayoutTry3-6, StandingsNames, DrawMatchCursors,
    MarkChampions, Mode40LeagueEnd, ChampionBanner, ChampionScrollStart, PrintChampionName, WinLeadBiasByOption.
  - This is reachability evidence the disassembly may merge. It does not affect classification.
- L006 is now verified. Every listed menu and progression milestone has been visited, including two league-champion
  endings (S06 and S27).

## Rejection review for the 8-byte residue (unchanged evidence; reviewed against primary artifacts)
| Run | Bytes | Claim tested | Decisive evidence | What it rules out | Strongest competing reading | Disposition |
|---|---|---|---|---|---|---|
| b2 0x0040EA-0x0040EB ($80DA-$80DB) | 2 | code (RTS, RTS) after RTS $80D9, before entry $80DC | refs3.json: no abs/branch/word/word-1 ref in bank 2 or the fixed banks (the $80D9 words are in banks 22/24 own-context only); rtsidiom.json: of the 6 classified RTS;RTS pairs, the second RTS is a branch target in 3 and a $C316 table target in 1; these have neither | a referenced entry or RTS-trick target | padding/fill bytes = generic data; equally plausible 2-byte $60 fill | unknown: policy says an unreferenced return opcode lacks structure on its own; no uniform run long enough to call fill |
| b2 0x005998-0x00599B ($9988) | 4 | code `LDA #$80 / LDA #$00` between jump-table target $9987 (RTS) and target $998C | $C316 table at $9957 = [$9987, $998C] (exact targets, not -1), so neither $9988 nor $998A is an entry; both loads are overwritten by `LDA $0531` at $998C | entry via the known dispatch table | data (two immediate-like byte pairs) vs dead leftover code; no consumer either way | unknown: the role discriminator is missing |
| b2 0x005B58 ($9B48) | 1 | code RTS after EditedLineupPtrs ($9B3C-$9B47, 6 words, team 12..17) | a 7th word would be $A960, outside the $7Fxx domain; no ref | table continuation | lone padding byte vs lone RTS | unknown (single return opcode) |
| b9 0x012175 ($A165) | 1 | code RTS after RTS $A164, before code $A166 | no ref (refs3) | referenced entry | padding vs stray RTS | unknown (single return opcode) |

- The independent constraints were combined for each run: neighbours, decode, references, table domains and the RTS
  idiom census.
- None of these runs has a consumer, a sibling or a semantic link that discriminates code from data, unlike C01 and C02.
- Concrete revised test: none remains, short of a newly discovered reference or an observed fetch.
- Gameplay now covers every menu milestone and both league endings with 0 fetches. Further runtime sweeps have a slim
  prospect.

## Leads (analysis/leads.json, pass redirect-003)
- L021: ready. Apply C01 and C02. The 8-byte residue stays as reviewed above.
- L025: ready. Apply F01.
- L006: verified. The 6-team league completion and its ending are done.
- No new IDs.

## Unresolved questions
- The purpose of C01 and C02, and of the 21 Static routines.
- Whether the residue bytes are padding or dead code. No discriminating evidence is available.

## Deferred work
- Merging the r3 observed starts into map provenance. That is the disassembly's choice; the data is in
  r3-static-observed.json.
- The 8-byte residue: the missing fact is a reference or a discriminating structure.
- After round 4 the ROM will be at most 8 bytes short of complete. Final editorial audit and verification remain required.

## Artifacts (analysis/redirect/r3-redirect-a/)
- Proposals: proposals.json, write_props.py.
- Static scans: fidelity3.py/json, refs3.py/json, rtsidiom.py/json, islands3.py, copies3.py, splitdisp3.py.
- Gameplay: frames/ (nl*, lp*, end_*, end2_*), r3-observed.json, r3-static-observed.json, l6loop.py, live3.py,
  rdaemon.py, rr.py.
- Pre-edit copies: leads.before.json, scenarios.before.json.
- Snapshots: scratch/snaps/league6x1_made.json and league6x1_final_game_end.json.
- Disk: 5.0G free. The /tmp snapshots and store copy were removed.
