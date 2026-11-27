# Round 01 commentary handoff

round=1 phase=commentary status=current-source-readability-complete. Ready for
redirect. ROM remains structurally incomplete; final editorial/archive audit pending.

Scope: all 607 classified code regions (6462 instruction starts), all 32 classified
data regions and explicit notes for all 123 unknown intervals. The original 762-entry
physical scaffold is intact and reviewed. 223 functional explanations plus 136 local
block notes cover the current source; 652 physical-fragment unit attachments point
to their relevant contracts. No classified-source readability gaps remain.

- [Readable source](../../commentary/source/game.asm),
  [shared symbols](../../commentary/source/symbols.inc),
  [refreshed reading copy](../../commentary/reading-after.asm),
  [review inventory](../../commentary/review.json).
- [Prose assessment](../round-01/commentary-readability-assessment.md),
  [uncertainties](../../commentary/uncertainties.md),
  [final verification](../round-01-commentary-verification.json).

Readability is assessed by explanations, not inventory counts alone. Examples:
RunObjectTasks explains the 18-slot active/timer/continuation contract;
ExtractInlineWord explains stack levels and skipped two-byte arguments;
LookupMutableMetatileTile explains layout -> mutable RAM ID -> pointer -> 5x5 tile;
AnimateCurrentObject explains five-byte records, image retries, movement/facing and
cooperative delay. Other coverage includes CHR replacement, PPU strip buffering,
player action priorities, inventory suspension/reload and three music interpreters.
Encounter fragments use established mechanical names with explicit missing-body
limits. Fourteen stage-table names now match actual consumers (map bank/root,
width/height, initial player/camera coordinates and music), without structural edits.

NMI correction: $CAB7 decrements the gate and RTIs; $CABA dispatches through the
configurable callback; $CABD is the default frame body. Reentrant NMIs skip callback
work. Preserve this distinction in subsequent commentary carry-forward.

Fresh final verification: ok=true, byte_identical=true. Reading copy refreshed after
final source edits. Final source hashes are recorded in the linked report. Input
SHA256 unchanged; internal state equals frozen phase1-state; current map equals
round-01-disassembly checkpoint; map validation succeeds. Classification unchanged:
19055/131088 bytes (14.54%), code 13531, data 5524, unknown 112033. Execution coverage
is unchanged from the disassembly handoff and remains separate from readability.

## Redirect questions

1. Continue $E27E/$E341 asset roots through counted CHR/screen decoder and map
   consumers. Establish bounded stream/layout/metatile slices; unclassified
   reference symbols are consumer names, not backing classifications.
2. Concrete connected record lead: [bounded observation](../round-01/commentary-spawn-record-observation.json).
   $BCB1 reads seven-byte records at $BB8B (stage, world X/Y, task pointer), starting
   cursor zero, adding seven, continuing while cursor<$0120. This gives 42 records,
   final start $011F, exclusive endpoint $BCB1. Physical $1BB9B..$1BCC1 is exactly
   adjacent to the classified loader, with stage bytes all 0..7. Inspect competing
   code evidence and handler contracts in context [12,13,14,15], then propose the
   data interval and connected handlers together. No annotation was made here.
3. Continue scheduler/animation indirect domains with producer contracts. Distinguish
   structural support, artificial execution and natural selection. Preserve inline
   barriers and effective physical bank identities.

Unknown enemy/puzzle identities, score units, effect-field reuse and backing table
extents are localized in uncertainties.md. Uniform generic-data purposes remain
unknown without blocking readability or undoing accepted classification.

No toolkit issues found; [persistent report](../toolkit-issues.md): zero bugs,
missing-feature findings or blockers this phase. Toolkit/input unchanged; no
publication, archive, deletion, cleanup or commit. Disk ~3.4 GiB free. No live process
or unfinished commentary frontier needs resumption.
