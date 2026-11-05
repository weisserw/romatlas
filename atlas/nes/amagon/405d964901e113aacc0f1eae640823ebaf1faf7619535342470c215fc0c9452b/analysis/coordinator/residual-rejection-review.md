# Coordinator residual rejection review

Checkpoint reviewed: round2-disassembly-final, state/current map SHA256
317037a30049ed1ac66e7a9a945560cbf7eb104f092ab7f812c91b755588ba73.
This is a scoped reasoning review, not terminal acceptance: the region5333 annotation
correction and subsequent phases/final editorial audit are still pending.

The coordinator read the full round1 isolated-rts-review.json and round2 redirect
review.md, then checked their decisive primary evidence independently. See
[raw neighbors and pointer ownership](residual-primary-check.json),
[expanded direct-reference check](residual-expanded-references.json), and
[retained fetch contexts](residual-retained-fetches.json). The checks retrieved
immutable ROM bytes and current map ownership, not merely hashes or prior decisions.

| Exact candidate | Boundary evidence | Competing roles |
| --- | --- | --- |
| physical117608, fixed CPU $CB58 | Prior instruction JMP $CEFE ends at this byte; next PHP/STA body starts $CB59. | Return-only routine or non-code signature/padding. |
| physical122120, fixed CPU $DCF8 | Prior sprite delay ends RTS at $DCF7; distinct OAM installer starts $DCF9. | Orphan return-only routine or non-code boundary byte. |
| physical127720, fixed CPU $F2D8 | Prior JMP $F42D ends at this byte; event setup starts $F2D9. | Return-only routine or non-code signature/padding. |

Each candidate is exactly $60. Its fixed-bank context is effective PRG8K slots
[unknown,unknown,14,15]. The primary neighboring bytes match the saved review.
All 9,109 current accepted instruction starts were represented in the targeted
saved-decode query: no direct decoded references or successors address these three
candidates. Raw little-endian word matches for $DCF8/$F2D8 were checked in the ROM
and are owned by accepted planar Image; $CB58 has none in the reviewed occurrences.
All eighteen retained context aggregate artifacts were inspected for these physical
fetch identities; none contains a candidate fetch. The paginated natural-context
artifact and bounded aggregates are not exhaustive execution coverage.

The preceding JMP/RTS facts exclude ordinary sequential fallthrough only. They do
not prove data. Empty direct-reference/fetch results do not exclude indirect control
or future natural use, and are not a universal caller prerequisite. An independently
supported static entry/dispatch or calling contract could establish Code without
observed execution. Here a solitary legal return supplies hardware semantics but
no independent role constraint that distinguishes it from a non-code $60. Adjacent
routine conventions are suggestive, not discriminating. One isolated byte has no
clear non-instruction pattern, uniform run or bounded data grammar; generic Data
would therefore also overclaim. The accepted generic-Data evidence for unrelated
asset pools does not extend into these control boundaries.

The strongest competing interpretations both satisfy the combined current facts.
Forcing a PC or JSR would execute RTS under a supplied stack precondition regardless
of its original role; that test would not discriminate the alternatives. Broad
later-game/ending routes already exist, but no specific changed route or consumer
connects them to these bytes. An unchanged replay or renamed search has no justified
expected discriminating result. No corrected inference or narrower useful test was
found in this review; this is a reasoning conclusion, not a missing-fact list alone.

Retaining Unknown is sound on these premises. Reuse this review after unrelated
region5333 evidence correction if these bytes, neighbors, entry/format constraints
and retained evidence remain unchanged. Reopen for a concrete new or independently
justified constraint, including a corrected inference on unchanged bytes; no user
permission or observed natural reachability is inherently required. Do not relaunch
unchanged planning automatically. Final terminal acceptance still requires completing
the remaining coordinator workflow and proving current-source editorial readiness.
