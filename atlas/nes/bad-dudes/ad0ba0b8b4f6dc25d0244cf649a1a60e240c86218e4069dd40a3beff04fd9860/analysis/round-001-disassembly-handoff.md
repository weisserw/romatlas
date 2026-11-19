# Round 001 disassembly handoff

Pass complete; ROM incomplete. This is a sequential disassembly phase boundary,
not terminal investigation exhaustion.

ROM classification: 63.16% (165586 / 262160 bytes classified; 96574 bytes unknown).

22423 Code; 12091 Data; 131072 Image; 1208 classified regions, 390 unknown gaps.
[Exact counts/provenance](round-001-stats.json), [per-bank counts](round-001-counts.md),
[exact unknown complement](round-001-unknown.json). Header16/CHR131072 fully typed;
PRG banks10/11 have no code classified yet. All classified intervals have evidence.
Static152438 bytes, Observed13148 bytes. Runtime union7652 ROM opcode starts,
16540 bytes in fetched instructions; natural subset6561 opcode starts. These
coverage metrics do not replace classification or prove natural later-stage use.

Checkpoint: [map](../maps/round-001-disassembly.json),
[source](../generated/round-001-disassembly),
[verification](round-001-disassembly-verification.json).
Internal map.validate and portable map validation passed; generated assembly
rebuilds byte-identically. SHA256 remains
ad0ba0b8b4f6dc25d0244cf649a1a60e240c86218e4069dd40a3beff04fd9860.
State saved analysis/state.json; portable current map maps/current.json. Baseline
also verified. Full current-source commentary/editorial readability remains next.

Natural title identified Bad Dudes, US licensing screen; natural City reached.
Artificial but initialized visits to Big Rig/Sewer/Forest/Train/Cave/Factory
confirmed distinct layouts and accepted move/attack/jump fetches. See
[scenarios](scenarios.json), [gameplay sources](gameplay.md), later-montage.png.
Correct controller schema: {buttons:[...],port:1}; [] releases. Earlier
{button,pressed} attempts silently did nothing and were superseded.

Reusable safe stage recipe: start emulator, import transition-2-snapshot.json
(frame2998), reassert buttons[], run_until memory$09==$20 (55604 operations,
max1000000, timeout30000ms), export/import stage-init-hook-snapshot.json. At hook
write only RAM$02C0=1..6; step180frames. Fresh sessions need map.load and
emulator.start before snapshot import. Snapshot files contain local ROM state;
keep local. See stages4.py/stages5.py and RPC transcript for exact requests.

Early briefing level edits froze; replaying RAM effects of initialization on City
preserved execution but did not load new backgrounds. Those narrowed rejections
led to the successful hook. Level7-at-hook freezes and is not a valid ending.
Factory health/time-assisted1200-frame progression reached later enemies, but no
boss/elevator/ending confirmed. Patches and budgets are in scenarios.json. Do not
repeat failed recipes unchanged; preserve the missing transition contract.

Final loader window3060..3092 and caller-derived static closure supplied connected
code. Static calls preserve pre-call entry banks, stop at mapper/indirect barriers;
no callee return-bank assumptions. Validated vectors added no further bytes.
Uniform00/FF runs have exact adjacent-boundary inspection; only bounded non-code
runs typed Data, without invented fill/sample semantics. Stage selector data at
physical$1C906..$1C911 has consumer and neighboring routine boundaries; full table
schema/extent beyond demonstrated prefix is not claimed. Embedded bank5 routine
physical$B9B6..$BA0A is real static code, so data-bank typing needs care.

[Leads](leads.json): L001 tile/map non-code boundaries in PRG2–5; L002 Factory
boss/elevator/ending state contract from factory-progress-snapshot.json and
$02C1 consumer physical$1C9B2; L003 bank0–1 sound/DMC sample consumers;
L004 boss/actor dispatch into banks10–11. L005 safe stage hook verified; L006
unsafe early/out-of-range recipes rejected narrowly. These are concrete redirect
questions, not proof remaining bytes are exhausted. Saved candidates/contexts
and round-001-rpc.jsonl support reuse without repeating accepted experiments.

Toolkit: [NES-001](toolkit-issues.md), one confirmed documentation/API input-schema
mismatch; zero suspected bugs or missing-feature findings; workaround works,
no blockers. Toolkit unchanged. [Progress](progress.md) and
[investigation/rejection review](investigation.md) retain the full phase history.
