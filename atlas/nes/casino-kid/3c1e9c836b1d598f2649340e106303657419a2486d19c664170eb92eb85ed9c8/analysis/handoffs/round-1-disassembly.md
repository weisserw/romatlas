# Round 1 disassembly handoff

Scope: one disassembly pass on a fresh workspace (Casino Kid, SOFEL 1989, US; NES mapper 2
UxROM, 8 x 16 KiB PRG, CHR-RAM). Baseline created; full map built and verified.

## Checkpoint / verification
- Final checkpoint: pass-002 -> maps/pass-002.json, generated/pass-002,
  analysis/pass-002-verification.json (ok, byte_identical). Earlier: pass-001.
- Internal store analysis/state.json == maps/current.json (map.validate ok).
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
  Per-bank table in analysis/progress.md ("Round 1 disassembly").

## Findings
- Fixed bank 7: vectors NMI $E5FA (jmp ($07FE) when $0100=$80), RESET $C000 -> $C115,
  IRQ $E608 (RTI). API jump table $C019-$C112; mode entries $C149/$C158/$C167/$C176/$C185/$C19F/$C1B9
  switch to bank 5/6/6/6/4/3/5 entry points ($8010..$801C). Bank select $DEC7 ($14&7 via
  bus-conflict table $DED4). Joypad $E528/$E4E1 ($F5-$F8), NMI frame handler $E609.
- Inline-parameter/table conventions (tools/walk.py, tools/build.py): $D608/$D64A (3 bytes,
  confirmed on 18 executed sites), $D806-prefixed callees (5), bank 3 $90F2(2)/$9110(4),
  bank 4 $965E/$95D7/$969C/$9626/$8B1C (2); jump tables after $C06A/$E463 and $C06D/$E47B
  (count via $19).
- Bank roles (inferred from flow/observation): 4 = poker, 3 = blackjack (static only),
  5 = casino overworld/dialogue, 6 = title/intro/password/ending + music, 0-2 = graphics data.
- Debug strings in bank 5 ($93DF "I AM A RICHMAN", $93FD "I WANNA SEE THE KING") checked with
  A+Start on overworld entry; used (RAM-written password buffer $05DC) to reach the King
  poker match and, with King money $075B patched, the ending credits.
- Static code prediction validated: 0 conflicts between 8250 observed opcode starts and pass-001.

## Unresolved questions
- Opponent selection/location (bank 5 $A524-$A6A8, $055B/$055C/$0569-$056C) -> natural
  blackjack/poker opponents not reached (lead L1).
- 15042 Code bytes are Static-only (bank 3 entirely). Inline contracts outside $D608/$D64A not
  yet confirmed by execution (L3).
- Unreferenced code fragments (L7) and far-call trampoline $C070/$E492 (L4).
- Generic data semantics: banks 0-2 graphics, fixed-bank tables $C3D7/$CC25/$D348/$D9F1,
  envelope-like tables $ED7D-$F7B8, bank 6 music data (L5, L6).

## Deferred work
- Commentary: labels/explanations for 1493 regions (only ~20 labels exist).
- Blackjack scenario; natural password typing (vertical cursor moves unreliable with short presses).

## Artifacts
- analysis/progress.md, analysis/investigation.md, analysis/scenarios.json, analysis/leads.json,
  analysis/gameplay.md, analysis/toolkit-issues.md
- traces/coverage.json; snapshots traces/snapshots/{casino,pw,kinggame,poker2}.json (same build)
- Workspace tools: tools/{daemon,rpc,play,route,cov,walk,m6502,tables,build,compare,gaps,codescan,mkmap,ldis,disx,rdwho}.py,
  tools/routes.json (reproducible input routes). Rebuild map: python3 tools/mkmap.py -> scratch/newmap.json, then map.load.
