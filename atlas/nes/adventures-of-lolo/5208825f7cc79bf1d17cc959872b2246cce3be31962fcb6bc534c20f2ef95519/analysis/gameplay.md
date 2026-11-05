# Gameplay familiarization

ROM likely Adventures of Lolo, US NES: observed HAL 1988/1989, Lolo/Lala introduction and Floor 1 briefing. External source https://etherealgames.com/nes/a/adventures-of-lolo/passwords/ lists GCVT for final room (10-5), CHZP for 5-1 and DVYB for 10-1. These are hypotheses until entered or checked against this ROM. https://etherealgames.com/nes/a/adventures-of-lolo/action-replay-codes-us/ suggests internal RAM $57 lives and $58 shots; unverified.

Observed route: 380 idle frames reaches title (initial 180-frame request timed out at 80; subsequent 300 completed). Start 2 frames/release +45 opens menu; another Start 2/release +45 begins castle approach. Another 45 +600 idle shows Floor 1 mission briefing. Long waits run tracing disabled; precise operations and trace state in pass001-rpc.jsonl. Frame filenames room1 currently capture briefing, not yet actual room.

Controls/menu and progression remain under investigation. First title/menu showed START/PASSWORD. Password shortcut permits broad later-game visits without solving every puzzle; no ROM data downloaded.

Confirmed artificial progression shortcut: main loop at $98E3 passes $55 room index to $B44A; clear increments at $999F. $67 nonzero takes death branch and reloads via $98CD after decrementing $57. On clean room1 snapshot, write RAM $55=24,$57=99,$67=1; release all inputs; run240frames with effective PRG[0,1,2,3]. Screenshot room25.ppm shows distinct dense puzzle; lives98 confirms reload. Repeat $55=49,$67=1 produced final-room layout (room50.ppm), lives97. Snapshots and RAM saved. 24-frame Left+A /24 Down (room25) and24 Left+A /24 Up (room50) exercise movement/shoot mechanics; these artificial states do not prove natural progression.
