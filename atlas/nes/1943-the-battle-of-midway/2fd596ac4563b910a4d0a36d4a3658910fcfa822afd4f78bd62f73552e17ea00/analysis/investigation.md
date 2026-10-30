# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 disassembly — completed incomplete, pass-001-final

ROM classification: 67.25% (88154 / 131088 bytes classified; 42934 bytes unknown).

Internal store: analysis/state.json; current ROM-free portable mapping: maps/current.json. Classification is 12434 code,22472 data,53248 image bytes; remaining42934 unknown. Header16 bytes included in full physical total. All provenance is observed actual instruction fetch or explicit Static inference; artificial scenarios remain artificial and do not establish natural reachability.

| PRG 8KiB bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 1099 | 1906 | 5187 |
| 9 | 0 | 8192 | 0 |
| 10 | 2241 | 0 | 5951 |
| 11 | 321 | 0 | 7871 |
| 12 | 828 | 0 | 7364 |
| 13 | 2337 | 0 | 5855 |
| 14 | 3139 | 64 | 4989 |
| 15 | 2469 | 6 | 5717 |


### Evidence and verification
- Validated vector entries and effective mapper2 contexts, then actual title/menu/natural stage1 combat fetches and supported conditional/direct-call paths. Inspected official-instruction decoding and mapped bank views before acceptance. Classified image banks using complete tile raster inspection; generic data after bounded consumer/reference and record-pattern inspection; no observed fetch conflicts with data in combined captures.
- Evidence: analysis/evidence/decoded-initial-candidates.json, decoded-with-direct-callees.json, new-runtime-decodes.json, decoded-combat-all.json, decoded-final-static.json, decoded-final-callees.json, contexts-all.json, uniform-runs.json, bank0.png..bank3.png. Complete request/result history: pass-001-rpc.jsonl. Precise remaining intervals and per-bank counts: classification-final.json.
- Baseline and intermediate pass-001 retained. Final checkpoint maps/pass-001-final.json, generated/pass-001-final/game.asm, analysis/pass-001-final-verification.json: ok=true,byte_identical=true, map SHA256 1235bbba785ad78213dfc49a39546d54d1f07fd228cdc14e4b403d4e75d91ece. Generated diagnostics empty. Input SHA256 unchanged2fd596ac4563b910a4d0a36d4a3658910fcfa822afd4f78bd62f73552e17ea00.

### Pass outcome and next phase
- Natural stage1 shooting/movement/special inputs exercised; title, attract, upgrade, carrier launch visited. Upgrade screen B allocates points, then YES confirmation. Later-stage$0715 edits and final$2F state execute with distinct streams/enemies but do not establish full representative later-level/boss completion. Wrong$9C shortcut rejected after reset evidence.
- Ending direct$FA40 entry produced stripes. Corrected full result setup$F541 with $0739=0,$0730=$30 produced WHAT A PITY! and resumed flight; no ending/credits visit established. Both bounded experiments preserved. No additional arbitrary RAM/register trials; redirect should determine result flag/initialization contract or use validated password inputs.
- This is a normal single-pass handoff, not terminal exhaustion. Straight startup/runtime fetches, conditional continuations, direct-call entry contracts, and obvious graphics/uniform/record data have been processed. Remaining promising work needs dispatcher/table bounds and bank/selector contracts, so hand it to redirect instead of speculative code/data promotion. See leads.json and handoff-round-001-disassembly.md. Commentary has not run; full current-source readability remains pending.
- Toolkit report analysis/toolkit-issues.md:2 confirmed documentation/API inconsistencies (NES-001 input schema,NES-002 discovery budget),0 suspected,0 missing features,0 blockers. Workarounds used; toolkit unchanged.
- RPC server closed cleanly. REPL no longer owns a live server. Resume with configured run.py serve, map.load and replay/import snapshots; final-live.snapshot captures last artificial state, not a recommended clean natural baseline. For gameplay branch prefer combat1.snapshot or stage1-entry.snapshot.
- Disk at handoff:12GiB free, scratch240KiB before removal of duplicate PNG previews, cited local evidence about82MiB. No ROM/assets published; no archive/deletion performed. Selected captures/snapshots are retained locally.

## Round001 redirect complete — ready for independent disassembly

Resumed durable round001-20261030 outcome without replaying completed experiments. Combined proposals:17234 unknown Code+25688 unknown Data=42922B; independent one-byte divider RTS Data→Code correction at72615. No annotations accepted here. Current pass-001-final remains88154/131088 classified,42934 unknown. Conditional residual12 singleton bytes has localized role ambiguity, not missing semantic names. Connected command/actor/helper/audio graphs and independent data containers are packaged together; no known promising actionable avenue deferred merely as independent. Final-phase encounters and victory screen were artificially visited; natural boss/ending completion unproved, unchanged waits diminished.

Fresh packaging-verification.json verifies exact197-range unknown complement, disjoint proposals, instruction non-overlap, physical bank identity and preservation hashes. Existing byte-identical commentary/checkpoint verification retained; no fresh runtime/build claimed. R001/R002/R004/R005 ready, R003 verified research; leads/scenarios updated. Handoff: analysis/handoff-round-001-redirect.md; primary contracts/outcome/residual-review in analysis/redirect/round001-20261030/. Next phase independently accepts/rejects annotations, verifies checkpoint, then full commentary and redirect. Not terminal exhaustion.

Old PTY70778 is unavailable; no live process claimed. Map/state/source/ROM/toolkit unchanged; no archive/publish/deletion/cleanup. Disk11GiB free,scratch156KiB. Toolkit report2 confirmed,0 suspected,0 missing,0 blockers; no new toolkit issues found.

## Round002 disassembly — complete pass, classification incomplete

ROM classification: 99.99% (131076 / 131088 bytes classified; 12 bytes unknown).

Single disassembly pass from pass-001-final accepted17,234 previously unknown Code bytes and25,688 previously unknown Data bytes, all Static, plus corrected exactly1 previously classified Data byte at72615 to Code. No proposal intervals rejected after independent review. Structural classification now29,669 Code+48,159 Data+53,248 Image=131,076. Data includes the16-byte iNES header; PRG Data alone is48,143. Execution coverage and natural reachability are separate. Exact unique-byte ledger: analysis/classification-round002-final.json.

| PRG 8KiB bank | Code | Data | Image | Classified | Unknown |
|---|---:|---:|---:|---:|---:|
| 0 | 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 4096 | 4096 | 8192 | 0 |
| 3 | 0 | 8192 | 0 | 8192 | 0 |
| 4 | 0 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 0 | 8192 | 8192 | 0 |
| 8 | 1885 | 6307 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 | 8192 | 0 |
| 10 | 2387 | 5805 | 0 | 8192 | 0 |
| 11 | 2185 | 6007 | 0 | 8192 | 0 |
| 12 | 3126 | 5065 | 0 | 8191 | 1 |
| 13 | 5594 | 2594 | 0 | 8188 | 4 |
| 14 | 7244 | 946 | 0 | 8190 | 2 |
| 15 | 7248 | 939 | 0 | 8187 | 5 |

All12 remaining intervals are single bytes: 100262 (bank6:$8796,$60), 107629 (bank6:$A45D,$60), 107656 (bank6:$A478,$60), 111752 (bank6:$B478,$60), 112351 (bank6:$B6CF,$60), 115687 (bank7:$C3D7,$FF), 121412 (bank7:$DA34,$60), 123792 (bank7:$E380,$60), 123895 (bank7:$E3E7,$60), 123937 (bank7:$E411,$60), 127724 (bank7:$F2DC,$60), 129107 (bank7:$F843,$60). Eleven return-opcode bytes and one $FF have materially ambiguous code/data roles despite surrounding supported routines/records; absence of entry or execution is not data proof. The localized residual-review remains applicable; no schema-only unknowns remain. Whole-ROM classification remains incomplete.

Evidence: analysis/disassembly/round002-20261030/review.md independently reviews dispatch consumption, stack/entry/mapper contracts and container boundaries; review-rpc.jsonl freshly decodes all13,847 connected instructions in238 bounded requests; decode-boundary-review.json verifies exact physical mapping, all34 inline word prefixes, no instruction overlap or code/data conflict; data-inspection.json inspects182 exact blocks and base consumers. Application split184 unknown data fragments and238 code fragments, preserving every prior region verbatim except id250's one-byte suffix correction. state-before.json preserves the old record including its superseded boundary premise. No statically inferred bytes marked Observed.

Fresh map.validate, map.save analysis/state.json, map.export maps/current.json, checkpoint maps/pass-002-final.json and generated/pass-002-final, analysis/pass-002-final-verification.json: ok=true,byte_identical=true. Map/state/checkpoint SHA25604757fd24b163af0a28eb86a6e072b7023fc4b9fb81642493b60ce7e9adc8f36. ROM SHA2562fd596ac4563b910a4d0a36d4a3658910fcfa822afd4f78bd62f73552e17ea00 unchanged. Previous map SHA2561235bbba785ad78213dfc49a39546d54d1f07fd228cdc14e4b403d4e75d91ece and commentary source SHA256516546b9a822d3be95a01fc28c173266326ebdde5607bc19f1928157fca6da5b preserved.

R001/R002/R004/R005 verified in this pass; R003 successful artificial final-phase encounters and MISSION24 CLEAR/TOKAKU victory already documented in redirect scenarios/snapshots/runtime-actions and reused without unchanged replay. Natural final boss/ending completion remains unproved. Prior failed stage/ending trials remain historical with localized scopes. Toolkit report analysis/toolkit-issues.md remains2 confirmed documentation/API inconsistencies,0 suspected,0 missing features,0 blockers; no new toolkit issues found and toolkit unchanged. RPC servers closed after bounded requests.

Next phase is full current-source commentary, migrating preserved commentary/source and frozen baseline into fresh generation. Newly classified code/data requires full readability coverage; old commentary success does not cover new source. Correct misleading result flag name and divider suffix physical comment. Redirect follows commentary and should review the exact12 singleton questions using evidence-policy.md before a terminal judgment. No archive/publish/deletion/cleanup or terminal exhaustion claim. Compact handoff: analysis/handoff-round-002-disassembly.md.

## Round002 redirect — terminal-incomplete assessment for coordinator review

Unique attempt analysis/redirect/round002-20261030/ starts at accepted pass-002-final and full current-source commentary gate (source SHAaf584d13044200e6a0139e6c23264e570c81980bfe13e07334a6d6f4b74d980a). Fresh audit.py reads current internal map, ROM, neighboring regions, primary supported graph and raw pointer candidates; exact complement remains12 singleton bytes/131088, classified131076 (99.99%). No proposals/accepted annotations or new ready leads. All prior accepted leads and histories retained. No classification, map, state, source, ROM or toolkit edit.

Localized rejection-review.md independently challenges header-named bank6:$8796, redundant RTS suffixes, adjacent return entries and isolated$FF. Distinguishes exact supported header/data/command bank contracts: five$8796 header words are outside actual$8040 command directory, indexed$8008 consumer fixes X=0, fixed$8000/$8003 audio calls select bank5, background header consumers select bank4. Repeated return-shaped suffixes and known neighboring return contracts do not independently discriminate intentional stubs from filler/scalars; lack of entry alone is neither data proof nor a universal caller requirement. Generic-data fallback assessed on each exact singleton and lacks affirmative non-code pattern/consumer. No surrounding classified records rejected. Primary artifacts, competing interpretations, actual negative scopes and specific insufficiency are linked in review and residual-ledger.json.

Scenarios reviewed against prior exact runtime actions: natural stage1 plus artificially initialized final waves, ship/terrain, large-aircraft encounters and MISSION24 CLEAR/TOKAKU/victory cover representative later systems/terminal screen. Natural final defeat remains unproved; unchanged camera$0618 waits/special sequence have diminished prospects. No replay or arbitrary sweep. Full current-source semantic questions provide no concrete new residual role/consumer. No interrupted productive investigation remains; stop assessment is subject to coordinator rejection review, not ROM completion. Final editorial/fidelity acceptance remains separate. Editorial worker should clarify ambiguous positive-result wording at commentary/source/game.asm:20466–20467 using accurate StageRepeatRequested explanation/evaluator at20677; source left unchanged.

Handoff analysis/handoff-round-002-redirect.md. Preservation hashes freshly match ROM, internal/current/checkpoint maps and source. Existing byte-identical checkpoint/commentary verifications retained; no fresh build/runtime claim. Disk11GiB free; scratch156KiB; no archive/publish/deletion/cleanup. Toolkit issues remain2 confirmed documentation/API inconsistencies,0 suspected,0 missing features,0 blockers; no new toolkit issues found.
