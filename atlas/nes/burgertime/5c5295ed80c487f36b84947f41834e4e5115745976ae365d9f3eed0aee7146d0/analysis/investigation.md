# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 disassembly — easy-gain batch complete, ROM incomplete
Checkpoint: [pass-001-final verification](pass-001-final-verification.json),
[exact map](../maps/pass-001-final.json). Configured release binary used;
ROM SHA-256 unchanged. No earlier leads or handoff existed.

ROM classification: 99.94% (24577 / 24592 bytes classified; 15 bytes unknown).

| Physical section | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| Header 0x0000–0x000F | 0 | 16 | 16 | 0 |
| PRG0 0x0010–0x200F | 6755 | 1436 | 8191 | 1 |
| PRG1 0x2010–0x400F | 4541 | 3637 | 8178 | 14 |
| CHR0 0x4010–0x600F | 0 | 8192 | 8192 | 0 |

Evidence: [structural review](round01/structural-notes.md),
[5243 inspected instructions](round01/decoded-expanded.json),
[discovery with explicit views](round01/discovery-expanded.json),
[data annotation results](round01/data-annotations.json).
Seven main-state handlers; layout-command, motion, ingredient and audio-command
handler contracts justified connected Static code. Physical aliases preserve
NROM [0,1,0,1]; pointer words alone never counted as code evidence. Generic
non-code regions received bounded pattern/consumer review, with exact intervals
and explicit uncertainty where sub-table layout/meaning remains unresolved.
All 109 classified regions carry evidence (61 Code, 44 additional data regions
plus initial header/CHR/vector/main-table annotations; total count see state).
Detailed full current-source readability is next commentary phase's task.

Gameplay: [checklist and recipes](scenarios.json), [game notes](gameplay.md).
Natural startup/menu and initial gameplay screen visited. Artificial initialization
successfully reached middle layout3 and later layout6; representative movement,
pepper inputs, enemies and ingredient states exercised. GAME OVER rendered by
supported artificial JMP-entry contract. First completion-flag shortcut was
superseded after blank transition; exact patches/results retained, not retried
unchanged. Six-layout wrap and increasing total stage established statically;
ending/credits/boss existence is unproven, not fabricated. Layouts2/4/5 deferred
because shared consumers and representative milestones cover the structural batch.

Runtime: 3863 accepted unique physical instruction starts (8167 instruction bytes),
including artificial observations, separate from 11296 statically classified code
bytes. [Final contextual roots](round01/final-contexts.json) and
[full coverage](round01/final-coverage-all.json) persisted. No fetched root conflicts
with data or unknown. Trace aggregate_dropped=0; zero detail capacity deliberately
retained no event detail (65528479 dropped details). Trace-disabled settling intervals
are in scenarios and do not claim observed instructions from those intervals.

Residual claims, not terminal rejections:
- R01-001 [0x005D,0x005E), CPU $C04D: lone extra RTS beside scrolling routine.
  No independently supported alternate entry yet; valid decoding alone insufficient.
- R01-002 [0x3485,0x3488), CPU $F475: JMP $F493 between final coordinate stream
  and audio reset routine. Target contract known, trampoline role still needs
  discriminating entry/boundary reasoning. Do not turn missing caller into a
  universal prerequisite; independently supported structural contract could suffice.
- R01-003 [0x3609,0x3614), CPU $F5F9–$F603: LDA $05A2 / BEQ / JSR $F6BA /
  BCS / RTS priority alternate slice. Neighboring audio routine provides promising
  contract. Redirect should inspect incoming control flow and shared entry semantics.

These are first-pass deferred structural reviews, not failed experiments or dead
code claims. Their exact recipes are in leads.json. No terminal exhaustion claimed;
commentary should finish readability then redirect can resolve the compact batch.

Verification: baseline, pass-001 and fresh pass-001-final all byte-identical;
map.validate passed before final export. Final map hash
75d8105e19a68e00cad655743bcbc11b76ec970b0aad9b97fafe83352f60c04f.
No toolkit issues found; [persistent report](toolkit-issues.md), no blockers.
Handoff disk: 3.3 GiB free; scratch80KiB, round01 evidence36MiB. No cleanup made.

## Round 1 redirect — combined contract review
[Evidence and detailed results](redirect/round01-contract-review/findings.md),
[compact handoff](round01/redirect-handoff.md), [leads](leads.json).
R01-003 now ready with +11-byte Static code proposal based on independently
established music identity, replacement-mask semantics, preserved loader inputs,
exact branch convergence and stack balance. Natural reachability unresolved.
R01-004 ready for four live semantic metadata corrections. R01-001/002 remain
ambiguous after physical boundary and reference/word checks; localized reviews
compare actual role claims and competing interpretations. Exact complement remains
15 bytes, no accepted structural changes in redirect. Other concrete avenues
triaged; connected downstream audio already classified. Next disassembly reviews
combined ready batch. This is not terminal exhaustion. No toolkit issues found.

## Round 2 disassembly — ready batch verified

ROM classification: 99.98% (24588 / 24592 bytes classified; 4 bytes unknown).

R01-003 verified +11 Static code bytes from independent music-request, mask-helper,
loader convergence and stack-contract checks; no natural reachability claim.
R01-004 verified four semantic map corrections preserving roles/bounds.
[Review and competing interpretations](round02/review.md),
[exact counts](round02/counts.json), [handoff](round02/disassembly-handoff.md).
Per-bank classified/unknown: Header16/0, PRG0 8191/1, PRG1 8189/3, CHR0 8192/0.
Residuals R01-001 [005D,005E), R01-002 [3485,3488) remain materially ambiguous;
prior primary rejection review reused, no unsupported data fallback or opcode-only
classification. Connected audio already classified; gameplay evidence reused.
Authoritative checkpoint pass-002-reviewed freshly byte-identical; state/map saved
and validated. Superseded pass-002-final is unchanged pre-edit attempt retained.
Commentary editorial source/review preserved for follow-up; phase not run here.
No toolkit issues found; [report](toolkit-issues.md), no toolkit changes.

## Round 3 disassembly — explicit user fallback exception

ROM classification: 100.00% (24592 / 24592 bytes classified; 0 bytes unknown).

This assignment count comprises **24,588 evidence-backed bytes (99.98%) and four
user-authorized Manual fallback bytes**, not newly proven data. Fresh exact internal
complement was [{offset:93,length:1},{offset:13445,length:3}]. Only physical
[0x005D,0x005E) and [0x3485,0x3488) were annotated generic Data, unknown meaning,
Manual provenance through map.annotate under explicit <=10-byte batch fallback
authorization. Code/data ambiguity remains: isolated RTS versus non-code; possible
audio JMP wrapper versus unframed data. Historical evidence/rejections preserved;
no redirect research or new execution claims. This exception does not change the
ordinary evidence-backed classification contract for other bytes.

Per-bank assigned/unknown: Header16/0, PRG0 8192/0, PRG1 8192/0, CHR0 8192/0;
Manual fallback counts 0/1/3/0 respectively. All prior regions unchanged. State/map
saved, validated and equal; ROM and editorial source hashes unchanged. Fresh
checkpoint **pass-003-user-fallback** verified byte-identical;
[verification](pass-003-user-fallback-verification.json),
[fallback record](round03/fallback-record.json), [counts](round03/counts.json),
[handoff](round03/disassembly-handoff.md). Current-source commentary and final
strict editorial audit are pending the separate phase. No toolkit issues found;
[report](toolkit-issues.md). Disk3.3GiB free, scratch80KiB; no toolkit changes,
cleanup, archive or publication.
