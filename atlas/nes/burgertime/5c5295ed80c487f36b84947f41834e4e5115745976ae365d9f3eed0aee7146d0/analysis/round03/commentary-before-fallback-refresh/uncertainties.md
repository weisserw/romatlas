# Reviewed uncertainty and redirect questions

The source explains established mechanics throughout; these are limits of meaning or
structural questions, not unread source. CPU addresses use C000-FFFF NROM view,
effective 8 KiB slot banks [0,1,0,1], physical offset = CPU address - BFF0.

The two unclassified intervals remain [005D,005E) and [3485,3488): extra RTS
following scrolling and possible F475 audio trampoline (R01-001/002). Their
localized competing interpretations remain unresolved. The former eleven-byte
unknown [3609,3614) is now CheckMusicReplacementPriority, accepted as Static code
in pass-002-reviewed from its independently supported caller/slot/header contract.
It uses the active music ID to test the incoming header's replacement mask, loads
the selected effect on permission and otherwise returns. Natural entry remains
unresolved; this is not observed execution or proof of dead code.

Audio interpretation now exposes a useful exact flag contract: ConsumeAudioRequest
leaves carry from ASL(request-1) alive through BIT(header). BIT sets N/V, not carry.
Negative requests pass the signed request gate; header sign selects music, carry
selects effect/control, and overflow selects a control variant. The accepted alternate
priority entry follows the supported contract described above.
Natural validity and complete request-pointer extent for negative IDs are unresolved.

SelectIngredientSegmentOneThenFallThrough and its selector-two entry do not branch
to the shared tail: LDA #1/#2 makes the following BEQ false, so they fall through to
selector three. Source preserves this exact behavior. Whether bounce phases intentionally use the same final row-Y adjustment remains
unresolved. The established fallthrough needs no further experiment for editorial
acceptance; do not "repair" the instructions from apparent animation intent.

Round2 disassembly reconciled these four source-backed metadata corrections
without changing physical boundaries or generic non-code acceptance:

- [1278,128E), CPU D268..D27D: five signed two-byte pepper offsets, then twelve
  remap bytes from D272. The map now records five pairs plus twelve remaps.
  ReadPlayerMovementAndPepper indexes the remap with direction nibble; 12..15
  would touch following code. Opposing-input behavior is not a proved valid domain.
- [0B7B,0B7F): first InitialExtraLifeParameters byte seeds BCD extra-life increment and
  both thresholds in InitializeLivesPepperAndExtraLife. Source calls the payload
  InitialExtraLifeParameters; remaining three bytes' specific role is unresolved.
- [26B6,26BE): eight pepper stun durations, indexed by one-based stage capped at
  eight using CPU E6A5 as table-minus-one. The map now calls this PepperStunDurationValues.
- [0F74,10F4): DrawDescriptorGroup uses eight-byte stride for bonus descriptor
  families; seven-byte stride is selected separately by DrawPlayerModeIndicator.
  The map now distinguishes the two strides.

CollisionCoordinateRecords is likewise indexed one-based: CPU D11F is a synthetic
base two bytes before the stored pointer table at D121. Source symbols explicitly
say MinusTwo/MinusOne; those operand bases are not extra data classification.

Generic classified non-code retains honest notes where precise roles are unknown:
two movement-adjacent constants, duplicate ingredient selector runs, internal
StageLayoutRecords sub-table boundaries, individual audio sequence extents and
terminal zero/FF fill. Existing consumers/patterns/boundaries were inspected; no
unknown game meaning was promoted to an invented format. CHR remains the bounded
512-tile pattern asset, not semantic names for every picture.
