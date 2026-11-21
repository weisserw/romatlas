# Investigated limitations

Round002 current-source readability is complete; final editorial acceptance remains pending. Scope and fresh verification are recorded in analysis/round-002-commentary-handoff.md; these limitations do not imply whole-ROM classification.

The twelve remaining Unknown bytes comprise ten isolated $60 bytes without independent entry/return contracts and one $C9,$F9 pair whose immediate-comparison versus stored-word boundary is unresolved. Their source notes preserve the bytes and uncertainty. Structural classification belongs to the next redirect/disassembly cycle.

The accepted player/object dispatch tables, all selected-bank wrapper templates and audio command/effect tables supply finite structural contracts. Natural selection of every player state, actor type, audio entry and stage remains unresolved. Artificial progression observations establish only the actual fetched contextual starts recorded in their evidence, not an unchanged natural route.

Stage layout consumers support five-byte metatiles and fifteen/sixteen-byte layout records; R001-R07 corroborates stage pointer endpoints and index domains within those records. The complete layout index-list extents and exact original asset organization remain unresolved. Generic parameter/script/graphic containers reuse accepted bounded reference/record inspection and retain their narrower source notes.

Player state IDs, overlay effects, material interactions and many sound identities retain mechanical interpretations. CPU RAM aliases marked unresolved preserve storage identity without inventing a game role. Shared scratch and reused arrays may require context-specific names; the alternate strided PPU queue now has its own aliases.

Audio note/gate timing, counted loops, register shadows and envelope/pitch scripts are mechanically established. Song identities and original stream organization remain uncertain. DMC-related shadow fields do not establish complete DMC playback compatibility.

No full-game reachability, whole-ROM completion, hardware compatibility or final editorial acceptance is claimed.

Round002 resumed player/actor review: the state$2E position snapshot in physical14137..14482 saves player X-high into its fourth scratch byte, while state$2F later restores that byte as Y-high (physical14486..14665). This is the exact byte-backed behavior, not corrected source. A bounded grab/escape trace with nonzero coordinate high bytes could clarify whether normal game coordinates mask it. Material3 contact, special-terrain mode and the alternate-scene selectors have explicit mechanical descriptions; exact terrain/story names remain tentative. Vector table row extents and natural selector reach are not inferred from the integrator alone.

Type-$48 reflected spawn selector overlaps code: physical73989..74130 / CPU$A0F5..$A181 selects Y=0/4 then reads$A186,Y. Nonreflected bytes$00,$FF yield one selector0 child; reflected reads$20,$94 from the dispatcher instruction at physical74138, yielding one selector$20 child before the negative operand stops iteration. The nearby declared selector bytes$10,$FF at$A188 are not selected by this opening index. Exact mechanics are preserved; intended overlap versus latent defect is unresolved. A bounded reflected/nonreflected spawn capture could clarify natural variant use; no reclassification is implied.

Type-$4A contact at physical24680 has an encoded JSR$8C2D into physical19517 under the static[2,3,30,31] view, inside preserved vector-table Data18647..20597. Source describes the direct call and localizes the target/context uncertainty. A bounded natural-contact/effective-bank check is in commentary-questions.md; no classification was changed. Shared spawn-history removal also retains its exact same-column-versus-row comparison and branch into append scanning; intended deletion behavior is uncertain. Effect timelines retain first-match duplicate comparisons and only advance noise phase when no earlier pulse2 event branches away.
