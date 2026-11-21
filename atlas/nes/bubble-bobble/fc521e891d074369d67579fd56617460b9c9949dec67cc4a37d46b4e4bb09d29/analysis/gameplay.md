# Gameplay familiarization

Identified locally by title: Bubble Bobble, 1988 Taito America NES. Header mapper 1,
128 KiB PRG and 32 KiB CHR; precise revision not externally matched.

Official manual: https://www.nintendo.co.jp/clv/manuals/en/pdf/CLV-P-NABKE.pdf
(search excerpt consulted). Select chooses password option, Start enters it;
left/right moves password cursor. Continue round selection uses A/B. Bubbles and
jump controls remain to confirm locally against frame captures.

Practical later-game routes (external hypotheses, not structural evidence):
https://strategywiki.org/wiki/Bubble_Bobble/NES_Walkthrough and
https://gamefaqs.gamespot.com/nes/563392-bubble-bobble/cheats . Passwords may select
later rounds; Neoseeker search excerpt gives EECFG for round 100 and HEAGD for
Super mode stage select. These are untested and version applicability unresolved.
Good/best endings involve additional items and two-player state; manual progression
is not a practical complete-game route for this pass.

NES API correction: emulator.input takes `buttons` array, not the shared protocol's
single-button/pressed form. Early purported Start scenarios were no-input attract
mode and establish no gameplay reachability. See toolkit issue NES-001.
