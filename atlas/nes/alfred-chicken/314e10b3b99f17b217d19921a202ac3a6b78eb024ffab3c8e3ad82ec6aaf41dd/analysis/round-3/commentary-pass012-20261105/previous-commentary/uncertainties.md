# Round 1 commentary uncertainties

All current classified instructions were read and named/commented in this phase. The remaining uncertainties below concern game meaning, unclassified continuation bytes or unproven data formats; they are not skipped source ranges. Physical offsets include the iNES header. Prose references to bank0..bank6 in routine comments mean the 16KiB mapper register; explicit execution-view comments list physical 8KiB bank indices for the four CPU slots.

Actor parameters A..G are reused by handler families. Source names deliberately identify the array field while nearby routine notes explain its local use (packed timer, phase, lifetime, saved state, persistence ID or moving-platform delta). The two projectile slots use interleaved zero-page fields. Scratch/work aliases denote storage shared by unrelated routines and do not imply one universal type. Audio channel records are named by established bytecode/APU mechanics, not by instrument identities.

Flight wave and substate scripts (`source/game.asm:2944` onward; physical $12DDF..$1364F) establish timed records, state dispatch and actor groups. Their natural selectors, complete stream bounds and terminal/boss conditions remain unresolved. Source-backed shortcuts were artificial scenarios. Redirect can follow the late-flight snapshot's wave/substate transitions and descriptor consumers; no PC injection or arbitrary RAM scan is needed.

Map/graphics resources remain partially unknown. The 209 source gaps retain exact bytes and explicit uncertainty notes; the generic patterned data slices retain prior consumer/pattern evidence without an invented asset schema. The compressed decoder is explained at `source/game.asm:20169`, but that format alone does not prove whole resource bounds. Redirect should connect source/destination contracts to bank0 mixed intervals and bank1/2 graphics before extending structural claims.

Actor state names describe established movement/contact/animation mechanics. Visual species, original author names and some reward/effect identities remain unknown. For example the balloon particle interpretation is tentative; ID/state $50's tile-$D9 special case is stated mechanically. Contact routines whose bodies end at unknown gaps do not claim the missing response was reviewed or classified.

Player transitions are known mechanically from source and saved experiments, while later boss/ending semantics remain open. The stage/message/flight selectors and persistence flags should be investigated together; the existing gameplay guide's hidden-can/fifth-stage claim is an external hypothesis. Do not infer an ending from a selector write or successful transition alone.

ROM reads and calls whose targets remain unclassified keep their numeric CPU aliases. Named target operands were resolved only when all saved effective-bank contexts agreed on a physical instruction start. This preserves uncertainty rather than inventing target routines. The original maps/state and classification are unchanged.

## Round 2 limits and redirect questions

All currently classified bytes now have readable source explanations. No unknown range was reclassified by commentary. New resource extents come from accepted consumer/format evidence, not natural selector coverage. Behavior and persistence row prefixes remain partial; failed whole-height interpretations do not invalidate accepted bounded prefixes. Named Backing labels identify local physical emission positions, not bank-independent CPU addresses.

- Actor16 platform uses a synthetic return to CPU96E8 plus saved coordinate bytes; its three substates are closed, while unrelated actor substates still need individual domains and stack contracts.
- Encounter value/cursor/timer names describe mechanics. Exact encounter identity, normal reachability and every audiovisual meaning remain uncertain. Artificial boss/stage/quest-over routes are not natural-clear evidence.
- Bank5 graphics gaps near physicalACE7/B03D/B54A need concrete entry/stream-boundary evidence before additional code claims.
- Fixed collision/persistence tails need explicit return contracts and operand/table barriers. Caller JSR alone does not justify fallthrough where helpers unwind the stack.
- Audio grammar differs by channel, notably triangle F6 gate length. Rest loops and container markers have structural bounds without proof of natural selection.
