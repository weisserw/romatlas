# Round 4 disassembly complete — pass-013

ROM classification: 99.98% (131059 / 131088 bytes classified; 29 bytes unknown).

Accepted7784newbytes:3781code+4003data, all Static. Exact current kinds: {'Code': 48847, 'Data': 81412, 'Text': 800}. R1-L04/L05 verified; all prior classified intervals preserved. Independent configured-binary opcode/length/text/mapping audit covers1776newinstructionviews. Data literal bytes and complete PPU grammar counts checked independently; story167records plusFE terminator. Evidence: review.md, independent-audit.json, new-code.txt, annotation-results.json and immutable analysis/redirect/r3-20261105 primary contracts/proofs.

Conditional stack/bank predicates retained: reset C9D7 stage-tail loader distinct from scene C9C2; explicit callback restoration separate from return; pause and projectile hit paths intentionally discard return frames; unknown map-bank slots remain null. No observed or natural-reachability upgrades. Existing broad artificial later-stage/boss/reward/quest-over evidence reused from scenarios; no unchanged runtime/native replay or baseline initialization.

Fresh map.validate passes; maps/current.json exported; analysis/state.json saved. maps/pass-013.json and generated/pass-013/game.asm verify ok=true,byte_identical=true in analysis/pass-013-verification.json. Map SHA25636f1a1c42910c33c11752baca0153176d41de82419e682d1647305681fde93c0. ROM/configured binary and ENTIRE prior commentary unchanged (protected-identity-check.json). Current commentary remains pass-012 and requires full preservation/carryover before next full-current-source phase. No commentary work performed here.

| Physical PRG8K bank | Code | Data/text | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| 0 | 334 | 7858 | 8192 | 0 |
| 1 | 775 | 7417 | 8192 | 0 |
| 2 | 3031 | 5161 | 8192 | 0 |
| 3 | 1065 | 7126 | 8191 | 1 |
| 4 | 424 | 7768 | 8192 | 0 |
| 5 | 5600 | 2589 | 8189 | 3 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 372 | 7820 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 3858 | 4334 | 8192 | 0 |
| 10 | 6408 | 1783 | 8191 | 1 |
| 11 | 6146 | 2046 | 8192 | 0 |
| 12 | 3335 | 4857 | 8192 | 0 |
| 13 | 2559 | 5633 | 8192 | 0 |
| 14 | 6938 | 1254 | 8192 | 0 |
| 15 | 8002 | 166 | 8168 | 24 |

Easy-gain gate: reviewed all connected ready roots/families and explicit return barriers; independent connected-closure.json combines accepted and proposed instruction references and finds zero direct control targets into residual29bytes, zero graph stops/fallthroughs there, pendingqueue0. No additional concrete supported continuation emerged. The four residual physical objects remain723D(1),A6C4(3),15F7B(1),1FFC7..1FFDF(24). Independently reviewed primary boundaries and alternate alignment/overlapping FDE6 call; no new discriminating code/data contract. Their uncertainty is localized, not inherited global rejection. See review.md and r3 deferred-review.md. Keep unknown pending a materially different contract/representation inference, not more legal decoding or speculative padding.

ROM incomplete; this is ordinary single-pass easy-gain completion, not terminal research exhaustion. Next separate full-current-source commentary must preserve the entire existing version first, then review all current classified source; final editorial acceptance and any terminal redirect review remain pending. No new toolkit issues found; existing nonblocking NES-001/NES-002 remain in analysis/toolkit-issues.md. No toolkit edits, cleanup/deletion, archive, publication or subagents. Handoff disk8.4GiB free, scratch17MiB; historical attempts preserved.
