# Round 2 independent disassembly review (active)

Input checkpoint pass-001-final. Final instruction/contract/data artifacts reviewed directly; diagnostic dispatch-discovery.json is excluded. Structural role and reachability remain separate. The overlap audit is a consistency check only.

## Reviewed primary contracts

CA5C loads scene-dependent word tables, then count and four OAM fields per count; CAE2..CAE7 restores three saved registers. Count-zero is unsafe because the loop decrements after its first body. D74E initializes 8x8 grid loops; each index selects D84F/D94F words and D7AF..D7E2 copies exactly4x4 bytes. D81D..D833 copies exactly64 attributes. These independently support consumed graphic record formats, with original selector domains separate.

C375 one-based request decrements then doubles its index; record header supplies low PPU address and terminal cursor, then bytes2..limit-1 are copied. Specific palette record lengths can be tested, while the entire surrounding pointer domain need not have natural selection proof. C69A separately reads scene palette bases at D05E; these are not palette records.

E2A0/E2A4/E2A8/E2AC write pointer high40 and bases00/08/0C/04; E2BE stores Y0..3 only. A35F builds each destination from three RAM0461/046B/0475 words; offsets1/4/7 plus three stores stay in ten-byte masks. AEF7 derives its two RAM pointers from mode*2 and copies ten bytes; these instructions support finite RAM contracts, not arbitrary indirect-store safety. C278 destinations are HUD buffer contracts; any unsupported external invocation is separate from these static paths.

## Isolated routine alternatives

80C9 bank0 establishes opening timing/scroll fields and joins8147, whose bounded timer/frame progression returns. A4DB bank2 gates active actor/state flags, loads paired four-entry offsets, assembles sprite parameters and callsCA5C; A98D uses existing movement stream and sharedA95B/A946 continuations. AF68 bank0 sets one per-object flag then joins the clear-state RTS helper. B147 bank4 coherently tests round63 and score270F, sets0626=FE/0624=1, increments0484 and returns; no ending/natural-caller claim. E576 sets preset28/Y04 then tail-calls accepted APU initializer. E6D6 tests audio duration, silences4000/0558 and updates scheduler flag. E99A adds a16-bit increment with carry into0510/0511. These use independently established field/helper contracts plus coherent boundaries/control, beyond legal decoding alone.

Correction: E816/E837 manipulate4008/400A/400B triangle registers and0572/0576; redirect description “DPCM-control paths” is over-specific and wrong. They are triangle-channel initialization/update paths. Preserve code proposal, correct semantic wording. Incoming carry affects ADC choices; no constant output claimed.

The seven isolated00/60 bytes have no independently supported intended entry or bounded consumed object; retain Unknown. No arbitrary register invocation or unobserved claim can resolve role.

## Specific-role correction: scene5 default grid

D765..D77A always replaces scene5 with (round adjustment &7)+11, so the redirect probe’s selected set range(9)|range(11,19) included unselected grid5. Its assumed bank0 block interpretations overlap119 bytes of the independently supported metasprite pointer table at physical378E..3816. Recomputing the supported selected set with5 excluded leaves28 exclusively inferred16-byte “tile blocks” at3645..3805. Their bytes are repeated signed low-valued movement/frame patterns (3645..378E) or already supported pointer bytes (378E onward). Retain generic data for the former and the independent pointer role for the latter; reject a consumer-proven4x4 tile-block role for this set. The unused64-byte grid itself remains patterned data in a dense adjacent grid container, with no natural-selector claim. No unknown-byte acceptance gain depends on this over-specific tile interpretation.

## Narrow data annotations

Use generic data for residual repeated parameter/threshold/frame/coordinate containers where finite selector extent is unresolved, unselected ROM pointer suffixes, the gallery4 combined phase-pattern container, audio duration/envelope/token containers beyond proven consumed records, and the DPCM inter-sample15-byte interval. Preserve their visible bounded non-code constraints and consumer/reference inspection. Do not claim precise original object grouping, generator, padding purpose or complete natural domains. Count-prefixed sprites, count-limited palette/VRAM records, FF-bounded parallel animation prefixes, actual selected4x4 blocks/64-byte attributes,36-byte reels and explicit113/257-byte DPCM samples retain their supported specific roles.

Fresh primary-decodes.json is immutable inspection only; some bounded windows deliberately extend into adjoining data. Printed offsets are physical, and decode.cpu/context gives CPU views. Those extra decoded bytes are not code proposals or acceptance evidence. EEF2 is an audio table base, not an instruction consumer entry: actual readers are EDFD/EE18/EE1E, and EDE1 reads EF88 duration data.
