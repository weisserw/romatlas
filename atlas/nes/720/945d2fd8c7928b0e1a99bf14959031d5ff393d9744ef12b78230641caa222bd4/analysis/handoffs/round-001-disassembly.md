# Round 001 disassembly handoff — phase finished, ROM incomplete

ROM classification: 91.64% (120134 / 131088 bytes classified; 10954 bytes unknown).

Checkpoint `pass-001` is fresh and byte-identical verified. `analysis/state.json`, `maps/current.json`, and `maps/pass-001.json` match map SHA256 `44848ce7a8323b7fcffbc8cb9542913b0d6a37253c6c8ef2f4bf7e9e48890e4b`. Verification: [report](../pass-001-verification.json). Source: [game.asm](../../generated/pass-001/game.asm). Input SHA256 unchanged `945d2fd8c7928b0e1a99bf14959031d5ff393d9744ef12b78230641caa222bd4`.

14,761 Code bytes; 105,373 Data bytes; 643 evidence-bearing regions; 126 unknown intervals in [unknown-final.json](../round-001/unknown-final.json). Header adds 16 Data bytes outside banks. Exact accounting and execution counts: [final-counts.json](../round-001/final-counts.json).

| Physical 8 KiB PRG bank | Code | Data | Unknown |
|---|---:|---:|---:|
|0|751|7043|398|
|1|3059|714|4419|
|2–11 (each)|0|8192|0|
|12|710|7482|0|
|13|0|8192|0|
|14|5910|0|2282|
|15|4331|6|3855|

Validated vectors NMI $C372/reset $C000/IRQ $E898 in effective fixed banks14/15. Static code closure follows validated control paths and direct JSR callee entry under pre-call banks; it does not assume bank-preserving returns. Two small audio continuations use independently understood RAM/APU-only helper/loop contracts. Observed instructions retain exact mapped views; Static reachability is separate.

Banks2–11 show continuous graphics/map patterns with local tile visualization and boundary review; mixed/encoded formats remain generic Data. Audio driver $8890..$8B55 anchors instrument/pointer/pitch/stream data in banks12/13. Bank0 prefix and middle record domains and bank1 sampled lookup payloads have explicit repeated-layout/consumer evidence. Detailed schemas remain unresolved without leaving obvious non-code unknown. See [data-review](../round-001/data-review.json), [tile overview](../round-001/tile-overview.png), [sound listing](../round-001/sound-listing.json), [sound tail](../round-001/sound-tail-listing.json).

ROM visually identified as NES 720°. Corrected input route naturally reaches Class1/Main Park. Artificial understood $D93A entry initializes Ramp/Slalom/Downhill/Jump, then actual input/fetch evidence samples their mechanics. Class4 Ramp visited through class selector patches before loader, skipping only pure $C35D countdown via $30=0. Failure-path $C2DE call shows TOP TEN high-score terminal screen. These are artificial visits, not natural event/terminal reachability. Recipes and persistent snapshots are in [scenarios](../scenarios.json); selected frames include rampplay-2, event2/3/4mechanics-0, class4mechanics-0 and gameover-0 under round-001. Aggregate union: 5,868 physical opcode starts; 4,868 in natural scenario files, 5,180 in artificial scenario files, overlapping. Detail capacity0 intentionally drops detail; recorded aggregate drops0.

TI-001: NES silently ignores shared protocol `{button,pressed}` and accepts only full `{port:1,buttons:[...]}` sets. Earlier menu/title/held/park/startgame names therefore represent NO-INPUT attract runs; their fetch evidence remains valid and causal input claims were corrected. See [toolkit issues](../toolkit-issues.md): one confirmed documentation/API-validation mismatch, no blocker. No toolkit changes. Several long experiments received exit143; cause uncertain, not a confirmed toolkit issue. Partial evidence retained and per-action snapshots now allow resumption.

Leads L001–L004 verified; L005–L007 are ready redirect questions in [leads.json](../leads.json): mixed fixed-bank code/inline records at physical $1F4CE..$1F98F and $1F9EE..$1FFFA; event-result/timer/class-completion contract; shop/world-state interaction contract. Result/medal screens and shops are deferred for these concrete missing contracts. Distinct ending unknown; class4 cap is not proof of absence. Reuse saved scenes rather than replaying intros. Broad easy gains and bounded static closure are complete for this pass; residual mixed structure requires redirect investigation, not speculative annotation. This is not terminal research exhaustion or editorial completion.

No active experiment needs to keep a server alive. No ROM/toolkit/publication/archive changes; all local evidence preserved. Scratch4KiB, round-001 evidence239MiB, free disk about11GiB at handoff. Next phase: current-source commentary, then redirect.
