# Commentary uncertainties — round 1

The fixed CPU PRG view is physical 8 KiB banks [0,1,2,3]. CNROM writes select CHR independently. CPU symbols in equates identify consumer addresses, including unclassified bytes, and do not add structural classification. Physical offsets include the 16-byte header. No source interpretation changes the frozen map.

The 64 exact unknown ranges (10,456 bytes) have local boundary notes. Mixed inner state handlers, dispatch tables, music streams and other gaps remain raw emissions. See analysis/disassembly/pass-001/unknown.json. A known pointer is not evidence for an unknown target or its full extent.

Names describe mechanical behavior. Traffic/secondary/four-actor pools have strides five/five/four; particular vehicle identities and original field names remain uncertain. Coordinate names follow their OAM consumers. Speed/time values have game-internal units. Road marker handling is established, while intended routes and naturally produced selectors remain partly untested. Artificial scenarios in the pass-001 handoff do not prove natural reachability.

Source-linked redirect questions:

- `DispatchAudioOpcode` / `AudioOpcodeHandlers` (CPU $BCC6/$BCD8; physical $3CD6/$3CE8) and audio tail physical $400B..$5100: establish command-specific stack/pointer contracts and bounded stream structure. `StopAudioStream` discards a parser return address; `PushAudioLoopFrame` writes three-byte software frames. Inspect handlers together with music/effect parsers rather than treating pointer values as code proof.
- `DispatchPlayerState`, `DispatchTerrainEffects`, `DispatchTrafficState`, `DispatchSecondaryActors` and corresponding sprite selectors: consumer mechanics are named; prove finite selector domains/entry contracts before adding unknown handlers/tables. Unclassified gaps remain explicit.
- `EndingFrameDispatch` ($8F5D; physical $0F6D) reads selector RAM $0500 and paired handlers at $90D3. `EndingApproach` advances that selector. Recover exact selector bounds and finish/return prerequisites; the rescue initialization scenario alone did not establish a complete ending.
- Non-code physical $5100..$800A remains generic where exact formats are uncertain. Existing road/tile/palette consumers support useful bounded inspection, but generic data classification does not require inventing original objects. Small unknown graphics/audio gaps merit separate boundary checks.

CHR tile layout is established from the NES format. Art identities are not assigned. Generic repeated runs are described as non-code, without asserting original fill intent. No full-game compatibility, complete classification or final editorial acceptance is claimed.
