# Round 1 / disassembly handoff

Scope: first easy-gain disassembly pass on a fresh workspace (Casino Kid 2, NES mapper 2,
128 KiB PRG, CHR RAM). Baseline created once (maps/baseline.json, generated/baseline).

Checkpoint: **pass-001** — maps/pass-001.json (map sha256 78c994c36016e0bf743363f93ed0bf103cc4b392a175acf4f5d5507cd951dcdd),
generated/pass-001, analysis/pass-001-verification.json. ROM sha256 335680fc...abfa.

Verification: map.validate ok; maps/current.json identical to analysis/state.json;
`run.py checkpoint pass-001` validated, generated and verified: ok=true, byte_identical=true.

ROM classification: 87.31% (114447 / 131088 bytes classified; 16641 bytes unknown).

Status: incomplete. Per-bank counts: analysis/progress.md ("Round 1 / disassembly").

Findings:
- Bank switching: single fixed-bank routine $E30E (alias $C025), bank number in zp $14,
  bus-conflict table $E31B. Fixed-bank API jump table $C010-$C120; each switchable bank has
  `jmp $C019` at $8000 and an entry jmp table at $8010 (bank 2: $8012).
- Inline-parameter callees (3/5/2/4-byte) and inline jump-table dispatchers $E8DD/$E8F5;
  details and rules in analysis/investigation.md.
- Code: 19,637 B observed + 17,335 B static (local recursive descent, toolkit-validated views).
  Data: 67,556 B generic (window illegal-opcode rule) + 9,919 B padding. Specific data roles
  mostly unidentified (generic).
- Gameplay: boot, title, story, world map, travel, blackjack (all choices), roulette betting,
  password screen visited; poker locked; opponent defeat/ending not reached (analysis/scenarios.json).

Artifacts: analysis/investigation.md, analysis/leads.json, analysis/scenarios.json,
analysis/gameplay.md, analysis/toolkit-issues.md, analysis/tools/round01/ (scripts),
traces/round01/ (coverage/context/edge dumps, gzipped same-build snapshots).

Leads consumed: none (no prior leads). Produced: R1-L001 ($C5CE table/records, 1888 B),
R1-L002 (bank 4 $8DDA-$90D0), R1-L003 (bank 5 $831B), R1-L004 (bankroll/progression RAM),
R1-L005 (roulette spin input), R1-L006 (bank-4 inline callees), R1-L007 (specialize graphics/text
data), R1-L008 (residual small ranges).

Unresolved questions:
- Displayed bankroll / defeat / unlock variables (needed for later opponents, poker, ending).
- Whether low-rate data windows (24-35% illegal) hide short unreached code heads.
- Extents of the 13 inline dispatch tables (heuristic, confidence 0.8).

Deferred: commentary on generated/pass-001; redirect on leads above; specific data typing.
Toolkit: 2 confirmed bugs (TK-001, TK-002), 2 missing features (TK-003, TK-004); none blocking.
