# Native port architecture

Status: skeleton pending. Record a concise source-informed design here as the
fixed pipeline progresses; track stage/worker status and resumption in progress.md.
The behavior inventory and source/test/implementation links live in behaviors.md.

## Modules and ownership

Describe modules/files, state owners, interfaces, algorithms, dependencies and
shared platform helpers. Choose boundaries around game concepts, not banks.
Keep rendering/audio/frontends and game rules coherently separated.

## Identifiers and asset definitions

Inventory actors, states, levels, sound, music, tables and other assets with source
references and honest provisional names. Explain typed IDs versus extensible
asset slots, ROM physical/banked addressing and decoded formats.

## Scheduling and debug contracts

Explain simulation/input/audio/presentation ordering, numeric/coordinate units,
relevant source quirks and invariants. Link debug/api.md for the semantic state/
event schema, deterministic reset and validated replay checkpoints.

## Decisions and uncertainty

Record design decisions, source facts, implemented assumptions and unresolved
questions. Keep source/contract links and correction reasons as decisions change.
No milestone loop or recurring planning/QA cycle is part of this pipeline.
