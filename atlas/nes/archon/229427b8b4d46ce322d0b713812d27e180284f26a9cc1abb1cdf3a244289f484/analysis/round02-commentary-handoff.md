# Round02 commentary handoff

Current-source readability is complete for maps/pass-002-final.json / analysis/state.json. Classification remains99.99% (131078/131088 bytes):23122 Code /107956 Data /10 Unknown. This is not whole-ROM completion or final editorial acceptance.

Entire prior commentary preserved at analysis/round-02/commentary-before before fresh preparation. Its game.asm hash remains1c1a9ecb022eac6839a608c3b801e79d9ee27c503f9f0f0d7733621c51c3efe1. Migration reused5144 exact physical range/kind blocks and sound names/prose/review, then reviewed all5803 new Code blocks and184 changed/new Data blocks, including old routine gaps/tails. Source explains meaningful control flow, RAM fields, finite dispatch domains, inline payload formats and generic Data limits.

All11138 scaffold records reviewed:10919 instruction regions,212 Data regions and7 Unknown intervals.615 meaningful physical units cover396 code routines/blocks,212 Data structures/envelopes and7 unresolved-byte units; current source locations/context refreshed. No classified readability ranges pending. See commentary/progress.md, commentary/review.json and analysis/round-02/current-unit-index.json.

Examples: seven special actions plus cancellation and their selection stack contracts; computer destination scoring, bounded route alternatives and effect dispatch; credits row reveals and menu transitions; audio command/loop handlers and demonstration sprite/tile scripts; six-byte attribute packing and prepared-destination stack helper;$0300 buffered actor prefixes versus$0800 direct uploads. Prior overclaims on input predicates,960-byte clear,sprite sequence widths,APU/script boundaries and marker subdivisions were corrected. Mechanical names and tentative prose retain uncertain game meanings.

Two additional wording corrections require later summary/map consistency review (no structural gain):

- CPU$DCDA CPY7 /$DCDC BEQ $DCE2 skips only the used-spell store; it still indexes the eighth word at$DC69 and jumps to already-classified$E202. Seven spells plus cancellation, not eight spells and not cancellation excluded before dispatch.
- Physical$004CF..$0050B contains three twenty-byte computer reach/avoidance arrays, loaded at CPU$82C6/$82D4/$82ED into$0674/$0676/$067A. Local envelope renamed ComputerArenaDistanceParameters; current map's ArenaObstaclePlacementParameters label remains unchanged. Preserve earlier review history; reconcile naming later.

Exact primary instructions and scope: analysis/round02-commentary-questions.md. Unresolved meanings include complete audio/asset container subdivisions, specific instrument names and some AI thresholds. These are explained locally rather than deferred source work.

Fresh final verification is byte-identical; reading-after.asm refreshed. Source SHA256e267f017d8423aa7c474488b1b2e44b28bb60d5a0da0985f3464e1908e53e025. Map SHA256309999c210ff7f476e6da91342d4cd4aab4a929fe8125ece4e8e7060d5f0c947 and original ROM SHA256229427b8b4d46ce322d0b713812d27e180284f26a9cc1abb1cdf3a244289f484 unchanged. Integrity checks: analysis/round02-commentary-integrity.json; build proof: commentary/verification.json. No final audit command run; editorial_audit remains pending.

Remaining Unknown intervals, ends exclusive: $00ABF..$00AC1,$00E87..$00E88,$02481..$02482,$1DC0F..$1DC10,$1DC23..$1DC24,$1F08D..$1F090,$1F9F7..$1F9F8. Ten bytes retain code/data ambiguity without a supported entry or specific data role. Commentary found no new structural premise for them; this is incomplete classification, not terminal exhaustion. Redirect can review these exact residuals and the corrected consumer wording.

No map edits, toolkit changes, archive, publication, cleanup or deletion. Local ROM/source/assets/evidence/history preserved.
