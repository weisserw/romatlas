# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 disassembly — supported findings and frontier

ROM classification: 50.20% (65807 / 131088 bytes classified; 65281 bytes unknown).

Baseline and final map/source rebuild verified; see progress.md for exact per-bank
counts and final checkpoint. Primary structural evidence: graphics-contract.json
and unselected-tile-contract.json establish bank/dest/count/source records, bounded
null-word lists and count*16 planar uploads at CPU $f9c6-$fa70. Uniform patterns
were reviewed against known code complement and neighboring bytes, classified
generic Data when specific semantics remained unresolved. Static/fetched code
keeps bank views; pointer candidates alone were not promoted to code.

Lead attempts: no inherited leads. Startup/static/vector batch verified. Scene
initializer $c230, main-state13, scene$22, demo$b5 and pause$38 contracts verified
through working terrain/action captures; broad shortcut routes are visited artificial
execution only. Finalboss substate$92=3 produced rescue boat and terminal ending.
Transformation state8/12 produced Megagon, laser effect unresolved at zero score.
No failed shortcut premise is propagated into a broader rejection.

Static candidate overlaps at physical130876/130878 conflict with accepted instruction
coverage and were skipped; this rejects only those candidate boundaries, not nearby
non-code or alternate independently supported structure. See candidate-overlaps.json.
First root-offset probes were erroneous and returned no candidates; corrected
fixed-bank formula CPU+65552 used thereafter. Early button/pressed inputs silently
released all buttons due NES-001; those observations do not reject gameplay routes.
An exploratory pagination loop at end-of-page was interrupted; recovered 1709
context roots from durable log and used bounded range(total) pagination thereafter.
No partial classification was lost; evidence retained.

Ready redirect avenues: R1-scene-assets bank4 record consumers; R1-state-dispatch
state table and thirteen-scene boss tables; R1-graphics-gaps residual image/data
complement. See leads.json for concrete recipes, exact intervals and evidence.
This is an easy-gain handoff, not terminal exhaustion; redirect should follow the
connected handlers/structures and independent concrete avenues before deciding
on another classification cycle. Unknown meaning does not block generic Data.

Primary files under round1-disassembly/: fixed-decode.json (linear inspection only,
not blanket code evidence), scene-init-decode.json, finalboss-update-decode.json,
expanded*-candidates/accepted.json, final-observed-additions.json, classification.json,
unknown-ranges.json, all milestone contexts/frames/state and durable snapshots.
RPC log keeps exact requests; selected contexts keep artificial provenance.
No toolkit edits. Toolkit report has one confirmed documentation bug, no blockers:
[toolkit-issues.md](toolkit-issues.md).

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
