# Game familiarization

Title observed in ROM: City Connection (1985 Nintendo/Jaleco title screen); NTSC CNROM32KiB PRG/32KiB CHR. Exact regional marketing date/revision unresolved.

Sources consulted 2026-11-26 (external descriptions are hypotheses):
- https://tasvideos.org/5977S — NES warpless route says completing Tokyo is story endpoint although game continues; describes balloon warps and oil mechanics.
- https://tasvideos.org/6035S — NES warp route discusses three balloons and distinct stage/background progression; do not inherit its RAM assumptions untested.
- https://strategywiki.org/wiki/City_Connection — holding Up with jump rises through platforms.

Tested: Start selects 1P; emulator.input must use buttons array, not shared protocol's button/pressed example. Right/left driving, Up+A jumping and B oil were exercised in New York and artificial Tokyo. New York appears with Statue of Liberty; Tokyo frame has pagoda and Fuji. No separate boss observed. Streets are painted by driving; police/cats are obstacles (screen observations only, detailed collision contracts deferred).

Local progression contract: CPU $D216 increments RAM $A0; >=6 wraps to0 and increments $A1. CPU $D149 tests RAM $63 bit7 and transfers to that transition; patching $A0=4,$63=$80 reached score/oil bonus, then $5A=$FF satisfied bonus scroll termination at $D299 and initialized Tokyo. See shortcut.py and late.py. $17=1 patch shortened ready delay; all resulting execution is artificial. Distinct late gameplay visited; completing Tokyo to loop/ending remains a concrete redirect milestone, not a proven distinct ending.

Round001 redirect broadened artificial visits to all six stage indices: additional London1/Paris2/mountain-background3/fourth-later-background4 with ordinary initialized screens and bounded right/up/A then left/B controls; Tokyo5 was already visited. analysis/redirect/round-001-a/finish-progression.py uses supported completion(stage5,$63=$80), bonus handshake$5A=$FF and ready delay$17=1 to demonstrate wrap to New York0/course1. exercise.py crosses readiness and captures airborne player/oil projectile in stage4 and wrapped0. Exact logs/captures/snapshots remain local; all1879 saved contextual roots are artificial and within accepted Code,0aggregate drops. No separate ending/credits observed on this route; their absence is not proven. Initial long progression host exited143 after cities1..3; revised shorter route completed with saved incremental logs. Natural title-wait attempt90frames/30seconds timed out, no attract-visit claim. Source resolves counter mechanics: title JMPs shared RestoreNmiRegisters whose first instruction calls IncrementFrameCounter; correct prior commentary next phase.
