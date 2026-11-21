# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 interim evidence

Validated reset E000 (physical1e010), NMI FFF4 and IRQ FFF7. Conservative observed/static annotations and generic zero-run data are saved in analysis/state.json; exact boundaries in round-001/static-candidates.json, startup-static.json, intro-next-static.json, startgame-static.json and uniform-runs.json. CHR classified as independent PPU pattern backing from validated iNES lengths, without game-specific tile grouping. Header and vectors classified separately. maps/pass-001-working.json is verified; later small code gains pending final checkpoint.

Input schema obstacle NES-001 explains initial stalled play; singular button/pressed runs are unattended startup only. Actual array Start advances to options. Menu disassembly at CPU84EE checks a five-frame timer at RAM0563, so two-frame taps can be missed; practical retry uses twelve-frame presses/releases. Long Start from options reaches stage card 'DAYTIME SCENE1 A JOURNEY THROUGH TRANSYLVANIA'.

Concrete progression contract discovered statically: menu start handler CPU8569 reads selection05e4; selected start item returns through89ea after fade. Hidden menu code CPU862B uses a modulo14 sound-test value and writes stage index RAM027B; stage-index consumers at8663/866b use parity to select day/night scene graphics. Difficulty RAM06c4 cycles0..2 in menu (Select, not Right); reset initializes1. Further stage transition implementation remains to inspect. Physical/bank context: menu PRG8/9 mapped8000/BFFF, fixed14/15 atC000/FFFF; all raw decode evidence persisted under round-001/*-decode.json. Decoder output starting8559 was intentionally exploratory and begins midinstruction; no annotation based on that misaligned prefix.


## Round 001 disassembly pass end — incomplete

ROM classification: 69.21% (181447 / 262160 bytes classified; 80713 bytes unknown).

Final checkpoint maps/pass-001-final.json, generated/pass-001-final, analysis/pass-001-final-verification.json: map validates and rebuild is byte-identical. Input SHA256 remains configured3d1356622288e880ecf7bf9155ac84dc3d63798593e25c2cf5ca5a5488ff85c3. Internal state analysis/state.json and portable maps/current.json saved.

| PRG 8KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
|0|0|2720|5472|
|1|0|5654|2538|
|2|0|1755|6437|
|3|0|2772|5420|
|4|0|2269|5923|
|5|0|5840|2352|
|6|0|3763|4429|
|7|0|5652|2540|
|8|1525|0|6667|
|9|25|5409|2758|
|10|865|0|7327|
|11|1672|2530|3990|
|12|0|1442|6750|
|13|1268|0|6924|
|14|2355|0|5837|
|15|2837|6|5349|

Header16 Data; final vectors6 Data included in bank15; CHR131072 Image and0 unknown. 4859 regions all carry structural evidence. Code/data counts, provenance and exact unknown complement: round-001/counts.json, bank-counts.json, unknown-ranges.json. Observed execution is separate: 4369 unique physical opcode starts in retained scenario aggregates, 3757 in natural-route windows; this samples execution and does not count all elapsed untraced gameplay. No full compatibility claim.

Natural forest Scene1 exercised movement, jump, attack and crouch. Artificial stage6/final-stage13 starts verified playable castles. Artificial completion continuation reaches animated Dracula coffin cutscene through frame4110; credits/final terminal screen not yet observed. Representative stage6/13/ending incremental code gains359/626/161 bytes diminished; unresolved dispatcher/level structures and boss/ending progression require connected redirect investigation, not repetitive unattended playback. Broader completion is not asserted and this is not terminal workflow exhaustion.

Toolkit findings: one confirmed protocol/documentation mismatch NES-001, zero suspected bugs and zero missing-feature findings; no blocker after array-form input workaround. See toolkit-issues.md. Earlier singular button/pressed sequences are invalid input routes and only unattended execution evidence. Commentary/readability audit has not run.

## Round001 redirect connected batch

Current-source commentary gate passed before research. Consumer-derived14-stage domains resolved exact compressed map, metatile, attribute, collision, palette and spawn bounds. Existing stage6/13 snapshots were mid-decompressor; a50000-operation continuation verified both full decoded RAM maps byte-for-byte. All28 concrete spawn identifiers,45 supported actor state-zero contracts,113 state words,19 screen words and7 boss words received connected static inspection. Direct callees, produced actor types and sound command initialization were included in the same batch. See redirect/round-001-connected/results.md, barrier-review.json and proposal-summary.json.

R001–R003 improved to ready; R004–R006 added ready. Unique proposed unknown-byte union64792 is unaccepted; actual unknown remains80713. Code candidates require independent bank/stack review, especially JSR return edges across routines that discard return addresses. Exact bank-split unknown ledger records225 intervals totaling80713 bytes with neighboring classified regions, physical banks, possible CPU aliases and evidence-linked dispositions. Remaining ideas are deferred for concrete missing domains or diminished structural benefit, not because they are independent. This is a nonterminal handoff, not an exhaustion assessment. No new toolkit issue; existing NES-001 remains nonblocking.

## Round002 disassembly pass end — incomplete, nonterminal

ROM classification: 98.67% (258679 / 262160 bytes classified; 3481 bytes unknown).

Current phase disassembly complete for this one pass; checkpoint maps/pass-002-final.json, generated/pass-002-final, analysis/pass-002-final-verification.json. Saved analysis/state.json equals exported map and checkpoint exactly. Configured release toolkit validates and verifies byte-identical; separate fresh verify in round-002/independent-verification.json also passes. ROM SHA256 unchanged.

Gain77232 unique bytes:19286 Code and57946 Data; all64792 redirect proposed bytes independently accepted with corrected bank/stack contracts, plus5112 connected Code and7328 consumer/pattern Data bytes. No byte quota, no Unknown sweep, no overlap/conflicting instruction starts. Current totals29833 Code,97774 Data,131072 Image;12962 regions all have evidence. Full object Data bounds retained in round-002/data-review.json while existing overlap evidence remains unchanged. All R001–R006 structurally verified; natural selection of every handler and semantic/full family domains remain separate.

| PRG8KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
|0|0|8192|0|
|1|0|8192|0|
|2|0|8192|0|
|3|0|8192|0|
|4|0|8192|0|
|5|0|8192|0|
|6|0|8192|0|
|7|0|8192|0|
|8|4850|2853|489|
|9|25|8167|0|
|10|1051|7137|4|
|11|4839|3329|24|
|12|0|8192|0|
|13|5769|1487|936|
|14|6539|226|1427|
|15|6760|831|601|

Header16 Data; CHR131072 Image, no unknown. Exact40-interval/3481-byte complement and bank counts in round-002/unknown-ranges.json and bank-counts.json. Strong residuals include bank8 screen/input/sound fragments, bank13 actor/control routines, bank14 initialization/PPU/handler slices and bank15 startup/input/table remnants. These need concrete entry/control/format bounds and bank/return models, not a universal live-caller requirement. Review/code cache and six remaining mapper/indirect return barriers preserve a usable redirect frontier. This is a single-pass handoff, not terminal exhaustion.

No new gameplay replay: natural forest and artificial stage6/13/ending snapshots/decoder RAM confirmations reused. Observed execution unchanged (4369 physical opcode starts in retained aggregates;3757 natural-route samples). No new Static bytes labeled Observed. Boss defeat and credits remain unverified; S006 remains deferred for concrete boss-state/position transition research. No full hardware/game compatibility claim.

Prior commentary/source hashes match round001 verification (round-002/commentary-preserved.json), preserved for next commentary carry. Current-source readability for new classification has not run; whole-ROM final editorial audit remains pending. Historical provisional F14E return inference retained and corrected in round-002/stack-context-correction.md; final normal-return relation propagates effective output banks and localizes nonlocal stack paths.

Toolkit findings: existing confirmed nonblocking NES-001 remains, zero new suspected bugs or missing features; no new toolkit issues found. See analysis/toolkit-issues.md. No toolkit/ROM edits, publication, archive, deletion or cleanup.

## Round002 redirect contracts
Research-only round-002-contracts complete withR0073400/R00881ready, all3481residual bytes proposed/accepted0. Exact46claimroles andrecipes inproposals.json/leads.json; full40intervalledger retained. Installedinterrupts, mapperparameters, stage/setup/camera/action/item/effectcontracts and genericnoncode consumer/pattern evidence resolved prior graph-proof limitations. R008soundtemplate singledoutforindependentreview becausebank8 JSR8003callingsemantics remainunresolved. Bossposition+camera shortcutattempted withretainedstage13snapshot; visiblecastle restored, intendedboss/defeat/credits notvisited, stage/regioninitializationmissing. Seeanalysis/round-002-redirect-handoff.md andlinkedprimaryevidence. No map/state/source/ROM/toolkit mutations.

## Round003 disassembly pass end — structural classification complete, overall incomplete

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Exactly one disassembly pass; no baseline replay. Gain3481 Static bytes:2941 Code/540 Data. Totals32774 Code/98314 Data/131072 Image;13019 regions all carry evidence. Exact unknown complement empty; per-bank counts in round-003/bank-counts.json and counts.json. No added execution evidence; retained4369 physical opcode starts and3757 natural-route samples remain separate.

R0073400 and R00881 independently verified after semantic/control/stack/bank and pattern/consumer/boundary review. See round-003/review-notes.md, code-decode.json, consumer-decode.json and decisions.json. R008 accepted specifically as relocated code template: full accepted CD6A opcode/control/RAM-field correspondence plus localJMP/table relocations and independent95FE boundary. Bank8 JSR8003 goes to screen initialization; working sound function/caller and natural use unresolved, no Observed annotation. BA03 consumer is0508, not0500. CAB8 uses bank11 until restore13; stage0 source0 and demo source8/save-bank restoration explicitly modeled. Unproved unrelated slots remain null locally; no false bank preservation. Nonlocal return-address discard paths never supply ordinary-return witnesses; F14A/F14E correction and E8D0/B676 nonzero-frame save-bank precondition retained.

| PRG8KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
|0|0|8192|0|
|1|0|8192|0|
|2|0|8192|0|
|3|0|8192|0|
|4|0|8192|0|
|5|0|8192|0|
|6|0|8192|0|
|7|0|8192|0|
|8|5111|3081|0|
|9|25|8167|0|
|10|1051|7141|0|
|11|4863|3329|0|
|12|0|8192|0|
|13|6653|1539|0|
|14|7897|295|0|
|15|7174|1018|0|

Header16 Data; CHR131072 Image; every bank0 unknown. Final checkpoint maps/pass-003-final-context.json, generated/pass-003-final-context and analysis/pass-003-final-context-verification.json. Original pass-003-final preserved; final consistency review corrected E429's broad13 default to actual screen-return8/9 in context-correction.json, classification unchanged. State/current/final checkpoint equality and fresh verification recorded in round-003/final-integrity.json and independent-verification-context.json. ROM hash unchanged.

No unchanged gameplay replay. Natural forest and artificial stage6/13/ending evidence reused. S006 boss position/camera shortcut already attempted, visible castle restored but no active boss/defeat/credits. Stage/region initialization contract still missing; no useful changed supported route found, so no repeat teleport/intro/coffin. Scenarios.json retains documented attempt.

Prior commentary/source preserved unchanged for next carry. Current-source commentary for these3481 newly classified bytes and affected dependencies remains required, followed by final editorial audit and fresh byte-identical verification. Zero unknown alone does not establish overall completion. Handoff round-003-disassembly-handoff.md; leadsR007/R008 consumed structurally.

Toolkit findings: existing1 confirmed nonblocking NES-001;0 suspected/0 missing features; no new toolkit issues found. See [toolkit-issues.md](toolkit-issues.md). No toolkit/ROM edits, publication, archive, deletion or cleanup.

## Round003 redirect complete — empty structural lead result
Checkpoint pass-003-final-context;0 unknown,262160 classified. Attempt analysis/redirect/round-003-structural-frontier records actual-source/uncertainty and decisive template/context/boss review, exact empty ledger, no new/improved leads (proposed0/accepted0). R001–R008 remain consumed. Two editorial wording/count findings handed to final acceptance; no structural defect. Full readability complete; final editorial audit/fresh verification pending. Handoff analysis/round-003-redirect-handoff.md. No unchanged gameplay replay or source/map/state/ROM/toolkit changes; no new toolkit issues, existing NES-001 nonblocking.
