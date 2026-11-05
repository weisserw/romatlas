# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 — disassembly active

Input identity verified by run.py inspect; baseline established and byte-identical
assembly verified, see [baseline-verification.json](baseline-verification.json).
Baseline raw emission is not classification. Mapper 2, 131088 physical bytes,
128 KiB PRG and CHR RAM. Title capture identifies Amagon (1988 American Sammy).
Persistent phase evidence: [round1-disassembly/rpc.jsonl](round1-disassembly/rpc.jsonl).
First exploratory static roots used wrong physical offsets and produced no
candidates; no incorrect annotations were committed. Corrected fixed-bank mapping
is file offset CPU address + 65552. Bounded natural title/game input ongoing;
classification and per-bank output counts pending. No ready input leads.

### Substantive batch checkpoint

Natural Start press/release (replacement button arrays) reached Zone 1-1 map and
actual gameplay; firstgame.ppm and firstgame.snapshot preserve gameplay frame 997.
ROM analysis identifies $22 as scene index, $35 main state, $b5 demo playback,
and state 13 handler at CPU $c230 initializes scene-dependent assets and state 0.
Bounded shortcut testing now patches scene 2/state13 from the clean gameplay snapshot.
No arbitrary RAM brute force. See fixed-decode.json for static consumer evidence.
Static batch A contains 1932 instruction/context candidates from observed roots
and contiguous ROM interface JMP slots. Annotation review in progress; not yet counted.

### Batch A verified

Internal map saved and exported; map.validate passed. Checkpoint
[round1-disassembly-a-verification.json](round1-disassembly-a-verification.json)
confirms byte-identical rebuild. 420 regions: 8906 code, 39600 image and 2978 data
bytes (51484 classified / 131088; 79604 unknown). Structural image proofs use
five-byte descriptors and tile-count upload loops, not raw-export fallback.
Two static candidates conflicting with observed instruction boundaries were skipped
and recorded in candidate-overlaps.json. No input leads to verify.
Later action-tested scenes were successfully displayed (not just RAM writes):
2 jungle, 4 river, 6 rain forest, 8 mountain, 10/11 beach, 12 final boss arena.
Files named ending.* actually capture final boss arena; final ending shortcut is
separately named ending-shortcut.*. Final boss substate $92=3 enters the observed
post-defeat handler; $01a0=0 and $01f0=1 accelerate player-conversion/defeat animation,
with consumer contract in finalboss-update-decode.json. Runtime verification ongoing.

## Round 1 — disassembly final handoff

ROM classification: 50.20% (65807 / 131088 bytes classified; 65281 bytes unknown).

Incomplete: 65281 bytes remain structurally unknown. Classified Code 10873,
Image 52000, Data 2934; 1359 regions all carry evidence.
Header16 bytes classified separately. This is one completed easy-gain disassembly
pass, not terminal investigation exhaustion.

| Physical PRG8K bank | Code | Image | Data | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 0 | 0 |
| 1 | 0 | 8176 | 0 | 16 |
| 2 | 0 | 8192 | 0 | 0 |
| 3 | 0 | 8000 | 0 | 192 |
| 4 | 0 | 0 | 0 | 8192 |
| 5 | 0 | 1616 | 35 | 6541 |
| 6 | 0 | 8192 | 0 | 0 |
| 7 | 0 | 8192 | 0 | 0 |
| 8 | 1343 | 0 | 0 | 6849 |
| 9 | 0 | 1440 | 181 | 6571 |
| 10 | 601 | 0 | 1968 | 5623 |
| 11 | 644 | 0 | 157 | 7391 |
| 12 | 9 | 0 | 0 | 8183 |
| 13 | 589 | 0 | 102 | 7501 |
| 14 | 4622 | 0 | 52 | 3518 |
| 15 | 3065 | 0 | 423 | 4704 |

Verified baseline and final checkpoints are byte-identical. Final map.validate and
portable map validation passed. Final checkpoint: maps/round1-disassembly-final.json,
generated/round1-disassembly-final/game.asm and
analysis/round1-disassembly-final-verification.json. Input SHA256 unchanged.
Internal state analysis/state.json; current map maps/current.json. Counts derive
from unique physical classified bytes, excluding unknown export fallback.
Execution separately: 5556 unique opcode starts,6482 contextual roots,7100 contextual
edges, aggregate_dropped0. Detail capacity0 deliberately retains no events;
11562194 dropped detail events are not aggregate loss. Runtime trace summary and
context captures preserve physical offset, CPU alias, effective bank and artificial
status. Natural reachability is claimed only for title/start/early play.

Broad progression used supported scene/state RAM contracts with replayable snapshots
and action inputs through later areas, final boss, rescue boat and terminal ending.
Megagon form was successfully displayed and Up+B exercised without claiming a laser
at zero score. Exact recipes and deferred same-zone scenes: analysis/scenarios.json.

Static discovery and upload consumer/descriptor evidence yielded connected code and
52KB of planar image sources; generic uniform runs were annotated only after
checking neighboring patterns and conflicting code. Seven unselected descriptors
were accepted on bounded pool layout, consumer semantics and tile-source consistency,
without inventing natural selection. Two conflicting static decodes were skipped.
No original caller/layout or dead-code claims were inferred.

Ordinary waits/input variation stopped after representative systems and ending
were visited; further scene permutations have low first-pass value. Residual bank4
records, state/boss pointer tables and asset gaps need connected consumer/boundary
inspection in redirect, not additional long gameplay. Concrete leads are ready in
analysis/leads.json. Readability/editorial review remains commentary work; no full
readability completion claim.

Toolkit findings: one confirmed documentation/API mismatch NES-001, zero suspected
bugs, zero missing-feature entries; no blockers. Workaround uses buttons arrays.
See analysis/toolkit-issues.md; no toolkit changes made. Disk checked at handoff:
8.9GiB available, scratch196KiB, phase evidence about133MiB before final artifacts.
No evidence/archive cleanup or publication performed.

## Round 1 — commentary active

Input maps/round1-disassembly-final.json, analysis/state.json, maps/current.json;
prepared immutable commentary baseline and independent source. All classified
instructions have an initial complete mechanical group/naming pass (260 groups,
941 symbols). First edited-source verification passed byte-identical. Remaining
source consistency/format checks and final verification are tracked in
commentary/progress.md. Structural classifications unchanged; final audit pending.

### Round 1 commentary complete; next phase redirect

Output: commentary/source/game.asm + symbols.inc, reading-after.asm and review.json.
All classified code/data/image source and unknown-slice notes reviewed; 1,552
records, 265 routine/structure units. Final byte-identical verification passed and
reading copy refreshed; see commentary/verification.json. Current map equals frozen
final disassembly checkpoint; ROM unchanged. Classification 50.20%, full-ROM
classification incomplete. Final editorial acceptance remains pending.

Priority structural concern: byte-reversed NMI premise ($FED8 versus actual $D8FE)
affects justification for two static ranges totaling 99 bytes. No classification
changes made. Exact evidence, scope and independent-provenance check are in
round1-commentary/vector-root-review.json and round1-commentary-handoff.md.
No pending current-source readability ranges; remaining work is redirect structural
investigation plus eventual final editorial acceptance. NES-001 remains nonblocking;
no new toolkit issue, toolkit edit, cleanup, archive or publication.

## Round1 redirect active

Research attempt redirect/r1-20261105-structural resumes completed commentary checkpoint. Priority false NMI root localized review plus scene assets, state dispatch and residual graphics avenues. Map/state/source unchanged; exact unknown complement saved in attempt unknown-ledger.json.

### Round1 redirect complete; ready combined review batch

Research handoff: [round1-redirect-handoff.md](round1-redirect-handoff.md). Accepted checkpoint unchanged:65807classified/65281unknown. Sevenready stableleads propose65278newunique bytes (8236Code,56428Data,614Text), plus99existingCode->Data correction. Projected residual3isolatedRTSbytes is conditional on independent acceptance, not current classification. Exact current complement and scoped residual review in redirect/r1-20261105-structural; results.json verifies zero overlap/conflicts and immutable map/state/source/ROM hashes. No toolkit edits, runtime replay, cleanup, archive, publication or finaleditorial audit. Next: independent disassembly review of combinedproposal, then commentary/redirect.

## Round2 disassembly active

Input: maps/round1-disassembly-final.json (SHA256 f26ec6e4448f5f469d5adbbf78e36d04572fbbdb1f6bb5ac8c0d39675ac8edd2), 65807 classified,65281 unknown. Consuming seven R1 ready leads from combined-proposal-final.json. Independent review checklist: exact complement/partition; final instruction/bank/control-flow audit; consumer/extent and generic-pattern review; four static local calling contracts; independent false-NMI payload correction; retained isolated RTS ambiguity. No new gameplay replay on unchanged premises. Output checkpoint pending.

## Round2 disassembly complete — incomplete classification; next commentary

Input checkpoint: maps/round1-disassembly-final.json SHA256 f26ec6e4448f5f469d5adbbf78e36d04572fbbdb1f6bb5ac8c0d39675ac8edd2. Consumed IDs: R1-scene-assets, R1-state-dispatch, R1-graphics-gaps, R1-fixed-tail-provenance, R1-audio-contracts, R1-static-local-contracts, R1-residual-structures. All seven verified after independent review, with historical claims/corrections preserved. Output: maps/round2-disassembly-final.json = maps/current.json = analysis/state.json SHA256 317037a30049ed1ac66e7a9a945560cbf7eb104f092ab7f812c91b755588ba73.

ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).

Unique gain65278:8236Code,56428Data,614Text. Separately corrected99existingCode bytes to Data using independent duplicate-payload/metadata evidence and corrected CpuVectors NMI evidence to$d8fe. Type totals: {'Data': 59461, 'Image': 52000, 'Code': 19010, 'Text': 614}. Header16 classified. Exact per-bank physical accounting:

| Physical8KiB bank | Classified | Unknown |
| --- | ---: | ---: |
| 0 | 8192 | 0 |
| 1 | 8192 | 0 |
| 2 | 8192 | 0 |
| 3 | 8192 | 0 |
| 4 | 8192 | 0 |
| 5 | 8192 | 0 |
| 6 | 8192 | 0 |
| 7 | 8192 | 0 |
| 8 | 8192 | 0 |
| 9 | 8192 | 0 |
| 10 | 8192 | 0 |
| 11 | 8192 | 0 |
| 12 | 8192 | 0 |
| 13 | 8192 | 0 |
| 14 | 8190 | 2 |
| 15 | 8191 | 1 |

Unknown complement:117608/$cb58,122120/$dcf8,127720/$f2d8, one byte$60 each; fixed effective banks14/15. Return-only routine and non-code both fit; no independent entry/format. See round2-disassembly/unknown-ranges.json and redirect/r1-20261105-structural/isolated-rts-review.json. Rounded100.00% does not establish whole-ROM completion.

Fresh validation, generation and whole-file byte-identical verification passed: round2-disassembly-final-verification.json; source generated/round2-disassembly-final. Internal annotation batches, fresh3913instruction decode, zero resolved successor exceptions, raw vectors/duplicate/event/sound/Text checks and review decisions: round2-disassembly/review.md, mechanical-review.json, data-claim-review.json, annotations.jsonl and batches.json. New provenanceStatic; no new fetch/natural-use claim.

Reused representative later-area/boss/boat/ending/Megagon scenarios; no unchanged replay or new patches. Current commentary/source and historical checkpoints/evidence preserved; current-source readability work for new bytes and final editorial audit remain pending. Next commentary then redirect; no terminal workflow claim. Existing NES-001 documentation/API mismatch remains nonblocking; no new toolkit issues, no toolkit edits. See [toolkit-issues.md](toolkit-issues.md). No archive/publication/deletion.

Round2 handoff storage check:8.8GiB available; durable phase evidence 15.2MiB. No new exploratory scratch/captures; existing scratch preserved.

## Round2 commentary active

Fresh verified baseline prepared after preserving entire prior commentary under
analysis/round-2/. Carried 5196 unchanged instructions by physical identity and width,
plus unchanged data prose/fields; corrected false-root Code excluded. New source
frontier:3913 instructions in146 fragments (analysis/round2-commentary/new-fragments.json),
plus new Data/Text structures and affected old dependencies. Current-source readability
not complete; same commentary phase must resume until source, inventory, verification
and reading copy complete. commentary/progress.md records exact active scope.

## Round2 commentary completed current-source readability

No deferred readability ranges. All5453current scaffold records and814meaningful units
reviewed, including all3913new instruction starts, new Data/Text and corrected99-byte
false-vector Data. Fresh actual edited-source verification PASSED byte-identical;
reading-after refreshed. Source game.asm SHAf02e1c3acf6c4a6221e02f14b512c246ab21622970bc91fce674bdb71327efb3.
Map/state/baseline unchanged SHA317037a30049ed1ac66e7a9a945560cbf7eb104f092ab7f812c91b755588ba73;
ROM unchanged. Classification remains incomplete131085/131088 despite rounding100.00%.
Final editorial audit pending. Exact narrow redirect evidence correction: Data region5333
physical88655..88767 has false stride7 prose; consumer has stride8/copy7, Data bounds
unchanged. See commentary/uncertainties.md and analysis/round2-commentary-handoff.md.
No terminal claim or next-phase work; no toolkit changes, archive/publish/cleanup/deletion.

Round2 commentary same-phase gate repair: corrected inventory paths relative to
commentary/source and populated per-region compact-unit intersections. Strict cached
validation passed13517actual source locations,5453scaffold identities/statuses,
814compact units and7250embedded intersections; every Code region has units.
Fresh verify/read refreshed with unchanged source bytes/hashes. Read-only helper
status check pending confirmation; final editorial audit intentionally stays pending.

Round2 commentary repaired gate CONFIRMED: archive.editorial_status read-only inspection
finds5453/5453regions and7250/7250embedded unit intersections complete; sole remaining
reason is the deliberately pending final editorial audit. Fresh actual-source verify
and reading refresh passed. Handoff/scope proof updated; source/map/state/ROM hashes
unchanged. Current-source commentary gate now passes; redirect still follows.

## Round2 redirect active: bounded stride correction

Attempt analysis/redirect/r2-20261105-stride-review preserves checkpoint SHA317037a30049ed1ac66e7a9a945560cbf7eb104f092ab7f812c91b755588ba73. Raw consumer confirms region5333 stride8/copy7, not stride7. Annotation-only proposal saved in consumer-correction.json; exact three-byte Unknown complement/neighbors saved in unknown-ledger.json. No map/state/source/ROM edit. Reviewing source opportunities and retained primary rejection reasoning before combined handoff.

Round2 redirect completed research handoff: one ready annotation-only lead R2-display-quantization-stride-correction;0newclassificationbytes. Rawconsumer/recordproof, fresh exact complement and compact primary rejection/source-opportunity review saved under analysis/redirect/r2-20261105-stride-review. All3isolatedRTS bytes remain Unknown; no further promising actionable classification avenue identified. Next disassembly correct known false region5333 evidence, then commentary; final editorial audit still pending. No terminal-success claim. ROM/map/state/source hashes unchanged at handoff;8.8GiB free, scratch212KiB. See analysis/round2-redirect-handoff.md.

## Round3 disassembly active

Input checkpoint317037a30049ed1ac66e7a9a945560cbf7eb104f092ab7f812c91b755588ba73. Independently matched immutable physical88635..88655 to consumer bytes and all fourteen8byte rows. DEY/TYA/ASLx3/TAY computes8*(incomingY-1)mod256; loop copies7. Narrow annotation-only correction pending. First annotation request used lowercase enum and was rejected transactionally; retry uses adapter's explicit Data/Static spellings. No state change from rejection. Existing residual review reused; no replay/new classification.

## Round3 disassembly completed: annotation evidence correction

ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).

Incomplete despite rounded100.00%. Exactly one pass;0newbytes,0observed additions. Independently reviewed consumer physical88635..88655 with effective slots[10,11,14,15]: DEY/TYA/ASLx3/TAY computes8*(incomingY-1)mod256; CPX7 loop copies7. Existing112byte Data region5333 holds fourteen8byte storage records. Corrected only comment/evidence via supported map.annotate; kind/bounds/provenance/confidence/labels/views and all other regions preserved. Historical stride7 snapshots retained. No natural selector-domain/arbitraryY-safety/skipped-byte-role assertion. Lead R2-display-quantization-stride-correction verified.

Per physical8KiB bank counts(classified/unknown):

|Bank|Classified|Unknown|
|---|---:|---:|
|0|8192|0|
|1|8192|0|
|2|8192|0|
|3|8192|0|
|4|8192|0|
|5|8192|0|
|6|8192|0|
|7|8192|0|
|8|8192|0|
|9|8192|0|
|10|8192|0|
|11|8192|0|
|12|8192|0|
|13|8192|0|
|14|8190|2|
|15|8191|1|

Header16/16 classified. Type counts: Data59461,Image52000,Code19010,Text614. Three isolated$60 offsets117608/$cb58,122120/$dcf8,127720/$f2d8 remain Unknown in fixed banks14/15; independent entry/data role still ambiguous. Reused primary coordinator residual-rejection-review.md on unchanged premises: no universal caller prerequisite, no forced-execution or unchanged-search replay. Broad later-area/boss/boat/ending/Megagon scenarios reused; no new useful gameplay hypothesis.

map.validate and fresh round3-disassembly-final generation/whole-file verification passed byte-identical. State=current=checkpoint SHA0ca03ff1a520d52325c68310613d72f5bc5fc9f74812c7716b7e0a98a696b960. ROM unchanged; commentary/source SHA f02e1c3acf6c4a6221e02f14b512c246ab21622970bc91fce674bdb71327efb3 unchanged; entire commentary untouched. Evidence: round3-disassembly/independent-review.json,rpc.jsonl,classification.json; verification round3-disassembly-final-verification.json.

Next commentary correct inherited stride wording/incomingY-minus-one shorthand, preserving payload; then redirect/final audit. This is necessary evidence repair, not terminal success. Existing [NES-001](toolkit-issues.md) documentation/API mismatch remains nonblocking; no new toolkit issues found, no toolkit edits.8.8GiB available,scratch212KiB; no cleanup/deletion/archive/publication.

## Round3 commentary complete

Preserved the entire previous commentary under analysis/round-3/commentary before
supported preparation of the current frozen baseline. Independent map comparison
found only region5333 comment/evidence changed; generated before/game.asm differs
only in its annotation comment. Physical identities, boundaries, kinds, execution
views and dependencies are unchanged. Carried all sound names/prose/review credit;
no whole-source reread or cosmetic rewrite was needed. Four comment lines now state
fourteen eight-byte records and the exact 8*(incoming Y-1) modulo256 index. Seven
bytes are copied for each valid record and the eighth is skipped; arbitrary selector
safety, natural selector domain and skipped-byte meaning remain unclaimed.

All current-source readability is complete with no deferred ranges: 19010 Code,
59461 Data, 614 Text and 52000 Image bytes. Strict current-baseline validation passed
5453 exact scaffold entries, 814 compact units, 7250 embedded intersections and
13517 source locations relative to commentary/source. Every Code region has units.
The four-line correction preserves source line positions and all payload/directives.
Fresh actual-source verification PASSED byte-identical and reading-after.asm refreshed.
Source game.asm SHA256 fbe2f4b270523429673a6b4a72c3ea2efb82b48406e5255e9b7d3c183fa80145.
Current map/state SHA256 0ca03ff1a520d52325c68310613d72f5bc5fc9f74812c7716b7e0a98a696b960.

Classification remains incomplete: 131085/131088 bytes, exactly three Unknown despite
rounded100.00%. Their uncertainty notes and sound residual rejection reasoning remain.
No new gameplay research or unchanged replay. Final editorial audit remains pending;
no audit, redirect, archive, publication, cleanup, toolkit or map/state edit performed.
Evidence: analysis/round3-commentary/carry-equivalence.json,
location-contract-validation.json and scope-verification.json; compact handoff at
analysis/round3-commentary-handoff.md. No remaining current-source readability gaps.

## Round3 redirect complete: terminal-incomplete research

Attempt redirect/r3-20261105-residual-review confirms accepted region5333 annotation and four current-source comments resolved; R2-display-quantization-stride-correction remains verified. No new/reopened ready leads,0proposed/accepted bytes. Current map complement freshly checked:117608/$CB58,122120/$DCF8,127720/$F2D8 each one raw$60;131085/131088 classified,3Unknown despite rounded100%. Current neighbors/raw windows and protected hashes unchanged at checkpoint0ca03ff1a520d52325c68310613d72f5bc5fc9f74812c7716b7e0a98a696b960.

Sound primary rejection and prior connected/source triage reused on unchanged premises. No new independent constraint separates return-only code from non-code, and no useful revised inference/test survives compact rejection review. No universal caller prerequisite imposed; generic Data also unsupported for these isolated bytes. No unchanged replay, arbitraryPCexecution or empty classification cycle. No interrupted frontier. Durable checkpoint/results/exact ledger/rejection review in attempt directory; lead pass history appended preserving all verified IDs. Coordinator must inspect stopping reasoning. Final editorial acceptance/archive preparation pending separately; no whole-ROM/orchestration completion claim. See [round3-redirect-handoff.md](round3-redirect-handoff.md). ROM/map/state/source/binary preserved;8.8GiB free,scratch212KiB; no cleanup/deletion/archive/publish/toolkit edit.

## Final editorial acceptance

Targeted current-source final review completed using valid prior full-source credit.
Two concrete remaining inconsistencies fixed: envelope continuation comment described
pitch-slide instead of base-volume convergence; event AND mask used a field-offset
name, now distinct EventRotatedFlagsMask=$0007. Five matching review summaries corrected.
No game.asm line shifts or structural edits. Substantive report: commentary/editorial-audit.md.

Strict validation PASSED5453 exact regions,814 compact units,7250 embedded units and
13517 relative source locations; every Code region carries reviewed units. Official
commentary.py audit PASSED fresh byte-identical rebuild and reading refresh. Final
editorial gate reviewed; read-only archive.editorial_status and protected hashes are
recorded in analysis/final-editorial. Current source game.asm SHA256
ab538bf392d027a90eb36cdb69c5bd847aadde4069c1ddf71ca02c48abd7ae3f;
symbols.inc597c411b68101d02fcc60323b3fa679a253622a066775107659ca5951665de65.

Classification remains incomplete3 at131085/131088. No research reopening, map/state/
frozen-baseline/toolkit edits, archive execution, cleanup, deletion or publication.
Coordinator final options reconciliation/completion audit is still outstanding.

## Coordinator terminal handoff — justified incomplete

All authorized coordinator phases and final editorial acceptance are complete. This
is terminal-incomplete orchestration, NOT complete ROM classification.

ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).

Exact Unknown physical117608/$CB58,122120/$DCF8,127720/$F2D8 remain solitary$60;
primary-evidence rejection review supports ambiguity and no useful revised test or
ready lead remains. No interrupted productive frontier or external blocker is hidden.
Eight leads are verified; no empty further cycle is warranted on unchanged premises.

Final map/state/checkpoint/frozen baseline SHA0ca03ff1a520d52325c68310613d72f5bc5fc9f74812c7716b7e0a98a696b960.
Final actual source/editorial audit passes byte-identical verification, complete
5453regions/814compact/7250embedded units and13517valid locations; reading copy
matches source. Final source game.asm SHAab538bf392d027a90eb36cdb69c5bd847aadde4069c1ddf71ca02c48abd7ae3f,
symbols.inc SHA597c411b68101d02fcc60323b3fa679a253622a066775107659ca5951665de65.
Configured binary independently validates current store and map. ROM/raw original
baseline unchanged; no toolkit implementation change.

Final handoff analysis/terminal-handoff.md; coordinator requirement audit
analysis/coordinator/completion-audit.md; exact current verification
analysis/current-audit.json. archive-options.json/document-selection-audit.json
account for all46applicableMarkdown:27additionalfinal/history documents selected,
9byte-identical copiedworkflowtemplates omitted,0unaccounted. Final results/handoffs
and correction/rejection history retained. Omission is metadata only, not deletion.
No archive execution, cleanup, publication or commercial-ROM/asset download occurred.
All phase workers are finished; future analysis remains locally resumable if a
concrete new independent structural constraint emerges. Final handoff is ready for
separately authorized archive preparation; archive execution is not part of this goal.
