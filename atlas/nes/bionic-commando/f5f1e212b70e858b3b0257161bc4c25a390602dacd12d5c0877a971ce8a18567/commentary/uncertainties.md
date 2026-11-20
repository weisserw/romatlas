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
  across mapper helpers. The E000 BRK/RTI cleanup path is now established under the Round011 contract; inline-selector dispatch remains unsupported.

All 33 remaining unknown bytes across 16 exact intervals retain honest local boundary/role notes.
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
  The renderer retains a four-byte ambiguous prefix; its tile/attribute fragment,
  saved-X footer and header publisher have separate supported contracts. The
  footer requires saved X atop the stack, which the inner balanced PHA/PLA does
  not supply. The BRK/RTI pair now connects the exclusion scan under the Round011 contract.
  Counter call/bit-operation fragments and the conditional BNE are preserved code, but the actual BNE
  targets an operand at physical $3D6B5 and the final load is truncated. There
  is no functioning counter helper or successful-return claim.
- The BC00 vector and bank-call-restore template are preserved code. Actual
  mapper $0B selects lower physical banks 22/23, and CPU $BC00 is collision
  Data at physical $2FC10. This rejects a functioning callee, not the preserved
  template role; no replacement bank or callee is inferred.
  Generic data shelves have exact consumer offsets/pattern notes where available;
  unresolved record subdivisions and game meanings are semantic uncertainty,
  not omitted readability work.

Round004 dependencies updated by Round011: CMP #$2B supplies the flags
restored through BRK/RTI into the initial E002 BEQ. Under the explicit
interrupt/stack contract the backward scan is coherent; natural entry is unresolved.
The horizontal header producer publishes a 34-byte queue and its consumer mutates
destination/count and clears readiness. Natural isolated/reset-alias calls remain unverified.


Round007 arithmetic correction: the BCC at physical $3E9D7 is accepted code.
It targets the JMP entry at CPU $E9E3, physical $3E9F3, not an operand.
Carry from the trial subtraction bypasses commit on borrow; no borrow falls
through to the inverse-store/carry-add fragment. Its high result remains in A,
with the final dangling STA Unknown. The target JMP is CPU $9103, but the lower
bank context is unresolved at this arithmetic entry. The source's named grapple
target is valid only under its separate banks 28/29 vector contract, not inferred
for the trial path. No natural arithmetic caller or working enclosing algorithm
is established. The one-byte arithmetic prefix also remains Unknown.

Residual intervals and candidate-specific limits are recorded in
analysis/evidence/round-007/unknown-ranges.json and review.md. These are reviewed
ambiguities, not omitted naming/comment work. Prior superseded commentary and
its former operand allegation remain historical in analysis/round-7/commentary-round-006.

Round008: physical $3D6A5..$3D6A7 is accepted conditional code. AND #$1F
sets Z from the incremented packed byte. Nonzero branches to CPU $D6A5 /
physical $3D6B5, still an accepted JMP operand. Zero falls through on exactly
the eight original values with low bits 31; DEC restores the original byte
(including $FF after full-byte wrap), and AND #$E0 clears its low counter.
This validates the conditional role without validating its target or a working
helper. The dangling B5 at $3D6AF remains Unknown. Current exact residuals:
analysis/evidence/round-008/unknown-ranges.json; independent review and finite
byte-domain check are alongside it. Prior contrary boundary wording remains
historical in analysis/round-8/commentary-round-007.

Round009: the former twelve-byte prefix at physical $3D67E is accepted
Code6/Data6 by exact $61E relocation to $3D060, including all three table
words and four following calls. Actual $E000 BRK/RTI enters object cleanup and is not the original
inline dispatcher, and the third relocated destination is overwritten by
TickAnimation. Structure does not establish a working dispatch or caller.
Original $AF producers/reset bound its selector to 0/1/2; this is not proof
of that ABI for the preserved copy. Residual Unknown evidence is now
analysis/evidence/round-009/unknown-ranges.json (33 bytes, 16 intervals).

Round010: physical $3D62B+3 is preserved CLC/ADC $B3, supported by the
88-byte exact clone and independent $61E table/call relocation anchors.
It adds the shared metatile tile offset to incoming A before paired tile stores.
The orphan $1D at $3D62A remains Unknown; no missing load repair, natural
entry, outer renderer or functioning dispatcher follows. Current residuals:
analysis/evidence/round-010/unknown-ranges.json, 30 bytes in 16 intervals.

Round011: $3E010 Code1 BRK and $3E011 Data1 discarded padding connect
the prior scan/filter under fixed banks30/31, intact stack, no NMI hijack or
intervening interrupt. I=1 excludes maskable IRQ. RTI at actual IRQ-vector
physical $3C0C2 restores flags/SP and leaves A/X/Y unchanged; three residual
stack writes, stacked B marker and timing effects remain. No natural entry,
original intent or hardware/game compatibility claim follows. The preserved
JSR $E000 can return from cleanup to CPU $D674, the first inline table byte;
return balance does not implement selector dispatch. No classification change
is requested from this semantic observation. Residuals: round-011 evidence,
28 Unknown bytes/15 intervals. Prior two-byte ABI rejection is superseded only
within this exact structural scope; the inline dispatcher remains unsupported.


Final editorial acceptance correction (round-011-classified): the opening
33-byte/16-interval summary and pending editorial statement above are historical
and superseded. Current residuals are 28 Unknown bytes in 15 intervals;
editorial review is accepted in editorial-audit.md and review.json, subject to the
fresh audit result. Round009/010 residual counts remain historical checkpoints.
Room101 is the pre-final Albatross interpretation, not a proven final cockpit
arena; its accepted kind70/health39/room96 mechanics remain unchanged. Artificial
terminal display was visited; natural final victory and full timed credits remain
unproved. No new runtime evidence or structural interpretation follows.
