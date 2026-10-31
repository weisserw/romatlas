# Round 4 commentary gate — complete

Current-source readability is complete on verified pass-004. Fresh supported verification passed with `ok=true` and `byte_identical=true`; the reading copy was refreshed after the last source edit. Final editorial/archive acceptance remains pending and was not run.

All **12160** full-ROM scaffold entries and **4266** discovered unit records are reviewed. The source contains **1938 Code fragments / 33204 instruction starts**. Code, instruction-byte and reviewed-unit unions each cover **71545 bytes**, with aligned boundaries and no missing/extra bytes. Source-local kinds match the scaffold; **3086** local path references and **16426** inventory source references were checked. All **6231** symbols are unique. All **152** actual uniform-run claims match immutable ROM bytes. Exact records: [gate.json](gate.json), [source consistency](current-source-consistency.json), [final consistency](final-consistency.json), [verification](gate-verification.json).

The entire accepted pass-003 commentary is preserved in `prior-pass003-commentary/` and `pre-prepare-original/`. All **1822 earlier Code fragments / 59266 bytes** carried by exact physical ranges; same-kind source/review credit was reused with dependencies corrected. New scope is **116 native fragments / 12279 Code bytes**, **656 Data fragments / 16276 bytes** and **393 Text fragments / 3519 bytes**. New routine units, 309 descriptive internal branches, guarded RAM/field/constants, bank-qualified ROM reads and 23 exact-view direct-call aliases explain the added source. Source locations and contexts were refreshed across the full inventory.

Mechanical corrections include the bank-56 predicate's unchanged selected index, five identical tests and carry-clear result under explicit preconditions; six bank-3 map-2 windows and bank-53 restoration; fixed wrapper post-mapper views; synthetic DEB4 and saved-A7 nonlocal returns; B1C0 retained as Code despite indexed reads; four-by-256 PPU copies from six source origins; actual X&1 geometry biases; serializer descriptors/XOR/alphabet/patches; command $25 = decimal 37 VM packets; narrow seven-byte ID list; and native RTS at physical 2139. Eighteen stale dependency paragraphs and targeted English defects were repaired. No ROM bug, natural reachability or full compatibility claim follows.

Examples (current source):
- **mode5_map2**: `commentary/source/game.asm:12400` — `B03RunBankTwoDescriptorSaleAction`.
- **restore53**: `commentary/source/game.asm:14514` — `B03RestoreModeFiveDataBank`.
- **serializer**: `commentary/source/game.asm:121944` — `B60BuildSerializedRecordDisplay`.
- **code_as_data**: `commentary/source/game.asm:7117` — `B01CollectActorsWithinBudget`.
- **predicate_x**: `commentary/source/game.asm:108081` — `B56RepeatSelectedRecordPredicate`.
- **ppu_copy**: `commentary/source/game.asm:110135` — `B57UploadStartupNametable_At816F`.
- **synthetic_return**: `commentary/source/game.asm:128695` — `B62RecordPanelCallbackResumeDEB4`.
- **nonlocal_return**: `commentary/source/game.asm:117375` — `B59ReturnNonlocallyToOuterModeNine`.
- **unknown_trap**: `commentary/source/game.asm:104609` — physical 458977, Unknown.
- **startup_data**: `commentary/source/game.asm:110120` — physical 467273, Data.
- **mask_byte**: `commentary/source/game.asm:7103` — physical 12751, Data.
- **coordinate_bias**: `commentary/source/game.asm:92545` — physical 431169, Data.
- **counted_vm**: `commentary/source/game.asm:56401` — physical 262241, Data.

Unresolved meanings remain explicitly bounded: generic numeric/matrix/glyph schemas, full natural selector domains and eight instruction-shaped Unknown residues. All **23 Unknown bytes** have local primary alternatives and limits, including BRK return-PC caution; no new concrete redirect lead emerged. Residual rejection review belongs to the next redirect/coordinator work, not this readability gate. See [residue notes](residue-notes.json), [noncode review](noncode-review.json) and `commentary/uncertainties.md`.

Classification remains **655353 / 655376 = 99.99649056%**, incomplete despite rounded 100.00%. Counts: Code **71545**, Data **356709**, Text **96027**, Image **131072**, Unknown **23**. Map/state/pass-004 SHA256 remains `bfc7f09f81cec0f452e0eb7d631e9b484dedab66d37a5a21bec5b96251ac0106`.

Protected output identities:

- Source game.asm: `ca6487fe2c440425c297b6cafcc65df152b6a0f3166853dc9bbc27192a1b6bde`
- Review: `f044775cdadccecd0a4032e612045d962d0820483616bd4df9edda983a31760c`
- Reading: `166e692ec9bbdea989db4fa91e8209727b72f506b22ab460527fc6f31f345e62`

ROM, maps, state, configured binary and preserved prior source/reading identities match. No live process or server. No toolkit issues found: zero confirmed bugs, suspected bugs or missing features; no blocker. Migration duplicates/includes, a context helper's missing metadata markers and a source-location interior-offset assumption were local editorial/helper errors corrected before final checks. No toolkit edits, map changes, ROM edits, final audit, archive or publication.

Coordinator wording repair: the bank-56 mapping note now states only that Unknown bytes `$80D1..$80D4` are outside the Data range. The unsupported fatal-loop purpose was removed from source and both review summaries. Full pre-repair commentary and gate history are preserved in `pre-wording-repair-001/`; source locations, consistency checks, byte-identical verification and reading copy were refreshed. Inherited frozen phase-1 evidence remains historical and is not adopted as current interpretation.
