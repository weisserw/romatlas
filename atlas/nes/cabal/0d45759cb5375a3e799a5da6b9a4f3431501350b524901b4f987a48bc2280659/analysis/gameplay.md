# Cabal NES familiarization
ROM title observed in startup: Cabal, Milton Bradley / Fabtek / Rare, copyright1989. Mapper7 128KiB PRG; precise region/revision not independently established.

Manual NES-C7-USA transcription: https://www.world-of-nintendo.com/manuals/nes/cabal.shtml (consulted2026-11-21). Five levels with four scenes each; final scene of each level has a war machine. Start begins mission/level, pauses or continues. Hold A fires, tap A grenades; directions aim while A held. B plus directions provides running/roll/jump/duck. Five starting lives, three continues. Ending leads to initials. Manual statements are hypotheses until tested.

https://gamefaqs.gamespot.com/nes/587166-cabal/cheats lists auto-grenade input on level screen (hold Select, B B B A Start), untested. Listed SCHLIKA/F2 shortcut is explicitly Amiga, inapplicable NES. No applicable built-in stage shortcut confirmed. No ROM/assets downloaded.

Observed route: startup360frame request stopped at10000000tick budget (~335frames), title snapshot; +60frames reaches title. Start2frames/release30frames, Start2frames/release40frames yields Level1-1 battlefield and5lives. All captures/snapshots local analysis/evidence/round001.

## Round002 bounded natural continuation
Reused redirect unpaused natural first-battlefield snapshot; explicit release/right-fire/up-roll/left-fire segments each requested10frames with1500000operations/8000ms. All timed out with partial progress,2972contextual fetch roots retained; no990C fetch or later milestone demonstrated. Later-scene shortcut negatives remain valid and unchanged retries are deferred for missing render/actor-initialization contract. Evidence: analysis/evidence/round002/gameplay*. No external lookup needed for this reused local route.

## Round002-a redirect: post-ending terminal ranking shortcut
After inspecting the actual819C post-illustrated-ending continuation, restored the retained round001 ending snapshot, sought a bank0 instruction boundary (<=300operations), setPC819C/stack255, then ran release/Start/release for120000operations each,timeout8000ms. All budgets completed; captured BEST5PLAYERS terminal scoreboard and ranking/title fetches. This is artificial continuation after demonstrated artificial ending display, not natural victory. Exact patches, operation results, contexts, snapshot and viewed image: analysis/redirect/round002-a/terminal-probe.py, terminal-run.json, terminal-contexts.json, terminal.snapshot.json, terminal.png. Existing later-scene/boss failures were reviewed and unchanged patches not repeated.
