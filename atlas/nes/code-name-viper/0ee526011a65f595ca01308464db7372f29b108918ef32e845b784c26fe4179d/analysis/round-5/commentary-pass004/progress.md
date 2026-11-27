# Round04 commentary — complete current-source readability

Current map/state SHA256:
db5ce160d67a8705fb7da671627035cdd170680a7af6bccb90756afcdb8c010e.
Full previous commentary preserved before preparation at
analysis/round-4/commentary-pass003. Fresh baseline verified;1353 classified
physical ranges carried only where bounds, kind and phase1 dependencies matched.
Useful names/prose/review retained, affected dependency statements corrected.

All75 new Code runs (7431 bytes) and111 Data slices (13139 bytes) now have
mechanical-purpose prose and meaningful names.29 internal routines/services are
additionally named and explained.715 distinct unit names and1560 scaffold entries
cover the entire current source;1539 classified regions contain32786 Code bytes,
97666 Data bytes,588 Text bytes and131072 Image bytes. All21 Unknown intervals
(48 bytes) retain explicit competing-role/boundary notes. No readability range is
pending. Current-source inventory: analysis/commentary-round04/inventory-check.json.

The source explains actor timers, signed motion, allocation carry, facing/animation,
script command strides/sentinels, terrain checkpoint keys and item consumption,
coordinate-to-PPU conversion, finite nametable writes, decimal formatting, controller
reset chord and balanced counted audio continuation.499 additional bank-resolved
control operands use source labels. Contextual actor fields, timers, checkpoint keys,
RAM destination and sequence constants retain exact addressing widths.111 data
slice notes replace compact structural evidence with readable format/consumer prose;
uncertain schemas and gameplay meanings remain tentative.

Corrected prior selector metadata at physical $1fe78 / CPU $fe68 to actual $04,
continuing the eleven-byte sequence before FE69 A0 00. The ten stage-indexed
selectors do not establish stage-ten use of the suffix. Historical records remain;
analysis/investigation.md appends the correction. C061/FA5B explicitly restores
banks two/three; F5E1 writes one RAM byte, not a fill. These are narrow contracts.

Fresh final verification: ok=true, byte_identical=true. ROM SHA256 unchanged:
0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d.
Source hashes:
game.asm cd7ccb7bab7efede44cb5350efb7607bebcfa8d0ba2edaa64bb22fed927444aa
constants.inc 0a5947bb4a5875f0212069a86f35cb62ce8ecfdaa6100e7c09deb67691e848fd
reading-after.asm refreshed after final verification. Final editorial audit remains
pending for final acceptance and was not invoked. Classification remains99.98%,
not complete. Precise structural questions: commentary/uncertainties.md and
analysis/pass-004/unknown.json. Overall goal remains active; next phase is redirect.
