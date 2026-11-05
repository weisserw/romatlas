# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 01 disassembly in progress
Baseline maps/baseline.json and generated/baseline verified byte-identical; report analysis/baseline-verification.json. ROM unchanged SHA256 d72122bb27376997493440c5a693eab566cb96e7150d5c059e2efaeadefeb1e1. Persistent live session process running python3 run.py serve via /tmp/romphase.py; request/reply evidence analysis/round01-rpc.jsonl. Startup at frame 168; vector roots $C000/$D1C8/$C0BA, context [0,1,14,15]. Initial wrong nested cpu_context root rejected without mutation; corrected cpu_address/context fields work. Trace detail dropped intentionally bounded, aggregates intact. No classification saved yet.
Observed-context static discovery candidate artifact analysis/round01-candidates.json saved (1557 unique starts in initial batch); classification pending review, not a completion claim. Working gameplay identity Air Fortress US; external route findings recorded in analysis/gameplay.md. Long intro/story observed to frame1459; Start during opening/story has not yet entered gameplay. Trace disabled while advancing story to avoid unproductive detail costs; reenable before gameplay. Snapshot analysis/round01-intro.snapshot at frame258 durable, same-build only.
Confirmed Air Fortress US title in analysis/actual-start.ppm at frame4345. All prior button/pressed requests were ineffective due NES API schema; report NES-001 in analysis/toolkit-issues.md. Correct buttons array skips intro immediately and reaches title. Input Start (buttons array) pressed2frames then released; entering stage1 with tracing enabled. Initial checkpoint round01-disassembly-initial verified; 36,372 classified (22.20%).
Stage1 reached naturally at frame4947 after corrected Start. analysis/shooter-active.ppm shows player ship and scenery; durable analysis/round01-shooter1.snapshot. Intermediate classification gains +1,256 code bytes and uniform non-code annotations saved; exact final counts pending. Ordinary input A/right held through approach then A/up30 frames to exercise shooter.
Source-supported SUGA buffer patch accepted: analysis/stage8-accepted.ppm says LAST AIR FORTRESS; RAM $45=7 confirms index. $C8=1 is not stage number (earlier hypothesis superseded); password compare stores table index&7 to $45 and index&8 to $C7. Stage8 live snapshot analysis/round01-stage8-active.snapshot at frame820 artificial, with screenshot analysis/stage8-shooter.ppm. Exercising A/B/right/up45frames under tracing.
Artificial interior transition accepted: PC=$CF4E after operation boundary switches effective banks to [4,5,14,15], entering normal game initialization; analysis/interior8.ppm shows 'YOU ARE NOW ENTERING THE AIR FORTRESS.' Further120frames plus45frames A/B/right/up planned to exercise gameplay. This is artificial reachability only; no natural stage-clear claim.
Interior8 room gameplay confirmed in analysis/interior8-action.ppm and interior8-move.ppm; A/B/right/up45 and A/right/down30 advanced room view. Ending dispatcher PC=$CF61 reached firstquest ATTENTION screen after30 traced +240 untraced frames. Restored interior8 snapshot, reasserted buttons[], patched quest flag $C7=8 (password consumer domain), operation boundary then PC=$CF61;30 traced+360untraced frames reached final CONGRATULATIONS screen (analysis/ending-final.ppm). Natural stage completion/core destruction not demonstrated. All edits exact in round01-rpc.jsonl; final snapshot export underway. First/last stage systems and both endings now representative visited, credits follow-up bounded.

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

## Round 01 commentary in progress
Prepared commentary with documented commentary.py prepare; frozen baseline matches maps/round01-disassembly.json. Initial source rebuild byte-identical. Read classified instruction inventory across presentation/shooter banks 0/1, interior banks 4/5 and fixed bank14, plus classified data/unknown complement. Wrote analysis/commentary-round01-edit.py with explicit physical scope groups and substantive mechanical interpretations; transformations, operand symbols, review/source links and fresh verification remain pending. Exact remaining frontier: apply and validate these source annotations across all 663 classified regions, all unknown complement notes, resolve bank-aware target names, inspect transformed source and verify. Do not hand off to redirect yet. Inline descriptors after $D4B3/$D4B9/$D4BF must remain raw where unclassified. Current source still preparation copy until edit script is completed/applied.

Round01 commentary source applied and refined: 588 classified code fragments in 136 meaningful mechanical units; 923 physical inventory entries have source-linked review. Known RAM/hardware/table operands named; cross-slot calls resolved only from recorded effective bank contexts. All raw unknown boundaries annotated, inline argument barriers retained. Fresh verification byte_identical=true, source game.asm SHA256 6f2c2f7d74f8115d6d4a1847421be4cc7a46f1d44c5b8a04d1d8f93b12204442. Current frontier: final semantic consistency inspection, document investigated uncertainties and review limits, refresh reading-after.asm after final edits, then fresh verification/handoff. No classification/map changes. The prior 'apply pending' frontier is superseded.

## Round 01 commentary complete — current source only
Entire round01 current-source readability scope completed: 588 code fragments / 9,950 code bytes in five code-bearing banks (0,1,4,5,14), 75 classified non-code regions and 260 unknown complement entries; all 923 physical entries source-linked and 136 mechanical units explained. Known routines/branches/RAM/hardware/tables/constants named in commentary/source/game.asm. Unknown gaps retain explicit local boundaries, including inline transfer arguments; no classification change. Final semantic inspection corrected shot overflow/unknown underflow, pointer-only transfer mode and scratch lifetimes. No pending readability ranges. Whole-ROM editorial/archive acceptance remains pending.

Fresh final commentary/verification.json byte_identical=true, source SHA256 010b8d9eb146c2089195468a490ede3cfc45bbd1e416d368246bf8e7d59d52eb; commentary/reading-after.asm refreshed. maps/current.json, frozen disassembly, analysis/state.json and commentary/phase1-state.json all retain 84eaf5c077665aeb5d3090b27778b7c129630df53d8b5ae41b114074e19c1397; ROM hash unchanged. Classification remains 41.45%, 95,945 unknown. Significant semantics and fourteen unresolved code target contracts are explained in commentary/uncertainties.md; no natural core/ending route inferred. Toolkit NES-001 remains the sole confirmed mismatch; zero suspected/missing features, no blocker. No toolkit/map edits, archive, cleanup or publishing.

Exact phase frontier: commentary finished, awaiting coordinator source acceptance; redirect has not started. Compact next-phase handoff analysis/handoff-round01-commentary.md identifies artifacts/examples, unresolved R01-L3..L6 and verification identity. Frozen comparison source preserved. Existing live emulator frontier is unchanged from disassembly.

## Coordinator boundary: round01 commentary accepted; redirect active
Inspected handoff, uncertainty record and source examples for password selector, inline transfer, MMC1, PPU queue and HUD mechanics. Independently ran `python3 /root/romatlas/scripts/commentary.py verify . --json`: byte_identical=true, game.asm SHA256 010b8d9eb146c2089195468a490ede3cfc45bbd1e416d368246bf8e7d59d52eb. Current-source readability gate accepted; final editorial acceptance remains pending. Separate worker /root/round01_redirect starts from analysis/handoff-round01-commentary.md and unchanged round01-disassembly checkpoint; pending lead IDs R01-L3..L6. Previous goal turn made source-edit progress and verified live worker waits; no blocking condition. Archive.py document-selection contract inspected read-only; explicit options will be finalized after terminal review. No archive or cleanup authorized.

## Round01 redirect in progress
Accepted current-source commentary checkpoint 84eaf5c077665aeb5d3090b27778b7c129630df53d8b5ae41b114074e19c1397. Research attempt analysis/redirect/round01-static-contracts; inspect direct unresolved calls, bank setter and connected room/ending paths before any handoff. Map/source/ROM remain unchanged. Initial disk free 9.3GiB; no cleanup performed. Exact unknown complement remains round01-counts.json (95,945 bytes /260 gaps); preparing evidence-linked dispositions.

Round01 redirect substantive checkpoint: static timing corrects earlier supposed congratulations terminal; actual STAFF credits visited, frame4221, saved analysis/redirect/round01-static-contracts/credits-active.snapshot and credits-entry.ppm.png. Source-backed stage data generic proposal64,607 unknown bytes/19 exact intervals; bank-split272-entry unknown ledger preserves95,945 total. Last11-byte odd-bank reset-like signatures excluded. Core type$17 death and type$18 escape consumer established; first experiment interrupted pendingNMI before initializer, corrected run_until verifies type23/health20 and continuation remains live. Durable full findings.md records primary evidence, correction and exact frontier. Next: finish core/escape, all35 supported handler contracts, inline boundaries and audio-data independent avenue. No source/map changes.

Redirect checkpoint: core-fixed experiment verified type$17 death ->$65BF=1; separate documented proximity bypass selected normal escape launch, image escape-active.ppm.png/frame1784, flags$65BF/$65C0=[1,2]. All35 interior and31 shooter handler graphs followed;99 source-supported boundary roots extend ordinary call continuations. Configured decoder validates6115 unique proposed Static starts/13741bytes, zero overlaps. Corrected shooter$9DD0 consumes caller return plus3 inline bytes and never locally resumes; invalid provisional continuation removed. Audio7785byte generic-data candidate independently supported by direct table consumers. Exact current frontier: descriptor/record data extents, fixed-code continuation bank contexts and combined proposal/ledger/handoff audit. Latest findings.md records corrections and primary links; source/map unchanged.

## Round01 redirect complete — ready for independent disassembly
Combined proposal analysis/redirect/round01-static-contracts/proposal.json contains95,839 proposed NEW bytes in448 regions (14,834 Code /80,957 Data /48 Text);6,614 distinct starts validated by configured decoder, zero overlap/conflicts. Zero newly accepted bytes: current67,911classified/95,945unknown unchanged. IF all independently accepted,106bytes remain in10 explicit structural deferrals. Exact original complement/neighbor ledger preserved with proposed dispositions. Full connected actors/player/secondary handlers, room/stage data, core+escape and credits visits, bounded containers, inline/mapper contracts and audio/generic lookup audit are complete for this batch. findings.md documents corrections and actual stopping reasons; no terminal exhaustion claimed.

Handoff analysis/handoff-round01-redirect.md; leads R01-L3..L8 ready and R01-L9 deferred106bytes, historical outcomes preserved. Next phase is independent disassembly then complete current-source commentary. ROM/map/state/source hashes unchanged (checkpoint-hashes.json). No toolkit edits, cleanup/delete, archive or publish. Successful credits/core/escape snapshots and captures remain local.

## Coordinator boundary: round01 redirect accepted for round02 review
Read compact handoff, proposal summary and connected/independent stopping rationale. No terminal exhaustion claimed:95,839 proposed new bytes require independent disassembly. Separate round02_disassembly worker starts from unchanged84eaf5c077665aeb5d3090b27778b7c129630df53d8b5ae41b114074e19c1397 checkpoint and R01-L3..L8 ready; R01-L9 retains106 projected residual bytes. Round01 redirect worker confirmed writes finished before launch. Preliminary archive-options.json now preserves redirect handoff as well as findings; terminal accounting still required. Previous turn was progress (proposal artifacts) and verified live-worker wait, no blocker.

## Round02 disassembly in progress
Separate independent classification review of round01 redirect proposal; input map/state SHA84eaf5c077665aeb5d3090b27778b7c129630df53d8b5ae41b114074e19c1397. Reviewed disassembly prompt, evidence policy, workflow, handoff, findings, graph construction and data claims. Review specifically checks stack-consuming inline helpers, return continuations, effective physical banks, bounded containers and generic noncode pattern evidence before annotation. Existing commentary remains untouched. Leads R01-L3..L8 in review; L9 retained deferred pending discriminating contract. Output checkpoint planned round02-disassembly; no classification change yet.

Round02 review milestone: accepted the full deduplicated95,839-byte batch after independent physical/view/overlap audit, independent five-container endpoint checks, source-backed dispatch/fragment roots, fresh stack-reader and mapper decodes, and inspection of all57 small blocks and generic stage/audio evidence. Important independent nuance: C77E optionally aborts its caller on exhaustion, but successful-slot path supports local continuation; 9DD0 remains unconditional nonlocal return. Exact rationale analysis/round02-review/review.md and independent-audit.json. No annotations saved yet;106-byte L9 local deferrals retained.

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

## Coordinator boundary: round02 disassembly accepted; commentary active
Inspected phase handoff, byte-identical report and independent review of stack/bank/container/generic-data claims. Required user-facing classification printed:99.94% (163750/163856;106unknown). Separate round02_commentary begins full current-source readability on93c0081fcdf9d4b49643f903e12e656fc94e073182258da7701ce4ee7ea82be1 after disassembly worker finished. Prior commentary must be preserved/carry valid physical review; final editorial acceptance and residual redirect still pending. Preliminary archive document selection expanded for round02 handoff/independent review; refresh all historical Markdown at terminal. Previous turn produced accepted map/checkpoint and verified live-worker waits, no blocker.

## Round02 commentary in progress
Preserving entire round01 commentary before fresh preparation. Current checkpoint93c0081fcdf9d4b49643f903e12e656fc94e073182258da7701ce4ee7ea82be1 differs from frozen round01 baseline. Exact frontier: prepare new source, carry valid physical-range review/names, explain all newly classified code/data and update stale dependencies. Full current-source readability remains pending; no redirect or final audit started.

Round02 commentary substantive draft: fresh baseline prepared and whole prior directory preserved at analysis/round-02/commentary-round01-preserved. Explicit new unit manifest/edit script analysis/round02-commentary-edit.py covers all new presentation/shooter/interior/player/secondary/fixed/audio mechanics, preserving prior physical code entries. Source draft written; current frontier is complete application of review/data notes and build correction (duplicate labels in first intermediate draft), followed by semantic consistency pass, verification/read refresh and handoff. No successful readability/verification claim yet.

## Round02 commentary complete — current source only
Preserved ENTIRE previous commentary at analysis/round-02/commentary-round01-preserved before fresh preparation. Carried valid physical entries/prose/review and revised changed dependencies. All24,784Code bytes/796fragments, classified noncode and106unknown bytes/ten boundaries now have current source explanations;1,121scaffold entries and228mechanical units source-linked/reviewed. New units explain35interior/31shooter handler families, player/secondary states, core/escape, STAFF timing, bounded containers, inline stack contracts and effective mapper continuations. Generic original schemas/resource units remain honest uncertainty.

Fresh final commentary/verification.json byte_identical=true; source SHA cc6041f9e700c2fd54aee479175e5c9706f977cc648c29354e7cdde6a92f5283; reading-after.asm refreshed. Map/state/frozen baseline SHA93c0081fcdf9d4b49643f903e12e656fc94e073182258da7701ce4ee7ea82be1 and ROM unchanged. No pending current-source readability ranges. Final editorial_audit remains pending; no audit/redirect/toolkit edits/archive/cleanup/delete/publish. Exact frontier: commentary complete awaiting coordinator acceptance; residual106byte R01-L9 structural questions remain for redirect. Handoff analysis/handoff-round02-commentary.md.

## Coordinator boundary: round02 commentary accepted; residual redirect active
Read handoff/uncertainties and actual source examples for TailInlineShooterGenerator, DecodeStageColumnStream changed bank context and FindSecondarySlotOrAbortCaller. Independently reran commentary.py verify: byte_identical=true, source SHAcc6041f9e700c2fd54aee479175e5c9706f977cc648c29354e7cdde6a92f5283. Full current-source readability gate accepted; terminal editorial audit pending. Separate round02_redirect launched after worker confirmed no further writes, starting R01-L9 and exact106-byte complement at93c0081fcdf9d4b49643f903e12e656fc94e073182258da7701ce4ee7ea82be1. Terminal reasoning review still required, no exhaustion accepted. Previous turn made source/verification progress and verified live-worker wait; no blocker. Archive-options selects commentary handoff, preliminary accounting will refresh at terminal.

## Round02 redirect active — residual contracts
Unique attempt analysis/redirect/round02-residual-contracts. Exact106-byte complement persisted; no map/source changes. Corrected inference: absence of direct callers is not a structural veto. Inspect sibling field contracts and vector/reset mapper contracts independently before inheriting L9 deferral.

## Round02 redirect complete — ready106-byte residual proposal
Attempt analysis/redirect/round02-residual-contracts: corrected independent static field/sibling/vector-origin contracts reopen R01-L9 without caller scans or runtime replay. R02-L1 proposes29 utility Code bytes; R02-L2 proposes49 bootstrap template Code +28 vector Data bytes in localized4/3/4 splits. Total proposed106, accepted0; actual106unknown/10ranges and99.94% unchanged. All24 proposal regions exactly cover unknown complement without overlap. Seven stored JMPs explicitly do NOT describe post-INC effective fetch: mapper changes upper banks to14/15 and nextFFF9 byte isFF. Stored code-template role is separately supported by vector-origin/self-vector reset/known reset-target/seven special-position copies. Full primary reasoning and competitor comparison in findings.md; fresh configured decode/masks/fixed-tail artifacts beside proposal. Utilities have no unknown continuations. Current-source semantic limits and saved broad gameplay assessed, no other concrete structural frontier. Handoff analysis/handoff-round02-redirect.md; map/state/source/ROM hashes unchanged; final editorial audit pending coordinator. No terminal incomplete/exhaustion, cleanup/delete/archive/publish/toolkit edits. Frontier: independent review R02-L1/L2; worker writes finished.

## Coordinator boundary: residual redirect ready; round03 disassembly active
Read compact handoff and substantive findings comparing utility contracts and vector-anchored stored templates with actual MMC1 fetch contradiction and concrete noncode competitor. Ready106-byte batch, not terminal exhaustion; exact structural claims require independent review. round02_redirect writes finished before separate round03_disassembly launch on unchanged93c0081fcdf9d4b49643f903e12e656fc94e073182258da7701ce4ee7ea82be1. Accepted classification remains99.94%/106unknown until annotation/verification. New handoff/findings selected for preservation; terminal accounting/editorial gates remain pending. Previous goal turn progressed residual inference/proposals and verified worker waits; no blocker.

## Round03 disassembly independent review active
Input state93c0081fcdf9d4b49643f903e12e656fc94e073182258da7701ce4ee7ea82be1. Reviewing exact106-byte residual proposal against immutable bytes, configured decoder, accepted RAM/queue/audio contracts and mapper source. Old commentary remains unchanged. Frontier: independent structural sufficiency review before annotation; no gameplay replay or phase launch.

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

## Coordinator boundary: full classification verified; round03 commentary active
Printed required user-facing100.00%(163856/163856;0unknown). Inspected accepted handoff/review/verification: authoritative round03-disassembly-accepted checkpoint ca0e8adc5134a79e32469916d8bf67dc033e26142cf3363b41abd5f4ed0644c0; premature old-map checkpoint explicitly excluded. Independent review preserves actual mapper fetch contradiction and corrects postreset lower slots to selected pair. Separate commentary launched after disassembly worker finished, must preserve/carry whole prior source/review and explain106new bytes. Structural classification full; final ROM/editorial/orchestration completion NOT yet established. Prior goal turn progressed accepted map and verified worker waits; no blocker. Final editorial audit, redirect closure and archive document accounting pending.

## Round03 commentary in progress
Preserved entire round02 commentary under analysis/round-03/commentary-round02-preserved before preparation. Authoritative current map ca0e8adc5134a79e32469916d8bf67dc033e26142cf3363b41abd5f4ed0644c0. Frontier: prepare new frozen baseline; carry unchanged physical source/review; explain new106 bytes and replace stale unknown/reset notes. No readability completion or editorial acceptance yet.

## Round03 commentary complete — current source
Current-source readability covers all163856 physical bytes:24862 Code,104266 Data,1960 Text,32768 Image,0unknown. The1135-entry scaffold is fully reviewed;252 meaningful units comprise228 valid carried units and24 additions. Entire prior commentary is preserved under analysis/round-03/commentary-round02-preserved. Fresh baseline/state/map SHA256 ca0e8adc5134a79e32469916d8bf67dc033e26142cf3363b41abd5f4ed0644c0; authoritative checkpoint maps/round03-disassembly-accepted.json. Premature old-map checkpoint is excluded.

All unchanged physical source blocks retain their prior useful prose/symbols and review; a mechanical comparison confirmed1071 blocks byte-for-byte after removing obsolete unknown-gap emission. Full source inventory retains carried classified-data format/uncertainty explanations. Refreshed source links account for shifted lines. No stale unclassified gaps or pending source review remain.

New106 bytes explained as78Code/28Data across24 regions. ClearInteriorVerticalVelocity clears PlayerVelocityYLo/Hi; DiscardPendingPpuQueue discards the published queue without changing its build cursor and documents interrupt atomicity limits; DecodeSignedAudioFiveBits returns signed five-bit A-low/Y-high for the pitch helper using the existing stream/sign mask. Seven trailers explain4-byte stored prefixes,3-byte stored JMPs and4-byte reset/IRQ words. Prewrite mode2 aliases retain the selected lower pair after MMC1 reset and switch upper slots to14/15; actual nextFFF9 is bank15FF, not copied JMP. Structural templates do not establish working reset paths or natural fetches. Replaced stale unknown/rejection prose in source and uncertainties.

Final fresh verify byte_identical=true, source SHA256 973e08a3d00ee621678d2877f13ca0c55b52b5bc66da5a381d3cf2812fb1ce10; reading-after.asm refreshed SHA256 7f68b71308fca5670d5af90fafa249e258909f0d4adb14107d86b4388b783316. ROM SHA256 d72122bb27376997493440c5a693eab566cb96e7150d5c059e2efaeadefeb1e1 unchanged. No natural execution/caller claims added. Remaining semantic uncertainties retain ordinary local prose; no readability gaps or productive structural residual. All bytes classified does not itself establish final editorial acceptance. Ready for coordinator final acceptance/audit; audit deliberately not invoked in this phase. No classification/toolkit edits, redirect, archive, cleanup/delete or publishing. Toolkit existing NES-001 confirmed documentation/API mismatch, zero new issues, no blocker; analysis/toolkit-issues.md.

## Terminal coordinator acceptance — complete
ROM classification: 100.00% (163856 / 163856 bytes classified; 0 bytes unknown). Final round03 redirect closure found no actionable structural lead. Final editorial assessment corrected stale current uncertainty counts/residual prose, repaired review source-root paths and attached existing meaningful units to intersecting code regions. commentary/audit.json now passes all1135 regions,1039 unit-region associations (252 unique units), editorial_audit=true and fresh byte-identical rebuild. Configured release binary store/map validate both succeeded. analysis/current-audit.json independently confirms complete exclusive/evidenced physical union and current ROM/map/source identities. commentary/reading-after.asm refreshed. Final handoff analysis/final-handoff.md and requirement-by-requirement analysis/completion-audit.md establish completion separately from execution/natural reachability/hardware limits. Authoritative accepted checkpoint remains round03-disassembly-accepted. Archive document contract inspected; archive-options.json and analysis/archive-document-accounting.json account for every relevant Markdown; only duplicate historical instruction copies omitted. No archive, cleanup/delete, publishing or toolkit edits performed. Prior goal turn progressed full commentary/redirect closure and verified worker waits; no blocker and no unfinished phase.
