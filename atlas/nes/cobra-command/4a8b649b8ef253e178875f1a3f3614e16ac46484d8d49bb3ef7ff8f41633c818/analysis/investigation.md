# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 active structural findings
No input leads consumed. Initial static vector traversal stops at conservative mapper-write/call barriers; observed roots supplement it. Natural-static-initial-candidates.json retains first scan. Four false fallthroughs into adjacent tables arose from untracked constant flags/nonzero selector tables; data-batch.json records exact barriers and independent bounds. Revised natural-static-candidates.json has9561 candidates and no unofficial/BRK artifacts. Direct JSR target expansion uses caller's pre-call bank context (unchanged on entry), not assumed post-return banks; bounded four-depth expansion reaches510+13+5 direct call contexts. It exposes two further table fallthroughs after CLC/BCC and LDA#$86/BNE: physical$1e078..1e08e (pointer/constants, adjacent next routine$1e08f) and$1efea..1f031 (18 visibly repeated4-byte records, consumer$efda indexed, next routine$1f032). These are data barriers, not executable KIL routines. Preserve prior candidate artifacts; no natural reachability claimed for static extra paths.

Gameplay: intro,title,equipment,stage1 flight/shooting and Start reversal visited naturally. Select opens equipment. Patching$cb/$88=2 plus Select alone remained equipment$4e0a (area2-* artifacts), so don't retry unchanged. Revised method reproduces documented bank5 exit RAM assignments and clears$05a2 flag; fixed main menu exit then reads$cb and calls full scene loader. Exact script scene_experiment.py, clean equipment-menu-snapshot.json. Artificial observations separate.


## Round 1 final findings / frontier

ROM classification: 89.32% (234161 / 262160 bytes classified; 27999 bytes unknown).

Single authorized disassembly pass completed. Code 21382 bytes; non-code 212779 bytes; 1011 evidence-annotated regions (1011/1011 nonempty evidence). Exact physical unknown complement and all bank totals: `round01/final-metrics.json`. Header16/16 classified. CHR banks0–15 each8192/8192 classified Image,0 unknown.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 8 | 8184 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 8 | 8184 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 8 | 8184 | 8192 | 0 |
| 6 | 5484 | 0 | 5484 | 2708 |
| 7 | 1780 | 4576 | 6356 | 1836 |
| 8 | 379 | 3572 | 3951 | 4241 |
| 9 | 462 | 6437 | 6899 | 1293 |
| 10 | 1064 | 4470 | 5534 | 2658 |
| 11 | 42 | 395 | 437 | 7755 |
| 12 | 1864 | 5681 | 7545 | 647 |
| 13 | 8 | 6774 | 6782 | 1410 |
| 14 | 6107 | 28 | 6135 | 2057 |
| 15 | 4168 | 630 | 4798 | 3394 |

Fresh checkpoint **pass-001**: map.validate passed; `maps/current.json` and `maps/pass-001.json`; generated `generated/pass-001`; `analysis/pass-001-verification.json` confirms ok=true,byte_identical=true. Map SHA256 `30a0e9f8324b21c1d9582312c4fde1da220b1d0f3600ed103cc5bb694765d69a`. Input ROM SHA256 unchanged `4a8b649b8ef253e178875f1a3f3614e16ac46484d8d49bb3ef7ff8f41633c818`. No toolkit edits/publishing/archive/cleanup.

Execution is separate:5928 unique physical fetch starts;11575 naturally observed opcode bytes;9975 artificial-scenario observed opcode bytes;12675 union (overlap counted once). Trace aggregate drops0 in saved scenario summaries; detail capacity0 intentionally discarded bulk events. Disabled-trace ending progression supplies frame/state evidence, no fabricated fetch coverage.

Natural: intro,title,equipment,stage1 flight/missiles/guns,Start reversal,Select menu. Artificial: later surface scenes2/4/6/8,final underground9,complete ending message and victory/STAFF/GRAPHICS credits. Boss-condition patch attempted without confirmed boss; rescue-specific/spatial contracts remain for redirect. All final outcomes and exact replay/patch budgets consolidated in `analysis/scenarios.json`; source hypotheses in `analysis/gameplay.md`.

Consumed leads:none (initial queue empty). Produced research leads R1-L001..R1-L004 in `analysis/leads.json`. Broad later-surface tests yielded14,0,17,0 new fetched code bytes; cave9 added7 and boss-condition6. Direct static paths, direct-call target closure and bounded stack/bank-preserving continuations were classified; remaining promising continuations need indirect-selector/return-stack/bank-restoration or mixed-block boundary research. No raw unknown gap was treated as classified. This is the easy-gain phase handoff to commentary/redirect, not terminal workflow exhaustion.

Five large non-code blocks (24006 bytes) accepted after precise neighboring boundaries, references/pattern inspection and checking every saved fetched instruction for conflicts; `round01/large-data-review.json`. These generic structural roles do not claim complete original schemas. Classification complete only for accepted ranges. Readability/editorial audit remains for commentary across all current classified source.

**no toolkit issues found**; persistent report: [toolkit-issues.md](toolkit-issues.md). Latest verification valid; no live server required. Resume with configured binary via run.py serve,map.load analysis/state.json,emulator.start,import chosen persisted snapshot,then explicitly reassert buttons and configure/clear traces. All standalone round01 scripts are local reproducible evidence, not portable sharing artifacts. Scratch12KiB; analysis/round01~151MiB;3.6GiB free at handoff.

### Final toolkit report correction
Concrete repeated static-discovery table fallthroughs documented as missing feature NES-MF-001 (constant Z/C propagation), not an advertised-behavior bug. One missing feature,zero confirmed/suspected bugs,no blockers; [toolkit-issues.md](toolkit-issues.md). This supersedes preliminary no-issues wording. Workaround validated Data barriers; no toolkit edits. Final immutable map/checkpoint/input identity and count audit passed; classification line and checkpoint remain unchanged.

## Round01 redirect r01-static-contracts — ready combined batch
Input/current checkpoint pass-001 unchanged(234161classified/27999unknown). Connected main75/auxiliary80 actor graphs, synthetic RTS/state contracts, sound51, projectile5/16, menu action/subaction/text/PPU, fixed scene/camera/spawn/data and IRQ entry investigated. Corrected overbroad stack/bank/table rejection premises using independent field/control/physical constraints; exact findings and primary artifacts in redirect/r01-static-contracts/findings.md. Artificial rescue verified; default high-health placement failed and justified position revision succeeded, boss identity remains qualified. All attempt history preserved.

Combined proposal27992 bytes(15691Code+12301Data),accepted0; descriptor4 and semantic48byte corrections are zero gain. Detailed exact complement/proposal/consistency/residual review retained in the attempt directory. Seven ambiguous isolated opcode/zero bytes remain outside proposals; no unchanged failed replay or speculative classification. Ready new R1-R001..R004 and strengthened historical leads recorded. Continue with independent disassembly review; no terminal ROM completion/exhaustion claim. Compact handoff handoffs/round01-redirect.md.

## Round02 disassembly pass complete — incomplete classification

ROM classification: 100.00% (262153 / 262160 bytes classified; 7 bytes unknown).

Accepted27992 previously unknown bytes:15691 Code/12301 Data. Corrected4 existing Code bytes to sprite-zero descriptor Data and48-byte scene sound-command evidence. Totals37069 Code/225084 non-code;1438 regions, all evidence annotated. Independent primary contract/bank/stack/branch/data review: round02/review.md; every proposed physical opcode decoded again with configured binary, exact candidate instruction boundaries/context match. Data spans' immutable bytes, nearby roles/references and fetch-conflict check: round02/independent-data-review.json. Per-batch data acceptance saved before code acceptance, final map validation true.

Input pass-001 map30a0e9f8324b21c1d9582312c4fde1da220b1d0f3600ed103cc5bb694765d69a. Output pass-002 map328a244dca1aece5e30489a075a5f36aca7fb35ae277de0b933ce97a0352e760; analysis/pass-002-verification.json confirms byte_identical=true and unchanged ROM SHA2564a8b649b8ef253e178875f1a3f3614e16ac46484d8d49bb3ef7ff8f41633c818. State saved, maps/current.json exported, checkpoint map/source retained. Existing maintained commentary/source unchanged; new generated/pass-002 is checkpoint source, not reconciled commentary.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 8 | 8184 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 8 | 8184 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 8 | 8184 | 8192 | 0 |
| 6 | 7924 | 268 | 8192 | 0 |
| 7 | 2974 | 5218 | 8192 | 0 |
| 8 | 4194 | 3996 | 8190 | 2 |
| 9 | 462 | 7730 | 8192 | 0 |
| 10 | 3438 | 4751 | 8189 | 3 |
| 11 | 2755 | 5437 | 8192 | 0 |
| 12 | 2200 | 5991 | 8191 | 1 |
| 13 | 8 | 8184 | 8192 | 0 |
| 14 | 6973 | 1219 | 8192 | 0 |
| 15 | 6117 | 2074 | 8191 | 1 |

Header16/16 classified,0unknown. Every CHR bank0..15 is8192/8192 Data/Image,0unknown. Exact metrics/complement: round02/final-metrics.json. Seven bytes/six residual intervals108fb+1,10e2c+1,14422+2,14558+1,184c3+1,1fd8e+1 remain materially ambiguous isolated RTS/zero; reviewed actual evidence/alternative roles rather than requiring natural execution or semantic names. Rounded100.00 does not mean complete.

Consumed ready leads R1-L001/L002/L003,C001/C002/C004,R001/R002/R003/R004 verified; L004/C003 previously verified experiments retained. Connected batch includes downstream ordinary paths and independently bounded records, not only original lead intervals. No rejected proposed bytes; residual7 deliberately unproposed/unknown.

No new emulator scenarios executed: reused broad stages/menus/mechanics/ending/credits and redirect rescue/high-health artificial evidence; scenarios.json/gameplay.md retained. Unchanged startup or failed boss replay supplies no substantial new evidence. Natural boss identity remains qualified. This is one easy-gain pass, not a terminal all-methods exhaustion decision. Next commentary must freeze/reconcile prior maintained source against pass-002, then redirect may evaluate remaining isolated-byte ambiguity.

Toolkit: no new toolkit issues found; existing NES-MF-001 is one missing feature,zero confirmed/suspected bugs,no blockers. See toolkit-issues.md. No toolkit edits,archive,cleanup,publishing or ROM mutation.

## Explicit user-directed residual fallback

See manual-fallback/conclusions.md for exact six ranges, preserved preflight complement, prior code/data ambiguity and authorization. Seven bytes added as generic Data/Manual without structural proof or further redirect. ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown). Distinguish262153 structurally supported bytes from7fallback bytes; meaning and true structural role remain unresolved for7. Checkpoint pass-003-manual-fallback byte-identical; mapb94f534aac83b7f8eb2cf735e14380fa341fb38f929c4a68fed99cbd85527dca. Historical rejection review remains sound and unchanged; the user changed the acceptance rule for this bounded residue. Commentary reconciliation pending; handoffs/manual-fallback-disassembly.md.
