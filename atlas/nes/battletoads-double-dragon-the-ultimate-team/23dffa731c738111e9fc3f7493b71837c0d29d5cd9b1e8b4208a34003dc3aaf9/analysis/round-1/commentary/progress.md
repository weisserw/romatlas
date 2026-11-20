# Commentary progress

## How the commented source is maintained

source/game.asm is produced by `commentary/tools/annot.py build` from the frozen
before/game.asm plus the notes in commentary/notes/:
- symbols.txt: hardware registers, zero-page/RAM variables, object-table fields
  (15 slots, parallel arrays $03C2-$05B1), sound-driver RAM rows.
- b0_a..b0_e.txt (bank 0), b3.txt, b4.txt, b5.txt, b6.txt, b127.txt (banks 1/2/7):
  `@B:ADDR Name` labels, `|` block comments, `=B:ADDR` end-of-line comments,
  `?B:ADDR` notes for unclassified gaps, `%B` bank banners, `~B:S-E` ranges where
  object-field names are suppressed (RAM reused by menu code).
- stubs.txt / stubgaps.txt: the shared top-of-bank stubs and their gaps.
- extra.txt: late corrections.
Instruction bytes are never edited; operands are symbolized with the same values.
Unknown gaps get an automatic boundary note unless a `?` note overrides it.
Edit notes, then run `python3 commentary/tools/annot.py build`, the commentary
verify command and `python3 commentary/tools/fill_review.py` (refreshes review.json
units/sources). Helpers: `annot.py clist|facts|index`, tools/show.sh, tools/renote.py,
tools/gaps_report.py N (lists routines of >= N instructions without block comments).

## Round 1 status: COMPLETE (current-source readability)

- All 696 routine entry points in classified code (8 banks) have descriptive names;
  624 have block comments (every routine of 4+ instructions; the remaining ~70 are
  1-3 instruction helpers/entry stubs with self-describing names).
- Branch targets keep address-based labels L_Bb_AAAA (mechanical), except where named.
- RAM: ~70 zero-page/RAM symbols, 34 object fields, sound-driver rows.
- Every classified data region and unknown gap has a source note; review.json:
  1532/1532 scaffold entries status=reviewed with summaries and source lines,
  624 units. editorial_audit stays pending (final acceptance only).
- Verification: fresh byte-identical verify after the final build
  (commentary/verification.json); reading copy refreshed (reading-after.asm).

## Names / interpretations established this round
Main-thread/NMI coroutine (WaitForNmiThenResume, GameFrameLoop, Nmi_Common,
Nmi_GameFrame), far-call/bank-service stubs, stage loader (StartStageLoad +
bank 6 LoaderStep/LoaderFrame with 14 command pairs), object engine
(UpdateAllObjects, ObjTypeHandlers, MoveObjAtSpeed, ApplyGravity, FindAttackTarget,
ApplyHitReaction, animation scripts AdvanceAnimScript + AnimCommandTable $E0-$FE),
scoring (AddScore), joypad, sound driver (StartMusic, UpdateSound, script commands),
PCM sample player, background streaming (bank 5), camera (bank 4), sprite renderer
(bank 6), graphics decompressors (banks 1/7).

## Unresolved (see uncertainties.md)
Exact game meaning of many object fields (Obj_03E0, Obj_04B2, Obj_04DF, Obj_04EE,
Obj_051B partly, Obj_0539, Obj_0548, Obj_0575, Obj_0584, Obj_05A2, Obj_05B1),
which stage numbers correspond to which levels, the identity of the scripted
enemies, and the RAM-code origin ($0532, $0600, $0720/$0728/$0782).
