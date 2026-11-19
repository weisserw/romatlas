# Battleship gameplay notes (round 001)

Identification: local text at file $3D52 onward names MINDSCAPE, BATTLESHIP, 1993; mapper3 32KiB PRG/32KiB CHR. NTSC profile works. Copyright screen at 180 frames, title START/CODE at 362 frames.

Sources consulted (no ROM/assets downloaded):
- https://www.digitpress.com/library/manuals/nes/Battleship.pdf — NES manual: choose CODE, Start; directions edit four decimal digits, Start submits. Weapon descriptions. Applicable NES version, menu controls confirmed locally, code routes being tested.
- https://etherealgames.com/nes/b/battleship/passwords/ — 8 ranks with 5 battles each; 8-1=9617, 8-5=8397, ending=1992. Hypotheses until local acceptance.
- https://consoleclassix.com/nes/battleship.html — A places/confirms/fires, B rotates ships, Select chooses special weapons; confirmed rotation/placement in local deployment.

Observed: choose START after title with Start; wait battle banner and ship deployment. Title at frame362; Start1/release8, Start10/release90, wait180 yields deployment frame651. A places a ship if it does not collide with previous placements; down moves placement, B rotates. Full first deployment reached ACCEPT SETUP at frame805. An exploratory write RAM $84=6,$85=0 after that screen had no established progression effect, and marks subsequent route artificial. Accept with A8/release8 reached attack initialization. Preserve natural replay up to frame805 separately from artificial continuation.

Static shortcut contracts: $97=rank (0..7), $98=battle within rank (0..4), $99=absolute battle index; main progression $8C64 compares $99 to $28 and transitions to ending. Password digits $0661..$0664 read at $B4D3, packed and searched in 43-entry table $BFDD..$C032. Two extra codes set $065A/$065B. Prefer patching digits while CODE editor active and submitting Start over jumping into arbitrary routines. Stage RAM changes alone do not prove a level visit.

Completed checks:9617 was accepted with $97=7,$98=0,$99=35 and later fleet deployment rendered; A placement,B rotation,and movement exercised (late-mechanics.ppm).1992 reached ship/fireworks ending(ending.ppm) and A/release advanced animation. Both digit-injection routes are artificial, with ordinary supported submission and actual subsequent CPU fetch evidence. First combat attack animation rendered(fire-result.ppm); exact weapon selector not separately confirmed. No natural full campaign completion is claimed. Durable local replay at analysis/evidence/replay-round001.py; exact inputs/releases in scenarios.json.
