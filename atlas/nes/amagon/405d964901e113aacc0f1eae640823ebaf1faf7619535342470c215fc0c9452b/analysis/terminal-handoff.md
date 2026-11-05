# Terminal handoff — justified incomplete ROM analysis

ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).

**ROM classification remains incomplete despite rounding.** The coordinator workflow
has reached its documented terminal-incomplete handoff after rejection review and
final current-source editorial acceptance. There is no interrupted phase, ready lead
or identified useful revised method. This is not full hardware/game compatibility,
complete game semantics or proof of every possible execution route.

## Exact result and checkpoint

Code19010, Data59461, Text614 and Image52000 physical bytes are classified. The header
is included. Physical PRG8K banks0–13 are fully classified; bank14 has8190 classified
and2 unknown; bank15 has8191 classified and1 unknown. Unknown gaps and raw fallback
were excluded from classified counts.

| Physical byte | Fixed CPU alias | Current role |
| --- | --- | --- |
|117608|$CB58|Unknown $60|
|122120|$DCF8|Unknown $60|
|127720|$F2D8|Unknown $60|

Each fits return-only Code or a non-code boundary byte. Prior JMP/RTS instructions
exclude sequential fallthrough only. No independent entry/dispatch or data grammar
currently discriminates the alternatives. Absence of direct references or retained
fetches does not prove Data, and natural reachability is not universally required
for static classification. Forced PC/JSR would supply the disputed role rather than
test it. See the [coordinator rejection review](coordinator/residual-rejection-review.md),
[terminal research decision/addendum](coordinator/terminal-research-decision.md) and
[final redirect handoff](round3-redirect-handoff.md). Reopen for a concrete independent
structural constraint or justified corrected inference; do not repeat unchanged planning.

Current state/current map/final checkpoint/frozen commentary baseline SHA256:
`0ca03ff1a520d52325c68310613d72f5bc5fc9f74812c7716b7e0a98a696b960`.
The final checkpoint is maps/round3-disassembly-final.json; internal state is
analysis/state.json and portable export is maps/current.json. They remain local.
The original raw maps/baseline.json is separately preserved; the redirect3 equality
list's baseline path typo is corrected in the coordinator addendum above.

Input ROM SHA256 remains
`405d964901e113aacc0f1eae640823ebaf1faf7619535342470c215fc0c9452b`.
No ROM, extracted asset, internal state or source was published. No archive or cleanup
was executed. Toolkit implementation was not modified.

## Readable source and verification

Current local buildable source: commentary/source/game.asm and symbols.inc.
Reading copy: commentary/reading-after.asm. All5453 scaffold regions,814 compact
meaningful units and7250 embedded unit intersections are reviewed. Unknown meanings
have useful mechanics and explicit limits. Final targeted actual-source review fixed
a base-volume comment and an offset-name reused as a bit mask. The earlier false NMI
root and display stride annotation are corrected in current state/source; historical
records and prior commentary snapshots remain preserved.

Official commentary/audit.json reports byte_identical=true, ok=true and
editorial_audit=true, with5453 reviewed regions and7250 embedded units. Separate
read-only editorial status is complete=true with no reasons. Final map/store
validation also passes. [Current final audit](current-audit.json) records the exact
counts, map/source identities and validation scope; [editorial handoff](final-editorial-handoff.md)
and commentary/editorial-audit.md describe actual readability checks rather than
using byte identity as proof of meaning.

Final source SHA256:
- game.asm: ab538bf392d027a90eb36cdb69c5bd847aadde4069c1ddf71ca02c48abd7ae3f
- symbols.inc:597c411b68101d02fcc60323b3fa679a253622a066775107659ca5951665de65

Representative natural startup and artificial later-area/mechanics/boss/boat/terminal
ending/Megagon routes are in scenarios.json. The terminal screenshot was inspected;
all cited scenario evidence exists. Scenes3/5/7/9 remain explicit same-zone deferrals.
Laser damage at zero score is unproven. Broader exhaustive gameplay is not claimed.

## Preservation and future work

archive-options.json explicitly selects additional final/history Markdown and omits
only duplicated copied workflow templates. The complete selection contract and
[document-selection verification](coordinator/document-selection-audit.json) account
for every otherwise unrecognized analysis/commentary Markdown document. Final results,
handoffs, investigation reasoning and historical conclusions are preserved. Omission
is selection metadata only; no files were deleted. This prepares future archive
work without running archive.py, publishing, or authorizing cleanup.

[Completion audit](coordinator/completion-audit.md) links requirement evidence;
progress.md, investigation.md and leads.json retain every round and lead outcome.
Eight leads are verified and none is ready. [Toolkit report](toolkit-issues.md) has one
nonblocking confirmed NES input-documentation mismatch; use replacement buttons arrays
and [] release. No toolkit repair was made. This terminal result remains locally
resumable if a genuinely useful new constraint appears.
