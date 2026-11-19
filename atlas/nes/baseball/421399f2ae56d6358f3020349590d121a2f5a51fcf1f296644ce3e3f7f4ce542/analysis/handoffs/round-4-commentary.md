# Round 4 -- commentary final acceptance (fallback byte) (2026-11-18)

Status: COMPLETE. Current-source readability complete; final editorial audit PASSED.
This does not establish evidence-backed whole-ROM classification (see "Classification").

## Scope
- maps/current.json == maps/pass-004-fallback.json == analysis/state.json (sha256
  fc5d69baba0dabbf68abf52a4456ef48b0b3e675c94c7ad6141b774daf3f75fa) differed from the round-3
  commentary baseline (pass-003, a60ef9e5...) only in region 796: physical 0x2604 (offset 9732,
  length 1), CPU $E5F4, PRG bank 1, kind Data, provenance Manual, no label -- a USER-DIRECTED
  FALLBACK, not an observed or statically proven data role.
- Preserved copy: the entire round-3 commentary directory moved unchanged to
  analysis/round-3/commentary/ (round-3 tools copied to analysis/round-3/tools-commentary-r3/;
  analysis/tools-commentary-r3/ also left in place). pre-l013/ and pre-l014/ untouched.
- Fresh baseline: `commentary.py prepare` (472 scaffold entries, byte-identical baseline).
  Scaffold delta: unknown-002604 replaced by region-796 (Data); every other entry identical.

## Carry-over method
- analysis/tools-commentary-r4/ = copy of r3 tools (inline-site path pointed at r4). apply.py
  renders commentary/source/game.asm from the frozen before/ + notes; review.py rebuilds units
  from source and carries review_round/review_note by identical physical offset/length/kind from
  analysis/round-3/commentary/review.json (asserts same id and reviewed status).
- Re-render check: before any edit, the only diff from the round-3 source was the $E5F4 region
  header, so all carried names/comments re-render and re-verify unchanged.

## Edits this round
- $E5F4: honest note -- generic Data of unknown meaning, typed Data only by explicit
  user-directed fallback for this batch (Manual provenance), not evidence-backed; no reference,
  pointer, fall-through or consumer (L-009); the $60 would decode as RTS; reachability unknown.
  Label Unknown_E5F4 -> neutral Data_E5F4. review entry region-796 summary says the same.
- Editorial fixes: BaseApproachOffsets/ThrowSideX labels at $DFCA/$DFCD with exact
  overlapping-window description (was "Y/X pairs per base") and symbolized operands (were
  Data_DFC4+n); format comments at the $E01D throw-choice tables (ThrowDirBase R/L/D/U ->
  1st/3rd/home/2nd) and HomeRunFlashColors; OutAtBase routine comment; review.json no longer
  records data tables ($CE1A, $DFC4, $E01D) as continuations of preceding code routines.
- Strict spot checks (no further defects): comment identifiers all resolve except deliberate
  "round 1 called it X" history notes; no live stale L-013/L-014 names; no camera-view swap
  wording; 0 auto labels; 0 raw address operands; no duplicated comment lines; the only
  Unknown/unclassified wording is the header note and the fallback note.

## Verification / audit
- `commentary.py audit`: ok true, byte_identical true, reviewed_regions 472, reviewed_units 620,
  editorial_audit true (commentary/audit.json). commentary/verification.json and
  commentary/reading-after.asm refreshed.
- commentary/source/game.asm sha256 62a7a7028fcaf150a318185eed9025dc4caa2782ca0632fefc688975de98ac1b.
- input/game.nes sha256 421399f2... unchanged. maps/*.json and analysis/state.json not modified.

## Classification (reported separately)
- 24592/24592 bytes carry a region kind: 24591 evidence-backed + 1 byte (0x2604/$E5F4) typed
  Data only by the user-directed fallback. Evidence-backed whole-ROM classification is NOT
  claimed complete; L-009 remains materially ambiguous.

## Remaining documented uncertainties (commentary/uncertainties.md)
$E5F4 code-vs-data (L-009); $DFC4-$DFC9 no reader; RunnerSideTable geometry described
mechanically; HitPower/HitAngle role swap; return-to-base path static-only; song-to-event map;
unreferenced fragments (AnimateCycleAndSleep, RestartFieldersReturn, FielderGoRole5,
VectorAngleCode, SubtractY32nd, SumOfSquaresXY); CameraRecC8 has no reader.
