# Gameplay research

Game identified locally from the title screen (round 1): Castlevania (Konami, 1987, "Licensed by
Nintendo of America" US release), UxROM 128 KiB PRG, CHR-RAM. No online sources consulted in round 1
(not needed for the easy-gain pass); everything below was confirmed locally unless marked hypothesis.

## Controls (confirmed locally)

- Title: Start begins the game (state $18: 1 title -> 3 -> 4 -> 5 gameplay, about 10 s to control).
- Gameplay: B = whip, A = jump, Up+B = sub-weapon, D-pad moves (Down = crouch). Standard Castlevania.
- Toolkit note: press buttons with `emulator.input {"buttons":[...],"port":1}`; the documented
  `button`/`pressed` form is silently ignored (analysis/toolkit-issues.md TK-NES-001).

## RAM contract (confirmed locally unless noted)

| Address | Meaning | Evidence |
|---|---|---|
| $18 | game state, dispatched by GameStateDispatch ($C1A1) via JumpTableDispatch ($CA1E) | 1 title, 2 attract demo, 3/4 game start, 5 gameplay; table at $C1B1 has 16 handlers |
| $19 | sub-state within $18 | 6 during normal play |
| $1A | frame counter | incremented each NMI at $C1A1 |
| $24 | saved 16K PRG bank, restored by BankRestoreSaved ($C193) | set to 6 at reset |
| $28 | stage number (block index); HUD shows it as STAGE nn | write + death reloads that stage |
| $2A | lives (HUD shows P-nn) | decremented on death |
| $45 | player health (64 = full) | writing 0 kills Simon |
| $04/$05 | raw joypad 1/2 bits (A,B,Sel,Start,U,D,L,R from bit 7) | ReadJoypads $C87E |
| $F5 | new-press bits pad 1 (hypothesis from $C8CD use) | not yet confirmed |

## Shortcuts

- Stage warp (artificial, confirmed): from a stage-1 gameplay snapshot write $28 = N, $45 = 0,
  $2A = 4, then run ~200 frames: the death sequence reloads stage N with dependent state initialised.
  Recorded in analysis/scenarios.json. This is artificial reachability, not natural progression.
- No password system or built-in stage select is known for this game.

## Open hypotheses

- Boss rooms are at the end of stages 3, 6, 9, 12, 15 and 18; reaching them needs a position/checkpoint
  RAM contract not yet identified.
- Unvisited game states $18 = 6..15 (death, game over, continue, ending/credits) can be probed with
  bounded artificial state writes; see scenarios.json.

## Round 2 additions (confirmed locally, artificial setups)

- Boss-room contract: StageCameraLockTable ($FC3A, 19 bytes, high nibble = StageHalf, low = page)
  gives the boss room for stages 3 ($02), 6/15/18 ($10), 9 ($05), 12 ($03); CheckCameraLimits sets
  CameraLocked $48 when the camera reaches that page at offset 0.
- Position warp: write $46/$2F/$2E plus Simon ObjY $0354/$3F, ObjX $038C, world X $40/$41, then
  GameState $18=5 sub $19=0 with $4A=5/$1F=5 (the stair half-change restart): the screen redraws at
  the new camera position. Approach the lock page from just before it so columns/spawners stream in.
- Correct-graphics stage warp: write $28=N-1 during GameState $0C (stage clear); the clear sequence
  loads stage N with its graphics set (the death-reload warp keeps the old CHR set).
- BossHealth $01A9 (=$40 full) can be capped at 1; FinalBossPhase $7A marks Dracula's second form.
- After Dracula: GameState $0C -> $0F ending (castle collapse, credits ~3600 frames) -> second quest
  (Quest $2B=1, stage 0). Recipes: analysis/scenarios.json SC-BOSS3, SC-WARP-CLEAR, SC-BOSS18, SC-ENDING2.
