# Code Name: Viper — local gameplay notes

Identification: title captured at frame 60, analysis/pass-001/boot.png, Capcom
1989 screen, Nintendo license. Region/revision not independently identified.

Confirmed local route: reset, neutral 60 frames yields title. Press Start for one
frame then release; difficulty menu appears. Press Start (two frames, release two)
chooses default Easy; transition is initially black. Press A and advance 30 frames
reaches office briefing. Distinguish this from gameplay.

Actionable external hypotheses consulted 2026-11-27:
- https://www.gamingalexandria.com/highquality/NES/Code%20Name%20Viper/Code%20Name%20Viper%20-%20Manual.pdf
  Search extract says A advances conversations; up is needed before jumping to an
  upper level. Full manual not retrieved yet. Unconfirmed for this ROM.
- https://etherealgames.com/nes/c/code-name-viper/passwords/ lists 040471 (Area 4),
  081629 (Area 8), 745765 (last boss), 217298 (end). Not yet locally tested.
- https://tasvideos.org/UserFiles/Info/638525806574855840 search extract names RAM
  $0030 as Current Level. Page fetch failed. This is a RAM hypothesis requiring
  local confirmation, not a trustworthy stage-entry contract by itself.

No ROMs or extracted assets downloaded. All local captures/snapshots remain local.

Local correction: decoded and tested password handler accepts digits at RAM
$0730-$0735. 040471 selects stage index3; 745765 selects index8 and reaches boss
room. This ROM's stage6 record is 081620 and stage7 is 171602. 217298 selects
stage0, quest1; the external 'End' label is misleading here. See investigation.md
L001/L002 for map countdown and preview wait contracts. Forest background and HUD
were reached for stage3; active player mechanics remain explicitly unconfirmed.
