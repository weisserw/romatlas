# Round 001 disassembly handoff

Single substantive pass finished; **ROM incomplete**, ready for full current-source commentary then redirect.

ROM classification: 34.17% (89582 / 262160 bytes classified; 172578 bytes unknown).

Code 11,482; Text 72,754; other Data 5,346. All 1,360 classified regions have evidence. [Per-bank counts and exact unknown complement](../round-001/classification-counts.json). Execution is separate: 4,603 physical instruction starts retained across durable post-recovery scenarios; no aggregate drops. Current-source readability is pending commentary.

[Internal state](../state.json), [portable current map](../../maps/current.json), [verified pass-001 map](../../maps/pass-001.json), [generated source](../../generated/pass-001/game.asm), [byte-identical verification](../pass-001-verification.json). Input SHA-256 unchanged. Baseline already established; do not repeat it. Toolkit/binary unchanged; no toolkit issues found ([report](../toolkit-issues.md)).

Accepted batch: decoded reset/NMI/direct control-flow graph and saved gameplay roots, exact prose runs, clearly uniform non-code runs, iNES header/vectors and four consumer-backed 512-byte town direction maps. Static provenance retained where natural execution not proven. No whole-bank data guesses or legal-decoding-only acceptance. Code evidence and diagnostics: [batch05](../round-001/batch05-code.json), [town maps](../round-001/town-map-data-evidence.json).

Game is Hillsfar (NES). Verified title/camp/sample cleric, overworld, riding controls, artificial supported ride completion, city entry, town movement/search and target-range location message. [Checklist](../scenarios.json), [manual/research](../gameplay.md), [exact inputs/budgets](../round-001/routes.jsonl). B toggles town status overlay; A at tested sites searches empty area. No range minigame/maze/arena/ending visit claimed.

Useful shortcut: banks [20,21,30,31], extent $03F4=35; patch $03F5=34 and $03DB=8 then60 frames. Normal scroll updater completes ride; [contract](../round-001/riding-progress-contract.json). City Yes uses Left/A. All later scenarios inherit artificial provenance. New server: map.load, emulator.start, snapshot.import(path), reassert buttons. [Batch driver](../round-001/batch.py) saves contexts/snapshot each run; avoid idle cross-call servers, which host termination interrupted initially.

Redirect frontier, with no terminal exhaustion claim:

- **R001-BUILDING-ENTRY:** import [gameplay21.snapshot](../round-001/gameplay21.snapshot). District1 at x13,y4,west yields “YOUR RIGHT SIDE TARGET SHOOTING.” Prior arena guess was corrected. Resolve side/neighbor/action dispatcher before repeating A or further navigation. Exact coordinate/facing/persistence contract and four map origins are saved.
- **R001-GRAPH-CONTINUE:** follow bank wrappers and indirect handlers using accepted context roots and current-source references; preserve data barriers.
- **R001-GRAPHICS-CONSUMER:** derive packed stream pointers/terminators for PPU loader around $D2FC..D33F; patterned unknown banks remain unsupported as whole-bank data.
- **R001-LATER-ENDING:** derive class/guild/quest flags and supported transition contracts before patching later quests, bosses or ending. Inspected $0720/$0721 uses and bank0..7 script prefixes are starting evidence, not safe complete contracts.

All lead outcomes/recipes are in [leads](../leads.json); reasoning/history in [investigation](../investigation.md). Easy rooted gains and bounded immediate gameplay attempts are complete for this pass. Further useful progress needs these concrete consumer/state contracts; this is not a terminal stopping assessment.
