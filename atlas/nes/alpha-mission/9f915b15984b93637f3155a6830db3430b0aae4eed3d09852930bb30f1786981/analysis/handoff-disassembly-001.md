# Round 001 disassembly handoff

Input: initialized empty `analysis/state.json`. One productive disassembly pass completed.
Output: **disassembly-001-final**.

**ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).**

12,664 Code;20,054 Data;66 Text;32,768 CHR Image;135 regions. All Static with
structural evidence, code views use physical 8KiB PRG slots `[0,1,2,3]` (CNROM).
Whole-image classification is subject to commentary/editorial evidence review;
full source readability and natural execution coverage are not certified.

- [Internal state](state.json), [portable map](../maps/current.json),
  [frozen checkpoint map](../maps/disassembly-001-final.json),
  [byte-identical verification](disassembly-001-final-verification.json).
- [Counts by bank](disassembly-001/bank-counts.md), [progress](progress.md),
  [investigation/lead outcomes](investigation.md), [lead recipes](leads.json).
- [Generated source](../generated/disassembly-001-final/game.asm).

Findings: game identified from title as Alpha Mission (SNK1987, Nintendo of America).
Validated vector roots and state RTS dispatcher. Expanded supported actor, boss,
weapon, armor, collision and item dispatch families through direct connected flow.
Reviewed data references/consumers and repeated record formats; classified generic
Data where non-code is clear but internal semantics remain unresolved. CHR partition
is hardware-defined. Raw baseline/export fallback never counted toward coverage.

Key evidence: [connected listing](disassembly-001/all-dispatch-listing.txt),
[item listing](disassembly-001/item-listing.txt),
[isolated helper contracts](disassembly-001/isolated-routines-discovery.json),
[PRG tail patterns](disassembly-001/data-pattern-review.json),
[residual data bytes/references](disassembly-001/residual-data-review.json).
Static helper routines `$80D2,$82B9,$924D/$9265,$9578` have coherent hardware/RAM/
return contracts; callers/reachability are unresolved. Review these and broad data
roles as part of full current-source commentary; address-based code labels remain.

Gameplay: [scenarios](scenarios.json) records exact inputs/releases, budgets,
patches, snapshots, context and outcomes. [Gameplay notes](gameplay.md) preserve
external sources and local confirmation status. Natural title/attract and actual
real-player Start route; artificial later areas6/12/13, rendered first-boss fight
(actor40,E8=1;1,747 unique opcode starts) and Octo armor activation (B0=1,DA=FE).
Trace aggregates have no aggregate drops; zero detail capacity intentionally drops
detail. Old `button/pressed` route names were corrected to unmodified attract-mode
evidence after diagnosing NES input schema; natural reachability claims were removed.

Unresolved concrete redirect question D001-L05: after initializing final-base
terrain at scroll0116, patching28=EF and29/2A=0115 triggers command55, E8=6/ED=1,
but six base-part objects are missing;240 further firing frames freeze scroll/state.
[Before](disassembly-001/base-trigger-ram.json),
[after](disassembly-001/base-battle-ram.json),
[recipe](disassembly-001/base-trigger-recipe.json).
Investigate `$8976` event-spawn/terrain initialization and `$B0DD-$B152` part contracts
before retrying. No ending/loop completion claimed. External references describe
looping areas with no conventional ending, still unconfirmed for this ROM.

Deferred: full source commentary/editorial audit; deeper semantic subdivision of
generic data; remaining per-area permutations; terminal progression after missing
spawn prerequisites are understood. Structural unknown complement is empty.

Toolkit: one confirmed documentation/API-validation issue **NES-001**, no blocker;
[toolkit report](toolkit-issues.md). Correct NES input is `buttons:[...]`, release
`buttons:[]`; generic documented `button/pressed` silently clears inputs. No toolkit
edits. All servers exited; use per-script `common.py`, explicit snapshot import,
and reassert intended inputs when resuming. Snapshots include local ROM state and
must remain local; only supported portable map is a sharing deliverable.

Final input SHA256 unchanged:
`9f915b15984b93637f3155a6830db3430b0aae4eed3d09852930bb30f1786981`.
Final map SHA256:
`71f91f57da5affd908db67474f695f700a6275f1130d1e261c14d63f50b9f736`.
No publishing/archive, no toolkit modification. Overall workflow remains incomplete
until subsequent commentary/redirect/editorial gates are satisfied.
