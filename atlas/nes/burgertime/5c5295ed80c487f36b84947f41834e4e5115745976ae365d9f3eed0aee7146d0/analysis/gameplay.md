# BurgerTime (NES) familiarization, round 1
Local title and license text identify BurgerTime, Data East/Nintendo. NROM-128,
NTSC debugger; exact marketed region/revision remains unverified.

NES manual search result: https://www.neshq.com/games/b/burgertime/burgertime-man01.pdf
and https://www.nesfiles.com/NES/BurgerTime/BurgerTime.pdf. Direction pad moves
Peter; Select chooses one/two players; Start begins/pauses. Walk across ingredients
to drop them. Search extraction's A/B line says “punch” and may be extraction
noise; do not adopt it as confirmed. Direct manual fetch timed out. Search also
found NES guide https://gamefaqs.gamespot.com/nes/587165-burgertime/faqs/39347,
but direct fetch was restricted. No ROM/assets downloaded.

Local confirmation: title reached around frame132, title menu frame179. A Start
press at frames132-134 was too early; second Start at179-181 began game. Player1
READY at241; first layout rendered at361. Rights/A held241-361; initial ready
sequence consumed much of that interval, so do not claim continuous player motion.
A/B pepper mechanism independently supported by controller/pepper code $D1C1.

Progression contract recovered locally: $0400/$0401 are total stage counters per
player; $1B/$23 select layout1..6; $0402 holds zero-based cyclic layout and $0403
holds min(stage-1,5) difficulty column. Stage completion increments $1B/$23 and
wraps7 to1 ($C2AC), while $0400 increases at $C836. Thus a distinct ending/credits
is not established; six-layout loop is statically supported. No separate boss
system found in this bounded pass.

Artificial shortcut: restore active.snapshot, release input, complete one CPU
operation; RAM $0400=N-1, $1B=N, $51=0, $31=0, $16=6; PC=$FD4F, stack=$FF.
Main dispatcher has JMP contract, so discard old nested wait return frames and
re-enter supported stage-initialization handler6. Effective banks [0,1,0,1].
Allow about600 frames for READY/initialization; use trace-disabled settling if
needed, then enable tracing and exercise movement/pepper for30-35 frames.
Sixth layout successfully rendered: stage6-after-wait.ppm, stage values6/1/5/5.
This is artificial reachability, not proof a natural play route selected it.
The earlier completion-flag patch during initialization failed; see scenarios.
