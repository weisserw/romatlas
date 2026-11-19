# Coordinator terminal handoff — justified incomplete

The coordinator workflow has reached its documented terminal handoff. **ROM classification remains incomplete: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).** This is a reasoned investigation stop, not blocked work, whole-ROM completion, natural execution coverage or full game/hardware compatibility.

Final checkpoint: **round-008-disassembly-final**. Current map, internal state and checkpoint map share SHA256 `f8ffa36af6bf8a03285d7c52e8eac89a23f7db1099bddde94667325e559df8dc`. Counts are Code 57734, Data 73338 (including the 16-byte header), CHR Image 131072. Static provenance covers 247704 bytes and Observed provenance 14440; every classified region retains structural evidence. [Independent classification verification](final-classification-verification.json).

Exact unresolved physical intervals, including the ROM header in offsets:

| Offset | Length | Remaining ambiguity |
|---:|---:|---|
|1222|1|Sound separator versus stored BRK fragment|
|13978|10|Opaque sound-bank data versus stored instruction fragment|
|66280|3|Detached bank8 active transfer versus opaque datum|
|91735|1|Extra bank11 empty return versus separator/remnant|
|117378|1|Extra fixed-bank empty return versus separator/remnant|

The final [redirect handoff](redirect/round-008-r1/handoff.md) and [rejection review](redirect/round-008-r1/rejection-review.md) evaluate the actual claims, decisive primary evidence, negative-result scope, combined constraints and competing interpretations. The coordinator inspected those primary bytes, native decodes and current-source boundaries, checked the bank11 branch to the first RTS and the independent accepted return-table analogue, and recorded [acceptance reasoning](round-008-coordinator-terminal-review.json). Failed safe/native execution claims were not promoted into universal caller, runtime, complete-routine or original-generator prerequisites. Generic Data was considered independently of semantics. No useful revised inference/test, ready lead or productive interrupted frontier remains. Future evidence can reopen exact claims through the retained [stable lead histories](leads.json); unchanged replay is not the next step.

Separate sequential workers completed disassembly, current-source commentary and redirect through ROUND008. [Progress](progress.md) records input/output checkpoints, verification, lead outcomes and phase handoffs; prior plans, attempts, source reviews and whole commentary directories remain local and preserved. The operationally premature ROUND006 unsuffixed no-gain checkpoint is retained as history; ROUND006's accepted checkpoint has the `-accepted` suffix. The final current checkpoint is unambiguous.

Current-source readability and final editorial acceptance both pass: **2387 scaffold records and 3071 meaningful units reviewed, zero pending, all 57734 Code bytes covered**. [Final editorial handoff](final-editorial-handoff.md) and [qualitative acceptance](../commentary/editorial-audit.md) explain the review and targeted timer/index wording corrections. Sound prior reviews were reused. Unknown ranges retain local mechanical/boundary notes and honest uncertainty. Fresh supported [audit](../commentary/audit.json) reports `ok=true`, `byte_identical=true`, `editorial_audit=true`; [verification](../commentary/verification.json) matches the current source and the [reading copy](../commentary/reading-after.asm) is refreshed.

Final source SHA256: game.asm `716fd5416f8d1c04ba69b5ff34e5b4b3fb4a9735641ea1e0992eb94186eac9c8`; symbols.inc `5742d0e3704d761dc24874bd3e175cf00ff21222aa8cfa2762f596d39a931fe1`. Source, assets, traces, snapshots and internal state stay local.

Broad best-effort progression includes 23 saved scenarios, representative mechanics across seven stages, later Factory/helicopter, terminal desk and actual White House credits. [Final gameplay review](redirect/round-008-r1/avenue-assessment.json) and [credits adjudication](round-002-disassembly/credits-adjudication.md) retain limits: natural boss victory and full credits are not claimed. Earlier failed shortcuts remain history; no useful untried shortcut or residual-specific runtime discriminator survived review.

No new toolkit issues were found. Existing NES-001 documentation/API mismatch and NES-002 indexed-RAM analysis missing feature have bounded workarounds and are nonblocking; [local report](toolkit-issues.md). The configured release binary and input ROM remain unchanged. No toolkit edits, archive execution, cleanup/deletion or publication occurred.

Archive preparation only: [archive-options.json](../archive-options.json) explicitly selects additional final Markdown and omits duplicate scratch instruction templates. [Document-selection verification](final-archive-selection-verification.json) checks the actual `scripts/archive.py` contract and accounts for every unrecognized analysis/commentary Markdown path, including this handoff and final rejection/editorial conclusions. [Completion audit](coordinator-completion-audit.json) records requirement-by-requirement evidence. No archive invocation is authorized or performed here.
