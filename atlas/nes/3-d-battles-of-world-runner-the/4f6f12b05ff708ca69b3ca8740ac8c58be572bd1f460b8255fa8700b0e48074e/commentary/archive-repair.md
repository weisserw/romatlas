# Archive repair

The prior archive failed because the ROM-free reconstruction recipe cannot encode new quoted assembly data literals absent from its generated baseline. The first rejected operand was the pause message. Inspection found thirteen such text operands in QueuedTextMessages.

Normalized only game.asm lines 21312, 21315, 21318, 21321, 21324, 21327, 21330, 21333, 21336, 21339, 21342, 21345 and 21348 to equivalent hexadecimal byte operands. Original text remains in inline comments, including spaces. Labels, existing comments, record coordinates, terminators, line counts and source locations are preserved. No assembly regeneration, classification changes or new review credit were needed. Pre-edit source, reading copy, verification, audit and archive options are preserved locally under analysis/archive-repair-backups; invariants.json records immutable artifact hashes.

Reviewed all thirteen changed lines and surrounding record layout, and independently checked each numeric sequence against the original ASCII string plus NUL. The strict commentary audit performs a fresh byte-identical rebuild, validates the existing reviewed scaffold/units and refreshes the reading copy. Its current result is recorded in commentary/audit.json. All recorded ROM, map/state, review and frozen-baseline hashes remain unchanged. Historical handoffs retain their original source hashes; current verification.json identifies the repaired source.

archive-options.json retains every prior selected document and adds this report, with no omissions. Only the disposable /tmp/romatlas-archive-probe-gwtetxzs/archive destination is authorized for the archive test. The probe checks exact source reconstruction and a byte-identical restored build; its machine result is retained locally in analysis/archive-repair-backups/probe-result.json.

Structural classification remains 131075/131088 bytes, with thirteen unknown bytes in eight fragments. Existing semantic, historical and gameplay limitations remain in the final handoff and uncertainties. No new disassembly/research, toolkit edits, downloads, real-destination publication or workspace deletion were performed. Editorial completion was not required for this repair; existing acceptance is retained subject to the fresh strict audit.

Fresh strict audit passed: ok=true, byte_identical=true, editorial_audit=true, 7483 reviewed regions and 7692 reviewed nested unit attachments. Repaired game.asm SHA256: 55608ebb29a0a23e82c7bdedee25e9b60c4f1d38f386fc30ea0289d1398a8d0c.

Disposable archive probe returned ok=true, exact_source=true, byte_identical=true and editorial_complete=true. No residual archive blocker remains. This report is included in the final probe rerun; the real atlas destination is untouched.
