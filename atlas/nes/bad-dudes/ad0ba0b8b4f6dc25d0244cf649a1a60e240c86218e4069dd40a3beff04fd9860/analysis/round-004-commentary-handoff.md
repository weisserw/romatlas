# ROUND004 commentary handoff — current-source readability complete

ROM classification remains **99.96% (262046/262160;114 Unknown bytes)**.
Current-source readability is complete; whole-ROM classification and final editorial
acceptance remain separate.

All **2386 scaffold records/3064 units reviewed**, **0 pending/partial**. Reviewed
units cover all **57636 classified Code bytes**, with no uncovered intervals.
The full physical partition, source anchors and unit bounds pass with0 errors.
Preserved ROUND003 source/review remains in
[round003 preservation](round-004/commentary-round003-preserved/);2289 unchanged
physical/kind/view blocks retain valid review credit. No fresh preparation on resume.

[Code batch](round-004/commentary-batch-001-code.json) explains1020 newly classified
Code bytes: bank5 motion/clamp/countdown/copy and225-byte stride-record prefix,
44 detached epilogues, bank13 restricted paired store and fixed ordinary/synthetic/NMI
continuations. [Data/boundary batch](round-004/commentary-batch-002-data-unknown-heads.json)
reviews70 Data bytes and eleven Unknown intervals without fabricated meanings.
C607 performs SBC $60,X then44-byte $02A2->$02FA copy; C8BD stores incoming A to
final CHR latch, clears requests and releases OAM. Record fields and branch symbols
name established mechanics. Origin comparison equality and unsigned7-minus-Y
arithmetic were corrected during integration.

[Dependencies](round-004/commentary-dependencies.json):24 concrete findings and103
adjoining boundaries checked;139 prior valid dependency reviews preserved,0 pending.
Bank13 retains normalMMC3mode0, Hbit6clear/low3<=5, even selector then odd CHR data,
unchanged ROM readback and possible infinite retry. No writable mirror, natural FF
record, successful update or eventual return claim. Fixed continuations retain
bank6 context, ordinary/synthetic/NMI frame distinctions and busy-wait assumptions.
Detached epilogues do not infer incoming edges or tail-JMP fallthrough.

[Final scope assessment](round-004/commentary-scope-assessment-final.json) records
qualitative review and exact checks. [Uncertainties](../commentary/uncertainties.md)
and local source notes preserve unresolved natural callers, visual/game meanings,
foreign encoded origins, A000-slot successor contracts and exact114-byte structural
ambiguity. These are explained limitations, not skipped readability work.

Fresh supported [verify](../commentary/verification.json): **ok:true,
byte_identical:true**. [Reading copy](../commentary/reading-after.asm) refreshed
after final edits. Source SHA256907de1037f4e132054bb68513de85bbbb8ea62a84d7dd3906aded68544c8637a;
symbols85869fa80d10cd255509fd2517340ba0b99bb6a2d7bd255c49cb88ef46663919.
Current map and internal state match frozen commentary baseline/state at
1db9d5c4f74b94f28be6f6627f33831e9924b1679dd73058e5ce48a8d28fa140.
Editorial audit remains pending and was not run. No ROM/map/state/toolkit edits,
archive, cleanup/deletion or publication. Over4GiB free; bounded local artifacts.

[Toolkit report](toolkit-issues.md): no new toolkit issues found. ExistingNES-001
confirmed documentation/API mismatch andNES-002 indexed-RAM analysis missing
feature remain nonblocking;0suspectedbugs/0blockers. No toolkit modifications.

Next phase: coordinator reviews this gate, then ROUND004 REDIRECT against the
remaining114 bytes and prior residual evidence. This worker has stopped writing.
