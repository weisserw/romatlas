# Round 4 - redirect handoff

Status: **complete**. All three optional questions are answered, and none changes classification, bank context or
region boundaries. Recommendation: **final acceptance**. One low-priority commentary lead, **L-013**, is ready to be
applied in the final-acceptance editorial step. No disassembly round is needed.

- Attempt dir: **analysis/redirect/r4-m1/**. It contains notes.md, run_r4.py, spin-panel-reads.json,
  ball-marker-reads.json, r4_spin.png, r4_mark0.png and r4_mark1.png.
- Input checkpoint: pass-004 (byte_identical, map sha256 35ed6454...).
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). Accepted bytes are unchanged and
  no bytes are proposed.
- Not modified: the map, analysis/state.json, the commentary source, input/game.nes and the toolkit.
- Server: started through the rpcd bridge (scratch /tmp/ra-2707-r4r). Only its own three PIDs were killed, and the
  scratch directory was removed.
- No RAM or register writes were used. All runtime evidence comes from natural input starting at the aim0 route.

## Outcomes
### (3) BallMarkerSprite bank context: confirmed bank 4 (static contract and natural trace)
- **Static contract.**
  - The only entry into DrawBallMarker is DrawBallSprite with Tmp09 != 0.
  - Tmp09 is set only by BuildSprites, and BuildSprites runs `jsr SelectBank4_Temp` first.
  - None of the sprite helpers in between writes a bank.
- **Runtime (natural input).** Select+A while aiming starts ball calling: GameCmd $10, CalledBall 10.
  - The ring marker is drawn (r4_mark0.png).
  - All 2079 reads of phys $13219-$14000 have prg_banks [8,9,14,15], which is 16 KiB bank 4.
- **Refinement.** $B209 is a one-entry set-pointer word, and its target record $B5D8-$B5E4 was read. The rest of the
  3560-byte block holds other metasprite records reached through the other bank-4 set pointers.
- **Classification impact: none.** The observed context equals the region's physical bank, and bank 4 holds only Data
  and Padding.

### (2) SET SPIN window attribute extent: confirmed 6 bytes (natural trace)
- Input: Select+B while aiming. The trace read exactly $9043-$909A: an 82-byte rectangle plus 6 attribute bytes. The
  following 32-byte uniform run $909B-$90BA was not read.
- **Classification impact: none.** Everything is inside the single Data region 998. The run's role is still unknown,
  but that is a question of meaning only.

### (1) Speed Pool total: elapsed seconds (static)
- GameFrame adds ShotClockTick seconds to PlayerScore, capped at 9999. The heading is TIME, and the unlock condition
  is a time below 141.
- ChallengeResult inverts only THREE BALL. Speed Pool's ascending thresholds therefore rank slower totals higher.
  This is a likely original-game quirk (a missing inversion); the intent cannot be proved from the ROM.
- A natural three-round clear is not needed.

### Bookkeeping
- **L-008** (bank-6 screen-script labels) was applied in round-2 commentary, but leads.json still said "ready". It is
  now marked verified.

## Leads produced / updated (analysis/leads.json)
- **L-013 (ready, low, commentary only).**
  1. SpinPanelAttrs is exactly 6 bytes (traced), and the run that follows is unread.
  2. BallMarkerSprite: bank 4 is confirmed at runtime, and it is a set-pointer word pointing to the record at
     $B5D8. A label for that record is optional.
  3. ChallengeResult: the Speed Pool total is time, and its ranking is non-inverted (likely a quirk).
  - No map change is required.
- **L-008:** marked verified (bookkeeping).
- **L-012:** history note added.
- A pass entry was added for round 4 redirect r4-m1.

## Gameplay (analysis/scenarios.json)
- **S22**: the SET SPIN panel, with a read trace.
- **S23**: call-shot ball selection (GameCmd $10) with the marker sprite.

Both were visited with natural input. Every major milestone was already visited (boot/credits, menus, every mode,
tournament rounds through World Championship, ending, match won/lost, Challenge end/EVALUATION). No later-game
milestone remains unvisited.

## Compact rejection / terminal review (evidence-policy.md)
1. **BallMarkerSprite bank-context claim.**
   - The claim is that the reader at fixed-bank DrawBallMarker reads phys $13219- with bank 4 mapped.
   - Decisive evidence: the single-entry static path through SelectBank4_Temp with no intervening bank write, and the
     natural trace in which every read was in bank-4 context.
   - This rules out a different bank being mapped during the read.
   - The strongest competing interpretation was another bank at $8000 during NMI sprite building. The trace rules it
     out for the observed calling mode.
   - Decision: keep. The region bank and kind are unchanged.
2. **Region 998 extent and role.**
   - Window rectangle + attributes = 88 bytes, traced. The rest is a generic tail plus menu-script pointers that
     already have readers ($90BB-).
   - Strongest competitor: code in the unread 32-byte run. It is unlikely: the run is uniform, has no flow target and
     no operand, and the split-immediate scan found no hit.
   - Decision: keep one Data region. A split would add a boundary without changing kind.
3. **Low-confidence kept code** ($D9B0, $DE1C, $97B1, $FB32, gap fills; r2-q1 review).
   - The premises are unchanged. The round-4 split-immediate scan found no reference, and no r4 read touched a Code
     region.
   - The sound review is reused; no revised test is indicated.
4. **Why no further redirect or disassembly step is useful.**
   - There are 0 unknown bytes, and no code/data conflict or bank-context discrepancy was found.
   - All three remaining questions are now answered.
   - The residual items are meaning only, and none can change classification:
     - the role of the unread $909B-$90BA run;
     - whether NameCharMap codes $6B-$7F are ever printed;
     - the developers' intent for the Speed Pool ranking;
     - natural (unpatched) reachability of the win/loss screens.
   - Deferred: a natural manual-shot table clear. It is impractical and would only show reachability.

## Recommendation
Go to **final acceptance** (editorial audit). Apply L-013 there as a commentary edit; it needs no new map checkpoint.
Do not run another full round.
