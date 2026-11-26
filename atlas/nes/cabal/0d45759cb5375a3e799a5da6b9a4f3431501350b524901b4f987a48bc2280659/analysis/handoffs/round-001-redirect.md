# Round001-a redirect handoff — combined research batch ready

Redirect resumed the existing unfinished attempt. Research phase complete; next
phase is independent disassembly review of the combined proposals. No terminal
exhaustion or whole-ROM completion claim follows.

Accepted checkpoint remains pass-001: 117404/131088 classified (89.56%),13684
unknown bytes in162 intervals. ROM SHA256 remains
0d45759cb5375a3e799a5da6b9a4f3431501350b524901b4f987a48bc2280659.
Maps/current and analysis/state remain byte-identical to frozen commentary baselines
(handoff-consistency.json). No map/source/toolkit edits, deletion, archive or sharing.

## Review package

- reviewed-code-proposals.json / reviewed-static-graph.json / walk-reviewed.py:
  3149 proposed instruction starts,6412 bytes, including21-byte E05A correction
  already classified as Data. 6391 proposed previously unknown code bytes. Keep
  per-edge reasons; JSR continuations require callee-return/bank review. No blanket
  graph acceptance. Proposed instruction alignment has no overlap.
- supplemental-code-proposals.json:18 common-tail JMP bytes plus separate2-byte
  B0 $990C cursor-routine prefix. The latter has weaker independent routine evidence
  and must be judged separately, not accepted to force coverage.
- data-proposals.json:7273 bytes of consumer-backed tables, bounded packed-stream
  suffixes/prefixes, mixed patterned data and uniform FF runs, each with exact
  physical bounds, effective banks, references and rationale. Generic Data remains
  sufficient where schema/meaning unresolved. Do not label mixed data as fill.
- contracts.md: BRK stack convention; finite directories; isolated loader/copy
  contracts; graph corrections; localized tiny-range review; progression results.
- unknown-dispositions.json: exact original162-interval complement, neighbors and
  per-interval proposal dispositions. No accepted residual reduction yet.
- handoff-consistency.json: proposed6411 unknown Code +7273 unknown Data =13684,
  plus21 existing Data correction; no proposal overlaps. This arithmetic is not
  evidence of acceptance. Historical intermediate graphs/residual files preserved.

## Connected findings and closed/static avenues

BRK B0 continuation is BRK+1 after IRQ stack transform/RTS, not an inline byte.
Common tail bank save/restore and bounded RAM indexed writes permit local static
continuation beyond conservative discovery stops. Direct callers $808F/$81AB prove
the $E05A..$E06E stub deserves code review despite prior generic-data heuristic.
Targeted direct-control audit found no other classified-data target conflicts.

Reviewed service-directory handlers, their ordinary continuations/shared helpers,
transfer dispatchers, all64 masked player handlers, audio and prompt handlers, and
associated tables as one connected graph. Corrected bank1 front directory to9
entries; previous index9 read instruction bytes. Removed B3 $8105 apparent loop
fallthrough: four-byte offset table read by $80E2. Mask AND7 proves8 prompt entries;
AND3F proves64 player entries. RAM0B initializer4 and +2/wrap1A prove13 transfer
entries for both banks. Audio dispatch has a second <80 source; do not claim a
16-selector global bound.23 adjacent pointer entries and compatible handlers are
a distinct extent/contract proposal, independent of natural selector production.

Independent mechanical PPU loaders/copy contracts and stored player handlers have
been followed through their useful continuations; known packed asset boundaries,
lookup records and common tails are included. Additional game-semantic naming or
complete selector histories do not discriminate Code/Data further. Broad raw-body
scan is not repeated without a concrete new control/reference contradiction.

Final boundary review resolved B3 $86F2..$86F4 via AND3/read$86F1,Y, including one
already-Code RTS byte as dual-use table entry. B0 $990C..$990D proposed LDY$A4 feeds
complete cursor increment/wrap/store tail; known $987D/$9884 path supplies identical
Y contract into$990E. Alternative two-byte constants lack a read consumer, but
independent reviewer may still find static role insufficient. No missing caller is
being imposed as a universal prerequisite.

## Progression investigation and actual deferrals

NMI-freeze premise rejected by interrupt/DMA/bank-switch/accepted-fetch evidence.
Natural interlude at$82DA..$82EB requires exact Start edge10; existing unpause
experiment exits it and captures actual soldiers. Start gating is resolved locally.

Four later level/scene patches render distinct scenery. Sustained combat, bosses,
natural final victory and highscore terminal remain unconfirmed. Actual later
blocking site is sprite0-hit split synchronization at$946D/$9470. Full attempts:

1. Resume completed-load snapshots, release/Start/release then movement/fire.
2. Inspect PPU/CHR/OAM, compute sprite/background overlap; relocate known sprite
   tile49 via RAM0200=[16,73,2,32], restoring each snapshot first.
3. Seek actual instruction-boundary PC946D/9470 then setPC9484, preserving entered
   routine stack/WaitNMI/RTS and asserting Start; execute bounded continuation.

All four scenes fail sustained combat confirmation under both revised methods,
though extra actor helpers execute. Static overlap ignored full rendering/scroll
timing, so its failure does not establish a toolkit defect. Further unchanged
Start/NMI patches, coordinate sweeps or repeated skips lack a new discriminating
prerequisite. Full loader/render-timing contract is missing and further immediate
unknown-structure prospects diminished after the independent static batch. This is
the actual reason for gameplay deferral. Artificial ending display from earlier
work remains separate; no natural ending claim. Scenarios and negative histories
are durable in analysis/scenarios.json and level*-{resumed,sprite-shortcut,split-wait}-*
artifacts. Scripts record inputs, patches, banks and budgets; timeout/operation-budget
stops are partial results, not execution proof beyond recorded accepted fetches.

Other apparent opportunities now have proposals/constraints rather than being
reserved for another research cycle. Disassembly should review this batch, classify
only supported parts, then full current-source commentary and redirect. If it rejects
a localized claim, preserve the decisive reason so next redirect can assess a
corrected premise rather than repeat this graph unchanged.

No toolkit issues found in this resumed research; existing toolkit report retained.
No toolkit changes. Disk2.4GiB free, attempt89MiB, scratch632KiB; no cleanup performed.
