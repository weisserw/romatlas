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
