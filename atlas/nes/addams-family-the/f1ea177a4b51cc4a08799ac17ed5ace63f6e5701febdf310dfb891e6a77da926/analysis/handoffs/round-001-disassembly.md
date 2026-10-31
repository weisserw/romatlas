# Round 001 disassembly handoff

Exactly one pass completed; incomplete classification. Next: full current-source commentary, then redirect. No live server, toolkit edits, cleanup, publishing or ROM changes.

ROM classification: 55.88% (146501 / 262160 bytes classified; 115659 bytes unknown).

State analysis/state.json; map maps/current.json. Final checkpoint maps/pass-001-verified.json / generated/pass-001-verified / analysis/pass-001-verified-verification.json: valid and byte-identical; map SHA256 d9865903bf1c965ce68368ef12bb243fa04dcb03c7a6afaae1577c7b30dc3c5f. Earlier checkpoints preserved. Code14089, Data1340, Image131072,489regions; all classified bytes evidence annotated. Per-bank and exact unknown complement: analysis/round-001/classification-summary.json and per-bank.md. Execution separate: persisted union3685ROM opcode starts,1RAM start. Interrupted aggregates not counted.

Reset CPU$C000, effective banks[0,1,14,15]; NMI$0104/IRQ$0314 RAM handlers do not prove ROM upload origins. Reset+observed static graph inspected, official opcodes, exact nonoverlapping bounds; mapper effects/unresolved indirects stop propagation. Candidates/proposals and original discovery files in round-001. CHR section classified Image; maximal uniform PRG runs have bounded flank review in uniform-data-review.md. Consumed scene pointer tables and repeated ordinal bytes classified Data without full natural selector claims.

Game The Addams Family. NES input uses {buttons:[...]}, [] releases; generic button/pressed silently ignored (NES-001, one confirmed documentation/validation issue; no blockers). See toolkit-issues.md. Servers42641/28114 exited143 externally; bounded.py now owns a short subprocess and saves roots/summary before normal exit. No toolkit bug inferred from process exits.

Natural route: title, gate/hint/continue acknowledgements, garden HUD1492 and right90frames. Durable active.snapshot (banks[4,5,14,15]). Resume: map.load, emulator.start, snapshot.import(path) adopts immediately; snapshot.save establishes a name. Reassert buttons. Artificial room reload: restore active,buttons=[],$71=room,$80=1; main$C1DF->$C0F2 initializes scene. $80=2 restarts game and resets room. Crypt02/hallway03 had45load+45exercise images but lost aggregate on interruption; pond0E,river19,Gomez20,roof0D,freezer21 each20load+20exercise with saved contexts. Ending-attempt.py adds RAM$0471=$51 after20loadframes, then20frames produces congratulations/Morticia/family treasure terminal screen. ROM Morticia handler physical$4EA2–4EA4 emits that transition value. Ending is artificial, not natural quest completion. See scenarios.json/gameplay.md and *-bounded-* files.

Gains: startup3582code; scene batch10174; ending62; roof/freezer alternatives271. Representative enemy/interior/swim/gondola/hazard/terminal systems visited. Further nonuniform roles and mapper/indirect closure need focused investigation. No terminal exhaustion claim; do not repeat intro or unchanged experiments.

Redirect avenues:
- Scene mode$045B dispatch: initializer pointer bytes physical$1C150..$1C164 and NMI phase pointers$1C2AC..$1C2C2. Derive supported target contracts/effective bank from consumer and selected mapper state; pointers alone do not prove target code. init-decode/transition-decode and current source are starting evidence.
- Mapper helper CPU$C081 mirrors PRG selection at$0468 then serial MMC1 writes; $C046/$C05A/$C06E affect other registers. Generic traversal stops every JSR. Prove preserving/changing effects and resume justified static successors in connected batches.
- Room$71 initializer CPU$D3F8 and banked consumers: use persisted room roots and exact unknown complement to identify bounded record/pointer/data roles. Nonuniform unknowns remain ambiguous; generic Data is sufficient when non-code is established, without semantic completion.

Current-source readability/editorial review has not run. Toolkit report has one confirmed issue, no blocker.
