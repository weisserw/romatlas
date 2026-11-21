# Round 3 — commentary handoff (COMPLETE; final editorial audit passed)

- Input checkpoint: pass-004 (maps/current.json == maps/pass-004.json == analysis/state.json, sha256 119004fa...d55690),
  100.00% classified (262160/262160, 0 unknown). The map changed since commentary-002 (pass-003), so the whole round-2
  commentary was preserved under analysis/round-2/commentary/ (plus analysis/round-2/commentary-tools/) and commentary/ was
  re-prepared with `scripts/commentary.py prepare` (fresh before/, baseline.json = pass-004, 1365-entry scaffold).
- Carry-forward: notes in analysis/commentary-tools/notes/*.txt (bank + CPU address keys) re-applied by
  `python3 analysis/commentary-tools/cmt.py review`. Diff of frozen before/ vs round 2: only b0 $9AB8 (Unknown -> Code) and
  25 bank-6 regions (Padding -> Data); every other record carried unchanged; only notes touching these were invalidated/rewritten.
- Output checkpoint: **commentary-003** = commentary/source/game.asm (sha256 3cdcb94751744f2824d7029b15f04016aa445bafb20af7ec5b90e4a0ec277131).

## Verification / audit
- `python3 /root/romatlas/scripts/commentary.py audit <workspace>` -> ok true, byte_identical true, reviewed_regions 1365,
  reviewed_units 1453, editorial_audit true (commentary/audit.json; copies analysis/commentary-003-audit.json and
  analysis/commentary-003-verification.json). Reading copy refreshed (commentary/reading-after.asm).
- Non-comment diff vs commentary-002 limited to the $9AB8 instruction and .org/.reloc wrappers of the 24 template runs.

## Scope / readability
- ALL classified source covered: banks 0-7 code, every Data/Text/Padding block, header, CHR. No unknown regions remain.
- review.json: 1365/1365 entries reviewed with summary + source; 1453 units (397 distinct, spanning units split per entry);
  every Code entry has units. editorial_audit status "reviewed" with concrete notes.

## Editorial findings fixed
- Changed regions: UpdateBoyStop_OrphanRts (b0 $9AB8) documented as structural-only Static code with explicit
  "no reachability is claimed; why the extra byte is there is not known"; template runs at +$3A/+$50 labelled
  "ObjAnimTimer/ObjAnimOn for slots 3-10 (all $FF)"; TypeBounces $A0B3-$A0BD note rewritten; file header now says pass-004,
  no unknowns; stale "Blocks marked Padding"/"phase 1 classified these as padding" text removed.
- Stale numeric aliases / contradictory names (28 prose fixes): 7 old artefacts where a numeric value had been replaced by an
  unrelated RAM name (ComputeBlobDrawY "$90 pixels", UpdateBlobEating "$68", UpdateBlobWalk "$18/$34", JackUpdate "$76" and
  "stream $18", HorizDistanceBoyBlob "$78 (far)", DrawMetasprite "$80 ends the list"); undefined name JackActive -> FormFlag74;
  raw RAM addresses in prose ($4A, $81, $93, $85, $9C/$9D, $77, $B4/$B5, $3C/$75/$33, $78, $79, $7A, $7C/$37/$73, $5D, $7D/$7E,
  $9F, $6E) replaced by symbols.
- Redundant comment: EnterRoomRight inline that duplicated its header shortened; "(bubble trap?)" reworded tentatively.
- Review summaries: terse first sentences (e.g. "NMI handler.") now include the following sentence.
- Checked clean: no Sub_/Data_/generic labels (cmt.py todo empty); numeric memory operands only $0500/$0600 page clears and the
  explained BIT-skip $02A0; no unsupported unused/dead claims.

## Residual uncertainties (commentary/uncertainties.md)
- Mechanical-only meanings: BlobEatState/Timer, FormFlag74, FormEffectSel, ObjHover, UnwrittenFlag92, TorchFlag, set-script
  scenes, edge markers, pharmacy offer tables, DebugTextBoyH consumer; why $9AB8's extra RTS exists.
- Music sequences and screen streams documented by format, not decoded per entry; RectTileData/frame-list byte ownership not split.
- '; -- observed/static from here' provenance markers kept deliberately (carry Observed/Static distinction).

## Leads
- Consumed: L-023, L-024 (documentation of their pass-004 outcomes). Produced: none. No open structural questions.

## Deferred work
- None required for commentary. Optional: per-entry decoding of music/screen streams; tightening some long older headers.
- ROM classification completion is a separate coordinator decision (not declared here).

## Toolkit
- No toolkit issues found (analysis/toolkit-issues.md unchanged; TK-001..TK-003 stand). Toolkit not modified.
