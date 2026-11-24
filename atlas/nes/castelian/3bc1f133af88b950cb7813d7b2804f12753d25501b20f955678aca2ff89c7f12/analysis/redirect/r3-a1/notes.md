# Redirect attempt r3-a1 (round 3)

- Input checkpoint: pass-003 (maps/pass-003.json == analysis/state.json, sha256 9baf6bef...e17b6,
  599 regions; 131086/131088 classified). Commentary gate: analysis/handoffs/round-3-commentary.md
  (COMPLETE, byte-identical). Prior attempts r1-a1, r2-a1 reused read-only.
- No map/state/source/runtime changes; no emulator runs (none discriminating, see below).
- input/game.nes sha256 3bc1f133... unchanged. Residual bytes re-read: phys 63168 context
  `4C 17 FD | 60 | 31`, phys 79509 context `F8 FF | EA | A2 25` (identical to r2-a1).

## What changed since r2-a1 (and whether it bears on any structural question)
1. Round-3 disassembly (L2-02): provenance Static->Observed and region splits only; no kind/bounds
   changes near $B6B0/$B685 or the orphan routines. The newly observed regions (S13b/S14, artificial)
   do not include $A019/$A0EC/$E4B3/$E585 or bank 3 $A500-$AEFF (r2-a1 coverage). No bearing.
2. Round-3 commentary: prose corrections (game-logic phase, gameplay NMI, PlayerState_Walk,
   SetupTower, game-over screen), ~70 address->name substitutions, 11 trampoline renames. These
   correct descriptions of *already-observed* code; none introduces a new caller, selector,
   pointer or reader. uncertainties.md round-3 additions ($01F0/$0214 write-only, InitTowerVars06B3,
   static-only HERO award etc.) are semantic and concern classified bytes only.
3. Fact surfaced by commentary (pre-existing, now documented): `BottomChain2000_EntryLo` ($9F8B
   bank 4) is an executed RTS whose byte doubles as entry 0 of a split lo-byte table. Considered as a
   possible "convention" for $B6B0: it does not apply - that $60 is both fall-through-executed and
   read by an indexed table load; $B6B0 has neither (previous insn is JMP; no table/pointer covers it).

## New negative check this attempt (method not used by r1-a1/r2-a1)
r1/r2 word scans and immediate-pair scans cannot see split hi/lo tables. Enumerated every indirect
control transfer in the commented source and its operand source:
- `jmp (vector)` writers: NmiVector, BonusFadeVector, DrawVecSolid/Trigger/Row/Lift/Special/Break -
  all from immediates or from SolidDraw/LiftDraw/SpecialDraw lo/hi tables, TriggerCheckEntry,
  ScriptPosCheckEntry; none yields $A019/$A0EC/$E4B3/$E585/$B6B0/$B685.
- All 7 `pha/pha/rts` dispatches decoded (target = entry+1): CopyRowsTop2000/2400 (12 each),
  BottomChain2000/2400 (11 each), BonusStateDispatch (8), RasterEffectDispatch (16),
  PlayerStateDispatch (15 words). Targets: $829B..$8939, $89D0..$906E, $9137..$9C61, $6391/$98FE..$9E68,
  bank 6 $83D0..$8650/$BB23, $C746/$C753/$C760, $DDC9..$EE8C. No orphan or residual address
  (nor target-1 form) appears.
Result: confirms "no caller/selector" for $A019/$A0EC/$E4B3/$E585 and "no reference" for the two
residual bytes under a method that covers split tables. Premises of r2-a1 unchanged.

## L2-01 rejection review
Reused unchanged from analysis/redirect/r2-a1/notes.md (section "L2-01 rejection review"). Premises
re-checked: ROM bytes identical; neighbours unchanged in pass-003 (Code Observed PrintPlayersOption /
Data OptionStrings; Data sound region / Code Observed CopyRamTileRoutine); no new reference (split-table
check above); no new convention (BottomChain2000 RTS/table overlap is not analogous: entered and
referenced); no original listing. Both readings (dead opcode vs pad byte) still predict identical
runtime behaviour, so no runtime test discriminates. Outcome: unknown retained, terminal.

## Other avenues (dispositions)
- Orphan static code $A019/$A0EC, $E4B3-$E4CC, $E585-$E590: structure unchanged (Code, Static);
  reachability none - now also excluded from every split/RTS/indirect dispatch table. No method left.
- Bank 3 $A500-$AEFF (Data, 40x64-byte bitmap pairs): no reader; already classified Data, a reader
  would only add a role. No new mode/state to watch (all scenarios.json milestones visited). Deferred.
- $6391 BottomChain2400 entry 0, SoundMisc+6, $01F0/$0214, InitTowerVars06B3, TowerTextPalette 9th
  byte: semantic questions on classified bytes; no classification impact.
