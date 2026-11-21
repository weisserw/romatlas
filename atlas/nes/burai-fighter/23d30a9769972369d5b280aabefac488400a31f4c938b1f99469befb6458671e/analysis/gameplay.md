# Gameplay familiarization

ROM visually identifies as Burai Fighter, TAXAN 1989 title, Japanese subtitle. Exact release/revision unresolved; NTSC emulator used. Mapper4 /32KiB PRG/32KiB CHR agrees with workspace inspection. Title/attract progression confirmed locally; pressing Start during attract returns to title, next Start opens SELECT LEVEL difficulty menu, next Start chooses Eagle. Early uncorrected button/pressed experiments are invalid input tests; corrected API is {buttons:[...],port:1}, [] releases.

Sources consulted (2026-11-21):
- https://www.thegameisafootarcade.com/wp-content/uploads/2017/02/Burai-Fighter-Game-Manual.pdf — manual: directional movement, B fire, A cobalt bomb, Start start/pause, Select new/password. Stages 1/2/4/5/7 side view, 3/6 overhead base-search. Source descriptions are runtime hypotheses, not classification evidence.
- https://www.neoseeker.com/burai-fighter/cheats/nes/ — search-visible cheat listing: Eagle BALL/CAMP/DOLL/EDEN/FACE/GAME for stages2..7; KAGA99 lives, LTTB or LOBB weapons. PAL passwords differ (CQMP/FQCR/GQMR). Page open403 but search retained useful text. Version applicability unconfirmed.
- https://gamefaqs.gamespot.com/nes/563393-burai-fighter/faqs/41660 — search-visible walkthrough: Ace/Ultimate graphical ending versus lower-difficulty text ending; top-view wraps and base combat. Direct page restricted. Password GAME/NILE/IDOL/ICHI reportedly stage7 across difficulties, with conflicting tables for middle difficulties; use Eagle to avoid ambiguity.

Local evidence: analysis/round-001/menu.png shows natural attract gameplay; real-menu.png shows NEW GAME/PASSWORD; real-start.png shows EAGLE/ALBATROSS/ACE. Durable snapshots and exact RPC input/frame requests are in analysis/round-001/rpc.jsonl. Later stages and ending remain to test.
