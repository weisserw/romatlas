# Terminal handoff — workflow finished, ROM classification incomplete

ROM classification: 100.00% (262153 / 262160 bytes classified; 7 bytes unknown).

The percentage is rounded. Whole-ROM classification is **incomplete**. The coordinator workflow reached its justified terminal incomplete handoff after two sequential disassembly/commentary/redirect rounds, independent proposal acceptance, residual rejection review, coordinator examination of primary evidence, and final editorial acceptance. No productive phase or experiment is interrupted. This is not a blocked handoff or a claim of complete hardware/game compatibility.

## Current artifacts

- Classification checkpoint **pass-002**: [map](../maps/current.json), [internal state](state.json), [checkpoint verification](pass-002-verification.json), [exact per-bank metrics](round02/final-metrics.json). Map SHA256 `328a244dca1aece5e30489a075a5f36aca7fb35ae277de0b933ce97a0352e760`.
- Readable maintained [assembly](../commentary/source/game.asm) and refreshed [reading copy](../commentary/reading-after.asm). Current source SHA256 `733b383ddb54e191e7df18048de76361e70ddac3bcae31e01204376d2e41c1ee`.
- [Final editorial audit](../commentary/editorial-audit.md), [supported audit result](../commentary/audit.json), and [verification](../commentary/verification.json): byte-identical, editorial audit complete, 1444 reviewed regions and 1853 units. No current classified readability backlog.
- [Coordinator rejection assessment](coordinator-rejection-review.md), [final redirect handoff](handoffs/round02-redirect.md), [compact rejection review](redirect/r02-residual-constraints/rejection-review.md), and [exact residual ledger](redirect/r02-residual-constraints/ledger.json).
- [Lead history](leads.json), [phase and checkpoint history](progress.md), [investigation record](investigation.md), [gameplay sources](gameplay.md), and [scenario recipes/outcomes](scenarios.json).
- [Document selection](../archive-options.json) explicitly preserves additional final results, both rounds' handoffs and historical commentary conclusions. Only three duplicate historical phase-instruction documents are omitted. [Final completion checks](completion-audit.json) record document-contract accounting and current identities. No archive was executed.

The original input SHA256 remains `4a8b649b8ef253e178875f1a3f3614e16ac46484d8d49bb3ef7ff8f41633c818`. Final assembly reconstructs all bytes exactly, including unknown inclusions; fidelity does not classify those bytes.

## Exact residual and stopping rationale

Unknown physical intervals: `$108fb+1`, `$10e2c+1`, `$14422+2`, `$14558+1` (five isolated return-opcode bytes), `$184c3+1` and `$1fd8e+1` (two isolated zero bytes). Code-versus-data roles remain materially ambiguous. Neighboring return/tail-jump barriers, bank-aware dispatch domains, literal and synthetic-return encodings, generic-data/table/suffix alternatives and positive IRQ/metatile controls were evaluated. Neither hypothetical successful execution nor mere adjacency supplies the missing structural discrimination. The independently supported neighboring intervals remain classified.

No ready lead or justified discriminating next test survived the final review. New independent evidence or a concrete corrected inference can reopen the same residual frontier; do not replay unchanged startup, boss-flag or arbitrary-PC experiments. Load the configured binary through `run.py serve` and the saved state only when a concrete new task warrants it. Exact context and recipes remain in the ledger and scenario records. Source and maps are local; the internal state is not a portable sharing map.

Representative gameplay includes natural stage-one flight/firing and menus; artificial later surface areas, final cave, ending and credits; successful initialized/aligned rescue; and a corrected high-health actor placement/firing test. Natural boss identity/encounter and several exact asset/song/game names remain uncertain. Those limits are separate from the seven structural unknown bytes and do not leave a productive classification experiment pending.

Toolkit status: one existing missing feature, NES-MF-001 (constant-flag branch pruning); zero confirmed/suspected bugs and no blockers. [Issue record](toolkit-issues.md). No toolkit modification, archive, cleanup, workspace deletion or publication occurred. Prior commentary is preserved under `analysis/round-02/commentary-pass-001`; all checkpoints and investigation history remain local.
