# Reviewed uncertainties after round 3

All 220,484 currently classified bytes are readable: 20,952 code, 68,460 data and 131,072 CHR. The 41,676 unknown bytes retain 188 exact physical boundary notes. Their code/data role or extent remains unresolved; classified generic storage does not require an invented internal schema. Prior commentary and uncertainties remain intact under analysis/round-3/commentary-round2 and analysis/round-2/commentary-round1.

The parent/NMI/player/audio contracts from round2 remain valid. The extended audio slice is twenty-five words for $E7..$FF, without a guard for $E0..$E6. Inline dispatchers remove saved returns; pointer resolvers retain them and return into a JMP over inline data. Selector bounds are local contracts, not arbitrary-corruption safety.

Round3 resolved the PRG2 interlude graph under effective [4,5,14,15]. Its large actor reuses $0200 with a different layout from ordinary 32-byte gameplay actors; small objects have sixteen-byte stride. The five-mode initializer, thirteen dispatch tables and twelve sign-selected inline pairs have explicit stack/selector contracts. Only the fixed sound enqueue leaves this graph, preserving banks and stack. The separate pointer resolver at physical $0A831 has coherent mechanics but no established caller. Natural selection of secondary state four and other unobserved states is not claimed.

Two duplicated six-phase transition families use $C8/$C9. Physical $13E72 is LDA $C8, ending the preceding sprite backing; $13E73 is its operand, never an INY root. Mixed audio storage stops at the independently supported PPU helpers $12FDE..$1300E. The ambiguous $1300E..$13010 A9 00 pair, isolated $0AB20 RTS and $0BFDE two-byte tail stay unknown. Replicated reset JMP templates are structurally classified, but MMC1 reset can remap the CPU before their physical fetch. No new runtime coverage follows.

Parent interlude data is bounded to seven pointer words, five thirteen-byte records, seven override bytes and five actor modes. Actor OAM arithmetic selects seven destination words; the eighth aligned word has data representation without an established selection. Ten actor-state frame-pointer offsets are data, but complete per-actor animation source bounds remain unresolved. The nine-word response dispatcher and its inventory/strength handlers are explained; the neighboring unselected sprite-pointer word is classified pointer data with unresolved natural selection. Three single-piece and five four-piece sprite records have independent terminators.

The round2 six-byte stage headers, fifteen-byte room records, forty-byte auxiliary cache, graphics descriptor formats and overlapping sound reset spans remain explained. Room/camera/actor fields are mechanically named; exact original game names and scene assignments remain uncertain. Audio fields can be reused across envelopes, loops and substreams. CHR remains planar tile backing without a complete tile/scene assignment.

Concrete redirect questions remain in the round3 disassembly primary review (analysis/pass-003/review.md), with historical experiments retained:

- Localize actor animation source-record bounds around physical $1E120. Ten frame-offset cells and destination arithmetic do not establish every actor's source layout; follow the actor's actual writer/entry contract.
- Establish separate writer domains for other inline actor state tables. The response selector's finite 2..8 domain does not generalize to unrelated dispatchers.
- Bound mixed PRG1 script/text/asset spans using localized consumers. Known PPU/code islands prevent a blanket data inference. Generic scene/collision backing is classified without claiming complete natural selector domains.
- Revisit the exact small ambiguous tails only with a discriminating reference, entry contract or boundary; do not re-run unchanged pointer decoding or startup loops.

The duplicated text $80 comparison still shadows its second branch; preserve its bytes. Both erasure handlers consume a token, blank the remembered cursor minus $21 and resume parsing. This is a ROM observation, not a toolkit defect.

Prior six-stage, later-room, boss/fairy and THE END artificial progression remains the available runtime evidence. Natural completion, credits and full flight mechanics are not established. No new runtime scenario was needed for commentary. These are analysis uncertainties, not deferred classified-source readability. Final editorial/archive acceptance and whole-ROM classification remain separate.
