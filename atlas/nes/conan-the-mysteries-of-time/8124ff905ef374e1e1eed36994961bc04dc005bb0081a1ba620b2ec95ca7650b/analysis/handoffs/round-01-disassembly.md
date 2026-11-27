# Round 01 disassembly handoff

Pass complete; ROM incomplete. round=1 phase=disassembly. Next: full current-source
commentary, then redirect on the connected asset/handler questions below.

ROM classification: 14.54% (19055 / 131088 bytes classified; 112033 bytes unknown).

Code 13531 bytes; data 5524 bytes, including 16-byte header.
639/639 classified regions have evidence; 6462 supported decoded instruction starts.
Execution is separate: at least 4129 unique fetched opcode starts in captured contextual pages;
2473 in natural startup/gameplay windows and 3580 in artificial windows (overlap included).
The startup context page omits one of 1001 roots; no unrecorded root was used for classification.
Zero aggregate drops in saved trace summaries; detail capacity 0 intentionally drops detail.

| Physical 8 KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 0 | 128 | 8064 |
| 1 | 0 | 1537 | 6655 |
| 2 | 0 | 0 | 8192 |
| 3 | 0 | 0 | 8192 |
| 4 | 0 | 0 | 8192 |
| 5 | 0 | 513 | 7679 |
| 6 | 0 | 0 | 8192 |
| 7 | 0 | 0 | 8192 |
| 8 | 0 | 0 | 8192 |
| 9 | 0 | 0 | 8192 |
| 10 | 0 | 0 | 8192 |
| 11 | 0 | 1615 | 6577 |
| 12 | 1879 | 0 | 6313 |
| 13 | 1290 | 626 | 6276 |
| 14 | 4490 | 118 | 3584 |
| 15 | 5872 | 971 | 1349 |

## Artifacts and verification

- [Internal state](../state.json), [portable current map](../../maps/current.json).
- [Frozen checkpoint map](../../maps/round-01-disassembly.json),
  [source](../../generated/round-01-disassembly/game.asm),
  [verification](../round-01-disassembly-verification.json): valid map, byte-identical rebuild.
- [Instruction evidence](../round-01/classification-instructions.json),
  [unknown complement](../round-01/classification-unknown.json),
  [data pattern review](../round-01/data-pattern-review.json),
  [bounded RPC history](../round-01/rpc.jsonl), [counts](../round-01/counts.json).
- [Reproducible scenarios](../scenarios.json), [research](../gameplay.md),
  [lead outcomes](../leads.json), [investigation](../investigation.md).
- [Ending](../round-01/terminal-no-input.png),
  [inventory](../round-01/inventory-select.png),
  [ship](../round-01/advance-stage3.png); other stage captures/snapshots linked in scenarios.

## Scope and evidence

Verified vectors $CAA6/$CAFB/$CAE2; code includes accepted fetches and connected static
control flow, not legal decoding alone. Fixed UxROM banks 14/15 allow upper-code
continuation while lower banks invalidate at calls/writes. Inline words are consumed
via $D611 through $D4D1/$D55D/$D5E6: honor these barriers during editorial review.
Fourteen stage tables at physical 0x1D971..0x1D9E0 are finite eight-byte parallel
arrays guarded by stage <8; consumer-specific field labels remain conservative.
Uniform runs are generic data after reference/pattern checks, not asserted fill.
All stage indices 0–7 and 'Conan the King' terminal screen visited, with representative
movement/attack inputs and inventory. Later progression/inventory are artificial
RAM edits, not natural reachability. Natural cave route and exact patches/budgets
are in scenarios; snapshots are durable local files, not server-local names.

## Outcomes, questions and deferred work

R1-L01 inline protocol, R1-L02 stage shortcut and R1-L03 tables verified. No initial
ready leads existed. Ordinary play and obvious static continuations yielded a
substantial connected batch; every remaining range was not exhaustively scanned.
Next R1-Q01: follow $E27E/$E341 -> stage record roots -> $D8EC/$D91C data consumers,
recover bounds or supported generic-data slices across banks 0–11. Missing exact
format endpoints requires redirect research; do not require semantic names for
clear non-code. R1-Q02: follow scheduler/entity indirect dispatch to residual engine
code/data, preserving inline barriers and bank context. Missing selector contracts
justify deferral to redirect, not a terminal blocker. No terminal rejection review
is needed for this ordinary easy-gain handoff; no terminal exhaustion claimed.

Full current-source readability/editorial coverage is pending commentary.
No toolkit issues found; [report](../toolkit-issues.md): zero confirmed/suspected bugs,
zero missing-feature findings, zero blockers. Input/toolkit unchanged; no publication,
cleanup, archive or broad commit. Disk remains >3 GiB free; evidence ~90 MiB, log bounded
at 128MiB; no live commands require resumption.
