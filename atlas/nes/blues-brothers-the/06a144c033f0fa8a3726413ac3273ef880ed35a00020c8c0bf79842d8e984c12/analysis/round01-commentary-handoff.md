# Round01 commentary handoff

Current-source readability complete; next phase is redirect after coordinator review. ROM classification remains **48.71% (63847/131088 bytes)**: 12750 Code / 51097 Data / 67241 Unknown. No structural map or state changed. Final editorial/archive acceptance remains a separate pending gate; audit was not run.

Reviewed the full compact listing and all 2312 frozen scaffold entries. Source now contains 139 connected mechanical routine groups, 646 semantic routine/branch/data labels, consistent hardware/RAM/object/player/audio field symbols and useful constants. All 81 Data spans have supported role/format or unresolved uniform-pattern notes. All 155 Unknown spans have local boundary/uncertainty notes, including referenced yet unclassified palettes/dispatch/resource areas. Exact Code coverage check leaves zero uncovered classified bytes. Segmented inventory records preserve the instruction-sized scaffold; 139 logical groups are compactly recorded in [commentary-units.json](round01/commentary-units.json). See [coverage](round01/commentary-coverage.json), [review](../commentary/review.json), [progress](../commentary/progress.md).

Source examples (commentary/source/game.asm):
- Bank four auxiliary sprite shift/emission: lines1037/1130. Emitter label AND explanation follow the preceding RTS.
- Bank five pair-overlap tests and spawn grid/ring:1262/1939; PPU scroll upload:2192; OAM four-byte advance with caller-stack unwind:3339.
- Bank six type dispatch:3407; chapter resource pointers:4433. Unknown dispatch targets stay unknown.
- Fixed object 24-bit motion:5089; spawn/type descriptor decoding:7353; music voices:9619; negative audio command dispatch:9993; APU command mechanics:10586.
- Player jump:12933; camera bounds/deltas:14176; request-to-OAM renderer:15706; mapper helper contract:16316; gameplay frame loop:16803; NMI bank/register restoration:18091; graphics decompression:18184.

Final correction review checked every unit entry against physical location and adjacent instruction/return boundaries. Corrected the auxiliary emitter boundary, indirect-JMP offset accounting, negative audio command entry, extra-displacement heading, frame/linked-object alias and gravity-cap prose. Operand-constant review also kept audio timer phase two distinct from the numerically identical player-slot count. Exact encoding widths and values preserved.

Fresh final verification: [verification.json](../commentary/verification.json) reports ok=true and byte_identical=true. ROM SHA256 unchanged `06a144c033f0fa8a3726413ac3273ef880ed35a00020c8c0bf79842d8e984c12`. Current source SHA256 `470f10f1379c921f74289eec50bb1ec2b5845b51fd9004f8761a636ca4abbc85`. [reading-after.asm](../commentary/reading-after.asm) freshly refreshed after that verification. Frozen before/, baseline.json and phase1-state.json preserved. The corrected disassembly source remains historical and unchanged.

No naming/comment gaps remain in this baseline. Unresolved meanings are substantive, documented in [uncertainties.md](../commentary/uncertainties.md): exact spatial/compressed resource extents; unchecked object dispatch destinations and species; some descriptor/gate fields; fragmented collision/render continuations; audible sample identities; normal screen initialization behind artificial progression artifacts. CPU RAM helper origin is claimed only for the explicit sixteen-byte install loop, not unrelated RAM execution. No hardware/game compatibility claim follows.

Existing redirect questions in analysis/leads.json remain actionable:
1. Resource bounds: narrow physical read tracing on saved chapter transitions; relate loader source/end pointers to partial spatial/compressed resources.
2. Object handler contracts: inspect bank-six state/type dispatch consumers with five-slot Y context; pointer arrays alone are not code evidence.
3. Screen initialization: compare normal transition to artificial shortcut around FB80/PPU/scroll setup; repair entry contracts instead of repeated long blind play.

No toolkit issues found; [toolkit-issues.md](toolkit-issues.md) retained. No toolkit edits, runtime captures, redirect, archive, cleanup/deletion, publication or external communications performed in commentary. One-shot editing scripts and intermediate metadata remain local evidence; do not rerun them over maintained source. Coordinator can inspect source and verification directly; no live process dependency.
