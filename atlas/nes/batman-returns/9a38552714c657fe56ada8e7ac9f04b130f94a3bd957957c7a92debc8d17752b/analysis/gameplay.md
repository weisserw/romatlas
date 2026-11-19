# Gameplay research

## Identification (locally confirmed)

- Title screen (local framebuffer, Round 1): "BATMAN RETURNS", (c) 1993 Konami, licensed by
  Nintendo. Konami logo, "BATMAN" logo, then title. Bank 15 tail contains ASCII "RD053" and
  "BATMAN RETURNS" (Konami-style internal header). NES, NTSC, MMC3, USA release assumed
  (version uncertain; no region/revision proof beyond the header text).

## Controls / menus (locally confirmed unless noted)

- Title: Start opens a START / PASSWORD menu. Start on START begins the intro cutscene
  (Shreck's / Gotham Plaza story text, ~5000 frames). Start/A/B do not skip the cutscene
  in this build (tested: text keeps typing).
- In stage: Right walks, B attacks, A jumps (bounded input pattern used; exact move set
  not characterised).
- Toolkit note: NES emulator.input requires `{"port":1,"buttons":[...]}` (held set);
  the `button`/`pressed` form is silently ignored (see analysis/toolkit-issues.md TK-001).

## Stage select / shortcuts

- Source: Almar's Guides Action Replay level select
  https://almarsguides.com/retro/walkthroughs/nes/Games/BatmanReturns/ActionReplay/LevelSelectCodes/
  gives code `003E:??` with values 01=1-2, 02=2-1, 03..06=2-2 parts 1-4, 07=2-3,
  08..0B=3-1 parts 1-4, 0C=3-2, 0D=3-3, 0E=4-1, 0F=4-2, 10=4-3, 11=5-1, 12=5-2,
  13=6-1, 14=6-2 part 1, 15=6-2 part 2.
- Local confirmation: writing CPU RAM $003E during the intro (snapshot `pre_stage`,
  just before the stage-1 title card) loads the selected stage: $3E=7 showed the
  "STAGE 2-3 SHRECK'S DEPARTMENT STORE" title card and gameplay. See analysis/scenarios.json.
- Search results (TCRF, Supercheats; not fetched successfully, unconfirmed): hidden level
  select via password (reported as "D*31 11 3191 11" or "Y*31 21 3191 11"; sources disagree)
  using controller 2 Left/Right; a copy-protection check tied to an unused Konami-logo check
  in stage 2-2 that can make 3-1 unwinnable. https://tcrf.net/Batman_Returns_(NES) (DNS failure
  on new.tcrf.net during this pass).
