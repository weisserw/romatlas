# Disassembly progress

## Round 1 disassembly complete — incomplete ROM classification

ROM classification: 100.00% (49167 / 49168 bytes classified; 1 bytes unknown).

Rounding to 100.00% does not establish completion: one byte remains unknown. Code15189; generic/specific data17594; CHR image16384. Classification evidence is static; execution evidence is tracked separately and does not mark all classified code Observed.

| Bank | Code | Data/image | Unknown |
| --- | ---: | ---: | ---: |
| header | 0 | 16 | 0 |
| PRG 0 | 6761 | 1430 | 1 |
| PRG 1 | 6029 | 2163 | 0 |
| PRG 2 | 1358 | 6834 | 0 |
| PRG 3 | 1041 | 7151 | 0 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |

Configured `/root/romatlas/target/release/romatlas` used. Baseline was established once and verified. Final checkpoint `analysis/pass-003-verification.json` confirms byte-identical rebuild from `maps/pass-003.json`; current portable map `maps/current.json`, internal state `analysis/state.json`, source `generated/pass-003/game.asm`. Input SHA256 unchanged. Earlier pass-001 is a verified empty map produced after a rejected missing-id annotation; retained as historical evidence. Pass-002 contains initial vector code plus header/CHR; pass-003 final map.

Static evidence: validated vectors and fixed CNROM PRG [0,1,2,3]; 7046-instruction recursive graph reviewed, then consumer/record inspection, stage loader, bounded PPU helper contracts and 16-command sound dispatch closure. Exact structural rationale persisted in every current region annotation. See round01/candidate-disassembly.txt, remaining-code.txt, static-discovery.json, sound-discovery.json, data-evidence.json. No classification conflicts. Generic data inspection distinguishes low-valued records, PPU streams, repetitive sound commands and uniform tails; original generator/layout semantics were not required.

Gameplay: initial generic button requests had no effect and reached **attract** gameplay, not player gameplay (correction of prior live progress claim). Correct NES buttons-array route reached natural round1 READY. Source-informed RAM shortcuts visited/exercised stages15,34,36 (DoH) and ending narrative. Framebuffer, snapshots, exact input/patch budgets and observations are in scenarios.json and round01/batch-evidence.jsonl. Artificial state never establishes natural reachability. Combined exercise windows retained 1732 unique fetched starts, 1816 edges, no aggregate drops; initial attract window1186 starts. These are scenario-specific and overlap; do not sum as global execution coverage.

Remaining byte physical$009E (CPU$808E), value$60. No observed fetch, supported live pointer or branch reaches it. Adjacent reset routine ends an infinite loop at$009B and signature starts$009F. Unreferenced RTS alone cannot prove code; generic data lacks pattern/consumer evidence for this isolated byte. Redirect should investigate references/entry contracts or bounded unreachable alignment interpretation. Further automatic classification would weaken policy. This is an ordinary easy-gain handoff, not terminal exhaustion.

Consumed leads: none (initialized empty list). Produced L-R1-001 residual byte, L-R1-002 remaining gameplay/ending chain, L-R1-003 generic data subdivision/readability. See leads.json and investigation.md. Next: full current-source commentary, then redirect residual-byte review and any concrete progression questions.

Toolkit: **no toolkit issues found**. See toolkit-issues.md. API schema mistakes corrected locally; bounded timeouts behaved as expected. No toolkit edits. Disk8.6GiB free, scratch8KiB at handoff; no cleanup/deletion performed.

Compact handoff: `analysis/round01/disassembly-handoff.md`.

## Reuse / next actions

Load analysis/state.json into a new server. Snapshot import applies saved state directly (name/load is not required); reassert buttons arrays. Never import raw baseline as classifications. Stable physical residual [158,159). Current comment/label editorial quality is preliminary: per-region structural evidence exists, but full source behavior commentary remains a separate phase. Use generated/pass-003/game.asm and embedded source numeric branches to trace. Sound closure classified all dispatch handlers and PPU clear/life-erasure contracts statically; natural use remains unresolved.
