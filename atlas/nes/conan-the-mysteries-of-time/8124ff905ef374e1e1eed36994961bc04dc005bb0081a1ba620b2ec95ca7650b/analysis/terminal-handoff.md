# Terminal incomplete orchestration handoff

The coordinator workflow has reached its justified terminal incomplete handoff.
ROM classification is NOT complete, and this is not blocked work or a claim of
hardware/game compatibility. No ready classification lead or useful revised method
survived the required rejection review on the present evidence.

ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).

Code: 26,849 bytes. Data: 104,238 bytes, including the 16-byte header. The rounded
percentage does not eliminate the unknown byte. Exact residual: physical
[$1CAF1,$1CAF2), CPU $CAE1 in permanent physical 8 KiB PRG bank 14, byte $60.
Per-bank counts and structural evidence are in the
[last disassembly handoff](handoffs/round-02-disassembly.md).

## Coordinator rejection review

I inspected the [worker rejection review](redirect/r02-cae1-review-20261127/rejection-review.md),
its [primary audit and local ROM bytes](redirect/r02-cae1-review-20261127/primary-audit.json),
the audit script, NMI code, scheduler and callback installers in the current source.
The accepted-instruction inventory covers all 26,849 code bytes. No direct decoded
reference or literal little-endian $CAE1 word was found. This excludes those forms
only, not computed pointers, altered returns or unobserved execution.

The stronger inference was checked independently: the $CADE JMP $D4F9 does not
push $CAE0. The NMI's earlier JSR through $CABA remains the return obligation;
the scheduler's tail callback can return to $CAB2 without visiting $CAE1. IRQ starts
at $CAE2. Thus no neighboring stack/control contract needs this byte. A redundant
RTS/no-op entry and a spare data byte both fit the observations. A single valid
opcode supplies no discriminating routine contract, and this isolated byte has no
recognizable non-code format or uniform-run support for generic data. Requiring
structural discrimination is justified; requiring natural use universally would
not be. No counterexample was propagated beyond this single-byte claim.

No concrete corrected inference or discriminating experiment remains identified.
Fabricating an RTS stack merely demonstrates opcode behavior. Repeating unchanged
gameplay or sweeping arbitrary callbacks would not settle its structural role.
The review is sound on unchanged premises, so no empty cycle is scheduled.
Reopen redirect for an independently supported entry/producer, return contract,
format constraint or corrected inference that distinguishes this byte. Preserve
the saved frontier; observations are useful but not a universal prerequisite.

## Deliverables and resumption

- Final structural checkpoint: [map](../maps/round-02-disassembly.json),
  [state](state.json), [portable mapping](../maps/current.json),
  [byte-identical checkpoint verification](round-02-disassembly-verification.json).
- Current readable local source: [assembly](../commentary/source/game.asm),
  [symbols](../commentary/source/symbols.inc), [reading copy](../commentary/reading-after.asm).
  All current classified source is readable, with uncertainty localized;
  [editorial acceptance](../commentary/editorial-audit.md) and
  [fresh audit](../commentary/audit.json) are complete.
- [Final verification](final-verification.json), [archive-preserved classification audit](current-audit.json), [completion audit](final-completion-audit.md),
  [exact unknown ledger](redirect/r02-cae1-review-20261127/unknown-ledger.json),
  [lead history](leads.json), [final redirect handoff](handoffs/round-02-redirect.md).
- [Scenarios](scenarios.json) preserve title, cave mechanics, all stage indices
  0–7, inventory and ending routes/captures. Later visits use documented RAM edits
  through the game loader; they do not establish natural progression. Natural boss
  victories are explicitly deferred without a concrete connection to the residual.
- [Archive selection](../archive-options.json) and
  [document accounting](archive-document-selection.md) are prepared only.
  No archive, cleanup, deletion, publication or commit has been performed.

The input SHA256 remains
`8124ff905ef374e1e1eed36994961bc04dc005bb0081a1ba620b2ec95ca7650b`.
The final structural map/state SHA256 remains
`171e9c2d3ffa53f33df490ade67191a3b7d8342fca8f67167e635f4ad3b7948e`.
Prior commentary, phase handoffs, checkpoints and investigation outcomes are retained.
There are no live phase workers/servers requiring resumption. No toolkit issues
found; [persistent report](toolkit-issues.md) has no blockers. All ROM, restored
source, assets, traces and internal state stay local; only the platform-supported
ROM-free portable mapping is a sharing deliverable, and nothing is published here.
