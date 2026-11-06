# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 disassembly
ROM classification: 63.72% (167060 / 262160 bytes classified; 95100 bytes unknown).

Classified startup vector closure and observed supported routes; evidence in round-001/discovery.json, candidate-listing.txt, runtime-listing*.txt and static-frontier-*.json/txt. Maximal uniform PRG runs (uniform-candidates.json) underwent neighboring-byte/reference/code-conflict inspection; generic Data only, unknown semantics accepted. Header/CHR exact extents derive validated cartridge layout; main dispatch57-byte structure uses actual stride3 bank/target consumer plus independent code boundaries. Handlers and static code claims separate from natural selection.

| Bank | Code | Data/image/text | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 2010 | 0 | 6182 |
| PRG01 | 396 | 0 | 7796 |
| PRG02 | 372 | 359 | 7461 |
| PRG03 | 218 | 303 | 7671 |
| PRG04 | 0 | 0 | 8192 |
| PRG05 | 0 | 4093 | 4099 |
| PRG06 | 0 | 37 | 8155 |
| PRG07 | 0 | 2864 | 5328 |
| PRG08 | 0 | 0 | 8192 |
| PRG09 | 0 | 4466 | 3726 |
| PRG10 | 555 | 2040 | 5597 |
| PRG11 | 0 | 8192 | 0 |
| PRG12 | 1249 | 103 | 6840 |
| PRG13 | 0 | 978 | 7214 |
| PRG14 | 3045 | 309 | 4838 |
| PRG15 | 4022 | 361 | 3809 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

All current classifications validated and pass-001 byte-identical; exact residuals round-001/unknown-complement.json. Gameplay routes/snapshots/patches in scenarios.json; THE END confirmed ending-final.png. First Dpad/Start stage6 menu attempt did not change selection; replaced using independently decoded Select/index contract, not retried unchanged. Early budget/timeouts preserved state; long-lived environment process termination recovered from snapshots. No generic whole-gap or pointer-only code annotations.

Ready research: indexed main handlers and call-bank return contracts, cutscene state dispatch at physical0x5D40 and audio command dispatch0x18F47; physical0x1E0A4 main indirect destination now structurally resolved by table but additional handler bodies need safe context reasoning. Current static-frontier traversal retains mapper/JSR barriers; concrete continued research belongs to redirect. Not a terminal rejection/exhaustion review. No toolkit issues found; see toolkit-issues.md.

## Round001 redirect completed
[Attempt findings](redirect/round001-20261106/findings.md) and [combined handoff](handoffs/round-001-redirect.md): bank-return/indirect/actor/text/resource connected and independent batch proposes55183 unique unknown bytes for independent review. Zero bytes accepted here; input pass001 unchanged. Mixed/nonlocal special resource pointers localized instead of rejecting other containers; missing semantics do not block genericData. Explicit selector and MainPrgPair preconditions retained. No terminal exhaustion or runtime reachability claims. Continue with combined disassembly review; preserve completed attempt records.

## Round002 disassembly completed — incomplete ROM

ROM classification: 84.77% (222243 / 262160 bytes classified; 39917 bytes unknown).

Accepted55183 unique formerly unknown bytes:27632Code and27551Data,399 additional regions, all Static. Fresh [pass-002 verification](pass-002-verification.json) ok=true/byte_identical=true; internal map validated. [maps/pass-002.json](../maps/pass-002.json) and maps/current.json SHA256aa5bc54ceebed0be0c35d2a9012b6b6de91dcd9345a4927ff8db4850addee119; generated/pass-002 is separate from preserved commentary/source and all frozen/previous checkpoints. ROM SHA256 unchanged b9ab5fbc92df5f496af543711adcdead16c02cbd08aca69f5f3a49d593a25c2f.

Independent review: [decisions](round-002/review-decisions.md), [11926-instruction/305-group mechanical audit](round-002/independent-review.json), [fresh consumers](round-002/consumer-review.txt), [accepted exact regions](round-002/accepted-regions.json). Every candidate effective view and physical boundary freshly checked; coherent indexed consumers/routine contracts provide structural support beyond decoding. Audio continuations explicitly require RAM08 matching caller main pair. Fixed14/15 views use C000/E000. Partial actor/scene selector slices do not claim full domains/natural selection. Resource18 descriptors, all layout cells and inclusive spawn ends independently checked; chain exceptions13→15 (222-byte gap) and15→16 (alternate/shared row order) invalidate universal chain model only. Own bounded resource spans accepted; gap unchanged. Special4/5 shared86E5 excluded.

| Bank | Code | Data/image/text | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6454 | 52 | 1686 |
| PRG01 | 6359 | 32 | 1801 |
| PRG02 | 988 | 381 | 6823 |
| PRG03 | 5964 | 493 | 1735 |
| PRG04 | 5359 | 1447 | 1386 |
| PRG05 | 3271 | 4093 | 828 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 |
| PRG09 | 0 | 7973 | 219 |
| PRG10 | 740 | 2040 | 5412 |
| PRG11 | 0 | 8192 | 0 |
| PRG12 | 2017 | 143 | 6032 |
| PRG13 | 0 | 978 | 7214 |
| PRG14 | 3722 | 517 | 3953 |
| PRG15 | 4625 | 739 | 2828 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

[Exact residual complement](round-002/unknown-complement.json):331 intervals; [counts](round-002/counts.json), [summary](round-002/summary.json). Code39499 bytes; structural evidence present in all2217 classified regions (100% annotation coverage). Runtime coverage unchanged:4976 unique physical fetched starts,4198 natural and3086 artificial overlapping; no execution claim added. Representative Stage2/4/6 mechanics and THE END already visited (scenarios.json); no unchanged replay needed. Current-source readability was verified for pass001 only; new map requires full current-source commentary next. Original source hashes verified unchanged, recorded in round-002/source-preservation.json; frozen baseline/before and original review retained.

All five ready leads verified on their exact bounded proposals; histories retained in leads.json. Remaining questions are mapper-shadow/caller preconditions, unbounded indirect selector states, stack-reset transitions and mixed residuals, not proof of dead code or terminal exhaustion. No obvious connected accepted proposal left pending; refreshed map should guide next redirect after commentary. No toolkit issues found; [persistent report](toolkit-issues.md). No toolkit edits, input mutation, archive, cleanup, deletion or publication.

## Round002 redirect (2026-11-06), completed supported batch
See `redirect/round002-20261106/findings.md`, final-audit.json and `handoffs/round-002-redirect.md`. Fresh current-source direct edges and source-supported conditional normal-return witnesses expose3504 code bytes; explicit CA75 mapping opens all five bank12 scene handlers; secondary-action/HUD dispatches followed coherently. Independent consumer/boundary/pattern review proposes32239 Data bytes including generic containers without invented schemas.35743 unique proposed,0 accepted;39917 current unknown,4174 projected. FE sprite-skip terminology and CA87 restoration context corrected locally; mixed7EA0 split by coherent routine/records,1E759 mixed suffix excluded. No ROM/map/source edits. Remaining concrete contract/encoding barriers retained in exact331-interval ledger; no terminal exhaustion or repeated completed research. Prior attempt evidence immutable.

## Round003 disassembly completed — incomplete ROM

ROM classification: 98.41% (257986 / 262160 bytes classified; 4174 bytes unknown).

Accepted35743 formerly unknown bytes:3504Code/32239Data across203 additional Static regions. [Fresh pass003 verification](pass-003-verification.json) ok=true/byte_identical=true; internal map validated, state/current/checkpoint byte-identical SHA25612504fb181283bedd0308ead6ddd422844df02882751d05249744e47c8b82ea5. ROM unchanged b9ab5fbc92df5f496af543711adcdead16c02cbd08aca69f5f3a49d593a25c2f. [Review decisions](round-003/review-decisions.md), [independent audit](round-003/independent-review.json), [fresh consumers](round-003/consumer-review.txt), [accepted regions](round-003/accepted-regions.json).1612 fresh instruction starts/80 groups; pointer words/countXY endpoints and physical identities verified, zero errors.

CA87 remaps saved12; CA8A separately requires restored0/1. Audio returns conditional on08 matching caller; CD14 restores2/3 specifically. Actor5/5/6, active-secondary3 andHUD9 slices retain bounded claims. Scene12 finite0..4 proof follows state writers/outer5 exit. Mixed7EA0 independently coherent16Data/43Code/16Data; no live caller requirement. Generic audio/text/scene/sprite/resource noncode classification does not claim exact full schemas; FE tile skip retained.1E759 only516Data;16-byte suffix unknown. All four ready leads verified on exact supported proposals; earlier history preserved.

| Bank | Code | Data/image/text | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6556 | 800 | 836 |
| PRG01 | 6430 | 1561 | 201 |
| PRG02 | 1307 | 6839 | 46 |
| PRG03 | 6201 | 1991 | 0 |
| PRG04 | 5359 | 2457 | 376 |
| PRG05 | 3271 | 4770 | 151 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 |
| PRG09 | 0 | 8192 | 0 |
| PRG10 | 740 | 7237 | 215 |
| PRG11 | 0 | 8192 | 0 |
| PRG12 | 2678 | 5072 | 442 |
| PRG13 | 0 | 8192 | 0 |
| PRG14 | 5096 | 1479 | 1617 |
| PRG15 | 5365 | 2537 | 290 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

[Exact unknown complement](round-003/unknown-complement.json):179 intervals/4174bytes; [counts](round-003/counts.json), [summary](round-003/summary.json). Code43003bytes;2420 classified regions with structural evidence (100% annotation coverage). Execution coverage unchanged4976 unique physical starts,4198 natural/3086 artificial overlapping; no new runtime claim. Representative Stage2/4/6 mechanics and THE END retained in scenarios.json; no unchanged replay needed. Authored source hashes preserved [audit](round-003/source-preservation.json), frozen/before/review/history unchanged. Current-source readability was verified for pass002 only; full current-source commentary required next.

Remaining4174bytes localized mixed player/actor/menu/fixed-helper fragments have unsupported entry/stack/mapping contexts or ambiguous code/record endpoints; these are questions for refreshed-complement redirect after commentary, not a terminal exhaustion claim. No supported proposal left pending in this batch. No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, ROM mutation, cleanup, deletion, archive or publication.

## Round004 disassembly complete — incomplete ROM

ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).

Rounded100.00% is not completion: physical01CD35..01CD38 remains unknown. Accepted4171 formerly unknown bytes1403Code+2768Data in190 Static regions. All641 proposed instruction views independently fresh-decoded in48groups; exact physical identities, control boundaries, noncode patterns and primary routine/stack/mapping contracts checked. [Review decisions](round-004/review-decisions.md), [audit](round-004/independent-review.json), [fresh consumers](round-004/consumer-review.txt), [accepted slices](round-004/accepted-regions.json). All2420 old regions preserved exactly. Total2610regions,100% evidence annotation coverage; code44406bytes.

Address correction: physical01D557/CPU D547 JSR8000 usesResource4/5 selector0→824F; physical01D55A/CPU D54A JSRMapMain restores0/1 from physical01D55D/CPU D54D onward. Candidate views correct; redirect prose suffixes had mixed physical/CPU spaces. Historical artifacts preserved. IRQ CHR2..5 retainsPRG2/3; audio returns explicit Main shadow08; resource6 tails retain conditions. Partial player5word/3setter and7dispatch-return witnesses do not imply full selectors. Seven24-bit powers10..10^7 endphysical01F180, then11byte history-clear code. C519 increments/clamps20, no loop. Generic data supported by references/neighbors/patterns without invented schemas.

| Bank | Code | Data/image/text | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6862 | 1330 | 0 |
| PRG01 | 6430 | 1762 | 0 |
| PRG02 | 1353 | 6839 | 0 |
| PRG03 | 6201 | 1991 | 0 |
| PRG04 | 5362 | 2830 | 0 |
| PRG05 | 3271 | 4921 | 0 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 |
| PRG09 | 0 | 8192 | 0 |
| PRG10 | 740 | 7452 | 0 |
| PRG11 | 0 | 8192 | 0 |
| PRG12 | 2707 | 5485 | 0 |
| PRG13 | 0 | 8192 | 0 |
| PRG14 | 5869 | 2320 | 3 |
| PRG15 | 5611 | 2581 | 0 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

[Counts](round-004/counts.json), [exact complement](round-004/unknown-complement.json), [summary](round-004/summary.json). State/current/[pass004map](../maps/pass-004.json) byte-identical SHA256d32f95247f3c547c153f2215edb55e3ad99070fb0dea4fe852c0f4952229a5f8; fresh [verification](pass-004-verification.json) ok=true/byte_identical=true. ROM unchanged b9ab5fbc92df5f496af543711adcdead16c02cbd08aca69f5f3a49d593a25c2f. [Source hashes](round-004/source-preservation.json) unchanged; frozen/before/review/history retained. Full current-source readability is verified for previous pass003 only; commentary must cover newly classified4171bytes, then redirect reviews strongest residual reasoning. No terminal exhaustion or final editorial claim.

Residual3bytes A001FF atphysical01CD35..01CD38/CPUCD25..CD28 have no fallthrough from preceding JMPMapMain and no known direct reference; LDY1+unofficial instruction versus short data remains ambiguous. All three R003 ready leads verified for exact combined batch; no unrestricted context/selector claim. No new runtime evidence:4976unique physical starts4198natural/3086artificial overlap unchanged; representative Stage2/4/6 mechanics/THE END retained, no unchanged replay. No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, ROM mutation, cleanup, archive, deletion or publication.

## Round004 residual review
See redirect/round004-20261106/rejection-review.md and primary-audit.json. Corrected narrow alternative entries evaluated; no ready lead, three structural unknown bytes retained. Coordinator review pending; no unfinished discriminating experiment.
