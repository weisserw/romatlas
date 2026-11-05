# Final editorial acceptance

Final targeted actual-source review completed on 2026-11-05. This accepts current
source readability, not whole-ROM classification. Valid full-source review credit
from rounds 1–3 covers all 5453 physical scaffold regions, 814 compact units and
7250 embedded unit intersections. Round3's carry-equivalence and location-contract
proofs remain the starting evidence; no mandatory whole-source reread or cosmetic
rewrite was performed. The official audit supplies fresh fidelity/inventory checks,
not a judgment of prose quality.

## Actual-source checks and resolutions

- Audio control and branch continuations: read game.asm:184–432 against the RAM
  aliases, pitch interpolation, base-volume convergence and nibble decoder. Found
  ContinueVoiceEnvelope at game.asm:368 incorrectly said “pitch-slide state.” Its
  incoming increment updates AudioVoice0BaseVolume toward a target; corrected the
  comment and every corresponding compact/embedded review summary. Routine names,
  flags, shared arithmetic scratch and hold/restart descriptions remain useful.
- Field/constant consistency: checked immediate symbolic operands against their
  uses and definitions. Found game.asm:33746 used EventSelectorOffset as an AND
  mask after four ROLs of event type/flags. Added EventRotatedFlagsMask=$0007 and
  used it there; EventSelectorOffset remains the actual record-byte-seven offset.
  This is a mechanical mask name, with no invented game meaning. Event record
  dispatch/initialization (game.asm:33403–33445,33655–33810) retains offset, size,
  type-mask and signed-clear distinctions. Object template copy/application
  (game.asm:34165–34280) uses sixteen-byte scratch and parallel object arrays;
  source explains bank save/restore and fifteen applied motion bytes.
- Display record hotspot: read game.asm:3505–3592. Three ASLs after DEY establish
  8*(incoming Y-1) modulo256, seven copied bytes, fourteen eight-byte storage rows.
  Skipped-byte role, natural domain and arbitrary-Y safety remain explicitly
  unresolved. Accepted round3 annotation already agrees; no structural edit needed.
- Audio periods: read consumers at game.asm:550–598 and storage at1492 onward.
  AudioNotePeriodsCpu=$88aa matches physical67770; preceding envelope/pointer
  storage is not mislabeled as the note table. Doubled-note indexing and target
  period loads support existing prose without claiming the full selector domain.
- NMI and bank context: read game.asm:25698–25710,25844–25980 and36325–36360.
  The true handler is $d8fe; immutable vector words retain FE D8 byte order.
  Audio bank four is a 16 KiB mapper selection; physical execution views use
  8 KiB banks8/9, with fixed14/15. Existing mapper description distinguishes them.
  Former $fed8 decoding remains accepted duplicated Data with honest format limits.
- Generic Continue/address labels: checked the audio joins, NMI split/restore,
  event-state joins, object-template branches and metatile row/column loop
  (game.asm:4177–4205) against enclosing algorithm comments and named RAM/fields.
  Their parent routine names and nearby explanations supply control-flow purpose;
  did not rename sound branches solely to remove address suffixes. The two concrete
  defects above were corrected despite prior reviewed status.
- Raw/generic boundaries: checked the NMI-adjacent256-byte generic block, duplicated
  terrain-like tail, envelope payload boundary and all three isolated Unknown notes
  at game.asm:22135,26756,33399. Existing bounded consumer/pattern rationale is
  retained. Generic assets do not acquire speculative formats or labels. The exact
  three $60 bytes remain Unknown; coordinator terminal rejection review is reused.

## Acceptance evidence and limits

Only two source lines changed in game.asm; one constant definition was appended to
symbols.inc. All game.asm line numbers remain stable. Prior review was preserved at
analysis/final-editorial/review-before-final.json. The final validation artifact
checks exact frozen scaffold/phase1 identity, reviewed statuses, nonempty summaries,
all ranges, every compact/embedded source location relative to commentary/source
(`game.asm`, never `source/game.asm`), and units for every Code region.

Evidence: analysis/final-editorial/location-contract-validation.json and
validation-hashes.json; commentary/audit.json and verification.json record the fresh
byte-identical rebuild and reading-after refresh. A read-only archive.editorial_status
result is recorded separately; archive() was not called.

Some actor/audio/asset purposes, selector domains and generic record subdivisions
remain unknown, as documented locally and in uncertainties.md. No new research
frontier or structural conflict arose. Classification remains incomplete:
131085/131088 bytes, Unknown117608/122120/127720, despite rounded100%. No ROM,
map/state, frozen baseline, toolkit, archive, cleanup or publication edit is part of
this acceptance. Historical maps/baseline.json remains the original raw baseline;
the round3 equality reference is commentary/baseline.json per coordinator addendum.
