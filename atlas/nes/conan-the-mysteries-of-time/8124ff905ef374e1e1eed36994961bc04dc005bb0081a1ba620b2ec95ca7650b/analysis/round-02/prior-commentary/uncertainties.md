# Round 1 unresolved meanings and redirect questions

Current classified-source readability is complete. These limits concern game
meaning or unclassified backing, not skipped source review.

- Bank-12/13 encounter fragments have coherent distance/facing/animation/cell-update
  explanations, but enemy/puzzle identities and omitted middle bodies remain unknown.
  Names describe mechanics rather than asserting a boss identity.
- Stage graphics roots, compressed stream endpoints, map-layout extents, metatile
  pointer domains, damage/duration/period tables and most action images are still
  unclassified. Their named reference addresses do not establish backing structure.
  Follow LoadStageState/LoadStageMapRoots/ReloadStageGraphics and consumers
  DecodeRunStream/LookupMutableMetatileTile/AnimateCurrentObject.
- SpawnStageRecordObjects provides a particularly concrete bounded lead:
  seven-byte records from CPU $BB8B, cursor starts 0, advances 7 and stops when
  >=$0120. This executes 42 record iterations, with final start $011F and exclusive
  endpoint $BCB1, exactly the classified consumer entry. All parsed stage bytes
  lie in 0..7. Physical interval is $1BB9B..$1BCC1 in handler banks 12/13.
  See ../analysis/round-01/commentary-spawn-record-observation.json. Redirect should
  verify record semantics, independent end boundary and competing code evidence,
  then propose data classification and handler contracts together. No structural
  classification was changed in commentary.
- Scheduler flags, pointer continuations and inline-word extraction are explained,
  but remaining indirect handler domains require selector/producer investigation.
  Observed execution and natural reachability remain separate claims.
- ObjectHealth is reused as an effect-owner field in CreateEffectObject. Scratch
  Work/HandlerWork names are deliberately mechanical because routines reuse bytes.
  Score glyph carry is understood, but numeric scoring units are not established.
- NMI distinction resolved during review: $CAB7 balances the gate and RTIs, $CABA
  jumps through the callback, $CABD is the default frame body. Nested NMIs skip work.
  The map callback is an ordinary NMI callback, not a separate nested-NMI handler.
- Generic uniform runs retain generic-data status. Their fill/resource purpose is
  unknown; prior boundary/reference checks suffice for that conservative role.
