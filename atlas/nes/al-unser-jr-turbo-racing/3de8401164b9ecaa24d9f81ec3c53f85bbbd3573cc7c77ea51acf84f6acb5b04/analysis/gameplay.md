# Gameplay research

No research performed yet. Follow workflow.md's Game familiarization section. Record source URLs, version uncertainty, actionable routes and local confirmation separately.

## Round 1 familiarization

Title capture confirms Al Unser Jr. Turbo Racing, Data East USA copyright 1988. NES/FH-USA manual transcription consulted: https://www.free80sarcade.com/nes_Alunserracing.php ; manual scan found at https://www.digitpress.com/library/manuals/nes/Al%20Unser%20Jr.%20Turbo%20Racing.pdf . Revision beyond observed title not established.

Manual hypotheses: A accelerates, B brakes, left/right steer, up/down shift three gears; up+A sustains turbo. Start begins/pauses. Modes are Time Trial A (traffic), B (solo), and World Grand Prix. Practice allows circuit/lap selection; championship has car/name/color customization, advice, practice, qualifying, races and standings. Six car attributes affect speed, acceleration, durability, suspension, pit speed and turbo. Sixteen circuits include Japan; direct trial selection should provide later-course access. Championship race completion/ending routes remain untested.

Confirmed locally: mode selection; trial A setup, Brazil course/settings, music, starting grid and racing. Accelerator/turbo and steer/brake inputs produced driving execution and frame changes; detailed physical fidelity not evaluated. Initial menu inputs with short releases hit transitions rather than intended selection. Longer waits reliably moved the setup cursor. Fifteen right presses from a previously advanced circuit appeared to wrap to Brazil (course screenshot confirms Brazil); do not claim Japan visited. Selector RAM differences remain hypotheses. Source descriptions do not establish code classification.

## Round 1 redirect local progression
Correct source contract: configured course$6401 latches to internal$bf; $062e/$062f are audio RAM. Ordinary trial setup cursor3 RIGHT selectsSpain(course1), LEFTwrap selectsU.S.A.(course15). NamedROUND02/ROUND16setup plusactualdrivingframes retained underredirect/r1-contracts. GrandPrixentry,playerconfirmation,choices andBrazilqualifying initializednaturally. Supportedartificialfinish flags producedqualifyingresults; recordedcourse15/finishshortcut thennormalBGM/player-ready gates reachedfinalGPrace/results/driverpoints andactuallosingchampionshipclosingmessage. These are artificialprogressionobservations, notnaturalchampionshipcompletion.

Source-supportedartificialpitentry fromoriginalnaturaldriving snapshot ($46=1,$1b16,$dd7) reachedPITSTOP. Refuelinteraction readselector$63a5=1then2 afterA;repairnotentered. Exactrecipes,negativeearlyterminalcallattempt,andcaptures arein scenarios.json andredirect/r1-contracts/findings.md. No additionalexternalresearchorROM/assetsdownloads.
