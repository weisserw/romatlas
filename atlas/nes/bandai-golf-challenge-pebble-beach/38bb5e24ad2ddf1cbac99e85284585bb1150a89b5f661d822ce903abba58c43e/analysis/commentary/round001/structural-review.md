# Independent commentary structural review

Input is frozen pass-001-final, map hash
401371b3a2cf4bc85c43e2696f369e3fefa76e0e853acccd920a0e82aed52db9.
All physical offsets include the sixteen-byte header. Fixed CPU PRG slots are
[0,1,2,3]; CHR banking does not change this mapping. No map/state edits were made.

Read the entire canonical instruction source, all 84 region evidence records,
`analysis/disassembly/round001/dispatch-evidence.md` and `data-inspection.md`.
Rechecked the dispatcher stack mechanics and every inline word against classified
Code, plus the terrain response words. The ten inline tables consume their JSR
return and therefore do not have executable fallthrough. None needs an invented
natural selector domain to support its structural extent.

The three operand entries are real coherent alternate streams, not errors to
linearize away: CPU $87D5 is LDX $00A9, whereas $87D6 is LDA #0; CPU $8CDF is
LDA #$68, whereas $8CE0 executes PLA; CPU $A040 is LDA $6868, whereas $A041
executes two PLAs. Their callers/branches and convergent stack effects explain
both streams. Source preserves primary bytes and documents alternate execution
with exact equates. Neither stream is dismissed as dead.

Reviewed every non-code interval against its adjacent code endpoints, accepted
indexed consumers and visible record/value patterns. `data-windows.json` retains
bounded per-256-byte window summaries and long-run boundaries for all Data/Image
regions, independently recomputed from this ROM. Broad tail physical $3261..$8009
has sound period/parameter/descriptor/command material first; physical $3710 is
the CPU $B700 course-pointer base; the following windows exhibit the repeated
literal/run grammar decoded by $9314. Attribute pointers at CPU $F3C2 and six-byte
expansion records at $F563, followed by height strips at $FB5D, have separate
consumers. The entire tail was examined in bounded windows; it is not inferred
from lack of execution or from byte-identical emission. Long $FF runs are exactly
physical $3664..$370F and $7F6D..$8009. They support generic Data without asserting
original linker intent. No code-versus-data or boundary contradiction found.
Specific stream and record boundaries within that combined annotation remain
unresolved and are not invented by the source notes.

Header and CHR bounds match iNES extents; each CHR bank is 8192 bytes. CHR has the
NES planar tile organization, but CPU code explicitly reads CHR via PPUDATA for
course loading. Image classification therefore describes its PPU storage role,
not a claim that every byte is exclusively visible art. No CPU PRG entry is created
from these PPU data reads.

One phase-1 semantic defect was corrected in readable source: physical
$0556..$0557 contains $44/$6A, which are zero-page setup destinations consumed by
STA $0000,Y, not sprite coordinates. Source now calls it
SetupChoiceDestinationBytes and uses twoPlayerMode/teeChoice consistently.
The existing portable label HandicapCursorPositions and its old coordinate prose
remain frozen history; Data role, two-byte extent and provenance are unchanged.
Required metadata refinement before final acceptance must correct that label/evidence in the current map,
not rewrite this frozen baseline. Similarly the nineteen bytes at physical
$11B1..$11C3 are course-strip boundaries, made explicit by consumers in source.

No structural rejection or redirect investigation is needed to finish this phase.
Open meanings concern original units, display intent and sound/course subformats,
not unsupported Code/Data substitutions. Natural callers/selector reachability
remain separate from static structure, especially SetRenderingMask and alternate
entry domains. No full gameplay or hardware compatibility claim follows.
