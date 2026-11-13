# Disassembly progress

## pass-001 — initial checkpoint (historical)

ROM classification: 48.92% (192355 / 393232 bytes classified; 200877 bytes unknown).

Baseline established once, byte-identical: [baseline-verification.json](baseline-verification.json).
Current verified checkpoint: [pass-001-verification.json](pass-001-verification.json),
map [../maps/pass-001.json](../maps/pass-001.json), internal [state.json](state.json).
Input identity unchanged. Current source: ../generated/pass-001/game.asm.

Accepted structural evidence: 1,353 code bytes / 624 instructions in explicit
bank-31 CPU views rooted in reset/NMI/IRQ; 131,072 CHR image bytes; 45,730 generic
uniform-data bytes; 14,162 readable English text bytes; 16 header bytes; 6 vector
bytes; 16 bytes of bounded sprite tile/X tables. Total 499 annotated regions,
all Static provenance; no observed execution. Annotation evidence coverage of
classified bytes 100%; semantic/readability review remains for commentary.

Mapper 5 debugger start explicitly rejected (documented scope). Gameplay and RAM
shortcuts are unavailable; this does not block static classification. Fixed-bank
interpreter consumer discovered at $E42A with split dispatch arrays $EC3F/$ED3F;
163 distinct targets. These remain unclassified pending bounded structural review,
not rejected as unused. Audio dispatch's $8000 target requires bank contract.

Primary evidence: [disassembly/pass-001/graph.json](disassembly/pass-001/graph.json),
[graph-listing.txt](disassembly/pass-001/graph-listing.txt),
[run-boundaries.txt](disassembly/pass-001/run-boundaries.txt),
[text-candidates.json](disassembly/pass-001/text-candidates.json),
[chr-samples.png](disassembly/pass-001/chr-samples.png),
[counts.json](disassembly/pass-001/counts.json). Runtime failure retained in rpc.jsonl.

Per physical 8 KiB PRG bank (Data includes Text):

| Bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 1944 | 1944 | 6248 |
| 1 | 0 | 304 | 304 | 7888 |
| 2 | 0 | 1787 | 1787 | 6405 |
| 3 | 0 | 761 | 761 | 7431 |
| 4 | 0 | 1247 | 1247 | 6945 |
| 5 | 0 | 2110 | 2110 | 6082 |
| 6 | 0 | 1333 | 1333 | 6859 |
| 7 | 0 | 603 | 603 | 7589 |
| 8 | 0 | 3619 | 3619 | 4573 |
| 9 | 0 | 102 | 102 | 8090 |
| 10 | 0 | 1521 | 1521 | 6671 |
| 11 | 0 | 1320 | 1320 | 6872 |
| 12 | 0 | 0 | 0 | 8192 |
| 13 | 0 | 2671 | 2671 | 5521 |
| 14 | 0 | 0 | 0 | 8192 |
| 15 | 0 | 2833 | 2833 | 5359 |
| 16 | 0 | 1410 | 1410 | 6782 |
| 17 | 0 | 2127 | 2127 | 6065 |
| 18 | 0 | 788 | 788 | 7404 |
| 19 | 0 | 1568 | 1568 | 6624 |
| 20 | 0 | 1938 | 1938 | 6254 |
| 21 | 0 | 3244 | 3244 | 4948 |
| 22 | 0 | 2776 | 2776 | 5416 |
| 23 | 0 | 2262 | 2262 | 5930 |
| 24 | 0 | 1014 | 1014 | 7178 |
| 25 | 0 | 8192 | 8192 | 0 |
| 26 | 0 | 7035 | 7035 | 1157 |
| 27 | 0 | 228 | 228 | 7964 |
| 28 | 0 | 87 | 87 | 8105 |
| 29 | 0 | 4678 | 4678 | 3514 |
| 30 | 0 | 326 | 326 | 7866 |
| 31 | 1353 | 86 | 1439 | 6753 |

Header 16/16 classified; CHR banks 0–15 each 8,192/8,192 classified.

Phase finished after bounded easy gains; no terminal exhaustion claim. Static
interpreter, banked audio and script-format avenues remain productive for redirect
after commentary. No previous leads existed; new L001–L003 are recorded in
[leads.json](leads.json). Supporting interpreter inspection is in
[disassembly/pass-001/interpreter-listing.txt](disassembly/pass-001/interpreter-listing.txt).
Exact remaining complement: [unknown-ranges.json](disassembly/pass-001/unknown-ranges.json).

Verification: internal map.validate passed; portable CLI map validation passed;
source generation and ca65/ld65 rebuild passed byte-identical whole-image comparison.
Both baseline and pass-001 match original SHA-256. No dropped traces or execution
budget stops: execution never started. Execution coverage 0 bytes; no accepted
fetches, no natural or artificial gameplay visits.

Toolkit findings: 1 documented missing feature (NES-M5-001, MMC5 debugger),
0 confirmed bugs, 0 suspected bugs; runtime progression blocked, static work
unblocked. See [toolkit-issues.md](toolkit-issues.md).

Full current-source readability/commentary and final editorial audit remain pending.
Phase handoff: [handoffs/pass-001-disassembly.md](handoffs/pass-001-disassembly.md).

## pass-001 — final connected extension; disassembly complete, ROM incomplete

ROM classification: 49.56% (194874 / 393232 bytes classified; 198358 bytes unknown).

Exact percentage: 49.55700451641779%. Final checkpoint is
[pass-001-extended-verification.json](pass-001-extended-verification.json),
[portable map](../maps/pass-001-extended.json), current source
[game.asm](../generated/pass-001-extended/game.asm). `pass-001` is initial;
`pass-001-final` preserves an identical pre-extension checkpoint created after a
rejected in-memory annotation proposal. Neither supersedes the final extended map.

L001 consumed within this phase: accepted 2,005 additional native code bytes
from independently inspected VM handler/stack/arithmetic contracts, 512 bytes of
paired opcode dispatch storage, and 2 bytes of inline boot-frame metadata.
One-byte $EBAC remains unknown rather than broad rejection of neighboring handlers.
The $EE71 dispatcher ends $EE8E; its zero first pointer aliases JMP operands, so
remaining table extent stays unresolved. A proposed $EE8C end was correctly
rejected by map validation as truncated JMP; corrected before save/export.

Final 513 regions: Code 3358, Data 46282, Text 14162,
Image 131072. Every classified byte has Static role/evidence annotation.
No observed execution. Per-bank final counts replace the historical table above:

| Bank | Code | Data including Text | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 1944 | 1944 | 6248 |
| 1 | 0 | 304 | 304 | 7888 |
| 2 | 0 | 1787 | 1787 | 6405 |
| 3 | 0 | 761 | 761 | 7431 |
| 4 | 0 | 1247 | 1247 | 6945 |
| 5 | 0 | 2110 | 2110 | 6082 |
| 6 | 0 | 1333 | 1333 | 6859 |
| 7 | 0 | 603 | 603 | 7589 |
| 8 | 0 | 3619 | 3619 | 4573 |
| 9 | 0 | 102 | 102 | 8090 |
| 10 | 0 | 1521 | 1521 | 6671 |
| 11 | 0 | 1320 | 1320 | 6872 |
| 12 | 0 | 0 | 0 | 8192 |
| 13 | 0 | 2671 | 2671 | 5521 |
| 14 | 0 | 0 | 0 | 8192 |
| 15 | 0 | 2833 | 2833 | 5359 |
| 16 | 0 | 1410 | 1410 | 6782 |
| 17 | 0 | 2127 | 2127 | 6065 |
| 18 | 0 | 788 | 788 | 7404 |
| 19 | 0 | 1568 | 1568 | 6624 |
| 20 | 0 | 1938 | 1938 | 6254 |
| 21 | 0 | 3244 | 3244 | 4948 |
| 22 | 0 | 2776 | 2776 | 5416 |
| 23 | 0 | 2262 | 2262 | 5930 |
| 24 | 0 | 1014 | 1014 | 7178 |
| 25 | 0 | 8192 | 8192 | 0 |
| 26 | 0 | 7035 | 7035 | 1157 |
| 27 | 0 | 228 | 228 | 7964 |
| 28 | 0 | 87 | 87 | 8105 |
| 29 | 0 | 4678 | 4678 | 3514 |
| 30 | 0 | 326 | 326 | 7866 |
| 31 | 3358 | 600 | 3958 | 4234 |

Final internal validate, portable validate, generation and whole-ROM byte-identical
rebuild passed; original input hash unchanged. Full current-source commentary
and final editorial acceptance remain pending.

Produced/consumed lead IDs: no inherited leads; L001 produced then verified in
this pass; L002/L003 remain ready; L004 split from L001 for extended operation
table/native helper review. No terminal exhaustion claim. Useful static avenues
continue in redirect after commentary. Toolkit report unchanged: 1 documented
missing feature (mapper5 runtime), 0 confirmed/suspected bugs; runtime-only blocker.

## pass-001 — complete current-source commentary

Readability complete for all current classified source from pass-001-extended:
3,358 code + 46,282 data + 14,162 text + 131,072 image bytes. All 732 inventory
intervals reviewed (513 classified, 219 explicitly explained unknown), with 676
compact reviewed units. No prior review existed; new credit is physical-range
based. All classified opcode-table targets have meaningful entry labels, with
shared family contracts, functional branches, VM RAM pairs and hardware/transfer
symbols. Text previews and generic-data/CHR boundary notes remain local.

Fresh commentary verify rebuilt byte-identically; reading-after refreshed after
final edits. Input, current map and internal state match the frozen commentary
manifest. Source hash: 0aac66b70f93c089a3cc592cab01c12b2c16a3737244f3a6e431724c6096fdd8.
No unfinished readability ranges. Classification unchanged at 194874/393232
(49.56%); 198358 bytes unknown, execution zero. Final editorial audit remains
pending for coordinator acceptance; no audit, redirect, cleanup or publication.

Open semantic questions: L002 PRG mode/audio return contract; L003 VM bytecode
objects; L004 extended table/native helper contracts. Mechanical counter naming
retains tentative random purpose. Toolkit report unchanged: 1 documented missing
feature (NES-M5-001), 0 confirmed/suspected bugs, runtime-only blocker; see
[toolkit-issues.md](toolkit-issues.md). Source readability does not imply completion.
[Commentary handoff](handoffs/pass-001-commentary.md),
[source](../commentary/source/game.asm), [reading copy](../commentary/reading-after.asm),
[review inventory](../commentary/review.json),
[limitations](../commentary/uncertainties.md).

## redirect-001 — active static investigation

Input checkpoint pass-001-extended; classification unchanged 194874/393232,
198358 unknown (49.55700451641779%). Research active in
[redirect/redirect-001](redirect/redirect-001); no classification/toolkit changes.
L004 extended array entries 1..47 and coherent 32-bit/bitfield helpers have a
promising bounded structural proof; selector0 aliases accepted dispatcher code.
L002 bank28 audio channel engine and bank restoration now inspected; MMC5 reset
mode3 hardware premise being cross-checked against reset code. L003 inline VM
objects remain active. Next: complete audio/table consumer and native-script
boundary investigations, then combine all supported findings for independent
classification. No terminal/exhaustion assessment made. Runtime remains unavailable.

redirect-001 frontier update: coherent audio records now span 20 encoded lengths
from CPU $8500 through $A5BC, all 80 track origins internal; code and lookup bundle
boundaries $8000/$8392/$8450/$84A9 checked. VM prefix scan has zero grammar stops
or overlapping decodes across 25 fixed-bank script entries; calls identify min/max,
copy/bank-switch and register-service gateway helpers. Gateway $F8ED dispatch array
$F917..$F957 has 32 complete target words and connects to the previously unknown
native service block. Service boundary review preserves inline data $FA75..$FA7F
and $FC55..$FC5B. Primary listings and scripts are in redirect-001. Next frontier:
finish service exact intervals, test whether the established VM grammar extends
usefully to the already identified bank0 script payloads; no map/source edits.

redirect-001 substantial proposal checkpoint (research remains active): corrected
origins support988 native frame stubs and57516 VM representation records across
banks0–16/26/30/31. Encoded branch origins, complete handler operand contracts,
nonoverlapping instruction destinations and coherent frame returns jointly support
the role; zero decoder stops alone is not the proof. Exact proposed novel bytes
125525 (118281 Data +7244 Code); zero kind conflicts. Current accepted map remains
194874 classified,198358 unknown. Hypothetical residual if all proposals accepted:
72833 bytes; this is not current classification. See
[proposals](redirect/redirect-001/proposals.json),
[exact current complement/dispositions](redirect/redirect-001/unknown-ledger.json),
[active frontier](redirect/redirect-001/active.md).

Fresh residual triage found five connected native helpers in script banks0/1/6/9/11;
listings retained, not yet packaged. Opaque bank20 is mixed: initial tile/record-like
bytes followed by coherent native code, so no whole-bank generic-data shortcut is
supported. Its local B199 frame-enter helper consumes inline register-count byte
and16-bit local displacement, executes native code at caller return+4 with X=0,Y=1,
then restores frame/registers. B35A consumes inline target word and stack-adjust
byte around a local indirect call. Exact bank20 code/table/meta boundaries and
independent data-bank structural triage remain productive redirect frontier.

redirect-001 continuation checkpoint saved: latest combined **135828 proposed novel
bytes** (122963 Data,12865 Code), plus one accepted Text→Code correction at physical
81169. Accepted/current map remains194874 classified/198358 unknown; hypothetical
post-proposal residual62530 is not current classification. Bank20 native/inline
partition and connected helper families complete; bank17 exact457-record prefix
localized by native consumer and literal bank-selection evidence. L006 residual
asset/record-layout work remains active, with concrete bank18 stride/extent tests.
Do not declare exhaustion or force an empty workflow iteration at this checkpoint.
[Complete handoff](handoffs/redirect-001-active.md),
[2071 compact proposal segments](redirect/redirect-001/proposal-segments.json).
Input/map/state/source hashes unchanged; toolkit findings unchanged (NES-M5-001,
0 confirmed/suspected bugs). No runtime execution, archival, cleanup or publication.

## Round 1 redirect complete; round 2 disassembly starting

Coordinator inspected the completed continuation appended to
[redirect-001 handoff](handoffs/redirect-001-active.md) and
[gap-layout review](redirect/redirect-001/gap-layout-review.md).
L002–L006 now form one ready batch: 198300 proposed novel bytes
(185374 Data / 12926 Code), plus the one Text-to-Code correction at physical81169.
These are proposals; accepted checkpoint remains pass-001-extended:
194874 classified / 198358 unknown, 49.55700451641779%.
Hypothetical residual58 is not an accepted count. Connected and identified independent
avenues were investigated across two sequential redirect contexts in the same attempt.
Remaining local ambiguities are documented for later redirect, not terminally rejected.
Next phase: separate round-2 disassembly worker independently reviews full batch;
then full current-source commentary and redirect. No terminal assessment yet.

## Pass-002 — reviewed redirect batch accepted

ROM classification: 99.99% (393174 / 393232 bytes classified; 58 bytes unknown).

Exactly one disassembly pass. Consumed ready L002–L006 combined supported batch,
198300 novel bytes (185374 Data,12926 Code), plus physical81169 Text→Code RTS
correction. No other accepted classification changed. All additions Static,
reachability unobserved. Evidence and independent review:
[review](disassembly/pass-002/review.md), [reproducible checks/annotation](disassembly/pass-002/apply.py).
Exact per-bank and total counts: [counts](disassembly/pass-002/counts.json);
[58-byte complement](disassembly/pass-002/unknown-ranges.json).

Physical PRG banks17–22,24,27 each8188 classified/4 unknown; bank23
8174/18; bank30 8191/1; bank31 8185/7; all other PRG banks8192/0.
Header16/0; all16 CHR banks8192/0. Kind totals: Code16285, Data231656,
Text14161, Image131072;2785 regions.

Saved analysis/state.json and maps/current.json, validated and created NEW
[pass-002 checkpoint](../maps/pass-002.json); [verification](pass-002-verification.json)
confirms byte-identical rebuild with unchanged input SHA256. Map SHA256
b420bacc698c7a22659e5c1904aae320c7ba2eb6de461a4afbd3e11550476385.
Runtime progression and RAM shortcuts remain unavailable through documented mapper5
limitation; unchanged failed startup not repeated. Scenario checklist retained.
Toolkit report: [NES-M5-001](toolkit-issues.md), one documented missing feature,
zero confirmed/suspected bugs; no toolkit edits. Existing commentary untouched,
full current-source commentary is next. Produced L007 narrow residual review;
not terminal exhaustion or completion. No deletion, archive or publication.

## Redirect-002: primary residual reasoning review

Current-source commentary gate complete; accepted checkpoint pass-002 unchanged
393174/393232 classified,58Unknown. Revisited decisive primary bytes, exact bank
heads, numeric record boundary, BRK/IRQ/RTI control contract and return family;
corrected overbroad prefix/schema rejections. Proposed57 bytes (51Data,6Code),
separable independent review batch L008; one detached C0E9 retainedUnknown.
Research/evidence and exact complement/dispositions:
[redirect-002/review.md](redirect/redirect-002/review.md),
[proposals.json](redirect/redirect-002/proposals.json),
[unknown-ledger.json](redirect/redirect-002/unknown-ledger.json).
L007 history preserved; no map,state,ROM,source or toolkit changes. No terminal
exhaustion claimed. Runtime progression remains unavailable, unchanged mapper5
startup not retried. Toolkit report unchanged NES-M5-001 documented missing feature;
no new toolkit issues found, no confirmed/suspected bugs. Disk6.6GiB free,
scratch4KiB; no deletion,archive or publication.

### Pass-002 commentary gate confirmed for Redirect-002 input

[commentary-pass-002 handoff](handoffs/commentary-pass-002.md) confirms full
current-source readability complete, no remaining readability ranges:2798 scaffold
entries,3112 compact units,512 unchanged classified regions carried/reviewed.
Fresh final verification byte_identical=true; source SHA256
acd2885f832bd8a00590d174f9203f62be2b55ca85c0bd845bbbdbeed64922e8.
reading-after.asm refreshed and matched; accepted map/state unchanged.
Commentary and classification are distinct gates:58 bytes remain Unknown before
and after this redirect. Final editorial audit remains pending.

## Pass-003 — independent localized review

ROM classification: 100.00% (393225 / 393232 bytes classified; 7 bytes unknown).

Rounded percentage does not establish completion. Exactly one disassembly pass;
L008 accepted51 Static Data bytes (36 repeated-prefix,14 numeric transition,
1 BRK signature). Six proposed Code bytes retained Unknown: the bounded RTS run,
accepted hook and native neighbors do not distinguish empty entries from return-valued
padding. This is a scoped structural sufficiency judgment without a caller requirement.
Detached C0E9 also remains Unknown. Previous evidence and rejections preserved.
[Independent review](disassembly/pass-003/review.md),
[annotation/check script](disassembly/pass-003/apply.py),
[counts/per-bank inventory](disassembly/pass-003/counts.json),
[exact complement](disassembly/pass-003/unknown-ranges.json).

Physical PRG bank30:8191 classified/1 unknown; bank31:8186/6; all other
PRG banks8192/0. Header16/0; each of16 CHR banks8192/0.
Kind totals Code16285,Data231707,Text14161,Image131072;2796 regions.
Unrounded classification99.99821988037596%. Validated/saved internal store,
exported maps/current.json; NEW [pass-003](../maps/pass-003.json),
[fresh verification](pass-003-verification.json) byte_identical=true.
ROM SHA unchanged. Commentary source untouched; separate current-map commentary next.
Scenarios reviewed including ending: mapper5 remains unavailable; unchanged runtime
failure not retried. [Toolkit report](toolkit-issues.md): NES-M5-001 documented
missing feature unchanged; no new toolkit issues found; no toolkit changes.
Incomplete, no terminal exhaustion claimed. No deletion/archive/publication.
Disk6.5GiB free; shared scratch4KiB.

## Pass-003 commentary — phase complete

Full current-source readability complete; no remaining readability ranges.
393225 classified bytes covered;7Unknown bytes honestly explained. Entire prior
commentary preserved at analysis/round-2/commentary before fresh preparation.
2787 unchanged scaffold entries retain physical-interval review credit, native
prose/names and57516 VM records;11 new Data intervals explain51 accepted bytes
and the BRK dependency is corrected.2798 scaffold entries/3123 units reviewed.
Fresh verification byte_identical=true; source SHA256
 e0dd381ce63861b555fe848e41f5563daf7cafc9741f07f44a0edc8990b1f5e1.
Reading copy refreshed and matched; baseline map/state equal current accepted
map/state. [Handoff](handoffs/commentary-pass-003.md),
[scope checks](commentary-pass-003/scope-verification.json),
[review credit](commentary-pass-003/review-credit.json).
Classification remains incomplete,393225/393232; final editorial audit pending.
Toolkit report unchanged NES-M5-001 documented missing feature; no new toolkit
issues found. No toolkit changes, deletion, archive or publication.

## Redirect-003 — primary rejection review; incomplete

Current-source readability gate complete, source SHA256 e0dd381ce63861b555fe848e41f5563daf7cafc9741f07f44a0edc8990b1f5e1. Accepted393225/393232;7Unknown in2 intervals, no proposal/new acceptance. Reviewed exact map complement, current native neighbors/NMI call and prior decisive artifacts; no useful revised inference/test survives for C0E9 or F9F2:F9F8. Code and generic Data alternatives assessed independently; no universal caller or data-semantics prerequisite. [Compact rejection review](redirect/redirect-003/review.md), [primary evidence](redirect/redirect-003/primary.json), [exact ledger](redirect/redirect-003/unknown-ledger.json). L007/L008 history appended; no ready lead. Gameplay later/ending milestones remain unavailable through original mapper5 rejection; no unchanged runtime retry. Terminal-incomplete assessment requires coordinator primary-artifact inspection. ROM,map,state,source,configured binary preserved; disk6.5GiB free,scratch4KiB,no cleanup/archive/publication. NES-M5-001 unchanged; no new toolkit issues found.

## Coordinator terminal acceptance — workflow complete, classification incomplete

Accepted redirect-003 terminal-incomplete assessment after inspection of rejection
reasoning and decisive primary bytes/source/caller artifacts. No concrete corrected
inference or useful next test remains. See [coordinator review](final-acceptance/coordinator-review.md)
and [terminal handoff](final-handoff.md). This completes orchestration only.

ROM classification: 100.00% (393225 / 393232 bytes classified; 7 bytes unknown).
Unrounded99.99821988037596%; ROM classification remains incomplete. Residuals are
physical246009 length1 and260610 length6. Checkpoint pass-003; source readability
complete. Fresh final editorial audit passed2798 regions/3123units with byte-identical
rebuild and refreshed reading copy. Store/map validation passed. Source-location
prefix metadata was corrected and5921 referenced locations validated; original
metadata preserved. No source byte changed. See commentary/audit.json and final-
acceptance artifacts. Source SHAe0dd381ce63861b555fe848e41f5563daf7cafc9741f07f44a0edc8990b1f5e1.

Archive contract inspected and archive-options.json prepared; all unrecognized
analysis/commentary Markdown accounted for, final conclusions/handoffs selected.
Seven disposable operational copies/snapshot omitted from future archive selection
only; nothing deleted. Gameplay unavailable under documented mapper5 limitation,
not completed. No toolkit edits, archive execution, cleanup or publication.
