# Amagon familiarization — round 1

Title capture identifies Amagon, American Sammy copyright 1988; US manual
NES-M5-USA appears applicable, exact revision not externally established.
Sources consulted 2026-11-05:
- https://www.world-of-nintendo.com/manuals/nes/amagon.shtml (manual transcription)
- https://www.neoseeker.com/nes-hardware/faqs/139429-nes-game-endings.html
- https://tasvideos.org/Forum/Topics/12052?CurrentPage=1&Highlight=294299

Actionable source hypotheses: A jumps, B fires; Select transforms after a Mega-key
and sufficient score; Up+B fires Megagon laser. Twelve scenes in six zones.
Continues only unlocked after reaching Zone 4. Scene-end requires enemy clearance
or a boss. Final 6-2 boss defeat should lead to rescue boat/fireworks and terminal
text. Runtime title and Zone 1-1 map transition confirmed so far. Later stages and
ending remain unconfirmed until scenario evidence records them.

Important replay API correction: NES uses emulator.input {port:1,buttons:[...]}
for the full current held set and [] to release. button/pressed is silently ignored.
See toolkit issue NES-001. Early documented-field runs stayed on title and are
not evidence of gameplay or of unresponsive game controls.

## Confirmation update at coordinator terminal preparation

The preceding familiarization notes are the historical pre-progression record.
The completed [scenario ledger](scenarios.json) now records twelve visited milestones
and four deferred same-zone scene permutations (3/5/7/9), with no visit claimed for
the latter. Natural title/Start/early gameplay is separated from artificial shortcuts.
Supported scene-initializer RAM patches reached representative later zones and final
boss arena with action inputs. Final-boss substate/animation patches reached the
rescue boat and state18 terminal screen; terminal-later.ppm visibly states
YOU SURVIVED YOUR MISSION / THE END. Exact patches, held/released buttons, budgets,
clean snapshots, effective-bank notes and retained evidence are in scenarios.json
and its linked records. The coordinator checked that every cited scenario evidence
file exists and directly inspected the terminal screenshot during orchestration.

Megagon transformation was displayed and Up+B exercised, but score was zero; laser
availability or damage is not established. Guides remain experiment hypotheses,
not independent classification evidence or proof of natural reachability. Broad
representative progression is complete as a best-effort workflow task; every level,
interaction or hardware state is not claimed. No unchanged gameplay replay offers
a currently identified discriminating test for the three isolated unknown RTS bytes.
