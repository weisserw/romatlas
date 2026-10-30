# Gameplay research — 1943 (NES)

Local title identifies Capcom 1988 1943, one player, Start and Password options (analysis/evidence/startup.ppm). Revision/region remains inferred from English title; no external ROM identity match attempted.

Sources consulted:
- https://strategywiki.org/wiki/1943:_The_Battle_of_Midway/Password_system — NES password is five characters; first selects stages1–23, last checksum/upgrade constraints. Later-stage routes are useful hypotheses. Stage12 prefix H, stage22 R, stage23 T. Remaining maximum-upgrade candidate Y29U is derived from source tables (offense/defense6+6, weapon6/energy6, timer6 =>30 points is not explicitly in checksum table); do not use unchecked formula as established password. Known simpler candidate RDING found in search snippets, not locally verified.
- https://www.thegameisafootarcade.com/wp-content/uploads/2017/02/1943-The-Battle-of-Midway-Game-Manual.pdf — original NES manual reference retrieved, scanned PDF lacks text extraction. No full content copied.
- https://gamefaqs.gamespot.com/nes/587059-1943-the-battle-of-midway/faqs/25887/-pre-made-passwords- — retrieval restricted, not relied upon.

Locally confirmed controls/API:
- emulator.input requires whole held-button list {buttons:[...],port:1}; releases use empty list. Shared button/pressed schema silently does nothing (NES-001).
- Start from attract/prologue returns to title; Start at title enters upgrades. Upgrade cursor uses Up/Down. B adds a point to selected attribute (decoded consumer $A7D0 tests edge bit$40); A and Right do not allocate. Repeated B press/release distributes three starting points. Start then enters launch sequence.
- Current RAM candidate stage variable $9C is zero after initialization; load/reset references and level-associated lookup arithmetic provide a promising transition lead, not a proven level edit contract yet.

Progression confirmed: title, prologue, upgrade allocation, launch from carrier. Actual combat, later stages, bosses and ending still pending in this pass. Captures are local evidence and not shareable assets.

Final pass correction: $9C is not a usable stage selector; launch reset it. Password consumer locally establishes RAM$0715=(stage−1)*2. Artificial stage/phase22 and47 are recorded with preserved values and changed stream/enemy behavior, but full later-stage/boss visits remain unproven. Final47 state quickly reached GAME OVER. Natural stage1 combat was visited with movement and shooting/special inputs. Direct ending jump failed (stripes); revised full stage-result entry displayed WHAT A PITY! then flight. Ending/credits not visited. These failures scope to initialization recipes, not a conclusion about unused or inaccessible game content. See scenarios.json, leads D003–D005/R003.

Round002 reuse note: the prior Round001 redirect supersedes the initial pass's unsuccessful ending status. Full result initializer $F541 with phase47/equal tallies displayed MISSION24 CLEAR/TOKAKU/victory; full phase initializer $F3FB produced distinct final waves, ship/terrain and large-aircraft encounters, with movement/fire/special inputs exercised. Exact patches, state, input releases and budgets remain in analysis/redirect/round001-20261030/runtime-actions.json and analysis/scenarios.json. These successful visits are artificial, not proof of natural boss defeat or ending reachability. Round002 reused these records without replaying unchanged experiments; no new gameplay route claim.
