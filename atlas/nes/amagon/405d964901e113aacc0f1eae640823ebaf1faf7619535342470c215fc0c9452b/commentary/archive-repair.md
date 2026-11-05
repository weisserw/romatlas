# Archive repair

## Diagnosis and minimal repair

The archive recipe rejects changed non-assembly assets unless they are original-ROM
slices. commentary/source/mapping.json was carried from round 2: only region 5333
(physical 88655..88767) comment and evidence retained the superseded stride-seven
interpretation. Current maps/current.json, frozen commentary/baseline.json and
commentary/before/mapping.json already agree on fourteen eight-byte storage records.
The annotated assembly already describes that correction.

Reconciled just those two metadata strings with the accepted frozen mapping; the
result is byte-for-byte equal to that mapping. No assembly, symbols, ROM bytes,
classification, review credit, frozen baseline or historical findings were changed.
The old source mapping and archive options are preserved locally under
analysis/archive-repair-backups/, alongside the exact two-field diff and protected
file hashes. This backup is local evidence, not a portable mapping deliverable.
archive-options.json additionally selects this final report; previous document
selections and omissions are retained.

## Review and verification

Reviewed the affected mapping record and existing source/game.asm lines 3503–3592:
DEY followed by three ASLs computes 8*(incoming Y-1) modulo 256; the copy loop stops
at seven bytes; fourteen eight-byte records follow. No source-code edit was needed.
The metadata repair preserves physical identities, CPU/bank views and all other
fields. The corrected evidence preserves uncertainty about natural selectors and
the skipped byte. Fresh strict commentary audit passed: ok=true, byte_identical=true,
editorial_audit=true, 5453 reviewed regions and 7250 embedded units. It refreshed
the reading copy. Protected-file hashes confirm assembly, symbols, review metadata,
ROM, current maps/state and checked frozen artifacts are unchanged. Receipts are
retained locally in analysis/archive-repair-backups/.

The disposable archive probe verifies regenerated-baseline reconstruction, exact
restored source-tree equality and a byte-identical ROM rebuild before publishing
its staged result. The probe returned ok=true, exact_source=true,
byte_identical=true and editorial_complete=true with no review-status reasons.
Only /tmp/romatlas-archive-probe-6h5oouf7/archive was published. The probe is rerun
after this report update so the archived report matches the final workspace.
The final probe receipt is archive-probe-result.json in the local backup directory.

## Residual limitations

Classification remains incomplete: exactly three bytes at physical offsets 117608,
122120 and 127720 remain Unknown. Final handoffs, historical conclusions and semantic
limitations are preserved. Byte identity does not establish full gameplay or hardware
compatibility. No shared toolkit change, unrelated research, download, real atlas
publication or workspace deletion was performed. Only the authorized disposable
probe destination is used for archive testing.
