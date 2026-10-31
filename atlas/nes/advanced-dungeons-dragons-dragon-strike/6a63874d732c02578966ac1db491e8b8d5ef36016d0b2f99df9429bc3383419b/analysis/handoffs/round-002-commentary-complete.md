# ROUND002 current-source commentary complete

The SAME interrupted commentary phase is now complete for the current-source naming/comment scope. No classified-source readability backlog remains. This is not final editorial/archive acceptance or whole-ROM classification completion.

## Verified identity and inventory

- ROM: input/game.nes,524304 bytes, SHA256 6a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b, unchanged.
- maps/current.json and analysis/state.json remain byte-identical to incoming identity8c90238b3ebc1e65c9ea29eb0c92fe1d942bc7fc1e7e8a798945cc58fa5902ac. No prepare/regeneration or structural map edits occurred.
- Final maintained source SHA2560b99d165d71dbdccebe869a990a3f1ad7c8fb49ba7bfe912bff1b10ca7e6bbf8. Fresh commentary.py verify passed byte-identically using the configured release binary; commentary.py read refreshed reading-after.asm at this source.
- All23,753 Code scaffold records/48,874 bytes are reviewed. All1,876 Data records/213,280 bytes, the one Image record/262,144 bytes and both Unknown records/six bytes remain reviewed. All2,461 discovered meaningful units are reviewed and source-linked. Pending records/bytes/ranges are zero in analysis/round-002/commentary-frontier.json.
-28,093 region/unit source references resolve under commentary/source; references use game.asm:N, not the invalid source/game.asm:N prefix.

## Scope assessment and examples

Valid earlier review credit, names, source prose, physical equality/dependency proofs and the entire frozen prior tree were preserved. Continuation04 directly decoded/reviewed the remaining8,375 Code bytes in banks26/27/28/30 and reconciled concrete older dependency claims. Its source routines, meaningful branches, fields and constants explain purpose, inputs/state, algorithms, bounds and bank contracts. Unknown original game identities retain mechanical names and ordinary uncertainty. Classified data had already received bounded consumer/format review; affected palette, coordinate, scroll and menu tables now have more specific connected explanations.

Bank26 now explains scene/CHR/view/reference initialization, thirteen-byte actor slices and explicit table coordinates, palette overrides, animated overview, slot/class/mode menus, encoded text, message pages, title input gates and public CPU veneers. Source distinguishes CHR register values from physical PRG context and records capacity/positive-count assumptions.

Bank27 now explains the full password editor: selected/shared rows, repeats and independent directional maps, saturated six-letter cursor, empty-password acceptance, failure reset, render cursors, packed-slot copies and validation. The codec stores21 field bits plus a three-bit XOR check and tries mode candidates0..2; failed letter conversion can partially modify packed bytes. Source preserves the caller contract between player Y and InputSelection-selected slot.

Bank28 was inspected independently, including important differences: one heading row, ASCII-like/punctuation text mapping without bank26's repeat token, hex bias$40 instead of$89, and password footer row24/offset13. Equal CPU addresses were not used as proof of equal bodies or data.

Fixed30 now explains palette category/depletion updates, object and sampled actor rendering, shared actor states/timers/class policies, signed contact rectangles, health depletion and compaction, six-bit fractional movement, twelve-direction steering, marker effects/lifetimes and source relocation/clamps. Source preserves marker flag subtraction's full-byte underflow to$FF and the outer scan's delayed handling of shifted records.

Concrete stale unknown-gap claims were reconciled in connected stage-directory initializers, special-actor dispatches, recorded-input abort, menus/titles and fixed helpers. Existing physically proved shared stage continuations retain their proof files and bank-specific runtime resource values. Updated reviewed summaries match affected source notes. No cosmetic whole-source rewrite or discarded review credit was needed.

## Investigated limitations

Six structural Unknown PRG bytes remain at fixed30 CPU$C000 and fixed31 CPU$E529; rounded100.00% classification is not whole-ROM completion. Their explanatory source notes remain honest, with no invented trampoline/data decision.

Exact graphic/song/item identities, some mixed resource subarray boundaries, audio/game coupling and certain bank-dependent API gameplay payloads remain tentative. Fixed actor APIs$8012/$8018/$801B require the actual mapped PRG pair; CPU aliases are not physical identities. Source preserves literal carry/overflow, flag underflow, partial writes and unchecked valid-data assumptions rather than repairing them. commentary/uncertainties.md records the useful remaining questions. No natural cheat activation, full hardware/game compatibility, portable export or public asset claim follows.

## Deferred terminal acceptance contract

Per coordinator instruction, no editorial audit or own-unit projection was run. commentary/review.json editorial_audit stays pending. After the next redirect/coordinator rejection review, separately assigned terminal acceptance must give EVERY Code scaffold entry its own real reviewed meaningful-unit linkage, project existing valid unit spans without invented opcode-level reviews, and add genuine veneer/tail units where necessary before final audit.

Preserve analysis/coordinator/unit-linkage-contract.json and its historical19 records/47-byte uncovered requirement. Read-only current counting after these new real routine units finds16 reviewed Code records/44 bytes outside all current unit spans:20..29 and16404..16413(audio API veneers),33471..33474,49855..49858,66239..66242,82623..82626,99007..99010,115391..115394,131775..131778,148159..148162(stage tails),220285..220286 and237078..237079(title tails).23,042 Code entries currently lack their own units even though most lie in meaningful spans. These are terminal metadata/acceptance tasks, not pending source-review credit. Do not fabricate per-opcode units, rerun one-shot scripts, archive, publish or delete investigation records.

The one-shot continuation scripts are historical edit records and include subsequently corrected intermediate symbol choices; do NOT rerun them over maintained source. The reusable commentary-checkpoint.py remains safe for location/count refresh and now reports the zero-backlog/current-source-complete status separately from editorial acceptance. Final passing verification supersedes intermediate failed builds during symbol reconciliation. No toolkit edits, ROM/map edits, competing writer, audit, archive or cleanup occurred.
