# Round 1 disassembly handoff

Incomplete. ROM classification: 70.37% (276715 / 393232 bytes classified; 116517 bytes unknown).

Final state analysis/state.json, map maps/current.json, verified checkpoint pass-004; full counts/evidence in analysis/progress.md. 417 code bytes, CHR/header and 19 exact uniform PRG data runs. All Static, zero observed execution. Baseline retained. Mapper69 rejected by emulator.start; all gameplay unavailable (analysis/scenarios.json), toolkit issue TK-001. No toolkit edits or ROM changes.

Review current generated/pass-004/game.asm in commentary. Redirect: (1) C117 RAM-clear continuation from reset with X=2,Y=7,A=0 and effective banks [12,13,14,15]; (2) D394 -> C229 bank-write helper with A=1A,X=09; (3) IRQ E2CA indirect table and D3 domain; (4) NMI calls C084/C1DB bank context. Static raw candidate lists are hypotheses; code only accepted where inspected. Local fixed-slot mapper evidence allows fixed-bank continuation across writes without inventing other slot values. See analysis/leads.json and analysis/round-01/ records. No terminal exhaustion claim.
