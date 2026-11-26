# Round002 current-source commentary completed

Input is verified checkpoint pass-002-reviewed. The entire previous commentary is
preserved at analysis/round-002/commentary-round001 before preparing this changed
baseline. Frozen before/, baseline.json and phase1-state.json are current; the two
map/state hashes are 68110f6fa29cc7e0d4fe1be187839d50c4b5d7151374d15cee17a354aa2785cc.

Current scope: all 19410 Code bytes / 9559 instruction records and all111676 Data
bytes, plus an explicit boundary/ambiguity note for physical[6428,6430). Relative to
round001, 6430 Code bytes /3155 instruction records were added (including21 bytes
corrected from Data);7273 newly accepted Data bytes were integrated. Classification
is still131086/131088, exact99.99847430733553%, not whole-ROM completion.

Read all3216 changed boundary records in compact physical/CPU order. Carried all366
valid semantic unit starts by exact physical boundaries and instruction identity;
revised50 prior summaries affected by solved gaps/contracts. Added155 code/control
units,58 compact data structures and15 bank-tail fallback units:594 meaningful units
in all. Full9629-entry scaffold retained; code entries link clipped semantic-unit
records. Source paths are relative to source/ for eventual audit compatibility.
No classified-source readability ranges remain pending.

New explanations cover reset/default-score initialization, five ten-byte high-score
records, lexicographic ranking and two independent three-character name editors;
signed pitch slides and remaining audio bytecode commands; auxiliary object pool,
pickups and integer dominant/minor-axis line stepping; boss packed records and
falling-object ring; scene-animation channels and bit-pattern expansion; complete
BRK/service/mapper restore path; E05A palette/service initialization. Data structures
have consumer/format explanations and meaningful table symbols. Established RAM,
field, scratch and UI-phase aliases are consistent in operands; generic data retains
honest mixed-format boundaries. Hardware/RAM/ROM and effective image identities stay
distinct. No opcode-byte or map classification was changed by commentary.

Concrete carried defects corrected include every obsolete unknown-gap/only-prefix
claim, the BRK inline-selector uncertainty, mapper write-target uncertainty, raw
service-directory high bytes, and scene-animation RAM64..6A aliases. Nonzero-image
fallback reset JMPs are annotated separately from normal post-mapper fetches in the
destination image. A self-comparison in B0$8F87 is explicitly documented without
changing it; its intended ranking test remains unresolved. Source uncertainty and
bounded next questions are in uncertainties.md.

Fresh final verification.json passes byte-identically with configured release binary;
ROM SHA256 remains0d45759cb5375a3e799a5da6b9a4f3431501350b524901b4f987a48bc2280659.
reading-after.asm was refreshed after the last source batch. Source game.asm SHA256
1ee77387266b2b5c4d036e3523b1acdc5f070e325c36298cb8f63ef974f1e8f5;
symbols.inc SHA256fe81a404fabe6f08886999623d841157d35351d7a26aa7cf08d1a03c1edd7db8.
Map, internal state, ROM and frozen current baseline remain unchanged. Final editorial
acceptance/audit is reserved for coordinator; no audit was run. No new gameplay or
redirect research, toolkit edits, deletion, archive operation, publication or commit.
No toolkit issues found; about2.3GiB free.
