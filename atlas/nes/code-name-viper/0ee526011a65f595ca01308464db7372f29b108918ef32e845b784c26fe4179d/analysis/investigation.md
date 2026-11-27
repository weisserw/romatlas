# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Pass 001 — first disassembly, 2026-11-27

Baseline established and byte identical (analysis/baseline-verification.json).
All evidence local in analysis/pass-001/batch-log.jsonl; bounded ROM fetch roots,
decodes and discovery calls carry physical offsets and effective 8 KiB PRG slots.
No initial ready leads existed. Header, CHR backing, fixed vectors, ASCII bank
markers, accepted fetch-backed code and static direct control-flow successors
classified. Static propagation stops at mapper-affecting writes/calls; pointer
hypotheses were not promoted to code. Uniform runs classified only where they do
not overlap any accepted code; inspected exact differing endpoints and existing
references/nearby discovered labels. The repeated $00/$FF/$20 patterns support
non-code independently of absent execution; specific layout/consumer remains
unresolved. No inferred Padding/fill layout claim was needed.

### L001 — password transition contract: verified, narrow current-ROM correction

Title identity is Code Name: Viper. Consumer physical 0x1B952, CPU $B942 under
banks [12,13,14,15], reads RAM digits $0730-$0735, compares via pointer $20/$21
initialized to $BAA1, advances by six, and on match indexes three adjacent arrays:
physical 0x1B9C0 stage ($30/$31), 0x1B9F6 quest ($AF), 0x1BA2C difficulty ($06DE).
Each metadata array has 54 entries (next consumer/table boundaries and repeated
stage 0..8 / quest 0..1 / difficulty 0..2 domains). Digit interval 0x1BAB1-0x1BBF5
contains 54 six-digit decimal records. Exact table ends at the next routine's
ASL/TAY consumer. Source comparator scans #$54 = 84 records; this broader scan
is NOT used to justify table extent. Overrun behavior is deferred rather than
silently treating #$54 as decimal 54.

Recipe: import clean password.snapshot.json, explicitly release all buttons,
write six numeric digits to RAM $0730, press Start two frames, release. Input ROM
unchanged. 040471 was accepted with stage3 and map; 745765 accepted stage8 and
boss room. All subsequent execution is artificial-state observation, not natural
reachability. External 081629 is rejected for this ROM; static record index6 is
081620 (stage6), index7 is 171602 (stage7). External 'end' 217298 actually selects
quest1 stage0, confirmed metadata/handler; no terminal-screen claim.

### L002 — world map / stage entry wait: supported partial contract, not universal skip

Physical 0x3C1D (CPU $BC0D, banks [0,1,14,15]) world-map path initializes a
16-bit $3A/$3B countdown to $0384. Loop 0x3C4A tests zero after frame helper.
Fixed NMI 0x1FB06-0x1FB11 decrements this value. From saved area4active snapshot,
RAM $3A/$3B := [1,0] progresses to destination preview. Preview wait routine
physical 0x3AD2-0x3AE3 (CPU $BAC2) calls frame helper, reads buttons, DEX/BNE;
saved area4-playfield snapshot stack $1FA/$1FB = $C8/$BA proves return $BAC8,
X237 is caller wait counter. After one operation establishes instruction boundary,
register X := 1 skips that understood wait. This reaches stage3 forest background
and HUD. Later controls attempted, player display/active mechanics not yet proved
at initial captures. Keep the rendering distinction explicit.

Same countdown patch applied too early in stage7 map route did not reach preview;
the map initialization likely overwrote it. Do not repeat unchanged timing blindly.
Use run_until on a proved map loop PC/context or read actual countdown after init.
This is a timing/contract lead, not proof all stage7 contents are code or data.

### L003 — final boss / ending route: partial visit, concrete deferred research

boss-arrival.ppm confirms stage8 boss room and both actors. right+B and A inputs
were exercised; boss-played.ppm shows conversation begun. Active combat and boss
victory/ending are not confirmed. Password 217298 does not bypass to an ending in
this ROM. Redirect should inspect stage8 dialogue completion and boss health/state
consumers, save clean boss snapshot, then patch understood RAM flags/health or
use ordinary dialogue inputs. Do not set random RAM or invent an ending selector.
The initial briefing's long dialogue route yielded text progress but less evidence;
password handler provided a better supported route. Later stages are not exhausted.

This is an ordinary easy-gain handoff, not terminal investigation exhaustion.
Obvious continuations need entry/transition contracts and are passed to redirect;
existing supported classification does not depend on resolving those semantics.

### Final pass result

ROM classification: 58.80% (154141 / 262160 bytes classified; 108019 bytes unknown).

Per-bank code/data/unknown counts and verification are retained in progress.md and
pass-001/counts.json. Final source is generated/pass-001/game.asm.
| header | 0 | 16 | 0 |
| PRG 00 | 300 | 999 | 6893 |
| PRG 01 | 653 | 542 | 6997 |
| PRG 02 | 979 | 316 | 6897 |
| PRG 03 | 662 | 511 | 7019 |
| PRG 04 | 0 | 317 | 7875 |
| PRG 05 | 5 | 1408 | 6779 |
| PRG 06 | 0 | 537 | 7655 |
| PRG 07 | 0 | 1341 | 6851 |
| PRG 08 | 0 | 282 | 7910 |
| PRG 09 | 371 | 0 | 7821 |
| PRG 10 | 0 | 611 | 7581 |
| PRG 11 | 0 | 317 | 7875 |
| PRG 12 | 2302 | 0 | 5890 |
| PRG 13 | 1593 | 901 | 5698 |
| PRG 14 | 4459 | 128 | 3605 |
| PRG 15 | 3268 | 251 | 4673 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

Stage3 active firing was finally confirmed in active-tiny.png and action-play.png: visible player/projectile and ammunition50 ->49 ->48. Entry delay needed 80 additional neutral frames; this corrects the earlier display-only uncertainty. Last 24-frame action batch added82 context roots, only43 additional static bytes; same-area repetition stopped. Longer unfinished mechanics batches were not used as evidence beyond saved accepted roots.

## Round01 redirect — connected and independent ready batch

Durable primary research: analysis/redirect/round01-01/findings.md and rpc.jsonl; exact final frontier and residual ledger in same directory. Read analysis/handoffs/round01-redirect.md before resuming. Proposals33157 currently unknown bytes, accepted0. Complete boss/ending artificial route to The End and corrected stage7 active mechanics now visited; scenarios retains old attempts. Password extra30 records invalid numeric domain; immutableROM condition constant; framewait contract recovered. New graphics generic-data and bounded split-dispatch layouts, reset inline-data boundary, and coherent player-state/animation graph provide actionable next disassembly recipes. Source-wide numeric calls triaged, no unchanged failed replay. Maps/source/ROM unchanged. Overall goal active; next disassembly independent review, full commentary, redirect.

## Round02 disassembly outcomes and evidence

ROM classification: 73.72% (193274 / 262160 bytes classified; 68886 bytes unknown).

Incomplete whole-ROM classification; overall goal remains active. Accepted39133 new unique bytes in one coherent independently reviewed batch;233 unknown intervals remain. Code18409, Data43205, Text588, CHR Image131072. All636 old map entries/evidence preserved exactly. All193274 classified bytes carry evidence annotations. Exact residual complement and per-bank report: analysis/pass-002/unknown.json and classification-report.json.

| PRG bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 00 | 300 | 999 | 6893 |
| 01 | 1014 | 1774 | 5404 |
| 02 | 2726 | 378 | 5088 |
| 03 | 671 | 1070 | 6451 |
| 04 | 0 | 8192 | 0 |
| 05 | 257 | 6378 | 1557 |
| 06 | 0 | 8192 | 0 |
| 07 | 0 | 8192 | 0 |
| 08 | 407 | 753 | 7032 |
| 09 | 490 | 1230 | 6472 |
| 10 | 0 | 4160 | 4032 |
| 11 | 0 | 317 | 7875 |
| 12 | 2302 | 0 | 5890 |
| 13 | 1648 | 1607 | 4937 |
| 14 | 4637 | 202 | 3353 |
| 15 | 3957 | 333 | 3902 |

Header16 bytes Data; CHR banks00..15 each8192 Image bytes classified, zero code/unknown. Data column above includes Text. Physical ROM identities and effective8KiB bank contexts retained.

Independent fresh decodes narrowed L004's two-byte code overrun; a separate pause-consumer proof then classified those bytes Data. Accepted scene/boss/ending/player/reset/graphics batch, actual artificial fetch roots and connected direct control successors. Followed explicit post-mapper stage-introduction/map-flight callees through complete useful render/cleanup/wait routines and bounded screen/motion/probe data. Review rationale, corrected endpoints and provenance: analysis/pass-002/review.md; durable RPC/decodes/accepted manifests in that directory. No unchanged gameplay replay: reused active mansion/frame677, terminal The End/frame2320, and completed progression checklist.

L004/L005/L006/L009/L010 verified with narrow structural roles; L007/L008 semantic answers retained for commentary. Isolated physical58BC return and FE68 singleEA remain unknown. New redirect questionsL011/L012 cover resource selector/bank identity and bank10 checkpoint/spawn records; no terminal exhaustion claim.

Execution metric separate: reused3285 contextual accepted-fetch roots/2777 unique physical starts in artificial snapshots. Current code provenance Observed10694 bytes and Static7715 bytes; these map provenance totals are not exhaustive execution coverage or natural reachability. Static data and inferred code receive no Observed claim. Source readability separate: pass002 has972 code regions/18409 bytes,1145 annotated physical regions; maintained pass001 commentary remains unchanged. Full current-source commentary migration/review is next.

Fresh maps/pass-002.json and generated/pass-002 verified byte-identical, analysis/pass-002-verification.json. Internal/current/checkpoint map SHA256de61d9a48d3713139a2fb103019075d49b8740ce5986b1c42bc5c0a96e905dd1; ROM SHA2560ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d unchanged. No baseline rerun. No toolkit issues found: zero confirmed/suspected bugs or missing features; no blocker, see analysis/toolkit-issues.md. No toolkit edit, cleanup/deletion, archive or publishing. Handoff analysis/handoffs/round02-disassembly.md.

## Round03 disassembly complete — pass003

ROM classification: 92.04% (241284 / 262160 bytes classified; 20876 bytes unknown).

Whole-ROM classification incomplete; overall goal active. Accepted48010 unique new bytes (6946 Static Code,41064 Static Data) in204 new regions; preserved all1145 prior regions/evidence exactly. 123 exact residual intervals: analysis/pass-003/unknown.json. Counts Code25355, Data84269, Text588, Image131072. All241284 classified bytes have evidence.

| PRG bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 00 | 439 | 7751 | 2 |
| 01 | 1481 | 6601 | 110 |
| 02 | 4718 | 884 | 2590 |
| 03 | 1790 | 5111 | 1291 |
| 04 | 0 | 8192 | 0 |
| 05 | 257 | 6378 | 1557 |
| 06 | 0 | 8192 | 0 |
| 07 | 0 | 8192 | 0 |
| 08 | 1380 | 1553 | 5259 |
| 09 | 795 | 2240 | 5157 |
| 10 | 0 | 8192 | 0 |
| 11 | 59 | 8133 | 0 |
| 12 | 2366 | 5817 | 9 |
| 13 | 1769 | 6268 | 155 |
| 14 | 4887 | 894 | 2411 |
| 15 | 5414 | 443 | 2335 |

Header16 Data; CHR00..15 each8192 Image, no unknown. Per-bank counts and exact residual ledger in analysis/pass-003/classification-report.json and unknown.json.

L011/L012/L013/L014 verified independently with fresh3465 CPU/bank instruction views, unchanged saved primary consumers/patterns, independent29 checkpoint streams/558 spawn records/219 terminators/16x31-byte descriptors. Aliases counted once. Critical mapper/audio/scene contracts and generic non-code fallback limits: analysis/pass-003/review.md. Prior even-selector permission remains rejected; $8000/$8001 remain barriers. No speculative actor enumeration or opcode-only code annotation. Original gameplay through late mansion/boss/ending reused; no unchanged replay/baseline rerun. All concrete connected graph continuations already packaged; remaining actor-instance0504/0506 and mixed endpoint questions need redirect research, not terminal rejection.

Execution coverage separate: no new runtime fetch observations; prior3285 contextual artificial-fetch roots/2777 physical starts retained. Code provenance Observed10694, Static14661 bytes, not exhaustive execution coverage or natural reachability. Source readability separate: maintained pass002 commentary untouched; generated/pass-003 has1123 code regions and needs full current-source commentary migration/review next. Final editorial audit pending.

Fresh maps/pass-003.json and generated/pass-003 byte-identical verified: analysis/pass-003-verification.json. Internal/current/checkpoint SHA256ed8d3f1839298b5116956b9a1290df6c932af010b6575ad227fd192497dbdbb6; ROM SHA2560ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d unchanged. No toolkit issues found; zero confirmed/suspected bugs/missing features, no blocker; analysis/toolkit-issues.md. No toolkit edits, deletion/cleanup/archive/publication or downloads. Compact handoff analysis/handoffs/round03-disassembly.md.

## Round03 disassembly final connected extension — pass003-final

ROM classification: 92.14% (241542 / 262160 bytes classified; 20618 bytes unknown).

Whole-ROM classification incomplete; overall goal active. Accepted48268 unique new bytes (6946 Static Code,41322 Static Data) in208 new regions; preserved all1145 prior regions/evidence exactly. 119 exact residual intervals: analysis/pass-003/unknown.json. Counts Code25355, Data84527, Text588, Image131072. All241542 classified bytes have evidence.

| PRG bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 00 | 439 | 7751 | 2 |
| 01 | 1481 | 6709 | 2 |
| 02 | 4718 | 884 | 2590 |
| 03 | 1790 | 5111 | 1291 |
| 04 | 0 | 8192 | 0 |
| 05 | 257 | 6378 | 1557 |
| 06 | 0 | 8192 | 0 |
| 07 | 0 | 8192 | 0 |
| 08 | 1380 | 1553 | 5259 |
| 09 | 795 | 2240 | 5157 |
| 10 | 0 | 8192 | 0 |
| 11 | 59 | 8133 | 0 |
| 12 | 2366 | 5817 | 9 |
| 13 | 1769 | 6418 | 5 |
| 14 | 4887 | 894 | 2411 |
| 15 | 5414 | 443 | 2335 |

Header16 Data; CHR00..15 each8192 Image, no unknown. Per-bank counts and exact residual ledger in analysis/pass-003/classification-report.json and unknown.json.

L011/L012/L013/L014 verified independently with fresh3465 CPU/bank instruction views, unchanged saved primary consumers/patterns, independent29 checkpoint streams/558 spawn records/219 terminators/16x31-byte descriptors. Aliases counted once. Critical mapper/audio/scene contracts and generic non-code fallback limits: analysis/pass-003/review.md. Prior even-selector permission remains rejected; $8000/$8001 remain barriers. No speculative actor enumeration or opcode-only code annotation. Original gameplay through late mansion/boss/ending reused; no unchanged replay/baseline rerun. All concrete connected graph continuations already packaged; remaining actor-instance0504/0506 and mixed endpoint questions need redirect research, not terminal rejection.

Execution coverage separate: no new runtime fetch observations; prior3285 contextual artificial-fetch roots/2777 physical starts retained. Code provenance Observed10694, Static14661 bytes, not exhaustive execution coverage or natural reachability. Source readability separate: maintained pass002 commentary untouched; generated/pass-003-final has1123 code regions and needs full current-source commentary migration/review next. Final editorial audit pending.

Fresh maps/pass-003-final.json and generated/pass-003-final byte-identical verified: analysis/pass-003-final-verification.json. Internal/current/checkpoint SHA2567b1032bc75e57ecef9741788fd7f1127b26c87fb506b6fba76d03cded25ad646; ROM SHA2560ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d unchanged. No toolkit issues found; zero confirmed/suspected bugs/missing features, no blocker; analysis/toolkit-issues.md. No toolkit edits, deletion/cleanup/archive/publication or downloads. Compact handoff analysis/handoffs/round03-disassembly.md.

Round04 disassembly ACTIVE; exact review frontier analysis/pass-004/frontier.md. Input pass003-final retained. FE68 actual04 correction acknowledged; primary structural review underway.

## Round04 disassembly — pass004 verified

ROM classification: 99.98% (262112 / 262160 bytes classified; 48 bytes unknown).

Whole-ROM incomplete; overall goal active. Accepted20570 unique new bytes (7431 Static Code,13139 Static Data),186 regions (75 Code,111 Data). All1353 prior annotations/evidence preserved exactly. Fresh3292 explicit CPU/bank instruction views and111 immutable data slices independently checked. Role/control/stack evidence and exact residual counterinterpretations: analysis/pass-004/review.md; accepted.json, fresh-code-review.json and data-review.json.

| PRG bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 00 | 439 | 7751 | 2 |
| 01 | 1481 | 6711 | 0 |
| 02 | 4748 | 3435 | 9 |
| 03 | 2157 | 6031 | 4 |
| 04 | 0 | 8192 | 0 |
| 05 | 260 | 7931 | 1 |
| 06 | 0 | 8192 | 0 |
| 07 | 0 | 8192 | 0 |
| 08 | 4798 | 3385 | 9 |
| 09 | 3851 | 4333 | 8 |
| 10 | 0 | 8192 | 0 |
| 11 | 59 | 8133 | 0 |
| 12 | 2373 | 5817 | 2 |
| 13 | 1769 | 6423 | 0 |
| 14 | 5126 | 3066 | 0 |
| 15 | 5725 | 2454 | 13 |

Header16 Data; CHR00..15 each8192 Image, zero unknown. Code32786, Data97666, Text588, Image131072. Exact21 residual intervals/48 bytes: analysis/pass-004/unknown.json. All262112 classified bytes evidence-annotated. Classification report has allbank counts. L015/L016 verified; Static structural roles do not imply natural selection. No lone opcode/pointer-only Code or invented trailer schema accepted. Primary FE68 correction04 remains required in commentary; no stage10domain claim.

Map validationok; fresh maps/pass-004.json and generated/pass-004 byte-identical rebuild verified in analysis/pass-004-verification.json. Internal/current/checkpoint SHA256db5ce160d67a8705fb7da671627035cdd170680a7af6bccb90756afcdb8c010e; unchanged ROM0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d. Prior map entries preserved. No baseline or unchanged gameplay rerun; broad late/boss/ending routes reused. No new runtime observations. Execution coverage separate: Observed Code10694, Static Code22092 bytes; prior contextual/artificial fetch evidence retained, not full execution coverage.

Current-source readability separate: preserved prior commentary untouched;186 new regions need full current-source commentary migration/review using generated/pass-004. Finaleditorial audit reserved. 48 tiny bytes remain genuinely structurally ambiguous, not a terminalexhaustion decision. No obvious connected easygain deferred; supported combined actor/terrain/helper/data batch applied. Next commentary then redirect for new supported discrimination. No toolkit issues found; zero confirmed/suspected bugs/missingfeatures and no blocker; analysis/toolkit-issues.md. No toolkit/ROM change, cleanup/deletion, archive, download or publication. Handoff analysis/handoffs/round04-disassembly.md.

## Round04 commentary correction and complete current-source coverage

Maintained source/review correct prior physical $1fe78 / CPU $fe68 byte from the
erroneous $ea statement to actual $04. It continues the eleven-byte bank-selector
metadata before FE69 A0 00; the narrow generic-data suffix does not establish a
stage-ten selection domain. Historical records and full pass003 commentary remain
preserved; see commentary/uncertainties.md and analysis/round-4/commentary-pass003.
All current classified source has readability coverage.48 bytes/21 structural
ambiguities remain separate from commentary completion; no exhaustion claim.

## Round05 disassembly — pass005 verified

ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).

Whole-ROM incomplete; overall goal active. Independent review accepted L017-L020:32 unique Static Code bytes in12fragments. All1539 prior annotation records preserved exactly. Exact primary review analysis/pass-005/review.md; fresh-review.json contains17 configured-bank instruction decodes; accepted.json and rpc.jsonl record application.

| PRG bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 00 | 439 | 7751 | 2 |
| 01 | 1481 | 6711 | 0 |
| 02 | 4749 | 3435 | 8 |
| 03 | 2159 | 6031 | 2 |
| 04 | 0 | 8192 | 0 |
| 05 | 260 | 7931 | 1 |
| 06 | 0 | 8192 | 0 |
| 07 | 0 | 8192 | 0 |
| 08 | 4806 | 3385 | 1 |
| 09 | 3859 | 4333 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 59 | 8133 | 0 |
| 12 | 2373 | 5817 | 2 |
| 13 | 1769 | 6423 | 0 |
| 14 | 5126 | 3066 | 0 |
| 15 | 5738 | 2454 | 0 |

Header16 Data; CHR00..15 each8192 Image, zero unknown. Code32818, Data97666, Text588, Image131072. All262144 classified bytes evidence-annotated; nine exact residuals in analysis/pass-005/unknown.json. Empty handlers require combined table/dispatcher/ordinary-stack contracts, animation wrappers require known interface/counterpart/field contracts, coordinate-copy is ONLY four-instruction fragment (no valid entry/exit or obsolete origin asserted), palette prefix has consumed Y and independently valid scene-resource contract. Natural selection unresolved; no new runtime observation. Execution provenance separately ObservedCode10694, StaticCode22124; not full coverage.

Map validationok; state/current/maps/pass-005.json SHA2569ee8e27a9dbcb20283460614a5113e3869648a4d45ea5334215468301f82964e; generated/pass-005 rebuilt byte-identically, analysis/pass-005-verification.json. ROM unchanged0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d. Prior commentary/source protected hashes unchanged, analysis/pass-005/protected-identities.json. Full current-source commentary migration/review required for12new regions; final editorial audit reserved. No baseline or unchanged gameplay replay; completed broad later/boss/ending routes reused. No connected unknown successor or useful local expansion deferred. Residual ambiguity not missing semantics alone; no forced genericData or terminal exhaustion. Next commentary then redirect exact16bytes/9fragments.

No toolkit issues found; zero confirmed/suspected bugs/missingfeatures and no blocker, analysis/toolkit-issues.md. No toolkit/ROM/source mutation, cleanup/deletion/archive/download/publication. Handoff analysis/handoffs/round05-disassembly.md.
