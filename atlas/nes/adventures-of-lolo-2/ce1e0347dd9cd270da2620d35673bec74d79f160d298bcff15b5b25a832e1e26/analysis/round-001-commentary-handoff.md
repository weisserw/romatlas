# Round 001 commentary handoff

Current-source readability is complete. All 341 code regions / 7,306 code bytes,
five classified data/image structures and 141 unknown intervals have been reviewed
and explained in `commentary/source/game.asm`; all 487 scaffold entries retain their
original physical boundaries. Inventory has 341 reviewed routine/sub-block units, 635 code labels
and 202 RAM/hardware symbols. Unknown roles and tentative game meanings remain
explicitly uncertain. There is no deferred classified-source readability work.

Classification remains **63.07%: 41,346 / 65,552 bytes; 24,206 unknown**.
Checkpoint `maps/round-001-disassembly.json`, `maps/current.json` and
`analysis/state.json` remain identical, SHA256
`8fc8b83a8eaf0a83b595f760f28dd1fe685c80196ee35fed45695069a12760d6`.
Frozen commentary baseline, `before/` and all historical evidence are preserved.

Fresh final `commentary/verification.json` reports `ok: true`,
`byte_identical: true`. Source SHA256 is
`2c0a0e4b05d1833b8103df9afbd517c83a6923ea538d5b01e41fcb839a0cb309`.
`commentary/reading-after.asm` was refreshed. Detailed scope/hash assessment:
`analysis/round-001-commentary-scope.json`. Final editorial acceptance remains
pending deliberately; no final audit was run.

Explanations cover animation streams/fractional scroll, password input/checksum,
room and player state, collision carry contracts and object arrays, OAM variable
and fixed formats, two-byte inline PPU descriptor pointers, queued transfer records,
NMI/controller/CHR service and stacked audio note/envelope command interpreters.
RAM overlays are named consistently, rather than assigning conflicting persistent
meanings to shared bytes. Data $3440..$38eb remains generic non-code after the prior
bounded reference/layout inspection; source explains format limits and exact bounds.

Redirect questions, with source entry names and physical bounds, are recorded in
`commentary/uncertainties.md`. Priorities remain L001: establish the unknown $801a
script return-stack contract before adopting return-adjacent bytes; and L002:
distinguish CHR-only local mapper writes from full callee PRG-preservation summaries.
The PPU transfer routine supplies a concrete separate inline-pointer example.
Other unresolved meanings include exact enemy/tile identities, audio parameter
continuations and natural ending reachability. Their established mechanics are
explained; these uncertainties do not require speculative classification.

Toolkit report `analysis/toolkit-issues.md`: existing TK-001 missing feature;
no confirmed/suspected bugs, no blockers and no new commentary findings.
No redirect, final audit, archive, toolkit edits, publication or cleanup performed.
Coordinator should review the current-source readability gate before redirect.

Coordinator gate repair: all 828 review region/unit source references now use
source-tree-relative `game.asm:line` and were checked for file existence and valid
line bounds under `commentary/source`. Scope assessment now checks both region
and unit links and records the review metadata hash. Fresh verification and reading
refresh passed after repair; source hash and frozen baseline remain unchanged.
No final audit was run.

Second coordinator gate repair: every code region now has its reviewed routine or
explained sub-block unit, including name, exact physical range, behavioral summary
and source-tree-relative location. Current totals are 341 units, zero code regions
without units and 828 validated region/unit references. Unit bounds are contained
within their original scaffold regions. Scope assessment checks these requirements.
Fresh byte-identical verification/read refresh passed; source and baseline unchanged.
