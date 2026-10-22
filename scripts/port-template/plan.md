# Stage 2: behavior planning

You are the behavior-planning subagent. Read workflow.md, prompts/common.md,
architecture.md, the skeleton's identifiers/debug schema and the actual source.
Preserve the skeleton; do not implement behavior or execute gameplay QA.

Identify all source-game behavior. Survey startup/reset, asset preparation and
formats, display/layout/palettes/effects, sound effects/music sequencing, input,
actors/AI/collision, gameplay rules/timers/scoring, scenes and progression,
menus/title screens, saves and ending/terminal states. Include applicable special
hardware behavior and cross-system scheduling. Inspect dispatch tables, handlers,
callers and data consumers so the inventory covers later areas and alternate
modes as well as the initial route. These are categories to investigate, not
claims that every game contains the same mechanics.

Create behaviors.md as the source-linked inventory, with a stable ID per cohesive
behavior area. For every area record:

- behavior/scope and all known entry points, source paths/labels and bank/physical
  context; adopted reference revision where applicable;
- related actors, states, levels, tables and asset identifiers, dependencies,
  shared state and update/call ordering;
- proposed source-tree modules/interfaces, contract test locations and ledger/
  uncertainty links;
- established observations, tentative interpretations, unresolved questions and
  source-backed reasons for exclusions;
- contract-worker status, trace/test evidence links and implementation status,
  initially pending.

Choose area boundaries around coherent game behavior rather than one task per
assembly routine. Cover all required behavior without duplicate ownership; record
cross-area effects explicitly. Add identifiers/modules missing from the skeleton
as declarations and design proposals, not game logic. Cross-check the inventory
against the source ledger and dispatch/data tables; an exclusion requires an
explanation, and lack of observed execution alone does not justify it.

Refine architecture.md with state owners, interfaces, algorithms, scheduling,
coordinate/quantity conventions, asset access and semantic debug observations.
Keep it actionable without a milestone plan or repeated planning cycles. Detailed
branch/carry tracing and executable tests belong to stage 3. Planning assertions
are leads, not source truth; label uncertainty honestly.

Hand off the complete behavior inventory, design and worker scopes in progress.md.
Stop when every identified area has source references and a contract assignment;
do not launch workers yourself. The behavior-contract stage coordinates them.
