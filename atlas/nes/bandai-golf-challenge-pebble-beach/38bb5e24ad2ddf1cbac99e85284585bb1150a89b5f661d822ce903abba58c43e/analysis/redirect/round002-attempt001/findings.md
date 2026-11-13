# Round002 redirect connected parameter review

Input pass-002-refined, map ff10526069bb622c0e919dc445090ae1f0a08f25b67aefde0bde2018d6a3a356;
88 regions/287 units current readability accepted. Fixed PRG slots [0,1,2,3].
All 65552 bytes classified, exact unknown complement empty. No map/state/source edits.

## Revised inference and prior barrier

Prior round001 findings deferred finer sound parameter arrays because pointer/index
paths did not prove full domains. The decisive current source is copied to
primary-sound-consumers.txt; prior accepted stream endpoint evidence/checker is in
../round001-attempt001/. This retry narrows the claim to the current sound bank,
combining finite bounded stream selectors, masked consumer widths, overlapping
biased bases, adjacent independently supported endpoints and amplitude consistency.
It does not assert the engine cannot accept arbitrary patched selectors. Stop if
selectors leave the proposed domains, command paths violate accepted stream bounds,
or the supposed envelope arrays conflict with amplitude representation.

## R002-R01 supported static proposal

Physical $3279..$32D0 inclusive (12921,length88), CPU B269..B2C0;
source game.asm5825..5840,6160..6179,6226..6253,6308..6324,6338..6342;
symbols.inc302..305. All are already Data/Static. Refine into:

- $3279..$327C / CPU B269..B26C: four channel enable masks [1,2,4,8].
- $327D..$3280 / CPU B26D..B270: four channel capability bytes [00,01,82,43].
- $3281..$32B0 / CPU B271..B2A0: three sixteen-phase vibrato banks.
- $32B1..$32D0 / CPU B2A1..B2C0: two sixteen-phase envelope amplitude banks,
  IDs1/2 relative to biased EnvelopeAmplitudes=B291. ID0 disables envelopes;
  B291..B2A0 is the third vibrato bank, not a separately stored envelope-zero bank.

TickSoundEngine AND03 establishes hardware channel0..3, stored and used by
GuardSoundChannel and flag reads. These four-entry extents need no natural selector
observation. Four mask values independently correspond to APU ownership bits.
CheckVibratoCommand stores argument&70; ApplySoundVibrato combines it with frame&0F.
The accepted22 bounded streams contain A3 arguments whose selected offsets are
{00,10,20}; all three banks have coherent signed periodic offsets. ParseSoundEnvelope
selects low nibble; accepted streams contain IDs1/2. ID0 initialization bypasses
ComputeEnvelopeVolume. The phase loop starts Y=3 and runs through0: four ROLs of an
initially zero work byte yield nibble0..15; envelope ID shifted4 OR phase gives the
sixteen-byte bank. Bank1 descends15..0, bank2 amplitudes all0..15. Last bank ends
exactly before independently accepted descriptor B2C1. Thus representations, selector
constraints and endpoints jointly discriminate arrays from arbitrary generic bytes.

Current ComputeEnvelopeVolume comment says "eight-step"; change to sixteen-phase.
The implementation is unchanged; this corrects the phase-count description. No code change, hardware claim or new fetch claim proposed.
Static command-path enumeration overapproximates counted-loop outcomes; selector
presence establishes encoded support, not natural selection. No claim that all engine
possible selectors0..7 or envelope IDs0..15 have dedicated arrays. This localized
schema does not require those unrestricted domains.

Run python3 analysis/redirect/round002-attempt001/verify_parameters.py; saved result
parameter-verification.json. Script fences paths B319..B654, max1024 states/stream,
reuses accepted command mechanics and validates selectors/representations. Independently
review source, masks, flag consumers, phase nibble and exact ends before annotation.
No artificial state or runtime experiment needed for these structural constraints.

## Other actionable avenues assessed

Round001 independent assessment remains sound for render-mask natural caller,
terrain domains, third sprite meaning, physical units/generators, division contract
and RAM45/81. Its decisive evidence is current matching consumer code and the
round001 data-inspection/dispatch reviews, not an execution-absence veto. No changed
entry, boundary or new selector writer contradicts those conclusions. Their unresolved
intent does not undermine structural classification or mechanical readability.
Remaining course/UI generic data has consumer/window/boundary evidence recorded in
analysis/disassembly/round001/data-inspection.md; no newly connected missing extent
or contradictory entry emerged. Original generator reconstruction is a stronger
claim than bounded data classification and is not required. The newly proven biased
sound base is confined to this parameter block; do not generalize it to other tables.

Progression checklist reviewed: late9/final18, flight/putting and terminal scorecard
return are visited with durable artificial-route evidence. No absent later milestone
with an obvious untried shortcut; collision/multiplayer permutations and failed manual
swing timing have no changed concrete question. No runtime replay warranted.

This is a productive combined refinement handoff, not a terminal incomplete finding.
Whole-ROM classification remains complete; final editorial audit/verification still
pending. One ready lead includes all connected mask/vibrato/envelope findings rather
than splitting them across cycles. No further identified promising actionable avenue
remains under current evidence after this connected review.
