# Archive repair

The archive failure was caused by stale commentary/source/mapping.json: its SHA256 was 0cecafc728ba0e4f20d8722f3e6771ce0f38a3ff89e6318a0d41cbac0a4e2f07 (round005), while the frozen round006 baseline and current map use 6f60cafafa039c6f901acf0666aa2fa81db83a96594ca1181105c9497520248b. The archive recipe accepts unchanged baseline assets or original-ROM slices; this stale JSON satisfies neither.

Backed up the stale source metadata and original archive options in analysis/archive-repair-backups/. Synchronized only source/mapping.json to the existing frozen baseline.json. The compared differences are the previously accepted region 14751 (physical 71932, CPU $98EC, banks [8,9,14,15]), 61 existing region comment corrections, and next_id. This reconciles the ancillary map with already reviewed source and final conclusions; no new classification or review credit was introduced. All other non-assembly source assets already match before/. No assembly, includes, labels, comments, frozen baselines, original ROM bytes, current map/state or review metadata were edited.

Added this final document to archive-options.json, retaining every existing document selection and omission. Backups and evidence remain local. Verification results will be recorded below. Classification remains incomplete: 34 unknown bytes in 28 ranges; gameplay and semantic limitations in the existing final records remain applicable. Only the disposable probe destination is authorized for this test.

Strict commentary audit rerun passed: ok=true, byte_identical=true, editorial_audit=true; 14779 reviewed regions and 18892 reviewed embedded unit associations. Full result: analysis/archive-repair-audit.json. Reading copy and verification records refreshed by the supported audit command. Preserved-file hash check passed for all 22 recorded files.

The disposable archive probe below is required to reconstruct the exact source tree and verify its rebuilt ROM before publication. Its result is retained locally in analysis/archive-repair-probe.json.
