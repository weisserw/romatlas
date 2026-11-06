# Gameplay research — American Gladiators (NES)

Identified against local title screen: American Gladiators, copyright 1991 GameTek /
Samuel Goldwyn, Incredible Technologies. US/NTSC-style ROM, mapper 1; exact revision
not independently identified. No ROM/assets downloaded.

Sources consulted 2026-11-05:
- https://www.digitpress.com/library/manuals/nes/American%20Gladiators.pdf
- https://www.gamingalexandria.com/highquality/NES/American%20Gladiators/American%20Gladiators%20-%20Manual.pdf
- https://consoleclassix.com/nes/american_gladiators.html
- https://watchcharge.com/american-gladiators-on-the-nes-an-under-rated-cartridge/

Actionable source hypotheses: five selectable events (Joust, Wall, Human Cannonball,
Powerball, Assault), four difficulty levels, then final Eliminator. Manual search
excerpts describe directional/Select navigation and Start/A confirmation. A secondary
source gives a level-3 / ten-life shortcut: highlight PASSWORD, press A,B,B,A,B,Start.
Not yet confirmed against this ROM. Full manual fetch failed (large PDF/no extracted
lines); search excerpts sufficient for menu experiment, not classification evidence.

Local observations: early Start attempts during copyright animation had no effect;
late held Start (approximately frame 600 onward) reached player selection. Further
Start/release pairs selected one player and revealed START/PASSWORD. Exact scripts,
actual frames, timeout states and captures: round-01/progressplay.py, menus.py,
round-01/rpc.jsonl and ../traces/round-01/. Source gameplay descriptions remain
hypotheses until screenshots/accepted opcode fetches demonstrate each state.

Confirmed later progress: built-in ABBAB password produces Level03/Lives10.
Replayed level3 menu and pressed Down 0..4 times, Start, then bounded settling and
Right+A+B action. All five distinct event screens observed: Joust, Wall, Human
Cannonball, Powerball, Assault (follow-0..4.ppm). Initial 240-frame settling was
insufficient; an additional 360 frames reached the actual event. Snapshots/scripts
retain each transition. Source https://gamefaqs.gamespot.com/nes/563382-american-gladiators/cheats
also lists level4 passwords; untested because local progression contract now allows
a direct bounded final-state experiment. Never treat external codes as structural proof.

Local supported progression contract: physical $0B57..$0B99 (bank0 CPU $8B47..$8B89)
combines current event's bit into RAM $0595, compares #$1F, increments $0591;
level index4 sets $0590=5 and jumps $89F2, index5 branches $8B89. Earlier initialization
stores level to $0591/$0592 and lives to $0593. Final/ending artificial experiments
are recorded separately in terminal.py; observations do not establish natural reachability.

Round01 redirect confirmed a supported artificially shortened completion route.
Restore follow-0 snapshot; retain natural stack/PC. Parent actor11 waits at$8B17,
savedY10. Set currentLevel$0591=3 or4, completedEventMask$0595=$1F,
eventResultFlag$0584=0, clear child10 flags$0554, and set pauseMode$0576=1.
The parent performs its own fade/remove/transition. Level3 advances to index4/event5:
Blaze portrait, then confirmation/settling reaches the Eliminator obstacle arena.
Right+A+B exercised the contender there. Level4 advances to index5, rendering
CONGRATULATIONS!! AMERICAN GLADIATORS GRAND CHAMPION, score/password terminal screen.
These are artificial progress shortcuts, not proof of an ordinary winning playthrough.
An earlier paused recipe and mistaken adjacent$0577 unpause write did not advance;
those failures are preserved. See redirect/r01-consumers-20261105/research.md,
completion-scheduler.py/final-arena.py, runtime.jsonl and selected PNG captures.
