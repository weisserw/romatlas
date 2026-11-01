# Round 02 commentary handoff

Full CURRENT-source readability is complete; whole-ROM classification is INCOMPLETE.
Classification: **100.00% rounded (262153/262160 bytes), seven unknown bytes**.
Checkpoint pass-002-disassembly-20261031-a; map/state unchanged at SHA256
ff89defe97828358320b8a0b360c796abc3e5924cd63b1894c98f36c6fd6c359.

Covered all 39,602 Code bytes/18,218 instructions, 91,479 Data bytes, 131,072 CHR
bytes and five unknown intervals. All 8,271 scaffold entries reviewed with fresh
source links; 677 code units and 409 named data/image structures. No readability
ranges deferred. Final editorial/archive audit remains PENDING, reserved for the
coordinator after redirect; it was not invoked during commentary.

Preservation/carry: entire prior 28-file commentary directory is at
analysis/round-02/commentary-round01/. Carried 7,522 exact instructions, 350 prose
starts and 21 older data/image explanations. Invalidated only five C01 Static
records/13 bytes; five three-byte variant arrays now explain $14010..$1401F.
Added 328 curated code/helper explanations, 268 descriptive data names and coherent
routine/branch/RAM/field/CPU-alias names. Source is independently maintained, not a
replacement by raw generated assembly. Research scripts and compact listings are
local evidence, not portable/shareable mapping.

Concrete readability examples in commentary/source/game.asm:
- line16919 BrawlQuantizeDirection: quadrant/ratio reduction to a 32-step direction;
  neighboring velocity conversion explains signed multi-byte components.
- line29389 ShootingDispatchAndActorHelpers: nonzero state dispatch, type-selected
  tables, hit flash, child allocation and motion. $04C1 is state; $04D1 is type.
- line37647 DrivingVariantArrays: corrected five arrays and mode2 entry correlation.
- line39653 DrivingUploadRasterTemplate: exact 153-byte DAE5→0600 copy and ROM/RAM distinction.
- line51084 ProjectionQuantizedRows: 32×128 stride, quantization and index128/border behavior.
- line51129 EncodedDataPool_18010: bounded generic fallback, no invented DMC sample spans.
- line63571 SetGameplayState: BIT-overlapped E84B LDA#3 execution alias explained.
- line70954 PlaySoftwareDACStream: bitwise DAC steps, pacing/count, bank/NMI restoration.
Footer units have explicit zero-based physical bank-tail names (01/03/05/07/09/11).
Generated target ownership is bounded by preserved/new physical logical-unit intervals;
ClipObjectComponent_Iny_04170 at line9071 retains sprite ownership, not footer ownership.

Remaining uncertainty is semantic, explicitly explained: song/instrument/sample identities,
full inner audio/PPU command schemas, original actor/sprite names, exact projection generator,
some object flags and generic constant grouping. Bounded primary consumer/neighbor/format
inspection was reused. Valid metasprite/scene/dialogue/screen formats and finite extents are
explained; natural selection of every record and natural final boss defeat remain unproved.

Unknown half-open physical ranges remain exactly:
$C1E2..$C1E5, $C7A6..$C7A7, $14379..$1437A, $1ACD0..$1ACD1, $1EE00..$1EE01.
Plausible JMP/isolated RTS encodings do not distinguish code from stored scalar/padding.
Naming E84B's overlapping entry adds no supported entry at residual C1E2. No new
structural redirect premise arose; see commentary/uncertainties.md for bounded limits.

Fresh final byte-identical rebuild: commentary/verification.json. Reading refreshed:
commentary/reading-after.asm. Source hashes: game.asm
c5d387b296d0e49aa63d650660be5ab8cad3bb0b195e0d3ca866a41aa31134ed;
symbols.inc 582db20960dcf3c79b8195328b855f5652a88675cd4765be80de2b8682ce300f.
ROM SHA256 remains a6a548c4a27d3961821312a95e45ecda0eea36e247ee2d963079c336b23882c2.
No toolkit edit/new issue, map/state mutation, archive execution, publication, cleanup/deletion
or download. Existing NES-001 remains tracked in analysis/toolkit-issues.md.
