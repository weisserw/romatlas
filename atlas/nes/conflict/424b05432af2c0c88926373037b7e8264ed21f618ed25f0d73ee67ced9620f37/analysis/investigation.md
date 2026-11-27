# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round1 disassembly investigation (current pass)

Baseline byte-identical; ROM identity fixed. Vectors C000/C01B, cold boot, menu and
field contexts validated; natural map1 route and artificial later/final/cursor/battle
routes separated in scenarios.json. Accepted ROM opcode roots retain effective8KiB
slots; PRG-RAM upload execution retained as PRG-RAM, not attributed to ROM origin.

Static method: bounded toolkit direct-flow candidates plus direct JSR callee targets,
safe indexed writes below mapper space, and balanced-stack/bank-preserving callee
contracts. Unknown mapper effects, indirect stores/jumps, unofficial unobserved paths,
conflicting data and unbalanced returns stop closure. Proofs in round1/contracts-*/
static-contract-proof.json and instructions.json; changed batches keep separate files.
Last six known fixed bank-wrapper contracts added68 bytes, showing diminishing
straight closure. No arbitrary pointer target accepted as code.

Data: iNES header/vectors, all separate CHR2bpp storage, exact uniform/ASCII runs,
connected map-expansion pointer/record streams, counted field-message aliases,
PPU equipment/dialogue records, and connected field sprite/tile/PPU/unit tables.
Primary review files: round1/pattern-review.md, stage-data-review.md,
field-message-proof.json, ascii-record-proof.json, dialogue-consumer-decode.json,
field-data-review.md. Uniform CA/AA opcodes assessed within row/record context.
Stage14 shared overlap and stage15 incomplete third-group model do not reject
independently clear non-code classification; exact semantic partition remains open.

Progression: title, setup/map selector, natural map1 field; artificial map15 and final
field layouts; red AI transition; targeted AH-1S selection/command, tactical battle
against infantry, battle settings, and red-strength0 encounter result. Final field
shortcut bypasses password and does not prove authentic full final initialization.
Production/password validation/campaign ending remain separate milestones. Slow
message printing explained earlier ineffective timing; generic button/pressed calls
also ignored by NES adapter, fixed to buttons array+port (NES-DOC-001).

New terminal shortcut prerequisite was proved by actual result: byte7004 high nibble
strength, low nibble ammunition (loader93B1). RedF0->00 and ID101->229 correlate
with red strength15->0; blueF8->F7 with unchanged strength15. Mirrored M1A1 special
coordinate/flag tank starts strength0 under exact same staged producer/attack entry.
Initial constructor alone still asks ordinary command; normal attack/weapon
confirmations are being followed, with bounded waits and no further flags/PC edits.
Its result, exact final frontier and lead status will be appended at pass completion.

Ready follow-ups after commentary are indexed R1-L01..L05 in leads.json. Do not
repeat unchanged failed input/movement hypotheses. This is an easy-gain pass,
not a terminal research-exhaustion or complete-ROM claim.

### Round1 pass completion
ROM classification: 68.23% (178867 / 262160 bytes classified; 83293 bytes unknown).
Final pass-001g verified byte-identical; state/current/frozen identity matches.
All-capture fetch conflict42DD resolved narrowly; original and corrected records
preserved. Corrected M1A1/T80 battle returned to field with world16->15 and candidate
M1A1 absent; no campaign terminal. Objective eligibility/turn-boundary checks remain
R1-L01 research. No active server or interrupted productive batch. One easy-gain
pass done; structural ROM completion/readability not claimed. See disassembly-round1-
handoff.md, round1/final-stats.json and exact275-range unknown-complement.json.

### Round1 redirect complete — combined ready batch

Input remains pass-001g,178867 classified/83293 unknown in275 intervals. No annotations/source/ROM/toolkit mutations. Combined proposals contain10094 physical Code bytes and30423 Data bytes (40517 total); independent review has not accepted them. Code/data/alignment/existing-map conflicts are zero.

Followed connected factory/field/wrappers/turn/battle-commit/objective/outcome/ending handlers, plus independent final-password/stage geometry, bank5 map container and audio structures. Actual defeat, ordinary-input factory panel, pending production, exact world insertion, final staff credits and END/restart have bounded reproducible evidence. Corrected premises and failed attempts remain retained. Current-source byte3/byte4 and mask02 invalidations are explicit; next full commentary is required after disassembly.

Ready lead IDs R1-L01..L04,L06..L08; L05 historical baseline review verified with new dependency invalidations. Exact complement/dispositions, claim review and compact handoff: redirect-round1-handoff.md and redirect/r1-static-continuations/. No terminal exhaustion or complete-ROM claim.

## Round2 disassembly COMPLETE — ROM INCOMPLETE

ROM classification: 85.38% (223826 / 262160 bytes classified; 38334 bytes unknown).

Fresh checkpoint pass-002 verified byte-identical: analysis/pass-002-verification.json,
maps/pass-002.json and generated/pass-002. Current map SHA256 38b30eb21a0c8e9c601f7d1b4d065e37185fec9a4855e2dc331582b1e632f959.
Unique classified counts: Code27503,Data64597,Image131072,Text654. Netgain44959:
readybatch40517 accepted, one unsupported old footerbyte withdrawn, connectedfollow-on
501Code+3942Data. Exact244interval complement and perbankcounts saved in
round2/unknown-complement.json and final-stats.json. Header16 andCHR131072 entirely
classified; PRGcounts below.

Independent primary contract/boundary review: round2/structural-review.md,
independent-decode.json/alignment-review.json; all4531readyinstructions freshly
matched but legaldecode alone was not acceptanceproof. Genericbank3footerwords
replace stalevectormirror wording; previousfooter1FFEFannotation retained andonly
thatbyte returnedUnknown. Follow-onA900lists, stagefixedrows159/954destinations,
and coordinate/value123triples supplied4443furtherbytes, ratherthan firstbatchstop.
Primaryproofs and leadoutcomes inround2/; R1-L01..04,L06..08 consumed. 981Dreturn
unsupported byendingresetcontract;1FFEFstillstructurallyambiguous.

No new gameplayreplay: saved actual artificialfetches establishObservations while
staticviews remaininferences; no naturalcampaign/earnedvictory claim. Existing
broad milestones include later/finalfields/password, combat/defeat, factorypanel/
pendingproduction/exactworldbirth andstaffcredits/END/restart. Connected obvious
staticcontinuations completed; no remaining productive readyrecipe inthisbatch.
This is single-pass easygain exhaustion, not terminal research orwholeROMcompletion.

Currentcommentary/source unchanged(hash7fefa470f60fbfb9b9f3c25176fd57ceb5161ae6adffb1740bed743a24b9f661).
Next fullcommentary mustpreserve priorwholecommentary/reviewhistory andcarryvalid
physicaldependencies, while correcting03BAbyte3movement vs03BBbyte4combatHP/ammo,
CB39/E048activeplayerunitcount/16comparison,turnquota andendingmask02; see
redirect/r1-static-continuations/commentary-invalidations.json. Also reconcile new
1FFEFUnknown boundary. Newsource501+ready10094Codebytes needs fullreview.

Mapvalidated/saved/exported. ROMunchanged. No toolkitissue added; NES-DOC-001
workaround remains. See toolkit-issues.md. No toolkitedit/build/publication/archive/
cleanup/deletion. Disk2.8GiBfree; round2evidence4.1MiB,scratch12KiB. Allhandlesclosed.

| Physical8KiB bank | Classified | Unknown |
|---:|---:|---:|
| 0 | 1999 | 6193 |
| 1 | 4311 | 3881 |
| 2 | 1175 | 7017 |
| 3 | 2094 | 6098 |
| 4 | 5598 | 2594 |
| 5 | 6140 | 2052 |
| 6 | 8176 | 16 |
| 7 | 8192 | 0 |
| 8 | 7222 | 970 |
| 9 | 6249 | 1943 |
| 10 | 8176 | 16 |
| 11 | 8192 | 0 |
| 12 | 8148 | 44 |
| 13 | 7805 | 387 |
| 14 | 3271 | 4921 |
| 15 | 5990 | 2202 |

### Round3 disassembly complete — pass-003b, ROM remains incomplete

ROM classification: 99.99% (262136 / 262160 bytes classified; 24 bytes unknown).

Accepted38310new uniqueUnknownbytes:15344Code/22966Data, allStatic; plus one
existingData byte$5836->Code retaining16byte suffix. UniqueCode/Data intersections,
physical/CPU aliases, effective8KiB banks, instruction operands and exact record
bounds independently verified (6702instructionviews/223decodegroups); no conflicts.
Primary review analysis/round3/structural-review.md and independent-review.json;
applied.json retains exact annotation IDs. ReadyR2-L02..08 now verified, histories
preserved in leads.json. Header16 andCHR131072remain classified. Internal total kinds:
{'Code': 42848, 'Data': 87562, 'Image': 131072, 'Text': 654}. Exact24Unknownbytes/11intervals retained in
analysis/round3/unknown-complement.json; seven source-templateEAs,$982D..$9830,
$1E00D..$1E010,$1FFEF,$20000..$2000A. Local materialambiguity review reused, no
terminal-exhaustion claim. Connected supported handlers/consumer data accepted;
no obvious readyrecipe deferred. Next fullcommentary mustreview freshsource and
preserve priorreview/history; commentary/source untouched hash9e2e25c7d996ddf316c0a6351ee5d1b7278f52437a55a6a96a5acd83b8e2fcb0.

Saved scenarios reviewed and reused: later/finalfield/password, battle/worldcommit,
production/birth, defeat/defaultending/credits/END/reset. No newruntime/patches,
no earned/naturalwinning orallpages claim. Scenario phase-review appended.
Mapvalidated/saved/exported and freshpass-003b byte-identical rebuild passed:
analysis/pass-003b-verification.json, maps/pass-003b.json, generated/pass-003b.
MapSHA256 d3932aa1bbf71f8200a2af01bac65ca0665f4f545b40d864d59dbb1805aba9e3.
InputROMSHA256 unchanged424b05432af2c0c88926373037b7e8264ed21f618ed25f0d73ee67ced9620f37.
NES-MAP-002 size-limit mismatch recorded; NES-DOC-001 remains in analysis/toolkit-issues.md.
Failed pass-003 map retained; lossless JSON whitespace compaction keeps all parsed values/evidence and permits file load. No toolkit edit/build, ROM/commentary rewrite, cleanup/delete/archive/publication,
orwhole-workspace commit. Review/apply/checkpoint processes closed; no live server.

| Physical 8KiB bank | Classified | Unknown |
|---:|---:|---:|
| 0 | 8191 | 1 |
| 1 | 8192 | 0 |
| 2 | 8191 | 1 |
| 3 | 8192 | 0 |
| 4 | 8188 | 4 |
| 5 | 8192 | 0 |
| 6 | 8191 | 1 |
| 7 | 8192 | 0 |
| 8 | 8191 | 1 |
| 9 | 8192 | 0 |
| 10 | 8191 | 1 |
| 11 | 8192 | 0 |
| 12 | 8191 | 1 |
| 13 | 8192 | 0 |
| 14 | 8189 | 3 |
| 15 | 8181 | 11 |

### Round4 disassembly — supported five-byte gain; incomplete

ROM classification: 99.99% (262141 / 262160 bytes classified; 19 bytes unknown).

Accepted R3-L01 Static Data2 at physical1E00D and reopened R2-R01 Static Code3 at982D, independently verified bank/consumer/peer/bounds. Source unchanged. Full review: analysis/round4/structural-review.md; exact summary/residuals: analysis/round4/summary.json; independent configured decode: analysis/round4/independent-review.json. Both leads verified; no unchanged retry warranted. Connected supported continuations were already classified;19 residualbytes/10intervals require discriminating new structure evidence in next redirect. Saved broad gameplay milestones reviewed/reused; no missing practical representative milestone or new runtime scenario. No terminal-exhaustion claim.

Fresh pass-004 validation/generation/byte-identical verification: analysis/pass-004-verification.json, maps/pass-004.json, generated/pass-004. Map SHA256 4adf3367de16326a97e0f83cd6ea6bd5cf6a6188257d341f0332c03008339d13. Fresh compact state reload/validation in analysis/round4/fresh-state-validation.json. Lossless JSON compaction preserves parsed objects under NES-MAP-002. ROM SHA256 unchanged; commentary source d3c8aa9a6605a91465fd47e7b00407fb01dad0e57bc1988c815615e18b5db4ce preserved. No new toolkit issues found; existing analysis/toolkit-issues.md NES-DOC-001/NES-MAP-002 retained. All processes closed; no live handles.

|Physical8KiB PRG bank|Classified|Unknown|
|---:|---:|---:|
|0|8191|1|
|1|8192|0|
|2|8191|1|
|3|8192|0|
|4|8191|1|
|5|8192|0|
|6|8191|1|
|7|8192|0|
|8|8191|1|
|9|8192|0|
|10|8191|1|
|11|8192|0|
|12|8191|1|
|13|8192|0|
|14|8191|1|
|15|8181|11|

Header16 and CHR131072 bytes fully classified; no trainer/trailing bytes.

### Round5 disassembly — seven Static Code bytes accepted; incomplete

ROM classification: 100.00% (262148 / 262160 bytes classified; 12 bytes unknown).

R4-L01 verified; exact independent review analysis/round5/structural-review.md, immutable-review.json and api-log.json. No observed fetch/natural-use claim. Connected supported continuation already classified; remaining three ranges physical1E00F length1,1FFEF length1,20000 length10 retain localized role ambiguity. Saved broad gameplay evidence reviewed/reused with no new runtime. No terminal exhaustion decision.

Fresh pass-005 byte-identical checkpoint: analysis/pass-005-verification.json, maps/pass-005.json, generated/pass-005. Map SHA256 e7d6e4a1b6e7fa1bf3e42d5cd6ecfdf37d94df7758bf4f52b83129957346bc23. Fresh compact state reload analysis/round5/fresh-state-validation.json; exact counts analysis/round5/summary.json. All prior region objects preserved. Commentary source a9278c94fc310866da79d2ba72815c21f5068cd8db1e325a32122100ef658323 unchanged whole; next full commentary must incorporate seven new Code bytes and reconcile old prefix wording. No new toolkit issues found; analysis/toolkit-issues.md NES-DOC-001/NES-MAP-002 retained. All servers/process handles closed.

|Physical8KiB PRG bank|Classified|Unknown|
|---:|---:|---:|
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
|14|8191|1|
|15|8181|11|

Header16 and CHR131072 fully classified.
