# Round 001 investigated limits

The entire current classified source has naming/commentary. Remaining uncertainty is
localized meaning, natural selection and some internal data schemas, not skipped ranges.
Fixed CPU PRG slots are physical banks [0,1,2,3]; ROM offsets include the 16-byte header.

- **Final-base behavior (D001-L05 resolved artificially).** Five terrain records at
  $FFE0 (directory entry $FDBE) create four type-$0D parts and one type-$0E part;
  $FF terminates the stream. The scan ceiling is six, not its initial part count.
  The dying $0E generates the type-$0F core at $06C0, the sixth encounter unit.
  Accepted artificial core-damage execution cleared EncounterRemaining/BaseActive.
  Ordinary post-base scrolling looped from the boundary after $011A to $0002;
  source $5F set LaterLoopMarker, extended progress through $0124 and looped to
  $0102. Both increased LoopDifficulty by four. The prior failed shortcut is
  preserved as a missing-spawn-prerequisite result, superseded only in that scope.
  These are supported artificial routes; natural player completion and an ending/
  credits screen remain unobserved. Not every extended area was traversed.
  Primary evidence and bounded recipes: [redirect handoff](../analysis/handoff-redirect-001.md),
  [records](../analysis/redirect/r001-final-base/spawn-records.json),
  [rejection review](../analysis/redirect/r001-final-base/rejection-review.md).
- **Standalone helpers.** `SetPpuAddress2234` ($80D2) has an exact hardware contract,
  but no known caller. Recording helpers ($82B9, $924D/$9265) attempt CPU $CBxx
  writes: CNROM maps that space to ROM/mapper hardware, not writable RAM. The
  shipped hooks at $82B8/$924C return immediately. These coherent static routines
  do not prove a usable recorder or natural reachability. `ExpireArmorTimer`
  ($9578) is the synthetic return of armor dispatch; no independent natural caller
  is asserted.
- **Actor identities.** Mechanical actor names describe approach, steering, script,
  terrain gun, segmented boss or damage behavior. Exact artwork/enemy identity and
  natural selection of every dispatcher entry remain unresolved. Types, field
  values and known stack/carry contracts are retained without inventing identity.
- **Shared scratch and auxiliary fields.** WorkValue1–5 and shared pointer cells
  intentionally have local meanings. Fields after ActorDirection are overloaded
  across actor types; ActorAimOrGroupPhase and auxiliary names do not assert a
  universal object class layout. Some routines intentionally read NMI-saved cells.
- **Broad encoded PRG blocks.** Prior bounded consumer/pattern reviews support
  non-code. Consumer-established starts now have local format notes, but source
  include splits and review-unit lengths do not certify original table endpoints,
  exhaustive schemas, original source organization or generation algorithms.
  The direction lookup is quantized; its exact original quantizer is unresolved.
- **Stack/carry contracts.** Helpers that discard one or two return addresses and
  subtraction helpers consuming incoming carry are documented locally. Comments
  explain the mechanical path; they do not infer that an arbitrary external caller
  could invoke these helpers safely.

No new structural conflict was found requiring reclassification during commentary.
The unknown physical complement remains empty. Deeper table semantics alone do not
invalidate the evidence-backed generic Data fallback under evidence-policy.md.
