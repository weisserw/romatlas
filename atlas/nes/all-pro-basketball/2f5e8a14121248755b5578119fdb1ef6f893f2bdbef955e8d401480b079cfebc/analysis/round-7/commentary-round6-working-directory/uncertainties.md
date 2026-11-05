# Commentary uncertainties — current pass-006-handoff source

Current-source readability covers 30100 Code bytes and 232042 classified non-code bytes. The remaining 18 bytes in nine intervals remain structurally unresolved with explicit source boundary notes. Final editorial acceptance and whole-ROM classification are separate gates. Historical uncertainty records remain preserved under analysis/round-4/commentary-round3 and earlier rounds.

Both sound copies now expose the pitch/envelope continuations and stop/volume/envelope commands. Their physical identities remain separate. The original copy's natural selection is unresolved; known parser, seven-track, arithmetic and APU mechanics do not identify every tune or sound effect.

Tournament standings, the twelve-letter continuation encoder/editor/decoder, result text scripts and terminal championship/final animations now have local mechanical explanations. Password fields, checksum, finite bounds, substitution rows and repeat-input behavior are described without claiming the original author's naming. The accepted artificial terminal scenarios are evidence for those contexts, not a proof of natural tournament completion or a separate credits sequence. Prior round-one questions about whether these handlers exist are superseded by the current classification and source; exact artwork/text identities remain uncertain.

Restart, set-play, special moving-target and scripted cutaway services now describe their actor reservations, coordinate setup, input masks, state progression and bank-local record contracts. Script records stride sixteen bytes; their field meanings are explained at ScriptActorLoadRecord, including projection mirroring and indirect continuations. Actor slots 12..16 are scene actors outside the normal 0..11 play set. Natural free-throw/dunk initialization and every visual identity of script/action selectors remain open. These uncertainties do not omit the established state mutations or control flow.

Roster/substitution menus explain nearest directional candidate selection, active/unavailable appearance flags, reserve groups, energy comparisons, profile/name/attribute rendering and display restoration. Exact game names of shared scratch values, auxiliary rule counters and several action flags remain uncertain; symbols name their mechanical uses. The same RAM byte can serve another bank-local caller contract.

Earlier stack-sensitive and bank-8/bank-10 continuation questions were resolved by round 3 structural contracts; the exact current structural complement is listed below. Bank wrappers distinguish MMC1 register values from effective physical 8 KiB bank indices.

Generic classified data retains the accepted per-range pattern/consumer/boundary evidence and any established names. Prior structural inspection and current consumers were reused; no unsupported table schema or semantic consumer was invented. CHR format is established, while exhaustive screen/animation ownership is not. No new runtime experiment or structural rejection was needed in this readability pass. No toolkit issue was found.


## Current dependency and boundary assessment

Upper-window menu/password copies require effective banks [10,11,14,15]; absolute B references stay in the original bank. Natural selection of copied entries is unresolved. Static correspondence and artificial decoder tests support mechanics only. Direct PPU rendering requires safe display timing; queued rendering is a separate interface. BRK signatures remain non-code.

The current 18-byte complement has nine reviewed boundaries:

- Physical 0x2410: Unresolved JMP $0000 stub. Interface alignment alone does not establish an executable entry or RAM destination.
- Physical 0xa410: Unresolved JMP $0000 stub in the second copy; no supported RAM execution origin is established.
- Physical 0x659d: Isolated RTS after a terminal result path. No established incoming edge distinguishes executable return from stored byte.
- Physical 0xe59d: Isolated RTS in the result copy; no established incoming edge distinguishes code from stored data.
- Physical 0x1d357: Unreferenced JMP $ED2C after indirect dispatch. The established movement table does not select this stub; that absence does not prove data.
- Physical 0x1df41: Single $1C byte: an unofficial-NOP entry would overlap the accepted following instruction. Padding/data versus an alternate entry remains unresolved.
- Physical 0x1ee13: Unreferenced JMP $EEAC after a terminal jump. The adjacent RTS now has an independent incoming branch, but this stub does not.
- Physical 0x1f7c9: Unknown $FA outside the eleven fixed-count indicator records. It matches a source-copy branch operand, but that origin does not classify the byte.
- Physical 0x1f7e3: Unresolved BEQ encoding targets physical $1F7C4 inside indicator fields, rather than an established code entry. The preceding five-byte prompt-state comparison is Static Code; this branch is excluded. Natural use remains unresolved; no prompt-state precondition is assumed.

New data consumers and mechanical formats are documented at their local includes. Exact artwork identities, some AI/event selector names and the natural role of historical copies remain uncertain. These are meaning limits, with no classified-source readability ranges deferred. Final editorial audit remains pending.

Round 4 boundary correction: Eleven four-byte tile-indicator records, ending in $6C. The F765 consumer advances X by four and stops at $2C, so it consumes exactly these 44 bytes. The following $FA is outside the table and remains unknown. Exact tile/indicator meanings remain unresolved. Source-copy operand origin is not a consumer proof for $FA. Historical phase-1 evidence text and prior records are preserved unchanged.

Round 5: Return immediately for game-mode table index 13. The nineteen-word table selects this handler through its interior word at physical $1C247; the ASL/indexed-load/indirect-JMP dispatcher preserves the ordinary main-loop JSR return frame. Effective fixed bank 14, CPU $C243. Static structure only; natural mode-13 selection and observed execution remain unresolved. The other isolated RTS bytes at $659D/$E59D retain their independent unresolved-entry notes.

Round 6: Compare prompt state in actor_flags ($0190) with two. The paired prompt writes independently set zero with text $C2/$BF and two with text $C1/$C0; later text/route consumers use this field. These constraints support this finite comparison fragment in effective banks [10,11,14,15]. It is not a complete callable guard: the following BEQ targets indicator fields and stays unknown. No caller, observed execution, natural use or actor_flags != 2 precondition is established.
