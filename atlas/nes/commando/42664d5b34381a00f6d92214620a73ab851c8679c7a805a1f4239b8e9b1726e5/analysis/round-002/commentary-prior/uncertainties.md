# Current-source uncertainties — round001

All currently classified source has been read and annotated mechanically. These
questions describe remaining meanings or structural contradictions, not skipped
readability. The full-ROM editorial audit remains pending because classification
is incomplete. The current map and classification evidence remain unchanged.

- **Palette bytes also classified as code:** physical $01C22B..$01C24B,
  CPU $C21B..$C23B, fixed effective banks14/15. UploadFlashPalette reads16 bytes
  from this base with X=0 or16, independently establishing32 palette bytes.
  Existing Code records extend beyond that table through physical $01C254.
  Source explicitly flags the dual role and retains byte-exact instruction
  spellings. Disassembly should inspect fetch evidence versus conservative branch
  fallthrough before changing classification; the table consumer does not prove
  that execution can never overlap data. The unconditional outcome of the preceding
  BEQ after a16-iteration DEY loop is a concrete static review route.
- **Seven candidate projectile slots versus six updates:** TrySpawnProjectile
  initializes X=6 and scans down through0 (RAM $0200..$0260, stride16), whereas
  UpdateProjectilePool and TestProjectileObjectHits visit six records. This is
  exact control-flow behavior, but the role of the seventh candidate is unresolved.
  Inspect the missing spawn initialization and callers before assuming an off-by-one
  bug or increasing a pool bound.
- **Supported later-stage transition remains unknown:** MissionIndex=$049D and
  AreaIndex=$049E are named consistently. This source pass does not recover the
  initialization/continue contract and does not validate earlier RAM patches.
  Retain round001 disassembly’s stage-load question and failed recipes unchanged.
- **Stack scheduler and dispatch tables:** SaveObjectContinuation/ResumeObjectContinuation
  pop and replace return addresses. DispatchInlineWordTable uses address-minus-one
  words adjacent to its call. Their mechanics are described; not every inline
  table/caller is classified. Do not treat pointer values alone as code evidence.
- **Overlaid record fields:** common position/state/direction fields are named.
  Fields10..15 are sprite pointers or velocities depending on record type;
  fields7/19 have multiple timers/auxiliary uses. Numeric type-specific offsets and
  neutral counter names preserve this distinction. Unknown game purposes are stated
  at the consuming routines rather than assigned speculative entity names.
- **Banked data and generic records:** graphics banks1..7 retain the reviewed2bpp
  tile role,512 tiles per8KiB bank. Banks8..11 retain generic non-code record data
  after the disassembly’s bounded pattern/consumer review. Record schemas and all
  upload consumers are unresolved. Uniform runs retain generic Data, without a
  claim of linker fill or unused storage. Source carries the evidence and bounds.
- **Partly classified routines:** every unknown incbin has physical bounds and an
  explicit unresolved structural-role note. Comments on adjacent fragments describe
  only their visible mechanics; hidden side effects, complete call contracts and
  natural reachability are not asserted. TerrainQuery returns a two-bit code;
  game-facing names for codes0..3 remain unresolved.

No new toolkit issue was found during commentary. Existing nonblocking NES-001 is
retained in ../analysis/toolkit-issues.md.
