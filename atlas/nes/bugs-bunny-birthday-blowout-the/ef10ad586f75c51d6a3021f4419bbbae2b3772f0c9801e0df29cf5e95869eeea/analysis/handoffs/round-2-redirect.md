# Round 2 redirect handoff (attempt r2-b3d9)

STATUS: INCOMPLETE. No ready classification leads. The 3-byte residual (L-015) is terminal
after the rejection review below. The boss milestone (L-005/S8) is now visited.

## Scope and checkpoint
- Input: checkpoint pass-002 (maps/pass-002.json = analysis/state.json, map sha256
  88ab22c6...; verified ok/byte_identical in the disassembly round), the round-2
  commentary handoff, and leads.json.
- This redirect made no changes to the map, source, ROM or binary. input/game.nes sha256
  is still ef10ad58....
- `analysis.unknown` was re-run: 2 ranges, 3 B (0x1C44F+2, 0x1C597+1). Classification is
  100.00% rounded, 262157/262160. Proposed 0 B, accepted 0 B.
- Attempt directory: analysis/redirect/r2-b3d9/. evidence.json there holds the unknown
  ledger, the checks and the tool index. Scratch is /tmp/rb2 (31 MB: boss-room snapshots
  and logs; disposable).

## Findings
1. **Bosses visited (L-005 verified, S8 visited; artificial).** Each boss room is a
   separate warp area behind a drop-through door: a round-4 WarpRecordTable record with
   scene 6..$0B. EnterDoorOrWarp matches Bugs' cell *after* the door animation, which
   moves him down 2 cells.
   - Recipe: goto(stage,3) via $0420/$0421/$043F=8. Write $0460-$0467 =
     [0,0,0,0,0,$0F,$EF,$0F]; this widens the area bounds so the player clamp keeps the
     patched X. Then write PlayerX = srcX*16+8, PlayerY = srcY*16-$20, $043F = 2, and
     step 180 frames.
   - Doors: 1-4 (7c,04), 2-4 (34,03), 3-4 (2d,0b), 4-4 (ec,0b), 5-4 (6d,1a), 6-4 (6d,0b).
   - Only internal RAM was patched, so TK-006 was not involved.
   - Results:
     - All six bosses spawned through the game's own SpawnTable: types $07, $08, $0A, $0E,
       $10/$11 and $12.
     - 1-4 and 2-4 were defeated. Their $58 controller became the $57 goal and RoundState
       reached 8. In 1-4 it continued to $80 and on to Stage 2 Round 1.
     - In 3-4 and 4-4 Bugs died early, even with health refills.
     - In 5-4 and 6-4 the bosses were fought for 1500 frames each without being defeated.
   - Coverage: 1375 newly observed opcode offsets, all inside existing Static Code regions
     (74 regions). None fell in Data or unknown bytes, so they confirm the static
     classification. This is an artificial scenario, so it says nothing about natural
     reachability. Evidence: r2-b3d9/boss-coverage.json and boss-*.png.
2. **L-015 runtime result.** Every bus read of $C43F (42) and $C587 (630) happens exactly
   1 CPU cycle after the opcode fetch of the preceding RTS ($C43E / $C586). That is the
   6502 RTS dummy read of PC+1, not a consumer. $C440 is never read. No residual byte was
   ever fetched as an opcode. Evidence: residual-rts-dummy-reads.json, tools/rtsdummy.py.
3. **L-015 static result.**
   - Relative branches in bank 14 (decoded and raw-byte scan): none reach the residuals.
   - A word scan of all PRG, code and data, for $C43E-$C440 and $C586-$C587 found nothing.
   - The game has no split lo/hi dispatch tables: all 5 `JMP (ind)` sites load from word
     tables.
4. **Orphan reachability (new L-016, deferred).** No opcode fetch was seen in any pass-2
   orphan range (L-009..L-013) or in the stage select at $9FC8. The check covered 11837
   observed offsets from round 1, r1-a7c2 and r2-b3d9. Every BankCall site uses an
   immediate selector, and none is $68. This is a reachability-only question;
   classification is unaffected.

## Ready leads
None.

## Deferred / rejected
- L-016 (orphan reachability): deferred. There is no static caller or selector to test,
  and further gameplay sweeps would be arbitrary.
- S10 mid-boss rooms (round 2; doors 1-2 (7c,07), 2-2 (60,03), 3-2 (45,03), 4-2 (7c,14),
  5-2 (7d,0b), 6-2 (ee,0b)): deferred. They would add coverage only, and the warp method
  is already shown to work. The recipe is in scenarios.json.
- L-006 (semantics of generic data banks): readability only, for a later commentary pass.
- The ending (S5) was already visited in round 1.

## Rejection review (evidence-policy.md), L-015
**$C43F-$C440 `01 00` (0x1C44F-0x1C450, fixed PRG14)**
- Candidate claims:
  - (a) Code: an alternate entry `ORA ($00,X)` that falls into PutMetatileInColumn.
  - (b) Generic data.
- Decisive evidence:
  - The bytes sit after an unconditional RTS ($C43E) and before $C441, which is entered
    only by JSR.
  - No branch, word, word-1, immediate-pair, split-table or indexed operand reaches them.
  - At runtime, the only access is the RTS dummy read; there are no fetches across all
    recorded scenarios, including bosses.
- What this rules out: a referenced entry or consumer in ordinary form, and any observed
  use. It does not rule out an unreferenced leftover of either kind.
- Independent constraints, taken together:
  - The code reading would OR a zero-page pointer byte into A. A is the metatile index
    that PutMetatileInColumn shifts immediately, so the reading is semantically
    incoherent. That is weak evidence against code.
  - L-009 was accepted because its prefix wrote a variable the routine reads. No such
    contract exists here.
  - The data reading has no consumer, no format, and no uniform or non-instruction
    pattern: two bytes, both valid opcodes or operands.
- Strongest competitor: generic 2-byte data, for example a leftover from an edited
  instruction.
- Remaining reason: the policy allows generic data where non-code is reasonably clear.
  Here both readings rest only on absence of use, and the policy says lack of execution
  does not prove data. Code versus data stays materially ambiguous.
- Revised test considered: none discriminates. Static reference search, data-read watch,
  fetch coverage and twin/byte-pattern search (no duplicate of the sequence) are all
  exhausted.

**$C587 `60` (0x1C597)**
- Candidate claims: (a) a stub RTS (code); (b) a data byte.
- Evidence: it follows AttributeIndex's RTS and precedes ApplyCameraScroll (Observed).
  There is no reference of any kind, and runtime shows only the RTS dummy read.
- Consistency check: the only other RTS,RTS pair inside classified code, $AD5E in bank 11,
  is a word target in ObjectTypeHandlers. The code standard therefore requires a
  reference, and $C587 has none. This is not a conflicting acceptance criterion.
- The policy names an unreferenced lone RTS as insufficient structure. A single byte
  cannot show a data pattern either.
- Remaining reason: no structure-bearing constraint exists. It stays unknown.

**Conclusion:** none of the methods tried produced a corrected inference or a new
discriminating test, so the 3 B residual is terminal. Reopen only if a consumer or caller
of these addresses is found.

## Toolkit
- No new toolkit issues.
- TK-002 (tracing is about 6 frames/s) limited the boss fights to 1500-2000 frames each.
- TK-006 was not needed: the door-warp shortcut patches internal RAM only.
