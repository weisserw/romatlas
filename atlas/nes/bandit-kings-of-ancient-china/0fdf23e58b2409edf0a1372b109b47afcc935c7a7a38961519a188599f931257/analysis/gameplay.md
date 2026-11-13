# Game familiarization

ROM identity: SHA-256 in workspace.json; iNES mapper 5, 256 KiB PRG + 128 KiB CHR.
Local internal trailer says `BANDIT KINGS30`; English scenario text names Gao Qiu,
Song Dynasty and four selectable scenarios (1101, 1103, 1104, 1105). Likely
Bandit Kings of Ancient China, exact region/revision unverified. No external
manual consulted because runtime start rejects MMC5 before progression can run.

Local text hypotheses: character/scenario/difficulty selection, A-button
confirmation, diplomacy/development/expeditions and battle, victory feast and
Northern Barbarian terminal event messages. These are readable text observations,
not visited states or confirmed controls. Later-game routes and RAM progression
contracts remain unverified. Native-mode testing is unavailable in the configured
debugger; do not replay unsupported start or arbitrary RAM edits.
