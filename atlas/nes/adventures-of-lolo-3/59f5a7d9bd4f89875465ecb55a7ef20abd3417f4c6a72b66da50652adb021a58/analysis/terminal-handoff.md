# Terminal handoff — pass005

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).

The coordinator workflow has reached a justified **terminal incomplete** result. Exact classification is 99.9980927677754%; rounded 100.00% does not mean whole-ROM completion. Five physical one-byte intervals remain unknown: [67407,67408), [68116,68117), [70230,70231), [86712,86713), [120565,120566). Each contains $60. Their retained-return and literal interpretations remain materially ambiguous.

## Current deliverables

- Checkpoint: `maps/pass-005.json`; `maps/current.json` and `analysis/state.json` match it. Map SHA256 `9ed86bef8db138cb02991ec2a718f4febe4c972a607a962a32558b900843601a`.
- Readable assembly: `commentary/source/game.asm`, with local build assets; refreshed expanded reading copy `commentary/reading-after.asm`. Source SHA256 `1eb0117282ff62bb8d9bd57521e9e3bf9e4e7fc32d0ac8230a754ba52cc3aba3`.
- Final editorial acceptance: `analysis/final-acceptance/HANDOFF.md`, `commentary/editorial-audit.md`, `commentary/audit.json`. All 1868 scaffold entries and 2032 meaningful units reviewed, zero pending. Five unknown notes are reviewed explanations, not classifications.
- Current gates: `analysis/current-audit.json`. Configured-binary map/store validations pass; fresh final audit reports byte-identical rebuilding; reading copy exactly matches current expanded source. Archive editorial-status check is complete with no reasons outstanding.
- Stopping evidence: `analysis/redirect/r5-return-contract-review/rejection-review.md` and `primary.json`; coordinator inspected actual bytes, current source, stack contracts, accepted comparison cases and strongest alternatives. No useful corrected inference or discriminating next test remains identified on current evidence. No ready leads remain; historical outcomes stay in `analysis/leads.json` and phase records.
- Broad progression: `analysis/scenarios.json` and `analysis/redirect/r1-node-and-residual/` preserve representative late areas, bosses, rescue/story and THE END, including exact artificial-state routes. These do not claim natural reachability.

## Limits and preservation

The first four residual bytes follow native tails; the fifth separates IRQ RTI from music entry. Independent boundaries are established, but their structural role is not. Neither legal RTS decoding nor lack of execution decides the role. The accepted terminal review does not prove the bytes are unused or unresolvable in principle. Resume only with new evidence, a justified changed inference/test, or explicit reassessment; do not repeat unchanged exhausted searches automatically.

`input/game.nes` remains unchanged with SHA256 `59f5a7d9bd4f89875465ecb55a7ef20abd3417f4c6a72b66da50652adb021a58`. No toolkit edits/rebuild, ROM downloads, archive execution, publication or cleanup/deletion occurred. Current source, assets, traces and internal state remain local. Only the supported ROM-free portable mapping is a sharing deliverable; internal state is not a portable map. No full hardware/game compatibility claim follows.

`archive-options.json` selects additional final Markdown and explicitly accounts for disposable duplicate instruction documents. Final conclusions, review history and handoffs are preserved. This prepares reviewable archive inputs; it does not execute an archive. Existing toolkit reports NES-DOC-001 and COMMENTARY-LOC-002 are retained locally in `analysis/toolkit-issues.md`; workarounds are applied and nonblocking.
