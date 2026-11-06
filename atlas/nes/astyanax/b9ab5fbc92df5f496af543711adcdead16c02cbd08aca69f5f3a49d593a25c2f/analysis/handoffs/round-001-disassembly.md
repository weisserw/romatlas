# Round 001 disassembly handoff

One substantive pass completed; ROM incomplete. Next: full current-source commentary, then redirect.

ROM classification: 63.72% (167060 / 262160 bytes classified; 95100 bytes unknown).

Checkpoint maps/pass-001.json; generated/pass-001/game.asm; analysis/pass-001-verification.json. Internal state analysis/state.json and exported maps/current.json validate, rebuild byte-identical. ROM SHA256 b9ab5fbc92df5f496af543711adcdead16c02cbd08aca69f5f3a49d593a25c2f unchanged; checkpoint map SHA256 1a328745e1ab3f23d4293e9ead30db6be74285b8424edc1ca61b589a5245ceed.

11867 code,131072 CHR image,24104 Data,17 Text bytes;1818/1818 classified regions have evidence. Execution coverage separately4976 unique physical accepted instruction starts (natural4198,artificial3086; overlap). Unknown complement225 intervals in round-001/unknown-complement.json; exact per-bank code/data/unknown in round-001/counts.json and progress.md. Commentary readability is pending.

Validated fixed-bank vectors resetE000/NMIE0D0/IRQE1C3, inspected1361 physical startup instructions; fetched gameplay/ending instructions and724 static direct-control successors. Main table physical1E0A7..1E0DF is57-byte bank/LE-handler records, consumer stride3 and independent boundary at NMI. Generic maximal uniform PRG runs inspected at exact boundaries, neighboring bytes, references and conflicting code; no semantic filler claim. No conflicting code candidates; all annotation-conflicts files empty.

Astyanax title confirmed. Stage selector title U D L R B B B B Start,2 frames pressed/2 released, wait80. Selector actually uses Select to cycle and A to enter (source bank10 CPU96E0/968C); RAM03E4 index0..9. Durable title/stage-select snapshots retained. Visited Stage2-1 forest naturally through supported inputs; Stage4-1 cave via03E4=4;Stage6-2 castle via03E4=9, thenA and initialization350 frames. Latter entries artificial, actual instruction fetch evidence explicit. Attack/jump/movement exercised; extended Stage6 attempt reached right edge of encounter room but final boss defeat unverified. Demo selector and ending input code confirmed; ending-final.png explicitly THE END. Credits not observed after bounded continuation. All recipes/statuses in scenarios.json and gameplay.md.

Ready next investigations in leads.json: R001-CALL-CONTEXT (callee/return bank contracts from19 main handlers), R001-INDIRECT (cutscene9D30 bank2/audio8F37 bank12 dispatch), R001-LEVEL-DATA (fixed-bank level init and mapper wrappers, mixed unknown banks4/6/8). Follow connected callees/data, don't stop at first tiny handler. Current analyzer retains conservative JSR/mapper barriers; missing bank-return summary is explicit fact needing research, not proof of non-code. No terminal exhaustion claim/review needed at this ordinary phase boundary.

No toolkit issues found; analysis/toolkit-issues.md updated. No blockers. Environment exited long-lived server143; use direct.py bounded subprocess batches and durable snapshot import, not abandoned file mailbox (scratch request is stale). No toolkit changes/publication/archive/cleanup.
