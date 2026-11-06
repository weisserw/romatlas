# Arch Rivals — gameplay familiarization

Local NTSC NES ROM: copyright screen names Arch Rivals, Acclaim and Rare; mapper7. Exact revision unidentified.

Sources: [NES manual](https://www.digitpress.com/library/manuals/nes/Arch%20Rivals.pdf) (PDF available but screenshot fetch partly timed out); [Take on the NES Library](https://takeontheneslibrary.com/finished/146-arch-rivals/) (controls/progression account, 2020). External descriptions remain hypotheses until tested.

The account describes a single four-quarter match, followed by score/statistics screens, with inter-quarter advertisements/tips and halftime cheerleaders. There are no level-based campaign milestones. Team selection uses B, player count Select, Start advances and confirms player selection; A opens instructions. In play, A shoots/jumps, B passes on offense and charges a punch on defense (release to punch). These controls motivated the bounded scenarios; full move semantics have not been isolated.

Confirmed local route: load `analysis/round-01/start-game.snapshot` (natural attract instructions), press Start for60 frames, release60; repeat four times using **NES buttons arrays**, then release and wait600 frames. Result: court, score0-0, first quarter03:49, Tyrone vs Vinnie. Saved clean state `court-clean.snapshot`; frame `court-clean.png`. Earlier scripts using the shared protocol's button/pressed schema supplied no input and are attract playback only (NES-001).

Source-backed progression shortcut: ASCII clock digits RAM $059e-$05a1, quarter index $05e6, tick divider $05d4. Static consumer at physical $26c4-$278e checks zero clock, processes quarter-end events, and selects a screen through $fc. Experiments retain a clean court snapshot and patch only these bytes; all resulting execution is artificial, not natural reachability. See progression.py and scenarios.json.
