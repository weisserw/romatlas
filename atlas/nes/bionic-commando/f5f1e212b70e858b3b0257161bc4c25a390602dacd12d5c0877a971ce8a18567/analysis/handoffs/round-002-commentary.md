# Round002 commentary — complete current-source readability

Checkpoint **round-002-final**. Map/state/frozen commentary baseline SHA256:
**8b9f176e93f6c9b137dee1440e49653ab5916fe0e1d2380cc255db2385825b65**.
Map, classification and ROM unchanged. Prior commentary preserved in full at
analysis/round-2/commentary-round-001 before fresh preparation; 547 identical
classified physical intervals and valid dependencies were carried and reverified.

**All current source readable:** 2,054 physical scaffold entries and 1,819 compact
units reviewed, no deferred readability range or internal naming batch.
31,899-line source, 280 exact symbols. Covered 43,751 Code /113,456 Data /99,328 Image
bytes and honest notes for 5,625 Unknown. Classification remains **97.85%**, separate
from this completed current-source gate and still incomplete.

Player, weapon, grapple, actor, room/map/menu, cinematic and audio graphs now have
purpose/state/algorithm prose and descriptive routine/internal-entry/branch names.
RAM/object fields, known slot offsets, repurposed scratch, button masks, hardware
and banked ABI calls are consistent. All new resource types explain their bounded
consumer/format evidence or retain explicit generic-data uncertainty. Older carried
material was corrected where new boundaries/contracts contradicted its prose.
331 missing accepted targets were named; 1,058 numeric control operands resolved
using physical identity and supported bank contracts. Decoder-default lower slots
were not carried through mapper helpers.

Representative source entries (source/game.asm line numbers):
- 1685: LoadNormalRoomSpawnList — room-indexed five-byte records, terminator and slots.
- 4163: DrawEquipmentFourSpriteCell — eight-byte record to two-by-two OAM block.
- 8265: TestSlotFourImpactOverlap — actual constant-$25 overwrite behavior.
- 12311: DecodeFourByteSwingFrame — packed direction/animation, signed offsets, facing/duration.
- 22930: RunRoomEncounterDefeatAndProgress — encoded damage, flags, palette/timer and mode transition.
- 28460: AllocateGeneralActorSlots — bounded allocation presets, not collision responses.

Other concrete corrections preserve read-versus-clear scratch, overwritten Y setup,
A=$81 overwritten before a status store, double draw/tick passes and differing
array strides. Grapple arc fields, reciprocal links and position backups are locally
explained. No exact narrative names or natural reachability were invented.

**Final fresh verification:** commentary/verification.json ok=true and
byte_identical=true. ROM SHA256 unchanged
f5f1e212b70e858b3b0257161bc4c25a390602dacd12d5c0877a971ce8a18567.
Source hashes:
- game.asm: 11ce7aa52164f4efbbb782b9af4b4165387fec91c1998d796614a48c99316b1e
- symbols.inc: af466475009ab712d82cea1f3caa049974600c355727922a1973372ed3d473a3
Fresh reading-after.asm SHA256:
58935a4a7f33dbb583c4b4a89e592269e1cc2d470d50c6269a9569092fdc5258.
Source locations refreshed; analysis/round-2/commentary-completion.json retains
machine counts/proof/examples. **Audit not run**; final editorial/archive acceptance
remains pending. This is not a whole-ROM or hardware/game compatibility claim.

Unresolved meanings: precise actor/weapon/story identities, unparsed generic
resource subdivisions and original schemas; natural final shot and complete timed
credits. The saved artificial terminal progression remains valid with its limits.
Only four numeric call/tail operands remain because the bodies are unclassified:
$3D7D6, $39217, $37F46 and $36BA7. Source notes, uncertainties.md and
unresolved-control-addresses.json localize them. Redirect questions and bounded
checks are published in analysis/round-2/commentary-redirect-questions.md; the last
two have explicit bank13 entry selections, strengthening existing residual leads.
Other residual questions from round-002-disassembly.md remain valid. No current-source
readability gap is being transferred to redirect.

NES-DOC-003 reports stale Game Boy wording in generated commentary workflow; local
NES wording corrected and original generated text retained. Existing toolkit issues
preserved; toolkit untouched. No ROM/map/classification edits, replay, deletion,
cleanup, publishing or final archive work. All source/assets/evidence remain local.
Disk4.1GiB available, analysis/scratch180KiB at completion.
