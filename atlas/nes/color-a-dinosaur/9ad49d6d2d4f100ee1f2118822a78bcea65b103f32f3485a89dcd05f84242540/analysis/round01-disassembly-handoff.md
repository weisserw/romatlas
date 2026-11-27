# Round1 disassembly handoff


## Incomplete; ready for commentary then redirect

ROM classification: 98.99% (129764 / 131088 bytes classified; 1324 bytes unknown).

Internal map: 149 regions; 4156 code bytes, 125608 data bytes. Every classified region has structural evidence/provenance; all code regions have explicit physical-to-CPU views. One Reset label only: full-current-source commentary is the next phase. Classification is separate from semantic readability and actual fetch coverage. The26 unknown intervals are all in physicalbank14; [exact complement/counts](evidence/classification-final.json). No whole-ROM completion or full compatibility claim.

| Physical8KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 0 | 8192 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 4156 | 2712 | 1324 |
| 15 | 0 | 8192 | 0 |

[State](state.json), [portable current map](../maps/current.json), [frozen checkpoint map](../maps/pass-001.json), [generated source](../generated/pass-001/game.asm), [verification](pass-001-verification.json). map.validate succeeds; final rebuild is byte-identical. ROM SHA256 remains9ad49d6d2d4f100ee1f2118822a78bcea65b103f32f3485a89dcd05f84242540.

Structural evidence: reviewed vector/observed-connected6502 code, fixed bank14; banked data file[16,114704) shows planar/repeated graphic patterns, bank-start bus-conflict values0..7, long zeros and DOS utility string residue. Fixed code$CF35/$CF1A reads banked bytes through($59),Y into$2007. This supports generic non-code payload, not a uniform asset format or original provenance. [Bank review](evidence/banked-data-review.json). Tail[120184,131082) follows final sound RTS and shows numeric masks/runs, tile-index/bitplane patterns, repeated note-like records and zeros; sound references$EF47/$EF5A/$EF6D corroborate data use. [Tail review](evidence/fixed-tail-review.json). Generic data labels intentionally avoid unsupported precise schemas. Initial zero runs retain separate maximal boundaries.

Execution:2,015 distinct fetched physical instruction starts,2,643 contextual roots,2,881 contextual edges in final aggregate; these include the separately marked artificial$4D experiment and are not natural-only coverage.3,863,344 accepted fetch events; aggregate_dropped=0. Detailcapacity0 intentionally retains no event detail; dropped detail count6,146,629 is not aggregate loss. [Trace summary](evidence/trace-summary-final.json), [roots](evidence/contexts-final.json), [coverage summary](evidence/execution-coverage-final.json). Long initial requests timed out after partial progress; server state was inspected and same session continued, without reset.

Progression: copyright/title, sixteen-picture grid and stegosaurus/point palette visited naturally. B/Select changed palette selection; successful fill, Start-return, alternate picture and freehand movement remain unverified despite bounded attempts. RAM$4D=0 at frame851 was tested under saved clean snapshot and failed to move coordinates or change display through879; restored natural851 state and released buttons. [Exact scenarios/statuses](scenarios.json), [gameplay/manual](gameplay.md). Boss/level/ending assumptions inapplicable to available coloring-activity evidence.

Handoff: [ready leads](leads.json) cover direct fixed-bank gaps, concrete input/dispatcher gate investigation, and optional data-role refinement. No prior leads existed; this pass neither rejected nor exhausted these avenues. Coordinator requested handoff after coherent supported gains, so no terminal exhaustion claim/rejection audit is made. [Investigation ledger](investigation.md).

Toolkit: no toolkit issues found. No blockers or toolkit edits. [Issue report](toolkit-issues.md). Disk at final inspection 3.6GiB free; scratch48KiB and selected evidence29MiB; nothing deleted. All internal state/assets/captures/source remain local.
