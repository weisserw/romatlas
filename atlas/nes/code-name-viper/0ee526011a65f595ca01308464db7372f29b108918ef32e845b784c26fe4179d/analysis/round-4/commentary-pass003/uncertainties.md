# Round03 commentary questions and established limits

Current source readability is complete. Classification remains separate:119 intervals,
20618 bytes, are unknown. Their exact ranges are analysis/pass-003/unknown.json;
source notes preserve their physical identities without assigning an invented role.

Resolved structural questions now integrated into source:

- L011: descriptor resource selectors and stage-selected physical PRG banks are
  independent. The initial stage associations are documented at SelectRoomResources;
  room descriptors are16 records of31 bytes. The $8700 placeholder aliases the next
  pointer grid and is not a seventeenth descriptor. Stage-seven PPU $2500 alias
  mechanics are established; original intent remains unknown.
- L012:32 checkpoint pointers select29 unique six-byte-record streams ending in $ff.
  FindCheckpointItemRecord compares the four-byte checkpoint key. Bank-zero spawn
  streams instead contain558 eight-byte records and219 sentinels; stage-four has13
  stored words. No shared format is inferred for every room/resource payload.
- L013/L014 and return contracts: only the explicit literal bank selector and stated
  fixed services establish changed effective banks. B6DA preserves two/three for
  initialized player slot zero in states $15/$16/$1e. Audio $854a consumes its caller
  return address to dispatch through seven or ten inline words, which are Data.
  Audio channel records initialized at $0780 with31-byte stride justify their RAM
  indirect stores; no generic mapper-writing or arbitrary-pointer permission follows.

Concrete redirect questions:

- Which supported actor instance gives $0504/$0506 and current animation constraints
  for the remaining dynamic metadata/dispatch paths? Reuse initialized slots and
  finite writer transitions; inspect only the resulting bounded target set. The
  behavior maps and88-entry split groups do not enumerate all handlers as Code.
- For mixed banks two/twelve and fixed-bank residuals, can a particular current
  classified edge establish an entry and effective bank/stack contract? Generic
  $8000/$8001 writes remain barriers; retain isolated $58bc RTS and $1fe68 EA unknown.
- For remaining repeated non-code candidates, inspect a narrow exact boundary,
  nearby accepted consumer and independent byte-domain/record constraints. Do not
  require original asset names or generator recovery to support generic Data.

Original sound semantic limits and historical questions are preserved below. Their
old233-interval count and unresolved L011/L012 structural frontier are superseded by
the resolutions above; other game-meaning caveats remain applicable.

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
