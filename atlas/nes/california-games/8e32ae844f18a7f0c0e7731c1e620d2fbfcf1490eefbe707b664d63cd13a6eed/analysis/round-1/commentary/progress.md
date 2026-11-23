# Commentary progress

## Round 1 commentary (2026-11-22) - input checkpoint maps/round01-final.json (sha256 b4a8fa61...)

Prepared with `scripts/commentary.py prepare` (byte-identical baseline, 2265 scaffold entries).

### How the source is produced
- `commentary/annotations.txt` is the durable hand-written commentary record (symbols, routine and
  branch labels, block/line comments, notes for data and unknown gaps).
- `analysis/tools/cmt_apply.py` builds `commentary/source/game.asm` from the frozen
  `commentary/before/game.asm` plus the annotations (bytes untouched; labels, exact-value symbol
  substitutions, comments, incbin splits only). It refuses to overwrite a hand-edited source
  (hash in `commentary/annotations.built`). Edit the annotations, then rebuild and verify.
- `analysis/tools/cmt_review.py` fills `review.json` (scaffold untouched) with status, summaries,
  source lines and routine units. Helpers: cmt_parse.py, cmt_xref.py, cmt_routines.py, cmt_dump.py.

### Scope covered (whole current source)
- All 579 code regions (24,686 bytes, 12,057 instructions): 458 routine/fragment entries labelled
  (all named; a few address-style names such as Disk_B1F8 where the role is unknown), 1,537 code labels total incl.
  generated branch labels (Loop/Skip/Return/Idle suffixes under the owning routine name).
- Every routine has a generated mechanical header (entry kind, fall-through, exit, calls, RAM
  reads/writes by symbol, hardware registers); 300 have hand-written prose blocks, the rest are
  named parts of larger routines described by label + header.
- 99 global RAM symbols, 38 event-scoped overlay symbols (skating, surfing, BMX, half pipe),
  hardware register and controller-mask constants, 27 table symbols, 8 BIT-skip alternate entries.
- All 902 data regions carry the phase-1 evidence class and the routines that reference them.
- All 784 unknown gaps carry a boundary line plus fill/reference notes; 14 gaps with concrete
  evidence (reset code, vector target, indirect-table targets) have explicit notes.
- review.json: all 2265 scaffold entries status=reviewed with summaries/sources; 876 code units.
  editorial_audit stays pending (final acceptance only).

### Key interpretations
Program architecture (NMI-driven frame handlers via FrameHandlerTable, IdleLoop), bank switching
(BankShadow, FarCall, CallBank1-7 BIT-skip chain, per-bank 00-07 bus-conflict tables and $8008 jump
tables), sound driver (8 slots, SongTable/SongTempoTable/SndCommandTable), text-screen/menu system
(BuildTextScreen, MenuFrame, MenuActionTable), registration and results, and the six events mapped to
code (half pipe/foot bag/flying disk/BMX mainly bank 0; skating/surfing mainly bank 7).

### Verification
`commentary.py verify`: ok, byte_identical true, source game.asm sha256 c0b3595f62bc45f1... (analysis/round01-commentary-verification.json).
Reading copy refreshed with `commentary.py read` -> commentary/reading-after.asm.

### Deferred
Event-internal variables outside the scoped sets remain numeric with mechanical descriptions;
branch labels are structural. See uncertainties.md and analysis/handoffs/round-01-commentary.md.
