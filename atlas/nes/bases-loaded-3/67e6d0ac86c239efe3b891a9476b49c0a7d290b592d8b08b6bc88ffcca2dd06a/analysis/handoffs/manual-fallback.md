# User-directed manual fallback for 2 residual bytes (2026-11-18)

This is a **user-directed fallback**. It is not an observed or statically proven data role.

## Authorization
The user explicitly authorized a batch-only exception: if at most 10 physical ROM bytes remain
unclassified, mark them as generic Data with unknown meaning, using Manual provenance. This
overrides the workspace rules that require more structural proof, for these bytes only. The
exception does not extend to anything else.

## Verified complement before the edit
- The store was loaded from analysis/state.json (sha256 b810b6fa…8c0d = maps/pass-003.json).
- `analysis.unknown` returned total 2: physical 230960 (len 1) and 235667 (len 1).
- This matches the user-supplied list exactly. 2 is within the limit of 10.

## Ranges and annotations
| Physical | File hex | Bank:CPU | Region id | Kind | Provenance | Confidence |
|---|---|---|---|---|---|---|
| 230960 | 0x38630 | 28:$8620 | 1252 | Data | Manual | 0.5 |
| 235667 | 0x39893 | 28:$9883 | 1253 | Data | Manual | 0.5 |

- No label was set and no semantics are claimed. The comment and evidence fields record the
  fallback, the unknown meaning and the prior ambiguity.
- Data regions do not accept execution views, so the bank context (PRG bank 28 in the $8000
  slot) is recorded in the evidence text instead.
- ROM bytes were not modified. The state diff against pass-003 is exactly 2 added regions
  (next_id 1252 → 1254). No existing region changed.

## Prior ambiguity (preserved)
Each byte is a lone $60 (RTS) that directly follows another RTS. No static edge, reader, pointer,
stack-dispatch target or observed fetch reaches either byte. Lead L-015 was rejected as an
evidence claim, both in analysis/handoffs/round-002-redirect.md and in round-002-redirect-b.md
(evidence: analysis/redirect/r002-a/, r002-b/). That rejection review was later re-checked by the
coordinator. Whether these bytes are code or data is still **unresolved**. The Data kind here is
an administrative fallback, not a finding. L-015 keeps status "rejected", and a
`user_directed_fallback` record was added to it in analysis/leads.json.

## Verification
- map.validate ok. After the edit, analysis.unknown total is 0. analysis/state.json is saved and
  maps/current.json is exported.
- `python3 run.py checkpoint pass-004-fallback` gave ok and byte_identical true
  (analysis/pass-004-fallback-verification.json, map sha256 c4dd4ae8…3dec,
  generated/pass-004-fallback).

## Classification reporting
- Evidence-backed classification: 393230 / 393232 bytes (all but the 2 bytes above).
- Manual user-directed fallback: 2 bytes (Data, meaning unknown).
- Total marked: 393232 / 393232. Report this as 393230 proven plus 2 manual fallback, not as
  fully evidence-backed.
