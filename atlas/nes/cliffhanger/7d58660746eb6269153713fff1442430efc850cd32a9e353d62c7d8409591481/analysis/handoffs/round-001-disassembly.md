# Round 001 disassembly handoff

ROM classification: 64.48% (169049 / 262160 bytes classified; 93111 bytes unknown).

Round 001 disassembly completed as one substantive easy-gain pass. Outcome: **incomplete ROM classification**. Next phase: full current-source commentary, then redirect investigation. This is not a terminal exhaustion decision.

Verified checkpoints: baseline (raw fidelity only), round-001-startup, and final **round-001-disassembly**. Final portable mapping maps/round-001-disassembly.json; working analysis/state.json and maps/current.json; source generated/round-001-disassembly/game.asm; verification analysis/round-001-disassembly-verification.json confirms ok=true, byte_identical=true. Input SHA256 unchanged, 7d58660746eb6269153713fff1442430efc850cd32a9e353d62c7d8409591481. Toolkit configured binary used exclusively; no toolkit edit/archive/publication/cleanup.

Classification evidence: header and physical section sizes from parsed iNES; CHR 131088..262160 is separate pattern-image storage; vectors physical131082..131088 encode C000/FF97/C14A in fixed bank15. Explicit bank-aware opcode views from validated roots, actual contextual instruction fetches, and connected static paths, preserving mapper barriers. Inspect contexts.json, contexts-shortcuts.json, contexts-mechanics.json and discovery/final-discovery JSON files. No conflicting candidate instruction starts were accepted. 91 constant-data runs (24,924 bytes) have exact boundaries, neighboring bytes and rationale in uniform-data-evidence.json. Nearby references/connected code checked; no semantic table/fill claim made from uniformity. Raw output gaps remain unknown.

Execution and annotations: 13,031 code bytes, 156,018 data/image/header/vector bytes; 616 evidence-bearing regions. Union of preserved contextual files contains 5,336 distinct ROM opcode starts; execution coverage counts opcode starts, separate from structural classified bytes. Trace mechanics summary reports 7,274 total unique instruction addresses including non-ROM execution, 7,446 contextual roots, aggregate_dropped=0. Zero-detail tracing deliberately drops event details; exact accepted-fetch/context aggregates persist. RAM instruction fetches do not establish any ROM upload origin. Full routine labels, RAM variables and current-source commentary are still pending; byte identity is not readability completion.

Gameplay: natural Sony/copyright/title and attract/demo cabin reached. **Controller correction NES-001 supersedes earlier attempted-control claims:** {button,pressed} shared-protocol requests silently had no effect; actual NES API is {port:1,buttons:[...]} and buttons:[] releases. correct-inputs.py, intro-controls.py and mechanics.py use valid API. A valid title Start press opens helicopter story; eight Start10/release20 pulses,120-frame settling, and Right10/Right+A10/Right+B10/Up+B10 produce player-started level1 movement/climb from ground to cabin upper-left (natural-play-before/after.png). No artificial patch used in this route.

Artificial progression: saved clean gameplay snapshot, restored for each target; patch RAM0404=target-1,0708=2 (CPU fixed bank14 stage-status handler),240 untraced frames plus10 traced frames. Stage4 snowy forest visible,0404=4/0708=0; stage14 cliff hanging visible,14/0; stage8 Level Completed interstitial,8/2; terminal15 Gabe/Jessie mountain/kiss dialogue,15/2. These establish artificial state observations, not natural late-game reachability. Valid mechanics retest in mechanics.py: forest route returns to title after a fall/death; cliff route displays ladder/cliff platform and reduced lives (before/after captures). Stage8 gameplay itself remains unconfirmed. Start30/release120 and controls in stage8/ending stay in presentation states. Ending dialogue visited, staff credits not confirmed. Boss-specific combat not confirmed; needs stage/actor setup contract rather than blind RAM edits. See gameplay.md and scenarios.json.

Lead outcomes: no initial ready leads (original leads.json empty). Verified reset/vector connected code; verified title input contract after correcting shared-schema attempt; verified stage0404/status0708 progression and terminal selector12; verified bounded constant runs as generic non-code. Initial ineffective input method superseded, not silently discarded. No structural rejection inherited. Stage8/credits continuation remains a redirect question, not proof of unreachable/dead content.

Unresolved and concrete next tasks: (1) Commentary explain current fixed-bank main loop, input/presentation $EDA3/$EDF4, mapper helper $D58E, and stage consumer $0404/$0708, preserving exact aliases. (2) Redirect inspect $ECCC presentation selector records and $CC55/$CC83 setup plus fixed-bank CPU $CE12/$CE21 stage tables; find stage8/credits/boss setup entry contracts. (3) Banks8..11 have repeated 1024-byte constant-field/record patterns and exact annotated lanes, promising static non-code extension; inspect consumers/neighboring references before interpreting complete records. (4) Sound bank6 connected code uses zero-page FA..FC and RAM07xx state; inspect dispatch/indirect target barriers. Exact residual complement unknown-ranges.json (179 ranges), per-bank counts bank-counts.json. Do not replay completed experiments without changed question; continue connected investigations rather than stopping at a first pointer.

Toolkit report: **one confirmed bug**, NES-001 input API documentation/validation mismatch, workaround valid buttons lists, no blocker. analysis/toolkit-issues.md has reproduction, commit and correction impact. Long initial server/step exited SIGTERM143 for uncertain external reason; preserved logs/snapshot and recovered bounded startup traces, no toolkit causality claimed. NES discovery implementation uses max_instructions (not shared budget); scripts use actual supported parameter. Disk at last check ~3.6GiB free; selected captures/snapshots local, no deletion.

| Physical bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 0 | 0 | 1252 | 6940 |
| PRG 1 | 0 | 1496 | 6696 |
| PRG 2 | 0 | 1606 | 6586 |
| PRG 3 | 0 | 865 | 7327 |
| PRG 4 | 0 | 120 | 8072 |
| PRG 5 | 0 | 2546 | 5646 |
| PRG 6 | 1567 | 383 | 6242 |
| PRG 7 | 0 | 3294 | 4898 |
| PRG 8 | 0 | 4031 | 4161 |
| PRG 9 | 0 | 3319 | 4873 |
| PRG 10 | 0 | 3808 | 4384 |
| PRG 11 | 0 | 2053 | 6139 |
| PRG 12 | 2650 | 1 | 5541 |
| PRG 13 | 1103 | 86 | 7003 |
| PRG 14 | 4380 | 0 | 3812 |
| PRG 15 | 3331 | 70 | 4791 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 0 |
| CHR 4 | 0 | 8192 | 0 |
| CHR 5 | 0 | 8192 | 0 |
| CHR 6 | 0 | 8192 | 0 |
| CHR 7 | 0 | 8192 | 0 |
| CHR 8 | 0 | 8192 | 0 |
| CHR 9 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |
