# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 disassembly
Verified initial fidelity baseline, preserved `maps/baseline.json` and generated baseline.
Supported vector roots plus sampled opcode contexts from intro/title/puzzle/password,
floors 5/10, final boss and expert room seeded bounded static discovery. Direct JSR
entries were recursively reseeded for at most twelve rounds (actual convergence in
saved `static-discovery-rounds.json`); no speculative call return fallthrough.
All 3,625 adopted starts inspected through bank-aware toolkit decode; official coherent
instructions only. Classified 7,306 code bytes, 32,768 CHR bytes, 16 header bytes,
6 vector bytes, 1,195 repeated non-code bytes, 55 bounded zero-tail bytes.
ROM classification: 63.07% (41346 / 65552 bytes classified; 24206 bytes unknown).
Per-bank exact code/data/unknown counts are `round-001/counts.json` and progress.md.
Fresh `round-001-disassembly` checkpoint validates/rebuilds byte-identically.

Broad normal progression: initial room movement and restart/life-loss; passwords HRKJ,
QTDV,VQTD,PROA accepted and movement/A/B tested in each. Screenshots and RAM compare
establish $78 stage indices 0/20/45/50/53, $79 floor 0/4/9/10/0. Ending shortcut restored
boss snapshot and completed one operation before PC=$9B3F; incoming stage50, floor10,
$B3=1 satisfy local increment/ending transition. Castle/rescue montage observed under
artificial state. No natural boss-defeat/terminal-credits claim.

New actionable questions L001/L002 in leads.json: determine inline-script $801A
contract and safe call continuations/PRG effects. No existing leads were supplied;
no unchanged failed lead retried. Repetitive early play stopped after distinct
milestones; later states used tested password routes. Ending montage sampled rather
than indefinitely waiting. Missing fact for more static closure is inline-data/callee
return contract, not additional blind gameplay. Ordinary handoff is useful; terminal
exhaustion is not asserted. Toolkit report: TK-001 missing feature, no blockers.

## Redirect round001 contracts — combined ready batch
Resumed and completed the existing `analysis/redirect/round001-contracts/` investigation without replaying completed experiments. Independent consumer, stack, finite-domain and boundary constraints support a combined 8,422 code /15,775 data-byte proposal; accepted classification remains unchanged. Continued ending callbacks, tutorial panels, initialized object dispatch, room/metatile helpers and all concrete adjacent audio/resource/sprite/table avenues. Four tiny gaps remain ambiguous (nine bytes); no bounded supported selector/entry currently resolves them. See `analysis/redirect/round001-contracts/handoff.md` for exact contracts, boundaries, method corrections and actual deferral reasons, and `proposal-audit.json` / `residual-inspection.json` for machine-readable evidence. L001/L002 history preserved and updated; ready L003..L005 added. This ready-batch handoff is not terminal exhaustion or ROM completion.

## Round 002 disassembly complete — incomplete ROM
ROM classification: 100.00% (65551 / 65552 bytes classified; 1 bytes unknown).
Rounded100.00% is not completion: physical $3C70 remains Unknown.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG0 | 3770 | 4422 | 0 |
| PRG1 | 5437 | 2754 | 1 |
| PRG2 | 6328 | 1864 | 0 |
| PRG3 | 193 | 7999 | 0 |
| CHR0 | 0 | 8192 | 0 |
| CHR1 | 0 | 8192 | 0 |
| CHR2 | 0 | 8192 | 0 |
| CHR3 | 0 | 8192 | 0 |

Accepted8422 new code+15783 new data bytes (24205 unique gain). Existing classifications and commentary preserved exactly. Corrected three invalid negative palette claims (timed8 argument order); independently resolved eight proposed residual bytes through indexed references. Primary review: `analysis/round-002/independent-review.md`; mechanical checks/fresh decode and exact counts/hashes alongside it. L001..L005 verified, L006 records remaining isolated RTS ambiguity and reopening condition. No new execution/natural-reachability claims. Prior broad gameplay checklist and evidence reviewed without unchanged replay.

Map validated and fresh final checkpoint `maps/round-002-disassembly-reviewed.json` generated under `generated/round-002-disassembly-reviewed`; `analysis/round-002-disassembly-reviewed-verification.json`: ok=true,byte_identical=true. State/current/checkpoint SHA256 fe00480c889d06946c5bc8401fdc293cc27b68246470b265b89d49d2c783a0b5. 5347 new annotation regions; old346 regions preserved exactly. Earlier `round-002-disassembly` is an unchanged-input verification accidentally run after a safely rejected annotation invocation; preserved as history, not final checkpoint. Annotation creation required id0; failed invocation logs retained, no toolkit defect.

Full ensuing commentary must review all current source while preserving prior accepted explanations. Current code15728 bytes. No final editorial audit, archive, cleanup/deletion, toolkit edit or publication. Existing TK-001 missing feature, zero new issues or blockers; see `analysis/toolkit-issues.md`. Disk9.5GiB free; attempt9.5MiB before final documentation; no live process. Handoff `analysis/round-002-disassembly-handoff.md`.
