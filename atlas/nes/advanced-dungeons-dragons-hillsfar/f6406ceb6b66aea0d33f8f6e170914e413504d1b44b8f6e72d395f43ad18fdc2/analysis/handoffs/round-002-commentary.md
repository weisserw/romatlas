# Round 002 commentary handoff

**Full current-source readability complete; ROM classification incomplete at 68.73%.** Current checkpoint remains **pass-002-verified**, with 180,185 classified / 81,975 unknown bytes. No classification, ROM, toolkit, runtime, publication or archive changes in commentary. Final editorial/archive acceptance is reserved.

[Source](../../commentary/source/game.asm), [symbols](../../commentary/source/symbols.inc), [reading copy](../../commentary/reading-after.asm), [review inventory](../../commentary/review.json), [scope report](../commentary/round002-review-summary.json), [verification](../../commentary/verification.json), [integrity](../commentary/round002-final-integrity.json). Entire previous commentary retained under `analysis/round-002/prior-commentary` before fresh supported preparation.

## Covered scope

All 2,976 scaffold regions have current source locations and reviewed readability status; no remaining readability range is deferred. Scope is 22,103 Code bytes / 10,575 accepted instruction starts, 72,754 Text bytes and 85,328 Data bytes. All unknown intervals have explicit boundary/ambiguity notes. 1,360 exact kind/range blocks carried valid prior prose, names and review; changed dependencies were corrected and carried edits freshly rebuilt. This is separate from execution coverage and structural validity.

509 combined entry contracts, 1,699 named instruction locations and 690 meaningful fragment inventory units explain the accepted code without inventing routine boundaries. New work covers all 34 fixed script handlers, 37 tavern dispatch entries and connected helper/stack contracts; arena action streams; archery equipment, aim, trajectory/error stepping and object/collision mechanics; lock generation/timer/menu; guild offers; fixed arithmetic, PPU/text and bank-switch continuations. Mode-local aliases distinguish reused RAM. 42 graphics and 27 screen resource comments give exact decoded sizes, consumer grammar and storage context; directories, script framing, tavern ordinal/count records and generic non-code bounds have light explanations.

Concrete examples: `ScriptAdvanceStage` adds four and aborts only for a result below decimal 13; otherwise it returns normally. `ArcheryStepLine` explains major/minor extent error accumulation. `TavernCallStream`/`TavernReturnStream` explain the local two-byte return stack and unchecked bounds. `CurrencyTrySubtractWord` returns Y=0/1 for success/failure. Generic non-code remains honestly generic where field schemas are unresolved.

**R001-COMMENTARY-CONTROLS corrected:** UP=$10, DOWN=$20, LEFT=$40, RIGHT=$80. Menu Up decrements, Down increments. Lock menu record X=8 stores selection at $064E, while the generic selection field base remains $0646. Travel speed/pose prose now matches Left/Right and Down. Verification confirms these semantic name changes preserve numeric operands.

## Required structural correction lead

**R002-COMMENTARY-PAGINATION-PROMPT** is ready for separate redirect/disassembly. [Primary artifact](../commentary/round002-pagination-prompt.json) includes exact bytes, current region, consumer, flag constraint, bank context and saved-root comparison. Physical **253734..253751**, fixed CPU **DF16..DF27**, is the 17-byte span `HIT`,00,`SELECT`,00,`BUTTON` copied by DEE7..DEFC. The first four bytes are currently Static Code (PHA / EOR #54 / BRK); the remaining 13 are Unknown.

DF10 AND #04 establishes Z; DF12 BEQ DF0E loops while zero. Its fallthrough has Z clear, and DF14 BNE DEC5 must branch without an intervening flag change. This excludes fallthrough into DF16 on this path. No exact saved instruction root in `observed-final-roots.json` occurs in the prompt span. Current Code provenance is Static closure from `fixed-connected.json`, not natural/observed prompt execution. The copy count establishes a consumed span; internal zero bytes do not imply one zero-terminated string.

Affected structural review credit is explicitly invalidated locally in review.json; source preserves its current emitted decode with a clear conflict explanation. Readability is complete despite this explained structural conflict. **Review and correct this before any terminal classification claim**; do not repeat unconstrained static fallthrough as a contrary entry contract. Check other genuine entry evidence, accept a localized role/boundary correction independently, then reverify through the ordinary workflow.

## Remaining meanings and actionable followthrough

- Follow saved **R002-TAVERN-LOCAL-RESIDUAL**: BF4B..BF57 (12 bytes) and BFD8..C000 (40 bytes) in PRG pair 24/25. Shared reset/trailer contracts may support narrower claims; a lone RTS/legal decode does not establish code.
- Lock/tool success and maze progression still need the actual object acceptance contract. Corrected pick-menu selection alone did not produce success. Guild reward needs state 2 / X=0 entry, not A-search with X=64.
- Arena controls do not establish victory; archery shot/miss does not establish all hit effects. Retirement dialogue does not establish natural quest completion or credits. Saved routes, snapshots and artificial provenance remain reusable.
- Several event fields, service subtypes, character flags, sound identities and image identities remain mechanically named. Mixed unknown ranges remain local; no map classification was invented to finish prose.

See [uncertainties](../../commentary/uncertainties.md), [leads](../leads.json), existing redirect/disassembly handoffs and [scenarios](../scenarios.json). No new runtime experiment was necessary for readable current-source mechanics.

## Verification and next phase

Fresh final rebuild is **byte-identical**, source SHA-256 `13eda2117ca001e6cb800818b3dfbe94ccedc3119bbf96b4bbca2ef31ed2baea`. Reading copy refreshed after the final source edit. Current map, state, frozen checkpoint and commentary baseline hashes agree. Input ROM and configured release binary match prior integrity evidence. No remaining classified-source readability scopes; final editorial audit remains pending.

**No toolkit issues found**; zero new bugs/features/blockers. [Persistent report](../toolkit-issues.md). Local carry-forward/source naming repairs were resolved and verified; prompt classification conflict is not a demonstrated toolkit bug. Next phase: separate redirect with the prompt correction and saved independent leads. This is an ordinary completed commentary handoff, not terminal exhaustion or whole-ROM completion.
