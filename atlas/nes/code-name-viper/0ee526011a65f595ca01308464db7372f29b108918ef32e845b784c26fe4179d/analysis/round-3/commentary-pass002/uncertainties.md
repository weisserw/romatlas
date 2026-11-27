# Round02 commentary questions and established limits

These are semantic or exact-boundary questions; they do not skip current-source
readability work or weaken classified structure. All ROM/source/assets remain local.

- **L011 — resource selector versus physical bank.** `SelectRoomResources`, physical
  124955..125158, reads descriptor/resource selectors and the four split pointer
  pairs at126812..126884. A selector's numerical equality with a stage or PRG-bank
  index is not established. Follow descriptor byte two and bank-selection consumers
  with explicit effective PRG slots before assigning particular terrain/graphics
  identities. Generic non-code containers already have independent structural review.
- **L011 — stage-seven PPU address alias.** `SaveMapPpuBytes`,15623..15654,
  indexes two stored seven-byte arrays at15654..15668. Stage seven aliases the first
  high-array byte as its low address and opcode$a5 at the following code boundary
  as its high address; fourteen-bit PPU masking gives$2500. It saves156 PPU bytes
  in scene RAM. Original shared-byte intent versus tolerated overread remains unknown;
  do not invent an eighth stored record or classify PPU contents as CPU ROM code.
- **L012 — checkpoint and room/spawn streams.** `RestoreCheckpointRoom`,
  126174..126305, selects PRG10/11 and indexes CPU$b746/$b747 by stage times eight
  OR checkpoint bits$06, installing `CheckpointStreamLo/Hi` ($a2/$a3). Follow these
  consumers into the remaining bank10/11 streams with exact finite record/terminator
  bounds. Bank0 `FindSpawnRecordList`/`FilterSpawnRecord` and fixed `$d3c4` row pointers
  provide separate room/object-record leads. Six/eight-byte record observations do
  not justify a single format for every residual range. Reuse saved evidence;
  unchanged gameplay replay is not the proposed check.
- **Player/actor meanings.** State transitions, action buffering, fixed-point motion,
  probes, item counters, collision side and script actors are explained mechanically.
  Exact item identities, some terrain bits/contact consequences and natural eligibility
  for later scenes remain uncertain. Actor/player address tables do not prove every
  target implementation or natural selection. The isolated shared RTS at physical
  $58bc and single$ea after the stage-bank table remain structurally unknown.
- **Audio, graphics and generic data.** Existing sound-engine record/command prose
  remains valid; original music/effect names and some score-level semantics remain
  unknown. Graphics/property-like containers were inspected through known consumers,
  repeated records and boundaries, but inner packing and specific resource identities
  are not invented. CHR comprises8192 sixteen-byte tiles; artwork names are unresolved.
  The233 explicit unknown intervals total68886 bytes and remain code/data ambiguous
  or lack exact endpoints. Readability notes do not classify them.

Resolved questions carried into source:

- **L007:** `MatchPasswordRecord`,112978..113088, scans84 candidates although only54
  six-digit records are stored. All30 extra candidates contain a byte outside0..9,
  so the digit-only editor cannot match them. Metadata arrays remain54 entries.
- **L008:** `FlushScrollBuffers`,117711..117750, reads immutable opcode$a2 at$c6ab.
  `$a2 & $08` is zero: BNE is not taken and the column-attribute call executes after
  a pending column upload. Original intent of this constant condition is unresolved.
- **Frame helper:** physical129246..129257 (`WaitMarkedNmiFrames`, CPU$f8ce) waits X
  marked NMI frames. Transition callers use$14 (twenty); it is not mapper selection.

Evidence: analysis/pass-002/review.md; analysis/redirect/round01-01/findings.md,
password-overread.json and password-follow.json; analysis/leads.json L007/L008/L011/L012.
