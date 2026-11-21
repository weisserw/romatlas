# Reviewed uncertainty and redirect questions

The source explains established mechanics throughout; these are limits of meaning or
structural questions, not unread source. CPU addresses use C000-FFFF NROM view,
effective 8 KiB slot banks [0,1,0,1], physical offset = CPU address - BFF0.

The three unclassified intervals remain unchanged: [005D,005E), [3485,3488),
[3609,3614). Respectively: extra RTS after scrolling; possible F475 audio trampoline;
alternate F5F9..F603 audio priority path. See R01-001..003 in analysis/leads.json.
No legal-opcode-only classification or dead-code claim was made.

Audio interpretation now exposes a useful exact flag contract: ConsumeAudioRequest
leaves carry from ASL(request-1) alive through BIT(header). BIT sets N/V, not carry.
Negative requests pass the signed request gate; header sign selects music, carry
selects effect/control, and overflow selects a control variant. Review the residual
priority path with this contract before imposing an actual-caller prerequisite.
Natural validity and complete request-pointer extent for negative IDs are unresolved.

SelectIngredientSegmentOneThenFallThrough and its selector-two entry do not branch
to the shared tail: LDA #1/#2 makes the following BEQ false, so they fall through to
selector three. Source preserves this exact behavior. A bounded runtime question is
whether bounce phases intentionally use the same final row-Y adjustment; do not
"repair" the instructions from their apparent animation intent.

Concrete stale map/evidence semantics to reconcile in redirect/disassembly, without
changing physical boundaries or generic non-code acceptance:

- [1278,128E), CPU D268..D27D: five signed two-byte pepper offsets, then twelve
  remap bytes from D272. Prior evidence says eight deltas plus fourteen remaps.
  ReadPlayerMovementAndPepper indexes the remap with direction nibble; 12..15
  would touch following code. Opposing-input behavior is not a proved valid domain.
- [0B7B,0B7F): first InitialEnemyPacing byte seeds BCD extra-life increment and
  both thresholds in InitializeLivesPepperAndExtraLife. Source calls the payload
  InitialExtraLifeParameters; remaining three bytes' specific role is unresolved.
- [26B6,26BE): eight pepper stun durations, indexed by one-based stage capped at
  eight using CPU E6A5 as table-minus-one. EntityStatusValues is stale metadata.
- [0F74,10F4): DrawDescriptorGroup uses eight-byte stride for bonus descriptor
  families; seven-byte stride is selected separately by DrawPlayerModeIndicator.
  Prior blanket seven-byte record wording is too broad.

CollisionCoordinateRecords is likewise indexed one-based: CPU D11F is a synthetic
base two bytes before the stored pointer table at D121. Source symbols explicitly
say MinusTwo/MinusOne; those operand bases are not extra data classification.

Generic classified non-code retains honest notes where precise roles are unknown:
two movement-adjacent constants, duplicate ingredient selector runs, internal
StageLayoutRecords sub-table boundaries, individual audio sequence extents and
terminal zero/FF fill. Existing consumers/patterns/boundaries were inspected; no
unknown game meaning was promoted to an invented format. CHR remains the bounded
512-tile pattern asset, not semantic names for every picture.
