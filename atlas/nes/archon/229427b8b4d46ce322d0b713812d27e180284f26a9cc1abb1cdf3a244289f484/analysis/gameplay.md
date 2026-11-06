# Archon game familiarization

ROM title image reads ARCHON, Activision/Bullet-Proof Software 1989. Local image identifies NES release; region/revision not independently identified. Board strategy and arena combat are distinct systems; no scrolling levels expected.

Confirmed locally: emulator.input for this NES adapter requires complete buttons arrays, e.g. {port:1,buttons:["start"]}; release {buttons:[]}. Copyright/title attract advances without inputs. From title Start enters menu; A selects 1 PLAYER; A selects LIGHT SIDE; A selects LIGHT FIRST. Snapshots r01-actual-menu, r01-options, r01-board, r01-firstboard record these stages (the latter names precede visual confirmation; consult PPM images). The rendered empty framed board after this route is a loading transition, not a completed gameplay visit.

Targeted web search found NES strategy guide https://gamefaqs.gamespot.com/nes/587093-archon/faqs/11891 and screenshots https://www.mobygames.com/game/73/archon-the-light-and-the-dark/screenshots/nes/ . Search excerpts describe Light/Dark pieces, combat, revive spell and winning by all five power points. Only excerpts consulted; these are hypotheses until locally tested. Version/revision applicability uncertain. Cross-platform manuals surfaced but are not taken as NES button evidence. No ROMs or extracted assets downloaded.

Planned representative milestones: strategy board and movement; arena combat and firing; wizard spells; alternate side/demo mode; endgame/win display. RAM shortcuts may accelerate battle/ending once board representation and transition contract are understood; avoid arbitrary brute force.

Board RAM contract verified by fixed initializer F37D and lookup CFD2: $044C..$049C is9x9 row-major board; $80 means empty. $61 is row and $63 column, with60/62 prior selected coordinates. Natural initial row4,col1 is a light knight03; row4,col0 light wizard08. Selection callback rejects outside0..8. The source selection flag $05ED is waited upon in D3E4; setting80 exits it. $0615 is a startup-selection delay (decremented in CB34), $0614 input debounce. These are source-grounded artificial shortcuts, not natural reachability proof.

Successful artificial combat route: import r01-before-shortcuts.snapshot; write60..63=[4,4,1,1],0615=0; A3frames release20 -> r01-knight-selected.snapshot. Write0472=14 (dark knight on adjacent row4,col2),61=4,63=2. A3 release45, clear0614 then A2release40 did not enter arena; final05ED=80 +80frames DID enter arena. Its rendered arena and fighter visits are r01-force-selection.ppm, r01-arena-b.ppm, r01-arena-combat.ppm; A+right60 and B+up30 then A+right60 exercise representative controls, all artificial. Effective banks switch for graphics loading, but fetched code remains physical banks0,1,14,15. Do not infer code in mapped graphics banks merely from mapping changes.

GameFAQs direct retrieval was restricted; search excerpts remain limited hypotheses. No further online walkthrough claim is used for annotations.
