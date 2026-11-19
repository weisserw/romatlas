# Round 1 — redirect handoff (2026-11-18), attempt r1-a

Status: READY LEADS for one disassembly/commentary cycle. Input checkpoint pass-001
(maps/current.json == maps/pass-001.json, 98.20%, 443 unknown bytes in 17 ranges).
Map, state, ROM, source and binary unchanged (redirect writes research/leads only).
Research dir: analysis/redirect/r1-a/ (ledger.json = exact unknown complement with
dispositions; tools wordsearch/brsearch/where/tilecheck/sq1_test; leads.before.json).

## Findings
- Reachability (all 17 ranges): byte search of PRG for JSR/JMP operands, relative-branch
  targets, 16-bit words equal to target and target-1 (covers inline task words and
  PHA/PHA/RTS dispatch) finds only incidental bytes; computed-control paths (E99A/E9B1
  JMP ($0000) from return addresses, EA1A, EB94 task dispatch) cannot produce these
  targets; no fetch in traces. => Reachability NONE FOUND. Gameplay (incl. menus) cannot
  expose callers; static classification per goal.md structural-evidence rule instead.
- SQ-3 table extents: RunFrameFirst/Last DBF6-DC05 proven 2x8 by consumer AND #$07;
  FielderReadyFrames DFEB-DFEE = roles 1..4 (all role writers produce bases 1..4);
  E01D-E036 consumer-anchored (E111/E0D5/E0D8/E0E0/E0ED/DB6A), contains KIL bytes.
- New fact: $CB is written only by SetCamera from CameraTables (record byte 4):
  $00 infield / $06 outfield. D810 indexes A+$CB into three 12-byte tables
  (= 2 cameras x 6) that abut exactly and end at observed D85B.
- SQ-1 confirmed: WritePpuInline (EA05) inline length = 1+count; EAF8-EAFE are
  count 6 + "CALLED"; RTS at EAFF. EA05 was the only JSR-GetInlineArgs routine missing
  from the traversal's inline contract list. Artificial mechanism test passed
  (sq1_test.py: 6 loop iterations, return to EAFF; artificial, mechanics only).
- SQ-2/SQ-4: every fragment except E5F4 begins right after an unconditional transfer
  of accepted code, tiles exactly with official opcodes, all branch targets in-slice or
  known entries (tilecheck.py), and supplies/uses established contracts (details per
  lead). C959 calls SetBallPositionInline (C963), not the velocity routine.
- Extra innings do not exist (C6CC: HalfInning==$12 -> GameSet unconditionally);
  scenario r1-extra-innings marked unavailable.

## Ready leads (analysis/leads.json; all Static provenance, context prg_banks [0,1,0,1])
| Lead | Physical ranges -> proposal | Bytes |
|---|---|---|
| L-001 | 0x1CA1-0x1CBB Code, 0x1CBC-0x1CBE inline Data (task entry $DC97), 0x1CBF-0x1CC9 Code, 0x24C8-0x24D1 Code | 51 |
| L-002 | 0x1820-0x1846 Code, 0x1847-0x186A Data (3x12 tables) | 75 |
| L-003 | 0x1C06-0x1C15 Data (2x8), 0x1C16-0x1C38 Code, 0x1C39-0x1C40 Data | 59 |
| L-004 | 0x1FFB-0x1FFE Data, 0x1FFF-0x202C Code, 0x202D-0x2046 Data | 76 |
| L-005 | 0x0969-0x096B Code + 0x096C-0x0971 inline Data + 0x0972 Code; Code 0x22B5-0x22E2, 0x2751-0x2755, 0x27CC-0x27DA, 0x2832-0x2837, 0x28C7-0x28E9, 0x29D2-0x29EA, 0x2A0C-0x2A14, 0x2B4C-0x2B50, 0x2BC4-0x2BCB, 0x3ABE-0x3ACE | 181 |
| L-008 | split region 0x2AEE+44: Code 0x2AEE-0x2B07, inline Data 0x2B08-0x2B0E, Code 0x2B0F-0x2B19; add EA05 to inline contracts | 0 net |
| L-010 | commentary corrections/naming (ClearBallPosition, $CB, reachability wording) | — |
Expected effect if all accepted: 442 bytes -> 24591/24592 (99.996%); residual E5F4.
Evidence strings: record "no known caller; reachability unknown" — never Observed/dead.

## Avenues investigated / deferred
- Callers/pointers for every range: exhausted by byte search (negative; scope = ROM-encoded
  references). Gameplay exploration for callers: not useful (no reference can exist).
- Substitution/pinch menus, other teams: deferred (L-006 broad progression; needs control
  research; cannot affect unknown ranges).
- Role-5 stub (DFFF, LDY #5) reading DFEF as a frame and E4B8 point semantics: meaning
  unresolved; does not affect code/data split (commentary uncertainty).
- Artificial tests of other fragments: not run; would show mechanics only and add nothing
  to the static contract evidence.

## Rejection review — residual E5F4 (0x2604, 1 byte, L-009)
- Claim considered: Code (unreferenced RTS) or generic Data.
- Decisive evidence: byte $60 between DrawTitleCursor's RTS (E5F3) and Random (E5F5,
  observed); no JSR/JMP/branch/word reference; no data consumer (word F4E5 only at EC36
  inside sprite-list data).
- What it rules out: any caller-based or consumer-based role; does not rule out either role.
- Independent constraints together: single byte, no internal structure, no repeated
  pattern or fill run (neighbors are code), no alignment convention found in this ROM.
- Strongest competing interpretation: leftover RTS of a removed/early-exit routine vs.
  assembler filler byte — indistinguishable from available evidence.
- Outcome: inference insufficient (evidence-policy: unreferenced return opcode alone lacks
  structure); keep Unknown. Reopen only with a new reference/consumer or padding convention.

## Next step
Another disassembly cycle IS warranted: apply L-008 then L-001..L-005 (validate with
analysis/redirect/r1-a/tilecheck.py), checkpoint pass-002, then commentary (L-010) and a
short redirect to confirm only E5F4 remains.
