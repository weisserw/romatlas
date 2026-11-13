# pass-001 disassembly handoff

Exactly one disassembly pass finished; ROM incomplete.

ROM classification: 49.56% (194874 / 393232 bytes classified; 198358 bytes unknown).
Exact unrounded percentage: 49.55700451641779%.

Checkpoint: [maps/pass-001-extended.json](../../maps/pass-001-extended.json),
[verification](../pass-001-extended-verification.json); internal [state.json](../state.json),
current portable map ../../maps/current.json; source ../../generated/pass-001-extended/game.asm.
Baseline established once and independently byte-identical. Input hash unchanged.

513 regions: code 3,358 bytes, Text 14,162, Image 131,072,
Data 46,282. Every region has Static provenance/evidence; observed coverage zero.
[Per-bank counts](../disassembly/pass-001/counts.json),
[exact unknown complement](../disassembly/pass-001/unknown-ranges.json),
[progress](../progress.md), [investigation](../investigation.md).

Accepted: reset/NMI/IRQ rooted bank31 flow and ordinary fixed-bank callees,
header/vectors, bounded stack-guard sprite arrays, sixteen CHR tile banks, 39
uniform generic-data runs, and 406 intelligible English text slices.
Header-defined CHR + rendered samples supports graphics classification. Uniform
regions have exact pattern bounds and checked neighbors; no fill purpose claimed.
Text slices do not claim full adjacent escape-format objects.

Validate/save/export all passed; portable validate + generated ca65/ld65 rebuild
byte-identical. All reconstruction files remain local. Maps are ROM-free metadata.
Current-source commentary is next; no full readability or completion claim.

Runtime: emulator.start explicitly rejects mapper5, documented scope. No gameplay,
state edits or accepted fetches. [Scenarios](../scenarios.json) records unavailable
menu/later/battle/ending states. [Toolkit issues](../toolkit-issues.md): NES-M5-001,
1 missing feature, 0 confirmed bugs, 0 suspected bugs. Runtime blocked; static
classification unblocked. No toolkit changes.

Redirect frontier [L002–L004](../leads.json):

- L001 verified within this pass: main VM handler/stack/arithmetic family,
  split 256-byte low/high arrays and inline frame word accepted (+2,519 bytes).
  $EBAC alone stays unknown; natural opcode selection remains unobserved.
- L004 ready: extended selector table at $EE8C aliases JMP operand bytes in its
  first zero entry. Code dispatcher ends $EE8E; establish remaining table extent
  and native helper contracts ($F1F8/$F22E/$F257/$F2A4) before more propagation.
- L002: audio caller literal $FC/$FD writes via X=1/2 helper before JSR $8000.
  Establish MMC5 PRG mode, effective physical banks and bank-return contract.
- L003: native bank-entry stubs wrap VM bytecode with $E42A frame metadata.
  Recover bounded script/data slices without treating bytecode as native 6502.

These are productive avenues, not rejected leads or terminal exhaustion. Ready
leads did not preexist this pass. Detailed deferred scope/reasons and replay
recipes are in leads.json/investigation.md. Preserve all artifacts; scripts and
rpc.jsonl are local evidence, not portable sharing deliverables.

Final checkpoint is pass-001-extended. Initial pass-001 and duplicate historical
pre-extension pass-001-final remain preserved. Final internal/portable validation
and byte-identical rebuild passed; no input modification. No inherited leads;
L001 produced/verified here, L002/L003/L004 produced ready for redirect.
