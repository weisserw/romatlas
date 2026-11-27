# Round 001 disassembly handoff

**Single pass complete; ROM incomplete.** ROM classification: 17.55% (23000 / 131088 bytes classified; 108088 bytes unknown).

Verified final checkpoint: `analysis/round-001-disassembly-final-verification.json`; map `maps/round-001-disassembly-final.json`; source `generated/round-001-disassembly-final`; live saved state `analysis/state.json`, exported map `maps/current.json`. Validation and byte-identical rebuild passed; input SHA-256 unchanged. 16-byte header plus per-8KiB-bank counts are in `analysis/round-001/classification-counts.json`, mirrored in progress/investigation. Unknown complement: `analysis/round-001/unknown-ranges.json`.

Accepted 20584 code and 2416 data/text bytes, with evidence on every region and explicit views on every code region. Retained actual ROM fetch roots: 8366 unique physical starts (coverage union/lower bound, not full execution coverage). Full-source commentary remains pending; proceed to commentary, then redirect. No completion or terminal-exhaustion claim.

Key reusable contracts:

- NES input is `{buttons:[...],port:1}`; reassert the complete intended button list after restore and clear with an empty array. Shared button/pressed schema silently fails (NES-001).
- Persistent snapshots are local same-build exports. New server: map.load saved state; emulator.start; snapshot.import path; configure tracing; reassert input. Initial and successful scene snapshots are retained in `analysis/round-001`.
- Attract countdown: $9F decrements every16 frames; saved, bounded $9F=1/$02=15 expiries advance startup. Edits are artificial.
- Stage recipe: restore `skip-2.snapshot.json`; empty inputs; advance10,000 cycles then finish an operation to get past pending NMI; set $0737 selector, $0702 preceding display number, $ED=0,$0738=9,$05F5=12; set main PC $816D with effective PRG banks[0,1,2,3]. Initial immediate-PC attempts were interrupted and rejected (`entry-trace.json`). `visit_one.py` and scenarios record successful state-dependent initialization. Selectors25–32 cover representative systems;48 initializes final shark stage24;49 initializes automatic ending stage25.
- Actual final shark appearance is `boss-long-0.ppm`; `boss-fire.py` exercised A/B/left20frames afterward. No defeat claim. `credits-2.ppm` and `terminal-0.ppm` visually establish credits/thanks after automatic ending waits. Bulk waits untraced; short milestone samples traced. These are artificial visits, not a natural full run.
- FF80 bank-call thunk sets low dispatch pointer $E4=X, high $E5=$80, selects bank from A and calls $8000 (JMP indirect). Stub tables6/6/11/33 entries are independently bounded; accepted static sibling routines do not imply natural selector reachability. `dispatch-*` evidence preserves exact contexts/bounds. Do not carry static bank context across unresolved return/mapper effects.
- Inline bytes physical$1A55E..$1A563 are five controller masks, not ISC code. Data barrier prevents false branch-fallthrough discovery. A transient PC$0095 during a frame step was **not** RAM fetch evidence; subsequent accepted ROM fetches are in `ram-instruction-check.json`.

Next redirect questions (not prevalidated proposals): inspect existing PPU-writing consumers to bound packed graphics/data streams; analyze indirect state tables and post-bankcall contracts; use unknown complement and references to distinguish remaining code/data, including banks9/14. Preserve structural inference vs reachability separation. Uniform data block semantics remain unresolved but their generic non-code classification is complete. Do not repeat rejected immediate-PC or legacy input recipes without corrected conditions.

All attempted lead outcomes are in investigation/leads; progression checklist is `analysis/scenarios.json`. Toolkit report: one confirmed NES input documentation/API-validation issue, no blockers; toolkit unchanged. Disk checks retained ~3.5GiB available; local artifacts bounded, no deletion/archive/publication.
