# Game familiarization

Local title confirms Cyber Stadium Series: Base Wars, Ultra/Konami 1991 (likely NES USA; no revision claim). Source: https://www.world-of-nintendo.com/manuals/nes/base_wars.shtml (NES-58-USA manual transcription, consulted 2026-11-13).

Manual hypotheses: open game permits 1–9 innings and human/CPU selections for both teams. A confirms, B cancels, Start enters mode menu. Pennant supports 2–6 teams, up to ten games, persistent rankings, shop and robot customization. A pitches/swings; holding A charges a fast pitch. Close base plays initiate combat; A/B attack; three destroyed robots ends a game. These routes can expose distinct systems without long manual matches. Menu routes and terminal transitions remain runtime-confirmed separately in scenarios.

Confirmed locally: title, mode select, open-game team selection and inning selector. NES RPC inputs use `buttons:[...]`, with an empty array releasing all port buttons. Initial button/pressed calls were ineffective; no reachability claimed from them.

Targeted shortcut source: https://etherealgames.com/nes/c/cyber-stadium-series-base-wars/preformatted-cht-file-action-replay-game-genie-us/ gives score $0445/$0446, innings $044A, strikes $0068, balls $0069, outs $006A, and ending Game Genie YENKGYAE. These are hypotheses, not annotations. Decoded GG changes CPU $C7FC operand $00 to $0F; local fixed-bank instruction is LDA #$00; STA $30; STA $34, followed by mode $20=$05. This provides a concrete state shortcut hypothesis without changing ROM backing.


## Redirect001 local progression confirmation

Source-derived artificial close-play shortcut reached and exercised robot duel; exact recipe and captures in redirect/round001-a/combat-shortcut-recipe.json and scenarios.json. Title-based ending entry followed ordinary animation/credits progression to stable phase55 with PRESENTED BY/KONAMI at frame8197; selected final frame and snapshot retained. These confirm local behavior in artificial RAM-edited scenarios, not natural routes or perfect rendering. No new external-source hypothesis used.
