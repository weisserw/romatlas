# Round002 independent disassembly review

Input identities in input-identities.json; input pass-001-final. Read required policies,
current evidence ledger/gameplay and redirect primary-evidence.txt and checker source.
No new runtime experiments: existing broad progression scenarios remain applicable.
Fixed physical 8 KiB PRG slots [0,1,2,3]; mapper3 changes CHR independently.

R001-R01 verified: raw bytes44/6A, LDA8546,X -> TAY -> STA0000,Y establish
addresses, not cursor coordinates. Retained exact two-byte Data/Static extent and
confidence. Current label corrected to SetupChoiceDestinations; historical evidence
and already corrected readable source preserved.

R001-R02 verified: PlaySoundNote's low-nibble mask and CMP0C reject guard bound
12 word entries, subsequent ASL and paired reads establish word representation.
StartSound computes B2C1+4*A, reading slot/mode/low/high with high OR20. Supported
last selector15 in terrain bounce, first stream atB319, all22 consistent slot/mode
fields and stream origins jointly constrain exact descriptor88-byte span. This does
not prove natural selection of every ID.

InitializeSoundTrack consumes four header bytes and starts cursor2. DecodeSoundEvent
uses base+2*cursor; command paths increment cursor, consume arguments, redirect loops,
and test FF before reading its argument. Independently read these source consumers,
compared descriptor/period bytes and reviewed the checker transition abstraction.
Fresh bounded checker result sound-verification.json agrees with redirect result;
all22 endpoints match next header, except final exclusive endpointB655. First6
unconditional B0,02 loops have structural FF tails; no executed-FF claim for them.
Final stream ends at physical3664 inclusive, not3663; remaining generic Data begins
3665. Checker explores both counted-loop outcomes; it is static overapproximation,
not timing or natural-execution evidence. Commands and header bounds connect every
stream; period24+descriptors88+streams828=940 refined existing Data bytes.

Generic gaps3279..32D0 and3665..8009 retained after prior consumer/window review;
no stronger ramp/envelope or course schema inferred. No code changes or new fetch
claims. Code9478/Data23306/Image32768 counts unchanged;84->88 regions.

API initially rejected new region without required id; unsaved session terminated,
leaving files unchanged. An inadvertently issued pass-002-final checkpoint therefore
captures unchanged input (preserved). Successful API retry supplies id0; authoritative
output is pass-002-refined, map.validate and byte-identical rebuild passed. This is a
caller schema error, no toolkit issue. No toolkit edits. Full commentary tree and
frozen round001 review preserved byte-for-byte; changed map regions require next
commentary reconciliation by physical identity. Do not regenerate over commentary.
