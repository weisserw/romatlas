# Round028 redirect: scoped terminal rejection review

Input: round-028-disassembly, current-source commentary complete. Exact physical
complement independently recomputed from state: 65533 classified / 65536; three
one-byte Unknowns. Proposed 0, accepted 0. [Fresh audit](checkpoint-audit.json)
records protected/state/map/source hashes and per-bank counts; no map/state/source,
ROM, toolkit, runtime or classification changes. This is a terminal-incomplete
recommendation for coordinator inspection, not final acceptance.

Primary evidence retrieved directly: [R27 assessment](../R27-20261027-origin-and-epilogue-v1/assessment.md),
[R27 physical source excerpts](../R27-20261027-origin-and-epilogue-v1/source-excerpts.txt),
[R27 analog/stack audit](../R27-20261027-origin-and-epilogue-v1/primary-audit.json),
[R26 raw-word audit](../R26-20261027-dispatch-contracts-v1/primary-audit.json),
and [current authoritative excerpts](current-source-excerpts.txt). Prior rejection
labels alone were not treated as evidence. No unchanged experiment was replayed.

| Exact claim/context | Decisive local constraints | Strongest alternative and disposition |
|---|---|---|
| Physical 2f09, length1, fixed bank0 CPU2f09: C9 as detached epilogue after JP3199 at2f06 | Preceding deactivation path tail-enters DeactivateCurrentActor; its own RET consumes the inherited caller return. Next independent routine starts2f0a. There is no fallthrough from the JP or return to2f09. R26 finds no encoded target word2f09. | Redundant original-source epilogue versus stored C9 separating routines. Both satisfy exact neighbors and stack mechanics. Unknown. |
| Physical34af, length1, fixed bank0 CPU34af: C9 as detached epilogue after JR34a4 at34ad | HideCurrentActorHorizontally loads C=e0 then JR StoreCurrentActorScreenX (not JP); shared store tail RET returns directly to original caller. Following routine begins34b0. No encoded target word34af. | Redundant epilogue versus stored C9. Supported shared-tail precondition establishes the preceding routine, but does not assign this byte to it. Unknown. |
| Physicalad6a, length1, bank2 CPU6d6a: C9 after JP6c10 atCPU6d67 | Eraser tail-enters RenderCurrentEndingCreditBlock; renderer returns to inherited caller, not6d6a. Accepted ending state9 word targets6d6b, the independent following handler. Conditional phase36/index8 erase contract establishes the renderer path, without crossing6d6a. No encoded target word6d6a. | Redundant eraser epilogue versus stored C9 before next handler. No independent bank2 entry/layout relationship distinguishes them. Unknown. |

The no-word search excludes ordinary literal encoded-pointer references only; it
neither excludes computed/RAM entries nor proves unused/dead/data. Tail control
and return-stack analysis excludes *fallthrough/return from these preceding tails*
only. Neither negative is generalized to all possible code roles. Boundaries are
exact; structural code-versus-data ownership remains the ambiguity.

Independent constraints considered together: physical C9 bytes, accepted adjacent
instruction boundaries, unconditional transfers, complete downstream return-stack
contracts, independent following entries, accepted dispatcher origins/domains,
and the three accepted JP/JR+RET analogs. R27 checked every such accepted adjacent
analog:30d6 has a conditional branch,5934 has state3 directory entry58b7,856b has
terminator branch. Those examples support separately entered returns, not a
compiler/layout rule for detached epilogues. Repeated tail+C9 appearance alone is
one correlated layout observation. No supported original generation convention
or semantic ownership follows. A caller/fetch is not required universally: an
independent layout/stack/consumer contract could suffice, but none here singles
out the byte. Arbitrarily executing RET would only demonstrate opcode mechanics.

This applies the same standard that accepted0728 and3420: those have independently
anchored directory/origin selection plus preserved outer stack and exact endpoint.
The residual three have the endpoint/opcode possibility without that independent
ownership relationship. Generic Data remains equally unsupported: each is a lone
instruction-looking byte, without uniform/repeated non-code structure or consumer
extent enclosing it. No specific data role emerged from neighboring consumers,
labels, raw references or prior bounded format inspection. Missing semantics is
not the reason to retain Unknown; both code and stored-byte interpretations survive.

No useful revised inference/test survives: the narrower single-byte claims are
already the minimum intervals; the template comparison was actually performed;
current accepted-origin corrections terminate at0728/3420, not these residuals;
ending renderer and state9 contracts select around6d6a. No new computed entry,
encoded origin or independently justified layout premise is available. Repeating
literal searches, arbitrary RAM/RET calls, broad gameplay sweeps or inventing a
source generator would not discriminate ownership. Reopen only with such concrete
independent evidence or a justified corrected inference; this is not an irrevocable
rejection of code. Connected continuations and other source gaps already reviewed
are classified; none supplies a new residual-linked recipe.

Broad best-effort gameplay is separately evidenced by scenarios.json round026:
initialized stage1 field, stage2 cavern, stage3 spikes and stage4 masonry/checkpoint
with bounded movement/whip/jump/rope inputs, then actual ending scene and visible
STAFF credits. Primary results: analysis/round-026/safe-gameplay-results.json,
visit-stage2-nested0-results.json, visit-stage3-nested0-results.json,
visit-stage4-nested14-results.json, ending-results.json and credits-results.json.
These use documented synchronized RAM shortcuts, dependent initialization and
accepted fetches; they are artificial visits, not natural progression. Stage4
nested14 was a practical later shortcut and did not establish a boss arena.
Boss encounter remains unvisited without a validated arena contract; final held-A
terminal phase remains unvisited behind later timed credit/final-actor gates.
Representative credits already visited; no new residual connection makes repeating
or extending that route a discriminating structural test. No exhaustive gameplay,
all-boss, natural ending or full hardware/fidelity coverage is claimed. Durable
credits snapshot and recipes remain preserved for future gameplay work.

Current readability remains complete (2807 records/3007 units,6045 references,
zero pending/deferred). Existing map validation/conflicts pass and configured-release
checkpoint/independent RGBDS rebuild are byte-identical65536 bytes, as linked in
checkpoint audit; this research-only pass preserves their exact protected inputs.
Existing confirmed T001 only; zero new toolkit issues, no blocker; see
analysis/toolkit-issues.md. Disk14GiB free; no bulk scratch or cleanup needed.

Primary audit index (current authoritative source line spans): bank_000.asm
6948–6970 covers2f06 tail/2f09 gap/2f0a next entry;7460–7474 covers
DeactivateCurrentActor through shared-tail RET. bank_000.asm8048–8080 covers
StoreCurrentActorScreenX RET,34adJR,34af gap and34b0 next entry.
bank_002.asm4656–4687 covers eraserJP/6d6a gap/6d6b following handler;
4532–4569 covers renderer selection, ordinary helper CALLs and counted-loop RET.
All spans are copied with exact line numbers into current-source-excerpts.txt.
