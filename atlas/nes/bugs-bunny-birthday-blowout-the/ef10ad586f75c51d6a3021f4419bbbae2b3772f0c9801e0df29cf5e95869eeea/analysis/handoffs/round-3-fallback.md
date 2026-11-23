# Round 3 manual-fallback handoff (user-authorized exception, 2026-11-22)

## Scope
- This phase covers only the 3 residual unknown bytes of lead L-015. It made no other
  map edits and did no redirect research.
- Authorization: the user granted a batch-only exception for <= 10 unclassified bytes.
  Those bytes may be marked as generic Data of UNKNOWN MEANING with Manual provenance.

## Checkpoint
- Input: pass-002 (map sha256 88ab22c6...).
- Output: pass-003. Files:
  - maps/pass-003.json, identical to maps/current.json and analysis/state.json.
  - analysis/pass-003-verification.json.
  - generated/pass-003.
- Map sha256: be088f7905b9e184cdcf32b34a57ec061e39af752fece9848856eb378eb7bc3f.

## Verification
- map.validate returned ok.
- analysis.unknown returned total 0, both after the save and after a fresh map.load.
- `run.py checkpoint pass-003` returned ok=true and byte_identical=true.
- input/game.nes is unchanged.

## Exact ranges, region ids and provenance
| Region | File offset | Length | CPU (PRG14 fixed) | Kind | Provenance | Confidence | Label |
|---|---|---|---|---|---|---|---|
| 1908 | 115791 (0x1C44F) | 2 | $C43F-$C440 | Data | Manual | 0.1 | none |
| 1909 | 116119 (0x1C597) | 1 | $C587 | Data | Manual | 0.1 | none |

- The complement was re-queried before editing. It matched the round-2 listing exactly.
- Each region's evidence field records that it is a user-directed fallback and is not an
  observed or statically proven data role. Both regions point back to L-015 and to the
  round-2-redirect rejection review.

## Counts
- 262160 / 262160 bytes classified (100.00%); 0 bytes unknown.
- Evidence-backed (Static/Observed): 262157 B.
- Manual fallback: 3 B.
- By kind: Code 41294, Data 89794 (3 of them Manual), Image 131072.

## What the commentary phase must reconcile
- The generated source now emits these bytes as data directives in regions 1908 and
  1909. Previously they were raw unknown-gap export.
  - Rebuild or refresh the commentary baseline from pass-003.
  - Carry existing notes by physical range.
  - Re-run commentary verify and confirm byte_identical.
- Comments at $C43F-$C440 and $C587 must say "unknown meaning, user-directed manual
  fallback (L-015), not evidence-backed".
  - Do not give them semantic names.
  - Do not call them dead code, padding or tables.
  - Either reading is still possible: an unreferenced alternate entry/leftover code
    (ORA ($00,X) at $C43F; RTS at $C587) or leftover data.
- Any summary must report 100% as "262157 B evidence-backed + 3 B Manual fallback", not
  as full evidence-backed classification.
- L-015 is now resolved-by-manual-fallback. If a new consumer or caller turns up, replace
  regions 1908 and 1909 with an evidence-backed classification.

## Toolkit
- TK-007: protocol.md's flat, lowercase map.annotate example does not match the NES
  server, which needs the region form with id 0 and Capitalized enums. This is a
  documentation mismatch and does not block anything.
- A stale `run.py serve` process (PID 1823535, started 2026-11-21, store
  analysis/state.json) from an earlier phase was still running. It was not started by
  this phase and was left alone. Its in-memory map is pass-002 or older, so it must not
  be told to map.save.
