# Round 03 commentary handoff

Full CURRENT-source readability complete; whole-ROM classification INCOMPLETE.
100.00% rounded (262156/262160 bytes classified), four unknown bytes.
Checkpoint pass-003-disassembly-20261031-b; map/state SHA256
8fac0ec90e59abe29b84a73cee9875b8e89ff5d7c5841e626579807c6a56138c unchanged.

All 39 previous commentary files preserved under analysis/round-03/commentary-round02/.
Fresh baseline/scaffold/mapping metadata; maintained source/symbols carried with
8,270 valid reviewed physical partitions. Full inventory now: 8,271 reviewed records,
678 code units, 409 structures, Code39605/Data91479/Image131072, 18,219 instructions.
No readability ranges deferred; scope assessment: commentary/readability-round03.json;
exact carry/link checks: commentary/research/round03-scope-check.json.

New source game.asm:12361 BrawlEnterGameOverState explains the three-byte Static
JMP at physical C1E2, CPU81D2/[6,7,14,15], including E84B overlapping entry and
ordinary return semantics; no observed/natural entry claim. BrawlRestartStage unit
ends before the new entry. ContinueRestart at game.asm:64901 now correctly describes
counter reset/fallthrough into the classified stage-transition entry. Existing
life-exhaustion extra-frame explanation remains. All carried source references
(16,865) still point to identical lines; no source naming/footer ownership drift.
Exact shared-tail citation is 1EAF2, not 1EAE2.

Unknown physical half-open ranges: C7A6..C7A7,14379..1437A,1ACD0..1ACD1,1EE00..1EE01.
All isolated60 alternatives/limits explained. Prior semantic uncertainties retained;
no new discriminating structural lead or toolkit issue. Existing NES-001 remains.
Fresh commentary/verification.json byte-identical; reading-after.asm refreshed.
Source SHA256 game.asm 1dbaafc64e68e62c8bb4f226dda909cd051f00fa4ce34cc28a1be2fa7f945709;
symbols.inc 582db20960dcf3c79b8195328b855f5652a88675cd4765be80de2b8682ce300f.
Input ROM unchanged. Final editorial/archive audit remains PENDING after redirect/
coordinator terminal review; audit was not invoked. No toolkit edit, archive,
publication, deletion/cleanup, download or map/state mutation.
