# Gameplay evidence — round 001

Identification: Attack of the Killer Tomatoes, NES (likely USA); local title and
1991 THQ/Children's Network copyright and John DeBello credit match the manual.
Exact regional revision is not independently established.

Sources consulted 2026-11-06:
- https://www.digitpress.com/library/manuals/nes/Attack%20of%20the%20Killer%20Tomatoes.pdf
- https://manuals.plus/m/53a07ebf7edf10fc26fd748ff1d81d7e7214c73e5945979315ba00557c3db2cb
- https://gamefaqs.gamespot.com/nes/587099-attack-of-the-killer-tomatoes/cheats
- https://github.com/FrodeSolheim/cheats/blob/master/nes.cht

Manual hypotheses: A jumps; B throws rocks during fights and accelerates movement
in some areas; Up/Down navigate maze exits. Progression includes city, sewer,
factory and Doomsday scaffolding with named bosses. Start begins from title,
credit and victory screens. Tested title Start and street movement/jump. Later
artificial visits exercised movement/jump/B and sewer Up/Down; no maze exit or
boss damage confirmed. The GameFAQs level-select and credit cheats explicitly
carry GB labels and are inapplicable to this NES investigation.

External RAM cheat suggestions $0722 health, $0718 lives match local HUD after
patching to 200/9; used only as bounded snapshot branches, not frozen cheats.
Do not infer structure from external RAM addresses alone.

Local progression contract: $0030 current mode, $0031 next mode, $0026 bit0 exits
the active loop (main driver CPU $D173..$D1AC). Stage/subarea byte $0716 has
comparisons to 0..5, but full domain/initialization not proved. Clean level1 state
branches with $0716=0, $31=6..9, $26=1 enter distinct later areas. Modes 0=title,
2=story conversation, 3=street introduction, 4=explosion scene (victory hypothesis,
not independently confirmed as completed ending). Mode9 enters tomato chamber;
no boss appears within tested waits/inputs. Do not claim a boss or full ending was
visited. Recipes and checklist: analysis/scenarios.json. Screens/snapshots local.
