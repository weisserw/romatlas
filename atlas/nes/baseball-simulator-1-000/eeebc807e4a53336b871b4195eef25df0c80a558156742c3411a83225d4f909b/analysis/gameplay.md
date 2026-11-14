# Gameplay research — round 001

Title screen identifies **Arkista's Ring**, American Sammy (copyright 1989), US English NES. ROM revision not independently identified.

Sources consulted 2026-11-13:
- https://etherealgames.com/nes/a/arkistas-ring/preformatted-cht-file-action-replay-game-genie-us/ lists RAM $03D0 as stage modifier, $03D6 health, $03DC treasure flags. It also contradictorily labels $03DB both level modifier and lives; do not rely on $03DB. These are experiment hypotheses, not structural evidence.
- https://gamefaqs.gamespot.com/nes/587094-arkistas-ring/faqs/63654 describes directional movement, items and nine continues; source search excerpt only, not fully reviewed.
- https://www.hardcoregaming101.net/arkistas-ring/ describes 124 stages and a final win message; later milestones remain hypotheses until observed.

Local: title/menu captured in round-001/startup.ppm. Start begins stage/gameplay; bounded directional+B route and pre-start stage patches recorded in round-001. Exact observed results and unresolved initialization contract are in the handoff and scenarios.json.

## Identification correction

The Arkista identification above is **superseded and rejected**. It came from an unrelated preexisting `/tmp/startup.png` after direct PPM inspection failed, not this ROM's capture. Actual workspace PPM files were converted with `analysis/scratch/ppm.py` into unique adjacent PNGs and inspected: they show starfield and CULTURE BRAIN PRESENTS. Consequently all Arkista source information and stage RAM hypotheses are inapplicable and cannot support game identification, progression or structure. The three artificial patches are failed mismatch experiments, not later-stage visits. No structural classifications depend on those external claims. Actual title identification continues locally.

## Confirmed title and corrected input contract

Actual title: **Baseball Simulator 1.000**, Culture Brain ©1989, from `round-001/intro-more-2.ppm.png`. NES API input is a full `buttons` array, not shared `button/pressed`; earlier routes are therefore no-input attract mode (TK-001). Corrected route starts with `buttons:["start"]`, 3 frames, then `buttons:[]`, 25 frames.

Applicable sources:
- https://www.world-of-nintendo.com/manuals/nes/baseball_simulator_1000.shtml (manual transcription, NES-UB-USA): modes Exhibition, Season, Edit, New League, Watch; choose with control pad and A. Ultra league provides special plays. Version applicability plausible, controls to confirm locally.
- https://gamefaqs.gamespot.com/nes/587111-baseball-simulator-1000/faqs/18654 (FAQ search excerpt): player one bats first in exhibition; Ultra pitching menu uses Up twice. These are hypotheses until tested.

Manual details used for next experiments: after selecting stadium, select starting pitcher and confirm YES, then accept lineup. Offense uses A to swing (brief tap bunts), directions move batter, B participates in running/Ultra selection. Season supports short five-game simulation and AUTO teams, a useful route toward terminal season results without playing 165 games. None of these later milestones is claimed visited until captured.

## Round001 redirect progression correction and results

Local redirect evidence now verifies an artificial top1->bottom1 transition, initialized9INBOT, actual pitcher delivery/ball and GAME SET. Exact calls, patches, bank context and budgets are retained in analysis/redirect/r001-static-contracts/ and scenarios.json; no natural terminal reachability or full fidelity claim.

Season setup reached visibly AUTO teams, opener, TOP/BOT AUTO, WATCH/SKIP and PLAY BALL. The earlier five-game cursor was not committed: immutable PRG-RAM inspection showed stored$6007=165. Late$87 suppression/timer patches and bounded commit intercepts failed to establish season progress. No five-game simulation or Season terminal result is claimed; all failed improved methods and the missing widget/port/transition contract are preserved. Do not repeat unchanged button/wait or raw-patch attempts.

## Round002 representative Ultra and defensive controls

Manual transcription revisited (2026-11-14): https://www.world-of-nintendo.com/manuals/nes/baseball_simulator_1000.shtml (NES-UB-USA). Actionable controls: double-Up opens Ultra-pitch selection, Up/Down changes it, A pitches; a use costs3 points. Start then defensive A opens CHANGE/MOVE; MOVE selects/repositions a fielder. B near a live ball requests Ultra fielding, a separate contact-dependent mechanic. These were hypotheses until tested locally.

Ordinary Ultra-row team selection now reached AL-SW/Super matchup, PLAY BALL and live1INTOP. Double-Up/Down changed the pitch symbol; A produced visible delivery and point decrease117->114. Earlier A pitch showed STRIKE199mph and120->117. No exact special-pitch name, successful special hit or Ultra-fielding contact is established. Filenames containing special-hit refer to attempted B input, not success. From artificial initialized9INBOT, Start/A displayed the defensive diamond and CHANGE/MOVE/END menu, followed by bounded MOVE/directional/A operations and B exit;12 released frames confirmed return to live game. Exact successful fielder displacement/catch remains unconfirmed. No further unchanged Season/late-flag trials were performed. See S008-S010 and round002 runtime/capture artifacts.
