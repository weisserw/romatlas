# Remaining meanings and structural questions

Current-source readability is complete; these are bounded semantic/structural
questions, not unreviewed source. Physical intervals and classification remain in
maps/pass-001.json and analysis/round001-accounting.json. Fixed PRG context [0,1,2,3].

- Entity type/outcome/directional dispatch: CPU $9FB1..$A086, $A8B4..$A900,
  tables $B0E7/$A903/$A9E6. Source explains pointer dispatch, working-record and
  OAM contracts. Most concrete per-type handlers remain structurally unknown.
  Redirect should follow existing R001-entity-dispatch, preserving caller/context
  evidence; do not infer code solely from the pointer words.
- Inventory attribute lookup: classified physical $C46..$C4C (CPU $8C36..$8C3C)
  has six bytes, but $8BF6 indexes using three-bit ownership values. Values 6/7
  would read following classified code bytes. Determine whether inventory state
  excludes those combinations or the representation intentionally aliases them.
  No table extension was made. CPU $9CA7 also encodes AND zero-page $FC, not an
  immediate mask; its initialization/contract remains unresolved.
- Shared work records have separate primary (32-byte) and secondary (24-byte)
  layouts. Consumer-derived aliases explain established fields; type-specific
  state fields, item names, outcome meanings and stage-feature variant remain
  unresolved. Names are recovered interpretations, not original symbols.
- Audio tables/streams and stage assets have consumer-level format explanations;
  complete internal subtable bounds, original generation and unused-byte claims
  remain unestablished. Generic non-code classification is retained where that
  is the supported role. CHR is named by bank/tile encoding, not artwork identity.
- Static code does not establish natural execution of every branch. The OAM
  invariant trap at $9FD6 and ending idle loop at $BCE2 retain their actual encoded
  mechanics. Artificial progression observations remain distinct from natural play.

No new toolkit finding arose during commentary. Existing NES-001 remains in
analysis/toolkit-issues.md, with its buttons-array workaround and no blocker.
