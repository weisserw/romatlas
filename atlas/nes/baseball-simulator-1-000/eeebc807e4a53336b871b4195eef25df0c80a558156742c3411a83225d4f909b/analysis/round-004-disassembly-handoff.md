# Round004 disassembly handoff

**Single disassembly pass complete; ROM incomplete; nonterminal easy-gain handoff.**

ROM classification: 99.95% (262031 / 262160 bytes classified; 129 bytes unknown).

Code76543, Data54416 (including16header), CHR131072;129Unknown in30spans. Per-bank exact counts: `analysis/round-004-disassembly/bank-counts.json`; exact physical frontier/raw bytes: `unknown-frontier.json` in the same directory. New unique50Static Code bytes/nine spans,0Data and0prior-kind changes. Accepted F01,F02(two spans),F03,F05,F06,F07,F08,F09; retained F04 physical014BF0..014BF3 Unknown. Independent rationale in `structural-review.md`, exact raw/context/configured decoder checks in `independent-review.json`, mutation log in `annotation-log.json`, exact acceptance audit in `final-acceptance-audit.json`. All relevant known continuations/consumer contracts examined rather than decoder-only acceptance.

F04 is an isolated duplicate JMP8BAF separated from the contiguous five-veneer family by the entire emitter body. Its destination/encoding alone lacks an independent role-specific constraint; this does not reject F02's contiguous family or other supported contracts. No genericData/dead-code/natural-caller prerequisite or batch-wide veto. Historical r003 proposal remains unchanged.

State/current/final map SHA256 `808eeb38ef5b3411a7764ae95b6c98939e707d53b791fc646322facbbfc3b026`; checkpoint `maps/round-004-disassembly-final.json`; fresh generated source `generated/round-004-disassembly-final`; report `analysis/round-004-disassembly-final-verification.json` ok=true,byte_identical=true. Input identity unchanged eeebc807e4a53336b871b4195eef25df0c80a558156742c3411a83225d4f909b. Exact hashes in `analysis/round-004-disassembly/final-hashes.json`.

Maintained commentary/source/game.asm SHA4acccb9ce71d6c7479ddf89f0239e1a2d3c9ea65b8882797731d39737594c77a remains unchanged. C004 current C05D->CFB4 rendering-disable/NMIworkbit2 semantic correction preserved. Next separate commentary must carry sound previous reviews, update all new/affected source and perform full-current readability gate. Final editorial2719Code-row meaningful-unit metadata gap remains separately pending; this disassembly did not edit source or claim editorial acceptance.

Broad progression checklist reviewed/reused: S005 artificial initialized9INBOT/pitch/GAMESET, S008 defensiveMOVE/menu exit, S009 naturalUltra special-pitch/point use. S006 Season setup/shortcut trials still face widget/parent-transition contract barrier; no new practical corrected route emerged from these fragments. No unchanged replay, rare-contact forcing, successful catch/special-hit/Season-terminal or exhaustive-gameplay claim.

Next redirect should localize strongest residuals using the new map and updated source. Coordinator explicitly identified a useful possible corrected inference: weigh regular6byte wrapper sibling spacing plus established5byte terminal service payload as positive padding-pattern evidence for EA singleton spacers; the existence of a possible NOP-prefix interpretation must not become a universal no-executable-interpretation requirement. Current pass has not adjudicated that new question or asserted terminal exhaustion. Other residual RTS/actor/short jump/BIT-trailer code-versus-data ambiguities remain localized; known actor helper-contract failures do not reject every subinterval. Reuse primary evidence rather than repeat unchanged tests.

Existing TK-001 confirmed input-schema mismatch remains nonblocking; no new toolkit issues found (0new confirmed/suspected/missing). See `analysis/toolkit-issues.md`. No toolkit/input changes, archive, cleanup, deletion or publication. Progress/investigation/leads history updated durably. Disk checked before/final; scratch small and unchanged.
