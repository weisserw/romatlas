# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.


## Pass-001 completed — first structural and gameplay batch

# Disassembly pass-001 handoff

Round1 disassembly complete; ROM incomplete. Ready for full current-source commentary,
then redirect investigation. This is an ordinary easy-gain handoff, not a terminal
exhaustion finding. No prior lead rejections existed. Do not rerun the baseline.

ROM classification: 84.05% (55096 / 65552 bytes classified; 10456 bytes unknown).

Checkpoint: [verification](../../pass-001-complete-verification.json),
[map](../../../maps/pass-001-complete.json),
[current source](../../../generated/pass-001-complete/game.asm).
map/state/current exports validate; pinned ca65/ld65 rebuilding is byte-identical.
Input SHA256 remains a6268aaed181bc2e58de599c89f3a70263feb069ff6c6b5fed926bea8f386c29.
94 regions: 10,150 Code bytes, 44,946 non-code bytes; all regions have evidence.
Static provenance intentionally remains separate from execution reachability.
Annotation labels/comments still need full commentary review; fidelity alone does not
establish readable source or editorial completion.

| Physical bank | Code | Non-code | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG-0 | 5132 | 46 | 3014 |
| PRG-1 | 5018 | 68 | 3106 |
| PRG-2 | 0 | 3856 | 4336 |
| PRG-3 | 0 | 8192 | 0 |
| CHR-0 | 0 | 8192 | 0 |
| CHR-1 | 0 | 8192 | 0 |
| CHR-2 | 0 | 8192 | 0 |
| CHR-3 | 0 | 8192 | 0 |

## Findings and evidence

- Bump 'n' Jump NES, fixed mapper3 PRG context[0,1,2,3]. Validated vectors at
  physical$7ffa point NMI$81f9 and reset/IRQ$8000. Header16 and CHR32768 structurally
  classified. CHR is separate from CPU-executable storage.
- Dispatcher mainstateRAM$14 reads 23 paired handlers atCPU$822c, physical$23c;
  exact endpoint is helper$825a. D1-001 initial control-flow closure produced8747
  code bytes, no incompatible instruction boundaries. Evidence: handler-discovery.json.
- Artificial accepted opcode roots (full mechanics-coverage.json), validated fixed
  physical mapping and static successor closure added1403 code bytes. These prove
  code execution in edited scenarios, never natural selection of every handler.
- Generic data gains:445 uniform bytes,11119 new patterned bytes in$5100-$7de8
  (old uniform intervals retained),546 bytes stage/palette/pattern-tail$7de8-$800a.
  Inspected repeated finite-symbol rows, ordered groups, palette values and boundaries;
  checked known stage/PPU consumers and conflicting code. No exact original object
  layout asserted. data-inspection.txt and uniform-data-evidence.json retain checks.

## Gameplay and reproducibility

Read ../../scenarios.json and ../../gameplay.md. Clean title snapshot and distinct
stage/ending snapshots are durable local artifacts. Named in-memory snapshots do
not survive server restart. All later-stage observations artificial.
Natural route:120 no-input frames,Start2,release120,Right+A180 → stage0 pre-race.
Natural capture summary reports1888 unique opcode starts. Full mechanics artificial
set2437 unique starts (not additive, not a byte percentage); 100 emulated frames
across restored branches, no aggregate drops. Capacity0 deliberately discards detail
while preserving aggregates; coverage/context provenance retained.

Stage7/15 initialized using$14=6,$15/$16=0,$56=7 or15,$3d=0,$4d=3.
After8 frames state9; set$15=127;Up24,A+Right4 → state11.
From initialized state11 set$15=48,$a9=240;Up12,A+Right8,B+Left8
→ verified state12 driving at early/middle/last courses. No successful jump claim.
Additional courses1/3 scenery visited in state11; their faster shortcut ran too early
and was reset by state11 initialization. An applied stage number alone was never
reported as a visit: actual captures/RAM/fetches accompany these results.
Ending shortcut: restoretitle, $14/$15/$16=[21,0,0],release buttons,16frames →
rescue animation/state22. Full ending return and boss encounter remain deferred.

Budgets: each frame request<=20, max_operations2000000,timeout30000ms.
Initial180-frame request timed out and is superseded by clean chunked run.
No speculative RAM brute force or PC edits. Discovery max20000 instructions,
1000-item pages. All subprocesses exited; no live server state is needed.

## Redirect frontier

D1-003 sound command dispatch physical$3ce5 and unknown$400b-$5100 is a concrete
connected research avenue. D1-004 actor/gameplay inner dispatches need exact selector
and table/entry contracts. D1-005 ending table$90d3 and stage15 finish/boss transition
need supported state prerequisites. D1-006 small screen/audio/data gaps should be
classified after bounded consumer/pattern checks; semantics are not mandatory.
No lead was rejected or declared exhausted. The unresolved inner indirect targets
and mixed data/code ranges require redirect investigation beyond this first easy
static/representative-play batch. Exact64-gap complement is unknown.json.
Do not replay long early routes: restore a successful initialized snapshot.

No toolkit issues found; persistent report ../../toolkit-issues.md.
No toolkit edits, archive/deletion/publication, or external ROM/assets downloads.
Historical pass-001 was an accidentally empty-map fidelity checkpoint; pass-001-static,
pass-001-final and pass-001-handoff are intermediate verified states. Use
pass-001-complete as authoritative phase checkpoint.

## Round 2 disassembly complete — pass-002-reviewed

ROM classification: 99.98% (65537 / 65552 bytes classified; 15 bytes unknown).

Independently accepted 4,057 new Code and 6,384 new Data bytes from D1-003..D1-006. 2,008 instruction starts/lengths freshly decoded under fixed CNROM PRG[0,1,2,3], no conflicts; original regions preserved. Total Code 14,207 and non-code 51,330 bytes in161 evidence-bearing regions. New annotations Static, with local data roles and primary contract links. Reachability remains separate.

| Physical bank | Code | Non-code | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG-0 | 7187 | 991 | 8178 | 14 |
| PRG-1 | 6944 | 1247 | 8191 | 1 |
| PRG-2 | 76 | 8116 | 8192 | 0 |
| PRG-3 | 0 | 8192 | 8192 | 0 |
| CHR-0 | 0 | 8192 | 8192 | 0 |
| CHR-1 | 0 | 8192 | 8192 | 0 |
| CHR-2 | 0 | 8192 | 8192 | 0 |
| CHR-3 | 0 | 8192 | 8192 | 0 |

Exactly15 bytes remain unknown in five end-exclusive ranges:007c..0080,0238..023c,049e..04a3,1219..121a,3b8a..3b8b. [Independent review](disassembly/pass-002/review.md) evaluates split stub/return/BRK versus data claims without requiring universal live callers. No new discriminating constraint found; no arbitrary-PC or unchanged gameplay replay. Original runtime recipes cover artificial early/mid/late, final opponent/contact/defeat,21 ending text records/return and airborne jump. Landing, natural defeat, ordinary-timing full ending and hazard permutations remain unclaimed/deferred without a residual classification question.

Fresh map.validate/save/export and [pass-002-reviewed verification](pass-002-reviewed-verification.json) passed; source rebuild byte-identical. Exact map/state/current SHA2561d8769b7ce8b13320c632c6204d6d8e894866768abe1266531c67b2633a613e2; ROM unchanged. [Counts](disassembly/pass-002/counts.json), [unknown complement](disassembly/pass-002/unknown.json), [integrity](disassembly/pass-002/integrity.json), [handoff](disassembly/pass-002/handoff.md).

ROM incomplete. Next is full current-source commentary reconciliation for the enlarged classified source; preserved commentary/source and frozen baseline are unchanged. Final editorial audit pending. No commentary/redirect phase performed here. No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, cleanup, archive, deletion or publication. All owned analysis servers exited;3.3GiB free.

## Redirect round 2

R2-001 ready: corrected BIT-derived zero/conditional default-course inference supports a four-byte split candidate049f..04a3, excluding priorRTI. See [findings](redirect/round-2/findings.md) for decisive primary artifacts, competing interpretations and review of all other residuals. Actual15unknown unchanged;4Code proposed,0accepted. No runtime replay or terminal exhaustion claim.

## Pass-003 outcome

ROM classification: 99.98% (65541 / 65552 bytes classified; 11 bytes unknown).

R2-001 verified after independent exact-boundary/semantic review:4 Static Code bytes049f..04a3; actual caller/natural reachability unknown,049e RTI separate. See [review](disassembly/pass-003/review.md), [handoff](disassembly/pass-003/handoff.md), [counts](disassembly/pass-003/counts.json) for per-bank counts and preserved primary evidence. Fresh map validation and byte-identical pass-003-reviewed verification passed. Remaining11bytes retain localized ambiguity; no unchanged replay. Full source preserved for next commentary. No toolkit issues found.
