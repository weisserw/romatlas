# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round001 disassembly completed; incomplete ROM

## Round 001 — disassembly phase completed; ROM incomplete

ROM classification: 100.00% (65551 / 65552 bytes classified; 1 bytes unknown).
Exact percentage: 99.99847449353186%. Rounded 100.00% does not establish completion.
Single unknown: physical$39C7, CPU$B9B7, bank1, one byte; L005 ready for redirect.

- L001 verified: Vectors and startup control closure. Reset/NMI/IRQ coherent official6502 graph in fixed CNROM contexts. Easy startup gains accepted.
- L002 verified: Broad progression through14-state initializer. Natural stage1; artificial desert/swamp/stage4 variants and4 distinct bosses; defeated final boss flag loops to selector 0 withC0=1.
- L003 verified: Impossible branch fallthrough data barriers. Physical1336:1392 object arrays and2AFE:2B06 split boss-render dispatch are data; earlier code candidates rejected only inside these intervals. LDY#2 at2AFA fixesZ=0 beforeBNE.
- L004 verified: Connected object/audio dispatch closures. Validated supported static handler contracts and downstream structure; accepted1830 additional code bytes before remaining table classification. Pointer values alone not used as code proof.
- L005 ready: Isolated RTS at physical39C7 / CPU B9B7. Redundant audio wrapper epilogue or data byte; unresolved code/data decision.

Counts/evidence: round-001/counts.json, connected-contracts.json, coverage.json,
static-listing-final.txt, scenarios.json and checkpoint verification above.
| header | 0 | 16 | 0 |
| PRG0 | 6867 | 1325 | 0 |
| PRG1 | 4970 | 3221 | 1 |
| PRG2 | 0 | 8192 | 0 |
| PRG3 | 0 | 8192 | 0 |
| CHR0 | 0 | 8192 | 0 |
| CHR1 | 0 | 8192 | 0 |
| CHR2 | 0 | 8192 | 0 |
| CHR3 | 0 | 8192 | 0 |

Generic data review checked consumers and repeated records; semantic reconstruction
is partial and is not an additional structural gate. Unknown one-byte RTS remains
localized; neither absence of fetch nor legal decoding alone settles it.
No failed lead was retried unchanged. Final-hit experiment failed; defeated flag
with independent main-loop contract succeeded. No toolkit edits or ROM changes.
