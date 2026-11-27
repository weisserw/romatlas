# Round001 connected redirect findings

Input checkpoint unchanged: state SHA2566a267a235d3e6df70c39619e1f756cec0cf20fe77f9f50a907a0b9a369173816;
ROM42664d5b34381a00f6d92214620a73ab851c8679c7a805a1f4239b8e9b1726e5.
No classification, source, toolkit or ROM edits. All claims below are proposals.

## Control contracts and supported continuation batch

C389 disables NMI, pops its JSR return pointer, adds4 to the saved return,
reads four following bytes into destination low/high and count low/high, and
clears RAM. Zero low count has256-iteration behavior. C034 directly jumps here,
so caller inline records apply there also. C031 jumps to C96C, which performs
its own two inline clears and returns normally; it does NOT consume its caller's
following four bytes. closure.py deliberately distinguishes these contracts.
23 inline records and their callers are in inline-records.json. Startup bytes
that formerly resembled BRKs/RTIs are inline data, permitting ordinary code after
them to be traced. YieldOneFrame and its delay trampolines save continuation
return addresses; their post-call bytes remain supported resumed code, not
ordinary immediately executed fallthrough.

closure.json and closure-unknown.json retain instruction bytes, physical offset,
CPU address, effective16KiB selector (bank0=>[0,1,14,15], bank6=>[12,13,14,15]),
and parent path. bank=-1 means switch windows explicitly unresolved:
[null,null,14,15]; only fixed-window successors are followed there. Mapper
trampolines and direct writes invalidate context; constant preceding LDA values
supply a new selector when justified. Unsupported lower-window continuations
stop. No pointer-only blanket ROM decoding is proposed. Roots are previously
accepted instructions, reset vector, projectile fallthrough, consumer-supported
sparse dispatch handlers, and three independently coherent static routines
(9B04 six-byte PPU stream; C718 modulo8 nametable fill; CCBE bounded controller
area selector). These last three have structural contracts but no asserted
natural entry. Ordinary direct JSR paths use the normal return convention except
explicit inline/stack/mapper contracts. Independent disassembly must validate
these contracts, re-decode exact instructions and prune any overreach.

No candidate instruction overlaps another instruction boundary, existing Data,
or the data proposals. Candidate count includes duplicate physical instructions
in different contexts; proposal-summary.json uses distinct physical byte counts.
All proposed instruction starts decode official instructions; no trap/unofficial
instruction was used to force a path. This supports rather than replaces the
control/stack evidence. Structural inference is Static except actual saved
stage-contexts.json and endpoint-contexts.json instruction fetches (artificial
state). Existing Observed map labels are not automatically treated as fetch proof.

## Dispatch graphs followed

Physical1D1DC..1D218 contains30 address-minus-one words consumed by D208.
26 are nonzero, four slots16..19 are zero. AND1F permits selectors30/31 but
those would read helper opcodes A80A/C8C8, not two additional valid table words.
The independent endpoint is D208 itself, not an invented32-entry extent.
Physical1D660..1D680 contains16 words after secondary dispatch; object field25
supplies selector and its global domain remains unresolved. Physical1EBC0..1EC20
contains48 words after the spawn-kind dispatch; selector92's full natural domain
remains unresolved. Endpoint routines D670/EC10 and coherent handlers were
independently inspected. Data role does not require proving every selector.
Nonzero handlers and their supported branches, shared helpers, yielding states
and associated spawn/copy routines were followed in the same closure.

Bank0 has its own sparse eight-word address-minus-one command table at physical
4E7..4F7, consumed by85B5 after X masked0..7. Zero slots4/5 are not code roots;
the six nonzero handlers form coherent note/tick/loop/pointer/channel contracts.
Bank0 request8003 is independently called by CE6B sequence: select bank0,
JSR8003, select bank6. No artificial execution was needed to prove its structure.
Bank0 code finishes862E RTS; data begins862F. Specific duration tables are read
at862F/8637, pitch base863F and sound headers via8700. Repetitive stream/data
content through physical1C90 was inspected; generic Data suffices where exact
individual stream layouts remain unproved. FF run after1C90 already classified.

## Palette correction

C21B..C29B (physical1C22B..1C2AB) is128 palette bytes. C21B uses X0/16 and
16 reads; C1B9 preset supplies C23B and32 reads; other presets supply C25B/C27B
and32 reads. All128 bytes fit NES palette values (maximum38 hex). Upload loop
LDY16; DEY; BNE repeats untilY0, then BEQ necessarily branches to C1DE. Its
fallthrough decode into palette bytes is impossible under this visible contract.
Four old saved context sets have no actual fetch in1C22B..1C2AB. Existing41 Code
bytes in1C22B..1C254 were conservative static fallthrough, not observed palette
execution. Independently review/retype these, and annotate remaining palette
bytes Data. Lack of fetch alone is not the data proof; bounded consumers and
control flow are the independent evidence.

## Projectile seventh slot

Unknown initializer1B935..1B977 is66 bytes of direct continuation after selected
slot state becomes80. It copies player X/Y, four sprite/motion fields, sets timer
20 or40 depending mode, attributes1 and requests sound15. It does not choose a
separate seventh-slot role. Allocator still visits seven slots0200..0260 while
ordinary updater and collision traversal use six. No special seventh updater
was identified in the recovered connected code. Retain this exact discrepancy;
no bug, intended spare role or natural seven-projectile occupancy is asserted.
The initializer itself has strong Static code evidence independently of this
unresolved game-design question.

## Progression and experiments

Old stage-init rejection retrieved from intercept.py, stage-decode.json, prior
scenario records. Prior B1E2 intercept missed; pre-start edits were cleared and
active-index edits left dependent map state stale. New method derives C389 inline
clear contracts and intercepts real newgame path at C5CC AFTER those clears.
From play-0.snapshot, Start reaches C5CC naturally at frame160 in116674 complete
operations, effective banks12/13/14/15. Save pre-stage.snapshot; set049D=index,
049E=index%4, release buttons. Supported remaining initializer establishes lives,
flags and map state. For1,3,15 run360 frames idle and120 Up/A/B (30-frame bounded
calls). Distinct coherent terrain, enemies and bullets captured; RAM confirms
indices persist. Final-stage4-4 and fortress-gate combat are artificial visits,
not natural progression. stage-contexts.json contains accepted2-frame fetch
samples after each restored visit.

Endpoint test: restore stage15, set0079..007B=[0,1,F2],0064=70 and holdUp/A/B.
The intended fractional borrow/end condition was source-supported, but dependent
terrain/spawn state was not synchronized.120 frames later player had died and
respawned (lives3->2,4-4 intro). Continue720 total frames yielded coherent fortress
gate combat, captured endpoint-23.ppm.png; no ending/credits claim. This rules
out using raw scroll endpoint alone as a reliable boss/ending shortcut. Do not
repeat unchanged. Next improved method needs transition/object/terrain state
contract after independently classified batch; no arbitrary RAM sweep proposed.

## Data review and stopping frontier

Data-proposals.json records exact intervals, first/last bytes, role evidence and
conflict checks. Besides audio, practical consumer and record inspection covered
bank6 sprite pointer/count records8000..90F3, level pointer/command records
A000..A6C0, display text, initializers, control sequences and spawn records. Generic
Data is used where stronger individual schemas are not established. Bank6 residual
9B14..9F7C was separately inspected: pointer/sprite streams and FF-terminated
sequential tile rows. Adjacent9B04 PPU helper excluded and separately supported.
No unsupported whole-gap data claim covers neighboring executable fragments.

unknown-complement.json is exact current141-range complement,29363 bytes, with
per-range bank/CPU identities and proposal/unproposed counts. It remains current
because map was not changed. Every supported continuation of the selected graphs
was included up to explicit mapper/window or selector boundaries. Sparse dispatch
zero entries and selectors outside bounded objects do not justify arbitrary roots.
Remaining small fragments include isolated trampolines, table cells and routines
without a supported entry/extent or a sufficiently discriminating non-code pattern;
these are kept separate, not rejected through a global missing-caller rule. The
next disassembly should adjudicate this combined batch before new residual-based
inference, because accepted boundaries/consumer source will change substantially.
This is a ready-batch handoff, not terminal exhaustion or a claim all remaining
avenues are impossible. Broad gameplay ending remains unresolved, with the actual
failed shortcut and useful pre-stage snapshot retained.

No new toolkit issues. Existing NES-001 remains nonblocking. Disk checked before
research:3.4GiB free; selected captures/snapshots only, no continuous capture.
