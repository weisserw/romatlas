# Game familiarization — round 01

Local copyright/title captures identify The Blues Brothers, Titus 1991 copyright, NES mapper 2. Exact region/revision remains unconfirmed.

Sources consulted 2026-11-20 (instructions are hypotheses until locally tested):
- https://www.world-of-nintendo.com/manuals/nes/blues_brothers.shtml — manual transcription: Start dismisses copyright, Start selects character and begins chapter; Select changes Jake/Elwood/both. A jumps, B accelerates, down crawls, down+A drops through some floors, up/down ladders. Eggs can be kicked, dogs/snakes ridden. Five chapters lead from department store, warehouse, prison, sewer and construction site to concert.
- https://gamefaqs.gamespot.com/nes/587148-the-blues-brothers/cheats — NES skip hypothesis: P2 down+right+A+B+Select+Start and P1 up+right+A+B+Select+Start enable skip, then A+B+Select+Start skips. Cross-platform entries on same page excluded.
- https://etherealgames.com/nes/t/the-blues-brothers/preformatted-cht-file-action-replay-game-genie-us/ — RAM hypotheses $0750 chapter, $042A/B energy, $0428/9 lives, $03C2/3 invincibility. ROM matches initial reset writes to energy addresses and multiple code references to $0750, but stage edit needs an initializing transition.

Confirmed: copyright -> title (Start 2 frames, release; traced until frame202), title -> chapter 1 prose (Start 8 frames/release20). Start4/release60 reaches black loading screen, pending first active frame. Captures and exact RPC budgets in analysis/round01/rpc-live.jsonl. No platform-wide compatibility claim.
