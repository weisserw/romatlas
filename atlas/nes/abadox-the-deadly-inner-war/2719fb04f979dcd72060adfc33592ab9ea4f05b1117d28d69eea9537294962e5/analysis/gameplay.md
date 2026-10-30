# Abadox familiarization — round 1

ROM-local title screenshot identifies **Abadox**, Milton Bradley/Natsume ©1989; likely North American release, revision not independently established. Mapper1 128KiB PRG +128KiB CHR.

Research: https://retromaggedon.com/index.php/abadox-nes-cheats/ (accessed 2026-10-30). Transcribed hypothesis: enter stage-select sequence Left, Up, Right, Down, Up, Right, Left, Down on credits/license screen after title; then Up=2, Left=3, Down=4, Right=5, Select=6, B=7 and Start. Alternate A,A,Up,B,B,Down,A,B then Start enables invincibility; Start twice in gameplay grants powerups. Source warns one code at a time. Not yet confirmed on this image. https://cah4e3.shedevr.org.ru/cheatsbase_fix.php distinguishes US license screen from Japanese title screen. Control timing/screen window must be validated locally.

Local confirmed: initial 120-frame idle displays title; Start held 2–3 frames leaves title and selects bank10/11. Gameplay/late stages/ending not yet verified. First bridge replay timeout advanced to frame125; bounded headless timeouts can leave partial state. Use snapshots plus explicit inputs, not elapsed assumptions.

## Locally confirmed routes

Durable `analysis/round1/game-entry.snapshot.json` is the US license screen (after title Start). Restore, explicitly release all buttons; each key held2frames then released2frames. Stage-select8 followed by Up/Down/Select/B and Start successfully initializes stages2/4/6/7 (RAM$003D=2/4/6/7), with distinct gameplay and movement/shooting exercised. Stage1 direct Start also works. These are built-in shortcut inputs, not evidence of normal sequential traversal.

Local handler `$CE3A-$CE81` accepts selector followed by optional **A**, then Start; A sets `$004F=1`, selecting the second stage half. Stage6 Select,A,Start confirms this checkpoint. The published ending recipe `A,A,Up,B,B,Down,A,B,B,Start` merely enters stage7 here. Corrected code-supported route `A,A,Up,B,B,Down,A,B,B,A,Start` enters stage7 half1 with `$05AD=4` and shows an animated spacecraft/planet explosion cutscene. This is verified against this ROM, unlike the published recipe. No terminal credits claimed yet.

RAM$037C=1 is written by the locally decoded invincibility sequence matcher; a stage6 artificial branch patches exactly this flag after restoring a clean snapshot. It scrolls through further enemies; damage immunity itself was not separately stress-tested. Preserve artificial-state distinction.

Round2 local confirmation: controller-only Left/Right selectors from the established license-snapshot unlock route visit stages3/5; RAM$3D and distinct scenes verified, A+B+Right60/release5 exercised. Stage6 source-backed artificial scroll/spawn-cursor threshold patch activates$BE99 script, and scheduler delay acceleration reaches boss-coded kind$40 formations; later script loops at$BEBE. Terminal boss defeat remains unconfirmed, stale layout/terrain backing limits shortcut fidelity. Exact patches/inputs/budgets and observations: redirect/r2-20261030-residual/findings.md, boss-patches.json, scheduler-recipe.json; updated scenarios.json. No new external claims or downloads.
