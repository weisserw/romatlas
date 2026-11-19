# Gameplay research — Battle Chess (NES)

Local title/copyright capture `evidence/pass-001/title.png` identifies Battle Chess, Data East USA, program ©1990, Beam Software. MMC1 256KiB PRG/CHR-RAM. Exact region/revision not externally matched.

Sources consulted 2026-11-19:
- NES manual: https://www.thegameisafootarcade.com/wp-content/uploads/2017/02/Battle-Chess-Game-Manual.pdf — Select opens options while highlight flashes; A confirms; pad moves; capture enters separate Battle Zone then returns. Main Move/Settings/Level menus. Confirmed locally: board, Select, Move submenu.
- NES FAQ search result: https://gamefaqs.gamespot.com/nes/587120-battle-chess/faqs/34913 — Setup Board allows rearrangement, Select then Clear/Restore/Done; Settings choose 2D/3D and Human/Nintendo per side. Direct fetch restricted, search extraction available. Setup recipe remains hypothesis until local confirmation.
- https://www.neoseeker.com/battle-chess/faqs/94232-nes-a.html is a mirror candidate for later research; not required for first pass.

Progression is chess phases, not numbered levels/bosses. Representative milestones: initial board, legal move/AI, 2D setting, setup-board editing, capture Battle Zone, check/promotion/castling and checkmate terminal result. No need for an entire naturally played match when setup or understood RAM can expose these states. Current first pass investigates built-in setup before deeper RAM contracts. Later-game/ending not yet confirmed.

Round001 redirect local confirmation: Setup menu record3 is Done; producer count4 includes header,Clear,Restore,Done. Cursor RAM8D=3+A triggers callback9156(action8),C2F3 initializes engine and copies board to mailbox. BC0 inherited demo/menu state disables normal moves; NewGame9698 BC1 contract enables manual processing. With BC1/B2FF, artificial near-mate Qb6-b7 accepted; negative status0551=80/0552=1 and marker observed. Capture variant adding bluepawn129atindex9 reaches distinct3D confrontation and dark banner/result presentation. Exact artifacts/recipes in redirect/round-001-a/review.md and scenarios.json. External manual's separateBattleZone wording is consistent with final distinct result field, but exact cinematic naming/complete return not asserted. All this progression is artificial after documenteddebug edits.
