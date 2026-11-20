# Commentary progress — round 002 current-source readability complete

Checkpoint round-002-final; maps/current.json, analysis/state.json and frozen
baseline.json SHA256:
8b9f176e93f6c9b137dee1440e49653ab5916fe0e1d2380cc255db2385825b65.
The entire prior commentary is preserved at analysis/round-2/commentary-round-001.
Preparation and the carried physical-range/dependency batch rebuilt byte-identically.
The active-pass history is preserved in analysis/round-2/commentary-progress-active-history.md.

All current classified source is named/explained, including the older carried
backlog and every newly accepted graph. All 2,054 physical scaffold entries and
1,819 meaningful compact units are reviewed; no current-source readability range
or internal naming batch remains pending. Source has 31,899 lines and 280 exact
symbol assignments. Scope: 43,751 Code, 113,456 Data, 99,328 Image and 5,625 honestly
explained Unknown bytes. Classification remains 97.85%, independently incomplete.

Coverage includes player input/actions/death/retry/physics, small-object and weapon
spawning, room lists and camera-timed spawns, area map routes/choices, equipment
menus and tile uploads, grapple attachment/climb/swing/release and segmented lines,
actor movement/contact/capture/link/spawn/defeat patterns, moving platforms,
cinematic state/fade/overlay sequencing and terminal static display, audio command
interpreters, fixed scroll/metatile/collision/PPU/OAM/MMC1 helpers and all resources.
Names consistently distinguish RAM fields, ordinary slots, the separately checked
display slot, temporary field reuse, hardware and mapper values versus physical
8 KiB bank identities. Added 331 missing accepted target names and resolved 1,058
numeric control operands using physical identities and supported bank contracts.

Concrete corrections preserve actual behavior: allocation presets select slots
11..18, 8..10 and 6..7; slot-four overlap overwrites a table load with constant $25;
one reset loop reads rather than clears scratch; one entry overwrites Y=$10 with
$18; a death branch overwrites A=$81 with $02 without storing status; some attack
completion paths execute two draw/animation passes. Grapple references, reciprocal
links, position backups and panel addresses explain repurposed fields locally.

Only four numeric control operands remain, because their target bodies are unknown:
physical $3D7D6, $39217, $37F46 and $36BA7. Their callers and exact bank/boundary
limits are explained in source and commentary/unresolved-control-addresses.json.
Concrete bounded questions are published in
analysis/round-2/commentary-redirect-questions.md. Narrative actor/weapon/story
identities and unparsed resource schemas remain tentative where stated; the natural
final shot and complete timed credits remain unverified. None is an omitted source
readability batch. uncertainties.md supersedes affected round-one premises.

Final fresh commentary.py verify: ok=true, byte_identical=true. Source SHA256:
- game.asm: 11ce7aa52164f4efbbb782b9af4b4165387fec91c1998d796614a48c99316b1e
- symbols.inc: af466475009ab712d82cea1f3caa049974600c355727922a1973372ed3d473a3
ROM SHA256 unchanged:
f5f1e212b70e858b3b0257161bc4c25a390602dacd12d5c0877a971ce8a18567.
reading-after.asm freshly refreshed; SHA256
58935a4a7f33dbb583c4b4a89e592269e1cc2d470d50c6269a9569092fdc5258.
Full completion facts/examples are in analysis/round-2/commentary-completion.json.
All review source locations refreshed. Final editorial audit remains pending and
was not run; source readability does not establish whole-ROM classification.

Reported NES-DOC-003 for generated workflow's stale Game Boy wording; preserved the
generated text and corrected the local NES workflow. No toolkit edits. No ROM/map/
classification edit, replay, deletion/cleanup, publication or final archive work.
Disk 4.1GiB available; cited evidence and 180KiB analysis/scratch retained locally.
