# Redirect attempt r2-a1 (round 2)

- Input checkpoint: pass-002 (maps/pass-002.json == analysis/state.json, 571 regions;
  131086/131088 classified, 2 unknown). Commentary gate: analysis/handoffs/round-2-commentary.md
  (COMPLETE, byte-identical). Prior attempt analysis/redirect/r1-a1/ reused (immutable).
- No map/state/source changes. Runtime used scratch store copies with the configured binary
  (/root/romatlas/target/release/romatlas); map.save never called.

## Tools (run from the workspace root)
- tools/survey.py      - ROM-wide contexts of `4C xx xx 60`, RTS/RTI+`60`, and `$EA` at region
                         boundaries vs current classification.
- tools/deadstarts.py  - classified Code instruction starts with a non-fall-through predecessor
                         and no decoded branch/JMP/JSR target (precedent test for a dead RTS).
- tools/covcheck.py    - per Static Code region: decoded instruction starts fetched in a saved
                         coverage list (FULL/partial).
- tools/s14_hero.py    - S14 HERO-difficulty run (options via buttons; then score/lives/timer patches).
- tools/s13b_p2wins.py - S13 variant with P2 score > P1 (PLAYER 2 WINS).

## Unknown complement (refreshed from analysis/state.json, pass-002)
| phys | bank/CPU | byte | left neighbour | right neighbour | disposition |
|---|---|---|---|---|---|
| 63168 | bank 3 $B6B0 (ctx [6,7,14,15]) | $60 | Code Observed, ends `jmp PrintBigText` ($B6AD, PrintPlayersOption) | Data OptionStrings $B6B1 | unknown retained (review R1 below) |
| 79509 | bank 4 $B685 (ctx [8,9,14,15]) | $EA | Data sound region $AD38-$B684 (last pattern ends `$FF` at $B684) | Code Observed CopyRamTileRoutine $B686 | unknown retained (review R2 below) |

## L2-01 rejection review (terminal for these two bytes)

Checks performed this attempt (new relative to r1-a1/round-2 disassembly):
1. Little-endian word search for $B6B0/$B6AF/$B685/$B684 over the whole ROM: the only hit
   ($B685 at phys 0x1F001) is the operand pair of `sta $B6` + next opcode inside observed code
   ($EFF0) - coincidental. No pointer, RTS-table (target-1) or JMP/JSR/branch reference.
2. Sibling-routine convention (bank 3): PrintSoundOption (jmp at $B677) and PrintDifficultyOption ($B67A, jmp at $B692)
   end `jmp PrintBigText` and are immediately followed by the next routine; PrintPlayersOption
   ($B695) is identical in shape and is the only one followed by an extra `$60`. All three
   `4C 17 FD` occurrences in the ROM are these routines.
3. Precedent for a dead RTS after a tail JMP (tools/survey.py, deadstarts.py): every other
   `4C xx xx 60` in classified code is either a branch-over-JMP idiom or an observed fetched RTS;
   no accepted Code region contains an unentered `$60` after JMP/RTS. Accepted unentered starts
   (deadstarts.py, 53) are all region starts of entry-table/jump-table/dispatched code, not
   stray return bytes. So the map has no "jmp+rts" authoring convention to appeal to.
4. Precedent for a lone pad byte: the only `$EA` at a region boundary in the ROM is $B685
   itself; no NOP/RTS padding or alignment convention exists elsewhere; neither $B686 nor
   $B6B1 is aligned to any power of two above 2.
5. Adjacent-structure membership: OptionStrings are raw ASCII, `$00`-terminated, with every
   pointer target ($B6B1,$B6B3,$B6B5,$B6BD,$B6C5,$B6CC,$B6D3,$B6D9) a string start; `$60` would be
   an unreferenced one-character prefix without its own terminator, so it is not a member of the
   string table format. In the sound grammar (Ch1_ReadByte) `$EA` is in the $80+ command range
   but is not a defined command ($F4,$F6-$FF); the last pattern ends at $B684 with `$FF`, and no
   order list points to $B685, so it is not a member of the walked pattern format. (Contrast the
   accepted $B67A-$B67C `7F 11 FF`, which is note/duration/terminator-shaped and lies *between*
   walked patterns inside the consumer-bounded region.)

| | R1: bank 3 $B6B0 `$60` | R2: bank 4 $B685 `$EA` |
|---|---|---|
| Candidate claims | (a) Code: dead `rts` left after tail-call `jmp`; (b) generic data/pad prefix of OptionStrings | (a) Code: unreachable `nop` before CopyRamTileRoutine; (b) generic data/pad after the sound data |
| Decisive evidence | ROM bytes $B695-$B6B1 (`.../4C 17 FD/60/31 00`), sibling routines, word/branch scans above | ROM bytes $B67A-$B68B (`.../F8 FF/EA/A2 25 BD...`), soundwalk (r1-a1/tools/soundwalk.py: covered 2378/2382, gaps $B67A-$B67C and $B685) |
| What it rules out | Any static reference or fall-through entry (prev insn is JMP); membership in the `$00`-terminated string format | Any static reference or fall-through entry (prev byte is data); membership in the pattern grammar |
| Independent constraints combined | no entry + no sibling precedent + non-member of the data format + no ROM-wide jmp/rts or pad convention | no entry + non-member of grammar + no ROM-wide NOP-pad or alignment convention |
| Strongest competing interpretation | Source-level stray `rts` after converting `jsr PrintBigText/rts` to a tail jump (the code reading) vs. a 1-byte pad/residue (the data reading). Both predict an unexecuted `$60`; nothing in the ROM distinguishes them. | Hand-placed `nop` (code reading) vs. leftover/pad byte (data reading); both predict an unexecuted `$EA`. |
| Why the inference is insufficient | Policy: an unreferenced return opcode lacks code structure on its own; generic data needs repeated/uniform/visibly non-instruction content or format membership - a lone valid opcode has none of these, and lack of execution alone does not prove data. | Same: single valid 1-byte opcode, no format membership, no uniform run, no convention. |
| Revised test considered | Runtime watch cannot help (both readings predict no fetch/read); a cross-bank duplicate of the routine/byte layout does not exist (unique `4C 17 FD`, unique `A2 25 BD`); no original listing available. | Same; the RAM-copy routine and sound data are unique in the ROM. |
| Outcome | Unknown retained; no further discriminating test exists. | Unknown retained; no further discriminating test exists. |

Residual: exactly 2 bytes (phys 63168, 79509). This review uses unchanged premises from the
round-2 disassembly handoff plus the new negative scans 1-5; reuse it unless a new reference,
an original listing or a ROM-wide convention is found.

## Other avenues
- Orphan static code $A019/$A0EC, $E4B3-$E4CC, $E585-$E590 (L1-06, conf 0.6 for $E4B3/$E585):
  word scan finds no `jsr/jmp` to $A019 (other $A010-$A022 slots are all called from the fixed
  bank) and none to $E4B3/$E585 or RTS-table form; the `$E585` byte pairs in bank 6 are
  `sta $E5` instructions. Structure (closed control flow, use of DoorEnterFramesLeft/Right,
  ExitFlag, twin of observed PlayerState_Door code; uniform 3-byte jmp table slot) still
  supports Code; no correction warranted. Reachability: none found; S14 HERO run did not fetch
  them. Deferred: no caller/selector exists to trace (negative evidence), no further method.
- Bank 3 $A500-$AEFF (Data by pattern): immediate-pair and absolute-ref scans (r1-a1 imm.py /
  refs.py rerun) find only false positives (fixed-bank stores of constants into RAM records and
  calls made with other banks mapped). A reader would only add a role; the Data classification
  would not change. Deferred: low value; a read watch in an unvisited mode is the only method and
  no unvisited mode remains (see scenarios).
- BottomChain2400 entry 0 ($6391) - commentary semantic question; classification unaffected.

## Runtime
- S14 HERO difficulty (log s14-run.log; coverage evidence/s14-hero-coverage.json): options
  navigation by buttons (Down x3 + Select toggled HeroMode $40=$FF; the Down x1 Select also
  toggled PLAYERS to 2 - so this is a 2-player HERO game); TimerSpeed $45=$2D confirmed. Traced
  from game start + ~240 frames of bounded inputs after writing P1 score $0C-$0E=50,99,00
  (artificial), then lives $12=1 and timer $41-$43=00,00,02 (artificial) -> TIME OUT -> P1 game
  over with no continue prompt (HERO). 15 Static code regions gained fetches; 6 fully fetched:
  bank 3 $BD1E-$BD2A (hi-score copy), bank 4 $97D3-$9840, fixed $C753-$C75F (RasterDelayB),
  $CDB1-$CDB3 (`jmp CheckExtraLifeHero`), $DDDB-$DDDD (`jmp StartFallStraight`),
  $DE05-$DE07 (`jmp StartJump`). Partial: bank 4 $9137/$98FE (104/364 each), bank 5 $A4DA
  (31/33), fixed $CE34 (45/69), $D1EC (27/112), $D8A3 (22/45), $E27C (43/273), $E665 (3/14),
  $E6E2 (13/22). Run `python3 analysis/redirect/r2-a1/tools/covcheck.py
  analysis/redirect/r2-a1/evidence/s14-hero-coverage.json` for the exact list.
- S13b PLAYER 2 WINS: see section appended below.

## S13b PLAYER 2 WINS (appended)
`python3 analysis/redirect/r2-a1/tools/s13b_p2wins.py` (scratch store copy; first attempt was
killed by its 1500 s wall-clock timeout during the traced window after reaching the screen; rerun
with the final 300 traced frames dropped completed, exit 0). Screen evidence/s13b-player2-wins.png
("PLAYER 2 GAME OVER ... PLAYER 2 WINS"); $F4FD fetched (s13b-run.log,
evidence/s13b-p2wins-coverage.json). Also fully fetched: bank 3 $BD1E-$BD2A. Artificial scenario
(score/lives/timer patches); provenance-only lead L2-02.
