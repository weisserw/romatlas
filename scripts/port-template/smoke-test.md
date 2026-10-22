# Initial port smoke test

Before each milestone handoff, run the applicable steps of this smoke test against
the native game. Use the device-free debug API (`python3 run.py debug`) with
bounded input/advance/capture calls. This uses the control API only; it does not
invoke the debug subprocess. Document steps requiring future milestones as
deferred, with their dependencies; do not mark them passed. The full route becomes
required once its dependencies are implemented, and before final implementation
completion. A limited early milestone is not blocked by unimplemented future steps. Do not launch SDL or
require physical controllers/audio devices on the headless server.

Choose one representative route appropriate to the game and record its inputs,
checkpoints, observations and local evidence in checks/smoke-test.md:

1. Boot to the title screen. Check its layout, graphics and colors against the
   matching original ROM, using a few corresponding reference observations.
2. Use the menus that lead into play: verify navigation, selection and starting
   the game respond correctly. Do not exercise every setting or mode.
3. Check the initial gameplay area's palette/colors and major visible elements.
   Exercise basic player movement and the game's primary action, a representative
   enemy/hazard interaction, and damage/death where applicable. For games without
   enemies, use the equivalent core interaction, such as ball/block collisions.
4. Continue until exiting the first level/area or a normal player death/game-over,
   whichever is appropriate to the game and the chosen route. Verify that the
   resulting transition works. A full playthrough or later-area coverage is not
   required.

Fix concrete defects exposed by this route, then recheck the failed and affected
steps. Reuse valid observations; do not restart unchanged successful checks to
accumulate confidence. Once the route passes, stop smoke testing. Do not expand it
into every animation frame, pixel-perfect comparisons, an exhaustive collision or
menu matrix, audio equivalence, later levels or a second full QA pass. Use bounded
experiments for diagnosis, changing the hypothesis/method after inconclusive runs.
If a required step remains broken or unavailable, report the concrete blocker and
leave completion pending; a build passing or a documented assumption cannot replace
execution of this smoke test.

Absence of hardware-induced lag, slowdown or sprite flicker is not a bug. Preserve
intentional timing and effects such as deliberate slow motion or invulnerability
flashing. Judge title/graphics/color accuracy from representative visible results,
not a requirement to prove every pixel or frame identical.

Record the native build and ROM identity, the route/end condition, checks performed,
results, fixes/rechecks and remaining out-of-scope limitations. Keep ROM bytes and
captures local. This smoke test establishes basic startup/playability and visible
accuracy on this route, not whole-game fidelity. After it and the required unit,
format/static/build checks pass, finish accounting/handoff and the administrative
audit, then stop.
