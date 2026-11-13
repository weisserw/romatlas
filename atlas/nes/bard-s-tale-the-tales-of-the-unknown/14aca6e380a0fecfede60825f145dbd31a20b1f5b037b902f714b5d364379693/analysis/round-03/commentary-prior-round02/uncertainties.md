# Round02 commentary uncertainties

Current-source review covers all 1,325 classified regions and 164 unknown gaps. The earlier complete uncertainty record is preserved in ../analysis/round-02/commentary-prior-round01/uncertainties.md. Its historical byte counts and unresolved-boundary claims describe the previous map; useful consumer distinctions and cautious game-meaning notes remain applicable.

Round02 now bounds sixteen area event lists, forty dispatch words, sparse map masks/literals, generated PPU handler templates, seven four-row credit pages and their terminal title. The old event-consumer distinction remains valid: map patch pairs, movement restrictions and three-byte area events are separate formats. Known templates remain Data, even when copied into executable cartridge RAM. The audio 840-byte work copy and graphics cursor overreads do not establish serialized object extents.

Unresolved game meanings include character condition bits, resource/stat fields, some menu services, item IDs, encounter effects, audio tracks/instruments and mixed encounter/audio/fixed-bank resource containers. Source comments state exact mechanical behavior and ordinary tentative purpose. Existing record copies, wrapped menu navigation, action arithmetic and bank restoration are named without inventing original game terminology.

Concrete structural follow-up remains R02-RESIDUAL-DISPATCH-CONTEXT in ../analysis/leads.json and the disassembly handoff: trace writers/transformations of RAM $0032/$003A and audio continuation $06F6, prove omitted lower-bank return contracts, and inspect the bank-12 $B463 summary stream table at $B8A6. Descriptor indirect transfers require effective physical lower-bank context; mapper selectors are not physical 8 KiB bank indices. Supported wrapper return paths do not prove natural reachability.

All 52,321 unknown bytes retain exact physical boundaries and explanatory source notes. No classification was inferred from legal decoding or raw rebuilding. Final editorial/archive acceptance remains pending; no hardware or whole-game compatibility claim is made.
