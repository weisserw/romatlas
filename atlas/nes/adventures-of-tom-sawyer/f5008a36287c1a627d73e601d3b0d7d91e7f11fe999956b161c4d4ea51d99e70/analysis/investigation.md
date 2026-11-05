# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 disassembly — active findings

- Baseline verified byte-identical; mapper1 effective reset context [0,1,14,15]. Reset vector $FFD0, NMI $F89F, IRQ $B1D1 (IRQ lower-bank alias depends on context).
- Observed startup/menu roots plus conservative static continuation: initial 3,821 code bytes. `round-001/startup-menus-contexts.json` and `startup-menus-discovery.json`. Static and observed provenance separate.
- iNES header and complete declared CHR graphics classified; raw baseline not imported as classification.
- Stack helper CPU $E333 consumes JSR return address and selects a little-endian inline word using A*2, then JMPs to it. Title, opening and main mode tables bounded by immediately following first handler. `inline-dispatch.json`, `title-dispatch.json`, `main-dispatch.json`, `menu-dispatch.json` preserve inspection. Table code-versus-data is independently supported; pointer targets still require routine validation.
- Hidden menu gives exact stage RAM contract and bypasses slow opening; see `gameplay.md`. Direct counter-only stage1 shortcut was superseded by real debug-menu route, not rejected as a structural claim.
- Uniform zero PRG ranges >=64 bytes checked against code regions; generic Data after boundary/pattern inspection; no fill purpose invented. Seven 10-byte scene descriptors are copied from CPU $8000..$8009 to ZP $90..$99 by actual header-loading loops. `round-001/data-proposals.json` contains exact intervals and evidence.

## Round 001 final outcome

ROM classification: 78.61% (206092 / 262160 bytes classified; 56068 bytes unknown).

Verified `pass-001-final` byte-identical, 649 structurally evidenced regions; 12,637 Code, 62,383 Data, 131,072 Image. Seven lower scene halves classified generic Data on bounded descriptor/consumer/pattern review, not lack of execution; exact ranges in handoff and scene-data-proposals.json. All six stage mechanics visited through artificial debug-menu setup; ending attempted with malformed display, bosses still deferred pending initialization contract. No terminal exhaustion claim: safe call-return closure, upper-half scene handlers, ending CHR preparation and boss/checkpoint state remain concrete actionable redirect avenues. See [compact phase handoff](handoffs/round-001-disassembly.md), [exact per-bank counts](round-001/counts.json), [residual complement](round-001/unknown-ranges.json), [lead recipes](leads.json). No toolkit issues found; report preserved. Current-source readability is the next commentary phase.

## Round 001 redirect: contracts, scene resources and progression

Completed combined research in redirect/round-001-contracts; handoffs/round-001-redirect.md is the compact entry. All four inherited leads are ready, with attempt history and concrete verification recipes in leads.json. New fixed-resource proposal is also ready. Proposed28125 Code/17304 Data bytes remain unannotated. Decoder agreement, exact current unknown ledger, physical identities and manual conditional contracts are saved.

Corrected premises: first scene header words are resource pointers, not init entries; AC5F accumulates results, not CHR; B848 is cave player/bounds control, not spawn. Corrected boundaries: vertical event table endsE59D before8 code bytes; sound command table has7 entries, mask7 alone does not justify an eighth. These localized counterexamples do not reject supported neighboring records or non-code corpora.

Ending now executes complete fixed prefix and reaches properly composed vignette/results/title return. Representative flying boss initializes through ordinary event consumption, health10 and AI, then appears onscreen with source-backed position accounting; room/background redraw is still unresolved after camera jump. Detailed failed title/input and same-coordinate camera-window attempts remain retained. No terminal exhaustion assertion; further work should independently validate this coherent batch before inheriting accepted maps, then use localized saved graph barriers rather than unchanged gameplay repetition.


## Round 002 independent disassembly

ROM classification: 95.95% (251552 / 262160 bytes classified; 10608 bytes unknown).

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 5003 | 2590 | 7593 | 599 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 4699 | 2450 | 7149 | 1043 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 4831 | 2798 | 7629 | 563 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 4076 | 1882 | 5958 | 2234 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 4619 | 2091 | 6710 | 1482 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 3409 | 1908 | 5317 | 2875 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 2294 | 4945 | 7239 | 953 |
| 14 | 5027 | 2548 | 7575 | 617 |
| 15 | 6835 | 1115 | 7950 | 242 |

Independent review accepted all45429 proposed bytes plus31 connected Code bytes, each Static. Primary proof and localized frontier: [contract-review](round-002/contract-review.md). Exact counters/unknowns, configured decoder and validation are in round-002/. Original graph reproduced exactly; six scene encoded-origin checks and mapper/palette shared-record partition checks passed. E349 broad arbitrary-pointer rejection narrowed to four callsites with0400..04D7 RAM destinations; exposes ordinary continuation. Other dynamic mapper/nonordinary-stack/selector and sound-pointer barriers remain unresolved, not rejected code. No unchanged runtime experiments repeated; progression scenarios and prior negative evidence retained. Fresh pass-002-final is byte-identical; new source needs commentary. No toolkit issues found.

## Round002 redirect — local-contracts combined batch

Input pass-002-final unchanged. Bounded auxiliary/inline handler arrays, narrow nonlocal exits and CHR-only mapper-list contexts expose coherent downstream code; inspect manual-contracts.md and table-review.json in analysis/redirect/round-002-local-contracts. Standalone algorithms and exact duplicated helpers are structural code claims under explicit preconditions, without invented actual callers. Broader template/tile/command blocks received practical role inspection and generic fallback only where semantics remain uncertain. Reset clones, RTI, prevector bytes and vector-like words were separately evaluated. A whole-data AC98..AD43 exploratory candidate was corrected locally: exact91-byte accepted-code duplicate ACC0..AD1B split to Code. Bank9/13 auxiliary headers do not support pointer-array summaries. No unchanged gameplay experiments replayed; six-stage/ending/boss evidence retained.

Ready lead R002-LOCAL-CONTRACTS proposes4182 Code +6426 Data =10608 exact current unknown bytes; accepted zero. Configured decoder checked2319 contextual instructions, zero discrepancies. Current residual remains10608 across65; projected residual zero only if independently accepted. Ledger tracks every exact range and subclaim. This is a research-to-disassembly gate, not terminal completion/exhaustion. Rejection scopes and reproducible recipes retained; no toolkit issues, no source/map/ROM changes.


## Round 003 independent disassembly — incomplete

ROM classification: 99.98% (262103 / 262160 bytes classified; 57 bytes unknown).

Accepted4481 new Code and6070 new Data, all Static. Totals Code45274, Data85757, Image131072; header16 Data, every CHR bank8192 Image. Original graph/preparation/references/complement reproduced in redirect/round-003-reproduction;2319 proposed contextual instructions had zero configured-decoder discrepancies, and all final accepted groups including the added299 bytes were decoded again in round-003/rpc.jsonl. New state saved, map exported and validated; fresh pass-003-final verified byte-identical. Map SHA256d04812ebf8f9948b33d274383decf2b38a6e0b28f79c2ce46f77dde2be7cbff5; input ROM unchanged.

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 5038 | 3154 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 5238 | 2954 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 4866 | 3326 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 4751 | 3441 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 5337 | 2798 | 8135 | 57 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 4380 | 3812 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 3008 | 5184 | 8192 | 0 |
| 14 | 5601 | 2591 | 8192 | 0 |
| 15 | 7055 | 1137 | 8192 | 0 |

Independent complete data review discovered bank9 BCE7..BE4B embedded code-looking bytes inside proposed generic data. Split356 bytes:299 complete routine bytes BD20..BE4B accepted Code;57-byte prefix BCE7..BD20 remains Unknown because its outgoing destinations lie in present template bytes. Generic neighbors remain independently supported. This localized rejection avoids projected completion; see [review](round-003/contract-review.md), [fresh data hex](round-003/data-inspection.txt), [exact counts](round-003/counts.json), [residual](round-003/unknown-ranges.json), [validation](round-003/validation.json), [verification](pass-003-final-verification.json).

R002-LOCAL-CONTRACTS partially verified; R002-CONTRACT-FRONTIER superseded for classification by narrower residual question. New R003-BANK9-PARTIAL-SUFFIX ready for redirect. Investigate current structural role of partially overwritten/misaligned suffix versus resource payload using accepted neighboring object families and relocation boundaries; do not use forced execution as intended-role proof. Not terminal exhaustion. Current-source commentary and final editorial/fidelity audit remain pending; prior commentary history untouched.

Historical correction: previous redirect's 'four-boss' phrase is unsupported; actual retained evidence supports a representative flying boss, six stage mechanics and ending scenario, reused without new replay. A1DD recognizer arm is LDA #0, not RTS; inherited wording corrected in new review, old attempt preserved. Bank9 lower-slot template targets require bank8 physical context, not bank9; generic annotation does not inherit incorrect template sizes.

No toolkit issues found; [persistent report](toolkit-issues.md), zero blockers. No toolkit changes, archive execution, cleanup/deletion or publishing. Disk9.1GiB available; scratch4KiB. [Handoff](handoffs/round-003-disassembly.md).

## Round003 commentary dependency outcome

Full current-source readability and fresh byte-identical verification completed; classification unchanged at262103/262160. Source now explains accepted auxiliary/template/intermission/scroll/sound mechanics, bank9 complete routines and reset-copy contexts. Earlier broad unknown-continuation/table notes are superseded locally by accepted structure; original historical research remains preserved. Only structural frontier is R003-BANK9-PARTIAL-SUFFIX: physical13CF7..13D30, bank9 CPU BCE7..BD20, context[8,9,14,15]. Follow the independent [contract review](round-003/contract-review.md) and [residual evidence](round-003/residual-evidence.json); compare current neighboring template/relocation boundaries against coherent remnant contracts rather than forcing execution. No structural investigation or classification attempted in commentary; no terminal exhaustion claim. [Commentary handoff](handoffs/round-003-commentary.md).

## Round004 independent disassembly — structural classification complete; final gates pending

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Accepted57 Static Code bytes, physical81143..81200, CPU BCE7..BD20 context[8,9,14,15]. Independent full356-byte/156-instruction comparison, semantic family identity, bounded preceding template and independently accepted299-byte continuation support a preserved instruction fragment. No caller, runnable external joins, natural reachability, actual fetch or overwrite chronology inferred. BCE6/BCCF/BCAD remain Data; no discovery through them. All prior classifications preserved. Totals Code45331, Data85757, Image131072; header16 Data, each CHR bank8192 Image, all unknown zero.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 5038 | 3154 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 5238 | 2954 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 4866 | 3326 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 4751 | 3441 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 5394 | 2798 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 4380 | 3812 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 3008 | 5184 | 8192 | 0 |
| 14 | 5601 | 2591 | 8192 | 0 |
| 15 | 7055 | 1137 | 8192 | 0 |

[Independent review](round-004/contract-review.md), [primary reproduction/decoder checks](round-004/review.json), [RPC](round-004/rpc.jsonl), [exact counts](round-004/counts.json), [validation](round-004/validation.json). Every classified region carries evidence. Execution coverage unchanged; reviewed and reused existing six-stage, ending and representative boss scenario records without replay.

Fresh [pass-004-final verification](pass-004-final-verification.json) byte-identical; map/state SHA256 95df5dcd3c8697d06ef1d6a85f9b948bc5b3d833c79c372440fe573e02989516. ROM hash unchanged. Current state saved and portable map exported. New [source](../generated/pass-004-final/game.asm) needs full fresh commentary integration; previous commentary/source and investigation history preserved. Final editorial/fidelity review remains pending: zero Unknown establishes structural classification only, not overall completion or full hardware/game compatibility.

Lead R003-BANK9-PARTIAL-SUFFIX verified for fragment structure only. No new structural residuals or gameplay experiments. No toolkit issues found; [persistent report](toolkit-issues.md), zero blockers. No toolkit edits/archive execution/cleanup/deletion/publishing. Disk9.1GiB free; round004 owned scratch4KiB. [Handoff](handoffs/round-004-disassembly.md).
