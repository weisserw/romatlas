# Disassembly 001 handoff

One full bounded disassembly pass completed. Structural classification is complete; whole-workflow completion remains pending full current-source commentary and final editorial audit.

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

Checkpoint: [verification](disassembly-001-verification.json), [frozen map](../maps/disassembly-001.json), [source](../generated/disassembly-001/game.asm), [internal state](state.json), [current export](../maps/current.json). Map validation and fresh generated rebuild passed byte-identical. Input hash remains `4c83b6150e3cde4f0e463e4d1e51b78452d72131c675d9b33df9c65cb9604a14`. Frozen map hash `da70ff81f39549f73a0390f057d4a57025500eeb2dc8575af315acbd7847bc0c`.

| Section | Code | Data | Image | Unknown |
| --- | ---: | ---: | ---: | ---: |
| Header | 0 | 16 | 0 | 0 |
| PRG0 ($0010) | 5098 | 3094 | 0 | 0 |
| PRG1 ($2010) | 6240 | 1952 | 0 | 0 |
| PRG2 ($4010) | 1710 | 6482 | 0 | 0 |
| PRG3 ($6010) | 7530 | 662 | 0 | 0 |
| CHR0 ($8010) | 0 | 0 | 8192 | 0 |
| CHR1 ($A010) | 0 | 0 | 8192 | 0 |
| CHR2 ($C010) | 0 | 0 | 8192 | 0 |
| CHR3 ($E010) | 0 | 0 | 8192 | 0 |
| Total | 20578 | 12206 | 32768 | 0 |

All 147 regions have structural evidence; all use conservative Static provenance. 102 Code regions still use generated labels and need descriptive routine/branch/RAM commentary. Current classification is grounded in vector/direct closure, supported dispatch/stack contracts, data consumers, repeated non-code formats, and cartridge-defined CHR storage. Generic composite Data is intentional where exact semantics remain unresolved. Byte identity is fidelity evidence, not classification proof.

Lead outcomes and primary artifacts are in [investigation.md](investigation.md) and [leads.json](leads.json). D001-VECTORS/TASKS/AI-DISPATCH/SOUND/DATA/PROGRESSION/COVERAGE-AUDIT verified. Initial scratch data start $13A5 was corrected to $13A7 before checkpoint; see D001-BOUNDARY-CORRECTION. Final accepted tick-wait code covers $13A0..$13A6. Preserve superseded scratch record with this correction.

Gameplay: natural opening→mission1 cockpit→stable NAVSTAR map; artificial supported initialization visited missions 4/7/10 with rendered M4/E10, M7/E18, M10/E25, steering/fire/weapon samples; final completion gate→result→ending sunset scene. Exact inputs, releases, patches and budgets in [scenarios.json](scenarios.json). Later natural reachability and exhaustive mechanics are not claimed. Local game identification likely Garry Kitchen’s Battletank, revision unconfirmed; [gameplay.md](gameplay.md) records sourced hypotheses.

Execution is separate: [coverage audit](disassembly-001/coverage-audit-summary.json) replayed seven snapshots for 4 frames each and retained [physical](disassembly-001/physical-coverage.json)/[contextual](disassembly-001/contextual-coverage.json) fetch records. 2293 unique physical opcode offsets, 3889 origin/context keys, zero starts inside Data/Image. Aggregate drops 0; raw details deliberately dropped at capacity0. This is a lower bound, not all-code reachability.

Readability inputs: reset $F568; NMI $E00B; IRQ $F567; main task/frame calls $F62F/$F632 through RAM $3B/$3D. Initializers: $DD5F opening→$D766 frame, $DF4A briefing/map→$B10C, $ACD8 mission→$85F4, $FE8C result/ending→$FA6C. $FBB7 advances mission index $4F and chooses terminal mode at 10. $EFE7 AI indirect dispatch uses $A2C0/$A2C8 lists and pushes $EFE9 for RTS continuation $EFEA. Sound $AF30 uses callback at record+6 in $B0AC eight-byte records; $A57A uses modulation pointers. $A58B/$A58E are overlapping entry views (LDY #4/BIT skipping alternate LDY #2), represented by the $A58B physical decode; explain this in source comments. See saved listings and closure artifacts in `disassembly-001/`.

Deferred semantics: exact music/control-stream opcodes, full screen-map subformats, original quantized-table generators, all natural callers and object/resupply/rescue contact routes. These do not leave classification unknown; editorial review may reopen a structural claim if it finds contrary evidence.

Toolkit: one confirmed nonblocking metadata reporting bug NES-001, no suspected or missing-feature-only findings; [report](toolkit-issues.md), [minimal reproduction](disassembly-001/observed-marker-repro.json). Discovery candidate flags stay Static/false despite accepted fetch coverage. Explicit coverage is the workaround; toolkit unchanged. No publication, archive, ROM edits or toolkit changes performed.
