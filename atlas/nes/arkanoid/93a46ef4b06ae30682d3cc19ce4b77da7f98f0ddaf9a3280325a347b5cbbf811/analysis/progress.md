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

## Round 1 commentary — current-source readability complete

Completed all pass-003 classified source in commentary/source/game.asm: 104 physical inventory entries,185 meaningful routine/control units,15,189 code bytes,17,594 data bytes,16,384 CHR bytes and an explained unknown physical $009E. Added functional algorithms/state changes, meaningful branch/routine names, shared RAM/object/hardware/sound symbols, scoped decimal aliases and concise data/format notes. CPU aliases remain distinct from physical storage labels. No pending readability ranges; final editorial acceptance remains separate and pending.

Fresh byte-identical verification passed with original SHA256 unchanged; final source SHA256 badcba5fdf8bdb771d927dd1ba95cd996e7357a19a3d13e4843b8e21793229b0. Refreshed commentary/reading-after.asm. Frozen map/state manifest hashes match current files. Primary evidence: commentary/review.json, commentary/verification.json, commentary/progress.md and commentary/uncertainties.md. Compact handoff: analysis/round01/commentary-handoff.md. Whole-ROM classification still lacks one byte; continue redirect from existing leads. No toolkit issues found; analysis/toolkit-issues.md. No toolkit edits or archive/publication/cleanup.

## Round 1 redirect — r01-20261106-a (research finished, handoff being finalized)

Input pass-003 and completed current-source commentary. Research only: map/state/input/source unchanged. Exact complement checked: classified49,167/49,168, residual physical[$009E,$009F), CPU$808E value$60, PRG[0,1,2,3]. New complete accepted-instruction/reference, pointer and sound-dispatch-domain review retains material code/data ambiguity; no new classification proposed. Primary records: analysis/redirect/r01-20261106-a/residual-evidence.json, reset-primary.txt, accepted-instructions.json, dispatch-domain.json, rejection-review.md.

Ready L-R1-003 improvement: type3 CapsuleLaser and type5 CapsuleExpand source labels are reversed by direct request/consumer contracts and observed artificial effects; type3 expands, type5 enables laser. Ending-return CHR prose also incorrectly says first bank: LDY1 selects table value1. Independent review and consistent commentary corrections required, then fresh fidelity/readability checks. Exact proof/recipe: analysis/redirect/r01-20261106-a/findings.md and power-contract-primary.txt.

Progression extended through natural active single-player launch/movement; all seven artificial pickups; actual laser firing; escape gate stage15->16 and active stage16; artificial player2 selection/initialization; boss hit-count-triggered erasure->ending story->ending-return->title. Natural boss defeat/collection/alternation remain unproved. Durable exact request/replies: runtime.jsonl, followup.jsonl, finish.jsonl, stage-complete.jsonl, two-player.jsonl in attempt directory; scripts and sparse RAM/state summaries identify actual outcomes, not merely patches. Initial traced180frame timeout preserved; revised <=40frame untraced travel and <=5frame trace windows worked. Final natural-two-player/port2 check is finishing. No toolkit issues found; no toolkit edits, archive, publish or deletion/cleanup.

Redirect handoff finalized: L-R1-001 blocked on localized structural ambiguity (not live reachability requirement); L-R1-002 representative behavior verified with explicit natural/artificial limits; L-R1-003 ready supported naming/CHR-prose corrections. Fresh natural menu start selects two-player mode1; snapshot labeled ready remains state4 intro and is not counted READY. Port2 movement from initialized active player2 moved paddleX88->103. Full connected boss/ending chain returns to title state1. No ongoing jobs/research frontier. Ready work proceeds to independent disassembly review then commentary; no terminal exhaustion assertion while that actionable correction remains.

Final identity artifact analysis/redirect/r01-20261106-a/identity-final.json confirms ROM,map,state,maintained source unchanged. Counts/code/data/CHR per bank identical to pass-003; proposed/accepted new bytes0/0, exact residual1. Prior fresh commentary verification remains valid for unchanged source; corrections will require new verification/readability handoff. Disk8.5GiB free; research64MiB, scratch8KiB. All retained local; no archive/publication/deletion/cleanup. Compact handoff: analysis/round01/redirect-handoff.md.

## Round 2 disassembly — pass-004

Input checkpoint pass-003; output fresh verified checkpoint pass-004. One bounded disassembly pass complete. L-R1-003 independently verified; commentary correction is pending. No classification annotations changed. ROM, internal state, portable map and maintained commentary source hashes are unchanged (disassembly-review.json). No gameplay replay: round1 representative progression remains valid with its natural/artificial limits.

ROM classification: 100.00% (49167 / 49168 bytes classified; 1 bytes unknown).

This rounded percentage is incomplete. Unique evidence-backed physical coverage was recomputed from internal regions (counts.json): header16/0; PRG0 8191/1; PRG1/2/3 each8192/0; CHR0/1 each8192/0 classified/unknown. Code15189,data17594,CHR16384 remain unchanged. Proposed/accepted classification gain0/0.

Accepted exact claim: type3 dispatch at physical$02C8 branches CPU$82FF (physical$030F), which requests form2 via$0129. ExpandPaddle tests bit2 and finishes with current form2; its position/spacing changes independently establish expansion. Type5 dispatch at physical$02D0 branches CPU$8319 (physical$0329), requesting form4; AnimateLaserPaddle tests bit4 and its terminator writes current4 and LaserEnabled1. Saved artificial capsule3 and laser5/firing observations corroborate this contract (reused-runtime-observations.json); natural pickup reachability is not inferred. Source CapsuleLaser at$82FF and CapsuleExpand at$8319 are reversed. Swap those labels and every reference, update effect-order prose (currently slow,catch,laser,multiball,expand...), and preserve correct ExpandPaddle/AnimateLaserPaddle consumer names. Original bytes must remain identical.

Ending correction independently accepted: immutable bytes physical$1CFF are LDY1/LDA$9FCB,Y/STA$9FCB,Y; physical$1FDB table[0,1] selects value1. Mapper3 bus-conflict-safe ROM-address write selects second 8KiB CHR bank. DrawEndingReturnInSteps first-CHR-set prose must become second-bank prose. This is a behavior explanation correction, no new classification. Maintained source left untouched for commentary.

Residual L-R1-001 reviewed against primary reset/signature artifacts and scoped rejection-review.md: physical[$009E,$009F), CPU$808E, fixed PRG[0,1,2,3],$60 remains ambiguous orphan RTS versus isolated non-code separator. No new discriminating structure; legal decoding alone cannot support code, and exclusion from bounded signature does not establish non-code. No universal caller requirement or dead-code assertion. Sound-dispatch finite extent and accepted uncalled helper comparisons remain sound. Do not repeat unchanged scans/arbitrary RTS execution.

Verification: map.load -> map.validate -> map.save -> map.export succeeded (disassembly-rpc.json). analysis/pass-004-verification.json confirms byte-identical full image against immutable ROM; maps/pass-004.json and generated/pass-004 are fresh structural checkpoint outputs. This does not verify future commentary changes. Previous commentary verification applies only to the unchanged maintained source; commentary worker must correct, review full current source and freshly verify it. Then redirect reviews changed claims/residual. No terminal exhaustion assertion before those ordered phases.

No toolkit issues found; analysis/toolkit-issues.md preserved. Disk8.5GiB free; scratch8KiB at batch start/handoff. No toolkit edits, archive, publication, deletion or cleanup. Main evidence: disassembly-review.py/json, reused-runtime-observations.json, counts.json, plus round1 power-contract-primary.txt/ending-chr-primary.json and residual rejection-review.md.

## Round 2 redirect — r02-20261106-a

Completed current-source correction/dependency review and terminal residual rejection review. No classification proposal or ready lead remains. Classification49,167/49,168; residual physical[$009E,$009F), CPU$808E=$60, fixedPRG[0,1,2,3]; orphan RTS versus isolated separator remains materially ambiguous. Exact complement/identities and evidence-scoped stopping reasoning: analysis/redirect/r02-20261106-a/{identity-and-residual.json,unknown-ledger.json,rejection-review.md,findings.md}. No new gameplay replay, whole scan, source/map/state edits or toolkit changes. Current-source readability complete; byte-identical commentary verification remains valid; final editorial audit pending. Coordinator must inspect rejection review before terminal acceptance. No toolkit issues found; analysis/toolkit-issues.md. Free disk 8.49GiB; no deletion/cleanup.

## Final editorial acceptance — pass-004

Final editorial review completed: all104 physical scaffold entries and185 meaningful units reviewed; no pending readability ranges. Reused valid source-order and round2 dependency reviews, checked complete inventory consistency and targeted remaining naming/CHR/paddle/unknown dependencies. No source defect or new promising structural issue found; corrected stale uncertainty-record lead status only. Official commentary.py audit passed, including fresh byte-identical49,168byte rebuild and refreshed reading-after.asm. Source SHA256 ec0812116fc091be71e54fe81f5b7952e28c9c77cf6ebe404f3366bdcbcea9aa unchanged. See commentary/editorial-audit.md and analysis/final-editorial-handoff.md (paths relative to workspace). Classification remains incomplete49,167/49,168, sole unknown physical$009E; coordinator accepted round2 rejection reasoning separately. No toolkit issues found; analysis/toolkit-issues.md. No archive/publication/deletion/cleanup or source/map/state/toolkit edits.

## Coordinator terminal handoff — orchestration complete, ROM incomplete

After separate sequential round01 and round02 phase workers, coordinator accepted the current-source rejection review against decisive primary artifacts; see coordinator.md. Final editorial acceptance then passed official audit with104 reviewed scaffold entries/185 units, fresh byte-identical rebuild and refreshed reading copy. Configured-binary store and portable map validations independently passed. Final checkpoint pass-004; ROM classification100.00% (49167 /49168 bytes classified;1 byte unknown). Rounded metric is incomplete: physical[158,159), CPU$808E=$60, PRG[0,1,2,3]. No ready leads or interrupted productive work remain. Unresolved byte lead stays resumable for new structural evidence/corrected inference; it does not block this justified terminal-incomplete orchestration result.

Terminal handoff: terminal-handoff.md; completion requirements audit: completion-audit.md; final machine checks: coordinator-verification.json. Archive script document-selection contract inspected; ../archive-options.json selects every additional final Markdown/history, with no disposable scratch Markdown omitted. archive-document-inventory.json accounts for all paths. No archive execution, toolkit edits, publication, cleanup or deletion. No toolkit issues found. Classification is not declared complete.
