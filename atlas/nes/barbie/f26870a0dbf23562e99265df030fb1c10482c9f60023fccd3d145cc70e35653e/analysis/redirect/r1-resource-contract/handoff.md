# Redirect R1 → coordinator / separate disassembly

**Ready combined batch; ROM remains incomplete.** Input checkpoint `pass-001-final`,
current-source commentary gate `analysis/commentary-r1/handoff.md`. This worker ran
redirect research only. No ROM, map, state, source, toolkit, archive or publishing
changes. All probe servers have exited.

Current accepted classification stays **186253 / 262160 (71.05%)**, with **75907
unknown**: Code18465 / Data36606 / Text110 / Image131072. Current source SHA256
`3bbab1d7b02c64a748a3db3c801698ff385609d0513e1767270da8dddf8cfc4f` unchanged.
[Final checks](final-checks.json) independently confirm input/map/state/source hashes
unchanged and preserve per-PRG-bank counts. [Exact unknown complement](unknown-ledger.json)
has277 bank-split ranges with neighbors, effective contexts and proposal dispositions.

[Final manifest](proposals.json) contains **34397 currently unknown bytes proposed**,
**0 accepted**, plus **9472 Image→Data role corrections**. All intervals there are
**end-exclusive**. No manifest-internal kind conflict, existing Code/Data conflict,
or instruction-boundary conflict. If every proposal were independently accepted,
remaining unknown would be41510; that is a forecast, not a classification result.
Provisional [initial manifest](proposals-initial.json) is retained as method history.

Ready IDs are **R1-001…R1-006**, with stable histories, physical ranges, contexts,
source/evidence links, explicit verification recipes and expected gains in
[analysis/leads.json](../../leads.json).

| Lead | Supported batch | Decisive artifacts |
|---|---|---|
| R1-001 | Twelve world containers, physical[014016,017d58) and[018016,019a51);22397 new Data bytes. Five plane strides, palette, encoded sixteen-column pages, eleven-byte metatile records and final order section. Every end independently matches next header or fill. | [structures](world-structures.json), [audit](world-audit.json), [parser](world.py) |
| R1-002 | Goal/checkpoint/reload scripts, synthetic returns, movement/load wrappers, stage-result and terminal gates. Five final checkpoint thresholds1..5=[3,3,2,3,1] and actual game transition chain tested. | [goal](goal.json), [pending](pending.json), [boss scenarios](boss-scenarios.json), [terminal steps](ending-terminal-steps.json) |
| R1-003 | Exact83-byte CHR[02514f,0251a2)→RAM0200 and414-byte CHR[026b2c,026cca)→RAM0200 copies, physical PPU reads and subsequent RAM fetches; scene callbacks installed by that payload. | [first match](comparison.json), [second match](world-comparison.json), [copy trace](copy-trace.json), [RAM fetches](payload-context.json) |
| R1-004 | CHR resource/fill Data: full bank5[025010,026010), bank6 tail[026b10,027010), full bank7[027010,028010). Residual Image121600 has independent coherent tile-content evidence. | [directories](directories.json), [nametable streams](nametable-streams.json), [all32-bank contact](chr-contact.png), [decoder](chr-contact.py) |
| R1-005 | Exact18-scene fixed directory,8-byte scene descriptors, missing scene1 NMI and scene2 initializer. Scene2 has six8-byte inline spawns fitting nine clear dynamic slots. | [scene contracts](scene-descriptors.json), [scene1](scene1-nmi.json), [scene2](scene2-init.json) |
| R1-006 | Connected graph from all five installed object tables: **9731 new candidate Code bytes**,4246 unique starts/110 ranges, plus bounded operand/response/stream Data. | [v6 audit](contract-audit-v6.json), [walk](contract-walk-v6.py), [new listing](contract-new-listing-v6.txt), [proofs](contract-proofs-v6.json), [summaries](contract-overrides-v6.json), [response resources](response-resources.json), [49 stream pages](response-domain.json) |

The final object batch continues far beyond its first entries. Toolkit discovery
stops every JSR/indexed store on mapper1; the first1065-byte proposal was therefore
partial. The same redirect continued with reviewed helper restoration, RAM address
bounds, known branch flags, collision operand/hit/miss contracts, explicit slot0
motion spawn, finite interaction/response pointer targets and wrapper bank changes.
V1–V6 evidence is retained. V6's sole barrier is the already supported four-byte
slot selector table; [claim-specific audit](selector-barrier-audit.md) rechecks the
original consumer/branch evidence and excludes its unsupported negative-frame
fallthrough. No unexplored supported caller/indirect continuation remains in this
bounded object graph.

Important review boundaries:

- Resource copies are **endpoint-inclusive** internally: directory end-start+1
  bytes copied. The extra byte can be the next payload's opcode or fill; copying
  does not prove its execution. Container partitions/manifest are half-open.
- Use Data resource containers for CHR uploads; uploaded execution views are
  documented unsupported. RAM fetches prove RAM execution, not immutable-source
  Code provenance. The exact source match is separate evidence.
- Bank7 ten palette/command streams each output1024 bytes and end at the next
  resource. Nine starts come from explicit caller arguments;0475 is independently
  constrained between0353 and04f8 with the same complete schema/palette/output.
  Commands positive=repeat,80..bf=literal,c0..ff=incrementing run,00=end.
- World resources4/8 have final page-order values one past their column count.
  This rejects a universal selector-in-bounds claim, not their encoded non-code
  extent. Generic Data remains sufficient for mechanically bounded metadata.
- Do not use original pointer-range-only object scan: it swallowed$60$a5/$bd$cb
  code as plausible pointers. Bounded tables and v6 graph exclude those roots.
  Slot count16 is not the handler selector domain.
- The response engine reuses low bits of `ObjectAnimationMask`; no new global
  field meaning is claimed. Five matrices are3x4/3x5, transitions closed within
  columns, and initial low3 states for the six selected streamed records are0..2.

Broad progression: all five final-checkpoint environments initialized, loading
cleared, Right/A/B15 frames exercised, and captures/snapshots saved. From the
jukebox final encounter, an explicit artificial JSR to supported goal helpereb48
ran fade/script;180 frames reached its music-zero wait. Setting only0488=0 advanced
to scene5 result. Supported activity overflow advanced to scene6 exterior; its
exact position98/activity-overflow gate advanced to scene11 ballroom, then A3 and
release30 exercised presentation. [Final image](ending-terminal.png),
[snapshot](ending-terminal.snapshot), scripts and all mutations are retained;
[scenarios](../../scenarios.json) updated. This is a source-backed artificial
terminal presentation route, not natural boss defeat, ordinary completion or a
full credits claim. The failed scene6 Start-only route is preserved and explained
by mode bit6.

Remaining frontier is nonterminal. World one-past selection semantics, original
unused directory selector domains and ordinary ending/music timing remain uncertain;
they do not change the supported structural extents. Negative PlayerAnimationFrame
validity is not globally proved; deliberately executing that table edge would test
CPU mechanics rather than settle intended structural role. Exhaustive actor-state
or gameplay permutations are not justified by current findings. Other residual
PRG ranges retain exact unknown dispositions and should be revisited against the
newly accepted source/references after independent disassembly and full commentary.
No claim of whole-ROM research exhaustion is made; no empty cycle is requested.

Toolkit report: [persistent issues](../../toolkit-issues.md) contains one suspected
bug NES-001 and two documented missing features NES-002/003; **no blocker**. No
confirmed new bug, toolkit edit or replacement binary. Disk at handoff:6.3GiB free,
attempt80MiB, scratch8KiB; no evidence deleted.

Next action: separate disassembly independently reviews/annotates the coherent
manifest, validates/rebuilds/checkpoints, then completes full current-source
commentary and redirect. Final editorial/archive audit remains coordinator-only.
