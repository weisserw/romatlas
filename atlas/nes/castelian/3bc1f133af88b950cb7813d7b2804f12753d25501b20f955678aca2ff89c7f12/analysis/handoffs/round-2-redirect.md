# Round 2 redirect handoff (attempt r2-a1)

- Status: COMPLETE. ROM still incomplete: 131086/131088 classified (2 unknown bytes).
- Scope: L2-01 residual bytes (terminal rejection review), static-only code reachability
  ($F4FD, $A019/$A0EC, $E4B3, $E585), bank 3 $A500-$AEFF, unvisited gameplay milestones.
- Input checkpoint: pass-002 (maps/pass-002.json sha256 9a7a6642...837f0f5 == analysis/state.json,
  571 regions). Commentary gate met (analysis/handoffs/round-2-commentary.md, byte-identical).
- No map/state/source/ROM changes; runtime on scratch store copies with the configured binary.
  Research dir: analysis/redirect/r2-a1/ (notes.md, tools/, evidence/, run logs). Prior attempt
  r1-a1 untouched.

## Findings
1. L2-01 (2 bytes) - terminal rejection review in analysis/redirect/r2-a1/notes.md. New checks:
   whole-ROM word/RTS-table scan (no reference), sibling routines (only PrintPlayersOption, of
   three identical tail-jump routines, is followed by `$60`), precedent scans (no accepted dead
   RTS after JMP, no NOP/RTS pad or alignment convention anywhere; $B685 is the only `$EA` at a
   region boundary), adjacent-format membership (`$60` is not a member of the `$00`-terminated
   OptionStrings; `$EA` is not a defined sound command and follows the last pattern's `$FF`).
   Code (dead rts/nop) vs data (pad) both predict an unexecuted byte; nothing discriminates.
   Unknown retained per evidence-policy (lone valid opcode, no uniform/visibly non-code content).
2. $F4FD PLAYER 2 WINS: visited (S13b, artificial score patch); `iny` at $F4FD fetched;
   screen evidence/s13b-player2-wins.png.
3. New milestone S14 HERO game (options by buttons, then score/lives/timer patches): HERO timer
   speed, no-continue game over and `jmp CheckExtraLifeHero` ($CDB1) executed; 6 Static code
   regions fully fetched, 9 partially (evidence/s14-hero-coverage.json).
4. Orphan code $A019/$A0EC, $E4B3-$E4CC, $E585-$E590: no caller/RTS-table entry found by word
   scan; structural Code support unchanged; no correction warranted (L1-06 attempt appended).
5. Bank 3 $A500-$AEFF: rerun ref/immediate-pair scans give only false positives; classification
   (generic Data) unaffected by any reader; no change.

## Ready leads (analysis/leads.json, pass r2-a1)
- L2-02 (low, provenance only, 0 bytes): add artificial-scenario Observed evidence for fully
  fetched Static Code $F4FD, bank 3 $BD1E-$BD2A, bank 4 $97D3-$9840, fixed $C753-$C75F,
  $CDB1-$CDB3, $DDDB-$DDDD, $DE05-$DE07; optionally split 9 partial regions. Recipe:
  `python3 analysis/redirect/r2-a1/tools/covcheck.py <coverage.json>` on the saved coverage lists.

## Rejected / deferred
- L2-01: blocked (terminal for both bytes) - see rejection review table; reopen only with a new
  reference, original listing or ROM-wide convention.
- Reachability of $A019/$A0EC, $E4B3, $E585: negative evidence (no caller/selector exists to
  trace); further gameplay would be arbitrary state search.
- $A500-$AEFF reader watch: cannot change classification; no unvisited mode remains.
- BottomChain2400 entry 0 ($6391): semantic question only.
- Gameplay: all checklist milestones visited (title, options, towers 1-8, pause, time out,
  continue/game over, tally, bonus stage, ending, 2P drawn/P1 wins/P2 wins, attract, HERO); no
  high-score name entry exists.

## Recommendation
Classification is terminal-incomplete at 2 unknown bytes (phys 63168 bank 3 $B6B0, phys 79509
bank 4 $B685) with the rejection review above. No classification proposal remains; another full
disassembly/commentary/redirect cycle is not justified for bytes. L2-02 is optional provenance
polish (no byte gain): apply it in a light disassembly touch-up or fold into final acceptance;
then proceed to final editorial audit/verification.
