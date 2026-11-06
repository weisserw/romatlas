# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round01 — baseline, vectors, graphics banks and initial gameplay
Baseline verified byte-identical; hardware mapper2 UxROM, 128KiB PRG, CHR RAM. Validated vectors resetC008/NMI+IRQFB12, physical1C018/1FB22. 102 static initial instruction candidates retained. Actual title/menu routes plus bounded static successors accepted2206 instructions4503bytes with explicit CPU views; provenance comes from saved actual fetch coverage or static control flow, not valid decoding alone. The discovery response's execution_coverage flag remained false for fetched roots, so saved contexts/coverage independently establish observed provenance.

Inspected complete large banks at4010..1C010 (six16KiB banks) as repeated planar graphics/noncode data, coherent local font/sprite contact sheets, extensive zero/FF runs and no instruction conflicts. Generic Data classification is structurally supported; exact consumer/type subdivision remains for commentary/redirect. At least observed F8A5 descriptor call at1D69C has source8000,dest1000,length1000 in context[2,3,14,15], establishing a4KiB direct graphics upload from4010. Source F8A5 uses source,destination,count 16bit descriptor and writes2007; recorded r01-decode-f8a5.json.

F862/F9B6 consume exactly5 inline bytes and adjust stacked return by5; F8A5 consumes6 and adjusts by6. Enrichment precisely adds inline parameter barriers at supported JSR sites and static continuation roots, without assuming whole-game reachability. See round01-inline-outcomes.json and round01-session-rpc.jsonl. Uniform FF intervals3F47+201,1D9AC+66,1FF99+113 classified generic Data after boundaries/no conflicting execution inspection; no original fill-purpose claim.

Gameplay: title and menu setup natural with corrected array-input contract (NES-001 documents silent per-button mismatch). Route1player/light/lightfirst reaches extensive staged board army introduction, distinct knight/banshee/djinni/sorceress classes visible, though this is not a combat visit. Bounded untraced fast-forward was used only for repeated introduction delays, not claimed as fetch evidence. All snapshots are local. Latest broad progression notes in scenarios.json.

Ready local hypothesis: board is81bytes at RAM044C..049C, row-major9x9. Natural RAM grid matches rendered armies and empties80; fixed initializer atF37D iterates two light values/five80/two dark values until X51, directly writing044C,X (r01-decode-1f380.json). This is concrete enough for bounded artificial endgame/board experiments; changed state alone will not establish code or victory.

## Round01 completion evidence

ROM classification: 83.57% (109551 / 131088 bytes classified; 21537 bytes unknown).

 disassembly pass completed. Physical unique-byte totals: 10739 Code, 98812 Data (including 16-byte header), 21537 unknown. All 5147 classified regions have structural evidence; 5116 Code regions have explicit valid CPU views. Labels/editorial commentary are not complete. Unknown complement is in round01-metrics.json; no raw-export fallback counted.



Per-bank counts and verification are recorded in progress.md and round01-metrics.json. Final checkpoint pass-001-final is byte-identical. Evidence-backed classification remains incomplete; no terminal exhaustion judgment. Additional helper contracts and exact victory/spell state transitions are concrete redirect work. Full non-code-bank contact sheet r01-all-bitmap-banks.ppm inspected all6144 tile-sized records, showing glyphs, title/logo art, sprites and noninstruction screen records; generic Data avoids inventing exact mixed subdivisions. Execution/data conflict audit returned zero.

Ending experiment counterexample only rejects the assumption that replacing dark board cells and forcing selection immediately produces terminal state. It does not reject eventual victory code, board structure or a normal end-turn route. Wizard experiment only rejects the tested same-cell/event/B recipe; wizard/selection callback and spell trigger remain unresolved. No unchanged retry recommended.

Per-bank final accounting (8KiB PRG; header separately16 Data bytes):

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 2983 | 0 | 5209 |
| 1 | 1516 | 206 | 6470 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 0 | 8192 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 3454 | 136 | 4602 |
| 15 | 2786 | 150 | 5256 |

Toolkit pass summary:1 confirmed bug NES-001, no suspected bugs or missing features, no blockers. Preserve toolkit-issues.md.

## Round01 redirect contracts
Completed coherent structural batch from current-source commentary: exactstack-inlinehelperclosures, human/computer/effect/selectiondispatches, AIroute/audiohandlerdomains, independentlysupported PPU/APU/menu/pause/terrainclonefunctions andnoncodedataenvelopes. Authoritativeevidence/result:redirect/r01-contracts/result.md; handoffround01-redirect-handoff.md. Proposed21527uniqueUnknownbytes forindependentdisassemblyreview, currentacceptedUnknown21537unchanged. Potentialresidual10isolatedstub/zeros bytes onlyifallaccepted. Graphicsactor domain0..19 independentlybound20existingDatachunks. Artificialterminalwinner andcompleteteleportmechanicvisited, priorfailedpremises/experimentspreserved. No terminalexhaustionclaim; nextdisassemblyreviewscombinedproposal, fullcurrent-sourcecommentaryandredirectfollow. No new toolkitissues;NES-001nonblocking. ROM/state/map/sourceunchanged.

## Round02 independently accepted connected batch

ROM classification: 99.99% (131078 / 131088 bytes classified; 10 bytes unknown).

Independently reviewed and accepted 21,527 formerly Unknown bytes: 12,383 Code, 380 stack-inline Data and 8,764 other Data. The accepted map now has 23,122 Code bytes and 107,956 Data bytes, including the 16-byte header. All 11,131 regions carry evidence; all Code has explicit validated execution views. Whole-ROM classification remains incomplete because ten bytes in seven exact intervals remain Unknown. No raw export fallback counted.

Fresh review decoded 9,295 proposal views and verified physical/effective banks, all intervals and 14,644 numeric predecessor edges; zero conflicts. 171 call/context checks establish 96 new inline records and exact return widths. Independent hardware/RAM contracts and finite dispatch consumers support static routines separately from natural reachability. Full Data byte/pattern/consumer inspection supports generic envelopes where precise schemas remain unresolved. Findings and localized wording corrections: [findings.md](disassembly/r02-review/findings.md). The APU suffix was split into its correct eight-byte script descriptor; sprite lengths, 960-byte clear, input predicate and DC69 domain claims were narrowed. Twenty already-classified 768-byte actor graphics prefixes were refined using C317 only; C381 independently copies 2048 bytes. No byte-role proposal required rejection.

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 5795 | 2394 | 3 |
| 1 | 3552 | 4639 | 1 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 0 | 8192 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 6698 | 1492 | 2 |
| 15 | 7077 | 1111 | 4 |

Header:16 Data bytes, outside the 8 KiB banks. Exact residuals and physical/CPU identities: [metrics.json](disassembly/r02-review/metrics.json). Unknown intervals (ends exclusive): $00ABF..$00AC1, $00E87, $02481, $1DC0F, $1DC23, $1F08D..$1F090 and $1F9F7. Isolated return/JMP/zero content lacks a supported entry or non-code role; adjacent accepted structure does not settle it. No dead-code or terminal exhaustion claim.

Saved scenarios were reused: representative natural board plus explicitly artificial arena, ending/winner and completed Teleport already satisfy practical progression review; no unchanged gameplay repeated. Composite saved execution is 4,686 distinct ROM instruction starts / 9,753 instruction bytes (execution-metrics.json), separate from structural classification and natural reachability. New annotations remain Static.

State saved to analysis/state.json, current map exported to maps/current.json and validated; fresh pass-002-final generated source and byte-identical verification passed. [Verification](pass-002-final-verification.json), [integrity](disassembly/r02-review/integrity.json). Input hash unchanged. Existing commentary SHA256 remains 1c1a9ecb022eac6839a608c3b801e79d9ee27c503f9f0f0d7733621c51c3efe1; full current-source commentary migration is the next phase, followed by redirect. No final editorial audit performed.

Lead outcomes: D01-STACK-HELPERS and D01-GRAPHICS-FORMATS verified by this review; D01-END and D01-SPELL preserve their successful artificial scenario evidence. Residual role questions remain localized for the next commentary/redirect review; no unsupported inference added.

Toolkit findings: no new toolkit issues found; existing NES-001 is one confirmed documentation/API contract bug, nonblocking with complete buttons arrays. Zero suspected bugs or missing features. See [toolkit-issues.md](toolkit-issues.md). Empty labels in proposal input were normalized to null after an expected schema rejection; successful annotate transactions were recovered into a validated local checkpoint before continuation. This was input normalization, not a toolkit defect. Toolkit unchanged; no publication, archive, cleanup or deletion. Free space about 8 GiB; all cited evidence retained.

## Round02 redirect: consumer correction and residual review
Direct accepted LDY0/sign branch/optionalINY then ADC$8AAF,Y resolves two zero bytes physical$00ABF..$00AC1 as proposedStaticData. Prior no-data-role rejection is overturned by its overlooked consumer; no gameplay replay needed. Remaining8bytes considered separately, including clone endpoint, palette extent, duplicated epilogues and JMP→RTS pair. See analysis/redirect/r02-residual-review/rejection-review.md for combined constraints and competing code/data readings. Map/source distance-array inconsistency and carried stale scope prose have explicit repair recipe. Leads D02-PROJECTILE-Y-ADJUST / D02-DISTANCE-CONSISTENCY ready; no classification changed during redirect. Handoff analysis/round02-redirect-handoff.md.

## Round03 disassembly complete — incomplete ROM

ROM classification: 99.99% (131080 / 131088 bytes classified; 8 bytes unknown).

Accepted 2 new Static Data bytes at $00ABF..$00AC1, ProjectileOriginYAdjustmentByDirection, through exact sign-selected consumer/boundary review. 23122 Code / 107958 Data; header16 Data included. Corrected region11047 label AND stale evidence to ComputerArenaDistanceParameters, preserving bounds and original evidence history (0 byte gain). $0678 derives from $067A+3. [Independent findings](disassembly/r03-review/findings.md), [metrics](disassembly/r03-review/metrics.json).

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 5795 | 2396 | 1 |
| 1 | 3552 | 4639 | 1 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 0 | 8192 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 6698 | 1492 | 2 |
| 15 | 7077 | 1111 | 4 |

Residual ends-exclusive intervals: $00E87..$00E88,$02481..$02482,$1DC0F..$1DC10,$1DC23..$1DC24,$1F08D..$1F090,$1F9F7..$1F9F8. Primary evidence and combined-constraint review retain material Code/Data ambiguity, without a universal live-caller prerequisite. Previous rejection records preserved; no speculative classification, terminal exhaustion or completion claim.

Saved board/arena/ending/Teleport evidence and progression checklist reused. No unchanged gameplay repeated; no new execution observation claimed. State saved and map exported/validated; fresh [pass-003-final verification](pass-003-final-verification.json) confirms byte identity. An unchanged pass-003 checkpoint created after an input-schema rejection is retained as history; final is authoritative. ROM and existing commentary hashes unchanged: [integrity](disassembly/r03-review/integrity.json).

D02-PROJECTILE-Y-ADJUST verified; D02-DISTANCE-CONSISTENCY map work verified, with four stale scope comments and new table/count prose deferred to next full commentary phase. Existing commentary deliberately unmodified. No new toolkit issues found; existing NES-001 nonblocking via complete buttons arrays, see [toolkit-issues.md](toolkit-issues.md). No toolkit edits, publication, archive, cleanup or deletion. About8GiB free. Next full current-source commentary then redirect; final editorial audit pending.
