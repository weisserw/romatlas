# Round 002 investigated limits

Current-source readability is complete. These are explicit limits of static meaning,
structural classification and reachability, not omitted commentary batches. Prior
round-one uncertainties are preserved with the entire prior commentary at
analysis/round-2/commentary-round-001; affected current claims are corrected here.

- Cinematic mechanics: accepted state graphs now explain CE/CF sequencing,
  countdowns, fades, overlays and the terminal static display. Saved artificial
  progression reaches the 2010.8.2/Joe screen. Natural victory and complete timed
  credits remain unverified. The last display frame is drawn without ticking its
  $FF duration; no valid redirect through trailing $FFFF is invented.
- Encounter mechanics: room101 kind70 has health $27 (39) and defeat requires the
  subtraction result to become negative. Earlier cockpit identity assumptions are
  not evidence. Damage, linked objects, palette/countdown phases and room writes
  are explained, but natural final-shot selection remains unresolved.
- Resource schemas: bounded consumer/block/flank evidence supports room cells,
  five/seven-byte spawn lists, metasprites, audio, dialogue/display and collision
  resources. Generic gaps retain unknown subdivisions, selector domains and game
  identities. No original serializer, song name, glyph grammar or exact numeric
  curve generator is invented. Schema uncertainty does not erase accepted Data.
- Actor and field meaning: routines use established movement, collision, countdown,
  allocation, rendering and state roles. Exact actor/weapon/story names remain
  tentative. Repurposed fields are explained locally: grapple arc coordinates,
  linked slot indices, position backups, player-specific motion presets and panel
  address scratch differ from ordinary animation/velocity/palette meanings.
- Round003 resolves the former four numeric control targets: the $D7C6 vector
  clears the upper PPU pattern half, $9207 compensates grapple/player displacement,
  and $BF36/$AB97 enter the supported scene state machines. The prior questions
  remain historical at analysis/round-2/commentary-redirect-questions.md. Current
  source has no unresolved numeric control operands.
- Fixed/banked ABI: physical identities and effective 8 KiB slots remain distinct
  from MMC1 values. Explicit selections and supported actor/audio/player entry
  contracts resolve named calls; decoder-default lower banks were not propagated
  across mapper helpers. The E000 trap/selector ambiguity is unchanged.

All 105 remaining unknown bytes across 20 exact intervals retain honest local boundary/role notes.
No new natural execution coverage, whole-ROM completion or full hardware/game
compatibility follows. Editorial/archive acceptance and audit remain pending.


Round003 mechanical limits and corrected interpretations:

- The independent division helper performs sixteen finite shift/subtract rounds;
  its one-byte trial remainder loses rotate overflow. Divisors 1..128 provide the
  ordinary unsigned quotient/remainder contract; unrestricted larger divisors and
  divisor zero need separate interpretation. No natural caller is invented.
- The object-type search literally scans Y=11..21, beyond the ordinary allocation
  group. Source documents the exact active/type reads, match/exhaustion return and
  scratch destruction rather than silently narrowing the loop to gameplay slots.
- Scene/resource and actor narrative names remain uncertain. Accepted inline table
  lengths and stack-consuming dispatch mechanics are firm; they do not prove every
  state is naturally selected. Auxiliary setup temporarily chooses mapper03 and
  reaches physical EC13; room restore and remembered bank restore are distinct.
- Grapple sprite arrays have four leading bytes then nine-entry offset/tile/attribute
  shelves. AnimationPointerHi/AnimationTimer store reference pixels in grapple
  states; ObjectFlashTimer stores the OAM start cursor during segment emission.
- Round004 resolves the seven reset prefixes only under their explicit upper-alias
  MMC1 entry contract. Copied vector words do not demonstrate natural startup.
  Round005 narrows the former renderer prefix to four leading and seven trailing
  opaque bytes around a supported 48-byte tile/attribute fragment; the separate
  21-byte header publisher does not resolve these outer boundaries. The trap pair remains opaque,
  although the following 21-byte exclusion/deactivation suffix has a local contract.
  Counter and arithmetic fragments retain their competing interpretations.
- The apparent BC00 wrapper/vector remains ambiguous because actual mapper0B
  resolves BC00 to the consumer-backed collision dictionary at physical2FC10.
  Generic data shelves have exact consumer offsets/pattern notes where available;
  unresolved record subdivisions and game meanings are semantic uncertainty,
  not omitted readability work.

Round004 dependencies: the initial BEQ at physical $3E012 consumes an incoming
equality flag whose producer remains unresolved across $E000/$E001. Later type
comparisons and deactivation joins are independently meaningful; the encompassing
scan is not claimed functional. The horizontal header producer publishes a 34-byte
queue and the accepted consumer mutates its destination/count and clears readiness.
Natural calls to the isolated publisher and reset aliases remain unverified.


Round005: The former 59-byte renderer and 64-byte arithmetic opaque spans now contain independently supported 48-byte tile/attribute and 17-byte double-shift operations. Their four outer unknown boundaries retain competing alignment, unmatched saved X, dangling byte and branch/vector conflicts respectively. This supersedes any older whole-span unknown descriptions; no live caller or enclosing working routine is claimed. See analysis/evidence/round-005/review.md.

Round006 supersedes the former 46-byte arithmetic-tail uncertainty: 21-byte trial subtraction and 22-byte inverse-store/carry-add operations have independent static register/flags contracts. Only the intervening two-byte BCC and final dangling STA remain unknown, alongside the one-byte prefix before the double shift. Q's high carry-add result is returned in A, not stored. These intact operations do not establish a working enclosing algorithm, natural caller or connection across the corrupt branch; see analysis/evidence/round-006/review.md.

Round006 concrete boundary discrepancy for redirect: the carried source labels physical $3E9F3 as ObjectBankVectors_Entry, while structural evidence calls it a vector operand. The BCC remains Unknown; resolve the effective CPU/physical target identity before any new control-flow claim.
