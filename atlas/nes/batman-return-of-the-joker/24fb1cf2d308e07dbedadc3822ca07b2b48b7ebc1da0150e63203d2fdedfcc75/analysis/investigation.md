# Investigation ledger

Round 1 disassembly: initial map had zero regions, restored before annotation. Vector-connected static code accepted with independent fixed-last-bank evidence in local mapper implementation `/root/romatlas/vendor/jgenesis/backend/nes-core/src/bus/cartridge/mappers/sunsoft.rs:244`. No engine code copied. Explicit contexts retain unknown switchable slots.

- R01-VECTORS verified: physical $2000A–$2000F gives E1D0/E2E6/E2A9. Reviewed coherent reset and interrupt code; connected direct branches and fixed-bank returning JSR continuations classify 395 bytes. Reachability static only.
- R01-GEOMETRY verified: 16-byte header and 262144-byte CHR storage. Tile sample at physical 131088 begins blank tile then two-plane patterns; final tile FF. Geometry comes from header, not decoding plausibility.
- R01-UNIFORM verified: 19 maximal PRG runs, exact bounds in round-01/classification-evidence.json; repeated non-instruction structure, no overlap with reviewed code. Generic data only, no padding claim.
- R01-BANK14 verified partial: reset-selected effective banks support C117 and D394 prefixes (22 bytes), reviewed in bank14-reviewed.json. Further bank-flow/callee effects deferred as concrete redirect work, not rejected code.
- R01-PLAY blocked: emulator.start explicit mapper69 rejection. Inputs, traces, snapshots, RAM patches and later-game routes cannot run. Retry only after separately authorized debugger feature change; no ROM header patch or alternate binary used.

Static discovery parameter correction: NES entry CPU address must be top-level cpu_address or view, unlike generic protocol example cpu_context.cpu_address; initial failed query retained. Discovery conservatively stops at any JSR or indirect/indexed store, even RAM writes; this is documented conservatism rather than proof that remaining bytes are unknown in principle. Final map validation and byte-identical checkpoint pass-004 succeeded. Per-bank counts and exact classification are in progress.md.

No previous rejected structural proposals. No terminal review needed for this nonterminal first easy-gain handoff. Redirect should resolve ready leads together with connected continuations. Structural and reachability claims remain separate.

## Round 1 redirect, pass-004, round-01-static

Combined ready static batch after connected and independent investigation: 23715 proposed new code bytes +2100 data bytes (25815 total); zero accepted changes this phase. Completed C117 clear/C229 mapper contracts, sound/control driver, bank2/4/5 actor-state graphs, seven bank5/twelve bank4 handlers, bank13 level setup, bank8 options and bank12 title, bank14 transitions/services, exact level descriptors and IRQ secondary effects. First9 IRQ selector values now supported by descriptor records. Finite tables and arithmetic lookup non-code ranges proposed with specific or generic roles according to evidence strength.

Primary report: analysis/redirect/round-01-static/findings.md; exact code/data proposals and unknown complement in that directory. Raw traversals had corrected inline pointer and constant-branch barriers; use review-code.json only as proposal list, with independent acceptance still required. Rejected provisional bank0 hypothesis superseded by actual BD59/BF51 selector2 writes and coherent bank2 code; retain negative artifact, never classify bank0 from it. Runtime unchanged/unavailable TK-001. Remaining upload provenance, actor factory domains and selected sound-stream origins are concrete missing contracts; no terminal exhaustion asserted. Leads R01-CLEAR/R01-BANKHELPER/R01-IRQ updated, R01-LOOKUPS added, prior outcomes preserved.


## Round 2 disassembly, pass-006

ROM classification: 76.93% (302530 / 393232 bytes classified; 90702 bytes unknown).

Per-bank physical code/data/unknown counts: analysis/round-02/counts.json and progress.md Round2 table. Independent exact review accepted all25815 proposed bytes with Static provenance after mapper, carry/stack, finite handler, descriptor/IRQ, palette and arithmetic constraints. Exact consumer/competing-interpretation reasoning in analysis/round-02/review.md; decoded primary instruction audit in decode-audit.json; refreshed unknown-ledger.json preserves90702 physically unknown bytes. No old classification overwritten. Every ready lead has a verified structural outcome recorded in leads.json; remaining natural selectors and indirect domains localized rather than made prerequisites for accepted code.

Fresh map validation and pass-006 byte-identical ca65 rebuild succeeded. pass-005 preserved as duplicate accepted-input map after rejected unsaved data-view parameter error; corrected on fresh map load. No toolkit modification or newly inferred toolkit issue. TK-001 still prevents every runtime milestone and RAM patch; scenarios reviewed with unchanged capability premise. Next phase full current-source commentary, then focused redirect on remaining indirect/table/upload contracts. Nonterminal easy-gain stopping after complete supported connected batch; no repeated failed experiments or unsupported whole-gap classifications.

## Round2 redirect, pass006

Combined static proposal ready in analysis/redirect/round-02-static:38667code+52032data bytes, not accepted. Thorough connected actor/type/state/helper and independent factories, tile/IRQ/scene, sound/DMC, metasprite, actor-record, RAM-copy-origin and residual boundary investigations. Primary artifacts, corrected failed attempts and exact frontiers in findings.md; exact accepted unknown ledger retains90702bytes with proposals. Proposal residual3bytes isolated bank8:8BB2JMP lacking role/entry proof. No terminal exhaustion. Metadata-only CHR upload origins do not reclassify storage or establish execution. Full handoff analysis/handoffs/round-02-redirect.md; persistent leads preserve all prior outcomes.


## Round 3 disassembly — one pass complete, ROM incomplete

ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).

| Physical section | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| PRG 00 | 6548 | 1644 | 8192 | 0 |
| PRG 01 | 0 | 8192 | 8192 | 0 |
| PRG 02 | 7128 | 1064 | 8192 | 0 |
| PRG 03 | 0 | 8192 | 8192 | 0 |
| PRG 04 | 6830 | 1362 | 8192 | 0 |
| PRG 05 | 6728 | 1464 | 8192 | 0 |
| PRG 06 | 0 | 8192 | 8192 | 0 |
| PRG 07 | 0 | 8192 | 8192 | 0 |
| PRG 08 | 6788 | 1401 | 8189 | 3 |
| PRG 09 | 0 | 8192 | 8192 | 0 |
| PRG 10 | 2539 | 5653 | 8192 | 0 |
| PRG 11 | 7357 | 835 | 8192 | 0 |
| PRG 12 | 6525 | 1667 | 8192 | 0 |
| PRG 13 | 6451 | 1741 | 8192 | 0 |
| PRG 14 | 5070 | 3122 | 8192 | 0 |
| PRG 15 | 835 | 7357 | 8192 | 0 |
| Header | 0 | 16 | 16 | 0 |
| CHR 00–31, each | 0 | 8192 | 8192 | 0 |

Accepted38667 code +52032 data bytes after independent17175-instruction/145-span decode and bank-aware structural review. All old regions preserved,1852 added Static regions, zero Observed bytes. Primary sprite partition/actor extent checks independently recomputed; ten local direct fallthroughs resolve to nine C0C6 literal barriers and bank5 constant BEQ. Exact evidence and competing interpretations: analysis/round-03/review.md, decode-audit.json, primary-checks.json, annotation-plan.json. Total code62799bytes. New unknown-ledger.json retains exactly physical68546 length3 bank8 CPU8BB2, isolated4C338C. All16 PRG-bank word-reference search negative; lack of entry does not classify bytes. Rounded100.00% is incomplete with3 unknown.

Final checkpoint pass-008, maps/pass-008.json, generated/pass-008/game.asm, analysis/pass-008-verification.json confirms byte_identical=true after validation/save/export. Map SHA256 b18f67ed4a03000ed2ed9e79239b7fb166fd1ce8ba04a84eba4d365c86ff421a. ROM SHA256 unchanged24fb1cf2d308e07dbedadc3822ca07b2b48b7ebc1da0150e63203d2fdedfcc75. pass-007 preserves an unchanged pass-006 duplicate caused by premature checkpoint sequencing while annotation was still in flight; no evidence overwritten. Final checkpoint awaited confirmed saved state. No toolkit bug inferred from caller sequencing error.

R02-ACTORS/FACTORIES/TILE-IRQ/SOUND/SPRITES verified structurally for exact reviewed batch; R02-UPLOAD-ORIGINS verified metadata-only, already classified CHR left intact. No uploaded views, RAM execution or natural selection claims. Full current-source commentary remains required for expanded code. One pass only; no commentary/redirect run. Residual is a nonterminal local ambiguity for subsequent reasoning review, no terminal exhaustion asserted.

Broad progression/ending/RAM shortcuts checklist reviewed, unavailable under unchanged TK-001 mapper69 debugger admission. No unchanged failed runtime experiment repeated. See [toolkit issues](toolkit-issues.md), one documented missing feature, no toolkit changes. Disk/evidence preserved, input unchanged, no publication/archive/deletion.


## Batch fallback disassembly — complete scoped pass

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

This map count includes 3 bytes of explicit user-directed Manual fallback, not new evidence-backed structural proof. The prior evidence-backed count remains 393229; the physical code-versus-data ambiguity remains unresolved. No full semantic or compatibility claim follows.

Scope/authorization: the user explicitly authorized this batch exception only after independently verifying the exact physical complement is at most 10 bytes. The byte-union audit found exactly [68546,68549), length3, physical PRG bank8, CPU alias $8BB2 under effective banks [8,13,14,15]. These bytes admit a JMP $8C33 veneer or opaque storage interpretation; the competing interpretations and prior rejection reasoning remain in analysis/final-handoff.md, analysis/coordinator-rejection-review.md and analysis/redirect/round-03-contract-review/findings.md. No new research, code proof, dead-code inference or natural-reachability claim was made.

Supported map.annotate added one unlabeled Data region, Manual provenance, with unknown meaning and explicit uncertainty. All prior regions are preserved exactly. Input checkpoint pass-008; output checkpoint batch-fallback-009. Before/after exact union audits: analysis/batch-fallback/before-union-audit.json and after-union-audit.json. Before:393229 classified/3 unknown; after:393232 classified/0 unknown, zero classified overlap. Totals:62799 Code,68289 Data,262144 Image. All16 physical PRG banks have8192 classified/0 unknown; header16/0; CHR262144/0. The sole gain is3 Manual Data bytes. Runtime observed bytes remain zero.

Configured /root/romatlas/target/release/romatlas validated the store, saved analysis/state.json, exported maps/current.json, then generated a fresh byte-identical checkpoint: maps/batch-fallback-009.json, generated/batch-fallback-009/game.asm, analysis/batch-fallback-009-verification.json. Store validation: analysis/batch-fallback/map-validation.json. Supported RPC transcript: analysis/batch-fallback/rpc-results.json. A rejected empty-label request made no mutation; corrected omission/null label succeeded, with rejected response preserved separately. No toolkit issue inferred.

Map/state SHA256:51e3a8458cdfa5eae69bd407bba5efb0ecf1666f8e1e04631790f1babe0fc1e9. Input SHA256 unchanged:24fb1cf2d308e07dbedadc3822ca07b2b48b7ebc1da0150e63203d2fdedfcc75. Prior internal state preserved in analysis/batch-fallback/state-before.json; pass008 and all prior evidence retained.

One disassembly phase only; no commentary or redirect performed. No leads consumed or produced; authorization replaces research only for this exact fallback. Fresh commentary/editorial acceptance and reading-copy refresh remain coordinator-owned. Prior readable source remains preserved and must explain this Manual fallback without claiming structural certainty. Existing TK-001 mapper69 admission limitation remains in analysis/toolkit-issues.md; no new toolkit issues found. Gameplay, bosses, ending and RAM scenarios remain unavailable; no unchanged experiment repeated. Disk about4.8GiB free, scratch20KiB; no cleanup required or performed. No toolkit changes, archive, deletion, publication or commit. Coordinator will print the classification percentage before commentary.
