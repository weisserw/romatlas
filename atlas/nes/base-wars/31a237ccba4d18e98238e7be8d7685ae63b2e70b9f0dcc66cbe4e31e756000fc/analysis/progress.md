# ROM analysis progress

Current coordinator result: TERMINAL INCOMPLETE, workflow handoff complete after round005 and final editorial acceptance. ROM classification: 100.00% (262156 / 262160 bytes classified; 4 bytes unknown). Rounded percentage does not establish ROM completion. Final checkpoint maps/pass-005-verified.json; current-source readability COMPLETE, final editorial audit COMPLETE, fresh byte-identical verification passed. Source45d31d59eba86be7c798cd054a452e8cb855919985657f554a5a43037f082a76. Final scope/artifacts: [final-handoff.md](final-handoff.md), [coordinator final audit](coordinator-final-audit.md). Structural terminal review accepted only after primary inspection/corrections; no ready leads or useful unchanged research cycle. Archive selection prepared, no archive/publication/deletion/cleanup/toolkit edits. Historical progress follows unchanged.

Current phase result: redirect004 combined ready research, pass004verified remains262128classified/32Unknown12ranges. Proposes28bytes(10Code18genericData), acceptance pending; wouldleave4isolatedRTSbytes. Full commentary004 current-source readability complete50138Codebytes; source60102fb5..., editorial audit pending. Handoff analysis/handoff-redirect-004-ready.md. Original progress entries preserved below.


Current phase result: commentary003 current-source readability complete and byte-identically verified. All49307Code bytes,382Data containers and71Unknown intervals explained;23918scaffold entries reviewed,1738Code units/2019total units,zero remaining readability gaps. Source e272216f33a34c0597713dc20e5e8cf0c10fe06b2cfbe361281b37ffc6111c51; reading copy refreshed. Classification remains241911/262160(92.28%),20249unknown. Final editorial audit pending; no redirect executed. Handoff analysis/handoff-commentary-003.md.

Phase: round 001 commentary completed; current-source readability complete, ready for coordinator handoff. Disassembly pass 001 is frozen and incomplete; current-source readability work is in progress. Historical disassembly startup follows.

Disassembly startup: Baseline established with `python3 run.py inspect` and `baseline`; baseline map/generated source rebuild byte-identically (SHA-256 31a237ccba4d18e98238e7be8d7685ae63b2e70b9f0dcc66cbe4e31e756000fc). Classification in progress; final counts/checkpoint pending.

Identified locally from title: Cyber Stadium Series: Base Wars, Ultra/Konami 1991. Mapper 4, 16 x8 KiB PRG banks and 16 x8 KiB CHR blocks. CHR structure and header annotated; accepted startup instruction roots and supported static successors being classified. No incoming ready leads.

Runtime: title, mode select and open-game team selection visited. NES input requires `buttons` array (shared `button`/`pressed` example is silently ignored). Bounded gameplay investigation ongoing; manual supports CPU-vs-CPU and selecting one inning as practical progression shortcuts. Durable captures and accepted fetch contexts under analysis/disassembly-001/.

Batch 2 saved: 152,060 classified bytes including 8,559 Code bytes; 110,100 unknown. Runtime extended through pitching/batted-ball fielding and artificial ending state ($30/$34=$0F) with city/congratulations animation (graphical artifacts; full clean ending initialization unproved). Natural pennant configuration, rosters, standings, change configuration and parts shop visited; bounded shopping experiment continues. No unsupported ROM upload-origin classification made for observed RAM dispatch execution. Final checkpoint pending.

## Round 001 verified checkpoint — disassembly pass complete, ROM incomplete

ROM classification: 61.39% (160941 / 262160 bytes classified; 101219 bytes unknown).
| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 2725 | 0 | 5467 |
| PRG01 | 0 | 2317 | 5875 |
| PRG02 | 405 | 0 | 7787 |
| PRG03 | 340 | 1308 | 6544 |
| PRG04 | 974 | 0 | 7218 |
| PRG05 | 1258 | 131 | 6803 |
| PRG06 | 0 | 0 | 8192 |
| PRG07 | 0 | 246 | 7946 |
| PRG08 | 2255 | 254 | 5683 |
| PRG09 | 3140 | 257 | 4795 |
| PRG10 | 370 | 65 | 7757 |
| PRG11 | 25 | 41 | 8126 |
| PRG12 | 1883 | 64 | 6245 |
| PRG13 | 403 | 6625 | 1164 |
| PRG14 | 1939 | 795 | 5458 |
| PRG15 | 1723 | 310 | 6159 |
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
sha256 31a237ccba4d18e98238e7be8d7685ae63b2e70b9f0dcc66cbe4e31e756000fc

Formula: X = 100 * C / T; T is all physical file bytes including header, PRG and CHR. C is unique evidence-backed Code/Data/Image bytes in analysis/state.json; gaps, Unknown and raw export fallback excluded. U = T - C. Exact kinds: 17,440 Code, 12,429 Data, 131,072 Image; 101,219 unknown. Every accepted region has evidence metadata; full source readability/editorial review belongs to commentary and remains pending.

State saved to [state.json](state.json), map exported to [current.json](../maps/current.json), validation passed. Fresh frozen map [pass-001.json](../maps/pass-001.json), generated source [game.asm](../generated/pass-001/game.asm), verification [pass-001-verification.json](pass-001-verification.json): byte_identical=true, map SHA-256 ed1020b1b919ae3bfba3768cb81da472a05fd202c7b6f095fabc52cd48063bb3. Input hash unchanged. Configured binary used throughout; toolkit repository HEAD 23521a5743b1cf8ce36c818b482f683068380acd (read-only inspection).

Evidence: [counts.json](disassembly-001/counts.json), [unknown complement](disassembly-001/unknown.json), [scenario checklist](scenarios.json), accepted instruction contexts/candidates in disassembly-001/batch{1,2,3,4}-*.json, [data conflict review](disassembly-001/data-conflict-review.json). No accepted fetch overlapped any of the 23 inspected uniform data runs. Generic data rationale: exact uniform spans and adjacent patterned records or bank-tail boundaries, no accepted code conflict; specific consumers/original fill purpose unresolved.

Execution separately: 8,241 physical instruction starts; 9,536 contextual roots, 11,410 contextual edges, aggregate_dropped=0. Bounded zero-detail tracing intentionally dropped detail; snapshots rewind cycles, so aggregates combine explicitly described scenarios and are not a single forward timeline. Natural title/open-match pitching/pennant/rosters/standings/shop and artificial fielding/ending scenarios are separated in scenarios.json. No ROM upload origin inferred from RAM execution.

Incoming leads: none. Outgoing R001-DISPATCH open; R001-COMBAT open (fielding attempt did not reach close base battle); R001-ENDING partial (city/congratulations with graphical artifacts, clean initialization unresolved); R001-DATA open. Details in leads.json and investigation.md. One bounded pass concluded after broad distinct-system gains; this is an ordinary handoff to commentary/redirect, not terminal exhaustion. No complete-game/fidelity/readability/compatibility claim.

Toolkit: one confirmed documentation/API consistency bug, no suspected bugs or missing features, no blockers; see [toolkit-issues.md](toolkit-issues.md). No toolkit edits, deletion, archive or publication performed. Server stopped after saving final-shop snapshot.


## Commentary 001 continuation checkpoint — interrupted

Full current-source readability is in progress. Reviewed 9,364/17,440 classified code bytes in PRG00/02/03/04/05/14/15, plus all non-code and unknown boundary notes; 415 behavioral units, dedicated audio/shared state and hardware symbols. Pending code PRG08/09/10/11/12/13 totals 8,076 bytes, exact intervals in commentary/readability-remaining.json. Resume same commentary baseline, no redirect. Latest fresh commentary verification true, source SHA-256 83f97828b204d8ef26630da4fef3404a3bbe5c36b83d53e58e051094c38c86f0; reading copy refreshed. Phase-1 map/state unchanged. No editorial audit. See commentary/progress.md and analysis/handoff-commentary-001.md for exact continuation. Existing toolkit NES-001 nonblocking; no new issues in commentary.


## Commentary 001 completed — ROM remains incomplete

Full current-source readability complete: 17,440/17,440 classified Code bytes reviewed, 640 meaningful behavioral units, all 8,800 scaffold entries reviewed, no uncovered code regions or pending readability units. Descriptive routines/branches/state/object-field symbols and coherent mechanics added throughout. Unknown bytes remain unknown with boundary notes. Final editorial audit reserved and pending.

Latest fresh verification byte_identical=true, source SHA-256 f3508c1090a3cae5ed2ec820827032ed679a7479a87cb212a69c117ff58bf4a7; reading copy refreshed. Phase-1 state/map unchanged and ROM hash preserved. Classification still 160941/262160 bytes, 61.39%, 101219 unknown. See analysis/handoff-commentary-001.md and analysis/commentary-001-readability.json. New bounded structural camera-lookup conflict recorded for redirect; other unresolved meanings in commentary/uncertainties.md. Existing NES-001 remains nonblocking; no new commentary toolkit issues, no toolkit edit/ROM mutation/archive/publication/cleanup.


## Redirect 001 — in progress

Unique research attempt analysis/redirect/round001-a; input maps/pass-001.json, full current-source readability complete. Investigating camera conflict, dispatcher/ending continuations and concrete residual data avenues. No structural edits; ROM/classification checkpoint unchanged.

Redirect 001 research checkpoint: camera no-fetch conflict verified; exact DMC sample consumer and41 fully parsed background RLE streams provide independent data evidence. Ending inline tables and all unique phase targets inspected; gameplay dispatcher now supplies concrete combat-bank entry via state6/7 and pair3 CPU8001. Runtime title-based ending shortcut in progress with bounded timeout results. See analysis/redirect/round001-a/frontier.md for exact evidence, remaining work and live session. This is an interrupted/resumable frontier, not phase completion or exhaustion; no map/source edits.

## Redirect 001 completed — combined ready proposals

Unique attempt analysis/redirect/round001-a completed after connected combat/ending/dispatch and independent audio/DMC/background/shop investigation. Existing classification unchanged160941/262160,101219 unknown. Proposed98 Data intervals union36333 bytes includes35658 formerly unknown and exact3-byte camera role correction;1210 Observed plus111 Static contextual Code entry proposals await independent review. Data-only projected residual65561 is a proposal, not acceptance. Exact unknown ledger and counterexample corrections preserved.

Artificial combat visited/exercised; artificial ending ordinary continuation reached stable phase55/CPU8B28 and publisher screen. Full histories/scenario evidence retained. All servers stopped. Map/state/ROM/source hashes unchanged; no toolkit edits or new issues (existingNES-001 only), no archive/publication/deletion/cleanup. Disk6.4GiB free, attempt29MiB. Read analysis/handoff-redirect-001.md for combined batch, exact recipes, limits and independent disassembly frontier; not terminal exhaustion.


## Disassembly 002 complete — verified, ROM incomplete

ROM classification: 78.74% (206425 / 262160 bytes classified; 55735 bytes unknown).

Exact kinds:27,217 Code;48,136 Data;131,072 Image. Net45,484 new classified bytes:35,658 accepted redirect Data bytes,9,780 Code bytes,46 newly connected gameplay-array Data bytes. Exact3byte camera Static Code-to-Data correction changes role without inflating total. Formula100*C/T uses unique evidence-backed physical bytes including header/PRG/CHR; unknown gaps/raw fallback excluded.

Independent structural review accepted all six ready leads at supported scope. Audio suspicious A9-prefixed section decoded against competing code: repeated descriptors/stream patterns, KIL/incoherent alternative and anchored consumers support generic non-code bulk, not a complete musical grammar. Camera forced branch/indexed read corrected old Static bytes. Fixed-bank DMC address/count and three non-sample gaps reviewed;41 complete RLE resources; combat nested action records/frame grids; inline dispatcher stack contract and local table bounds reviewed. Existing evidence preserved except exact camera correction. See disassembly-002/data-review.json/data-accepted.json.

Fresh bank-aware roots and connected CFG reviewed ordinary balanced-stack RTS contracts, branches, calls, data barriers and known mapper effects.1,080 conservative positive return contracts and explicit mapped continuations support extra handlers/helpers, all seven combat state roots, ending phase paths, shop debit/checksum and fixed pair1 bridges. Saved actual fetch roots remain Observed only for artificial routes; new static successors remain Static. No RAM upload origin or natural combat/ending/successful-purchase claim. See code-review.json, connected-decodes.json/connected-candidates.json/connected-contracts.json and code-accepted-batch1/2.json plus final code-accepted.json.

Localized exploratory counterexamples were excluded before annotation: stale bank afterFECC produced combat EOR-operand alias; explicit mapper barriers and changed-context fixed continuations corrected it. LDAFF/STA/BNE at121BC forced branch excludes apparent121CC fallthrough; consumer-supported46byte lookup/position arrays now Data. Original bank12CPU8000 metadata identity is not Code; bank2 inline table stops15words before802A helper. No other concrete contradiction in existing accepted Code was found.

Representative broad gameplay evidence reused: natural title/open match/pitching/fielding/pennant/rosters/standings/shop; artificial duel movement/attack and ending to stable publisher screen. Checklist reviewed in scenarios.json; no unchanged failed replay or new runtime experiment. Sports game has no conventional levels/bosses. Late-inning result and completed purchase remain unobserved, without blocking static structural classification.

map.validate passed; state/current/frozen pass002 hashes identical3a6c8b4897c313e3988e8af9d8bf00ffc104d678c6652c85f22b90eb0d66cb5e. Fresh checkpoint maps/pass-002.json and generated/pass-002 verified byte_identical=true in analysis/pass-002-verification.json. ROM SHA256 unchanged31a237ccba4d18e98238e7be8d7685ae63b2e70b9f0dcc66cbe4e31e756000fc. Prior pass001 and full commentary source hashes preserved (preservation-audit.json). Generated pass002 requires commentary catchup over new classifications; old commentary is intact. No final readability/editorial/fidelity/compatibility claim.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 2948 | 4923 | 321 |
| PRG01 | 0 | 8192 | 0 |
| PRG02 | 901 | 4227 | 3064 |
| PRG03 | 944 | 5618 | 1630 |
| PRG04 | 1507 | 56 | 6629 |
| PRG05 | 1617 | 216 | 6359 |
| PRG06 | 2371 | 5012 | 809 |
| PRG07 | 1970 | 2106 | 4116 |
| PRG08 | 2617 | 270 | 5305 |
| PRG09 | 4149 | 325 | 3718 |
| PRG10 | 370 | 65 | 7757 |
| PRG11 | 106 | 45 | 8041 |
| PRG12 | 2861 | 118 | 5213 |
| PRG13 | 471 | 6637 | 1084 |
| PRG14 | 2513 | 4478 | 1201 |
| PRG15 | 1872 | 5832 | 488 |
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


Unknown complement disassembly-002/unknown.json remains55,735 bytes. All six incoming leads verified with histories; new R002-CONNECTED-RESIDUALS names concrete lookup/return questions. residual-reference-frontier.json is a hypothesis inventory; filter mapper-register reads/writes, since CPU addresses alone do not prove ROM references. Exact combat arrays C25B/C334/C74E/ED47 and fixedFAEB/FAEC need consumer/domain/boundary checks; remaining mapper/indirect-store/stack contracts require semantic review, not blanket continuation. Larger PRG10/11 residuals remain ambiguous. Supported connected batch followed beyond original proposal ranges; further routine/data extent work usefully belongs to redirect after commentary. This is an ordinary easy-gain handoff, not terminal exhaustion; no terminal rejection audit claimed.

Toolkit: existing NES-001 input-schema documentation issue remains nonblocking; no new issues or toolkit edits. See [toolkit-issues.md](toolkit-issues.md). All subprocess servers stopped; no ROM changes/archive/publication/deletion/cleanup. Disk6.4GiB free; scratch4KiB.

Commentary002 active: prior commentary preserved at analysis/round-001/commentary; carry verified. New source review continues, exact active frontier in commentary/progress.md. Do not redirect before full readability completion.

Commentary002 complete: all current source readable, fresh byte-identical verification and reading copy complete. See analysis/handoff-commentary-002.md; full-ROM classification remains 78.74%, final editorial audit pending.


## Disassembly003 — verified supported connected batch

ROM classification: 92.28% (241911 / 262160 bytes classified; 20249 bytes unknown).

Added35486unique physical bytes:22090StaticCode in10371instructions;13396StaticData. All old accepted roles preserved. Independent fresh decoder/context/overlap audit and primary contract/consumer review: [structural-review.md](disassembly-003/structural-review.md), [decode-audit.json](disassembly-003/decode-audit.json), [data-boundary-review.json](disassembly-003/data-boundary-review.json). Each accepted annotation records Static evidence, not runtime reachability. Context and boundaries validated; current source commentary pending as a separate phase.

| Bank | Code | Data/other | Classified | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 16 | 0 |
| PRG00 | 3098 | 5090 | 8188 | 4 |
| PRG01 | 0 | 8192 | 8192 | 0 |
| PRG02 | 2537 | 5475 | 8012 | 180 |
| PRG03 | 1618 | 6574 | 8192 | 0 |
| PRG04 | 4700 | 3468 | 8168 | 24 |
| PRG05 | 6874 | 1279 | 8153 | 39 |
| PRG06 | 2959 | 5216 | 8175 | 17 |
| PRG07 | 5537 | 2652 | 8189 | 3 |
| PRG08 | 3531 | 386 | 3917 | 4275 |
| PRG09 | 6893 | 782 | 7675 | 517 |
| PRG10 | 370 | 3102 | 3472 | 4720 |
| PRG11 | 165 | 49 | 214 | 7978 |
| PRG12 | 4264 | 1735 | 5999 | 2193 |
| PRG13 | 1380 | 6787 | 8167 | 25 |
| PRG14 | 3251 | 4851 | 8102 | 90 |
| PRG15 | 2130 | 5878 | 8008 | 184 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

Exact residuals [unknown.json](disassembly-003/unknown.json). Ready leads verified as bounded structural roles, preserving history: static tails/dispatch/audio; typed mapper/stack/copy/stat/frame/editor contracts; bounded and inspected generic Data. Stronger universal termination/ordinary-return/pointer-preservation claims explicitly excluded. No current direct-control or practical newly exposed literal-consumer frontier; no exhaustive closure or terminal exhaustion claim. The20249remaining bytes include localized ambiguous wrappers1A1D3..1A1D7 and1327C..13280 plus larger physically bounded residuals for next redirect after commentary.

Gameplay evidence reused from scenarios.json: ordinary menus/open game; artificial duel, purchase/install/debit, completed-pitch thirdout, BTM1 acknowledgement, results panel and stable ending publisher screen. No changed conditions justified unchanged replays; complete natural pennant route remains unproved.

Fresh checkpoint maps/pass-003-verified.json generated/pass-003-verified; analysis/pass-003-verified-verification.json confirms byte-identical whole-ROM rebuild. Input ROM unchanged; prior commentary/source preserved. Existing nonblocking NES-001 and newly documented NES-002 annotation performance/missing batch feature and NES-003 saved-map reload-size mismatch; no toolkit edits; [toolkit-issues.md](toolkit-issues.md). Incomplete whole-ROM classification and editorial gates remain.

Disassembly003 checkpoint naming/serialization note: first maps/pass-003.json retained as failed17,815,576byte pretty-export history (reload rejected16MiBsize). Lossless whitespace-only compaction preserves full parsed current/state objects and metadata; fresh maps/pass-003-verified.json validates/rebuilds byte-identically. Future map.save/export pretty output may need the same lossless compaction before reload; see NES-003. Final hashes and preservation checks analysis/disassembly-003/final-audit.json. Current commentary/source SHA unchanged.


Redirect003 active: analysis/redirect/round003-a/checkpoint.json; current-source commentary003 complete. Exact current20,249byte complement verified; structural/noncode and bounded consumer methods active.

Redirect003 combined ready research complete: [handoff](handoff-redirect-003-ready.md), attemptredirect/round003-a. Proposed20,217new bytes (831StaticCode/19,386Data), acceptance pending; exact acceptedunknown20,249unchanged, wouldleave32bytes/12ranges. All-current-Code read audit corrected priornewCode-onlyconsumer scope; bounded field/sprite/pitch/pitcher data containers and coherentlocal routines supported. Zero proposal conflicts;5395directcontroledges clean. ROM/map/state/source unchanged. No terminalexhaustion/editorial/archive claim. Next independentdisassembly, fullcurrent-sourcecommentary,redirect.


## Disassembly004 — verified supported connected batch

ROM classification: 99.99% (262128 / 262160 bytes classified; 32 bytes unknown).

Added20217unique physical bytes:831StaticCode in395instructions and19386StaticData in61intervals. Prior classifications preserved. Independent primary consumers, bounded formats and scoped local routine contracts reviewed in [structural-review.md](disassembly-004/structural-review.md); fresh configured decode/context and boundary artifacts in disassembly-004. No new runtime/natural reachability claims. Corrected1327C pointerconsumer premise;135B5 restoration is conditional on helper return, no universal audio termination requirement.

| Bank | Code | Data/other | Classified | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 16 | 0 |
| PRG00 | 3098 | 5090 | 8188 | 4 |
| PRG01 | 0 | 8192 | 8192 | 0 |
| PRG02 | 2715 | 5475 | 8190 | 2 |
| PRG03 | 1618 | 6574 | 8192 | 0 |
| PRG04 | 4700 | 3492 | 8192 | 0 |
| PRG05 | 6874 | 1315 | 8189 | 3 |
| PRG06 | 2976 | 5216 | 8192 | 0 |
| PRG07 | 5537 | 2652 | 8189 | 3 |
| PRG08 | 3531 | 4661 | 8192 | 0 |
| PRG09 | 7290 | 902 | 8192 | 0 |
| PRG10 | 370 | 7822 | 8192 | 0 |
| PRG11 | 165 | 8027 | 8192 | 0 |
| PRG12 | 4285 | 3906 | 8191 | 1 |
| PRG13 | 1380 | 6807 | 8187 | 5 |
| PRG14 | 3332 | 4855 | 8187 | 5 |
| PRG15 | 2267 | 5916 | 8183 | 9 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

[Exact residual32bytes/12ranges](disassembly-004/unknown.json) retain material code/data ambiguity; no forced classification. Current directcontrol and official absolute-read audit has zero unsupported targets/residual reads; no exhaustive dynamic-selector closure claim. Broad gameplay evidence reused (purchase/install/debit, duel, pitch/thirdout, BTM1, results and ending publisher). No changed scenario justified unchanged replay; complete natural pennant route remains unproved.

Fresh configuredCLI load/validate/generate/rebuild: [pass-004-verified-verification.json](pass-004-verified-verification.json), byte-identical262160-byte ROM. Exact map maps/pass-004-verified.json and generated/pass-004-verified. NES-003 workaround: whitespace-only compaction18164632to11525822bytes preserving full parsed metadata before fresh load; no failed checkpoint attempt needed. Existing [toolkit issues](toolkit-issues.md) NES-001/002/003 remain; no toolkit edits. ROM/prior source/history unchanged; no archive/publication/deletion/cleanup. Whole-ROM classification incomplete; full current-source commentary and eventual editorial gates remain separate.


## Disassembly005 — independent revised structural batch

ROM classification: 100.00% (262156 / 262160 bytes classified; 4 bytes unknown).

Accepted28unique previouslyUnknownbytes:10StaticCode/fourinstructions/threewrappers and18genericStaticData/fiveintervals. All prior regions and full metadata preserved. Primary domain/stack/continuation/boundary evidence jointly evaluated; no natural caller/fetch or extra transformation prerequisite. No observed execution or specific Data schema asserted. [Independent review](disassembly-005/structural-review.md), fresh configured decode and boundary bytes in disassembly-005. Ready R004 leads verified.

| Bank | Code | Data/other | Classified | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 16 | 0 |
| PRG00 | 3101 | 5091 | 8192 | 0 |
| PRG01 | 0 | 8192 | 8192 | 0 |
| PRG02 | 2715 | 5475 | 8190 | 2 |
| PRG03 | 1618 | 6574 | 8192 | 0 |
| PRG04 | 4700 | 3492 | 8192 | 0 |
| PRG05 | 6874 | 1318 | 8192 | 0 |
| PRG06 | 2976 | 5216 | 8192 | 0 |
| PRG07 | 5540 | 2652 | 8192 | 0 |
| PRG08 | 3531 | 4661 | 8192 | 0 |
| PRG09 | 7290 | 902 | 8192 | 0 |
| PRG10 | 370 | 7822 | 8192 | 0 |
| PRG11 | 165 | 8027 | 8192 | 0 |
| PRG12 | 4285 | 3906 | 8191 | 1 |
| PRG13 | 1384 | 6807 | 8191 | 1 |
| PRG14 | 3332 | 4860 | 8192 | 0 |
| PRG15 | 2267 | 5925 | 8192 | 0 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

Exact residual four isolatedRTS at physical418E,47F0,18643,1A42A retain empty-routine versus residue ambiguity; [contexts](disassembly-005/unknown.json). Rounded100.00% does not establish completion. Code total50148bytes; execution coverage unchanged/separate, no new runtime. Prior representative purchase/duel/thirdout/BTM1/results/artificial ending evidence reused. Fresh progression predicate review needs unsupported cartridgeRAM writes; natural fullpennant unverified; no unchanged replay justified.

Fresh configuredCLI load/validate/generate/rebuild byte-identical262160bytes: [verification](pass-005-verified-verification.json), maps/pass-005-verified.json and generated/pass-005-verified. NES003 workaround losslessly compacted save/export JSON with parsed equality/full metadata preserved before fresh reload; compaction.json records sizes. [Toolkit report](toolkit-issues.md): NES001 confirmed documentation/API bug, NES002 missing batch feature, NES003 confirmed serialization/reload issue; none blocks this pass, no new issue/toolkit edit. ROM and commentary source SHA60102fb59892f20ed62a468ceef0e54f3b84372748ef726ae4a9feca86510b1a unchanged. No archive/publication/deletion/cleanup. Server stopped, no live writer. Classification incomplete; full current-source commentary and final editorial audit remain separate/pending.


## Redirect005 — terminal INCOMPLETE

No proposals or ready leads. Refreshed exact residual four single60 bytes:00418E/817E,0047F0/87E0 [2,3,14,15];018643/8633,01A42A/A41A [12,13,14,15]. Primary rejection reasoning and independent source/control-pointer/progression avenues reviewed; coherent empty routine versus residue remains undecided. See [terminal reasoning](redirect/round005-a/terminal-review.md), [compact rejection review](redirect/round005-a/rejection-review.json), [checks](redirect/round005-a/checks.json). Classified262156/262160; no structural mutations. Source readability complete; final editorial acceptance deferred. NES001/002/003 unchanged; no live handles or cleanup.
