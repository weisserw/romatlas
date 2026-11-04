# Round 002 current-source uncertainties

All current classified code/data and the sole unknown byte have source explanations. Names describe mechanical roles; exact enemy/tile/music/image identities remain unresolved where evidence supplies no game name.

- The only structural residual is physical $3C70, CPU $BC60 in effective PRG slots [0,1,2,3]. Source places an exact-byte note between the removal-state tail JMP and UpdateTrackingObjectType. No direct/encoded current entry discriminates RTS code from padding/data. Redirect L006 may reopen only with an independent entry/extent constraint; arbitrary-PC execution would not establish intended role.
- Former L001/L002 continuation questions are resolved for the accepted finite initialized domains: InitializeCountedAnimationRoots consumes N plus N little-endian pointers; supported scene callbacks preserve animation cursor/slot stack; local MMC3 writes here affect CHR only. Arbitrary bytecode/callbacks, type13+ and state12+ remain outside those conditional summaries. The prior questions below are historical, not current missing-code claims.
- SetAnimationPaletteGroup takes slot, pointer low, pointer high. Correct targets are physical $2445/$2449/$244D. DecodeSignedFiveBitParameter returns Y=$FF only on negative extension, otherwise zero. These corrections are in source.
- PlayerStateImageSlice $2B63..$2B66 and ObjectCountdownSlice $441A..$441D are supported physical lookup slices, not complete selector/table domains. FixedSpriteHorizontalOffsets $4ED1..$4ED3 has the exact two-item domain X=2,1. Their encoded bases lie in preceding code bytes and remain code.
- Audio F7 alternative frequency tables, unrooted audio fragments, original image ownership/full fixed-image selector domain and the generic tile-pattern tail have honest local format limits. No DMC sample asset layout is inferred merely from register commands.
- Metatile plane/attribute origins give useful consumer views into $3440..$38EB, but original combined table boundaries and several exact tile meanings remain unresolved. CHR bytes remain planar pattern backing without per-image ownership claims.
- Reset source uses effective slots [unknown,unknown,2,3] until registers6/7 establish [0,1,2,3]. A carried identity execution alias does not prove the reset-entry bank mapping.
- Natural boss defeat and terminal credits remain unproved. Prior ending observations came from the artificial initialized clear-transition route; source timing and visible hold loops are explained without claiming natural completion.

No classified-source readability gaps remain after the round002 verification. Final editorial acceptance remains reserved; this does not complete whole-ROM classification.

## Preserved round001 uncertainty record (historical)

# Current-source uncertainties and redirect questions

The classified source is explained throughout. Unresolved meanings below do not
change its structural roles. Unknown spans have individual physical boundary notes.

- Inline script entry CPU $801a lies in unknown physical $0010..$0092. Calls into
  it can consume return-adjacent bytes, including the continuation near physical
  $1b74. Determine its stack contract and stream termination before extending
  executable fallthrough. Retain L001 from the disassembly handoff.
- PPU transfers are a separate established inline-parameter mechanism: entries
  `TransferRepeatedPpuData`, `TransferAttributePpuData` and `TransferPpuData`
  read a two-byte descriptor pointer after JSR and adjust the stacked return
  address. The source explains this rather than labeling its payload as code.
  Their physical entry area is $50f3..$5125; some immediate/column branches
  still cross unknown intervals. Use the exact transfer mode and descriptor
  shape when following a new continuation.
- `ApplyChrBanks` (physical $539e..$53ae) writes MMC3 registers 0..5, hence CHR
  selection. This establishes its local register role, but does not establish
  that every caller or indirect script handler preserves PRG context. Retain
  L002 and inspect indirect dispatch and inline continuations before adding
  a safe return summary.
- Zero-page $88..$99 and several object/animation arrays are temporal overlays.
  Names beginning Shared spell out the established alternatives. Work15..Work23
  are routine-local scratch whose roles are explained by probe/render/transfer
  consumers. A symbol is not a claim that the byte has one persistent game role.
- Object types are mechanically packed as active/type/direction; tracking,
  reversing, facing and timed-projectile labels describe algorithms. Exact
  enemy names, object-state names and several tile-class meanings remain unresolved.
  Collision probes document thresholds and carry results without assigning unsupported
  names to tile codes.
- Generic data physical $3440..$38eb retains the disassembly's checked non-code
  layout; consumer/object boundaries and exact tile purpose remain unresolved.
  The uniform zero tail $7fc8..$7fff is non-code; original fill intent is unknown.
  CHR backing is planar tiles; no per-image game meaning or original asset layout
  is inferred.
- Audio code establishes note/envelope commands, separate stream stacks, APU
  shadow updates and effect priority. Some $e0/$e1/$e2/$f7 parameter destinations
  and initialization continuations remain unclassified; no unsupported instrument
  or sample layout is inferred.
- Ending loops were sampled after an artificial clear-transition entry. Their
  timing/rendering is documented; natural boss defeat and terminal credits remain
  unproven. Title/attract setup names are tentative where only local setup mechanics
  are visible.

Final editorial acceptance and full-ROM classification remain separate and pending.
