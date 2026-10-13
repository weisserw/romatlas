# Gameplay research

## Phase-2 audio findings — 2026-10-13

Static review identifies pulse1 effects, pulse2/wave music and loss noise, with
request names derived from game callers. UpdateAudio runs at VBlank independently
of foreground bonus-timer samples. Noise-triggered routing changes all channels;
music stereo places pulse2 left/right while pulse1/wave stay on both sides.
Tonal mute does not mute noise. Dormant sound-test key helpers alias DMA code
and have no established game route; they are not a shortcut to bonus/audio.
Existing stage-play replay remains the execution route; no new listening or
runtime result is claimed. See commentary/source/bank_001.asm by routine names.

## Phase-2 bonus/UI findings — 2026-10-13

Source-derived, not new emulator observations: TickBonusTimer samples frame
counter multiples of32 during active bonus play. Its values95/90/85/80 are
ticks, not seconds. GetBonusParameters selects awards500/700/1000/1500, with
later bonuses reusing the final record. Successful results display SPECIAL
BONUS and transfer ten points per step, then any residual single points. Bonus
score addition can wrap16 bits; ordinary brick points saturate instead. UI
renders four low decimal score digits plus a categorical upper-digit marker.
All current speed-display calls are no-op returns; no visible speed glyph was
established. See commentary/source/bank_000.asm and bank_001.asm by these names.
No new replay route is required for these literal static effects.

## Phase-2 static control findings — 2026-10-13

These are source-derived effects, not additional emulator observations. See
commentary/source/bank_000.asm (MovePaddle, InitializeBallServe,
SetPaddleRebound, ProbeBrickCollision, LoadNextScrollHitThreshold).
A-held selects five pixels per paddle update, B-held one, default three;
A wins simultaneous A/B. Left wins simultaneous Left/Right. Serial position
input has a separate clamped path. The inbound serve uses tier-1 diagonal
velocity; first rebound adopts the stored normal/bonus target tier. Ceiling
contact narrows an ordinary paddle while preserving its center; bonus play
does not narrow it. Bonus brick removal reports no reflection and consumes no
reserve ball on loss. Scrolling advancement counts paddle rebounds using a
table of thresholds (8,8,5,5,3,3,2,2,2,2 then implicit 1), not a frame delay.
The frame-based title/demo and previously recorded stage-play routes remain
the relevant replay inputs; no new route was executed for these static facts.

ROM header identifies ALLEY WAY; 32768 bytes, ROM-only controller, DMG. Header destination/revision bytes are both $00; exact external-booklet regional/version matching remains unconfirmed. External routes are hypotheses until confirmed locally.

Source: [US manual transcription, DMG-AW-USA-1](https://www.world-of-nintendo.com/manuals/game_boy/alleyway.shtml), consulted 2026-10-13. Start begins play; A releases the ball. Left/right move the paddle, with A accelerating and B slowing it. Start pauses/resumes ordinary play; Start+Select returns to title. These routes target title handling, launch, speed selection, pause and reset branches.

The manual describes normal, scrolling, descending and timed bonus patterns, repeated over 32 stages. Clearing bricks advances stages; bonus misses end the stage without losing a life. Every 1000 points can award a life within limits. These claims motivate later stage-loader, collision, bonus timer and life-counter investigations. No shortcut is established by this source. Version applicability beyond the US booklet is unconfirmed.

Initial replay is recorded in initial-requests.json: startup 1000000 operations; Start held 100000 then released for 100000; A held 100000 then released for 1000000. All runtime calls have 10000 ms timeout. Outcomes will be examined before claiming actual gameplay states.

Local confirmation in stage-play-results.json / frame_scenario.py stage-play: after22*10000 startup operations, hold Start2 frames/release2; state advances2 to4. Wait in10-frame increments until state5 (frame229), then hold A2/release2: state6 and moving ball coordinates FFB4-FFB7. Right2 frames moves FFC0 from40 to46; Left2 restores40. Start2/release2 changes6 to12 and freezes ball coordinates; repeating Start returns to6 and ball movement. B was sampled but no direction held with it, so its speed modifier remains a static-semantic/manual hypothesis. The routine10a4 selects paddle displacement based on input bits, supporting the manual's speed distinction without claiming runtime measurement.

The long stage-entry animation explains why early gameplay inputs were premature. Snapshots were not persisted; the route is fully replayable from reset. Source claims about later scrolling, descending, bonus and ending states are not yet exercised locally. Structural stage flags and layouts support their loaders but do not prove every gameplay outcome.

A targeted online search for an existing Alleyway disassembly did not locate an applicable original source or symbol set; no external source was used to classify the final zero reservation. Only manual claims above informed routes.

Pass 006 local confirmation: `frame_scenario.py zero-probe` adds Select+Start held for two frames and both released for two frames after normal play/pause/resume. FFA4 returns to 2 at frames 255 and 257, confirming the manual's return-to-title route in this ROM. The run also checks for cartridge-ROM reads/fetches in 0426-0445; none occur on this route with no trace drops.

New external technical lead: [ASMSchool index](http://gameboy.mongenel.com/asmschool.html), consulted 2026-10-13, mentions a planned Alleyway lesson but has no link to its source. Its linked Super Mario Land notes are not version-matched evidence for Alleyway. This did not change gameplay routes or classifications.
