# Native port architecture

Status: initial source survey and plan pending. Replace this template with a short,
actionable source-informed design before substantial implementation. Do not wait
for exhaustive understanding or user approval. Read prompts/start.md for scope.

## Source survey

Identify representative startup, gameplay, rendering and sound paths; explain
major systems, shared state and update order. List important unknowns and the
reasonable implementation assumptions used to proceed.

## Modules and ownership

Describe proposed modules/files, their responsibilities and state owners, small
interfaces, dependencies and data flow. Separate game rules, ROM asset loading,
presentation, sound/sequence logic and frontend integration where appropriate.
Explain reuse of platform-specific library helpers and meaningful exceptions.
Choose boundaries for this game; no fixed module inventory or file-size quota.

## Algorithms and scheduling

Explain the core algorithms and their invariants, simulation/audio/presentation
ordering and how mutable state crosses boundaries. Keep source-specific reasoning
close to the relevant implementation; avoid a literal assembly-shaped architecture.

## Implementation order and decisions

List an actionable order for implementing the whole game and focused unit tests
of intended logic. Revise this plan when design decisions change, including brief
reasons. Check affected boundaries after substantial features; fix concrete design
problems without repeatedly auditing/redesigning the entire project. Schedule the
bounded smoke-test.md route before completion; the debug subprocess performs scoped fidelity QA after each milestone.

## Living milestone plan and coordinator state

Planning owns this design; read prompts/plan.md. Record game modes/transitions and
special concerns alongside the systems above. Replace the implementation order
with a finite series covering zero to whole-game completion.

For every milestone: stable ID, dependencies, source/ledger coverage, deliverables,
checks, applicable smoke steps, QA entry routes, implementation status, QA status
and cycle/report references. Record deferred scope explicitly. Track current stage,
cycle, selected milestone and resumption point. Preserve revision reasons and prior
cycle findings. Final QA covers the entire game in two full passes; intermediate
cycles cover the cumulative implemented scope in one pass.
