
## Round 01 disassembly handoff — incomplete
ROM classification: 57.74% (151364 / 262160 bytes classified; 110796 bytes unknown).

Code 15706; non-code 135658. Provenance bytes: {'Static': 140886, 'Observed': 10478}.
Header:16 data; CHR:131072 image,0 unknown. PRG per physical8KiB bank:

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| PRG 00 | 0 | 0 | 8192 |
| PRG 01 | 2090 | 0 | 6102 |
| PRG 02 | 546 | 0 | 7646 |
| PRG 03 | 0 | 517 | 7675 |
| PRG 04 | 0 | 0 | 8192 |
| PRG 05 | 0 | 102 | 8090 |
| PRG 06 | 2659 | 30 | 5503 |
| PRG 07 | 498 | 103 | 7591 |
| PRG 08 | 1015 | 0 | 7177 |
| PRG 09 | 641 | 712 | 6839 |
| PRG 10 | 2952 | 32 | 5208 |
| PRG 11 | 44 | 0 | 8148 |
| PRG 12 | 0 | 0 | 8192 |
| PRG 13 | 800 | 605 | 6787 |
| PRG 14 | 892 | 10 | 7290 |
| PRG 15 | 3569 | 2459 | 2164 |

Exact unknown complement: analysis/evidence/round01/unknown-final.json (273 gaps).
Every classified region has structural evidence; code execution views are explicit.
Execution coverage separately: 5091 distinct physical accepted ROM instruction
roots across saved scenarios (mixed natural/artificial); it is not byte classification.

Final validation/rebuild: analysis/pass-001-final-verification.json; maps/current.json
is byte-for-byte the checkpoint map maps/pass-001-final.json, SHA256
687e9f29ec9f58f16b94a00c952536b7f9464a679019f8d37fe13100509a097b.
Generated current classified assembly: generated/pass-001-final/game.asm.
Baseline and interim checkpoints retained. Input ROM SHA256 unchanged.

Inline correction: first pre-repair snapshot had13764 code bytes; first repair
removed inline fallthrough and rederived13753 code bytes from accepted roots.
Later handler expansion exposed3 more sites; second pre-repair snapshot had15356
code bytes. Final root rederivation14528 plus1178 supported static handler bytes
=15706 code bytes. These totals involve differing scenario/root sets; net
difference is not the number of rejected table bytes. Eleven E116 call sites
audited; their11 barriers total188 bytes (tables/partial supported slices).
No additional unchecked E116 sites remain in accepted code.

Gameplay checklist and recipes: analysis/scenarios.json. Natural startup and
stage1; artificial stage2 shooting,stage4 driving,stage6 city,stage9 mansion/boss
meter/combat and ending dialogue. No natural late-game or boss-defeat claim.
Credits continuation produced dialogue with zero gain; a second input trial was
interrupted after its bounded wait and is explicitly attempted, not visited.
Remaining similar-stage variants/practice/sound deferred to focused questions,
not a claim of gameplay exhaustion. No terminal exhaustion claimed; this is an
easy-gain handoff with concrete redirect leads.

Toolkit:1 missing feature NES-001, no confirmed/suspected bugs or blockers;
analysis/toolkit-issues.md. Current source needs full commentary; labels mostly
structural fallback names. No archive/publication/deletion/toolkit edits.

Next: commentary across full current source, then redirect R01-L03/L05/L06.
