# Repository instructions

These rules apply to work in the ROMAtlas toolkit repository. The user's task
selects the workflow; this file does not activate disassembly, porting or QA.

## Shared rules

- Read [project.md](project.md) and the relevant current entries in
  [docs/progress.md](docs/progress.md) before changing the toolkit.
- Use Rust 1.90.0 and Cargo.lock. On Ubuntu/Debian, install the pinned tools with
  `bash scripts/setup.sh`. Build with `cargo build --locked --release`;
  CLI examples and integration scripts default to `target/release/romatlas`.
- Preserve unrelated changes and existing game workspaces. Do not implicitly
  migrate another platform's core, maps, toolchain or workspace.
- Never download or commit commercial ROMs or extracted assets. Keep generated
  ROMs, source, traces and captures local. Preserve dependency licenses/notices.
- Keep shared crates platform-neutral: do not import console adapters, cores or
  platform-specific CPU types into shared analysis/control/disassembly code.
  Shared agent logic uses CommandAdapter; platform behavior belongs in adapters.
- Keep the MIT native runtime independent of emulator cores. Never copy GPL
  emulator implementation into romatlas-port.
- Preserve deterministic emulated time and replay. Do not introduce host-time
  shortcuts. Keep CPU aliases, physical backing and execution evidence distinct.
- Do not launch SDL graphics or require unavailable GUI/audio devices on this
  headless server. Compile desktop code without opening a window.
- Follow [docs/development.md](docs/development.md) for relevant checks. Record
  actual changes, check results and limitations in docs/progress.md. Do not turn
  toolkit checks into additional game-fidelity acceptance requirements.

## Documentation and task entrypoints

- [Development guide](docs/development.md): architecture contracts, check commands
  and platform documentation.
- [README.md](README.md) and [protocol](docs/protocol.md): setup, CLI and APIs.
- [INSTRUCTIONS.md](INSTRUCTIONS.md): preparing and invoking per-game workflows.
- Disassembly: `scripts/disassemble.py` prepares a separate workspace; its
  `goal.md`, phase prompts, workflow and evidence policy own the procedure and
  classification/readability/rebuild completion criteria.
- Native porting: [porting guide](docs/porting.md); prepared workspace `goal.md`
  owns iterative planning, one-milestone porting and scoped QA subagents.
- Separately requested native QA: [debugging guide](docs/port-debugging.md);
  prepared workspace `debug/goal.md` owns QA and its acceptance criteria.

Preparing workflow files does not launch an agent or start QA. Select only the
workflow needed for the requested task; its completion criteria remain scoped to it.
