# Round 1 commentary handoff

Status: complete for the current classified source (not a whole-ROM completion claim).

Checkpoint: commentary-001 — maps/commentary-001.json, generated/commentary-001,
analysis/commentary-001-verification.json (validate ok, byte_identical=true).
Commented source: commentary/source/game.asm (verify: byte_identical=true,
commentary/verification.json); reading copy: commentary/reading-after.asm.
Map vs pass-001: identical classification; +271 region labels from commentary names.
The commentary baseline (commentary/baseline.json) is the pass-001 map; a later round
should treat a map that differs only by these labels as unchanged classification.

Scope covered: all classified source (banks 0, 6, 7 code: 8256 instructions; all data
blocks incl. banks 1-5 and header) plus notes on all 24 unknown blocks. review.json:
711/711 entries reviewed, 944 units; editorial_audit pending (final acceptance only).
Remaining unreviewed ranges: none.

Readability: routines named with block headers; RAM/hardware/constant symbols throughout;
inline comments on complex gameplay routines; inline args and display lists structured,
ASCII sources as strings.  Tentative areas listed in commentary/uncertainties.md
(fight reaction names, cast phase meanings, ranking rules, sound ids, bank-5 formats).

Reproduction: commentary/tools/annotate.py + spec.txt rebuild source/ from before/
(byte-neutral); build_review.py refreshes review.json.  Edit source/ directly in later
rounds or carry spec entries forward by physical address.

Findings: debug name "HBMAX " -> DebugHud ($9C); standing-code format (EncodeStandingCode);
SessionEvent 1 = lure lost, 2 = landed, 3 = time up; message ids 1-7; bank 5 stays mapped
during fishing and holds fish/lure tables; strike thresholds; MainMode handlers.

Unresolved questions: bank-5 table formats/extents (L-006); natural reachability of
unknown fixed-bank helpers (L-001/L-002); sound/music identities; on-screen meaning of
retrieve actions.

Leads: produced L-006 (bank 5 fishing tables split), L-007 (standing-code generator /
debug name for late-game routes); updated L-001, L-002, L-004 (analysis/leads.json).
Deferred: final editorial audit; semantic splitting of generic data blocks.
Toolkit: no new toolkit issues found (analysis/toolkit-issues.md unchanged).
