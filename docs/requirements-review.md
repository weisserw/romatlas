# Requirement review history

This file retains the implementation review history. The current requirement-by-requirement decision is in docs/completion-audit.md. Its inspected evidence covers the original scope; earlier open-work entries below are historical. Hardware support remains incremental with documented limits.

| Requirement | Evidence to inspect | Open work or limit |
|---|---|---|
| Inspect ROM metadata, banks, decoded bytes and unknown regions | CLI inspect/decode; rom.info/read/regions; session regression | Final API/result audit |
| Headless reset, inputs, bounded instruction/cycle/frame stepping | Emulator tests; typed execution tests; session regression | Broader CGB peripheral checks |
| Deterministic snapshots | Replay tests, RTC tests, homebrew regression | External state versions differ; version 7 is current |
| Actual CPU bus observations and bank identity | Trace tests; DMA tests; contextual bank-write regression | Full hardware timing remains incomplete |
| Registers, mapped memory, physical storage and mapper state | Execution adapter tests; emulator.memory/physical_memory/state | Final capability/result audit |
| Structural edits with confidence, evidence and conflicts | Store tests; map APIs; rejected-edit session cases | Final malformed-input audit |
| Static/observed discovery with uncertain targets | Discovery and pointer tests; context traversal tests | Do not treat a reached target as a fetched instruction |
| Unknown bytes stay unknown internally | Store gaps; analysis.unknown; export tests | Export uses lossless data directives, not a data claim |
| Generate without running the emulator | CLI generate; fidelity regression | External charmap resources are unsupported |
| Rebuild and compare all bytes with first mismatch location | Backend compare tests; fidelity fault injection; portability regression | Final file/path audit |
| Portable ROM-free map and clean recipient rebuild | Stock .sym export; portability/session/homebrew scripts | Keep runtime evidence out of portable maps |
| Community .sym labels, aliases and supported options | Map/store tests; imported-map session rebuild | Local RAM labels and external charmaps need explicit handling |
| Safe code/data/text/padding/image annotations | Content and argument tests; generation rebuild gate | Rejected annotations need a revised map or raw data |
| Preserve headers, checksums, banks and unknown bytes | Four-bank arbitrary-byte portability fixture | Broader input/fidelity audit |
| ROM-only, MBC1, MBC3, MBC5, common variants | Mapper tests; 89-case default and 55-case CGB hardware selections; unsupported-feature errors | Final cartridge-variant audit |
| DMG/CGB CPU, interrupts, timer, joypad, PPU and DMA | Decoder matrices; hardware gate; synthetic peripheral tests | Broader PPU/CGB peripheral review |
| Multiple real homebrew workflows | Pinned Armada/Unbricked gameplay, replay and rebuild reports | Broader CGB homebrew coverage |
| Complete decoder and independent opcode checks | Base/CB matrices; hand-derived operand/flag matrix | Final test-coverage audit |
| Trace budgets, filters, summaries and streaming | Trace/stream tests; contextual aggregates | Remaining performance review |
| Platform-neutral types and interfaces | Wide mock tests; execution/backend traits; boundary script | Dependency checks alone do not prove absence of hardware assumptions |
| No GB implementation imports in shared crates | Cargo metadata boundary check | Inspect source boundaries in final audit |
| CLI and persistent machine-readable API | Session tests; api.describe; protocol call table | Final parameter/error audit |
| Bounded runs, external tools and outputs | Budget/process tests; request limit tests; stream tests | Remaining performance review |
| Setup on a new VM with pinned tools | setup scripts, lockfile, toolchain and CI | Final fresh-setup audit |
| Licensed dependencies and homebrew sources | Architecture/license docs; pinned source notices | Final source/artifact hygiene audit |
| Source, tests, CI, AGENTS.md, guides and examples | Workspace, scripts, docs and workflow files | Final delivery audit |
| M1 through M4 exit conditions | Acceptance evidence above | Full project remains incomplete |

The protocol call-table check queried the running server and found all 43 advertised methods in docs/protocol.md. This proves method-name coverage only. It does not prove that each documented parameter, result, error or bound matches runtime behavior.

Use the actual CLI commands in README.md. The `analyze` line in project.md is an illustrative workflow, not a shipped command. Use the documented analysis RPC calls for discovery and pointer hypotheses.

## Current acceptance sweep

The current worktree passed cargo fmt --check, workspace clippy with -D warnings, workspace tests, all nine standalone core tests, debug/release builds, the dependency boundary check, session workflow, clean stock portability, content/fidelity fault tests, 89 default hardware cases, 55 CGB cases and both pinned homebrew workflows. The homebrew reports prove input-driven memory changes, snapshot replay, programmatic code/edge discovery and exact stock rebuilds.

These results prove the named cases. They do not close the complete API parameter/error, source-boundary, setup or delivery reviews by themselves. The architecture guide now describes current interfaces and limits rather than earlier pending work. Setup verification is in progress.

Setup completed successfully on the current Ubuntu VM with the pinned toolchain/tools and locked build. Existing tools/build caches were present, so this proves idempotence here and does not close the clean-environment check.

API error evidence now includes scripts/test-api.py: all 43 methods reject non-object params, and 37 negative cases preserve state/map/trace and avoid rejected outputs. The test passed in both the host environment and clean Ubuntu root. This does not prove every documented field/default; the broader review remains open.

Clean-environment setup evidence: scripts/test-fresh-setup.sh completed on an Ubuntu noble minbase root with empty Cargo/tools/target locations and no copied build caches. Its full command sequence passed; the success-only JSON and log are local under build/. This covers clean package/tool installation and acceptance checks on the existing host kernel. It does not imply all hardware compatibility or substitute for the final source/API/delivery audit.

## Source and delivery inspection

Shared core/analysis/agent/disasm/store source was inspected for Game Boy imports, SM83 types, fixed bank sizes and narrow address assumptions. The only matching 65536 constants were documented byte budgets, not address widths. The Game Boy .sym parser remains a format-specific adapter; the internal Store uses shared types. Cargo dependency boundaries and wide mock tests also pass.

Source delivery contains no ROM, object or image artifact outside ignored generated directories. Python bytecode is now ignored and the generated cache was removed. Opcode metadata has its CC0 license/revision; the fork and external source notices remain intact. README and homebrew guides now show current gate counts rather than obsolete failures. Fresh-run source copying excludes Python caches, and the runner clears stale success markers and includes the API regression in future runs. Shell syntax, boundary and API checks pass after these delivery changes.

## Defaults and result inspection

The API regression now checks documented 16-byte defaults, the 16-instruction decode default, whole-ROM unknown range, empty map/event pages, request/response budgets, loaded address-space sizes, enabled snapshot/input capabilities, one-operation step default, 1000-operation run default, cycle advancement and exact named-snapshot replay. All pass. Dispatch and adapter sources were inspected for operation/cycle/frame limits, physical/mapped read bounds, pointer widths/pages, trace config validation and errors. Stale MBC5/static and snapshot-format statements were corrected. Positive session/homebrew workflows and rejection tests cover the complementary map/discovery/generation paths.
