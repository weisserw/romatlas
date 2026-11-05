# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## R01 initial structural evidence
- Baseline raw emission verified, not counted as classification.
- Hardware vectors at physical $2000A..$2000F target NMI $D1C8, reset $C000, IRQ $C0BA in effective PRG context [0,1,14,15]. Reset begins mapper reset INC $C000, PPU disable, serial MMC1 configuration then initialization; mapper/call barriers preserved.
- Actual opcode aggregates plus constrained branch/fallthrough discovery justify 3,582 code bytes initially. Candidate views retained in analysis/round01-candidates.json, RPC evidence round01-rpc.jsonl. Observed execution and Static path bytes labeled separately. Initial generated source round01-disassembly-initial byte-identical.
- Declared 32KiB CHR section is planar PPU image data, separate CPU space. iNES header and hardware vectors independently bounded.
- Uniform-run review: inspected value transitions and PRG bank boundaries, nearby encoded/repeated records, and current code overlaps. Long FF runs and repeated 00/03/0A/FE runs have generic non-code structure; consumer/schema not asserted. Bank13 uniform FF terminates before a small boot-like trailer, so trailer remains unknown.
- No prior actionable leads. New strongest gameplay lead: RAM $C8 copied to $668C during initial bank0 setup, plausibly stage selector; investigate index domain and dependent initialization before shortcut.

### R01 password shortcut contract
Source-bounded editor at physical $1961 initializes $0414..$0417 to ASCII '0'; $1995..$1A03 edits cursor/characters; $1A2C onward compares these four bytes against ASCII password records. ROM table begins physical $1A93 ('I8TAKA916KAI8NYU2APPOS85SUGA...'). Clean password screen saved as analysis/round01-password-clean.snapshot at frame96 following reset30, Select+Start to skip intro/title selection, then Select+Start to enter Continue. All tap recipes hold2frames, release2frames. This is a supported RAM buffer edit test, subsequent observations artificial.
SUGA accepted LAST AIR FORTRESS; $45=7 at runtime, $C7 quest flag0, $C8=1. Prior $C8 stage-number premise rejected by actual compare consumer: table index&7->$45, index&8->$C7. Stage8 shooter at frame820; A/B/right/up45frames provides artificial observed fetches (trace summary aggregate_dropped0, 4,834 instruction starts in session at this point). Snapshot analysis/round01-before-interior.snapshot frame865. Supported fixed-bank transition stub CPU $CF4E physical$1CF5E loads bank selector2 through $D427, resets stack to$FA, jumps$8000, so no unknown caller stack dependency; testing PC=$CF4E after operation-boundary step and buttons release to initialize interior with stage8/current collected energy state. Exact RPC logged.
Interior stage8 initialization observed effective [4,5,14,15], entry message then room view (analysis/interior8-action.ppm) under A/B/right/up45frames; additional A/right/down30frames to exercise directional/weapon mechanics. No claim core destroyed or natural completion. Durable interior8 snapshot.
Ending test contract: fixed-bank CPU $CF61 loads selector0, calls mapper setter$D427 and jumps common bank0 vector$800C->$8FC0. $8FC0 checks known password quest flag$C7; firstquest0 branches$92F8; secondquest nonzero enters alternate ending setup with stream pointer$9131. Using initialized stage8 context and existing stack, this tail entry performs its own scene setup and has no required return/caller stack data. Direct PC transition tests artificial ending reachability only. Gameover entry$CF59->$8006->$95CA distinct, not used as ending.

## Round 01 disassembly completed — incomplete ROM
ROM classification: 41.45% (67911 / 163856 bytes classified; 95945 bytes unknown).

9950 code bytes, 23281 generic/structured data bytes, 1912 Text bytes, 32768 Image bytes. All 663 regions carry structural evidence; code views preserve physical identity and effective banks. Semantic labels/readability remain commentary work. Exact unknown complement and per-bank counts: analysis/round01-counts.json.

| Physical section | Code | Data/text/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2450 | 1972 | 3770 |
| PRG 01 | 799 | 3350 | 4043 |
| PRG 02 | 0 | 0 | 8192 |
| PRG 03 | 0 | 1448 | 6744 |
| PRG 04 | 912 | 0 | 7280 |
| PRG 05 | 935 | 1631 | 5626 |
| PRG 06 | 0 | 0 | 8192 |
| PRG 07 | 0 | 631 | 7561 |
| PRG 08 | 0 | 0 | 8192 |
| PRG 09 | 0 | 274 | 7918 |
| PRG 10 | 0 | 319 | 7873 |
| PRG 11 | 0 | 3584 | 4608 |
| PRG 12 | 0 | 2821 | 5371 |
| PRG 13 | 0 | 8181 | 11 |
| PRG 14 | 4854 | 0 | 3338 |
| PRG 15 | 0 | 966 | 7226 |
| CHR | 0 | 32768 | 0 |

Execution is separate: 6033 unique physical opcode starts; 8097 contextual roots, 6669 physical edges, aggregate_dropped=0. Detail capacity0 intentionally drops detail; request/reply evidence bounded at ~31MiB, no raw instruction stream. Replay/snapshot restoration means aggregate frame/cycle history is not one monotonic natural playthrough. Observed bytes have fetched-root provenance; connected static instructions remain Static.

Verified final checkpoint: maps/round01-disassembly.json, generated/round01-disassembly, analysis/round01-disassembly-verification.json. map.validate succeeded; generation and fresh assembly verification byte_identical=true. Map SHA256 84eaf5c077665aeb5d3090b27778b7c129630df53d8b5ae41b114074e19c1397. Input SHA256 unchanged d72122bb27376997493440c5a693eab566cb96e7150d5c059e2efaeadefeb1e1. Current internal state analysis/state.json and maps/current.json saved. Baseline and initial checkpoint preserved.

Lead outcomes: R01-L1 password table verified+annotated; R01-L2 C8 stage premise superseded by consumer proof ($45 index,$C7 quest). R01-L3..L6 pending redirect: room data/bounds, reactor+escape state, credit route, static call/bank contracts. No prior ready leads existed. Middle fortress/boss/core progression deferred with missing contracts in scenarios.json; representative natural stage1 shooter, artificial final-stage shooter/interior and both ending branches visited. Final congratulations remained stable after Start+600frames; final20frames produced no new code. No repeated long waits warranted. Detailed room/bank semantics require connected redirect research, not another unchanged gameplay sequence. This is a phase handoff, not terminal investigation exhaustion.

Toolkit findings: one confirmed documentation/API mismatch NES-001 (shared input button/pressed silently ignored on NES), zero suspected issues, zero missing-feature entries; no blockers after buttons-array workaround. See analysis/toolkit-issues.md. No toolkit edits, archive, deletion or publishing.

Final live frontier: artificial congratulations scene, frame2204, cycles65633956, CPU PC$D3CD mid-instruction, effective banks[0,1,14,15]; durable snapshot analysis/round01-final.snapshot. New server: map.load current state, emulator.start, snapshot.import with path, reassert buttons[], enable desired tracing. Never assume named snapshots persist. Source restoration/readability audit belongs to next commentary phase.

## Round02 disassembly complete — incomplete ROM
ROM classification: 99.94% (163750 / 163856 bytes classified; 106 bytes unknown).

Accepted95,839 new Static bytes in448 regions after independent review:14,834 Code,80,957 Data,48 Text. Total roles: {'Code': 24784, 'Data': 104238, 'Image': 32768, 'Text': 1960}. All1111 regions have evidence. Exact physical unknown complement and per-bank kinds in analysis/round02-counts.json.

| Physical section | Code | Data/text/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 3931 | 4261 | 0 |
| PRG 01 | 2971 | 5210 | 11 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 0 | 8181 | 11 |
| PRG 04 | 6970 | 1222 | 0 |
| PRG 05 | 3648 | 4524 | 20 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 0 | 8181 | 11 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8181 | 11 |
| PRG 10 | 0 | 8192 | 0 |
| PRG 11 | 0 | 8181 | 11 |
| PRG 12 | 0 | 8192 | 0 |
| PRG 13 | 0 | 8181 | 11 |
| PRG 14 | 7264 | 908 | 20 |
| PRG 15 | 0 | 8192 | 0 |
| CHR | 0 | 32768 | 0 |

Input checkpoint84eaf5c077665aeb5d3090b27778b7c129630df53d8b5ae41b114074e19c1397; output checkpoint maps/round02-disassembly.json SHA93c0081fcdf9d4b49643f903e12e656fc94e073182258da7701ce4ee7ea82be1. map.validate succeeded; fresh generated/round02-disassembly rebuilt byte-identically, analysis/round02-disassembly-verification.json. Current analysis/state.json and maps/current.json saved. ROM SHAd72122bb27376997493440c5a693eab566cb96e7150d5c059e2efaeadefeb1e1 unchanged. Prior checkpoints/source/commentary/evidence preserved.

R01-L3..L8 independently verified and annotated as one deduplicated batch. Review and decisive counterexample scope: analysis/round02-review/review.md; independent audit and fresh configured decoder primaries beside it. R01-L9 remains106 unknown bytes/10 ranges: seven11-byte bootstrap signatures and three utility-like ranges of9/5/15bytes. No natural caller prerequisite imposed; bootstrap fetch/mapper context or independent routine-role evidence remains the discriminator. No terminal exhaustion claimed. No new gameplay replay on unchanged records: broad representative systems already visited including actual STAFF, initialized-core death and escape (artificial), natural stage1.

Execution coverage remains separate: no new execution in this phase; round01 aggregate6033 physical opcode starts remains the last consolidated recorded metric, redirect contexts preserve additional artificial observations without claiming an updated unified count. New structural bytes all Static. Full updated-source readability review is pending a separate commentary phase; existing commentary/source/game.asm SHA010b8d9eb146c2089195468a490ede3cfc45bbd1e416d368246bf8e7d59d52eb unchanged.

Toolkit report analysis/toolkit-issues.md retains one confirmed documentation/API mismatch NES-001, zero suspected bugs or missing features; no blockers and no new issues this pass. No toolkit edits, archive, cleanup/delete or publishing. Exact handoff frontier: round02 classification/verification complete, awaiting full current-source commentary then redirect of residual106bytes.

## Round03 disassembly complete — classification full; editorial gates pending
ROM classification: 100.00% (163856 / 163856 bytes classified; 0 bytes unknown).

Accepted106 Static bytes in24 regions:78 Code,28 Data. R02-L1/L2 verified; exact independent review analysis/round03-review/review.md, fresh configured decodes and independent-audit.json. All prior map records byte-for-byte identical. Stored reset templates are structurally code; actual INC changes upper banks to14/15 and nextFFF9 fetch isFF, so no working reset/JMP flow claimed. Correction: postreset lower slots are selected pair[b-1,b], not necessarily0/1. Vector Data annotations correctly omit code-only views.

| Physical section | Code | Other classified | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 3931 | 4261 | 0 |
| PRG 01 | 2978 | 5214 | 0 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 7 | 8185 | 0 |
| PRG 04 | 6970 | 1222 | 0 |
| PRG 05 | 3664 | 4528 | 0 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 7 | 8185 | 0 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 7 | 8185 | 0 |
| PRG 10 | 0 | 8192 | 0 |
| PRG 11 | 7 | 8185 | 0 |
| PRG 12 | 0 | 8192 | 0 |
| PRG 13 | 7 | 8185 | 0 |
| PRG 14 | 7284 | 908 | 0 |
| PRG 15 | 0 | 8192 | 0 |
| CHR | 0 | 32768 | 0 |

Exact kinds/annotation counts: analysis/round03-counts.json. 1135/1135 regions have evidence. Valid saved analysis/state.json/maps/current.json SHAca0e8adc5134a79e32469916d8bf67dc033e26142cf3363b41abd5f4ed0644c0; fresh checkpoint maps/round03-disassembly-accepted.json, generated/round03-disassembly-accepted and analysis/round03-disassembly-accepted-verification.json byte_identical=true. ROM unchanged SHAd72122bb27376997493440c5a693eab566cb96e7150d5c059e2efaeadefeb1e1. An initial annotation request supplied code-only views to Data and was rejected atomically; no saved changes then. Premature checkpoint round03-disassembly preserves unchanged old map SHA93c0081fcdf9d4b49643f903e12e656fc94e073182258da7701ce4ee7ea82be1 and is NOT the accepted output; preserved rather than deleted. Corrected accepted checkpoint above is authoritative.

No new execution; last consolidated6033 physical opcode starts remains separate from full structural classification. Saved broad gameplay scenarios unchanged; no discriminating new replay needed. All new roles Static, no natural reachability claim. Existing commentary/source/game.asm unchanged SHAcc6041f9e700c2fd54aee479175e5c9706f977cc648c29354e7cdde6a92f5283, covering only prior checkpoint. Required next separate phase is full updated-source commentary preserving/carrying old work; final editorial audit and final byte-identical verification remain pending. No ROM completion claim.

Toolkit: existing NES-001 confirmed documentation/API mismatch; zero new issues, zero suspected or missing features, no blockers; analysis/toolkit-issues.md. Code-only view rejection was documented behavior, not a toolkit defect. No toolkit edits, archive, cleanup/delete, publishing or phase launch. No unknown residual/rejection to redirect. Worker writes finished.
