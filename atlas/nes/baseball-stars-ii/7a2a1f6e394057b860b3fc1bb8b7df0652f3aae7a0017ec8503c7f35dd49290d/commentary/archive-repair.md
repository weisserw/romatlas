# Archive repair record (2026-11-17)

## Diagnosis

`scripts/archive.py archive` failed with "changed non-assembly asset cannot be archived: mapping.json".
The archiver regenerates the baseline tree from the frozen `commentary/baseline.json` and requires every
non-assembly file in `commentary/source/` to equal its baseline counterpart (or a ROM slice).

`commentary/source/mapping.json` was a stale copy carried forward from the round-4 commentary tree. It
corresponded to the pass-004 map (sha256 077d58cd...; 4,107 regions, next_id 5155). The frozen baseline,
`commentary/before/mapping.json`, `maps/current.json` and `analysis/state.json` are all the pass-005 map
(sha256 d09c3179...; 4,111 regions, next_id 5159). The only difference was the four user-directed MANUAL
fallback regions 5155-5158 (8 bytes, generic Data of unknown meaning). All 4,107 shared regions were identical.
The assembly sources already carried the manual-fallback headers and were byte-identical; only the
sidecar metadata file was out of date.

## Edit

- Backed up the old file to `analysis/archive-repair-backups/source-mapping.json.pre-repair`.
- Replaced `commentary/source/mapping.json` with the frozen-baseline mapping (byte-equal to
  `commentary/before/mapping.json` and to a fresh `romatlas generate` from `commentary/baseline.json`).
- No `.asm`/`.inc` source, label, comment, review entry, baseline, map, evidence or ROM byte was changed.
  No bytes were reclassified; the manifest and frozen baseline hashes are unchanged.

## Verification

- `commentary.py verify`: ok, byte_identical true.
- Strict `commentary.py audit`: ok, byte_identical, 4,111/4,111 regions and 3,453 units reviewed,
  editorial_audit true.
- Disposable probe archive (`archive.py archive ... --output /tmp/...probe...` with every `--document`/`--omit-document`
  from `archive-options.json`): ok, exact_source true, byte_identical true, editorial_complete true
  (4,111/4,111 regions, 3,453/3,453 units, editorial audit reviewed, no reasons).

## Residual limitations

Unchanged from the final handoff (`analysis/handoffs/round-005-terminal.md`): 8 bytes rely on the
user-directed manual fallback rather than evidence; readability limitations are in
`commentary/uncertainties.md`.
