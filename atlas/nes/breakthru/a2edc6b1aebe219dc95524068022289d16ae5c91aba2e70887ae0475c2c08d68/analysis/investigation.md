# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 disassembly finished — incomplete

ROM classification: 97.91% (256690 / 262160 bytes classified; 5470 bytes unknown).

Saved 599 regions, all with structural evidence: 15837 code bytes,
109781 Data bytes and 131072 CHR Image bytes.
Header: 16 Data bytes outside the PRG table. All 16 CHR banks have 8192 Image
bytes each and zero unknown bytes. Exact counts and unknown complement are in
[round-001/counts.json](round-001/counts.json) and
[unknown-ranges.json](unknown-ranges.json). Raw fallback does not count.

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 3080 | 1701 | 4781 | 3411 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 7 | 8185 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 7 | 8185 | 8192 | 0 |
| 12 | 2354 | 4186 | 6540 | 1652 |
| 13 | 100 | 7842 | 7942 | 250 |
| 14 | 7805 | 275 | 8080 | 112 |
| 15 | 2463 | 5684 | 8147 | 45 |

Execution is separate: 3269 natural ROM opcode starts,
3136 artificial ROM opcode starts,
4196 unique starts in their union. These are bounded instruction-start
counts, not byte coverage percentages. Saved trace summaries report no aggregate
drops; detail drops were intentional. See [scenarios.json](scenarios.json).

Natural title/Mountain gameplay and artificial Bridges, Prairie, City, Airfield,
late Airfield enemy line, congratulations screen and aircraft ending animation
were confirmed visually and through accepted instruction fetches. Post-animation
return to the next loop was not observed. The ending animation was followed after
recognizing that congratulations precedes a separate $c45b routine.

Verification: map.validate and save/export succeeded. The final checkpoint
[maps/pass-001-final.json](../maps/pass-001-final.json) generated
[generated/pass-001-final/game.asm](../generated/pass-001-final/game.asm).
[pass-001-final-verification.json](pass-001-final-verification.json) confirms
byte_identical=true. Input SHA-256 remains unchanged. Configured toolkit commit:
6920600485871b6449a9f972c9854c07e8b92828; configured binary SHA-256:
8ff56b0b1cf8d2a3aca5430c70d34280ae3db36fc6774919062db4ea63843f82.
Baseline, initial and pass-001 checkpoints remain intact.

Pass stopping rationale: broad representative scenarios and bounded static review
provided a coherent supported batch. The late checkpoint added only two code bytes;
the separate ending animation was a productive alternative and added 823 bytes.
The remaining command, audio and animation fragments need local return-stack,
selector-domain or inline-data boundaries. Blind continuation crosses real data and
bank-changing calls. These are concrete redirect questions, not permanent
rejections or a terminal global exhaustion claim. No unchanged failed lead was
repeated: intro shortcuts and timeout retries changed entry contract or budget.

Full current-source commentary and final editorial review remain outstanding.
Byte-identical rebuilding establishes fidelity, not whole-ROM classification.
Input, internal state, snapshots, traces, extracted/generated assets and source
remain local. No archive, cleanup, publication, toolkit edit or workspace deletion
was performed. Toolkit findings: zero confirmed bugs, zero suspected bugs, zero
missing features and no blockers; no toolkit issues found. See
[toolkit-issues.md](toolkit-issues.md).

Exact lead outcomes and recipes: [leads.json](leads.json). Primary evidence is retained under round-001/: code-review.json/asm, static-more-review.json, fixed-gap-review.asm, fixed-accepted.json, data-layout-review.txt, record-data-review.txt, stage-metadata-review.json, gameplay-code-review.json, scenario roots and snapshots. Local protocol history is rpc-log.jsonl and batch-log.jsonl.

## Round001 redirect — static continuation contracts

See `redirect/round001-static-continuations/results.md` and `round-001-redirect-handoff.md`. Reopened R001/R002/R003 with corrected RAM-store fallthrough and independent duplicate/consumer bounds, not repeated gameplay or legal decoding alone. Connected object allocator/handlers, all ending actors/audio handlers, independent fixed/data slices yield5451 proposed bytes; accepted state unchanged. Exact complement disposes remaining19 hypothetical bytes narrowly; no terminal assessment while ready proposals exist. Phase frontier now independent disassembly review of combined batch; if interrupted before that review, retain this ready handoff. No toolkit issues found.

## Round 002 disassembly finished — incomplete

Current phase: disassembly complete; next phase full current-source commentary, then redirect. Input checkpoint pass-001-final (256690 classified/5470 unknown); output fresh pass-002-final. R001/R002/R003 independently verified:5451 bytes accepted (4832 Code,619 Data), all138 proposal slices. Per-slice [decisions](round-002/decisions.json), [contract review](round-002/review.md), [fresh configured-binary decode](round-002/independent-decode.json), [live internal-store counts](round-002/counts.json). No observed/natural reachability claim added.

ROM classification: 99.99% (262141 / 262160 bytes classified; 19 bytes unknown).

737 evidence-backed structural regions:20669 Code,110400 Data (including16-byte header),131072 CHR Image. All16 CHR banks retain8192 Image bytes and zero unknown each. PRG counts below use unique physical bytes, not CPU aliases/raw fallback.

| PRG 8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 6296 | 1892 | 8188 | 4 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 7 | 8185 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 7 | 8185 | 8192 | 0 |
| 12 | 3691 | 4500 | 8191 | 1 |
| 13 | 344 | 7848 | 8192 | 0 |
| 14 | 7827 | 365 | 8192 | 0 |
| 15 | 2476 | 5702 | 8178 | 14 |

Corrected RAM-store fallthrough and synthetic dispatcher return were accepted with enclosing contracts; connected first-pool allocator and second-pool copy included. All three ending actor graphs and downstream helper paths inspected. Explicit three-byte selector103810..103813 remains Data; next Code begins103813. Audio49-pointer storage and coherent routines accepted without inventing full token domain; explicit stack discards treated as terminal locally. Fixed pause helper preserves actual SHX abs,Y9E0007 as exact .byte with mnemonic in generated source (line10685); no repair of unusual semantics.

Exactly19 unknown bytes in five intervals:8714:1,13913:3,105610:1,130820:4,131072:10. [Current unknown complement](round-002/unknown-ranges.json), with prior [narrow residual evidence](redirect/round001-static-continuations/residual-review.json). Historical analysis/unknown-ranges.json remains the old pass001 complement and is preserved as evidence; use round002 file for current work. Noncode proposals received practical consumer/format inspection before Data annotations. Unsupported residuals were not guessed.

No gameplay repetition: prior natural Mountain and artificial Bridges/Prairie/City/Airfield/late enemy line/congratulations/aircraft visits remain recorded in scenarios.json. Post-aircraft next loop remains unobserved; no changed concrete method presently connects that milestone to these residuals. Execution metrics unchanged:3269 natural and3136 artificial ROM starts,4196 union (not byte coverage).

map.validate, map.save and map.export succeeded. [Fresh checkpoint verification](pass-002-final-verification.json) confirms byte_identical=true. State/current/checkpoint hash e88d90dd5992842c20317e59619cee3148dd84ce23395d5f01bb40f9f63e0eff; [identities](round-002/checkpoint-identities.json). Input ROM SHA256 unchanged. Configured binary used throughout; no toolkit changes.

Pass complete at supported easy-gain handoff, not global completion/terminal exhaustion. Current commentary source retains8292800b9b7ca4fad4f5e65d9f3d8b2e8098c225ace6c07ad771ac34b55448fb and frozen phase1 reference, but covers previous checkpoint: full current-source readability now needs the authorized commentary phase. Preserve prior notes/symbols while incorporating newly classified spans. Then redirect reviews exact19 residuals and strongest competing structural interpretations; this pass does not supply a terminal review.

One local pagination mistake repeatedly requested an empty map page until stopped, before annotations; its97MiB log is retained, with separate bounded final-rpc-log.jsonl. Two annotation-shape mistakes were corrected (views instead of top-level context, execution views only for Code); failed in-memory attempts were discarded before state save. No toolkit defect inferred. No toolkit issues found:zero confirmed/suspected bugs, missing features or blockers; [persistent report](toolkit-issues.md). Disk3.4GiB available, round002111MiB retained. No cleanup/archive/delete/publish/toolkit edit. Compact [handoff](round-002-disassembly-handoff.md).

## Round003 disassembly complete — incomplete ROM

Current phase: disassembly finished; next full current-source commentary, then redirect. Independently accepted R004 four exact generic Static Data slices8714:1,105610:1,130820:4,131072:10 (16 bytes), from pass-002-final. Output pass-003-final. [Review](round-003/review.md) assesses combined primary physical/source constraints and concrete competing code interpretations without requiring a consumer/schema for generic data. [Fresh inspection](round-003/independent-review.json), [live internal counts](round-003/counts.json), [current complement](round-003/unknown-ranges.json). No new Observed or natural reachability claim.

ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).

Unique physical classification:20669 Code,110416 Data (including16-byte header),131072 CHR Image;741 annotated regions. Rounded100.00% does not establish completion. Exactly13913:3 remains unknown: bank1 CPU$B649, C9 DF60. CMP#DF/RTS has coherent useful compare-return contract; isolated constants also possible. Independent bounds do not discriminate. No arbitrary forced execution or guessed code/data role. No terminal exhaustion judgment supplied this phase.

| PRG 8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 6296 | 1893 | 8189 | 3 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 7 | 8185 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 7 | 8185 | 8192 | 0 |
| 12 | 3691 | 4501 | 8192 | 0 |
| 13 | 344 | 7848 | 8192 | 0 |
| 14 | 7827 | 365 | 8192 | 0 |
| 15 | 2476 | 5716 | 8192 | 0 |

All16 CHR banks8192 Image classified/zero unknown. Previous natural/artifical execution coverage remains separate; this static pass performs no runtime replay. Scenarios/gameplay reviewed: representative areas, late Airfield mechanics, congratulations, aircraft and artificial post-ending restart already observed. No changed gameplay recipe discriminates the remaining compare island.

map.validate/save/export succeeded. [Fresh verification](pass-003-final-verification.json): byte_identical=true; map/state/checkpoint SHA256 e7d18ba36a6dee5f96484039ecd08668a198eb503f38fe550743e7189cd935c4. [Identities](round-003/checkpoint-identities.json) confirm ROM unchanged and commentary game.asm remains24eb4e18af23f9773c64a4383f9d65d07d069deb03ee95048ad8b8344b087b9c. All prior commentary/evidence/checkpoints retained intact; next full current-source commentary must update four formerly Unknown spans and stale adjacent prose. Final editorial audit remains separate.

No toolkit issues found:zero confirmed/suspected bugs, missing features or blockers; [report](toolkit-issues.md). Configured release binary only; no toolkit/ROM/commentary edits, archive, cleanup, deletion or publication. Bounded RPC861KiB, fresh review output22KiB; disk3.3GiB available. Handoff [round-003-disassembly-handoff.md](round-003-disassembly-handoff.md). Global goal not marked complete.

## Round003 compare-contract redirect

Reassessed exact13913:3 against primary decoded neighbors and current renderer source. Proposed R005 Static Code using combined independent DF clipping-bound semantics, compare-return contract and local uncalled metasprite-helper cluster; actual caller unknown. No map/state/source/ROM change; proposal only. Results and limitations: [review](redirect/round003-compare-contract/results.md). Existing broad progression reused; no unchanged runtime. Next independent disassembly review, not terminal exhaustion.

## Round004 disassembly active frontier

Independent R005 review active at pass-003-final (262157 classified;3 unknown). Assess exact13913:3 CMP#DF/RTS using primary neighbor boundaries, independently used rendering threshold and local helper cluster; live caller is not prerequisite. Preserve source/commentary and prior evidence. Exactly one pass; fresh pass-004-final checkpoint required. No unchanged gameplay replay.

## Round004 disassembly complete — structural classification complete; workflow pending

Current phase: disassembly finished; next full current-source commentary, then redirect and final editorial acceptance. Independently accepted R005 exactly13913:3 Static Code, CPU$B649 bank1 context[0,1,14,15], mechanical CompareAccumulatorWithDf. [Review](round-004/review.md) explains combined independent helper boundaries, real DF clipping-domain semantics, local uncalled duplicate helper and flag-return contract; it assesses the constants competitor and prior rejection scope. [Fresh physical/decode](round-004/independent-review.json). Caller, exact intended use and natural reachability remain unresolved. No Observed/dead-code claim.

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Unique physical bytes:20672 Code,110416 Data (including16-byte header),131072 CHR Image;742 annotated regions. Exact unknown complement empty, [internal unknown query](round-004/unknown-ranges.json); [internal unique counts](round-004/counts.json). All physical bytes now meet the structural classification contract. This is not global workflow completion.

| PRG 8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 6299 | 1893 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 7 | 8185 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 7 | 8185 | 8192 | 0 |
| 12 | 3691 | 4501 | 8192 | 0 |
| 13 | 344 | 7848 | 8192 | 0 |
| 14 | 7827 | 365 | 8192 | 0 |
| 15 | 2476 | 5716 | 8192 | 0 |

All16 CHR banks individually8192 Image classified/zero unknown; header16 Data. Per-bank details retained in counts.json. Existing broad natural/artificial progression, five areas, later mechanics, ending/aircraft and artificial restart reviewed and reused; no unchanged gameplay replay or new execution evidence. No connected unknown continuation remains after RTS.

map.annotate/validate/save/export succeeded. Fresh pass-004-final checkpoint [verification](pass-004-final-verification.json) byte_identical=true. State/current/checkpoint SHA256 a8d3fafaa2954317e567303d28a06d265a2fa885f65747dc604805bbab5b060d; [identities](round-004/checkpoint-identities.json). Input ROM unchanged. Commentary source remains6a4b44a4b937f9be225b74ca2e80fd13aeeec5ef997545fae7072f014f9ddf2c, intact. Prior742-range/696-unit readability applies to previous map: full current-source commentary must incorporate this newly classified routine and correct stale unknown/unsupported-entry prose before final editorial audit. Preserve previous symbols/comments/evidence.

No toolkit issues found:zero confirmed/suspected bugs, missing features or blockers; [persistent report](toolkit-issues.md). Configured release binary only; no toolkit/ROM/commentary edits, archive, cleanup, deletion or publication. Round004 evidence1.9MiB, disk3.4GiB free. Exactly one disassembly pass; prior checkpoints/history intact. Compact [handoff](round-004-disassembly-handoff.md). Global goal not marked complete.
