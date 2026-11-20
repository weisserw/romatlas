# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 01 disassembly — in progress
Baseline created once and byte-identically verified: [baseline-verification.json](baseline-verification.json), maps/baseline.json, generated/baseline. Configured release binary used unchanged. Mapper 2, 32 physical 8 KiB PRG banks, 262160 total bytes. Reset $CDDC and NMI $D110 validated at file $3FFFA; initial context [0,1,30,31]. Initial bounded static traversal: 196 instruction candidates; not yet annotated. Emulator title identifies Best of the Best: Championship Karate (1992, Electro Brain/Loriciel). Local RPC request/results are append-only in round-01/rpc.jsonl; resumable script round-01/session.py. Startup title scroll through frame ~530 inspected; gameplay still ongoing. Tracing at capacity zero retains aggregates; startup traced 933 contextual opcode roots, 745 physical opcode starts at frame 56; later additions pending. No classification percentage claimed yet. Disk initially 4.5 GiB free. No toolkit edits or cleanup.

Intermediate classification checkpoint (before artificial later-opponent merge): state.json saved, 4102 regions, 10098 code bytes plus 59135 uniform zero data bytes and 652 header/vector/fighter-record bytes. Natural Tsong Po first bout observed and movement/attacks exercised, durable bout.snapshot.json. Recovered 35-byte RAM fighter descriptor at $0426; name at +6. Clean menu-ready.snapshot.json exported; replacing $0426..$0448 from Cogneur ROM descriptor at file 148032..148066 changes menu opponent name to COGNEUR. Later-opponent bout initialization being tested. Artificial evidence will be marked separately. New generic data annotations use exact maximal >=256-byte zero runs, neighboring bytes recorded; no fill purpose assumed. No terminal exhaustion claim; substantial graphics/data and indirect dispatch questions remain for redirect.

## Round 01 disassembly — complete; classification incomplete
ROM classification: 27.38% (71779 / 262160 bytes classified; 190381 bytes unknown).

Current phase handoff: commentary, then redirect. Current internal state analysis/state.json and maps/current.json contain397 evidence-backed regions, after supported coalescing from4985 instruction-level regions. 11992 code bytes and59787 data bytes (header included). Counts exclude unknown raw export fallback. [Counts JSON](round-01/counts.json); [exact unknown complement](round-01/unknown-final.json). No terminal exhaustion decision: concrete research avenues R01-L02..04 remain.

Fresh verified checkpoint: [round-01-coalesced-verification.json](round-01-coalesced-verification.json), maps/round-01-coalesced.json, generated/round-01-coalesced/game.asm. map.validate passed; generated assembly rebuilt byte-identically to unchanged ROM SHA256 2f9be326bec3c4ed127ec17d91dca4a395d2b05ad4581f68990a8b17af6cdd2f. Map SHA256 9ac3052c5cf0acc7b514c33408d98ae69df9efe3cbb4080761e7c763465ef87e. Earlier raw-region round-01-disassembly checkpoint preserved. Whole-current-source readability/editorial review remains pending.

Execution metric separate: 7749 physical opcode starts,12159 contextual roots,12995 contextual edges; trace693 frames retained as aggregate capture over discontinuous snapshot routes,60804332 detail drops intentional at capacity0, aggregate_dropped0. These are observed coverage, not whole-ROM classification. Untraced waits and artificially edited scenes do not establish natural reachability. [Summary](round-01/trace-summary-final.json).

Gameplay checklist: title/menu and natural Tsong Po match visited; movement/two attacks/AI and ring transition exercised. Artificial Cogneur match visited after descriptor, player stats/flags and money patches; active fighters and actual fetches confirmed. Training, later-round victory, terminal ending and secret conversion credits deferred to consumer-contract investigation; concrete low-health/credits secret-name recipes sourced but unverified. [Scenarios](scenarios.json), [gameplay](gameplay.md), [handoff](handoffs/round-01-disassembly.md).

Toolkit issues:2 confirmed documentation/API contract bugs (TK-001 input and TK-002 discovery budget),0 suspected bugs,0 missing features,0 blockers; workarounds used. [Persistent report](toolkit-issues.md). Toolkit not edited. ROM hash verified by fresh checkpoint. Disk use remains bounded; no cleanup/archive/publish performed.

Per-bank classified/unknown counts (each PRG bank8192 bytes; header16 classified separately):
| 8 KiB PRG bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 0 | 376 | 7816 |
| 1 | 0 | 421 | 7771 |
| 2 | 0 | 349 | 7843 |
| 3 | 0 | 377 | 7815 |
| 4 | 0 | 501 | 7691 |
| 5 | 0 | 267 | 7925 |
| 6 | 0 | 291 | 7901 |
| 7 | 0 | 470 | 7722 |
| 8 | 0 | 338 | 7854 |
| 9 | 0 | 320 | 7872 |
| 10 | 0 | 258 | 7934 |
| 11 | 0 | 1022 | 7170 |
| 12 | 0 | 8192 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 0 | 8192 | 0 |
| 15 | 0 | 8192 | 0 |
| 16 | 0 | 356 | 7836 |
| 17 | 0 | 2391 | 5801 |
| 18 | 266 | 631 | 7295 |
| 19 | 5351 | 740 | 2101 |
| 20 | 0 | 23 | 8169 |
| 21 | 0 | 630 | 7562 |
| 22 | 0 | 18 | 8174 |
| 23 | 0 | 433 | 7759 |
| 24 | 898 | 1034 | 6260 |
| 25 | 0 | 8192 | 0 |
| 26 | 0 | 16 | 8176 |
| 27 | 0 | 2968 | 5224 |
| 28 | 0 | 761 | 7431 |
| 29 | 0 | 3399 | 4793 |
| 30 | 2596 | 0 | 5596 |
| 31 | 2881 | 421 | 4890 |

## Round 01 commentary — active
Prepared commentary/ from the coalesced checkpoint; initial independent rebuild passed byte identity. No prior commentary credit exists. Reviewing all 11992 classified code bytes, 59787 classified data bytes and explicit unknown boundaries; final editorial audit remains pending. Frozen baseline and original input preserved.

## Round 01 commentary — current-source readability complete
All currently classified source and unknown interval notes reviewed: 11992 code / 59787 data bytes; 503 scaffold intervals, 370 code intervals, 415 compact units. Source has 412 specific code anchors and 711 flow labels, with stable RAM/hardware/table operands named. No readability range deferred. Fresh final byte-identical verify passed in commentary/verification.json; game.asm SHA256 d05fe2c3b261a49ab2b51c96dc1345404a5db24ba7628e4d760d55221d806992; reading-after.asm refreshed. Final editorial audit remains pending. Map and state unchanged; classification remains 27.38% (71779/262160), 190381 unknown.

Confirmed secret-name ROM bytes `  LAYER   ` at physical 251649..251659 and classified consumer setting second resource to ten at 251672..251678. Full ending/progression still unresolved. Referee action meanings, indirect menu domain, pose/graphics objects and referenced table extents remain redirect research. [Commentary handoff](handoffs/round-01-commentary.md), [uncertainties](../commentary/uncertainties.md). No toolkit changes or cleanup/archive/publish.

## Round 01 redirect — complete-ready
Combined packet in [redirect handoff](handoffs/round-01-redirect.md) updates R01-L02..04. Proposed190374 unique unknown bytes; accepted0. Current map remains round-01-coalesced,71779 classified/190381 unknown. Artificial ending/credits and broad mechanics evidence retained; no unchanged replay needed.

## Round 02 disassembly — active
Input round-01-coalesced; independent exact-contract review of combined199 proposals underway. Prior complete commentary tree preserved at analysis/round-02/commentary-before before any future regeneration. No commentary phase performed. Seven isolated opcode bytes outside packet remain Unknown absent additional entry/layout support.

## Round 02 disassembly — pass complete; ROM incomplete
ROM classification: 100.00% (262153 / 262160 bytes classified; 7 bytes unknown).

Accepted190374 unique previously unknown bytes:13630 Code and176744 Data; total25622 Code and236531 Data including16-byte header. All new provenance Static; natural domains/reachability remain distinct. All199 combined proposal intervals reviewed; split around15 existing Data bytes, preserving original annotations. Independent review and primary constraints: [review](round-02/review.md), [fresh exact decodes](round-02/fresh-decodes.json), [decisions](round-02/review.json), [applied regions](round-02/applied.json). Local FC5E two-word ABI claim rejected: scratch07 is never read;45 executable bytes supported byFF94 veneer/control boundaries, exact comparison convention unresolved.

Fresh map.validate/save/export passed; unique [round-02-disassembly checkpoint](round-02-disassembly-verification.json) verifies unchanged-ROM byte identity. Map SHA256f3e913592366f0db58098a8979d579c519b26589952491f666f31d8cb22b7a0b; maps/round-02-disassembly.json and generated/round-02-disassembly/game.asm. [Exact counts](round-02/counts.json). Current annotation coverage262153 structural bytes in596 regions; raw fallback excluded. Current-source readability covers prior71779 bytes only; new source must receive commentary. Prior complete commentary tree preserved at round-02/commentary-before. Final editorial audit pending.

R01-L02 verified secret/ending, sample and progression contracts using saved artifacts; artificial OLRAC credits/Cogneur winner routes retained without replay. R01-L03 verified finite menu and connected bank-aware continuations/veneer code, with FC5E semantic narrowing. R01-L04 verified bounded graphics/action objects and inspected generic non-code storage. No unchanged runtime experiments or new natural reachability claims. Execution metric unchanged: round01 aggregate7749 physical opcode starts/12159 contextual roots/12995 contextual edges, plus separately retained round01 redirect checkpoint observations; no new runtime coverage claimed.

Remaining seven Unknown bytes:199683..199686,249693..249696,260427..260428. Plausible isolated opcodes lack independent entry/layout contract; do not infer Data or dead code. [Residual review](round-02/review.md). Next phase commentary, then redirect rejection review; no automatic terminal exhaustion claim. Toolkit issues: existing2 confirmed documentation/API bugs TK-001/TK-002;0 new,0 suspected,0 missing features,0 blockers; [report](toolkit-issues.md). No toolkit edits/archive/cleanup/publish.

Header16 Data separately; per-bank exact counts:

| 8 KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
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
| 14 | 0 | 8192 | 0 |
| 15 | 0 | 8192 | 0 |
| 16 | 0 | 8192 | 0 |
| 17 | 0 | 8192 | 0 |
| 18 | 269 | 7923 | 0 |
| 19 | 7364 | 828 | 0 |
| 20 | 0 | 8192 | 0 |
| 21 | 0 | 8192 | 0 |
| 22 | 0 | 8192 | 0 |
| 23 | 0 | 8192 | 0 |
| 24 | 6351 | 1838 | 3 |
| 25 | 0 | 8192 | 0 |
| 26 | 0 | 8192 | 0 |
| 27 | 0 | 8192 | 0 |
| 28 | 0 | 8192 | 0 |
| 29 | 0 | 8192 | 0 |
| 30 | 4394 | 3795 | 3 |
| 31 | 7244 | 947 | 1 |


## Round 02 commentary complete; redirect active
Current checkpoint round-02-disassembly; map f3e913592366f0db58098a8979d579c519b26589952491f666f31d8cb22b7a0b, source 828875cd6c1838e9924571e6f3276aeaacff173193b31cc9ec387dca705f96ac. Current readability complete; final editorial pending. Redirect attempt analysis/redirect/round-02-a investigates exact seven-byte complement via adjoining control/layout contracts and accepted-standard consistency, without map/source mutation.

Round02 redirect round-02-a complete-incomplete, pending coordinator primary-evidence rejection inspection. Proposed0/accepted0; unchanged seven Unknown bytes. Compared concrete menu dispatch duplicate (independent entry7), reset veneer/API layout, graphics word coincidence and return-stack contracts. No useful revised discriminating test survives; details analysis/redirect/round-02-a/rejection-review.md and analysis/handoffs/round-02-redirect.md. No map/source/ROM or toolkit mutation.


## Terminal coordinator handoff — justified incomplete
Separate phase workflow finished after round02 redirect and coordinator primary-evidence rejection review. Final editorial audit passed599 regions/1228 units, byte-identical, reading refreshed. ROM classification: 100.00% (262153 / 262160 bytes classified; 7 bytes unknown). No whole-ROM completion claim. Final checkpoint round-02-disassembly; R02-L01..03 remain unresolved/resumable under explicit reopen conditions. See [terminal handoff](terminal-handoff.md), [completion audit](completion-audit.json), archive-options.json. No archive, cleanup or publication.

## User-directed fallback disassembly — pass complete; commentary pending

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Coverage includes 7 Manual fallback bytes. Prior evidence-backed structural classification remains 262153 / 262160 bytes (99.997329875%); no new structural proof was obtained. Current Code25622/Data236538, provenance Static252971/Observed9182/Manual7. Exact verified pre-change physical complement: 199683..199686,249693..249696,260427..260428. It equals the expected seven bytes and stays within the explicitly authorized <=10-byte exception; no other bytes changed.

Explicit new user authorization relayed by coordinator directed these exact residuals to generic Data with Manual provenance and unknown meaning, as fallback rather than structural proof. Prior unresolved JMP $8A5C / JMP $CDDC / RTS ambiguity is retained, not reversed: menu dispatch supplies no $8BF3 entry; $CF4D has no bounded veneer-slot structure after the returning OAM helper; $F93B lacks an independent stack/control entry after the preceding RTS. Each remains possibly executable stub versus retained instruction-shaped residue. Reused prior practical reference/consumer, neighboring-control and format/layout inspection without research reopening. [Full retained authorization, ambiguity and handoff](handoffs/user-fallback-01-disassembly.md), [primary prior review](redirect/round-02-a/rejection-review.md).

Supported map.annotate applied three Data/Manual regions, preserving every preexisting region. map.validate/save/export passed. Fresh unique `python3 run.py checkpoint user-fallback-01` validated/generated/verified byte-identically; [report](user-fallback-01-verification.json). Map SHA256 e870ff399453b69cc107cf3e5ac76dad6048e16e75910b144cc0a7b1dc0e7525. [Counts](user-fallback-01/counts.json), [per-bank counts including Manual separately](user-fallback-01/bank-counts.md), [pre-change complement](user-fallback-01/complement-before.json), [RPC log](user-fallback-01/rpc.jsonl). Original ROM unchanged. Prior checkpoint/state/map retained. Execution coverage unchanged; current readability/editorial renewal belongs to following commentary phase. Toolkit report: existing2 confirmed nonblocking documentation/API bugs TK-001/TK-002;0 new,0 suspected,0 missing features,0 blockers; [report](toolkit-issues.md). No toolkit edits/deletion/archive/publication.


## Final coordinator acceptance — explicit Manual fallback exception
ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unclassified). Prior evidence-backed262153 plus user-directed Manual7 are separate; no whole-ROM evidence-backed proof asserted. Exact ranges/authorization/prior ambiguity retained in [manual fallback record](manual-fallback-record.md). All596 previous regions unchanged. New checkpoint user-fallback-01, map e870ff399453b69cc107cf3e5ac76dad6048e16e75910b144cc0a7b1dc0e7525. Separate commentary completed all599 intervals/1228 units, then targeted final review and strict audit passed with fresh byte identity; source5e07ad9c960fc4272e652bc7a48e6babd3bee09038821deebb83f69b660606d2, reading current. Coordinator independently verified current map/state/baseline identities, complete inventory, source/audit hashes, original ROM, exact seven-byte annotation limit and unchanged prior records. Both map/store validate. R02 leads stay blocked as structural inferences; no research reopening. archive.py document-selection contract inspected; archive-options.json includes final fallback and accounts every unrecognized Markdown. [Terminal handoff](terminal-handoff.md), [completion audit](completion-audit.json), [independent checks](user-fallback-01/coordinator-final-check.json). Prior terminal records and full commentary preserved. No archive/delete/toolkit edit/publication.
