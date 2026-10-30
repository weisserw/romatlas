# Round01 commentary handoff

**Full current-source readability complete. ROM classification still incomplete:
94.88%, 2,100 unknown bytes. Final editorial/archive acceptance remains pending.**

Started from verified **pass-002**, unchanged map/state hash
`ce45be0835a8cfcc257414a9ba00a884e3bfb25b46c8c9474a2159b14b959e04`.
All 117 physical scaffold regions have reviewed source locations and 279 compact
reviewed units. Covered **11,290 code bytes + 19,394 data bytes + 8,192 CHR bytes**;
all 19 unknown intervals have honest opaque-boundary notes. No remaining
classified-source readability ranges or interrupted frontier.

[Readable source](../commentary/source/game.asm), [reading copy](../commentary/reading-after.asm),
[review inventory](../commentary/review.json), [accounting](commentary-round01/readability-accounting.json).
218 named behavior entries, 705 routine/control labels, 126 zero-page symbols,
player/OAM fields and hardware constants, 102 ROM table/stream names. All direct
RAM/ROM operands and supported branch/call/jump targets use meaningful symbols;
unknown event identities use functional names and tentative prose.

Source explains reset/title and skill selection, foreground/NMI handshakes,
11-state NMI dispatch, 18 eight-byte player records, synthetic RTS dispatch and
nonlocal stack exits, 9×9 contacts in 16-byte rows, ball/AI cadence and steering,
scroll queues/compressed tile strips, scores/clock and four-channel audio commands.
Data stays exact local incbin emission with consumer/layout notes and named CPU
equates separate from physical emission labels. Generic data remains generic;
commentary does not invent structural classifications or natural reachability.

Fresh **byte-identical verification passed** using configured binary via
commentary.py verify, and commentary.py read refreshed the reading copy.
[Verification](../commentary/verification.json); final source SHA-256
`7685e3a7f6e442e1e929ea05d0baea2b7641c0e699902c8239f88921f448c560`.
Input, current map/state and frozen commentary baseline identities checked unchanged.
No final audit invoked; `editorial_audit` remains pending as required.

[Unresolved meanings/redirect questions](../commentary/uncertainties.md) include
player-handler entry/stack contracts into unknown islands; code-like scoreboard
and down/progress gaps; steering and pre-FF suffix code-like fragments; audio
interrupting gaps. Known state handlers return animation/palette in $12/$13 through
the synthesized continuation at CPU $9ED0; pointers alone do not establish code.
Exact 19 residuals remain [unknown-ranges.json](evidence/round01/unknown-ranges.json).
Preserve prior investigation/lead outcomes when selecting bounded next tests.

Toolkit: **no new commentary findings**; persistent [report](toolkit-issues.md)
retains **one confirmed input protocol mismatch**, no blocker; complete buttons
arrays are the tested workaround. No toolkit changes/archive/publish/delete.
All source, assets and evidence remain local.
