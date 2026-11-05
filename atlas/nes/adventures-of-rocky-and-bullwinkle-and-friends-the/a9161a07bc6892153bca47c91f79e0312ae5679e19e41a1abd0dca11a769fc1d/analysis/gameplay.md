# Gameplay familiarization — round 001

ROM local evidence: 1992 THQ / Radical splash; physical PRG text contains WHAT'S THE BUZZSAW / RIGHT ON TRACK / ROCKY SEAS AHEAD / THE MOOSE IS TOO RICH. Identification: The Adventures of Rocky and Bullwinkle and Friends, NES; title-screen confirmation pending.

Actionable external hypotheses (not structural evidence):
- https://www.consoledatabase.com/cheats/nes/adventuresofrockyandbullwinkleandfriends/ — controller 2 B,A,A,Start,A,B,B,A during gameplay enables B on controller 2 to skip levels, reportedly disables jumps and grants invulnerability. NES-specific listing; test against this ROM before relying on it.
- https://www.neoseeker.com/the-adventures-of-rocky-and-bullwinkle-and-friends/cheats/nes/ corroborates sequence.
- https://gamefaqs.gamespot.com/nes/587077-the-adventures-of-rocky-and-bullwinkle-and-friends/faqs/43026 search result describes five levels and six scenes in level 5, character selection and moving right; full guide fetch unavailable. Treat details as unconfirmed.

RPC control caveat found by reading configured toolkit NES adapter: input uses `{buttons:[...],port:1|2}` replacing held button set. Generic protocol `{button,pressed}` is silently ignored by NES adapter. Early Start attempt used generic form and had no effect; do not treat that as a tested route. Corrected routes will use buttons arrays and explicit releases.

Confirmed local progression: title, Level1 intro and playable Frostbite Falls; A jump and B bomb (HUD bombs10 to9) observed naturally. Source-supported debug flag/index transitions successfully initialize railway/log, desert/plane, ship deck, Asian/island architecture and final indoor scenes; these are artificial-state visits, not natural progression claims. See scenarios.json.

Additional control research: https://www.scribd.com/document/653730229/Adventures-of-Rocky-and-Bullwinkle-and-Friends-1992-THQ-Inc (scanned NES manual search excerpt) says Select switches characters while standing still; https://tasvideos.org/4529S independently says stop moving before changing characters and describes holding Select affecting movement/energy. https://speeddemosarchive.com/RockyBullwinkleAndFriends.html says left/right+B triggers character special moves. Prior Select2 immediately after movement did not visibly switch the moose; newly justified retry must release movement long enough to stop before Select, then inspect frame. Sources are route hypotheses until tested.

## Redirect: supported terminal presentation and dismissal

A materially changed ENDING-001 method bypassed streaming entirely: final-scene page3 event physical$1E50 has EventWriteByte payload83 07 FF; RunFrame checks$0783 and jumps$E2C9. Restore initialized scene-10b.snapshot, release both controllers and write only RAM$0783=$FF. Twenty bounded30-frame calls (max_operations3000000,timeout_ms15000 each) reached visible YOU WIN!! by360frames, stable through600. Selected local [win frame](redirect/r001-static-streams/ending-360.png), [plan](redirect/r001-static-streams/ending-test-plan.md), [exact requests/results](redirect/r001-static-streams/ending-rpc.jsonl), and [snapshot](redirect/r001-static-streams/ending.snapshot). This is artificial terminal presentation, not a boss encounter or natural final route.

Restore that terminal snapshot, press Start2frames and release120frames: [THQ opening frame](redirect/r001-static-streams/ending-dismiss.png), [continuation log](redirect/r001-static-streams/ending-continue-rpc.jsonl). Complete contextual aggregate pages saved for this dismissal; initial terminal test saved only first context page, so no complete observed-opcode coverage claim. The failed unchanged camera-edge warp remains preserved and was not repeated. scenarios.json updated.
