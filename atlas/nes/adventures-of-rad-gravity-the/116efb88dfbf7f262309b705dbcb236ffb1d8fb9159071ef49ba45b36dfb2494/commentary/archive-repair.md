# Archive repair

The failed archive rejected a newly authored quoted dialogue token: its recipe accepts baseline references and editorial metadata, but cannot introduce that assembly string literal. Inspected workspace configuration, final progress and handoffs, parent job diagnostic, and the archive.py/commentary.py contracts.

Normalized 17 dialogue records in commentary/source/game.asm from quoted ASCII to equivalent hexadecimal byte operands, keeping the complete readable text as a same-line comment. Existing zero terminators, labels, line counts, source locations, bank context and surrounding explanations are preserved. No source was regenerated. Backups of the original game.asm, audit/verification records and archive options are local under analysis/archive-repair-backups/; changed-ranges.json records line locations and source hashes. Independently compared every changed line's emitted bytes against its original ASCII plus terminator and reviewed its retained text. No other source lines changed.

Added this final document to archive-options.json; retained all existing selections and omissions. Original ROM, frozen baselines, maps, classification, review credit and historical evidence remain unchanged. Historical handoff hashes describe their original snapshots; current hashes are in commentary/audit.json and verification.json.

Verification: strict commentary audit and disposable archive probe results are recorded below after execution. The probe uses the requested archive.py argv, appending the current document and omission options, with destination /tmp/romatlas-archive-probe-ayxr2knf/archive. No real atlas destination is touched.

Residual limitations: whole-ROM classification remains incomplete by three bytes at physical 0x1C74D, 0x1C74E and 0x1E93D. Existing semantic and gameplay limitations and final findings remain in the selected documents. No new research, toolkit change or compatibility claim accompanies this spelling repair.

Strict commentary audit passed: ok=true, byte_identical=true, editorial_audit=true; 2864 regions and 2876 units reviewed. Reading copy refreshed by the audit. Review metadata needed no edits because all source line locations and meanings were preserved.

Disposable archive probe passed with ok=true, exact_source=true, byte_identical=true and editorial_complete=true. The archive regenerated a temporary baseline, reconstructed the exact authoritative source tree, rebuilt it byte-identically, validated map/store, and included this repair document. Full command/result is retained locally in analysis/archive-repair-backups/probe-result.json. Repeating the same probe after this documentation update ensures the final document is included. Original ROM/frozen identities and exact refreshed reading expansion were independently checked.
