# Current status — round 2 commentary complete

ROM classification: 100.00% (196617 / 196624 bytes classified; 7 bytes unknown).

Incomplete despite rounded percentage; seven unknown bytes. Final checkpoint pass-002-final. Full current-source readability is complete; redirect handoff analysis/handoff-r02-commentary.md. Final editorial/archive acceptance remains pending. Detailed results follow historical records below.

# Disassembly progress

Round 1 disassembly complete as a bounded pass; ROM remains incomplete.

ROM classification: 73.43% (144389 / 196624 bytes classified; 52235 bytes unknown).

Code 12476 bytes (Observed8770; Static3706). Data/image 131913 bytes. Unknown 52235 bytes, all in PRG. There are505 classified regions with structural evidence. Annotation semantics/readability are pending commentary; structural evidence is not an editorial completion claim.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG0 | 1883 | 0 | 6309 |
| PRG1 | 1063 | 372 | 6757 |
| PRG2 | 2448 | 0 | 5744 |
| PRG3 | 826 | 0 | 7366 |
| PRG4 | 1696 | 0 | 6496 |
| PRG5 | 625 | 0 | 7567 |
| PRG6 | 2604 | 79 | 5509 |
| PRG7 | 1331 | 374 | 6487 |
| CHR0–CHR15 (each) | 0 | 8192 | 0 |


Verified baseline and checkpoints pass-001 and pass-001-final rebuild byte-identically. Latest exact map maps/pass-001-final.json, export maps/current.json, internal analysis/state.json, generated source generated/pass-001-final/game.asm; report analysis/pass-001-final-verification.json. Input SHA256 remains50ab734317d23b5f07f73578215571e6eb8a9b898781abd712181f23bf578f78. Configured release binary used throughout; no toolkit edits.

Evidence: contexts-r01.json records4065 distinct accepted ROM opcode offsets across natural startup and artificial scenarios. Observed byte count8770 includes complete instruction operands. Static branch discovery added1873 bytes; direct JSR entry-contract discovery added1833. Static code reachability is unresolved separately. Trace detail capacity0 intentionally discarded details; aggregate_dropped0 in all inspected summaries. No aggregate loss reported. Untraced waiting frames do not contribute execution evidence. A bridge process ended; targeted replay recovered only affected evidence, preserving earlier saved roots.

Gameplay/scenarios: natural title and top-five display observed; all five galleries, bonus slots, mode menu and title-return initializer visited artificially. Actual R99 Balloon Saloon and R9 later galleries confirmed through06F2. Bills Thrills target throwing remains unobserved beyond its animated introduction. Zapper scoring cannot be exercised; manual describes a repeating score game with rounds capped99, no finite ending. Detailed recipes/statuses analysis/scenarios.json. Earlier0625 round hypothesis rejected and corrected, not promoted as successful progression.

Lead outcomes: R01-L002 and L003 verified; L001 dispatcher/table extents and L004 remaining PRG structures ready for redirect. Most remaining bytes need bounded consumer/format investigation rather than more blind play. No terminal exhaustion claim: this is an ordinary first-pass handoff with useful ready leads.

Toolkit: one documented missing feature NES-001 (Zapper), no confirmed/suspected bugs. See analysis/toolkit-issues.md. Does not block structural work. Disk checked at handoff: about6.6GiB free; scratch under0.3MiB; cited evidence/snapshots retained.

## Coordinator phase continuation

Current round: 1. Current phase: commentary, worker `/root/round01_commentary` confirmed running. Input checkpoint: pass-001-final, compact handoff analysis/handoff-r01-disassembly.md. Prior goal turn made progress by completing the disassembly pass and launching separate commentary; this continuation verifies the same live worker and resumes without another writer. Commentary scope is all currently classified source; interim exact coverage/remaining work is recorded in commentary/progress.md. No commentary completion or redirect transition is claimed yet. Leads R01-L001/R01-L004 remain queued for redirect after the readability gate. Output checkpoint and fresh commentary verification are pending.

## Round 1 commentary completed

Current-source readability completed across all 12,476 classified code bytes, accepted data/CHR and 215 explained unknown intervals. Full 720-entry inventory reviewed; 101 functional reading scopes and consistent shared/scene-specific symbols added. Maintained source commentary/source/game.asm, refreshed commentary/reading-after.asm, exact coverage commentary/coverage-r01.json. Fresh byte-identical verification commentary/verification.json (source SHA256 5f6f3b59d74116f019b9c3254548026087beaf07ba7458c591f868b39fbf5e7a). Maps/internal state and input remain unchanged. Whole-ROM classification remains 73.43%; final editorial/archive acceptance is pending. Resume workflow at redirect using analysis/handoff-r01-commentary.md and commentary/uncertainties.md; no readability backlog deliberately deferred. No new toolkit issues; existing NES-001 remains one missing feature with no structural blocker.

## Round 1 redirect active

Current phase: redirect, attempt analysis/redirect/r01-20261113-a/. Input pass-001-final and completed current-source commentary. Investigating dispatchers and connected handlers, graphics/streams, audio and progression contracts. Research records only; no terminal claim.

## Round 1 redirect completed — ready proposal batch

Current phase: redirect handoff complete, awaiting independent disassembly review. Attempt analysis/redirect/r01-20261113-a/; handoff analysis/handoff-r01-redirect.md. Connected/independent dispatcher, handler, graphic/animation/VRAM/audio and progression avenues investigated coherently. Proposed52228of52235unknown bytes (12077code,40151data), zero accepted map changes; seven isolated00/60bytes remain unproposed with primary review. Actual classification remains144389/196624,73.43%; no terminal exhaustion claim. Exact215-range ledger and candidate instructions/data carry boundaries/banks/evidence. Bills Thrills throwing/recovery now visited artificially with1576actual contextual opcode roots and no aggregate loss. Next commentary must correct reel wrap to36entries and review all newly accepted source; final editorial audit still pending. Input/map/state/source fingerprints unchanged; existing NES-001 only, no new toolkit defect or edit. Disk6.6GiBfree; scratch208KiB; selected research/snapshots about32MiB.

## Round 2 disassembly — active independent review

Current phase: disassembly, round2, input checkpoint pass-001-final. Input map/state SHA256 db252197fe7e5c3fb4e479eaee7888427f006a35a798a9899b20f09f1a665cda. Reviewing final explicit proposals and primary bank/stack/data evidence; no acceptance from diagnostic traversal or green overlap audit. Accepted map unchanged pending reviewed batches.

Round2 intermediate review: all5202 proposed starts independently redecoded under5337 explicit contexts; exact bytes/banks agree.471sprite records,87palette records, count-prefixed VRAM records and FF-bounded animation prefixes satisfy their declared count/termination constraints. Rejected default scene5 grid as proof of28exclusive4x4 tile-block roles: D765..D77A replaces index5 with11..18; retain independent generic movement-pattern/metasprite-pointer evidence. E816/E837 description corrected to triangle-channel paths. Review evidence analysis/disassembly/r02-review/review-notes.md and independent-checks.json. No mutations yet.

Round2 code batch saved/exported/validated:211new spans,12077new code bytes; classification 156466/196624 (79.58%),40158unknown. Code acceptance results: analysis/disassembly/r02-review/code-acceptance-results.json. Fresh pass-002-code checkpoint verification in progress before data batch.

## Round 2 disassembly — completed independent review

Current phase: disassembly complete, round2; handoff to commentary pending. Final checkpoint pass-002-final. ROM classification: 100.00% (196617 / 196624 bytes classified; 7 bytes unknown).

**Incomplete:**100.00% is rounded from99.9964399056%; seven bytes remain materially ambiguous.24553code bytes (Observed9412, Static15141);172064data/image bytes;944classified regions. Gain52228bytes:12077code and40151data. New observed code642bytes/271instruction starts uses saved artificial opcode fetch contexts, not natural reachability. Remaining new code11435bytes is Static. No old classifications were removed.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG0 | 3200 | 4992 | 0 |
| PRG1 | 1551 | 6640 | 1 |
| PRG2 | 5225 | 2963 | 4 |
| PRG3 | 2083 | 6108 | 1 |
| PRG4 | 3534 | 4658 | 0 |
| PRG5 | 3329 | 4862 | 1 |
| PRG6 | 2898 | 5294 | 0 |
| PRG7 | 2733 | 5459 | 0 |
| CHR0–CHR15 (each) | 0 | 8192 | 0 |

Input pass-001-final map/state db252197fe7e5c3fb4e479eaee7888427f006a35a798a9899b20f09f1a665cda. Final map/state SHA256 0adf8a365f40d54161c7e483866e2aac98e110b59154e3e64da2ac334243d6e8. Baseline and all historical checkpoints preserved. Fresh pass-002-code, pass-002-data and pass-002-final checkpoints validate and rebuild byte-identically; final report analysis/pass-002-final-verification.json and source generated/pass-002-final/game.asm. ROM SHA256 unchanged50ab734317d23b5f07f73578215571e6eb8a9b898781abd712181f23bf578f78. Configured release binary only; no toolkit edits.

Independent review: final5202instructions redecoded in5337bank views, exact bytes and physical identities checked. Inline table barriers, per-scene effective bank pairs, C52C return-pop contract, returning-callee continuations and finite RAM/APU indirect-store contracts were inspected; unbarriered discovery is excluded. Count/stride/terminator checks cover471metasprite proposals,87palette records, VRAM records and paired animation streams. Review notes, primary decodes, exact acceptance membership, annotation results and counts: analysis/disassembly/r02-review/. These are local internal evidence.

Narrowed claims: scene5 always replaces default grid5 with11..18, invalidating consumed4x4-role proof for28exclusive blocks, including119bytes overlapping a valid metasprite pointer table. Retained generic signed movement/frame patterns and independent pointers; no bytes accepted solely on the false tile interpretation. Generic roles also retain uncertain parameter, pointer-suffix, phase-pattern and audio-container grouping. E816/E837 are triangle-channel paths (4008/400A/400B), correcting redirect's DPCM wording. Ten isolated coherent routines accepted using field/helper/control/boundary contracts, not legal decoding alone. Scene9 unusual FF execution is preserved separately. No ending claim from B147.

Lead outcomes: R01-L001 verified structural code/dispatch batch; R01-L004 verified data with narrowed specific roles; L002/L003 successful routes reused without replay; L005 retains seven unknown bytes physical2A21,403A,41AC,42E8,4999,60CE,A1B2. Primary address-word occurrences at2A21/4999/A1B2 do not establish a valid consumed object or caller; loneRTS/BRK does not discriminate code/data. The residual and competing-role reasoning is preserved in seven-byte-primary-review.json and review-notes.md. No new useful direct gameplay test identified: all galleries/high rounds and Bill throwing already visited, target hits unavailable under NES-001; finite ending unestablished. Saved traces/snapshots reused, no unchanged startup replay.

Current maintained commentary/source/game.asm is preserved unchanged (SHA2565f6f3b59d74116f019b9c3254548026087beaf07ba7458c591f868b39fbf5e7a). Prior readability was complete for pass001 only; full current-source commentary and final editorial audit remain pending after this gain. Next commentary must cover all resulting source, correct reel36versus24 and triangle-channel wording, and retain current bank/reachability distinctions and preservation credit.

Toolkit report analysis/toolkit-issues.md remains one missing feature NES-001 (Zapper), zero confirmed/suspected bugs; no structural blocker or new issues. Failed local request-schema attempts were corrected before persistence and are not toolkit defects. Disk6.6GiBfree; shared scratch208KiB; no cleanup, archive, deletion or publication. Compact handoff analysis/handoff-r02-disassembly.md.

## Round 2 commentary active

Current phase: commentary; input pass-002-final, map/state 0adf8a365f40d54161c7e483866e2aac98e110b59154e3e64da2ac334243d6e8. Entire round1 commentary preserved at analysis/round-2/commentary before fresh preparation. Carry-forward review compares physical instruction ranges and dependencies; no stale assembly overwrite. Full current-source readability, verification and reading refresh pending.

## Round 2 commentary complete

Current phase: commentary handoff complete; ready for redirect. Full source readability covers 24,553 code bytes/10,755 instructions, 172,064 data/image bytes and seven explained unknown singleton bytes. All 951 scaffold entries and 298 functional scopes reviewed; 505 unchanged classified entries retain prior credit. Entire prior commentary preserved at analysis/round-2/commentary before fresh preparation. New names/comments include all recovered routines/data plus dependent old corrections; no classified readability gaps deferred.

Fresh byte-identical verification commentary/verification.json passed; source SHA256 a866725b5d57f4cdd4a0ea3d1397d8d1bd35e29ff604cfa8b67c578f41af6c48. Reading copy refreshed. ROM/map/state unchanged. Final editorial audit pending, not invoked. Seven unknown bytes retain independent uncertainty; no new discriminating structural proposal or toolkit issue. Handoff analysis/handoff-r02-commentary.md; exact coverage commentary/coverage-r02.json and residual questions commentary/uncertainties.md. No archive/deletion/publication/toolkit changes.

## Round 2 redirect active checkpoint

Input pass-002-final, map/state SHA2560adf8a365f40d54161c7e483866e2aac98e110b59154e3e64da2ac334243d6e8,196617classified/7unknown. Unique attempt analysis/redirect/r02-20261113-a. Primary reinspection found materially changed inference for physical02A21: runtime count-zero underflow rules out a safe empty metasprite, not generic data. Aligned scene3 pointer index35 targets it; preceding count28 record ends exactly there, next count3 records start2A22/2A2F and close2A3C. Whole local container parses39storage records with legal sprite attribute fields. Proposed one generic Static data byte; no safe-zero/semantic sentinel/natural-use claim. Recipe/primary evidence/preservation hashes and exact seven-byte complement saved in review.py, sprite-container-checks.json, unknown-ledger.json and findings.md. Other six residuals reconsidered separately; research record/rejection review/lead packaging underway. All identified independent avenues assessed, no new gameplay replay indicated. No map/state/source/ROM/toolkit mutations. Resumable same phase until handoff complete.

## Round 2 redirect complete: ready batch

Unique attempt analysis/redirect/r02-20261113-a finished coherent connected and independent triage. R02-L006ready proposes1generic Static data byte at02A21; accepted0this phase, exact current complement still7bytes. R01-L005history preserves earlier seven-byte rejections and records corrected split; six others remain ambiguous. Exact primary pointer/record arithmetic and strongest BRK alternative, six-byte localized rejection review, source extracts, ledger and reproducible read-only recipe preserved. analysis/handoff-r02-redirect.md is ready for independent disassembly. This is a ready-batch handoff, not terminal exhaustion. Source10463 stale “gaps unresolved” wording noted for final editorial audit. Preservation hashes checked; ROM/map/state/source/configured binary unchanged. Existing NES-001only, no new toolkit issue. No gameplay replay/archive/deletion/publication/toolkit edit; disk6.5GiBfree/scratch208KiB.

## Round 3 disassembly active

Current phase: disassembly, round3; input pass-002-final, map/state SHA2560adf8a365f40d54161c7e483866e2aac98e110b59154e3e64da2ac334243d6e8. Independent R02-L006 pointer/consumer/storage-boundary review underway; existing source preserved.

## Round 3 disassembly complete

Current phase: disassembly complete, round3; input pass-002-final; authoritative final checkpoint pass-003-accepted. ROM classification: 100.00% (196618 / 196624 bytes classified; 6 bytes unknown).

Incomplete: exact percentage 99.99694849051998%; six unknown bytes remain despite rounded100.00%. Gain1generic Data/Static byte02A21, code24553/data-image172065,945regions. Independent combined pointer/consumer/storage-grammar review accepts only generic storage, not safe empty metasprite or natural selection. Exact checks/reasoning: analysis/disassembly/r03-review/. Six residual physical offsets403A,41AC,42E8,4999,60CE,A1B2 retain localized ambiguity; no unchanged runtime replay.

| Bank | Classified | Unknown |
|---|---:|---:|
| header | 16 | 0 |
| PRG0 | 8192 | 0 |
| PRG1 | 8192 | 0 |
| PRG2 | 8188 | 4 |
| PRG3 | 8191 | 1 |
| PRG4 | 8192 | 0 |
| PRG5 | 8191 | 1 |
| PRG6 | 8192 | 0 |
| PRG7 | 8192 | 0 |
| CHR0 | 8192 | 0 |
| CHR1 | 8192 | 0 |
| CHR2 | 8192 | 0 |
| CHR3 | 8192 | 0 |
| CHR4 | 8192 | 0 |
| CHR5 | 8192 | 0 |
| CHR6 | 8192 | 0 |
| CHR7 | 8192 | 0 |
| CHR8 | 8192 | 0 |
| CHR9 | 8192 | 0 |
| CHR10 | 8192 | 0 |
| CHR11 | 8192 | 0 |
| CHR12 | 8192 | 0 |
| CHR13 | 8192 | 0 |
| CHR14 | 8192 | 0 |
| CHR15 | 8192 | 0 |

Map/state SHA256f7c49fce4ad15bbe7ad6b7d51354e5cf3aeb46760784f9902c148b9f198eaab1. Validation and fresh byte-identical rebuild passed: analysis/pass-003-accepted-verification.json. Preacceptance pass-003-final checkpoint preserved separately; it is not authoritative final R03. ROM unchanged; current maintained source remains SHAa866725b5d57f4cdd4a0ea3d1397d8d1bd35e29ff604cfa8b67c578f41af6c48. Next commentary incorporates one datum, preserves full prior work, and corrects stale Bank4AuxiliaryMotionStep gaps wording at10463. Full current-source readability and final editorial acceptance remain separate gates. Lead histories updated; compact handoff analysis/handoff-r03-disassembly.md.

Representative progression/scenarios reused: galleries/high rounds/Bill throwing covered; optical hits unavailable NES-001; no supported finite ending. Toolkit report analysis/toolkit-issues.md unchanged:1missing feature,0confirmed/suspected bugs, no new issue or structural blocker. Configured release binary only, no toolkit change/archive/deletion/publication. Disk6.5GiBfree; scratch208KiB.

## Round 3 commentary complete; redirect active

Current phase: redirect, round3, unique attempt r03-20261113-a. Input pass-003-accepted; map/state SHA256 f7c49fce4ad15bbe7ad6b7d51354e5cf3aeb46760784f9902c148b9f198eaab1. Commentary handoff completes all951entries/298scopes,950exact carries plus02A21; fresh byte-identical verify, source SHA256 f36f832dc1957a94513eae1f93f514df8c5940f136c0364f69823323b06fc468. Editorial gate pending. Redirect assessing changed generic-data reasoning and all six localized residuals against current source/primary artifacts, retaining prior immutable evidence and avoiding unchanged replay. Exact residuals403A,41AC,42E8,4999,60CE,A1B2. Research records only; no ROM/map/state/source/toolkit changes.

## Round 3 redirect complete: terminal-incomplete candidate

Current phase: redirect complete, round3, r03-20261113-a; unchanged pass-003-accepted/map-state f7c49fce4ad15bbe7ad6b7d51354e5cf3aeb46760784f9902c148b9f198eaab1. Current-source readability complete, final editorial pending. Exact complement independently recomputed:403A,41AC,42E8,4999,60CE,A1B2, each1byte. Classified196618/196624; proposed0/accepted0this phase; ready leads none. Changed02A21 generic-storage correction independently reassessed against all remaining candidates; no shared discriminating grammar/entry contract. Source, control/consumer/reference, bounds and isolated routine alternatives reviewed with scoped negatives; no useful revised inference/test survives. Compact localized rejection review and primary artifacts: analysis/redirect/r03-20261113-a/. All identified connected/independent avenues and progression limits addressed; no unchanged runtime replay. R01-L005 stable history and scenarios review appended. Handoff analysis/handoff-r03-redirect.md awaits coordinator terminal assessment. ROM/map/state/source/configured binary hashes preserved; no new toolkit issues (existing NES-001), no toolkit edits/archive/deletion/publication. Disk6.5GiBfree/scratch208KiB.


## Final editorial acceptance complete

Targeted acceptance reused all 951 reviewed entries/298 functional scopes and checked current dependency comments, mechanical routine/branch/data naming, symbols, unknown notes and 2604 inventory source links. Removed six misplaced scene-3 element-35 caveat copies beside unrelated pointer batches; owning table, generic byte and emitter retain the caveat. Blank lines preserve source locations; no instruction/data/physical boundary changed. Prior copies remain in analysis/final-editorial-before. Details: commentary/editorial-audit.md.

Supported final audit passed: 951 reviewed regions, 944 contained unit/fragment records representing 298 functional scopes, editorial gate true, fresh byte-identical rebuild and refreshed reading-after.asm. Source SHA256 241a4cbad07160804758c2e1fdfa57af79cfa550dc90ab6289456d7a3617cd1e; results analysis/final-editorial-audit.json. Frozen baseline/before unchanged; ROM/map/state/toolkit unchanged. Classification remains terminal incomplete, 196618/196624, six unknown bytes. No finite-ending, natural-use or full compatibility claim; no archive, cleanup or publication. Final orchestration/document manifest validation remains coordinator work.

## Coordinator terminal handoff

All sequential phases and final editorial acceptance have finished. Coordinator inspected the localized rejection review and decisive primary evidence and accepted justified terminal incomplete; no useful revised method/ready lead remains. Current source/editorial audit and byte-identical verification pass, with fresh store/map validation and exact six-byte complement. No active or externally blocked work is reported as success. Full terminal record: analysis/terminal-handoff.md. ROM classification remains incomplete: ROM classification: 100.00% (196618 / 196624 bytes classified; 6 bytes unknown). Checkpoint pass-003-accepted, source hash241a4cbad07160804758c2e1fdfa57af79cfa550dc90ab6289456d7a3617cd1e. Archive document manifest and completion audit are coordinator final checks; no archive, cleanup or publication is authorized or executed.

## Final coordinator checks passed

Fresh configured release store/map validation, exact ROM/map/state/checkpoint/binary identity and complement checks passed. Final editorial audit matches current source and refreshed reading copy. archive-options.json now selects37additional final/historical Markdown documents, omitted_documents is empty because no disposable scratch Markdown exists, and every contract-unrecognized Markdown is accounted for. All final handoffs/rejection reviews/conclusions are selected. analysis/coordinator/completion-audit.json records13proven requirement groups and no required work remaining. All phase/final-acceptance workers are completed. Workflow orchestration achieved its documented terminal-incomplete handoff; whole-ROM classification remains incomplete with6unknown bytes. No archive execution, cleanup, deletion, toolkit edit or publication occurred. Final handoff analysis/terminal-handoff.md is current.
