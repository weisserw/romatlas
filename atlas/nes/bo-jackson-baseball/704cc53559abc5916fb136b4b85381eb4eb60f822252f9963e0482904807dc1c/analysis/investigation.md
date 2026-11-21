# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## r001 batch A
- Validated fixed-bank vectors file0x2000A–0x2000F: NMI $E353, reset $E000, IRQ $E096, effective bank15. Boot reset context [0,1,14,15]. Observed execution establishes additional effective MMC3 layouts, including bank14 in $8000 and bank15 in $C000. Physical identities retained.
- 3,386 contextual instruction roots from natural boot/menu/1P team-selection; `r001/contexts.json`. Connected static graph: 1,833 unique candidate starts, `r001/candidates.json`, `r001/discovery-summary.json`. Discovery budget max50k per root batch, no unbounded recursive classification.
- Exact physical overlap file0x1C545 BIT $FFA0 consumes bytes A0 FF at0x1C546: alternate branch target $8536 executes LDY #$FF; fallthrough at$8533 executes LDY #$00 then BIT to skip it. Both paths join$8538 STY $59. Outer linear encoding selected; the alternate instruction lies in its operand bytes. This is intentional skip structure, not a generic-data boundary. Rebuild passed.
- Inspected bank-aware groups in `r001/decoded-groups.json` before annotation. Static code carries Static provenance, actual fetch bytes carry Observed; all classified groups have evidence and CPU views. CHR image role independently bounded by header and hardware PPU pattern addressing; semantic tileset subdivisions deferred.
- Batch A checkpoint links: `../maps/disassembly-r001-a.json`, `../generated/disassembly-r001-a/game.asm`, `disassembly-r001-a-verification.json`.

## r001 batch B — packed PCM proof
Consumer file0x1EE1C–0x1EEAE / CPU$EE0C–$EE9E selects sample bank through MMC3 register7 helper, reads three-byte header `[delay,count_lo,count_hi]`, then outputs low nibble<<3 and high nibble>>1 to DAC$4011 for each payload byte. It advances A000–BFFF pointer and switches to next PRG bank on wrap. Exact eight `[start,end)` records:
`[0010,0C1A),[0C1A,14AD),[14AD,2050),[2050,2938),[2938,2EDB),[2EDB,31C7),[31C7,4906),[4906,4EA9)`.
Each has delay40. All eight starts independently match bank/pointer immediates before direct JSR$EE0C at file1C8EA,1C8F7,1C90B,1C937,1CEA5,1CEB5,1D0BA,1D0DE (repeated references omitted). This anchors both type and extent, rather than merely fitting a byte pattern. The final boundary begins AD0203 (code-like) and is kept separate. Specific spoken phrases unresolved; natural playback of every record not asserted. See r001/voice-decoder.json, r001/voice-callers.json, r001/static-1ee1c.json and r001/augment.py. Generic uniform blocks were examined after nearby code/reference review; overlapping already-proven sample spans skipped. No generic claim about the whole remaining PRG gaps.

## r001 batch C
Roster proof `r001/rosters.json`: 26 records /10,882bytes across banks4/5. Repeated header, encoded player count, exact16-byte records and visible names jointly establish data type/extents. Specific bit meanings in ratings remain unknown. Natural lineup frame corroborates Baltimore names.
Incremental observed-root/static graph adds3,300 code bytes from7,929 distinct historical effective-context roots. Current data barriers respected, no additional overlapping candidate conflicts. All new regions evidence-tagged, `r001/incremental-new-regions.json`; exact roots and graph in `r001/all-roots.json`, `r001/incremental-candidates.json`. The fresh RPC session uses explicit historical fetch roots rather than pretending saved edges were live aggregates. Natural reachability claimed only for actual recorded scenario fetches. Snapshot-based replay led through pregame anthem into initial inning1 matchup; broad mechanical and terminal-state exploration remains active.

### PCM consumption correction (same pass, evidence retained)
Closer inspection of TYA/ADC after the count-high read shows playback pointer initializes to header+2, not header+3: the count-high byte supplies the first amplitude pair, and one stored byte at each count+3 record tail is not consumed by this loop. Earlier shorthand “header then payload” overstated the exact playback range. Record starts remain independently anchored by all eight caller bank/pointer constants; first seven ends equal the next anchor, and final tail retains repeated nibble-data structure before the next code-like bytes. Record structural classification remains sound; annotations now explicitly distinguish counted playback from full stored extent. Source decoder evidence unchanged, no repeated experiment or weakened range assertion. Original generator and reason for this offset remain unresolved.

## r001 batch D — broad mechanics and ending
Natural pitch selection/throw produced a live fielding scene (`r001/pitch-result.ppm`), while the artificial two-strike/two-out branch reached the half-inning scoreboard and later user batting (`r001/batting-play.ppm`, strike65mph). Corrected store-gadget recipe verified inning9/outs2/strikes2 and scores[1,0]; third out followed ordinary game logic into handshake (`r001/ninth-play-end.ppm`), then credits (`r001/final-out-attempt.ppm`, `r001/epilogue.ppm`). These prove artificial-state visits, not a naturally completed baseball game. Natural settings entry and Music/Icons toggles confirmed; number-of-innings change input was attempted but displayed9 remained, so do not claim it changed. Play Offs selected from main menu and reached team-selection flow, $6C10=3; full series/championship route not established.

Final broad root merge:18,628 distinct effective-context roots;13,222 additional code bytes. Candidate overlap list has four entries in two bounded bank2 chains: file0x4F50/4F53 and0x4FAB/4FAE are alternate LDY immediates inside preceding BIT absolute skip operands. Bytes `2C A0 3E 2C A0 00` and `2C A0 3E 2C A0 00 2C A0 1E` match conditional selection paths. As at0x1C545, retain outer linear encoding and explicitly document alternate entries; code physical union is classified, not duplicated. See `r001/final-conflicts.json`, `r001/final-new-decodes.json`. No accepted observed root intersects a Data/Image annotation; final merge asserts this before edits. Existing code/data barriers preserved.

## r001 final accounting and verification

ROM classification: 81.65% (321066 / 393232 bytes classified; 72166 bytes unknown).

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 1704 | 5319 | 7023 | 1169 |
| 3 | 0 | 213 | 213 | 7979 |
| 4 | 0 | 3039 | 3039 | 5153 |
| 5 | 0 | 7983 | 7983 | 209 |
| 6 | 0 | 1004 | 1004 | 7188 |
| 7 | 0 | 1208 | 1208 | 6984 |
| 8 | 0 | 749 | 749 | 7443 |
| 9 | 468 | 953 | 1421 | 6771 |
| 10 | 3730 | 338 | 4068 | 4124 |
| 11 | 1321 | 73 | 1394 | 6798 |
| 12 | 2672 | 225 | 2897 | 5295 |
| 13 | 3092 | 388 | 3480 | 4712 |
| 14 | 3397 | 72 | 3469 | 4723 |
| 15 | 4485 | 89 | 4574 | 3618 |

Header16 Data; CHR262144 Image, both fully structurally classified. Total Code20869/Data38053/Image262144/Unknown72166. Final verified map/source/report: `../maps/disassembly-r001.json`, `../generated/disassembly-r001/game.asm`, `disassembly-r001-verification.json`; validation and whole-file byte identity passed, ROM SHA unchanged.1038/1038 regions evidence-tagged;42 labeled; editorial review pending.

New end-of-pass fetch roots from later credits were all inside known Code; final2 merge classified0 additional bytes and preserves its candidate/summary artifacts. Exact complement `r001/unknown-final.json` has260 intervals. Full physical fetch/root accounting is in `r001/metrics-final.json`, not inferred from map provenance alone. No accepted root intersects data.

Lead outcomes: PCM, rosters and corrected inning shortcut verified; failed unmasked shortcut retained and superseded with IRQ/readback correction. Four final graph overlaps are intentional BIT-skip LDY entries, as documented above. Future avenues in leads.json are actionable redirect investigations with missing consumer/extent contracts; they were not rejected or declared exhausted. Broad game-state visits are adequate for this first pass; repetitive credits are no longer yielding new code. This is an ordinary incomplete pass handoff to commentary/redirect, not a terminal rejection review. No toolkit issues found; report `toolkit-issues.md`.

## Round 2 disassembly — completed pass, ROM incomplete

ROM classification: 98.93% (389040 / 393232 bytes classified; 4192 bytes unknown).

Accepted the independently reviewed redirect-r001 batch: 16,341 Static Code + 51,633 Static Data = 67,974 new classified bytes. All prior regions, IDs, views and evidence are unchanged. Structural totals are 37,210 Code, 89,686 Data and 262,144 Image bytes. Header: 16 Data; CHR: 32 reporting chunks each 8,192 Image/0 unknown. The 4,192-byte/79-span residual is actual code/data or return/bank/inline uncertainty; unnamed but clearly non-code resource bytes have been classified. No terminal exhaustion claim.

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 2310 | 5745 | 8055 | 137 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 950 | 7093 | 8043 | 149 |
| 10 | 6080 | 641 | 6721 | 1471 |
| 11 | 1450 | 6742 | 8192 | 0 |
| 12 | 7027 | 721 | 7748 | 444 |
| 13 | 6650 | 1072 | 7722 | 470 |
| 14 | 6332 | 1049 | 7381 | 811 |
| 15 | 6411 | 1071 | 7482 | 710 |

Independent review: [notes](r002/review-notes.md), [fresh contextual decode/boundary audit](r002/independent-audit.json), [screen audit](r002/screen-audit.json), [annotation ledger](r002/annotation-rpc.jsonl), [metrics](r002/metrics-final.json), [exact complement](r002/unknown-final.json). Three BIT operand alternate entries are documented without overlapping annotations. All regions carry evidence; every Code region has bank-aware views. Specific script/frame/PPU containers have meaningful labels; generic non-code data retains localized role limits.

Fresh checkpoint [disassembly-r002](../maps/disassembly-r002.json), [generated assembly](../generated/disassembly-r002/game.asm), [verification](disassembly-r002-verification.json): map validated and entire 393232-byte ROM rebuilt byte-identically with the configured binary. ROM SHA256 remains 704cc53559abc5916fb136b4b85381eb4eb60f822252f9963e0482904807dc1c.

Execution coverage is separate: this static review adds no new observed fetches or gameplay replay. Prior r001 metrics remain valid for their recorded scenarios (7567 accepted opcode starts, 16745 decoded-byte union); redirect-r001's additional natural playoff bracket/Quick and artificial trophy visits retain their own scenario evidence without being relabeled here. Broad progression checklist already includes pitching, batting, fielding, ninth inning, handshake, credits and trophy. Repeating these scenarios would not resolve the remaining structural contracts.

Next phase: full current-source commentary carrying forward r001's complete review and adding all new classification; then redirect on the refreshed complement. Prior commentary/source and checkpoint remain untouched and are complete only for r001. Eight ready redirect leads are verified for their supported batch; preserve historical attempts and residual uncertainties. Residual investigation is deferred to redirect, not declared exhausted: narrower standalone helper/stack/return/pointer-domain invariants are possible continuations; see [residual dispositions](redirect/r001-20261120/residual-frontier.json) and r002 residual-review-decodes.json. No toolkit issues found; [persistent report](toolkit-issues.md). No toolkit edits, archive, publication, cleanup or deletion occurred. [Compact handoff](handoff-disassembly-r002.md).

## Redirect r002 — coherent batch ready, classification unchanged

Input disassembly-r002/current-source commentary. Corrected structural reasoning and bounded source/API proofs support3888 new Code+297 new Data=4185 unknown bytes; accepted changes0. Two exact prior-type corrections (Data8528 tail2→Code; Code5737 spaces3→Data) resolve false native boundaries without rewriting prior evidence. New ready IDs: R002-CONTROL-CONTRACTS, R002-MIXED-DATA, R002-TYPE-CORRECTIONS.

Methods: supported standalone contracts; exact bank14 aliases and per-instruction PCs; mode1 mapper/cached-bank summaries with conservative joins; bounded replay/window pointers; ten-field diagnostic save/restore plus exact ASCII/RAM records; two-batch tile updates; deterministic roster-template branch; bounded03xx scroll writes/four-way handlers; finite10-handler auxiliary return proof; menu timeout cursor continuation beyond selected-choice exits. Primary artifacts and corrected exploratory premises are preserved in `redirect/r002-20261120/research.md`.

`combined-proposals.json` and `code-recipe.json` form one review batch. Input4192-byte/79-span unknown complement is independently confirmed in `unknown-ledger.json` and exactly partitioned. After proposals, only7 bytes/2spans remain: file56D4 length6 (native versus inline/mixed ambiguity) and1DF7F length1 (isolatedRTS after memory-test failure-loop JMP). This is a nonterminal handoff, not a claim that no future interpretation can resolve them. No map/state/source edits. Fresh configured-binary map validation and whole-ROM byte-identical readable-source rebuild pass in `verification.json`; current classification stays98.93%/4192Unknown.

Current-source review is reused except the narrow future invalidations in the two correction records; next disassembly independently accepts/rejects proposals, then full current-source commentary integrates new code/data and retires false RAM2020/source claims. Runtime progression routes and terminal shortcuts remain reusable; no unchanged gameplay replay was needed. No toolkit issues found; no toolkit edits/archive/publication/deletion.

## Round 3 disassembly — pass complete, ROM incomplete

ROM classification: 100.00% (393225 / 393232 bytes classified; 7 bytes unknown).

Accepted 3888 new Static Code +297 new Static Data =4185 bytes after independent primary-contract review, plus2 Data→Code and3 Code→Data corrections. Totals: 41097 Code +89984 Data +262144 Image. Header16 Data; CHR32 reporting chunks each8192 Image/0Unknown. All classified regions have evidence; every Code region has bank-aware views. Exact unique-byte provenance: {'Observed': 16070, 'Static': 377155}. No new observed/natural reachability claim.

| PRG bank | Code | Data | Classified | Unknown | Provenance |
|---:|---:|---:|---:|---:|---|
| 0 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 1 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 2 | 2431 | 5755 | 8186 | 6 | {'Observed': 1436, 'Static': 6750} |
| 3 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 4 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 5 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 6 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 7 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 8 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 9 | 1099 | 7093 | 8192 | 0 | {'Static': 7746, 'Observed': 446} |
| 10 | 7380 | 812 | 8192 | 0 | {'Static': 4995, 'Observed': 3197} |
| 11 | 1450 | 6742 | 8192 | 0 | {'Observed': 803, 'Static': 7389} |
| 12 | 7452 | 740 | 8192 | 0 | {'Static': 6618, 'Observed': 1574} |
| 13 | 7119 | 1073 | 8192 | 0 | {'Observed': 2308, 'Static': 5884} |
| 14 | 7096 | 1095 | 8191 | 1 | {'Observed': 2461, 'Static': 5730} |
| 15 | 7070 | 1122 | 8192 | 0 | {'Observed': 3845, 'Static': 4347} |

Review: [notes and narrow invalidation](r003/review-notes.md), [fresh2494-view audit](r003/independent-audit.json), [primary witnesses](r003/primary-witness-decodes.json), [annotations](r003/annotation-rpc.jsonl), [metrics](r003/metrics-final.json). Both corrections independently confirmed before dependencies; Data8528 retains first16bytes and original ID, Code5737 retired in favor of exact roster template Data. All other prior regions are unchanged. Historical evidence and current commentary preserved.

Authoritative fresh checkpoint: [disassembly-r003-final](../maps/disassembly-r003-final.json), [whole-ROM generated source](../generated/disassembly-r003-final/game.asm), [byte-identical verification](disassembly-r003-final-verification.json). Configured binary validates map and byte-identically rebuilds393232bytes; ROM hash unchanged. The earlier disassembly-r003 checkpoint was inadvertently created while the annotation process had failed before saving; it contains the unchanged r002 map and is preserved as historical superseded verification. First annotation attempt removed a region then attempted to reuse its removed ID; in-memory-only failure left persisted state unchanged. Retry edits existing8528 directly; no toolkit bug.

Exact residual:56D4/6bytes (native versus inline/mixed ambiguity),1DF7F/1byte (orphan RTS after failure-loop JMP). Neither is classified by size/appearance. No terminal exhaustion assertion. Existing broad progression evidence and scenarios reused, no unchanged gameplay replay. Next full current-source commentary must review all additions, correct DBF4 boundary and roster/template/CPU2020 claims, carry reusable prior review, and preserve full historical copies; then redirect on refreshed7-byte complement. No commentary or redirect performed here. No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, archive, publication, cleanup or deletion.

## Redirect r003 — saved-tail correction

New attempt `redirect/r003-20261120` reopens six-byte56D4 claim on unchanged primary evidence: nativeD6CA ends with JMP83CD, restoring old-tailD6C3 computed by opcode0C before redirect. Prior immediate-skip observation did not assess this later continuation. Exact handler/stack contract supports5StaticData+1StaticCode; ready R003-INLINE-SAVED-TAIL. Accepted0; current7Unknown/2spans unchanged. Remaining singleton1DF7F reviewed separately with no discriminating role contract; no universal caller requirement. Primary windows/source proof, review, exact ledger, fresh map/source verification and full handoff linked at `handoff-redirect-r003.md`. No classification/source edit, runtime replay, terminal exhaustion or final editorial audit.

## Round 4 disassembly — single pass complete, ROM incomplete

ROM classification: 100.00% (393231 / 393232 bytes classified; 1 bytes unknown).

Exact percentage 99.999745697197%; rounding does not imply completion. Accepted R003-INLINE-SAVED-TAIL:5Static Data at56D4 and1Static Code at56D9. Totals 41098Code +89989Data +262144Image; header16Data; CHR32 reporting chunks each8192Image/0Unknown. Provenance {'Observed': 16070, 'Static': 377161}. All prior regions and entire commentary tree unchanged.

| PRG bank | Code | Data | Classified | Unknown | Provenance |
|---:|---:|---:|---:|---:|---|
| 0 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 1 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 2 | 2432 | 5760 | 8192 | 0 | {'Observed': 1436, 'Static': 6756} |
| 3 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 4 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 5 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 6 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 7 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 8 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 9 | 1099 | 7093 | 8192 | 0 | {'Static': 7746, 'Observed': 446} |
| 10 | 7380 | 812 | 8192 | 0 | {'Static': 4995, 'Observed': 3197} |
| 11 | 1450 | 6742 | 8192 | 0 | {'Observed': 803, 'Static': 7389} |
| 12 | 7452 | 740 | 8192 | 0 | {'Static': 6618, 'Observed': 1574} |
| 13 | 7119 | 1073 | 8192 | 0 | {'Observed': 2308, 'Static': 5884} |
| 14 | 7096 | 1095 | 8191 | 1 | {'Observed': 2461, 'Static': 5730} |
| 15 | 7070 | 1122 | 8192 | 0 | {'Observed': 3845, 'Static': 4347} |

Independent primary-window comparison, configured-release decodes and saved-cursor/stack/context review: [review notes](r004/review-notes.md), [fresh witnesses](r004/primary-witness-decodes.json), [annotations](r004/annotation-rpc.jsonl), [exact metrics](r004/metrics-final.json). Earlier immediate-skip rejection is corrected by following D791 JMP83CD, restoring old-tailD6C3; handlers1/9 consume1/3 bytes and zero atD6C8 synthesizes entry toRTS D6C9. Structural inference only; no new runtime observation or natural reachability claim. Original historical rejection remains preserved.

Fresh [checkpoint](../maps/disassembly-r004.json) and [whole-ROM byte-identical verification](disassembly-r004-verification.json) pass; current map/state persist exact accepted annotations. [Consistency](r004/final-consistency.json) confirms all prior regions, input ROM and full commentary unchanged. Residual [unknown complement](r004/unknown-final.json): singleton1DF7F/bank14 CPU9F6F; preceding JMP9F57 blocks fallthrough only, stored60/opcode size proves neither code nor data. No terminal exhaustion acceptance. Representative later/endgame gameplay routes reused from existing scenarios, no unchanged replay.

Next full current-source commentary integrates six newly classified bytes, replaces the stale six-byte Unknown note, explains saved-tail restoration and terminal caller-return, carries reusable full-source review and preserves historical copies. Then redirect on exact singleton complement. No commentary/redirect or final editorial audit performed here. No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, archive, publication, cleanup or deletion. Compact [handoff](handoff-disassembly-r004.md).

## Redirect r004 — singleton rejection review

Scoped primary current instruction/ROM review at `redirect/r004-20261120/primary-evidence.json` confirms success9F56/failure9F57-loop/next sound9F70 boundaries around exact one-byte1DF7F. Reviewed strongest RTS and generic-data inferences under evidence-policy; neither has independent role support, and absence of caller is not itself rejection. No saved-tail/stack/data-consumer link analogous to accepted bank2 exists in the retrieved witnesses. Concrete competing models/discriminator and why proposed replay/PC-seeding/layout comparison cannot decide them are in `rejection-review.md`/`research.md`. No useful revised test survives; proposed terminal INCOMPLETE, ready0, coordinator acceptance required. Fresh verification passes, exact residual unchanged, readability complete, final editorial audit pending. See `handoff-redirect-r004.md` for checkpoint and changed-premise reopen criteria. No map/source/runtime edits or replay.


## User-directed residual fallback

Exact complement verified first: one byte physical122751/$1DF7F, <=10 threshold. User explicitly authorizes generic Data/Manual despite preserved code/data ambiguity, with no further residual research. Added region10347; all historical research and prior regions unchanged. Now393232/393232 annotated,0Unknown; independently structurally supported393231 plus1manual-exception byte. Fresh configured-release checkpoint disassembly-fallback-final-verified validates and rebuilds byte-identically. See [final fallback record](fallback-final/final-record.md) and exact accounting/RPC links there. No toolkit issues found; no toolkit changes. Commentary/editorial gates remain coordinator work.
