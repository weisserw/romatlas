# Disassembly progress

Round 001 disassembly pass finished. Structural classification is complete and the checkpoint rebuild is byte-identical. Overall workflow remains **incomplete pending current-source commentary and editorial audit**; this pass makes no full-game compatibility or complete natural-reachability claim.

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

Internal map: `analysis/state.json`; ROM-free portable map: `maps/current.json`. Frozen checkpoint: `maps/pass-001.json`; source: `generated/pass-001/game.asm`; verification: `analysis/pass-001-verification.json`. ROM SHA256 remains `a0c2f6de3927d0849e59e663e83bec37327ed76f55916f4d3775edeb054bf73f`; checkpoint map SHA256 `8a1f62f5895d14923a41b7a5777b3e895a5a9050cb27470760fc3e33750469a0`. `map.validate`, generation and byte-identical verification passed; generated diagnostics empty. Baseline preserved at `generated/baseline` / `maps/baseline.json` with separate verification.

## Physical counts

| Physical section | Code | Data | Text | Image | Unknown |
|---|---:|---:|---:|---:|---:|
| iNES header $0000-$000F | 0 | 16 | 0 | 0 | 0 |
| PRG 8KiB bank0 $0010-$200F | 7478 | 689 | 25 | 0 | 0 |
| PRG 8KiB bank1 $2010-$400F | 6270 | 561 | 1361 | 0 | 0 |
| PRG 8KiB bank2 $4010-$600F | 0 | 8192 | 0 | 0 | 0 |
| PRG 8KiB bank3 $6010-$800F | 0 | 8192 | 0 | 0 | 0 |
| CHR $8010-$1000F | 0 | 0 | 0 | 32768 | 0 |
| Total | 13748 | 17650 | 1386 | 32768 | 0 |

101 regions, all with structural evidence and Static provenance. 64 Code regions all have explicit CPU main views, effective PRG banks [0,1,2,3]. Map comments/labels are structural groundwork; full readability remains for commentary. Generic data remains intentionally generic where format/game meaning was not independently proved; those bytes are classified non-code, not unknown.

## Evidence and decisions

- Validated NMI/RESET/IRQ vectors at physical $800A-$800F target $8000/$8003/$8006; jump stubs lead to interrupt/reset routines. Static graph: 5299 official instructions,11017 bytes,no overlap; three indirect stops. `analysis/evidence/static-discovery.json`, `static-listing.json`.
- Consumer-supported dispatchers $97B2/$A049/$A727 and exact handler arrays establish 2232 more code bytes and50 data bytes. Connected handler graphs inspected for control/stack/RAM contracts, all official instructions, no overlaps or unresolved diagnostics. `dispatcher-discovery.json`, `dispatcher-new-listing.json`.
- Remaining499 bytes have independent coherent utility contracts (PPU shadows/registers, counted delay/fades, audio active state,12x8 grid predicates, alternate prefixes sharing existing tails); they were not classified from legal decoding alone. Detailed role/preconditions and natural-use uncertainty are in map evidence and `residual-listing.json`.
- CHR is parsed PPU pattern storage. NUL-terminated interface/credit text has text-renderer consumers. Resource region $3F71-$7526 contains repeated lookup/pointer/audio-stream records and known stage/password/audio consumers; exact global boundary is next uniform suffix. Generic resource data does not assert all internal layouts. Uniform $FF $7527-$8009 ends at independently fixed vectors. Small data intervals have per-slice references/boundary/rationale in `data-slices-inspection.json`.

## Gameplay and execution (separate metric)

Battleship, Mindscape1993, CNROM fixed PRG. First deployment, placement/rotation/confirmation, combat attack animation, CODE editor, rank8 deployment/mechanics and ending ship/fireworks visited. Full campaign natural completion is not claimed. See `analysis/scenarios.json`, `analysis/gameplay.md`, and durable replay `analysis/evidence/replay-round001.py`. Password digits injected into RAM $0661-$0664 then submitted through ordinary Start:9617 accepted rank7,battle0,absolute35;1992 accepted ending. Both are artificial-state observations. Exploratory cursor RAM84/85 edit happened after ACCEPT SETUP and showed no progression benefit; not reused as a shortcut.

`combined-coverage.json` contains2998 observed instruction starts /6161 instruction bytes from the gameplay/password session, mixing natural and explicitly artificial scenarios; all lie in classified Code. Separate clean startup session contains489 starts. These are not structural-classification counts. Combined trace retained no detail by design(capacity0), aggregate_dropped0, unique_contextual_edges3277; detail dropped1664669 is intentional, aggregates retained. Bounded successful execution and every input release are recorded. Initial failed probes used an unsupported trace enum and an insufficient explicit tick budget; corrected, no conclusions drawn from those failures.

No new classification leads remain. Commentary should review all current source and may refine generic resource schemas/natural uses without reopening sound structural evidence. `analysis/leads.json` retains verified/rejected lead outcomes; no failed lead needs unchanged replay.

Toolkit report: **no toolkit issues found**; see `analysis/toolkit-issues.md`. No toolkit edits, downloads of ROM/assets, archive, cleanup or publishing occurred. Disk at handoff:4.6GiB free; evidence54MiB,scratch140KiB before final notes. Input unchanged; all snapshots/captures/source remain local.

## Round 001 commentary outcome

Full current-source readability complete, separately from classification and final editorial acceptance. All 101 physical scaffold regions and 268 meaningful units reviewed. Added coherent contracts for reset/NMI/PPU/audio, placement/boards/turns/weapons, AI, password/menu/ending and data formats; consistent hardware/RAM/table/field symbols and descriptive branch destinations. Refined independently bounded resource structures while retaining honest generic payload notes. Original AI stack-unwind/boundary reads and ending-password adjacent-table reads are explained without modifying behavior. No structural map/state changes.

Fresh final commentary verify passed byte-identical; report commentary/verification.json and refreshed commentary/reading-after.asm. Source SHA256 1ebd56509e14fae8c1bc5b54cb3e80212f652370d8939519f3bcb469ab7ace27, symbols e18377e7928c130a0f360d427a0192c64644f3bbdd2b85b55aec2033f7d52921. Input and map hashes unchanged. See analysis/handoff-commentary-round-001.md, commentary/progress.md and uncertainties.md. Remaining readability gaps none; final editorial_audit pending for coordinator. No toolkit issues found; no toolkit edits, archive, publication, cleanup or downloads.

## Round 002 disassembly — supported music-boundary refinement

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

R001-music-consumer-boundaries verified by independent primary consumer/selector review and a separately implemented bounded decoder; original redirect evidence preserved. Six song pointers at physical2741..2753, six seven-byte sequences at2753..2795, six relative offsets at20856..20868, and six nonoverlapping stream bodies at20868..29991 now have specific Data annotations. Stream body ends are23550,25461,27339,27753,28020,29991 (exclusive). The decoder reads one additional ignored terminal delay byte: each next stream initial delay, finally first FF suffix byte29991. Storage annotations remain disjoint and suffix classification stays Data. Static provenance, effective fixed PRG banks[0,1,2,3]; natural use of all songs is not claimed. Original artifact's conservative “no complete pointer-table extent claim” is superseded for these six supported entries by finite selector domain + sequence indices + independent first-stream boundary, not pointer appearance alone.

Zero new classified bytes; totals unchanged: Code13748,Data17650,Text1386,Image32768,Unknown0. Per-bank classified/unknown: header16/0; PRG0 8192/0 (Code7478,Data689,Text25); PRG1 8192/0 (Code6270,Data561,Text1361); PRG2 8192/0 Data; PRG3 8192/0 Data; CHR32768/0 Image. Internal map now116 regions, all evidence-backed. Independent full partition and explicit kind check recorded in [counts](disassembly-round002/counts.json); [review](disassembly-round002/consumer-review.json), [recipe](disassembly-round002/review.py), [RPC record](disassembly-round002/rpc-log.json).

Saved analysis/state.json and exported maps/current.json; map.validate passed. Fresh pass-002 generated with no diagnostics and byte-identical verification: [report](pass-002-verification.json), [frozen map](../maps/pass-002.json), [source](../generated/pass-002/game.asm). Map SHA256 afe63dc6edc4bb336f186acec0de21bda32568c3c3ea32ec87f7c59fb479bad4. ROM SHA256 remains a0c2f6de3927d0849e59e663e83bec37327ed76f55916f4d3775edeb054bf73f. Baseline/pass-001 and all previous commentary/evidence retained.

Broad progression checklist reviewed and previous valid first/late-rank/ending evidence reused. No changed runtime question justified unchanged replay. No ready structural lead remains; exact unknown complement empty. Classification complete; overall workflow remains incomplete pending commentary carry for this refinement and final editorial/fidelity acceptance. Commentary source deliberately unchanged (SHA256 1ebd56509e14fae8c1bc5b54cb3e80212f652370d8939519f3bcb469ab7ace27). Next commentary worker should preserve existing prose/labels and incorporate the six-entry extent and ignored shared-boundary reads, then verify.

Toolkit: **no toolkit issues found**; [persistent report](toolkit-issues.md). No toolkit edits, cleanup/archive/publication/downloads. Disk4.6GiB free; analysis/scratch148KiB. Handoff: [round002](handoff-disassembly-round-002.md).


## Round 002 commentary and redirect outcomes

Separate commentary worker completed carry from pass-001 to pass-002, preserving
all prior commentary at analysis/round-001/commentary. Reviewed 116/116 regions and
300/300 units; no deferred readability range. Source and symbol hashes are in
commentary/verification.json. Fresh byte-identical rebuild and refreshed reading
copy passed. See handoff-commentary-round-002.md.

Separate redirect worker completed changed-dependency assessment from pass-002 and
that commentary handoff. R001 remains verified; no added/updated lead IDs, no ready
leads and no interrupted frontier. Exact Unknown-kind/gap complement is empty.
Primary findings and reduced-prospect dispositions are in
redirect/round-002-dependency-20261119/findings.md and handoff-redirect-round-002.md.
Reused unchanged sound prior findings; no empty third round is justified.

## Coordinator final acceptance

Final editorial review accepted current source and full inventory. Corrected the
round001 inventory source-path prefix before carry; round002 references resolve.
commentary.py audit passed: 116 reviewed regions, 300 reviewed units,
editorial_audit=true, byte_identical=true; reading-after.asm refreshed.
analysis/current-audit.json independently checks exact partition, annotation evidence,
fixed code-bank views, contained review units, map/store/frozen-checkpoint equality,
map/store validation and original ROM hash. No toolkit issues found.

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

Terminal checkpoint pass-002; whole-ROM structural classification and required
current-source editorial/fidelity gates complete. Semantic/art identities, static-only
natural callers and exhaustive gameplay remain explicit limits, not unknown ROM
classification. Final handoff: analysis/final-handoff.md. Archive options and their
inventory check are prepared without archive execution, cleanup or publication.
