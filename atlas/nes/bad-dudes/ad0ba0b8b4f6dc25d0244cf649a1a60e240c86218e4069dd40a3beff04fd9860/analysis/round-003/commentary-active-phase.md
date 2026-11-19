### ROUND003 commentary continuation B — incomplete

Batches007–013 complete ALL remaining new bank8/15 Code:54scaffolds2152bytes, including A23-byte suffix (2129new unread bytes);68net meaningful units added. Current2373scaffolds2262reviewed/111pending,2806units. Remaining111Code ranges5399bytes in banks6/7/13/14; no partial scaffolds. Scoped dependency inventory36reviewed/103pending after33additional checks. Fresh byte-identical verify/read and source-relative metadata/partition checks pass0errors. Classification99.54%,1204Unknown unchanged. Resume exact remaining commentary scope from analysis/round-003/commentary-continuation-latest.md, commentary-pending.json and commentary-pending-by-bank-b.json; dependencies in commentary-dependency-candidates-a.json. Final audit pending; no successful handoff or redirect. Writer B stopped writing for next sequential context.


### ROUND003 commentary batch014 (writer C; phase incomplete)

Completed three bank14 raster IRQ continuation scaffolds (253bytes), with five units explaining fixed delays, literal PPU internal-address reads, player nametable selection, two-phase control, shared guard and saved interrupt-frame restoration. Checked DEP109..112 against the classified stage/phase/mode RTS tables and downstream code; removed stale unclassified-table/continuation claims and used explicit CPU aliases for table operands. Current2265reviewed/108pending scaffolds,2811units;5146Codebytes and99dependency candidates remain. Fresh verify byte-identical and reading-after refreshed. Initial link check caught use of physical data labels as CPU aliases; corrected with distinct fixed-bank CPU constants before passing verification. No map/ROM/toolkit changes or final audit; continue full readability scope.


### ROUND003 commentary batch015 (writer C; phase incomplete)

Completed14bank14Code scaffolds314bytes with17units: three-phase vertical tile/attribute upload, HUD label/counter/timer/lives/actor-health tails, contact flag consumption and damage returns, bank6 wrappers, NMI transfer joins and literal CHR selection. Kept Unknown bank6 return boundaries explicit and distinguished lower-bank preservation from fixed-bank execution. DEP108/115/119 now reviewed after concrete consumer checks. Current2279reviewed/94pending scaffolds,2828units;4832unreadCodebytes/96dependencies remain. Fresh supported verify is byte-identical; reading-after refreshed, source-relative anchors/partition pass0errors. Bank14 life/pause graph remains; banks6/7/13 remain in full. No final audit or successful handoff.


### ROUND003 commentary batch016 (writer C; phase incomplete)

Completed all12remainingbank14Code scaffolds441bytes with24units across pause/resume, life-result RTS dispatch, first/second-player availability/counter/switch paths, saved-life choice, timer-health fallback and44-byte player restores. Two further dependencies DEP099/104 checked. Current2291reviewed/82pending,2852units;4391unreadCodebytes/94dependencies. Bank14 newCode is complete; banks6(24/2130bytes),7(29/1090),13(29/1171) remain. Fresh byte-identical verify/read pass and metadata anchors/partition0errors. Initial checks caught a missing second-entry symbol and incorrect C238 alias (C235 would reset lives); corrected to the actual bank6 screen-initialization call preserving lives before passing. No final audit or successful handoff.


### ROUND003 commentary batch017 (writer C; phase incomplete)

Completed7bank6Code scaffolds509bytes with16units: life/decision API jumps, status/continue screens with actual input0/1 result, PPU tile strings, session-selected player marker, selector OAM, pattern-match initialization and52-byte ending OAM. Explicit bank6 requirement propagated to fixed wrappers. DEP009/016 checked: corrected false repeated-register CHR claim (Y advances, writes registers0/1) and obsolete unclassified status-template claim. Current2298reviewed/75pending,2868units;3882Codebytes/92dependencies remain. Remaining banks6(17/1621bytes),7(29/1090),13(29/1171), no partial scaffold credit. Fresh byte-identical verify/read and source-relative anchors/partition0errors. Initial verify caught AB58 palette-record entry mistaken for AB49 command entry; corrected exact alias before pass. Final audit pending; same commentary phase continues.


### ROUND003 commentary batch018 / continuation C — incomplete

Completed15bank6Code scaffolds474bytes with21units across palette/screen-sequence completion, ending runtime reset and nine actors, terminal timer/input/sound-cycle, descriptor source/destination transfer pairs, alternate sprite stride and coordinate carry. Updated affected carried summaries to match corrected source. Current2313reviewed/60pending scaffolds,2889units;3408unreadCodebytes and92dependency candidates remain. Bank6 has two wholly unread large graphs(585bytes A46E..A6B7 and562bytes B76E..B9A0); banks7/13 remain29scaffolds each1090/1171bytes. No partial credit. Fresh supported verify byte-identical and reading-after refreshed; full physical partition/source-relative metadata0errors. Sequential C capacity checkpoint saved, same COMMENTARY phase resumes; no final audit or successful handoff.

## ROUND003 D batch019 — incomplete; resume commentary

Reviewed bank6 physical50302..50887 (585bytes) in11meaningful units: intro/options, input-pattern side effects, acceptance, OAM marker toggle and16-step blink, controller2 selection index, briefing state machine, character submission/comma/end-line/pending tails. Branch and RAM roles named; gameplay meanings of input patterns remain unresolved. Fresh supported verify is byte-identical; reading copy refreshed. Current2314/2373scaffolds reviewed,2900units;59Code scaffolds/2823bytes unread (bank6 one562byte graph;bank7 29/1090;bank13 29/1171). No partial scaffolds. Dependencies47reviewed/92pending of139. No successful handoff or final editorial audit.

## ROUND003 D batch020 — incomplete; resume commentary

Completed the full bank6 player-choice graph physical55166..55728 (562bytes),10meaningful units covering screen loop, conditional redraw, held-input choices, one-time confirmation, four source-pair redraw families and descriptor selector. Source-pair data consumer identifies the first48bytes at55728 as twelve two-origin records; remaining56bytes retain uncertainty. Fresh supported verify byte-identical/read refreshed; partition/anchors0errors. Current2315/2373reviewed,2910units;58Code scaffolds/2261bytes remain wholly unread (bank7 29/1090;bank13 29/1171). No partial coverage. Dependency47reviewed/92pending. No successful handoff/audit.

## ROUND003 D batch021 — incomplete; resume commentary

Completed all29remaining bank7Code scaffolds/1090bytes with39units: scroll request acknowledgement, secondary/vertical timer row pairs, four phase-layer continuations, initialization families, reverse-row publication, alternate palette/CHR setup, factory opening rows/palette tail and four-byte OAM template tail. Explicit IRQ-side wait contracts retained. Corrected D-local unit entries placed on preceding RTS (including B6 option entryA4A6); branch return aliases and routine entry aliases now separate. C225 identified as ApiQueueBackgroundPaletteBuffer in the palette copy tail. Fresh supported verify byte-identical/read refreshed; partition/anchors0errors. Current2344/2373reviewed,2949units;29Code scaffolds/1171bytes remain entirely unread, allbank13. No partialscaffolds. Dependencies47reviewed/92pending. No successful handoff/audit.

## ROUND003 D batches022–023 — incomplete; dependency checks remain

Completed all29remaining bank13Code scaffolds/1171bytes:022action/animation/terrain-timer/movement-bound tails21units768bytes,023collision-coordinate/axis-response/OAM tails14units403bytes. Collision relations explicitly use high nibbleY/low nibbleX (before2,same1,after0); quadrant and collision type remain distinct. Honest Unknown boundary after ROM/mapper-directed record write preserved. Current2373/2373scaffolds reviewed,2984units,0unreadCode/0partial. This is NOT phase completion:92of139dependencycandidates remain pending (47reviewed), plus concrete scope assessment/caller-name integration. Fresh supported verify byte-identical/read refreshed,partition/anchors0errors. Finaleditorialaudit pending; no successfulhandoff.

## ROUND003 D batch024 — incomplete;67dependency checks remain

Checked25specific dependencies (DEP001,003..008,010..015,017..028), each with preserved physical anchors and individual findings. Corrected modebit7 AND7F clear vs prior retain claim; removed obsolete Unknown completion/timer/ending boundary claims; named bank6 table CPU aliases and fixed APIs; identified seven eight-byte stage map records and retained locally unused/external-bank uncertainty. Input-pattern/CHR-loop/C235vsC238/AB49vsAB58 distinctions preserved. Fresh supported verify byte-identical/read refreshed,partition/anchors0errors. All2373scaffolds/2984units reviewed;0Code/partial. Dependency72reviewed/67pending of139. No successfulhandoff/audit.

## ROUND003 D batch025 — incomplete;39dependency checks remain

Checked28specific bank7dependencies DEP029..039,041..057 with individual evidence/findings. Classified RTS/CHR/row/palette/OAM data consumers now use explicit CPU aliases; obsolete unclassified-continuation disclaimers removed. Retained unchecked selector domains, lower-bank99A6identity uncertainty and auxiliary-table purpose uncertainty. Fresh supported verify byte-identical/read refreshed;partition/anchors0errors. All2373scaffolds/2984units reviewed,0Code/partial. Dependencies100reviewed/39pending of139. No successfulhandoff/audit.

## ROUND003 D batch026 — incomplete;16dependency checks remain

Checked23actor/player dependencies DEP076..098 individually. Known fixedterrain helpers now have descriptive aliases and B10caller guards; B9zero-step viewport result returns through an already classified RTS (removed false Unknown claim). Preserved four genuine actor-boundary Unknowns and unresolved enemy/scene/visual identities. Named thirteen animation/action/movement/collision table CPU views; obsolete terrain-timer/completion/movement-join gaps corrected. Fresh supported verify byte-identical/read refreshed,partition/anchors0errors. All2373scaffolds/2984units reviewed;0Code/partial. Dependencies123reviewed/16pending of139. No successfulhandoff/audit.

## ROUND003 D batch027 — final dependency batch; scope gate still active

Checked final16dependencies DEP100..103,105..107,113..114,116..118,120..123. All139candidates reviewed with individual outcomes. Corrected D2E5sound-request table role, D95964HUDtemplate/48awardoffset/mixedamount layout, and DCC3exactzero classified health-zero store. Fixed false bank7Data match for C3ACA00Ccall using explicit prior bank6map. C245now named stage timer/position restart (skips C242spritepalette queue), distinct C238screen-init/C235life-init. E6A9/E705/E71Fconsumer views retain unresolved selectorbounds rather than invent disjoint extents. Fresh supported verify byte-identical/read refreshed;partition/anchors0errors. All2373scaffolds/2984units reviewed,0Code/partial/dependencies. Caller-name/fullscope assessment remains before successful handoff. No finaleditorialaudit.

## ROUND003 D batches028–029 — current-source readability complete

The fullcurrent-source gate passes:2373/2373scaffolds reviewed,3000/3000units reviewed,139/139dependencies checked,0pending/partial. Reviewed-unit union covers all56616classifiedCodebytes with0uncovered intervals. Batch028corrects ten further stale API/row-return prose claims and integrates known fixed/localcaller names; batch029adds16exact entry/tail-head records with purpose notes, retaining prior valid units. Current bankzero uncertainty overview now distinguishes classified mechanics/tables from genuine boundary ambiguity. Additional bounds/context exclusions and qualitative assessment are recorded in analysis/round-003/commentary-scope-assessment-final.json.

Fresh supported verify byte-identical and reading-after refreshed after finalsourceedits. Map equals frozenbaseline; state equals frozenphase1state; ROMpartition/unitbounds/sourceanchors pass0errors. Classification remains99.54%,1204Unknown; gameplay/external-bank/Unknown-boundary limitations stay explicit. No finaleditorialaudit (stillpending), no map/state/toolkit/ROMchanges, no publication/archive/cleanup/deletion. Successful current-source handoff: analysis/round-003-commentary-handoff.md. WriterDstoppedediting after this record.
