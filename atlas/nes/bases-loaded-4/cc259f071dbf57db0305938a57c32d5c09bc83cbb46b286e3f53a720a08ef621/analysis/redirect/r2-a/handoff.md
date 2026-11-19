# Redirect handoff - round 2, attempt r2-a (2026-11-18) - STATUS: INCOMPLETE-NO-READY-LEADS (rejection review)

## Scope and checkpoint
- Input: checkpoint pass-002 (maps/current.json = maps/pass-002.json; 100% classified, 0 unknown;
  analysis/pass-002-verification.json byte_identical) and commentary round 2
  (analysis/commentary-handoff-round2.md). Prior redirect: analysis/redirect/r1-a/handoff.md.
- Changed only research records: analysis/leads.json (pass entry; attempts appended to L005,
  L011, L014), analysis/scenarios.json (S11), analysis/progress.md, this directory
  (leads-before.json / scenarios-before.json are the pre-edit copies). ROM, map, state,
  source, commentary and toolkit untouched. Proposed bytes 0; accepted bytes unchanged.
- Exact unknown complement: empty (0 bytes); no ledger refresh needed.

## (a) Contradiction audit - methods and results
1. `tools/consistency.py` -> `consistency.json`: decodes all 1,164 Code regions (29,921
   instructions) with their bank context and checks every jsr/jmp/branch target, every
   absolute read operand >= $8000, jmp (ind) vectors, region-end fall-through, the vectors
   and all 17,278 round-1 observed fetch offsets (cpu alias + bank context).
   Result: 0 undecodable/unofficial opcodes, 0 decode overruns, 0 flow targets into
   Data/Padding/mid-instruction/unresolved, 0 absolute reads into Code, 0 fetches in non-Code
   or mid-instruction, 0 cpu/bank mismatches. 55 region ends without terminal flow: 50 are
   `jsr JumpTableDispatch[Zp]` inline tables, 5 are always-taken branches (adc #$0C/#$08 after
   U26_VectorOctant -> bne; bcs/bcc pairs at unit 28 $8971/$A0A3; lda #0/beq $9171).
2. `tools/inline_tables.py` -> `inline-tables.txt`: all 50 inline tables are whole words, every
   word a code start in the same bank context; only the unit-28 9-word table is followed by
   Data (closed by the `and $9746,x` operand, L009).
3. `tools/farcalls.py` -> `farcalls.txt`: 59 static far calls; 58 resolve to code starts via
   the target unit's $8001 entry table. The exception is the known U20_FarCallUnit18Entry4
   (index 8 -> bytes `A9 00` of U18_LoadSceneRecord = $00A9): the unit-18 table is not a
   5th word (its value would be a zero-page address), so this confirms rather than
   contradicts the 4-word table. Unit-28 entry 7 ($84D8 sine table) and unit-22 zero slots
   are the previously recorded L003/unused cases.
4. `tools/imm_ptrs.py` -> `imm-ptrs.json`: immediate lo/hi pairs pointing into Code
   mid-instruction (14) were each inspected: all are VRAM addresses ($21CE, $21C3, $21D3),
   metasprite X/Y coordinates ($80/$B8, $A0/$D0) or limits ($8C/$98), not pointers.
5. DPCM: DmcSampleTable's 10 records (addr $C000+64v, len 16L+1) span exactly the bank-30/31
   DmcSample_* regions. Sample 0/8 starts at $D5C0, inside observed code $D5B2-$D5CD (dual
   use, already recorded in the map evidence: "overlap executed code and stay code").
   Sample 6 ends at $FD40, so $FD41-$FD4D (L011) lies outside every sample: no contradiction.
6. Commentary findings re-checked: unit-12 U12_QueueBlankColumn ($30 terminator) and unit-18
   bypassed fade-out are coherent closed code with semantic oddities only.
7. Data-read probe: ROM Read events are only available as a full bus stream (opcode/operand
   reads included, ~70k/frame), so it is usable only on code-free spans. Bank-27 span
   0x36768-0x38010 (6 A2 runs) over 600 watch-game frames: 0 reads
   (`readspan-bank27-watch600.json`); a partial menu-phase stream of the bank-21 span read
   482 data bytes, none in its two A2 runs. Neither positive nor contradicting evidence;
   cost ~0.35 s/frame/span, so not extended.

## (b) Gameplay milestones
- S11 (new, natural input, no RAM patch): valid password entry. `tools/password.py` models
  U16_EncodePassword/DecodePassword/Checksum; `tools/pw_route.py` enters 000D6BD640 with
  Up/Right presses -> NEW YORK SUPER SERIES, GAME 1 NY vs HAWAII, batting order. First
  observed fetches in 10 Static code regions (1,236 B, incl. U16 $B1A0 accepted path, $B750
  series check), all inside Code (`s11-coverage.txt`, `evidence/`). Glyph indices 10=R, 11=D,
  12=B, 14=C; the r1-a game-generated 000R6CB640 corresponds to ScheduleDay 0.
- Remaining checklist: title, menus, options, exhibition 1P/2P/watch, season game, invalid
  and valid password, game over, pennant, series, champion/terminal screen, sub-mode 19 are
  all visited. No unvisited representative milestone with a classification prospect remains.

## (c) Rejection review (strongest candidates)
| Candidate claim | Decisive evidence | What it rules out | Competing interpretation / why no proposal |
|---|---|---|---|
| $FD41-$FD4D is code or part of a DPCM sample (L011) | DmcSampleTable extents; consistency.json (no flow target, no fetch); reset at $FD4E is `78 D8` | sample membership; any control-flow entry | Only generic data remains; no consumer, so specific role stays unproven. No change. |
| L014 A2 runs (1,002 B) hide code | r1-a closure test + r2-a: no flow/operand/fetch reference; Data on both sides; 0 reads in tested scenarios | referenced or executed code | An unreferenced fragment entered through an undiscovered computed pointer; no such pointer exists in inline/entry tables, far calls or immediate pairs. No revised test with a discriminating result. |
| 42 L012 routines + unit 2/6 copies are data | closed decodes, same RAM/helper conventions (r1-a verify-proposals.txt), 0 inconsistencies here | data reading | Reachability unproven only; structural claim sound. |
| Unit-18 entry table has a 5th entry | entry word would be $00A9 (RAM) | 5-word table | Dead far call in unit 20; table stays 4 words. |
| Data reads into Code | 0 absolute-indexed operands into Code; immediate pairs are coordinates/VRAM | table overlapping code | Indirect reads via unknown pointers stay unobservable without a full read trace; DMC $D5C0 overlap is the only dual-use and is recorded. |

## Editorial items for final commentary audit (no disassembly change)
- fixed_30_31.asm DmcSample_D5C0..F9C0 headers still say "Generic data ... no direct consumer
  found"; Bank31Data_E000 says "no real consumer". Map evidence (and the table decode above)
  shows they are DmcSampleTable samples, $E000-$E0BF being the tail of sample $DAC0-$E0C0.
- Password glyph letters (10=R, 11=D, 12=B, 14=C) can replace the uncertainty in the U16
  password commentary; U16_PasswordGlyphs layout otherwise unchanged.
- Optional: S11 fetches could upgrade provenance notes for the 10 regions listed in
  s11-coverage.txt (partial coverage; not a classification change).

## Another disassembly cycle?
No. No structural contradiction, boundary or bank-context error was found and no ready lead
exists. Remaining work is the final editorial cleanup/audit (generic @Skip_/@Loop_ labels,
numeric pointer immediates, the DMC prose above) and verification.
