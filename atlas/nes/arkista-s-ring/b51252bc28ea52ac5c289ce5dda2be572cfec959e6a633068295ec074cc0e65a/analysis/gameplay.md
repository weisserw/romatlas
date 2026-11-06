# Gameplay research

No research performed yet. Follow workflow.md's Game familiarization section. Record source URLs, version uncertainty, actionable routes and local confirmation separately.

## Round 001 familiarization and confirmation

Title frame identifies Arkista's Ring (American Sammy/NMK; title copyright1989).
Exact revision beyond workspace SHA not established. Guide hypothesis: 31 distinct
stages repeat four times; key appears after defeating enemies, then exit. A fires,
B opens item selection, Select shows status between stages. Sources:
https://gamefaqs.gamespot.com/nes/587094-arkistas-ring/faqs/63654 and
https://gamefaqs.gamespot.com/nes/587094-arkistas-ring/cheats . No assets downloaded.
Version applicability uncertain until local tests; natural Start confirmed, artificial
right/up+A stages produced actual maps and accepted instruction fetches.

RAM cheat hypotheses from
https://etherealgames.com/nes/a/arkistas-ring/preformatted-cht-file-action-replay-game-genie-us/ :
$03D0 stage, $03D6 health, $03DB lives, $30 displayed level. Local static code confirms
$03D0 initialization/increment/wrap and $03DB life handling; only stage used as shortcut.
Source calls $30 both level and fake-stage modifier; do not treat it as loader state.
No ROM Game Genie changes used.

Static transition at $BC5E clears $19 before stage initialization $8552. Omitting
that precondition initially left black sprite-zero wait at $C4DD; corrected $19=0.
Visited stages8/24/31/32 via initialized transition, not isolated stage-byte write.
Ending shortcut sets $03D0=31,$2E=3 then enters stack-reset $BC5E; ending text appears.
Later600frames shows stage01 character/status display, so permanent terminal state
and natural full-game completion are not claimed. See scenarios.json for exact budgets.
