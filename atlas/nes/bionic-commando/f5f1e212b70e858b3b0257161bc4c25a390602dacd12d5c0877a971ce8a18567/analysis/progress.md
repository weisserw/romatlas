# Disassembly progress

Current coordinator result: justified terminal INCOMPLETE at round-011-classified. ROM classification: 99.99% (262132 / 262160 bytes classified; 28 bytes unknown). Full current-source readability, final editorial audit, archive document selection and coordinator completion verification passed. Orchestration is complete with justified terminal incomplete classification. Initialization entries below are preserved history.

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly in progress

Baseline established with byte-identical verification (analysis/baseline-verification.json).
Startup checkpoint round-001-startup verified byte-identical; internal state saved.
ROM identified visually as Bionic Commando (Capcom1988); NTSC mapper1,256KiB PRG,
CHR RAM. Startup title then communications intro reached naturally at frame398.
Classified2455 bytes (2433 code,22 header/vectors); unknown259705.
ROM classification: 0.94% (2455 / 262160 bytes classified; 259705 bytes unknown).
Startup candidates inspected in analysis/evidence/round-001/startup-decoded.json;
static provenance kept separate from observed-root contexts. Continuing broad gameplay,
RAM transition contract research and unambiguous non-code patterns. One documentation
issue recorded in analysis/toolkit-issues.md; workaround available.

### Graphics batch and map progression

Added98304 Image bytes, physical0x10010..0x2800F (effective8KiB banks8..19).
Complete512-tiles/bank contact sheet inspected (banks08-19-tiles.png); coherent
16-byte two-bitplane graphics throughout, exact aligned bank boundaries, and
$D28D transfer consumer independently corroborates16-byte PPU records.
ROM classification: 38.43% (100759 / 262160 bytes classified; 161401 bytes unknown).
Overworld reached naturally at frame2522 after12 A press120/release2 cycles;
trace disabled only for text waits. Saved dialogue-route.json and after-dialogue.png.
Continuing toward area1 and concrete later-area RAM shortcut investigation.

### Supported progression shortcut under test

$CB=1 is overworld hierarchy; $CC=1 map, $CC=2 equipment selection.
Local bank 26 handler at physical0x36514 sets room$4C=area$4D−1 and advances$CC;
$D6A0 dispatcher contract inspected. Clean snapshot saved as
analysis/evidence/round-001/pre-shortcut-snapshot.json (frame3002).
Patch internal RAM$004D=[01],$00CC=[02,00,00] at effective banks24,25,30,31;
60frames accepted execution produced initialized equipment selection screen at3062,
area1-shortcut-menu.png. Artificial observation, not natural reachability.
Start60 left equipment screen unchanged; trying freshA60 on selected END.
FreshB at overworld failed to advance; no unchanged retry planned.
Full160 uniform-run candidates (11251bytes) and16-byte flanks inspected, no
conflicting classified code. Adding genericData; roles remain unresolved for those
runs. Do not infer fill purpose or unknown source layout from uniformity.

### Data checkpoint / interruption recovery

Uniform11251bytes saved; current113177 classified,148983 unknown.
ROM classification: 43.17% (113177 / 262160 bytes classified; 148983 bytes unknown).
Checkpoint round-001-data verified byte-identical; see its verification JSON.
First live RPC host ended with external exit143 during equipment pulse test;
restarted host, restored analysis/state.json and pre-shortcut-snapshot.json.
Prior runtime aggregates survive only through saved startup-contexts/discovery evidence;
new session records new accepted execution. This is session interruption, not a
confirmed toolkit defect. Equipment A advances category, B cancels. Decoded
bank 26 $AACB continuation increments$CC, clears$CD/$CE/$CF. Now testing direct
supported loader entry by$4D=1,$CC..CF=[3,0,0,0], snapshotroom$4C=0 retained,
reasserted no buttons;60frame budget3000000 ticks,20000ms. Pass still in progress.

### Broad artificial milestones and classification batch

Self-contained probes restore the clean map snapshot then patch$4D(area),$4C(room),
$3F=0,$CC..CF=[3,0,0,0]. Reassert no buttons;100 frames untraced load, then8frames
right+B and8frames right+A traced. Actual accepted execution and rendered results:
area 12 / room 11 exterior (gunshots visible), neutral 13 / room 12 (NPC present),
area 12 / room 96 special interior. Each has *-result.json, *-contexts.json,
*-candidates.json, *.ppm/png and *-snapshot.json in evidence/round-001.
These are artificial visits; no normal map route or natural selection claimed.
$CB=4 explicit main-dispatch shortcut produced a wounded-person communications
cinematic (ending-cb4.*), whose identity as ending is not yet established.
Attempting bounded text progression and source-supported room98/101 transitions.

Every256-byte block in0x00610..0x0600F and0x08810..0x0EC0F inspected for repeated
map-cell/structure patterns (data-pattern-review.json). Added41972 previously
unknownData bytes; prior uniform runs preserved. Concrete room-table consumers
support resource role; exact original record schema unresolved. Data Crystal ROM
map independently corroborates ranges, but classification rests on local patterns.
Added3631 inspected gameplay Code bytes in102 groups(gameplay-new-decoded.json).
ROM classification: 60.57% (158780 / 262160 bytes classified; 103380 bytes unknown).
Saved state/map; final fresh verified checkpoint pending remaining milestone probes.

### Ending cinematic evidence and closing frontier

Confirmed a base explosion over the ocean and a helicopter lifting the player in
CB4 cinematic. Selected evidence: cb4-wait-2.png (explosion), cb4-wait2-0.png
(rescue), cb4-wait*-result.json (actual partial timeout states). This is a supported
artificial cinematic route, separate from defeating the final boss naturally.
After nine bounded timed waits, the latest durable snapshot is
cb4-wait3-snapshot.json at frame6949; black transition persists. Credits and terminal
screen are unverified. Collecting final8 traced frames and wait-state bytes, then
handing the unresolved cinematic transition/entry contract to redirect.

Normal gun cockpit attempt failed; revised Hyper Bazooka selection$04CA=5 yielded
black output with selectors[1,3,2,0,0], so boss defeat remains unproven. Do not infer
success from the fade or inventory write. No further unchanged shot retry planned.
Font image0x2F810..0x2FC0F inspected as64 coherent packed glyph tiles(font.png),
adding976 unknownImage bytes and promoting48 previously uniformData bytes; boundary
split preserves the earlier generic-data record's preceding32-byte portion.
ROM classification: 61.36% (160874 / 262160 bytes classified; 101286 bytes unknown).
Code8349,Data53197,Image99328. All annotations have structural evidence and Static
provenance; accepted contexts separately show natural/artificial execution roots.
Final checkpoint/handoff pending final small cinematic trace review. Disk4.1GiB free,
scratch160KiB, cited evidence98MiB; no cleanup or deletion performed.

## Round 001 disassembly phase closed — ROM incomplete

ROM classification: 61.42% (161031 / 262160 bytes classified; 101129 bytes unknown).

Final internal state: analysis/state.json; supported portable map: maps/current.json.
Fresh verified checkpoint: maps/round-001-final-tables.json,
generated/round-001-final-tables/game.asm,
analysis/round-001-final-tables-verification.json. Map validation passed and rebuild
is byte-identical. Input ROM SHA-256 remains f5f1e212b70e858b3b0257161bc4c25a390602dacd12d5c0877a971ce8a18567.
All 547 classified regions (161031 bytes) have evidence annotations.
Kinds: Code 8474, Data 53229, Image 99328. Static provenance is
intentional: structural classification is separate from observed reachability.
Saved execution evidence union: 2234 unique physical opcode starts,
2831 contextual starts. Natural startup/menu observations and
artificial later-game observations are separated by their scenario records; untraced
waits are excluded. This is not a whole-ROM execution percentage.

Final small batch added 125 cinematic code bytes and 32 inline dispatcher-pointer
bytes. Dispatcher tables have independently anchored adjacent code boundaries; no
claim all selectors occur naturally. Detailed exact unknown complement (203 prior
gaps, now updated) and per-bank metrics: evidence/round-001/unknown-ranges.json and
final-counts.json. The remaining code/data distinction in mixed banks and indirect
handler contracts need redirect research; no terminal exhaustion claim is made.

Latest cinematic snapshot cinematic-late-snapshot.json at frame6957 has selectors
[4,0,1,4,3], wait byte$74=0, bank context[24,25,30,31]. Base explosion and helicopter
rescue were visited; credits/terminal screen remain unverified after bounded waits.
Further long waiting or repeated cockpit shots is deferred until redirect resolves
the specific late-cinematic/player-entry contract. Representative area1, area12,
neutral13, special rooms96/98/101 and their mechanics have reproducible local routes.

Toolkit issues: two confirmed bugs/documentation defects, no blockers; see
analysis/toolkit-issues.md (NES-DOC-001 request budget naming; NES-OBS-002 observed
provenance fields). No toolkit edits, publishing, cleanup or deletion performed.
Hand off to commentary for all currently classified source, then redirect.

| Effective 8 KiB bank | Code | Data/Image | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 6656 | 6656 | 1536 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 111 | 300 | 411 | 7781 |
| 4 | 0 | 6177 | 6177 | 2015 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 29 | 3137 | 3166 | 5026 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 0 | 0 | 8192 |
| 21 | 303 | 64 | 367 | 7825 |
| 22 | 0 | 102 | 102 | 8090 |
| 23 | 0 | 3068 | 3068 | 5124 |
| 24 | 1968 | 0 | 1968 | 6224 |
| 25 | 0 | 719 | 719 | 7473 |
| 26 | 25 | 729 | 754 | 7438 |
| 27 | 1265 | 97 | 1362 | 6830 |
| 28 | 45 | 0 | 45 | 8147 |
| 29 | 0 | 192 | 192 | 8000 |
| 30 | 4003 | 32 | 4035 | 4157 |
| 31 | 725 | 196 | 921 | 7271 |

## Round 001 commentary phase complete — current-source readability

Checkpoint remains round-001-final-tables; maps/current.json and analysis/state.json
are unchanged from the frozen commentary baseline. All 747 current physical spans
are reviewed in commentary/review.json: 298 code fragments, 233 data, 16 image and
200 unknown spans, with 298 code-unit records and maintained source locations.
Source includes routine/control-flow names, 181 exact-value hardware/RAM/field
aliases and behavior/format explanations across the entire current classification.
Unknown bytes remain unclassified with honest boundary notes.

ROM classification: 61.42% (161031 / 262160 bytes classified; 101129 bytes unknown).
Current-source readability is complete; whole-ROM classification and final
editorial/archive acceptance remain separate and incomplete. Fresh final verification
in commentary/verification.json passed ok=true, byte_identical=true; reading-after.asm
was refreshed afterward. See analysis/handoffs/round-001-commentary.md for examples,
semantic uncertainties and redirect questions. No toolkit/ROM/map changes or publish.

## Round001 redirect — combined proposal ready

Completed research attempt analysis/redirect/round-001-a without map/source changes.
Ready leads R001-REDIRECT-CODE/DATA propose95126 previously unknown bytes
(35033 Code,60093 Data); accepted0. Actual classification remains161031/262160,
unknown101129; residual6003 only if independently accepted. Closed connected
cinematic, player/projectile, actor, audio, boss, fixed-helper, room and bank23
resource graphs; zero final decoder barriers/overlaps/existing-data conflicts.
Exact proposal/residual ledger and primary evidence retained in attempt directory.
Artificial terminal signature screen visited; natural final route remains unresolved.
Handoff analysis/handoffs/round-001-redirect.md; no exhaustion/final completion claim.

## Round 002 disassembly — reviewed batch saved, verification pending

ROM classification: 97.71% (256160 / 262160 bytes classified; 6000 bytes unknown).

Accepted95129 bytes:35277 Code and59852 Data. Corrected241-byte B07A
transfer routine island inside proposed metasprite Data; added3-byte C6AF vector.
Independent graph/consumer/extent review: analysis/evidence/round-002/review.md.
Map validate ok; saved analysis/state.json and maps/current.json. Fresh checkpoint
round-002-disassembly is running; no byte-identical result claimed yet.

## Round 002 disassembly complete — substantial reviewed batch

ROM classification: 97.85% (256535 / 262160 bytes classified; 5625 bytes unknown).

Gain95504 bytes:35277 Code and60227 Data, including375 additional bounded
residual-resource bytes. Total Code43751, Data113456, Image99328; header included
in Data. All previous regions preserved unchanged. The main95129-byte batch
corrected the B07A area-transfer island (241 bytes Data->Code) and added C6AF's
three-byte fixed vector. Generic audio data remains supported while eleven unused
cinematic-constant pointer-table claims were discarded. Evidence/consumer details:
analysis/evidence/round-002/review.md, independent-review.json, transfer-graph.json,
annotation-results.json and residual-data-results.json. R001-REDIRECT-CODE and
R001-REDIRECT-DATA verified with these localized corrections.

Both fresh checkpoints round-002-disassembly and round-002-final validated and
rebuilt byte-identically. Final report analysis/round-002-final-verification.json;
map/state SHA256 8b9f176e93f6c9b137dee1440e49653ab5916fe0e1d2380cc255db2385825b65.
ROM SHA256 unchanged f5f1e212b70e858b3b0257161bc4c25a390602dacd12d5c0877a971ce8a18567.

Current-source commentary remains the preserved round001 work; this new95504-byte
classification requires the next commentary phase to complete current readability.
Whole-ROM classification/editorial completion remains incomplete. No terminal
exhaustion claim: refreshed exact residual ledger has201 intervals/5625 bytes,
including unrooted code-shaped bodies, consumer/table gaps and mixed footer guards.
See analysis/handoffs/round-002-disassembly.md for concrete redirect questions.
Broad artificial terminal visits/routes retained; no unchanged replay performed.
Toolkit issues NES-DOC-001/NES-OBS-002 remain in analysis/toolkit-issues.md; no new
toolkit issues found this pass. No toolkit/ROM edits, deletion, archive or publish.

| Physical 8KiB bank | Code | Data/Image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1368 | 6792 | 8160 | 32 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 421 | 7211 | 7632 | 560 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1186 | 6914 | 8100 | 92 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 594 | 7310 | 7904 | 288 |
| 24 | 2329 | 5805 | 8134 | 58 |
| 25 | 0 | 8160 | 8160 | 32 |
| 26 | 6144 | 1270 | 7414 | 778 |
| 27 | 6362 | 702 | 7064 | 1128 |
| 28 | 6361 | 904 | 7265 | 927 |
| 29 | 7096 | 797 | 7893 | 299 |
| 30 | 7362 | 248 | 7610 | 582 |
| 31 | 4528 | 2815 | 7343 | 849 |

## Round002 commentary completed

Current-source readability complete on round-002-final, unchanged map/state
8b9f176e93f6c9b137dee1440e49653ab5916fe0e1d2380cc255db2385825b65.
All2054 scaffold entries/1819 compact units reviewed; no readability backlog.
Fresh byte-identical verify and reading refresh passed; audit intentionally pending.
Classification remains97.85%,5625 unknown. See handoffs/round-002-commentary.md;
bounded structural questions in round-2/commentary-redirect-questions.md. Prior
commentary preserved at round-2/commentary-round-001. Toolkit DOC003 recorded;
no toolkit/ROM/map/classification edits or publication/cleanup/archive work.


Round002 redirect-a: configured static inspection of201 residual intervals/5625 bytes; coherent scene/grapple/fixed continuation batch and independent resource review propose916 Code+4426 Data=5342 bytes, accepted0. Conditional residual283. Primary analysis/redirect/round-002-a/research.md, proposal.json and exact residual-ledger.json. Ready R002-REDIRECT-CODE/DATA for independent disassembly. No classification/source/ROM/toolkit changes; not terminal exhaustion.


## Round003 disassembly completed — round-003-classified

ROM classification: 99.89% (261877 / 262160 bytes classified; 283 bytes unknown).

Accepted R002-REDIRECT-CODE916 bytes and R002-REDIRECT-DATA4426 bytes Static after independent configured decode/stack/mapper/inline barriers and every generic-data boundary review. Fresh412 instructions, all external edges on accepted instruction grids. Full connected scene/grapple/initializer/helper batch retained; no proposal rejection.24 exact Unknown intervals remain; counts.json and unknown-ranges.json in analysis/evidence/round-003. Header16 classified separately; data/other includes image/pointer/palette roles. No new execution coverage or natural reachability claim. Reused prior artificial broad gameplay and terminal evidence; no unchanged replay.

Evidence/rejection scope/next frontier: analysis/evidence/round-003/review.md; handoff analysis/handoffs/round-003-disassembly.md. map.validate passed; fresh save/export and byte-identical checkpoint verification analysis/round-003-classified-verification.json. Current map/stateSHA29e4403b8b27d1c3ae75a0aec4c7214f1191a434339eca88ecdb9a1fb51359c9. round-003-final is preserved unchanged preacceptance verification, not current. All prior commentary preserved; full current-source commentary next. No ROM/toolkit/source edits, publication/archive/deletion. No new toolkit issues found; analysis/toolkit-issues.md retains two confirmed documentation bugs and one provenance bug, none blocking.

| Bank | Code | Data/other | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1368 | 6820 | 8188 | 4 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 421 | 7767 | 8188 | 4 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1186 | 7002 | 8188 | 4 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 594 | 7585 | 8179 | 13 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 0 | 8188 | 8188 | 4 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6587 | 1601 | 8188 | 4 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7107 | 1077 | 8184 | 8 |
| 30 | 7527 | 531 | 8058 | 134 |
| 31 | 4622 | 3470 | 8092 | 100 |


## Round004 disassembly completed — round-004-classified

ROM classification: 99.92% (261947 / 262160 bytes classified; 213 bytes unknown).

Accepted R003-RESET-ALIASES28, R003-HEADER-PUBLISHER21 and R003-TYPE-FILTER-SUFFIX21 Static Code bytes after independent fresh decoder, mapper/read/write/next-fetch and consumer/branch boundary review. Natural entry remains unresolved; no new execution coverage. Exact213 Unknown/17 intervals. Evidence: analysis/evidence/round-004/review.md, fresh-decode.json, target-grids.json, counts.json and unknown-ranges.json. map.validate passed; saved/exported map SHA ef40b6c7a98f2ad5f37da8693564ee177472e03a8869dde0d795f711d54331c7; fresh checkpoint byte-identical (analysis/round-004-classified-verification.json). Header16 classified separately. Full current-source commentary remains next; previous commentary unchanged. Broad prior artificial progression/terminal evidence reused, no changed replay needed. Incomplete; unresolved frontier remains renderer59, counter50, arithmetic64, wrapper/vector14, trap2 and isolated fragments24. No concrete connected extension omitted; local contradictions and ambiguity documented in review. No new toolkit issues found; analysis/toolkit-issues.md preserves two documentation bugs/one provenance bug, none blocking. No ROM/toolkit/source edits, archive/deletion/publication.

| Bank | Code | Data/other | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7548 | 531 | 8079 | 113 |
| 31 | 4643 | 3470 | 8113 | 79 |


## Round004 commentary complete

Complete current-source readability for checkpoint round-004-classified. The full previous commentary is preserved at analysis/round-4/commentary-round-003, including its frozen baseline, source, assets, review credit and historical records. Prepared a new byte-identical baseline only after preservation. Physical-boundary checks carry 2,081 exact valid region reviews and all 1,992 earlier units; reviewed nine new code units and two resized opaque boundaries. The current full-ROM scaffold has 2,092 reviewed spans and 2,001 reviewed units: 1,000 code, 1,059 data, 16 image and 17 explained unknown spans. No pending current-source readability remains; editorial_audit stays pending for final acceptance after redirect.

Added explicit upper-alias reset entry/mapper-transition comments for seven four-byte prefixes, a complete 34-byte horizontal upload-header contract, and the type-exclusion/deactivation suffix contract. All horizontal queue operands now share descriptive exact-value RAM aliases across producers, consumer and readiness checks. The scan decrement/retry branch now uses its own descriptive scan label. Updated reset footer prose, the old deactivation comment that incorrectly implied its continuation was missing, and the flash/OAM-cursor field description. Reconciled affected source locations and review summaries. Full source inventory checks found no code span lacking an explanation and no numeric control targets. Earlier sound data/actor/dispatch explanations and bank contexts were retained, without cosmetic rewriting.

The isolated reset aliases and publisher have static contracts, not observed natural callers. The incoming equality flag at physical $3E012 is unresolved across opaque $E000/$E001; the surrounding scan is not claimed to form a functioning loop. The 59-byte renderer prefix, counter/arithmetic fragments, BC00 conflict and isolated remnants retain their documented ambiguity. Remaining whole-ROM classification is 213 unknown bytes / 17 intervals; no classification edits or new execution claims were made. Natural final shot and full timed credits remain unverified. There are no deferred readability ranges.

Fresh configured-toolkit verification passed byte-identical after the completed source batch and again at handoff; reading-after.asm was refreshed. Exact hashes/counts, preservation checks and verification are in analysis/round-4/commentary-completion.json; physical carry boundaries are in carry-report.json. No final audit, map/state/ROM/toolkit edits, cleanup/deletion, publication or archival acceptance occurred. Next phase: redirect.


## Round005 disassembly completed — round-005-classified

ROM classification: 99.94% (262012 / 262160 bytes classified; 148 bytes unknown).

Accepted R004-PRESERVED-TILE-ATTRIBUTE-FRAGMENT48 and R004-PRESERVED-SHIFT48-LOOP17 as Static Code after independent fresh decode, exact branch and closed-loop review; natural reachability unresolved. Full evidence: analysis/evidence/round-005/review.md, fresh-decode.json, target-grids.json, counts.json and unknown-ranges.json. Header16 separately classified. map.validate passed; saved/exported map SHA92909a271b71405418235187a087b923aa6930b0c5ffd9260d7d70f9f7d2df85. Fresh checkpoint byte-identical: analysis/round-005-classified-verification.json. Exact complement148/19intervals. Prior commentary preserved unchanged; full current-source commentary next. Reused broad artificial terminal milestone, no changed replay. Remaining renderer11,counter50,arithmetic47,wrapper/vector14,trap2,isolated24. No supported connected extension omitted. Incomplete; structural reachability and final editorial gates remain separate. No new toolkit issues; analysis/toolkit-issues.md preserves three prior nonblocking findings. No toolkit/ROM/commentary source edits, archive, deletion or publication.

| Bank | Code | Data/other | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7596 | 531 | 8127 | 65 |
| 31 | 4660 | 3470 | 8130 | 62 |


## Round006 — independent arithmetic disassembly pass

ROM classification: 99.96% (262055 / 262160 bytes classified; 105 bytes unknown).

Accepted43 Static Code bytes: physical3E9C2+21 trial24bit subtraction and3E9D9+22 inverse register commit/carry addition, fixed slots30/31. R005-TRIAL-SUB24 and R005-COMMIT-ADD24 verified. Evidence: analysis/evidence/round-006/review.md and fresh-decode.json. Combined accepted shift byte order, borrow chain and inverse stores discriminate intact arithmetic operations from numeric data; damaged enclosing routine does not refute local role. Natural reachability unresolved; no new execution claim. BCC3E9D7+2→3E9F3 and danglingSTA3E9EF+1 remain unknown. Other prior candidate-specific residual review reused without new supported continuation.

map.validate passed; internal state saved, maps/current.json exported. Checkpoint round-006-classified byte-identical, mapSHA256 d85801c366f073fc9b23ff7c701138408e723560eb47795c6525eb68718b5312; analysis/round-006-classified-verification.json. ROM, prior regions and commentary source preserved (preservation.json). No toolkit edits/new issues; existing analysis/toolkit-issues.md remains applicable. Broad gameplay evidence/checklist unchanged; no changed RAM/terminal contract warranted replay. Natural final cockpit/credits unresolved.

Incomplete:105 Unknown bytes in20intervals. Exact physical residuals: 2E810+9, 34231+3, 34DFC+1, 359D3+1, 38AF8+1, 39319+1, 39EDE+1, 3B612+4, 3D62A+4, 3D65E+7, 3D67E+50, 3D7E5+3, 3DFD3+1, 3E010+2, 3E99A+11, 3E9B0+1, 3E9D7+2, 3E9EF+1, 3F448+1, 3F9E5+1. Counts include16classified header bytes. Full current-source commentary and redirect next; prior commentary unchanged and new code not yet editorially reviewed.

| Bank | Code | Data/other | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7596 | 531 | 8127 | 65 |
| 31 | 4703 | 3470 | 8173 | 19 |


## Round007 — independent fragment/template disassembly pass

ROM classification: 99.98% (262113 / 262160 bytes classified; 47 bytes unknown).

Accepted58 Static Code bytes across all four R006 leads. Independently decoded proposed intervals and accepted anchors, recomputed branches and reviewed primary source templates. Evidence: analysis/evidence/round-007/review.md, fresh-decode.json, anchors.json. Exact arithmetic/renderer targets are accepted JMP9103/INX, correcting prior overbroad operand rejections. Counter actual operand conflict/truncated load excluded. BC00 callee remains Data; acceptance establishes preserved wrapper/vector template only. Natural reachability unresolved, no new execution coverage. Prior regions, ROM and commentary source preserved (preservation.json).

map.validate passed; state saved and current map exported. Checkpoint round-007-classified byte-identical, mapSHA256 37154c863b7df03e73aa39ca73aad64dd18f36fe61f0fef6c30aa5cfb39f8bbc; analysis/round-007-classified-verification.json. Incomplete:47Unknown bytes/18intervals: 2E810+9, 34231+3, 34DFC+1, 359D3+1, 38AF8+1, 39319+1, 39EDE+1, 3B612+4, 3D62A+4, 3D67E+12, 3D6A5+2, 3D6AF+1, 3DFD3+1, 3E010+2, 3E9B0+1, 3E9EF+1, 3F448+1, 3F9E5+1. Counts include16classified header bytes. New code awaits full current-source commentary, prior commentary unchanged. Broad progression/artifical terminal evidence unchanged; no new gameplay RAM contract warrants replay; natural final cockpit/credits unproved. Existing toolkit report has three confirmed issues (two documentation bugs, one provenance bug), all worked around, no blockers/new findings; see analysis/toolkit-issues.md. No toolkit edits, publishing, archive, deletion or cleanup.

| Bank | Code | Data/other | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7641 | 531 | 8172 | 20 |
| 31 | 4716 | 3470 | 8186 | 6 |


## Round008 disassembly

ROM classification: 99.98% (262115 / 262160 bytes classified; 45 bytes unknown).

Accepted 2 Static Code bytes, physical $3D6A5+2, R007-COUNTER-CONDITIONAL-FRAGMENT. Independent exact decode, accepted field/flag boundaries, 256-value wrap arithmetic and signed target review support preserved conditional structure only. Actual taken physical $3D6B5 remains accepted JMP operand; no functioning helper/target code/natural caller asserted. Evidence: analysis/evidence/round-008/review.md, fresh-decode.json, anchors.json, finite-byte-check.json. Connected scope yields no further supported extension; prior residual-specific review retained. Broad artificial progression and terminal visits retained; natural cockpit defeat/credits unproved, no changed runtime contract or replay.

Internal-store unique-byte complement: 45 unknown bytes in 17 intervals, analysis/evidence/round-008/unknown-ranges.json. Counts including all 32 physical banks: analysis/evidence/round-008/counts.json. Header 16 classified bytes. Checkpoint round-008-classified validates and rebuilds byte-identically: analysis/round-008-classified-verification.json, map SHA256 e4bde7adb3fbc8ffc808ca84fa771aa2a1c073f2164d1e2ba68fca20268313ed. Input ROM and prior commentary source/symbol hashes preserved, preservation.json. No toolkit edits or new toolkit findings; existing issues remain in [toolkit-issues.md](toolkit-issues.md). Incomplete; next full current-source commentary then redirect.

| Physical 8 KiB bank | Code | Other classified | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7643 | 531 | 8174 | 18 |
| 31 | 4716 | 3470 | 8186 | 6 |


## Round009 disassembly

ROM classification: 99.99% (262127 / 262160 bytes classified; 33 bytes unknown).

Accepted R008-RELOCATED-SCROLL-DISPATCH:6 Static Code bytes physical $3D67E+6, CPU D66E fixed slots[unknown,unknown,30,31], and6 Static Data bytes $3D684+6. Independently verified all three inline word relocations and all four downstream call relocations ($61E), exact accepted template/AF contract and packed-field operation boundaries. Fresh evidence: analysis/evidence/round-009/review.md, fresh-decode.json, anchors.json, relocation-check.json. Actual E000 BRK0035 and overwritten third target remain broken functionality; no natural-use or repaired callee claim. Connected scope already classified; unsupported residuals remain localized. Broad artificial later-game and terminal visits preserved; natural cockpit victory/full credits unproved, no changed contract or replay.

Fresh internal-store unique-byte complement:33 bytes/16 intervals; all32 per-bank counts and exact residuals in analysis/evidence/round-009/counts.json and unknown-ranges.json. Header16 classified. Fresh map.validate passed, analysis/evidence/round-009/validation.json; checkpoint round-009-classified rebuilds byte-identically, analysis/round-009-classified-verification.json. Map SHA256 16e473e723b5f85e8a8243e6dfbf27befa49aca77819a1f3f6107c0df1c1a18d. ROM and prior commentary source/symbol hashes unchanged (preservation.json). No new toolkit findings; existing [toolkit-issues.md](toolkit-issues.md) preserved. Incomplete; next full current-source commentary then redirect. No toolkit/source edits, archive, cleanup/deletion, publish or subagents.

| Physical 8 KiB bank | Code | Other classified | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7649 | 537 | 8186 | 6 |
| 31 | 4716 | 3470 | 8186 | 6 |


## Round010 disassembly

ROM classification: 99.99% (262130 / 262160 bytes classified; 30 bytes unknown).

Accepted R009-RELOCATED-RENDERER-TILE-BASE:3 Static Code bytes physical $3D62B+3, CPU $D61B fixed slots[unknown,unknown,30,31]. Independently verified accepted original instruction alignment, exact88-byte copy, three table and four helper relocation anchors (+$61E), fresh decode and dataflow into accepted paired STA $034C,X tile producer. Orphan $3D62A remains Unknown; no missing B1 repair, natural caller or whole-function claim. Connected continuation already classified. Evidence: analysis/evidence/round-010/review.md, fresh-decode.json, anchors.json, independent-check.json. Broad gameplay/scenarios preserved; no changed progression contract supports replay.

Fresh internal-store complement:30 bytes/16 intervals; exact residuals and all32 per-bank counts in analysis/evidence/round-010/counts.json and unknown-ranges.json. Header16 classified. map.validate passed, validation.json; checkpoint round-010-classified byte-identical, analysis/round-010-classified-verification.json. Map SHA256 8587a03323dec7a53bd828f783f452d69e83d16170550b999fa55a88dffc3480. ROM and commentary source/symbols unchanged (preservation.json). No new toolkit issues; existing [toolkit-issues.md](toolkit-issues.md) preserved. Incomplete; next full current-source commentary then redirect. No toolkit/source edits, archive, cleanup/deletion, publication or subagents.

| Physical 8 KiB bank | Code | Other classified | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7652 | 537 | 8189 | 3 |
| 31 | 4716 | 3470 | 8186 | 6 |


## Round011 disassembly

ROM classification: 99.99% (262132 / 262160 bytes classified; 28 bytes unknown).

Accepted R010-BRK-RTI-SKIP: Static Code1 physical $3E010 (CPU $E000), Static Data1 BRK-discarded padding $3E011. Independently verified actual FFFE vector B2C0 -> accepted physical $3C0C2 RTI40 with fixed slots[unknown,unknown,30,31]. Hardware pushes E002/P and RTI restores PC/SP, architectural comparison flags and leaves A/X/Y unchanged; three residual stack writes and stacked B marker are explicit. Contract excludes NMI hijack/intervening interrupts, with I=1 sufficient for maskable IRQ exclusion; no natural caller, execution or timing-transparency claim. Decoder BRK length1 verified. Connected E002 filter/deactivation/scan and RTI already classified. Evidence: analysis/evidence/round-011/review.md, independent-check.json and fresh-decode.json.

Fresh internal-store complement:28 bytes/15 intervals; all32 per-bank counts and exact residuals in analysis/evidence/round-011/counts.json and unknown-ranges.json. Header16 classified. map.validate passed (validation.json); fresh checkpoint round-011-classified byte-identical (analysis/round-011-classified-verification.json). Map/state SHA256 094b219910335a9fe0bdcc2d53f8e4fc87edbf0ea7fd660c4c3529fba993b7d8. All existing regions, input ROM and prior commentary source/symbols preserved (preservation.json). Broad gameplay/scenarios unchanged: no changed RAM/entry/progression contract supports useful replay; prior artificial terminal signature is not natural cockpit victory/full timed credits. No new toolkit issues; existing [toolkit-issues.md](toolkit-issues.md) retained. Incomplete; next full current-source commentary then redirect. No toolkit/ROM/source edits, archive, deletion/cleanup, publication or subagents.

| Physical 8 KiB bank | Code | Other classified | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7652 | 537 | 8189 | 3 |
| 31 | 4717 | 3471 | 8188 | 4 |


## Round011 commentary

Current-source readability complete: 2107 reviewed spans/2018 units, prior2016 units retained; Code1 BRK/Data1 padding and connected scan/RTI/preserved E000 callee explanations updated. Fresh byte-identical verification and reading refresh pass. Classification incomplete:262132/262160,28 Unknown/15 intervals. Final editorial audit pending after redirect. See analysis/handoffs/round-011-commentary.md and analysis/round-11/commentary-completion.json for exact scope/hashes/limits. No new toolkit issues; prior three confirmed issues remain unblocked. No ROM/map/state/toolkit edits, archive, cleanup/deletion, publication or subagents.

## Coordinator terminal review — final acceptance pending

Accepted round011 redirect terminal-incomplete rationale after primary review;
see analysis/coordinator-rejection-review.md. Earlier round010 terminal recommendation
was returned for the actual IRQ/BRK-vector RTI correction and superseded by accepted
round011 Code1/Data1. Current checkpoint round-011-classified, map/state
094b219910335a9fe0bdcc2d53f8e4fc87edbf0ea7fd660c4c3529fba993b7d8.
ROM classification: 99.99% (262132 / 262160 bytes classified; 28 bytes unknown).
No ready leads/productive research frontier remains; whole-ROM classification
incomplete. Current-source readability complete; targeted final editorial acceptance,
archive-options.json and coordinator completion audit remain pending. No archive,
cleanup, publication or toolkit changes authorized/performed.

## Coordinator phase/checkpoint index

Each round used separate sequential disassembly, commentary and redirect worker contexts; no competing phase writers. Context rollovers resumed the same commentary/redirect phase. Every disassembly classification percentage was printed to the user before commentary. Durable handoffs include scope, checkpoint, verification and unresolved work. Valid prior commentary was preserved and carried by physical identity, not overwritten.

| Round | Disassembly input → verified output | Classified / unknown | Consumed redirect leads | Commentary gate | Redirect output |
|---:|---|---:|---|---|---|
| 1 | initialized; verified baseline → [round-001-final-tables](handoffs/round-001-disassembly.md) (valid; byte-identical) | 161031 / 101129 | startup/static/gameplay; initial R001 leads | [complete current source](handoffs/round-001-commentary.md); fresh verification/read | [R001-REDIRECT-CODE, R001-REDIRECT-DATA](handoffs/round-001-redirect.md) |
| 2 | round-001-final-tables → [round-002-final](handoffs/round-002-disassembly.md) (valid; byte-identical) | 256535 / 5625 | R001-REDIRECT-CODE, R001-REDIRECT-DATA | [complete current source](handoffs/round-002-commentary.md); fresh verification/read | [R002-REDIRECT-CODE, R002-REDIRECT-DATA](handoffs/round-002-redirect.md) |
| 3 | round-002-final → [round-003-classified](handoffs/round-003-disassembly.md) (valid; byte-identical) | 261877 / 283 | R002-REDIRECT-CODE, R002-REDIRECT-DATA | [complete current source](handoffs/round-003-commentary.md); fresh verification/read | [R003-RESET-ALIASES, R003-HEADER-PUBLISHER, R003-TYPE-FILTER-SUFFIX](handoffs/round-003-redirect.md) |
| 4 | round-003-classified → [round-004-classified](handoffs/round-004-disassembly.md) (valid; byte-identical) | 261947 / 213 | R003-RESET-ALIASES, R003-HEADER-PUBLISHER, R003-TYPE-FILTER-SUFFIX | [complete current source](handoffs/round-004-commentary.md); fresh verification/read | [R004-PRESERVED-TILE-ATTRIBUTE-FRAGMENT, R004-PRESERVED-SHIFT48-LOOP](handoffs/round-004-redirect.md) |
| 5 | round-004-classified → [round-005-classified](handoffs/round-005-disassembly.md) (valid; byte-identical) | 262012 / 148 | R004-PRESERVED-TILE-ATTRIBUTE-FRAGMENT, R004-PRESERVED-SHIFT48-LOOP | [complete current source](handoffs/round-005-commentary.md); fresh verification/read | [R005-TRIAL-SUB24, R005-COMMIT-ADD24](handoffs/round-005-redirect.md) |
| 6 | round-005-classified → [round-006-classified](handoffs/round-006-disassembly.md) (valid; byte-identical) | 262055 / 105 | R005-TRIAL-SUB24, R005-COMMIT-ADD24 | [complete current source](handoffs/round-006-commentary.md); fresh verification/read | [R006-TRIAL-BRANCH, R006-COUNTER-FRAGMENTS, R006-RENDERER-FOOTER, R006-PRESERVED-BC00-WRAPPER](handoffs/round-006-redirect.md) |
| 7 | round-006-classified → [round-007-classified](handoffs/round-007-disassembly.md) (valid; byte-identical) | 262113 / 47 | R006-TRIAL-BRANCH, R006-COUNTER-FRAGMENTS, R006-RENDERER-FOOTER, R006-PRESERVED-BC00-WRAPPER | [complete current source](handoffs/round-007-commentary.md); fresh verification/read | [R007-COUNTER-CONDITIONAL-FRAGMENT](handoffs/round-007-redirect.md) |
| 8 | round-007-classified → [round-008-classified](handoffs/round-008-disassembly.md) (valid; byte-identical) | 262115 / 45 | R007-COUNTER-CONDITIONAL-FRAGMENT | [complete current source](handoffs/round-008-commentary.md); fresh verification/read | [R008-RELOCATED-SCROLL-DISPATCH](handoffs/round-008-redirect.md) |
| 9 | round-008-classified → [round-009-classified](handoffs/round-009-disassembly.md) (valid; byte-identical) | 262127 / 33 | R008-RELOCATED-SCROLL-DISPATCH | [complete current source](handoffs/round-009-commentary.md); fresh verification/read | [R009-RELOCATED-RENDERER-TILE-BASE](handoffs/round-009-redirect.md) |
| 10 | round-009-classified → [round-010-classified](handoffs/round-010-disassembly.md) (valid; byte-identical) | 262130 / 30 | R009-RELOCATED-RENDERER-TILE-BASE | [complete current source](handoffs/round-010-commentary.md); fresh verification/read | [R010-BRK-RTI-SKIP](handoffs/round-010-redirect.md) |
| 11 | round-010-classified → [round-011-classified](handoffs/round-011-disassembly.md) (valid; byte-identical) | 262132 / 28 | R010-BRK-RTI-SKIP | [complete current source](handoffs/round-011-commentary.md); fresh verification/read | [no ready leads; terminal incomplete reviewed](handoffs/round-011-redirect.md) |

Round010 redirect first recommended terminal incomplete, but coordinator primary IRQ-vector/RTI review reopened the same phase; R010-BRK-RTI-SKIP then passed independent disassembly and commentary. Original recommendation and correction remain preserved. Round011 terminal reasoning is accepted separately in analysis/coordinator-rejection-review.md; final acceptance still pending here.
Historical R001-RESOURCE-RESIDUALS broad blocker is now explicitly superseded by accepted batches/current exact complement. R001-FINAL-SHOT remains a blocked, resumable gameplay hypothesis with corrected room identity and explicit new-evidence prerequisite; natural victory is not reported successful. This is not an outstanding productive classification phase.


## Final coordinator handoff — orchestration complete, ROM incomplete

All required work for this orchestration goal is complete. Final phase is terminal
handoff after coordinator rejection review and targeted editorial acceptance.
ROM classification: 99.99% (262132 / 262160 bytes classified; 28 bytes unknown).
Final checkpoint round-011-classified, map/state SHA256
094b219910335a9fe0bdcc2d53f8e4fc87edbf0ea7fd660c4c3529fba993b7d8.
Fresh editorial audit:2107 spans/2018 units, byte-identical; internal store and
portable map validations pass. Current reading copy matches final source exactly.
Source SHA af3fb0a1382c760c80bf0b1b50881e3dd7b5937a7934074a7e7094d5f2785502.

Archive-options.json selects91 additional final Markdown documents and explicitly
omits30 disposable generated instruction copies; all131 analysis/commentary Markdown
files accounted for (10 are recognized by the archive contract). All final results,
phase handoffs, research and rejection history selected. Manifest contract verified
without archive execution. Coordinator verification passed18 concrete checks:
analysis/coordinator-verification.json. Main result: analysis/final-handoff.md;
requirement audit: analysis/coordinator-completion-audit.md. No ready structural
leads, interrupted productive phase, or pending editorial task remains. Historical
blocked natural-victory lead remains resumable and explicitly unproved; ROM
classification is NOT complete. No archive, cleanup/deletion, publish or toolkit
change performed. Workspace/evidence remain local and intact.
