# Round 3 commentary gate — complete

Current-source readability is complete on the unchanged pass-003 map. Fresh supported verification passed with `ok=true` and `byte_identical=true`; `commentary/reading-after.asm` was refreshed after the last source edit. Final editorial/archive acceptance remains pending and was not run.

All **11300** scaffold entries and discovered units are reviewed. Current scope: **59266 Code**, **340433 Data**, **92508 Text**, **131072 Image** and **32097 Unknown** bytes. There are **313 Unknown ranges**, each with an honest source-local note. The 1822 Code fragments decode to **27680 instruction starts**; instruction bytes and reviewed-unit Code bytes both equal **59266**, with no missing/extra bytes and unit boundaries on instruction boundaries. Source-local kinds match the current full-ROM scaffold. All 982 local source path references resolve.

All 46398 earlier Code bytes carried exactly. New readability scope is **215 native fragments / 12868 Code bytes**, **454 Data fragments / 56994 bytes** and **250 Text fragments / 2377 bytes**. Prior pass-002 commentary is preserved at `prior-pass002-commentary/`. Exact checks and hashes: [gate.json](gate.json), [source consistency](current-source-consistency.json), [verification](gate-verification.json), [Data review](data-review.json), [Text review](text-review.json).

Examples in `commentary/source/game.asm`:

- Line 854: bank 0 packed-flag callback, related magnitude/adjustment helpers and native exits. Shared action-work symbols name 101 previously numeric operands.
- Line 92063: 110 character cells plus two special text pointers. The six disputed name-grid bytes retain Data boundaries; directional branches do not invent fallthrough code. Name input and option callbacks have explicit native units.
- Line 91711: active-A000 category reads retain ROM identities. Twelve mapper-register aliases on reads were repaired; write operands retain mapper names. Another 43 aliases used as unrelated arithmetic offsets were restored to numeric offsets. Helpers now guard both cases.
- Line 37940: generic packed-cell/metadata/sparse-row bytes keep their bounded pattern explanation and unresolved producer/schema. Eighteen false uniform draft notes were repaired; all 152 remaining uniform-run claims match direct ROM byte checks.
- Line 81306: frontend message/control pool after bank 56 installs physical bank 53. Printable fragments remain separate Text; controls, numeric fields and pointers retain composite Data boundaries. Embedded zeroes in fixed labels appear as literal `\0` in comments.

The callback pool has **143 entries / 102 distinct targets**. Its separately bounded ID lists contain **90 distinct IDs / nineteen zero bytes**. Full natural selector use is not claimed. The saved primary Data hashes all match (**462 candidates**). Thirty-two bank-qualified Data read aliases now name understood tables, replacing 33 numeric operands without assuming additional selector extents.

Availability review revised **52 of 58** stale paragraphs. Six retained statements independently match current Unknown boundaries: bank 55:8012, bank 59:801B, bank 60:8BC4/8DED and bank 62:DB99/DD10. See [retained dependencies](availability-review-retained.json). Generic patterns, record fields and arithmetic ratios retain ordinary uncertainty. `commentary/uncertainties.md` records the concrete bank-56 party-predicate register-contract question for a later bounded redirect check; this is not skipped commentary or new classification evidence.

Protected identities:

- ROM: `2d31a3d101fd629f8ff159de4ca15785444e4a3deb8b4fd7f114fa5cdef5ce87`
- Map/state/pass-003: `2309e48cb9b38fc5e68fc82b7e8ccdfb998d76cb86ce61093fb14f4fbed967aa`
- Source game.asm: `1a2a64aae2afd404a9133f734c4c34057d00fb740b82d981679252ad0bc266eb`
- Review: `e509f3996a63d5d1278ac997beb31d2b9942adb2f3481570c2404a095168f0c1`
- Reading copy: `55fdac7dc11a7248b958ec046e987a3356569489fc23b466c099710fae5ecfe2`

No toolkit issues found: zero confirmed bugs, suspected bugs or missing features. No ROM, toolkit, map or state edits; no live server, archive or publication. Classification remains **95.10%**, not whole-ROM completion. The coordinator may continue its workflow; final editorial acceptance remains separate.
