# Current-source limitations — round 001

Physical $39C7 (CPU $B9B7, effective PRG bank 1) remains Unknown. The preceding
code tail-jumps to FadeMusicPulseEnvelope; the next routine starts at UpdateMusic.
The byte encodes RTS in isolation. Neither that encoding nor missing execution
establishes intended role. L005 and analysis/round-001/residual-review.json preserve
the redirect question; commentary has not reclassified it.

Enemy names describe movement and drawing mechanics, because exact species names
and the designer's naming are not recovered. The eight-slot arrays are parallel;
EnemyDirectionOrDeltaY and action/pose fields deliberately serve different purposes
in different handlers. Source documents each established algorithm rather than
assuming one uniform object struct. Sound effects likewise use register/timer
mechanics rather than unconfirmed event names. Exact audible/game associations can
be recovered later without blocking current-source readability.

The player jump gap counter, stage-stream mode and some boss phase flags are named
from local consumers. Their exact design intent and every naturally reachable
combination remain uncertain. Artificial progression evidence stays separate from
normal reachability; no commentary claim expands execution coverage.

Large accepted data blocks retain partial internal schemas. The local source
names known table views and explains consumers, while preserving undecoded payload
through incbin. Individual music tracks, compressed-row boundaries, map-animation
structures and CHR tile meanings are not all named. This is semantic uncertainty,
not unknown classification. Biased table aliases can point before the first indexed
element, including into adjacent code; their use does not classify those base bytes
as additional data. PRG views remain fixed under CNROM; only CHR bank selection
changes through the mapper writes.

The music parser compares command tokens against $EF, while the independently
bounded eight-entry command directory represents $F8-$FF. Other high tokens would
index beyond that directory; no support for such tokens or valid stream use is
claimed. Music substreams have one saved return pointer per channel, so recursive
calls must not be assumed. Source explains these parser mechanics without claiming
a full music format specification or hardware compatibility.

Current-source readability and full-ROM classification are separate. The review
scaffold covers every current source region; final editorial_audit remains pending.
No toolkit changes, publication or final acceptance audit occurred in this phase.
