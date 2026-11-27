# Commentary uncertainties for redirect

These questions do not prevent explaining the classified mechanics. Current map
classification remains unchanged; all source/ROM assets stay local.

- **Password loop bound**: `MatchPasswordRecord`, physical 112978..113088,
  compares the record index against hexadecimal$54 (84). The accepted narrow
  table is 54 decimal records at113329..113653 with three 54-byte metadata arrays.
  Inspect the next 30 candidate records, their metadata addressing, and the code
  boundary near113671. Distinguish actual overread behavior, unreachable indices,
  or a misunderstood count; do not extend classification from the loop limit alone.
- **ROM-byte condition**: `FlushScrollBuffers`, physical 117728..117738, reads
  CPU$c6ab and tests bit3 before one attribute upload. This aliases an instruction
  byte in fixed bank14, not a RAM flag. Confirm immutable bytes and surrounding
  control flow; it may be an intentional constant condition. The listing retains
  the numeric ROM read and makes no patch or unsupported bug claim.
- **Scene/ending states**: `CheckPlayerStageTransitions` and
  `FinishGameplayTransition` route states$1c/$1e to unresolved helper$f8ce with
  X=$14. This is not mapper selection. Continue existing L004/L005 boss/late-map
  investigations using their snapshots; do not call those states an ending yet.
- **State/terrain meaning**: bank2 player dispatch and collision code establish
  motion, rectangle overlap and terrain routing, but exact state names, terrain
  flags, jump/climb/crouch semantics and contact consequences remain partly unknown.
  Named mechanical roles can guide bounded state/terrain inspection.
- **Banked resources and audio**: consumer-named CPU table equates describe
  accesses without classifying unknown backing bytes or declaring table extents.
  Audio bank 12 uses 31-byte channel records, concurrent stream ownership, packed
  note/envelope/modulation commands; original music/effect identities and all
  command meanings remain unresolved. Do not merge CPU aliases across banks.
- **Generic non-code and CHR**: uniform PRG intervals retain bounded structural
  evidence; their original fill purpose is unknown. CHR is 8192 sixteen-byte tiles,
  with individual artwork meanings and arrangement unresolved. Unknown 108019 ROM
  bytes remain code/data ambiguous with explicit interval notes in source.
