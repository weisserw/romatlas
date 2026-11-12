# Round 001 redirect handoff

Input checkpoint `round-001-commentary-final`; structural baseline
`round-001-disassembly-final`. Attempt `round-001-attempt-001` is complete as a
combined **ready proposal batch**, not a terminal exhaustion decision or acceptance
of new classifications. Maps/source/ROM are unchanged.

Current accepted accounting remains 59496/65552 classified, 6056 unknown (90.76%).
Five leads propose independent review of all 6056 unknown bytes: 1991 Code and
4065 Data. This is proposed gain only; accepted gain is zero. Two standalone
helpers (45 bytes) have weaker uncalled static contracts and are explicitly
separated so a rejection does not invalidate the connected handler/data batch.

Ready IDs in `analysis/leads.json`:

- `R001-TITLE-NMI`: installed $82CA interrupt contract; physical $02DA..$030E.
- `R001-INTERPRETERS`: player/car command and collision contracts, shared helper,
  direction/motion/animation data; 1386 bytes proposed.
- `R001-MIXED-GAPS`: street behavior/script/hit continuations, projectile responses,
  replacement-object helper and dependent non-code tables/scripts; 2405 bytes.
- `R001-BONUS-CONTRACTS`: $D9FA..$DC9C coherent bonus handlers, connected $CA9F/$D096
  callees and $DC9C..$E24B non-code container; 2168 bytes.
- `R001-STANDALONE-HELPERS`: edge-input function $8FAC and saturating meter-add $9C44;
  45 bytes. No caller or natural reachability claim.

Authoritative exact ranges, proposed types, CPU aliases and [0,1,2,3] contexts,
source links and bounded review recipes:
`analysis/redirect/round-001-attempt-001/proposals.json`.
Primary mechanical/structural findings and all connected continuations:
`analysis/redirect/round-001-attempt-001/structural-findings.md`.
The exact unchanged nineteen-range complement with evidence-linked dispositions:
`analysis/redirect/round-001-attempt-001/unknown-ledger.json`.
`dispatch-inspection.json`, bounded `linear-*.txt`, `data-inspection.json` and
`data-conflict-check.json` preserve supporting immutable inspection. Linear decoding
is an aid, not a proof; restart decoding at $3E0A after six data bytes. No table-wide
selector/reachability claim follows from pointer sequences. Generic Data proposals
have positive repeated-format/pattern and consumer evidence with independently
anchored container bounds, without claiming original subtable segmentation.

Independent avenues were followed in this same redirect: title installation,
player and car interpreter/collision paths, street object/projectile graph, shared
bonus handlers and both previously unknown callees, small uncalled routines and
all adjacent data containers. No identified actionable structural avenue is being
saved for an artificial extra redirect cycle; disassembly should review the combined
batch. Original semantics, exact subtype domains and art meanings remain locally
uncertain but are not added prerequisites for generic Data.

Driving research materially improved `R001-DRIVING` to verified artificial visibility.
Main $81FB still executes its introductory card at +300 frames; direct $D1FC body
initializes stage$11, NMI$D669 and a visible road/car/speed HUD by +240 frames.
Up/right45 frames moves car X$6E->$8C and speed30->38. A source-backed $CD=$FB row
patch reaches StreetActive=0 with StreetEndRow=$FC. `analysis/scenarios.json` retains
old attempt history and records new visits/patches/budgets. Selected evidence:
`driving-direct.png`, `driving-steered.ppm`, `runtime-log.jsonl`, reusable local
`driving-direct.snapshot` and `driving-steered.snapshot` in the attempt directory.

Limit: direct $D1FC edited SP without constructing a caller frame. In-mode observations
are valid; after its RTS this is not evidence of the main outcome branch. Improved
caller-correct recipe is PC=$8200 (real JSR$D1FC), SP=$80, from clean active state.
Natural success additionally requires the type12 object interaction at $B3A7 with
CarSpeed >=$58, which sets $A9; ending the route alone does not prove it. Further
natural gameplay permutations have diminishing structural prospects now that the
entire dependent graph is in the proposal; no natural-ending claim is made.

Verification: `unchanged-verification.json` confirms exact initial/final SHA256 for
ROM, map, state, game.asm and symbols.inc. Map/state remain
`5176915eb58cb7ad441494c0df2f162baffc5b3e64b05100545c82f4c300b94c`.
All bounded runtime servers finished. Disk has 7.8GiB free; attempt evidence is
12MiB. No evidence cleanup, map edits, commentary regeneration, archive, toolkit
changes or publication occurred. Existing confirmed documentation issue NES-001
remains; no new toolkit issue was found. Final classification, rebuilding and
editorial acceptance remain disassembly/commentary/coordinator work.
