# Round 1 commentary handoff

STATUS: COMPLETE — current-source readability pass finished; fresh byte-identical
verification passed; reading copy refreshed.  (ROM classification unchanged: 99.69%,
814 bytes unknown; final editorial audit not run — reserved for archive acceptance.)

Input checkpoint: pass-001 (maps/pass-001.json; commentary/manifest.json map sha256 54a37984...).

## Scope covered
| Bank | Code B | Status |
|---|---|---|
| 15 (fixed) | 5383 | done: reset/NMI/IRQ, MMC3 banking + bank-call dispatchers, PPU/VRAM queue, RLE screens, palettes/fades, joypad, full sound engine |
| 14 (fixed) | 6545 | done: map/metatile loading, column streaming/scroll, metasprites, collision probes, pickups (carrots), death check, object physics, templates, animation |
| 05 | 3436 | done: tile overrides, door/warp records, keyed type handlers ($07-$13,$2D,$2F,$58,$5A,$5B), game-over screen |
| 07 | 178 | done: logo screen |
| 08 | 578 | done: title, story pages, text typer |
| 09 | 4931 | done: map reveal, stage palettes/CHR, bonus games, round-clear and get-ready screens |
| 10/11 | 780/6180 | done: object update loop, spawning, ObjectTypeHandlers (99 types), platforms, contacts, removal |
| 12 | 5843 | done: player control/physics/ladders/camera, player drawing, RoundState handlers, hidden stage select |
| 13 | 6707 | done: mallet/body contacts, platforms riding, tile breaking, HUD, effects, special object, behaviours A-F |
| data banks 0-4, 6, CHR, header | - | boundary/role notes |

All 19 unknown ranges carry boundary/lead notes; all 188 data blocks carry a note.

## Method / how to continue
- commentary/source/game.asm = output of `python3 commentary/tools/annotate.py` (frozen
  commentary/before/game.asm + commentary/notes/*.txt).  Edit notes, re-run, then
  `python3 commentary/tools/review.py` (rewrites review.json units from tools/index.json)
  and `python3 /root/romatlas/scripts/commentary.py verify .` / `read .`.
- Helpers: tools/view.py BANK (compact view), tools/addr.py BANK LO HI, tools/extra.py.

## Verification
`commentary.py verify` -> {"ok": true, "byte_identical": true}; source sha256 in
commentary/verification.json.  input/game.nes unchanged (sha256 ef10ad58...).
Reading copy: commentary/reading-after.asm.  review.json: 987/987 scaffold entries
reviewed, 1329 units; editorial_audit pending (by design).

## Examples
- `NmiHandler`, `NmiVramJob`, `PpuUpdateFlags` bit constants; `BankCall8000` call sites
  annotated `; pair 6 entry 5 -> RunRoundStateHandler`.
- Object code reads `ldy #OBJ_XVEL / lda (ObjPtr),y`; `ObjectTypeHandlers` table ->
  `ObjType_14` etc. with "Object update handler for type $14" headers.
- `CheckPlayerDeath` (fall/health -> Lives, Carrots halved, RoundState 1; goal 8 -> $80).

## Unresolved meanings
See commentary/uncertainties.md: object type identities, boss behaviours A-F, bonus-game
rules, special object $0510, StageSpecial handlers, per-song/per-stage data extents.
446 small helpers/secondary entry labels have descriptive names but no separate header.

## Structural questions for redirect (evidence-policy review needed)
1. Bank 13 $A024-$A025 (physical $1A034-$1A035): bytes $05,$03 are read as data by
   StageSelectLoop (`cmp $A024,y`, stage/round limits) but phase 1 classified them as code
   (decoded `ORA $03` after an unconditional `BNE`).  Propose reclassifying as data.
2. 17-byte unknowns at bank 5 $BCFE (phys $0BD0E) and bank 11 $A83B (phys $1684B): their
   addresses are loaded into Tmp5D/5E right before `JSR ApplyMotionScript`, the same
   4-byte-record/$FF format as the classified scripts nearby.  Candidate data.
3. 3-byte unknowns bank 9 $A0F0, $B833 (and $B490 area): SrcPtr targets of DrawMetasprite
   (metasprite headers) - candidate data.
4. Hidden stage/round select (pair 6 entry 8, $9FC8, no static caller): reachability
   unknown; useful gameplay shortcut if reachable; artificial call possible for testing.
5. Carrots = $042A (resolves L-004): set >=10 carrots to exercise the bonus games.
6. L-001 orphan fixed-bank blocks ($F411 456 B etc.) remain unexplained.

## Remaining gaps
None for current-source readability.  Not done (separate gates): final editorial audit,
whole-ROM classification (814 unknown bytes).

## Toolkit
New TK-005 (missing feature: generated NES source lacks symbolic branch/call targets);
workaround in place, not blocking.  See analysis/toolkit-issues.md.
