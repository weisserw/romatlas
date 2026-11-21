# Round 2 disassembly handoff

Scope: one disassembly pass on pass-001 consuming ready leads L-001, L-004, L-008, L-009, L-010 (recipes from round-1 redirect r1-a1).
State restored from analysis/state.json (== pass-001); no baseline recreated; emulator not started (saved traces reused, no new gameplay).

- Checkpoint: **pass-002** — verification ok, byte_identical true (analysis/pass-002-verification.json; maps/pass-002.json; generated/pass-002).
  Map sha256 79bafc80194eafdc9f8b03cad3b7ed29e7a5e1dcb47ddf5909392c004098379b (== maps/current.json == analysis/state.json).
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
- Code 24897 B (1005 regions: 602 Observed, 403 Static); data 106189 B (281 regions); text 2 B (1). Per-bank table: analysis/progress.md.
- Status: incomplete as a project only because commentary/editorial carry-over is pending; classification contract met.

## Changed ranges vs pass-001 (commentary carry-over; full list analysis/pass-002/changed-ranges.json)
- Bank 0: 0x1660+52 data -> 0x1660+20 generic, 0x1674+26 BossActionTable ($9664), 0x168E+6 generic ($967E, words beyond bound).
  0x1793+35 -> BossBounceOffsets 0x1793+31; NEW CODE 0x17B2+2 BossActionMoveSpeed3 ($97A2), 0x17B4+2 BossActionMoveSpeed2 ($97A4).
- Bank 5: new data PasswordCursorSprite 0x15518+5 ($9508), SoundTestCursorSprite 0x160C1+5 ($A0B1), VsMatchPointTiles9 0x178F9+6 ($B8E9),
  VsMatchPointTiles 0x178FF+7 ($B8EF), text IntroScoreText 0x179D7+2 ($B9C7). 0x140DB+157 -> StageItemChoices 0x140DB+144 ($80CB) +
  ItemChrIndex 0x1416B+13 ($815B). StageTimeTable 0x15844+96 ($9834) labelled.
- Bank 6: 0x1B037+6 ($B027) generic data.
- Fixed: 0x1E010+8040 -> DmcSamples 0x1E010+2977 ($E000), DmcSamples_EBC0 0x1EBD0+689, DmcSamples_EE80 0x1EE90+497, DmcSamples_F080 0x1F090+1281,
  DmcSamples_F5C0 0x1F5D0+2465 (Observed) + generic gaps 0x1EBB1+31, 0x1EE81+15, 0x1F081+15, 0x1F591+63, 0x1FF71+7.
- L-009: 129 Static code regions split into 476 pieces at executed/unexecuted instruction-run boundaries + 53 same-extent Static->Observed.
  Byte classification unchanged; original labels kept on first piece. Commentary names/branch labels by CPU address remain valid, but new
  region boundaries appear mid-routine (region headers/.org) — re-run commentary prepare/annotate against pass-002.
- New map labels (avoid duplicates in commentary D/R tables): BossActionTable, BossBounceOffsets, BossActionMoveSpeed2/3, StageItemChoices,
  ItemChrIndex, StageTimeTable, PasswordCursorSprite, SoundTestCursorSprite, VsMatchPointTiles9, VsMatchPointTiles, IntroScoreText, DmcSamples*.

## Findings
- All five ready leads verified and applied (details analysis/progress.md, analysis/investigation.md, analysis/leads.json history).
- L-009 byte totals: 5418 B Observed (natural windows), 1859 B Observed artificial-only (evidence text: natural reachability not claimed),
  2803 B remain Static within split regions. Scripts: analysis/pass-002/{apply_leads.py,provenance_l009.py,apply_l009.py}.
- tools/tables_spec.py: BossActionTable SPEC entry added; $97EF EXTRA_ROOTS note updated (referenced by entry 12).

## Unresolved / deferred
- Reachability of 9526 B Static code (incl. $C149,$C21B,$C8DA,$C92A,$A94A, K22 $93E5, $97A2/$97A4) — L-006/L-011 blocked on a caller/producer.
- Generic data meanings (music/level data) and sound byte-code semantics — L-007 (commentary).
- Commentary: apply L-007 items (DMC sample map, password names, BossActionTable entry names, BossActionBlink comment fix) and carry names
  across the split regions. No toolkit issues found this pass (TK-001..TK-005 unchanged; analysis/toolkit-issues.md).
- Blockers: none. Server stopped; scratch temp removed.
