# Terminal handoff: justified incomplete ROM analysis

The coordinator workflow reached its documented terminal incomplete handoff after two disassembly/commentary/redirect rounds and independent rejection review. Current-source commentary and final editorial acceptance are complete. **ROM classification is not complete.** No active phase or blocked operation remains.

ROM classification: 100.00% (524298 / 524304 bytes classified; 6 bytes unknown).

The percentage is rounded (approximately99.998856% before rounding). Code48,874; Data213,280 (including the16-byte header); Image262,144. Every classified byte has structural evidence. Raw export emission is not counted as classification. [Per-bank counts](round-002/classification-counts.json) and the [exact final ledger](redirect/round002-20261031-a/unknown-ledger.json) retain physical identities and CPU context.

| Physical interval, end exclusive | Fixed PRG bank / CPU alias | Remaining ambiguity |
|---|---|---|
|245776..245779|30 / $C000|Three jump-shaped bytes targeting reset $FE98; no independent executable-header or bounded-data role proof.|
|255289..255292|31 / $E529|Three jump-shaped bytes targeting state setter $E614; local transition bypasses them, but that proves neither dead code nor Data.|

Effective fixed-bank context is [unknown,unknown,30,31]. The [coordinator rejection review](coordinator/rejection-review.md) inspected primary raw bytes, current source, control edges and competing interpretations under evidence-policy.md. Missing natural callers were not imposed as a universal prerequisite. The [worker review](redirect/round002-20261031-a/rejection-review.md) and [avenue dispositions](redirect/round002-20261031-a/results.md) preserve exact negative-result scope and the corrected bank14 base-plus-phase counterexample. No ready structural lead or productive interrupted frontier remains. New independent entry/template or bounded-data evidence may reopen the corresponding interval; arbitrary PC execution or unchanged gameplay does not supply that discriminator.

## Current source and verification

- Maintained source: [commentary/source/game.asm](../commentary/source/game.asm); [reading copy](../commentary/reading-after.asm).
- Full48,874-byte Code scope, all Data/Image and explained Unknown boundaries reviewed.2,465 distinct meaningful units are represented by25,503 membership records; these are not25,503 newly invented routines.
- [Official final audit](../commentary/audit.json): ok=true, editorial_audit=true, byte_identical=true;25,632 scaffold records reviewed. [Editorial findings](../commentary/editorial-audit.md), [final acceptance handoff](handoffs/terminal-editorial-acceptance.md) and [final results](terminal-editorial/final-results.json).
- Source SHA256: badaa8e4be4cd064d4c533321436ef01b18d0a4acfdda69c6d67eb11f662b2a2.
- Reading-copy SHA256:6dc23fb15c60cfd3e3ec4cff64adb26f5230da971c30cb73a5b9fb0353055871. Actual expansion equality was checked.
- Current map/internal state SHA256:8c90238b3ebc1e65c9ea29eb0c92fe1d942bc7fc1e7e8a798945cc58fa5902ac. [Verified map checkpoint](../maps/pass-002-disassembly-verified.json), [rebuild report](pass-002-disassembly-verified-verification.json), [compact reload/equality proof](round-002/compact-reload-verification.json).
- Input ROM SHA256 remains6a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b. All source/asset bytes stay local; final source edits changed74 comment lines, not instructions/directives.

Earlier commentary trees, failed/superseded checkpoints and investigation records remain preserved. The final phase handoffs are in [analysis/handoffs](handoffs/); exact progress and lead history remain in [progress.md](progress.md), [investigation.md](investigation.md) and [leads.json](leads.json). These records distinguish source readability, structural classification, annotation provenance and observed execution.

## Gameplay and limits

[Scenarios](scenarios.json) preserve valid press/release and bounded runtime recipes, snapshots and captures. Natural first gameplay and artificial password/progression routes cover representative early/middle/late missions, Queen/victory/home and scrolling named credits. Active password editing was separately tested. The artificial scene-success entry is explicitly distinguished from a legitimate boss defeat. A separate visible THE END capture remains attempted, not demonstrated; other mission/dragon permutations are deferred for diminished structural prospect. No exhaustive gameplay or full hardware/game compatibility is claimed.

Code annotation provenance is Observed13,485 / Static35,389 bytes. Retained execution evidence is a separate17,107 instruction bytes /8,485 physical opcode starts, including artificial scenarios and later-observed Static records. Unknown game identities and original mixed-resource organization retain useful mechanical descriptions and uncertainty in [commentary/uncertainties.md](../commentary/uncertainties.md).

## Toolkit and archive preparation

[Toolkit report](toolkit-issues.md): two confirmed bugs (NES-001 input schema, NES-002 map persistence size) and one missing performance feature (NES-003 source-location caching), no blocker. Use complete buttons arrays for NES input. Keep the compact12,205,564-byte state/map: supported save/export may recreate an18,607,346-byte pretty file exceeding the16MiB reader limit; lossless compaction preserves every field. The failed pretty checkpoint remains as evidence. The official uncached audit was allowed to finish without restart. No toolkit implementation changes or clean-worktree claim were made.

[archive-options.json](../archive-options.json) explicitly selects additional final Markdown and omits only byte-identical duplicate operational templates. [Document audit](coordinator/archive-document-audit.json) and [selection rationale](coordinator/document-selection.json) account for unrecognized Markdown under the actual archive.py contract. [Completion audit](coordinator/completion-audit.md) and [machine audit](current-audit.json) record requirement-level evidence.

**No archive execution, cleanup, workspace deletion or publication was performed or authorized.** The supported ROM-free portable mapping is maps/current.json; internal state, traces, snapshots, restored source/assets and ROMs remain local and are not substituted for that sharing deliverable.
