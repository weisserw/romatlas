# Round 01 disassembly handoff

Phase complete; ROM classification incomplete. Run commentary next, then redirect. Do not start a second disassembly pass from this handoff alone.

ROM classification: 48.71% (63847 / 131088 bytes classified; 67241 bytes unknown).

12750 Code / 51097 Data. Unique physical-byte counts, header included; all remaining gaps unknown. Per-8KiB-bank code/data/classified/unknown counts and exact unknown complement: [final-counts.json](round01/final-counts.json), also progress.md and investigation.md.

Authoritative state `analysis/state.json`; portable mapping `maps/current.json`; corrected checkpoint `maps/round01-disassembly-corrected.json`; current source `generated/round01-disassembly-corrected/game.asm`. Verification `analysis/round01-disassembly-corrected-verification.json`: ok=true, byte_identical=true; generator diagnostics empty. ROM SHA256 unchanged. Earlier `round01-interim` / `round01-disassembly` and pre-audit records are historical; their false static table fallthrough is superseded.

Evidence and boundaries:
- Accepted natural/artificial instruction fetches and exact effective banks in `round01/rpc-live.jsonl`; storage identity separates RAM from ROM. RAM execution was excluded, not attributed to an upload origin. `classification-candidates.json` is initial/superseded where audit says so.
- Current static closure uses direct branches and inspected JSR callee contracts, with mapper effects/calls/indirect targets stopping automatic propagation. `expansion-evidence.json` records current accepted closure. `fixed-resume-evidence.json` adds fixed-window sequential continuation after writes using UxROM invariant banks14/15 and null lower slots; no invented return context. Legal decoding alone is not evidence.
- `data-inspection.json`: practical reference/consumer inspection plus every256-byte-page pattern check justifies five partial repeated matrix/attribute intervals and exact uniform runs; generic non-code Data, no unsupported full-table extent/padding/dead claim. Header and vectors independently parsed.
- Critical audit: physical `$17FDB..$17FDE` contains four indexed sprite-layout values after complementary BNE/BEQ; no ordinary fallthrough. False BPL/JSR interpretation and dependent fake callee `$14A38` withdrawn. Observed routine begins `$17FDF` / CPU `$BFCF`. `audit-correction.json`, `pre-audit-state.json` and `*-pre-audit.json` retain correction history. A bounded complementary-branch scan found no other static-only post-pair continuation requiring this correction. Current `$14A38` is unknown; no code remains in corrected table. Do not resurrect this path from historical source.

Gameplay and contracts:
- Game locally identified as The Blues Brothers, Titus copyright1991, mapper2/128KiB PRG/CHR RAM. Exact region/revision unresolved. Manual/cheat sources and scope in gameplay.md.
- Natural copyright/title/chapter1 intro and run/jump. `chapter1-ready.json` and `pre-patch.json` durable clean snapshots.
- Artificial `$0750=stage`, instruction boundary, PC `$F5A2`, SP `$FF`; wait20, Start3/release75, right+A+B15/release exposes warehouse(stage1), prison(stage2), sewer(stage3). Restore pre-patch before each and release BOTH controllers. Distinct scene/action captures and snapshots retained. `progression.py` is exact reusable recipe.
- Construction(stage4) attempted; +40 right+B after energy `$042A=3` / invincibility hypothesis `$03C2=2` changes scrolling/HUD but rendering artifacts persist. Clean construction visit not established.
- Artificial ending: `$37=$0750=5`, instruction boundary, PC `$F50A`/SP `$FD`, wait50; Start3/release50; another Start3/release50. Welcome-to-band/staff credits and player/color animation appear with scroll/right-edge artifacts. Terminal system observed, no clean concert fidelity or natural completion claim.
- Built-in two-controller level-skip was attempted and did not change stage. No blind repetition warranted.
- `scenarios.json` contains milestones, exact patches, routes, outcomes and snapshots. Runtime totals8562 unique instruction identities/12113 contextual roots include RAM and artificial scenarios; not a classification metric. Detail drops intentional at capacity0; aggregate_dropped0. Inputs/budgets/timeouts and captures retained.

Interpretation roots for commentary (cover ALL classified source, including fragmented code/data):
- Fixed CPU `$F499` reset and `$FC33` NMI; helpers `$F481/$F486`, resource selector `$F678`, main stage selector `$37` / persisted chapter `$0750`, `$F50A` terminal decision, `$F5A2` chapter initialization. `$042A/B` energy reset stores, `$0428/9` life hypotheses and `$03C2/3` invincibility hypotheses remain distinguished.
- `$BFCF` sprite buffer allocator increments X by4, stores `$86`, overflow path pops two return bytes then RTS; preceding OAM sprite builder uses `$0200..$0203,X` and four offset bytes. Preserve stack-unwind contract when explaining callers; no universal normal-return assumption.
- Bank12/13 routines use Y-indexed object state fields; direct handler contracts can support unobserved code but pointer arrays alone do not.
- Generic matrix/uniform Data needs honest role/evidence comments; unresolved compressed formats should remain described as unresolved, not invented semantic schemas.

Redirect frontier in leads.json:
1. R01-RESOURCE-BOUNDS: adjacent compressed graphics/resources after partial+$2400 stage-bank slices; inspect selector/loader and narrowly trace physical reads on saved chapter transitions to prove source/lengths.
2. R01-STATE-HANDLERS: bank6$8000 Y-indexed object handler contracts, dispatch consumers and bounded arrays; substantial static expansion plausible.
3. R01-SCREEN-INITIALIZATION: compare normal transition against shortcut, inspect `$FB80` and preceding PPU/scroll setup; fix entry contract instead of arbitrary RAM or repeated long play.

No toolkit issues found; toolkit-issues.md preserved and updated. Socket prohibition was environment-only; stdio worked. Artificial display artifacts are not a demonstrated emulator defect. No toolkit edits, cleanup/deletion, archive, publishing or communications outside coordinator occurred. Evidence~102MiB, scratch12KiB, free disk~3.8GiB. Python PTY session36167 retains emulator/RPC function; snapshots are durable if it ends. Analysis pass is intentionally finished; coordinator should prefer persistent state/checkpoint over live emulator.
