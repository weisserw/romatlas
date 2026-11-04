# Game familiarization — round 1
Title is Adventures of Lolo III, confirmed by overworld and title ©1987,1990,1991. Earlier progress entry read the small Roman numeral as II; corrected here. Mapper 1, 128 KiB PRG/128 KiB CHR. Likely western release; exact revision/region unconfirmed.

Manual transcription: https://www.world-of-nintendo.com/manuals/nes/adventures_of_lolo_3.shtml . Controls: dpad movement; A swaps hero on map, shoots/uses powers in rooms; Select retries; Start returns to map and then password. Trees provide tutorials; progression includes cave, underwater, underground castles and King. These are sourced hypotheses except map hero swap/movement/first tree exterior observed locally. Do not use Lolo II four-character passwords: wrong game.

Password list: https://etherealgames.com/nes/a/adventures-of-lolo-3/passwords/ . Candidate shortcuts: caves/bosses `JY8H JPLG 893Q C♥2L`, underwater `V4Y♦ 23G△ 5J♦+ JNDN`, level17 `96HW 56R8 ♥B△B 93G5`, King `4R4N SPDD F♥L4 5BTJ`. Not yet tested. GameFAQs password-system FAQ fetch restricted; StrategyWiki password page 403; no ROM or extracted assets downloaded.

Local controller API accepts `buttons` array replacing full button state; generic protocol button/pressed silently gives an empty array. Use [] for release. Snapshot restore restores buttons: always reassert them. Trace-disabled traversal is faster but not observed evidence; sample tracing enabled at scene/mechanic milestones.

Round1 outcomes: underwater09 room1 visited under validated artificial password state; cave-region, King andlevel17 maps accepted, with room-entry obstacles. Exact successful/failed recipes and milestone statuses are in analysis/scenarios.json. No combat/ending confirmation. Snapshot import restores immediately; save a named snapshot afterwards if branching.

Round2 review correction to old round1 outcome paragraph: redirect subsequently validated the map-loop node/flag20 transition, cave04 boss, King17 multiphase defeat, late17 puzzle, finalrescue and THE END. All remain artificial progression observations; natural tutorial remains separately recorded. Exact prior/new recipes and phase reviews are in scenarios.json and redirect/r1-node-and-residual/HANDOFF.md. Round2 reused these saved routes/fetches and did not repeat ordinary scenarios.
