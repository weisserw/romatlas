# Round 3 redirect handoff (attempt r03-a1): READY-FOR-DISASSEMBLY

- Input checkpoint: pass-003 (maps/current.json == maps/pass-003.json == analysis/state.json;
  unchanged by this phase). 262,144 / 262,160 classified, 16 unknown bytes (all L-027).
- Research only. ROM, map, state and source are unchanged. Attempt dir: analysis/redirect/r03-a1/
  (notes.md = evidence; proposals.json = the batch, checked against pass-003: 0 conflicts).
- Proposed: 11 of 16 bytes (L-028). Residual if all accepted: 5 bytes (L-027 lone $60s).
  If the tier-B items are rejected: 8 bytes.

## Ready lead L-028 (analysis/leads.json)
| Range | Kind | Tier | Evidence |
|---|---|---|---|
| b6 $FE24-$FE2B (phys 0x37E34, 8) | Code, Static, uncalled | A | `jsr HideSprite($FE19) / stx OAMADDR / lda #$02` = the prefix of the sprite-clear + OAM-DMA idiom, which is complete (`8E 03 20 A9 02 8D 14 40`) in bank 0 Nmi_Common and bank 4 $C5F5 (after an identical sprite-clear loop). These bytes are code under both competing hypotheses (truncated leftover / overlapping entry). |
| b6 $FE2C (phys 0x37E3C, 1) | Data (generic) | B | `$8D` whose idiom operand `14 40` is missing; executed it would be `sta $00A2` using the `A2 00` of ldx #$00 at $FE2D (an arbitrary store into RunRamColumnCode's $9F-$A3 address bytes). Truncated opcode remnant; the overlapping instruction cannot be a map region. Reviewer may keep it unknown. |
| b0 $E536-$E537 (phys 0x06546, 2) | Code, Static, uncalled | B | `lda #$02` into TryLatchOntoPlayer ($E538 `sta $D1`, contract A = hit-box id; the caller does `lda #$03 / jsr $E538`). Competing frame-map tail is now unsupported: linked-state frames are $39/$3A/$3B (static writer bank 0 $B8FE-$B91D) and $5B (observed); observed meaningful entries are $E524-$E526 / $E52E-$E530; with Y = $5B the consumer reads the code byte $E546 every linked frame, so its index window is not a table extent. Not an exhaustive frame-domain proof. |

Recipe (one disassembly pass): verify the bytes in proposals.json; confirm the idiom with a byte
search for `8E 03 20 A9 02 8D 14 40`; read source around $B8F8-$B91D and MirrorPlayerAppearance;
annotate via map.annotate (Static, reachability unresolved). Runtime replay is not required;
analysis/redirect/r03-a1/latch.py reproduces the artificial latch (args: snapshot, slot, cycles,
'-', 'x'; env PL = player slot).

## Corrected premises / new inferences
- Round-3 disassembly premise "reads reaching $E536 would make it table data": the latch run shows
  the consumer reads classified code ($E546) for a normal linked frame, so consumer reads in this
  window do not discriminate. The question moves to the frame domain plus the entry contract.
- $FE24 "overlapping-entry vs leftover": the idiom match makes the first 8 bytes code either
  way; only the $8D byte remains a judgement.
- $9745: the "unused index-6 entry" alternative is excluded (second table index 6 would be $974A,
  the first table's index $0B entry).

## Rejection review (remaining 5 bytes, L-027, blocked)
| Range | Claim | Decisive evidence | What it rules out | Remaining reason |
|---|---|---|---|---|
| b0 $8D92 | dead rts vs pad | source: preceded by beq/bne pair that always leaves; followed by Observed CheckYDistance; refs_pass002.txt (no ref) | fall-through execution; any reference | both alternatives fit; no consumer or reference |
| b4 $B979 | dead rts vs pad | after `jmp $B987`; copy loop `lda $B97A,y` Y 0..$0C | membership in RamRoutineTemplate_B4 | same |
| b5 $ACFA | dead rts vs pad | after `jmp $AFB9` in GetTerrainHeight; SnapToTerrain starts $ACFB | fall-through; table $ACDA (ends $ACE0) | same |
| b6 $9745 | dead rts vs pad | after `jmp LoaderCommonTail`; StageNextTables read only for Y 7..$0B | unused table entry (no index-6 slot in the packed tables) | same |
| b6 $EFE5 | dead rts vs pad | after always-taken `bne $F00C`; MotionScript_B6_EFE6 pointer starts at $EFE6 | fall-through; script membership | same |
Strongest competing interpretation for each: one unread data/pad byte. Independent constraints
considered together (terminal predecessor, classified neighbours, no reader/reference, value $60):
they do not discriminate a dead rts from a pad byte, and the policy says an unreferenced return
opcode lacks structure on its own. No practical test discriminates: no caller, consumer, pointer
or runtime path exists to exercise. Reopen only with a new reference.

## Gameplay (analysis/scenarios.json)
- 2P Game B visited artificially ($16 := 2 on the 2P Game A snapshot): 171 extra bank-0 opcode
  addresses (player-vs-player paths); no unknown bytes executed (gameb.py / gameb_out.txt).
- Clinging-enemy latch visited artificially (latch.py). Natural spawn of an ObjType $3A that
  latches was not found (the Rider spawns it with Obj_0575 = $A1, which skips latching).
- All 7 levels, ending, post-ending exit, game over, 2P A/B are now visited; remaining
  'attempted' entries are invalid stage ids ($01, $07, $0A-$0D). No practical shortcut remains.

## Deferred
- Exhaustive linked-state player frame domain (would harden $E536-$E537): needs a full
  enumeration of player-state frame writers and animation scripts while Obj_0539 bit 6 is set;
  large and only firms up a 2-byte tier-B call.
- Server (/tmp/r03r.sock) shut down. Scratch 3.2 MB; ~4.2 GB free. No new toolkit issues.
