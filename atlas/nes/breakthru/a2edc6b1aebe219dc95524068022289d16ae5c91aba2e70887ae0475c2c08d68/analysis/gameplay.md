# BreakThru gameplay familiarization — round 001

Local title screen identifies BREAKTHRU, ©1987 Data East USA. USA-like revision
suggested by title; exact dump revision not independently identified. ROM SHA-256
is workspace.json identity. Do not use unrelated 1990s puzzle-game BreakThru! cheats.

External hypotheses consulted 2026-11-21:
- https://www.digitpress.com/library/manuals/nes/Breakthru.pdf — NES manual exists;
  PDF text extraction unavailable. No unconfirmed manual mechanics used for classification.
- https://www.youtube.com/watch?v=lIrQWpDO7Kc — NES full-run description says speed
  affects autoscroll and five stages loop with short interlude. Terminal-loop claim
  pending local confirmation.
- https://github.com/FrodeSolheim/cheats/blob/master/nes.cht — listed USA cheat $b2=9
  lives. ROM listed MD5 differs from this headered dump; local initialization/death
  code independently confirms $b2 as lives. RAM edits are artificial evidence only.

Locally confirmed: Start enters selected single-player game; title input must be
held long enough for polling. Direction right with A/B produces moving/firing and
jumping vehicle gameplay. Stage1 MOUNTAIN, stage2 BRIDGES, stage3 PRAIRIE intro,
stage4 city scenery. Direct main-loop continuation at CPU$c0c0 uses $8f=stage,
$90=checkpoint; maps PRG from stage and CHR from stage-1, calls$c2d0 initialization,
then gameplay$c1bc. This is a supported artificial shortcut, not natural progression.
$9a is frame parity/counter, not stage. $ab is rendering/list handoff state, not stage.
The former intro route$c095 does initialize the right intro card, but 312 frames
remained in timing/display loop. Direct$c0c0 bypasses that delay with established
setup contract. Use clean game snapshot and reassert buttons after import.

Round002 redirect confirms an artificial post-ending restart shortcut: restore the
saved aircraft-animation snapshot, release inputs and patch documented completion
latch$9E=1, retaining the current PC/stack. Six bounded frames produce fresh
$C12F..$C13C restart fetches, stage1/checkpoint0, PRG[0,1,14,15], and stage-map screen.
See redirect/round002-residual-roles/results.md and focused trace recipe. This
bypasses aircraft completion timing; natural ending completion and second-loop
playthrough remain unclaimed. Existing broad gameplay coverage need not repeat.
