# Round 001 disassembly handoff


ROM classification: 63.07% (41346 / 65552 bytes classified; 24206 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG0 | 1358 | 0 | 6834 |
| PRG1 | 1947 | 1195 | 5050 |
| PRG2 | 3936 | 0 | 4256 |
| PRG3 | 65 | 61 | 8066 |
| CHR0 | 0 | 8192 | 0 |
| CHR1 | 0 | 8192 | 0 |
| CHR2 | 0 | 8192 | 0 |
| CHR3 | 0 | 8192 | 0 |

Fresh checkpoint: `maps/round-001-disassembly.json`, generated source
`generated/round-001-disassembly`, verification
`analysis/round-001-disassembly-verification.json`: valid map, byte-identical rebuild.
Current state `analysis/state.json` and portable mapping `maps/current.json` saved.
2,431 distinct physical opcode starts observed across sampled milestone frames;
3,625 starts structurally classified (7,306 code bytes). Sampling excludes untraced
transit; runtime and classification coverage are separate. All classified code decoded
to official instructions with coherent control flow; no unsupported opcode paths adopted.
All regions have structural evidence; routine/variable commentary remains next phase.
Ending: artificial clear-transition entry from final boss yielded castle/rescue montage
through 3,180 frames; no natural boss-defeat or final terminal-credits claim.
One missing feature TK-001, no confirmed/suspected bugs, no blocker; see
[toolkit issues](toolkit-issues.md).
Handoff: [round-001-disassembly-handoff.md](round-001-disassembly-handoff.md).
Next phase: commentary, then redirect on inline-script/bank-effect contracts.

Scope: one easy-gain pass; baseline preserved. Normal input paths visited initial room,
restart/life loss, midgame, late game, boss and expert mechanics. Runtime route scripts,
RAM dumps, screenshots and snapshots remain local in `analysis/round-001/`.
Exact replay scripts: explore1/2/3.py, late.py, ending.py, credits.py.

Structural evidence is separate from reachability: static vector/branch/direct-call
paths were adopted without claiming observed execution. `contexts-ending.json` and
`contexts-credits.json` describe artificial PC-entry observations; other context roots
come from normal password/controller scenarios. Generic data $3440..$38EA is a
supported non-code slice of repeated tile-like/low-domain layouts after nearby-reference
and conflict inspection; semantic object boundaries remain unresolved. Zero tail
$7FC8..$7FFE is bounded by prior JMP and reset stub. CHR classified from iNES and
NES planar pattern-backing role.

Deferred connected work: L001 inline-script entry $801A can consume bytes following
JSR (example physical $1B74); determine its exact return-stack contract before treating
call continuations as executable. L002 mapper writes at physical $53A1/$53A7 select
CHR registers 0..5, but deriving safe call-preservation summaries must account for
indirect dispatch and script continuations. Current recursive direct-call reseeding
has exhausted its bounded connected batch; blind fallthrough would misclassify data.
Remaining unknown complement is `analysis/round-001/unknown-after.json`.
This is an ordinary useful handoff, not terminal exhaustion; no terminal rejection
review is asserted. ROM unchanged and all historical artifacts preserved.
