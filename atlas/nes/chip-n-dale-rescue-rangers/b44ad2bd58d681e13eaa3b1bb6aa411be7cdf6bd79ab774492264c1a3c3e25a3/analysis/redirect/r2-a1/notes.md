# Redirect r2-a1 notes (input checkpoint pass-002, 262159/262160 classified)

## $F62C (phys 0x1F63C, fixed bank, 8 KiB PRG bank 15, byte $60)
Prior barrier (r1-a1 / pass-002): a legal RTS after JMP is not enough by itself; no word $F62C/$F62B was found.
Changed facts / new inferences (tools/rtsjmp.py, rtsjmp-scan.txt):
1. Dead carry setup. PlayerBump_No_Skip = `clc ; jmp PlayerHitSolidSide ; rts`. PlayerHitSolidSide ($F721)
   has exactly one entry, this JMP. On every path it overwrites C before any read: the loop exit
   `cpy #$10` then `clc`, or SolidSideOverlap/ObjDistanceX (`sec; sbc`, `bcs`, `clc/rts`) and ObjectContact
   after the carry-setting jsr. So the `clc` is a dead instruction. It only makes sense as the first half of
   an original "return false" epilogue `clc ; rts`, matching PlayerBump_Yes `sec ; rts` and the routine's
   carry contract (callers WalkRight/WalkLeft do `jsr PlayerBump ; bcc`). The JMP was inserted between the two
   instructions. The RTS is the remaining epilogue instruction, so it is code inside the routine body, not
   inserted data.
2. No data role is possible. Neighbours are Observed code on both sides ($F629 JMP, fetched; StandOnPartner
   at $F62D). There is no alignment, because $F62D is odd. Only one indexed absolute operand in the code
   map has a base inside $F52D-$F62C: AimDirectionTable `lda $F586,y`. Its index is Y*4+Temp04 with
   Y in 0..7 and Temp04 in 0..2, so at most 30, which bounds reads to $F586-$F5A3. No hi/lo split table
   holds $F6 with a matching $2B/$2C. No immediate #$F6 / #$2B-2C pair exists. The only PHA;RTS byte pairs
   in PRG (0x7625, 0x1FCA0) lie in Data regions.
3. Precedent scan: every other RTS that follows an unconditional transfer inside classified code (6 cases)
   is a branch target. All 51 untargeted post-JMP/RTS instruction starts are handler entries, mostly
   `lda abs,x` actor handlers reached through split tables, and none is a lone RTS. So no precedent for a
   *reachable* orphan RTS exists. Together with (1), reachability stays unclaimed: no caller, and the
   byte was not fetched in the r1 traces or in the 2P run below.
Conclusion: there is a defensible static code classification (unreachable epilogue remnant, Static, about
0.6, reachability not claimed). The fallback, if the reviewer rejects that, is 1-B filler data, by
analogy with the accepted $803F decision. Either choice closes the last unknown byte.

## Runtime: natural 2-player start (tools/twop.py, twop-run.txt, twop-cov.json)
Fresh emulator.start. Pad 1: start 2f, none 60, select 2, none 10, start 2, none 100. Then 10x (start on
both pads 2f, none 30), none 60. $50 PlayerCountMinus1 = 1 at frame 556, level $31 = 0, both player
objects active. Then pad 1 right/left with B taps, 6x(20+2+10) frames per direction, ending at frame 940.
No memory writes, so this is ordinary gameplay. It fetched PlayerBump_LiftPartner and the rest of
PlayerBump up to the JMP at $F629. $F62C was not fetched. 4586 instruction starts were fetched.
Every instruction start of 14 Static code regions (230 B) was fetched, and 6 more were partly fetched
(provenance-upgrade.txt).

## Other commentary questions
- Cart RAM at $6000. The header has byte 6 = $10 (no battery) and byte 8 = 0 (iNES 1.0, unspecified).
  The only code that touches $6000-$7FFF is DemoInputRecorder, which sets $60/$62/$64/$66 hi pointers.
  No absolute $6000-$7FFF operand exists in any code region. PlayDemoInput replays ROM streams. The
  board's WRAM cannot be decided from the image, and shipped gameplay does not need it. The recorder
  looks like a leftover development tool. Classification is unaffected.
- Callers for L-011/L-012 routines: r1 raw word/JSR scans were negative, and this pass found no
  RTS-trick or split-table route to them. No new method is available. Reachability stays unclaimed.
- Semantics (metasprites $23/$24/$25, second HUD counter, extra heart sprite, sound commands 5-7) and the
  28 base-offset operands: every addressed byte is already classified. The open points concern only names
  and table-extent labels, not code versus data. These are commentary or final-editorial items, deferred
  because they have no structural prospect.
