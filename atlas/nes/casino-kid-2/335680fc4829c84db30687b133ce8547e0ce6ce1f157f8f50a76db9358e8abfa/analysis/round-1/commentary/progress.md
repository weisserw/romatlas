# Commentary progress

## Round 1 commentary (2026-11-23)

Prepared from checkpoint pass-001 (maps/current.json == analysis/state.json, sha256 78c994c3...1dcdd);
frozen before/ and reading-before.asm untouched.

Method: commentary/source/game.asm is produced from before/game.asm plus hand-written annotation
files by `analysis/tools/round01-commentary/build.sh` (gen.py; annotations in ann/bank0-7.txt and
ann/ram.txt). source/ is the maintained deliverable; re-running build.sh regenerates it from the
annotations (do not hand-edit game.asm without also updating ann/, or stop using the generator).
review.json is filled by review.py (scaffold preserved).

Scope covered: all 1,414 phase-1 code regions (17,713 instructions, banks 0-7) and all data,
padding and unknown ranges.
- Every routine entry (573 discovered entries plus API/bank-entry slots) has a descriptive label and
  a header: hand-written purpose text plus generated Calls/Reads/Writes/Updates/Hardware lines.
  No address-only routine labels remain; branch targets get <Routine>_Loop/_Next/_Return names.
- 402 RAM/hardware symbols (symbols.asm): system/library zero page, OAM/VRAM buffers, game state
  (StakeTier, GameOutcome, PlayerMoney, SelectedOpponent, OpponentResultBits), sound engine arrays,
  bank-scoped names (BjVar/PkrVar/RltVar/SndVar...) for mode-local RAM whose meaning is not decoded.
  All RAM operands are symbolic; buffer offsets use Base+n.
- Inline parameters rendered as .byte/.word with symbols: SaveZpBlock/RestoreZpBlock (count, address),
  multi-byte math (width, operand A, operand B), bank-3/5 clear/copy helpers; 13 dispatch tables
  rendered as .word label lists.
- Data notes: consumer list for referenced data, generic-data note otherwise, 17 named fixed-bank
  tables; every UNKNOWN range has an explicit boundary note.
- review.json: 2,786 scaffold entries reviewed (summary + source line), 1,458 routine units.
  editorial_audit left pending (final acceptance only).

Depth by area: fixed bank (system library, text printer, menus, NMI, world map) read instruction by
instruction; bank 2 (title/attract), bank 6 (sound engine) and main game flows of banks 3/4/5 read
at routine level; inner game routines of banks 3/4/5 named from call/RAM summaries plus targeted
reads (tentative wording, see uncertainties.md).

Names/interpretations established: see handoff analysis/handoffs/round-01-commentary.md.
Workspace tool fix: analysis/tools/round01/m6502.py names $EA as "dex" (should be nop); worked around
in the commentary parser only (round01 tool left unchanged as a historical record).

Verification: `commentary.py verify` ok, byte_identical true (commentary/verification.json);
reading copy refreshed (commentary/reading-after.asm).
