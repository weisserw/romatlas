# Commentary uncertainties — current pass-002-reviewed source

Current-source readability is complete for 27306 code bytes and 231913 classified non-code bytes. The 2941-byte unknown complement remains structurally unresolved and has explicit notes at each retained binary include. Final editorial acceptance and whole-ROM classification remain separate gates. The full prior uncertainties and their historical evidence remain in analysis/round-2/commentary-round1/uncertainties.md.

Both sound copies now expose the pitch/envelope continuations and stop/volume/envelope commands. Their physical identities remain separate. The original copy's natural selection is unresolved; known parser, seven-track, arithmetic and APU mechanics do not identify every tune or sound effect.

Tournament standings, the twelve-letter continuation encoder/editor/decoder, result text scripts and terminal championship/final animations now have local mechanical explanations. Password fields, checksum, finite bounds, substitution rows and repeat-input behavior are described without claiming the original author's naming. The accepted artificial terminal scenarios are evidence for those contexts, not a proof of natural tournament completion or a separate credits sequence. Prior round-one questions about whether these handlers exist are superseded by the current classification and source; exact artwork/text identities remain uncertain.

Restart, set-play, special moving-target and scripted cutaway services now describe their actor reservations, coordinate setup, input masks, state progression and bank-local record contracts. Script records stride sixteen bytes; their field meanings are explained at ScriptActorLoadRecord, including projection mirroring and indirect continuations. Actor slots 12..16 are scene actors outside the normal 0..11 play set. Natural free-throw/dunk initialization and every visual identity of script/action selectors remain open. These uncertainties do not omit the established state mutations or control flow.

Roster/substitution menus explain nearest directional candidate selection, active/unavailable appearance flags, reserve groups, energy comparisons, profile/name/attribute rendering and display restoration. Exact game names of shared scratch values, auxiliary rule counters and several action flags remain uncertain; symbols name their mechanical uses. The same RAM byte can serve another bank-local caller contract.

Remaining structural questions are localized in analysis/round2-disassembly/residual.json and the disassembly handoff: stack-sensitive fixed/actor/sprite fragments (especially physical $1EE5A), bank-8 $10139..$10183 and bank-10 $14049..$1434A continuations, isolated returns/placeholders, and late-copy pieces with unresolved ROM/branch dependencies. Do not infer ROM upload origins from RAM execution, invent entry preconditions, or extend code through those notes. Bank wrappers explain MMC1 register values separately from effective physical 8 KiB bank indices.

Generic classified data retains the accepted per-range pattern/consumer/boundary evidence and any established names. Prior structural inspection and current consumers were reused; no unsupported table schema or semantic consumer was invented. CHR format is established, while exhaustive screen/animation ownership is not. No new runtime experiment or structural rejection was needed in this readability pass. No toolkit issue was found.


## Round 3 dependency update

Upper-window menu/password copies require effective banks [10,11,14,15]; absolute B references stay in the original bank. Natural selection of copied entries is unresolved. Static correspondence and artificial decoder tests support mechanics only. Direct PPU rendering requires safe display timing; queued rendering is a separate interface. BRK signatures remain non-code.

The current 44-byte complement has ten reviewed boundaries:

- Physical 0x2410: Unresolved JMP $0000 stub. Interface alignment alone does not establish an executable entry or RAM destination.
- Physical 0xa410: Unresolved JMP $0000 stub in the second copy; no supported RAM execution origin is established.
- Physical 0x659d: Isolated RTS after a terminal result path. No established incoming edge distinguishes executable return from stored byte.
- Physical 0xe59d: Isolated RTS in the result copy; no established incoming edge distinguishes code from stored data.
- Physical 0x1c253: Isolated RTS after the dispatcher terminal path; entry remains unresolved.
- Physical 0x1d357: Unreferenced JMP $ED2C after indirect dispatch. The established movement table does not select this stub; that absence does not prove data.
- Physical 0x1df41: Single $1C byte: an unofficial-NOP entry would overlap the accepted following instruction. Padding/data versus an alternate entry remains unresolved.
- Physical 0x1ee13: Unreferenced JMP $EEAC after a terminal jump. The adjacent RTS now has an independent incoming branch, but this stub does not.
- Physical 0x1f7c9: Dangling operand from the source-copy branch crossing the matched suffix boundary. Copy origin does not determine this byte role.
- Physical 0x1f7de: Copied 27-byte routine whose relative branch reaches a differing prefix. Neither the lower alias nor upper context establishes the complete routine contract. Do not assume actor_flags != 2 merely to avoid the bad branch.

New data consumers and mechanical formats are documented at their local includes. Exact artwork identities, some AI/event selector names and the natural role of historical copies remain uncertain. These are meaning limits, with no classified-source readability ranges deferred. Final editorial audit remains pending.
