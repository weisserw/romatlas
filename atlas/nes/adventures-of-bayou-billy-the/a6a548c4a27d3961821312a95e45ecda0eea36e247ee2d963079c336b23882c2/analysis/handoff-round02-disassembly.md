# Round 02 disassembly handoff — incomplete

ROM classification: 100.00% (262153 / 262160 bytes classified; 7 bytes unknown).
This rounds from **99.99732987%** and does not mean complete classification.

Fresh verified checkpoint: **pass-002-disassembly-20261031-a**.
- Map: `maps/pass-002-disassembly-20261031-a.json`, identical to `maps/current.json` and `analysis/state.json`.
- Map SHA256: `ff89defe97828358320b8a0b360c796abc3e5924cd63b1894c98f36c6fd6c359`.
- Byte-identical rebuild: `analysis/pass-002-disassembly-20261031-a-verification.json`.
- Classified source: `generated/pass-002-disassembly-20261031-a/`.
- Original ROM SHA256 unchanged; no baseline reset, toolkit edit, archive, publication, deletion or cleanup.

Accepted the independently reviewed coherent redirect batch: **110789 new bytes**
(23909 Code, 86880 Data), plus **13 old Static Code bytes corrected to Data** in
C01. Current Code39602, Data91479, Image131072; non-code total222551.
All additions are Static; existing10478 Observed bytes remain preserved.
Eleven ready leads are now verified with appended outcomes/history; R01-L14 remains
blocked by role ambiguity. This is structural verification, not natural reachability
or complete semantic recovery.

Independent review: `analysis/disassembly/round02-20261031-a/review.md`.
Fresh configured-binary consumer reads: `primary-consumer-review.json` in that
directory. Separate reparsing in `independent-format-check.json` confirms all17
RLE screens,23 dialogue scripts/38tile strings,370bank2/158bank12 sprite records,
11 scene pools, projection rows,6brawl event lists and3demo streams.
`independent-audit.json` retains every data proposal's pattern/boundary review,
physical views,14E116 barriers and replicated footers; `acceptance-log.json`
records actual supported annotations. Counts, legal decoding and conflict checks
were supplementary, not sole evidence. Saved primary decode windows may extend
into adjacent data or start at a prior operand; they are immutable inspection,
not blanket code candidates. Graph/contract review determines accepted boundaries.

Critical carry-forward decisions:
- C01: mode2 correlates banks10/11 with entry800F. Five three-byte arrays at
  physical14010..1401F are consumed by81EF/805F/80D7; the old8000 path lost that
  mode/bank correlation. Removed only five Static records covering13bytes.
- Four always/exhaustive branch exclusions and all14E116 inline barriers retained;
  player table gains final wordC324. Shooting ignored state0 words may overlap
  preceding code and remain excluded from Data table claims.
- Generic audio/script/constant pools are classified after bounded consumer,
  reference, pattern and neighbor inspection; finer schemas/identities remain
  unresolved. Do not upgrade them into invented precise tables or sample spans.
- Projection is a consumer-bounded quantized32×128 lookup with separateFFborder;
  no exact original generator claim. Scene7/8 cross-table origin is not a column.
- Raster copy8432 independently proves153bytesDAE5→0600. Added135unknown source
  bytes as Data/code template; retained18existing Observed ROM-helper bytes Code.
- Six footer copies have8bytes Static code template +6vector bytes each. Pre-INC
  local view, copiedJMP template and actual final-bank post-reset fetch remain
  separate claims. Fixed code views use[null,null,14,15]. Bank wrapper return
  contracts require busy0 and balanced bank stack; no lowbank placeholder inference.

Exact remaining Unknown intervals (half open physical offsets):

| Range | CPU view | Effective PRG slots |
|---|---|---|
| $C1E2..$C1E5 | $81D2 | [6,7,14,15] |
| $C7A6..$C7A7 | $8796 | [6,7,14,15] |
| $14379..$1437A | $8369 | [10,11,14,15] |
| $1ACD0..$1ACD1 | $ACC0 | [12,13,14,15] |
| $1EE00..$1EE01 | $EDF0 | [null,null,14,15] |

Independent primary neighbor review found no newly spanning entry, stack,
consumer, finite-table or larger template constraint after accepting surrounding
graphs. First is a plausible JMP game-over stub; others are isolated60bytes.
A stored scalar/constant remains a competing interpretation. Neither code nor
non-code is established. Do not repeat artificial entry or decoding alone; seek
a discriminating constraint if commentary/redirect exposes one. Localized prior
review was checked against primary evidence, not treated as a historical veto.
Exact unknowns and per-bank/provenance counts: `unknown-final.json` and
`final-counts.json` in this attempt; also saved in progress/investigation.

Gameplay checklist reviewed and saved evidence reused: startup/stage1, all three
mechanics, later city/mansion, boss combat, moving credits, THE END and Start exit.
Late-game routes are artificial; natural boss defeat/ending remains unproved.
Deferred similar-stage/practice variants supplied no concrete residual-entry
hypothesis worth unchanged replay. No new emulator experiment was needed.
Productive normal gameplay/static/ready easy gains are exhausted for this pass;
this is an incomplete handoff, not terminal coordinator acceptance.

**Commentary carry-forward:** entire existing28-file `commentary/` remains
unchanged, including its high-quality round01 source, symbols, readability records,
uncertainties and historical evidence. Hash inventory:
`analysis/disassembly/round02-20261031-a/commentary-preserved-hashes.json`.
Next commentary must preserve sound existing explanations while incorporating
new classified code/data, the C01 correction, specific data formats, generic
fallback limits and all7unknown bytes. Do not replace it with the raw newly
generated listing. Full current-source readability and final editorial acceptance
remain separate from this byte-identical classification checkpoint. Redirect follows
commentary; new useful role/entry premises can then be investigated without forcing
seven bytes to satisfy a rounded percentage.

Toolkit: existing **NES-001 missing feature** only; no new confirmed/suspected
bugs or blockers. See `analysis/toolkit-issues.md`. Failed in-memory API attempts
were caller-shape corrections, saved in `api-note.md`; neither saved map/state.
Local evidence retained. Free9.63GiB, scratch587.5KiB at final counting.
