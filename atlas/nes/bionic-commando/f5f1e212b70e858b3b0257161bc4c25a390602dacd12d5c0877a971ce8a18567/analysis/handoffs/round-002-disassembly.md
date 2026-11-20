# Round002 disassembly — verified substantial pass

ROM classification: 97.85% (256535 / 262160 bytes classified; 5625 bytes unknown).

Input round-001-final-tables, map/state93cee40f9b4155b0230113cdb24f0bc550ddb2964c957f5fac942125a498cef8.
Final **round-002-final**; maps/current.json, analysis/state.json and frozen
maps/round-002-final.json SHA256 **8b9f176e93f6c9b137dee1440e49653ab5916fe0e1d2380cc255db2385825b65**.
Map.validate ok; fresh generated/round-002-final rebuilt byte-identically, report
analysis/round-002-final-verification.json. ROM SHA unchangedf5f1e212b70e858b3b0257161bc4c25a390602dacd12d5c0877a971ce8a18567.
All old classifications preserved unchanged. Intermediate round-002-disassembly also
verified; its6000 residual is superseded by final5625 after375-byte residual-data batch.

Gain **95,504 bytes**:35,277 Code and60,227 Data. Final total Code43,751,
Data113,456, Image99,328 (header included in Data). No conditional proposed bytes
included. Leads R001-REDIRECT-CODE and R001-REDIRECT-DATA verified with corrections;
lead attempt histories updated. Full per-bank counts and observations appended to
analysis/progress.md and analysis/investigation.md; machine counts and exact final
unknown complement in analysis/evidence/round-002/counts.json and unknown-ranges.json.

Independent evidence: analysis/evidence/round-002/review.md, review.py,
independent-review.json, graph-audit-summary.json, annotation-results.json,
residual-data-results.json, transfer-graph.json/.txt. Primary redirect graphs and
records preserved under analysis/redirect/round-001-a. Fresh configured-binary
decode matched all19393 primary instructions in615 bounded chunks. Branch/direct
local call starts, inline barriers, supported entry/mapper effects, room record
endpoints and141 constant firstframes checked independently. Static provenance only;
no new observed/natural reachability claim. Fixed code views leave lower slots
unknown rather than retaining decoder-default28/29 across mapper helpers.

Two important evidence corrections:
1. Proposed generic metasprite shelf contains **B07A..B16E area transfer code**.
   CA13 selects remembered mapper10/effective20/21 then CA18 jumpsB07A. New108-
   instruction coherent CC two-state initialization/display/countdown/equipment
   graph retypes241 proposed Data bytes to Code; inlineB088..B08C remains Data.
   Missing C6AF JMP D421 adds3 bytes. D421 clears eight camera bytes and returns;
   all other fixed callees already in reviewed graph. No frame record overlaps.
2. Audio probe retained eleven unused cinematic table constants. Those words are
   generic audio resource Data, not pointer tables. Only85C5/87C3 have supported
   audio8556 inline-dispatch role. Classification retained with narrower evidence.

The bounded residual follow-up accepted375 unmistakable generic noncode bytes,
including palette resources, uniformFF, repeated small-domain and signed-word
lookup shelves; see review appendix/results. No exact original schema invented.

Next **commentary** must complete readability across newly classified current
source. Prior commentary/source/game.asm and symbols.inc remain byte-identical to
round001; do not assume their old all-current gate covers this much larger map.
No annotation/source-label editorial gate is claimed by this phase.

Then redirect can research refreshed residuals using accepted boundaries:
- Mixed code/resource gaps: physical30091+58 (audio initializer-shaped body),
  36BA7+69,36C1A+134,37F5C+40,3911F+153,391D8+115,392B0+155,
  3D5EE+194,3DAC5+34 and3DF20+63. These have code-shaped slices but exact
  supported entry contracts/inline barriers versus embedded resources need local
  review. No actual-caller requirement is imposed; legal decoding alone is not
  classification. New current source should expose domains/ABI relationships.
- Bank7 tail FDC4+425/FF8D+131 contains mixed coordinate/descriptor/noisy bytes;
  inspect BD72 consumer and exact BDB7 record extent rather than blanketing tail.
- Small resource/table gaps such as35BA7+136,35E10+256,361A4+235,
  36770+220,3727E+112,38B27+232,3C33A+98,3E460+258,3E691+60,
  3E725+86 need practical consumer/neighbor review. They may support generic
  Data even when exact individual-table extent cannot be established. Do not
  preserve unknown merely because natural selectors or game meaning are absent.
- Mixed32-byte footer guards/vector copies7FF0,2BFF0,2FFF0,33FF0,37FF0,3BFF0
  and final3FFF7+19 need split boundary inference; a prefix contract must not
  veto a separately supported uniform middle. E000 trap/selector ambiguity stays
  localized. Original E000 exclusion remains intact.
These are concrete useful research questions, not a terminal exhaustion verdict.
This pass followed the proposed connected batch and the newly discovered B07A
continuation; further201-interval analysis belongs to refreshed commentary/redirect.

Gameplay: broad artificial progression and terminal2010.8.2/Joe screen already
preserved in round001-a; no unchanged experiments replayed. Reuse saved routes,
patches/snapshots and corrected room101 kind70 health39 contract. Natural final
shot/complete timed credits remain unverified; no compatibility/completion claim.

Toolkit issues: no new findings. Existing NES-DOC-001 and NES-OBS-002 remain in
analysis/toolkit-issues.md with workarounds. Configured release binary used;
no toolkit edits. No ROM edit/download, deletion/cleanup, archive or publication.
Disk4.0GiB free, analysis/scratch180KiB at handoff; cited evidence retained locally.
