# Investigation ledger

## Round 1 disassembly

ROM classification: 32.52% (42631 / 131088 bytes classified; 88457 bytes unknown).

6483 Code bytes, 36148 Data bytes, 381 annotated regions. Header contributes 16 classified bytes outside PRG banks. Each classified region has evidence; semantic/editorial readability remains for commentary. Execution and structural coverage are separate. Observed decode retains 2878 distinct physical instruction starts; final trace summary reports 3602 unique instruction identities and 5435 contextual roots, no aggregate drops. Detail capacity0 intentionally drops all detail. Tracing was disabled during later-stage initialization acceleration, then enabled during 30-frame input exercises; do not infer fetch evidence during disabled intervals.

| Physical 8KiB PRG bank | Code | Data | Unknown |
| --- | ---: | ---: | ---: |
| 0 | 0 | 1217 | 6975 |
| 1 | 385 | 355 | 7452 |
| 2 | 0 | 450 | 7742 |
| 3 | 0 | 195 | 7997 |
| 4 | 0 | 0 | 8192 |
| 5 | 294 | 1141 | 6757 |
| 6 | 532 | 0 | 7660 |
| 7 | 704 | 0 | 7488 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 0 | 0 | 8192 |
| 13 | 0 | 0 | 8192 |
| 14 | 3292 | 0 | 4900 |
| 15 | 1276 | 6 | 6910 |

Verified baseline: [baseline report](baseline-verification.json). Final checkpoint: [verification](round01-disassembly-final-verification.json), maps/round01-disassembly-final.json, generated/round01-disassembly-final/game.asm. map.validate passed; whole-file byte_identical=true. Internal resumable state analysis/state.json and maps/current.json match final checkpoint. Earlier round01-disassembly checkpoint preserved.

Evidence: round01/contexts.json and observed-decode.json (accepted fetch roots + bank-aware decode), static-first.json (validated C008/C0CA vector graph), static-connected.json (conservative successors from observed contracts), call-candidates.json/call-accepted.json (69 additional bytes from direct JSR entry contracts). Calls/mapper writes stop static continuation; unobserved code remains Static. data-inspection.json records exact non-code intervals. Header/vectors, 32 maximal uniform runs and two graphics backing banks (32KiB) classified Data; precise graphics segmentation remains unclaimed. Tile/bitplane patterns across both banks and E5D5/E5DD bank selection plus transfer consumers support generic non-code classification.

Gameplay: title at natural frame234; Start held2 frames/released then120 frames, Right/A/B180 frames, Right/B180 frames reached tile scene at frame716. Natural active player/boss/ending not confirmed. Clean snapshot saved. One-shot health/time patches $95=$0F/$92=$3B followed Right/A/B; timeout retained partial progress frame879, capture saved. Later-stage selector $9C derived from graphics/setup/config consumers. Stages2/4/6/7 attempted with PC=$E73C scene initialization, clean restore each. Distinct graphics observed for2/4/6; stage7 returned forest-like scene, no ending. Player remains absent and $94 remains0: initialized scene alone does not prove playable stage visit. Each artificial attempt recorded in scenarios.json; snapshots retained.

Lead outcome: R01-PROGRESSION is ready for redirect: understand concurrent task/player spawn contract ($F200..F273, $94 initialization mode, C2B2/C2F2 scheduling) and use a supported complete scene/player transition. PC=$E73C alone is insufficient. Do not repeat long identical input runs. Graphics/data consumers and connected unknown code also warrant static investigation after commentary; no claim of terminal exhaustion. No preexisting leads to verify.

Toolkit: [toolkit-issues.md](toolkit-issues.md), no toolkit issues found. Bounded timeout/operation-budget stops are supported behavior; state inspected afterward. ROM hash enforced by run.py. Free disk8.5GiB, scratch28KiB at handoff; no cleanup performed.

## Round 1 redirect — combined handoff ready

See analysis/redirect/r01-connected/handoff.md, frontier.md and analysis/leads.json. Fifteen supported ready leads propose88430 new classification bytes plus a local64-byte correction of existing graphics Data; accepted42631/88457 counts remain unchanged. Source/ROM/map/checkpoint hashes are intact. Actual artificial gameplay reaches initialized later stages, composite final boss and post-boss terminal throne scene; credits/natural defeat are not claimed. Static graph, field/stack/bank contracts, full sprite/audio grammar checks and generic non-code inspection are preserved. Exact current unknown ledger sums88457; proposal residual27 bytes is localized with actual structural ambiguity reasons. Independent disassembly next; no terminal-exhaustion claim or archive/publication.


## Round 2 disassembly — complete pass, ROM incomplete

ROM classification: 99.98% (131061 / 131088 bytes classified; 27 bytes unknown).

33017 Code bytes, 98044 Data bytes, including16-byte header outside PRG banks. Unique internal-map structural accounting; explicitUnknown and raw export fallback excluded. Accepted88430 new bytes (26470Code/61960Data), plus64-byte existingData-to-Code correction. No double counting. Per-bank exact counts:

| Physical 8KiB PRG bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 448 | 7744 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 898 | 7294 | 8192 | 0 |
| 6 | 633 | 7559 | 8192 | 0 |
| 7 | 1582 | 6606 | 8188 | 4 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 64 | 8128 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 7004 | 1181 | 8185 | 7 |
| 13 | 7077 | 1108 | 8185 | 7 |
| 14 | 7357 | 830 | 8187 | 5 |
| 15 | 7954 | 234 | 8188 | 4 |

Independent primary consumer/stack/field/bank review: [review](round02/independent-review.md), [fresh decode](round02/primary-decode.json), [exact code-boundary checks](round02/fresh-boundary-checks.json). All15 ready leads verified in leads.json with preserved historical attempts; no unsupported fragments forced into code. Graphics region314 split through supported map.annotate ID edit and new remainder annotations; left label preserved, obsolete whole-bank no-code rationale localized, historical checkpoint unchanged. Small evidence typo84AD localized to actual84BD. Specific known data/record roles and meaningful labels retained; generic patterned Data has practical bounded inspection and does not assert full semantics.

Final verified checkpoint **round02-disassembly-reviewed-final**: [verification](round02-disassembly-reviewed-final-verification.json), maps/round02-disassembly-reviewed-final.json and generated/round02-disassembly-reviewed-final/game.asm. map.validate passed and whole-file byte_identical=true; maps/current.json equals checkpoint map. analysis/state.json is supported saved internal state. Earlier round02-disassembly-final checkpoint remains preserved as pre-metadata-review history. Exact counts, physical residuals and hashes: round02/classification.json.

Residual27 bytes in9 intervals:00F9C3..00F9C7,018232..018235,01944E..019451,01967F..019680,01BE04..01BE0B,01C490..01C493,01D4AF..01D4B1,01E609..01E60A,01F2D1..01F2D4. NOP/fill, lone transfer/return and short literal/entry alternatives remain structurally unresolved. Reviewed rejection premises against accepted routine contracts; accepted helpers have independent field/API/family evidence, residual fragments do not. Exact alternatives and changed-evidence conditions in redirect/r01-connected/deferred-avenues.md; next redirect may review these narrow residuals after commentary. This is a single productive pass handoff, not a final whole-ROM exhaustion/completion claim.

Gameplay reused complete initialized active stage0/2/4/6/13, composite37 encounter, post-boss terminal throne scene and durable routes/snapshots in scenarios.json. Reviewed ending/credits obstacle and F2C4..F2DD pin/yield contract. No useful changed runtime condition identified; no unchanged long replay performed. Natural defeat/credits remain unverified; artificial observations and untraced setup retain their limits.

Prior commentary/source game.asm/constants.inc hashes unchanged. Full current-source commentary is now pending for new classification; final editorial/archive gates pending. No archive,cleanup,deletion,publishing or toolkit edit. [toolkit-issues.md](toolkit-issues.md): no toolkit issues found. Latest disk check8.4GiB free, scratch44KiB. Compact next-phase handoff: round02-disassembly-handoff.md.


## Round 3 disassembly — pass complete, ROM incomplete

ROM classification: 99.99% (131073 / 131088 bytes classified; 15 bytes unknown).

33022 Code, 98051 Data including16-byte header; accepted5Code+7Data. Unique internal-map accounting excludes Unknown/raw export fallback.

| Physical 8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 448 | 7744 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 898 | 7294 | 8192 | 0 |
| 6 | 633 | 7559 | 8192 | 0 |
| 7 | 1582 | 6606 | 8188 | 4 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 64 | 8128 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 7004 | 1181 | 8185 | 7 |
| 13 | 7077 | 1115 | 8192 | 0 |
| 14 | 7362 | 830 | 8192 | 0 |
| 15 | 7954 | 234 | 8188 | 4 |

Three R02 leads independently verified; historical attempts preserved in leads.json. Primary acceptance and localized residual decisions: [independent-review](round03/independent-review.md), [fresh decode](round03/fresh-primary-decode.json), [exact counts/ranges](round03/classification.json). Title return fallthrough, stage tile generic backing and current-cache default prefix accepted Static. D49F expressly compared to accepted D955/DA4C/DF95 field/count specialization standards; actual caller is a separate unresolved claim. No complete14-entry stage-table schema asserted. Supported connected downstream paths already classified.

Saved analysis/state.json, exported maps/current.json, map.validate ok. Fresh checkpoint round03-disassembly-reviewed-final passed whole-file byte-identical verification; [verification](round03-disassembly-reviewed-final-verification.json). Maintained commentary/source and ROM hashes unchanged. New map SHA25655a0ad55c4785fc44585933422a031a3e6e284ee2df6c8588dcc13d152cf9ceb.

Residual15 bytes in6 exact half-open physical ranges:00F9C3..00F9C7(4),018232..018235(3),01944E..019451(3),01967F..019680(1),01E609..01E60A(1),01F2D1..01F2D4(3). NOP/fill, spare thunk/encoded bytes, return/literal and clearflag alternate-entry ambiguities remain localized. Existing primary rejection review reused with its precise scope, no invented universal natural-fetch prerequisite. Further independent entry/field-family/non-code-envelope evidence may reopen them; this is not terminal exhaustion.

Existing broad stage0/2/4/6/13, boss and post-boss routes and ending limitation reviewed/reused; no useful changed condition warranted unchanged replay or arbitrary tiny-fragment execution. Current-source commentary integration for12new bytes pending; prior full round2 prose/review must be carried, not rewritten unchanged. Final editorial/archive acceptance pending. No archive, deletion, cleanup, publication or toolkit edits. [Toolkit issues](toolkit-issues.md): no toolkit issues found. Disk8.4GiB free; scratch44KiB. Compact handoff: round03-disassembly-handoff.md.


## Round 4 disassembly — pass complete, ROM incomplete

ROM classification: 99.99% (131076 / 131088 bytes classified; 12 bytes unknown).

33025 Code,98051 Data including16-byte header. Accepted3Code Static bytes at physical01944E..019451 / CPU943E with effective[12,13,14,15]. Independent combined encoded-boundary and guard/deactivation semantic/normal-return contract review supports the thunk. Guard excludesB2; no179-word table, natural selection or Observed claim. Existing downstream bodies already classified. Lead outcome/history preserved.

| Physical8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 448 | 7744 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 898 | 7294 | 8192 | 0 |
| 6 | 633 | 7559 | 8192 | 0 |
| 7 | 1582 | 6606 | 8188 | 4 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 64 | 8128 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 7007 | 1181 | 8188 | 4 |
| 13 | 7077 | 1115 | 8192 | 0 |
| 14 | 7362 | 830 | 8192 | 0 |
| 15 | 7954 | 234 | 8188 | 4 |

Primary evidence: [independent review](round04/independent-review.md), [fresh configured decode](round04/fresh-primary-decode.json), [exact counts/ranges](round04/classification.json). Remaining five half-open intervals:00F9C3..00F9C7(4EA),018232..018235(3EA),01967F..019680(RTS),01E609..01E60A(RTS),01F2D1..01F2D4(JSRE108). Precise primary residual review carried from redirect/r03-rejected-selector-contract/rejection-review.md; no newly supported connected unknown bytes. Existing broad gameplay, boss/postboss and final yield-loop evidence reused; no useful changed runtime condition justified unchanged replay or forced fragments. Natural credits remain unverified. This is not terminal exhaustion.

Saved state and exported map validate; fresh round04-disassembly-reviewed-final whole-file byte-identical verification passed. Map SHA25660778e26c4705c5eeb146960415dfa7538253107508bfbcd1c811bc36d60e59d. Maintained commentary/source, reading copy, constants and ROM unchanged (round04/input-integrity.json). Next commentary carries accepted round3 full698-region/476-unit review and prose for unchanged spans and integrates these3new bytes, replacing one obsolete Unknown note with a static thunk explanation. Final editorial/archive gates remain pending. No toolkit/source edits, archive, cleanup/deletion or publication. [Toolkit issues](toolkit-issues.md): no toolkit issues found. Disk8.3GiB free; compact evidence only. Handoff: round04-disassembly-handoff.md.


## Round 5 disassembly — pass complete, ROM INCOMPLETE

ROM classification: 100.00% (131083 / 131088 bytes classified; 5 bytes unknown).

The100.00% value is rounded; five Unknown bytes prevent whole-ROM completion.33025Code/98058Data including16-byte header. Accepted R04-ISOLATED-EA-GENERIC-DATA: exactly7Static generic Data bytes at00F9C3..00F9C7 and018232..018235, contexts[6,7,14,15]/[12,13,14,15]. Combined uniform pattern, closed predecessor and independently bounded Data successor review resolves the prior extra-envelope premise. HUD unofficial successor and explicit five-byte record/RTI-stack competitor independently checked. No schema/fill-purpose/natural reachability claim or neighbor extent change. All prior reviews retained; leads attempt history appended.

| Physical8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 448 | 7744 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 898 | 7294 | 8192 | 0 |
| 6 | 633 | 7559 | 8192 | 0 |
| 7 | 1582 | 6610 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 64 | 8128 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 7007 | 1184 | 8191 | 1 |
| 13 | 7077 | 1115 | 8192 | 0 |
| 14 | 7362 | 830 | 8192 | 0 |
| 15 | 7954 | 234 | 8188 | 4 |

Exact residual half-open physical ranges:01967F..019680(1),01E609..01E60A(1),01F2D1..01F2D4(3). Singleton empty-return and coherent F2C1 alternate-code ambiguities remain; no new independent constraint. See [independent review](round05/independent-review.md), [fresh primary decode](round05/fresh-primary-decode.json), [exact classification](round05/classification.json). Saved API state/export validated; fresh round05-disassembly-reviewed-final rebuild verified byte-identically. Map SHA2565b7c245080286a9f36fb101b8822c56a6c3dbdf95024187ac5aba640840e3c3f. Maintained full round4 commentary/source/constants/reading and inputROM hashes unchanged (round05/output-integrity.json).

Broad initialized stage0/2/4/6/13,boss and artificial postboss evidence reused; no unchanged replay/arbitrary-PC execution. Natural defeat/credits remain unverified. Next commentary carries valid full round4 review and integrates only7new Data bytes plus boundary dependencies, removing two obsolete Unknown notes. One productive pass, not terminal exhaustion; editorial/archive acceptance pending. No toolkit/source edits, archive, cleanup/deletion or publication. [Toolkit issues](toolkit-issues.md): no toolkit issues found. Disk8.3GiB free; scratch44KiB. Handoff:round05-disassembly-handoff.md.
