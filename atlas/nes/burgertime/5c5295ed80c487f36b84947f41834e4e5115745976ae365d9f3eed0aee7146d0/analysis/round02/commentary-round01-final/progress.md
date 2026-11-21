# Commentary progress

Round 1 current-source readability is complete, based on pass-001-final. Reviewed
all 112 scaffold intervals: 61 code intervals (11,296 bytes), 48 non-code/image
intervals (13,281 bytes), and three honestly explained unknown intervals (15 bytes).
The original 5,745-line source was read in full; edits and entry boundaries were
checked against the saved 5,243-instruction decode and physical dispatch words.

Source now has 274 routine/block contracts, 727 control labels and 387 address
symbols, plus explicit main-state/player-slot constants. All PRG data and unknown
bytes are visible as byte/word directives; dispatch tables use named handlers.
CHR remains a local binary asset. Persistent RAM, OAM fields, PPU/APU registers,
BCD scores, reused scratch and contextual audio fields are named consistently.
Source comments cover initialization/menu/state transitions, player/pepper,
entity animation and geometry, enemy priorities, ingredient falling/landing,
queues, bonus/scoring, NMI/PPU transfers and both audio interpreters.

Mechanical corrections include sprite-row Y updates (not attributes), exact
segment-wrapper fallthrough, one-based pointer/duration bases and audio flags
surviving request-index arithmetic. Significant limits and stale phase-1 semantic
metadata are in uncertainties.md; physical classification remains unchanged.
Review credit: all 112 regions reviewed, 322 compact units reviewed. Final
editorial_audit remains pending; no final acceptance audit was run.

Fresh final verify rebuilt source/game.o, linked the source and confirmed
byte identity with input/game.nes. Verification is in verification.json and
analysis/round01/commentary-final-verification.json. reading-after.asm was refreshed
after that verification. Earlier verified edit-batch evidence is retained in
analysis/round01/commentary-verification-batch01.json. Baseline, ROM, maps and state
are unchanged. No deferred readability ranges; redirect may now review structural
residuals. No toolkit issues found; no toolkit modifications or cleanup.
