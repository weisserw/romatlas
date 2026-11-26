# Round 3 commentary handoff (final acceptance)

- Status: **COMPLETE** - current-source readability complete, final editorial audit passed.
- Input checkpoint: pass-003 (maps/pass-003.json = maps/current.json). ROM classification
  unchanged by this phase: 100.00% (131088 / 131088; 0 unknown).
- Preserved round-2 copy: **[analysis/round-2/commentary/](../round-2/commentary/)** (entire
  directory moved intact; only the regenerable tools/__pycache__ was dropped). Round 1 remains
  at analysis/round-1/commentary/. Fresh baseline prepared from pass-003 (before/ byte_identical
  true, 772 scaffold entries).

## Scope

- Carried: all round-2 names/headers/locals/notes/RAM symbols re-applied by physical offset
  (commentary/tools/annotate.py + notes.txt); carried source diffed against round 2 - only the
  five L-015 regions differ; review credit carried via tools/review_update.py.
- Invalidated/replaced: the round-2 "still unknown" notes for the six L-015 bytes, now stub
  labels with low-confidence/unknown-reachability comments: UnrefReturnCarrySet (6:$9B8B,
  sec; rts), UnrefReturnStub_A1E4 (6:$A1E4), UnrefReturnStub_C916, UnrefReturnStub_CA5B,
  local @UnrefReturnStub ($C405 inside State08_StageExit).
- Editorial fixes: Data_B6_9005 -> UnusedFillRomTarget (ROM write target in credit data,
  explained); $07F0 -> UnusedFillEntry; scratch $4C/$4D/$4F -> Temp4C/Temp4D/Temp4F;
  AddDamageTo0514/0584 -> AddDamageToAux0514/AddDamageToObjKind; $00-$17 scratch convention
  documented; stale source-header sentence rewritten.

## Readability assessment

All classified source meets the standard: 0 address-default routine/data names, 0 structural
locals, every routine/data/padding region has a header or role/boundary note, symbolic RAM/field/
table operands (remaining raw absolute operands $01AB, $01B1, $07FF, $0100,x, jsr $838A are
commented; $00-$17 scratch numeric by documented convention), tentative prose for inferred roles.

## Verification and audit

- Audit: `python3 /root/romatlas/scripts/commentary.py audit <workspace>` -> ok true,
  byte_identical true, reviewed_regions 772, reviewed_units 1156, editorial_audit true
  ([commentary/audit.json](../../commentary/audit.json)); fresh verification written to
  [commentary/verification.json](../../commentary/verification.json); game.asm sha256
  a9fcd4da933024085615cb6e7cee1955e2ca05979acf7cb0914dfc9218356880.
- review.json: 772/772 entries and 1156/1156 units reviewed; pending 0; editorial_audit reviewed.
- Reading copy refreshed: [commentary/reading-after.asm](../../commentary/reading-after.asm).

## Remaining editorial issues

None known.

## Unresolved meanings (described mechanically in source; see commentary/uncertainties.md)

Reachability of the L-015 stubs, L-003 fragments and unreferenced routines (UnusedFill9005,
UnusedQueueRows, $CF61, $FF06); tentative boss identities for stages 6/9/12/15 and regular
bank-5 enemy identities; Mummy state 5 / Dracula state $0C setters; multi-use ObjAux fields;
partly decoded collision codes; bank-6 $B800 palette-like record with no reader; NmiLagFrame
jsr $838A effective target.

## Deliverable readiness

Commentary deliverable is ready for archive acceptance (classification 100%, audit passed).
No toolkit changes; no new toolkit issues. Nothing published or archived by this phase.
