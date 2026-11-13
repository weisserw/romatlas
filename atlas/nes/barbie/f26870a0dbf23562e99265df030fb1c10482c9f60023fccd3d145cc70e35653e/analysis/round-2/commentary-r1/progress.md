# Round 1 commentary — active

Frozen baseline: pass-001-final / maps/current.json and analysis/state.json, unchanged.
Initial preparation rebuilt byte-identically; no prior commentary existed.

Read all 12,143 baseline source lines across every PRG bank plus header/CHR.
Durable semantic inventory: analysis/commentary-r1/units.txt (376 routine/structure
entries), instruction identity index: analysis/commentary-r1/instructions.json.
Source application is next: routine comments, bank-aware call/branch symbols,
consistent RAM/hardware fields and explicit unknown-range boundaries. No bank is
claimed fully reviewed yet; all 966 scaffold entries remain pending until applied
source and coverage inspection. Classification is unchanged at 71.05%.

Exact remaining scope: apply/reconcile all units against classified instruction
starts, inspect orphan regions/control targets, describe every known data range and
unknown gap, fresh byte-identical verify, refresh reading copy, compact handoff.
No naming/comment backlog may be handed to redirect. Final editorial audit reserved
for coordinator. Structural gaps remain genuine unknown bytes.

## Applied source and verification checkpoint

Full source now contains 376 routine/control-structure explanations, 1,382 bank-aware
code labels (1,006 continuation labels refined to operation/field purpose), 485
RAM/hardware symbols and 169 bank-local ROM consumer aliases. Aliases inside unknown
ranges explicitly establish neither classification nor table extent. Button masks
are symbolic where controller meaning is established. Every raw gap has an honest
unknown boundary; all non-code payloads retain structural/evidence notes.

No orphan classified code region or code region over 500 bytes beyond a semantic
unit was found. Review reconciliation and final uncertainty/coverage records remain
pending; currently checking arithmetic/field interpretations and unit boundaries.
Corrected one metatile-stride interpretation to eleven bytes after counting the
three additions following index*8. Fresh source verify after symbol application:
byte_identical=true, source SHA256 2236eecf9ebb3c9ee3663a2361e608e4de48b0d8a87f4cc36398b41f6bf6a64f.
The stride prose correction followed that build; final verification still required.

Exact remaining scope: finish concrete correctness checks, reconcile full scaffold
review with named units, record unresolved meanings/redirect questions, final fresh
verify and refreshed reading copy, then handoff. No structural edits or toolkit edits.

## Round 1 current-source readability complete

Reconciled every physical scaffold interval against the source. All 966 entries
are reviewed, including honest boundary notes for unknown gaps. Classified code
coverage is exactly 18,465 bytes / 8,438 instruction starts, partitioned into 837
reviewed fragments belonging to 376 meaningful routines/structures. No orphan
classified instructions, pending review entry or naming/comment backlog remains.
The 36,606 data bytes, 110 text bytes and 131,072 currently Image-marked CHR bytes
have source notes; generic data preserves prior pattern/boundary evidence without
inventing schemas. See analysis/commentary-r1/coverage.json and review.json.

Final fresh verification passed: byte_identical=true; source SHA256
3bbab1d7b02c64a748a3db3c801698ff385609d0513e1767270da8dddf8cfc4f.
Reading copy refreshed with the supported read command. Frozen maps/current.json,
analysis/state.json and ROM backing remain byte-for-byte unchanged. ROM SHA256 is
f26870a0dbf23562e99265df030fb1c10482c9f60023fccd3d145cc70e35653e.

Unresolved meanings and actionable redirect questions are in uncertainties.md.
In particular, CHR backing does not prove a graphics role for every byte: resource
payloads read through PPU and executed in RAM warrant a role/provenance review by
redirect. This was flagged without changing the inherited map classification.
Other questions concern stage/boss/ending transitions, eleven-byte world records,
handler-specific object state and remaining packed drawing consumers.

Readability is complete for this current source; ROM classification is incomplete
at 71.05% / 75,907 unknown bytes and also carries the CHR role caveat. Final editorial
acceptance/audit remains pending for the coordinator and was not run. No new toolkit
issues found in commentary; persistent NES-001 remains one suspected input-schema
issue, no blocker, with buttons-array workaround. No toolkit modification, archive,
publication or ROM change occurred. All source/assets/evidence remain local.
