# Round 001 commentary handoff

Current-source readability complete; whole-ROM classification remains incomplete.
The pass-001-final map and state are unchanged: 256690/262160 classified, 5470 unknown.
No structural annotations or reachability claims were added during commentary.

[Buildable source](../commentary/source/game.asm),
[refreshed reading copy](../commentary/reading-after.asm),
[review inventory](../commentary/review.json),
[fresh verification](../commentary/verification.json) and
[readability checks](commentary/readability-checks.json).
All 737 scaffold ranges are reviewed (338 code, 260 data, one CHR image, 138 unknown),
with 512 compact units. Source has 502 specific routine/continuation explanations,
1125 context-resolved code/branch symbols and 225 RAM/hardware aliases, plus typed
object fields, masks, controller buttons and CPU table constants. Unknown ranges
retain honest local notes; generic data does not claim invented internal schemas.

Coverage includes every object-handler fragment and both object pools, sprite
allocation/clipping, audio/ending actors, reset/NMI, stage/player lifecycle, MMC1,
map/palette streaming, input, projectiles, terrain, collisions, score and queued audio.
All classified targets of the established object/audio/terrain/collision dispatch
tables have specific explanations. Reviewed corrections include the distinct $8500
terrain-response and $8600 occupancy-shape roles, internal RTS/no-op entries,
stack-unwinding audio handlers and shared scratch aliases. Existing frozen comparison
artifacts and phase-1 evidence remain intact.

Fresh supported verification: ok=true, byte_identical=true; source SHA-256
8292800b9b7ca4fad4f5e65d9f3d8b2e8098c225ace6c07ad771ac34b55448fb.
Input SHA-256 is unchanged. Reading copy refreshed after that final source edit.
Editorial audit remains pending for final acceptance; no audit/global completion claim.

Next worker: continue existing [R001–R003 leads](leads.json), using
[uncertainties](../commentary/uncertainties.md) and exact [unknown ranges](unknown-ranges.json).
R001: follow six-byte dispatcher/synthetic returns and intervening payloads
(source DispatchFirstObjectPool:432, ExpandObjectMetasprite:2808); sound-rule slices
at CPU $b995 include unknown interruptions. R002: distinguish ordinary command returns
from explicit stack unwinds (DispatchAudioCommand:6085, AudioStopMusicChannel:6105,
AudioStopEffectChannel:6329), and follow the three ending actor state dispatches
(UpdateEndingActors:4740) from saved snapshots. R003: narrow fixed islands by their
neighboring consumers, especially the ObjectSoundBridge at CPU $fe00
(source:13831); do not generalize an entire-gap rejection to independent data slices.
These are structural questions, not remaining readability backlog.

No toolkit issues found; zero findings/blockers:
[toolkit-issues.md](toolkit-issues.md). No archive, cleanup, deletion, publication,
ROM modification or toolkit edit. All generated/restored source and assets stay local.
