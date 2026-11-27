# Field cursor / unit contract

PRG-RAM $6000 count16, followed by16 records of8bytes (idle-unit-records.json);
record examples blue IDs14/15 at x5,y17/19, ID6 at x6,y20, ID54 at x8,y18;
red records have different flags. These are observations, field meanings provisional.
Fixed-bank routine near CPU$EF2E copies zp04/05 to viewport origin cache038D/038E;
other consumers use04 parity. Field reuses title state05 as map Y. Initially04=7,05=20.
Source/read inspection in field-main-decode.json, static contract files; exact immutable
ROM scans located writes to038D/E (physical1EF3E/1EF43).
After turn intro fully returned to main stack253, left12/release4/A8/release20
visibly moved cursor left (map1-ready-left-select.png) and entered a selection
response (command box being redrawn). Earlier directions during message printing
were ignored or entered redraw paths; do not assume initial camera fields are unit coords.

PRG-RAM editing not supported: NES emulator.rs392-409 rejects spaces other than
ram/cpu and any end>2048. Internal RAM cursor and loaded object mirrors remain
practical, while direct $6000 health edits are unavailable through this API.

Targeted cursor patch verified: RAM$04=6,$05=20 followed4 unpressed frames,
A6/release18 loaded unit mirror$03B7-$03BE = [6,6,20,87,248,0,16,129],
exactly PRG-RAM ID6 record (including x,y), displayed soldier portrait and unit
selection menu setup. Object-under-cursor$03AE=$85, command menu$03B2=4.
This is artificial cursor-state execution, not a natural reachability claim.
Strong useful continuation: read bank6 $B1FB (jump808A) to verify mirror writeback;
controlled loaded-mirror position edits could move one unit near an opponent through
ordinary game commit. Do not infer health semantics or terminal flag from raw values.

Correction after primary decode: bank6$B1FB is *turn/side progression comparison*,
not mirror writeback. It compares$03F6/$03F7 against$03F3/$03F4 and updates$03FA.
Do not use its entry for record commit. $B1C6 counts units with nonzero high/low
nibbles in record byte4 and flag bit$40 in record byte1, distinguishing unit groups.
Actual mirror writeback still requires identifying its direct consumer.

Role correction from actual tactical display: ID6 is BLUE AH-1S helicopter, and
redID101 is INFANTRYMEN (battle-ready.png), not an infantry-vs-tank matchup.
Earlier portrait-based infantry wording was provisional and incorrect. Selected
8-byte mirror/cursor mechanics remain directly verified; health/stat semantics
remain unresolved. Tactical strength15 is displayed for both participants.

Commit prerequisite correction: F3B7-F3F6 matches both full coordinate bytes of7001/2
against world6002/3; ID alone is not its lookup key. Original battle relocation
changed selected result coordinates without editing world record, so ordinary commit
later loops atF3D2. Tactical display/resolution proof stays valid, world return does
not. Corrected producer retains selected mirror's world coordinate bytes; cursor
alone controls remote artificial target lookup. Strength1/ammo14 (1E) replaces0,
so a real damage/death transition is sought rather than assuming initial zero is dead.
