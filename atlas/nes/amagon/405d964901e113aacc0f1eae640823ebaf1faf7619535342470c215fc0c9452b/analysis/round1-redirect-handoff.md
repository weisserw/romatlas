# Round1 redirect handoff

Ready for one independent disassembly review batch. Research attempt
`analysis/redirect/r1-20261105-structural` is complete; classification, state and
commented source are unchanged. No runtime/gameplay replay was needed.

Accepted checkpoint: `maps/round1-disassembly-final.json`, `maps/current.json` and
`analysis/state.json` are byte-identical SHA256
`f26ec6e4448f5f469d5adbbf78e36d04572fbbdb1f6bb5ac8c0d39675ac8edd2`.
ROM SHA256 remains `405d964901e113aacc0f1eae640823ebaf1faf7619535342470c215fc0c9452b`.
**Accepted counts remain 65,807 classified / 65,281 unknown (50.20%).** Current-source
commentary is complete; final editorial audit remains pending.

[Final proposal](redirect/r1-20261105-structural/combined-proposal-final.json)
contains **65,278 new unique proposed bytes: 8,236 Code, 56,428 Data, 614 Text**.
An additional **99 existing Code bytes are proposed for correction to Data**;
that correction adds no coverage. Zero candidate code/data conflicts or overlapping
instruction bytes remain. **If every proposal is accepted, three bytes remain
unknown**; this projection is neither accepted classification nor whole-ROM completion.
[Exact current complement](redirect/r1-20261105-structural/unknown-ledger-final.json)
retains all 193 accepted unknown ranges, split into 200 bank-specific ledger rows,
with neighbors, CPU aliases, proposed segments and residual dispositions.

The proposal's `segments` partition new coverage against the immutable map;
`claims` retain supporting parent structures. Apply segments only after review.
The `correction` object separately identifies the two existing regions to replace.
All new code is Static, with explicit physical/CPU/bank views. No new fetch or
natural-selection claim is made. Research scripts reproduce decoding and proposals,
not annotation; do not rerun the one-time lead-history finalizer unnecessarily.

Seven ready leads are recorded in [leads.json](leads.json), preserving original
three entries/history: `R1-scene-assets`, `R1-state-dispatch`, `R1-graphics-gaps`,
`R1-fixed-tail-provenance`, `R1-audio-contracts`, `R1-static-local-contracts`,
`R1-residual-structures`. Gains overlap; use the combined unique counts above.

The source scan covered current commented and opaque/generated sections: all 5,239
accepted instruction starts were decoded, references and dispatch/stack contracts
traced, all supported ordinary branches/callees continued, newly exposed indirect
paths revisited, and residual consumer gaps inspected directly. It followed all
26 main states; 13+13 scene handlers; ten boss substate tables plus secondary state93;
95 actor behavior and matching graphics rows; eight AI and eight PPU handlers;
event type/subtype/initializer paths; audio opcodes/sounds and timed completion
callbacks. Bank-five wrappers preserve caller banks; scene handlers use bank6;
fixed instructions retain physical banks14/15. Fixed traversal's 20 switched-target
barriers were resolved by separate explicit bank contracts, not silently ignored.

Useful primary evidence:

- [Final code](redirect/r1-20261105-structural/fixed-connected-code-final-review.json),
  [banked code](redirect/r1-20261105-structural/switched-connected-code-audited.json)
  and [boundary audit](redirect/r1-20261105-structural/code-boundary-audit.json).
- [Final vector review](redirect/r1-20261105-structural/vector-final-review.json):
  NMI is `$d8fe`; 95 eight-byte metadata rows end at physical130777, followed by
  295 patterned bytes exactly duplicated at98009..98304. This independent payload
  evidence supports correction; the invalid root alone would not. Eighteen retained
  context artifacts show no tail fetch, without excluding every indirect route.
- [Scene events](redirect/r1-20261105-structural/scene-events.json),
  [weights](redirect/r1-20261105-structural/timed-encounter-weights.json),
  [terrain](redirect/r1-20261105-structural/terrain-contract.json),
  [metatile/screens](redirect/r1-20261105-structural/screen-metatile-contract.json),
  [audio](redirect/r1-20261105-structural/sound-descriptors.json) and
  [HUD/Text](redirect/r1-20261105-structural/hud-text-contract.json).
- [Generic fallback review](redirect/r1-20261105-structural/generic-data-review.json),
  [complete residual bytes](redirect/r1-20261105-structural/residual-full-hex.txt)
  and [consumer references](redirect/r1-20261105-structural/residual-consumers.json).
  Known roles are retained; generic non-code is used where pattern/boundaries are
  clear but complete semantic schemas are not. No original generator or missing
  live caller is imposed as a universal prerequisite.

Supersession matters. Use `*-final-review`, `*-audited`, `combined-proposal-final`
and `unknown-ledger-final` above. Earlier artifacts remain historical. Main state
extent is52 bytes/26 words, correcting the original50-byte lead. Score extent is
8 words supported by AND7; the earliest pointed handler alone incorrectly admitted
intervening code. Final actor pools have23 rows/95 total, yielding760 metadata bytes,
not provisional768; the duplicate tail is295, not the earlier287-byte subinterval.
Twenty-six provisional rectangle-data bytes were legitimate code continuation;
final Data starts89969. A proposed291-byte collision-data gap was split after full
hex review exposed a meaningful52-byte player-foot routine at119733..119785.
The alternate wrapper's CPU entry is `$e39e`, not the earlier typo `$e3ae`.

No identified promising actionable avenue remains outside this packaged batch.
Connected handlers/assets and independent scene/audio/graphics opportunities were
investigated in the same redirect. Broad later-game/ending coverage already exists
in scenarios.json; repeating it has no directed discriminating hypothesis here.
External entry/natural use of the local routine group remains unresolved; internal
calls connect its helpers. Its RAM/stack/control/data contracts justify independent
Static review. Unselected
records retain structural claims without invented natural selection. Unknown
original pool organization or data meaning does not defer clear non-code.

The only projected residuals are physical117608 (`$cb58`),122120 (`$dcf8`) and127720
(`$f2d8`), each a solitary `$60`. Their [primary-evidence review](redirect/r1-20261105-structural/isolated-rts-review.json)
records neighboring bytes, all reviewed instruction references, retained fetch
contexts and raw pointer-word occurrences. There is no independent entry/format
contract or fetch. Raw word matches fall inside accepted planar Image. A return-only
routine and a non-code byte both fit; absence of execution proves neither. Forcing
PC/JSR would merely execute RTS and would not discriminate structure. Retain Unknown
unless independent entry or format evidence emerges. This review is reusable after
disassembly; this handoff is a ready batch, not terminal workflow exhaustion.

Disk remains8.9GiB free; attempt evidence about28MiB. No toolkit edits, deletion,
cleanup, archive, publication, map/state/source mutation or final audit performed.
