# Round 2 — redirect handoff (attempt r2-a1)

**Recommendation: one short NEXT CYCLE (disassembly), low value, no byte gain.** It would apply lead L-019:
merge the provenance of 142 artificially observed opcode starts, and correct commentary using the area-clear
contract. The classification residual is L-018 (2 B). Its review is now exhausted, so the expected outcome after
that cycle is **terminal incomplete by 2 bytes**. If the coordinator prefers, L-019 can instead be folded into
final acceptance; the residual is the same either way.

## Scope and checkpoint
- Input checkpoint **pass-002**. analysis/state.json is identical to maps/current.json, and
  analysis/pass-002-verification.json shows byte_identical. Classification is 262,158 / 262,160 bytes.
- This phase changed no map, state, source or ROM; state.json is still identical to current.json. Research is in
  `analysis/redirect/r2-a1/` (tools/, observed/contexts.json, observed-merge.json, clear-results.txt,
  l018-review.json, frames/). The r1-a1 and r1-a2 records are unchanged.
- Runtime: `run.py serve` through analysis/tools/rpcd.py, using the configured binary. The server has been shut down.
  Disk: about 4.3 GB free; scratch is 1.4 MB, and the clr* frames were promoted to the attempt dir and removed from scratch.

## Proposed vs accepted bytes
- Proposed classification: **0 B.** Residual: file 0x1C5AE-0x1C5AF (bank 14 $C59E). This is unchanged.

## Findings
1. **L-018 re-examined** with wider scans and a new dataflow check; it stays unknown (rejection review below).
2. **Area-clear contract solved (L-003 → visited).** Bank 8 `AreaExitCheck` compares Batman with per-area exit tables
   (AreaExitFlags/XHi/YRow/YLo/XLo, indexed by $88>>1).
   - Boss areas (flag bit7) need BossDefeatFlag $CE bit7.
   - Flag bit5 delays the exit through a countdown in **$051F (ObjXLo+15)**. The opcode at file 0x113EA is `dec $051F`.
   - The death-reload start spot of each boss area is its exit spot. Batman drops from it into the arena.
3. Artificial clears observed (clear-results.txt, frames/):
   - **idx 17 → ending** ($88 $22→$24). ObjKind42Defeated stores 5 in $051F, which is the exit delay. This answers a commentary uncertainty.
   - **idx 16 → area 17** ($20→$22; kinds 42/44/43 spawn).
   - **idx 6 → area 7** ($0C→$0E), with correct graphics.
   - Why idx 16 did not clear in round 1: at X=305 the ledge is not solid after the boss scroll, so Batman falls to
     (6,$EF) and YLo<$B0 never holds. The exit position had to be rewritten every frame (artificial).
4. Coverage: there are 0 fetches in Data, Text or unknown bytes. 142 opcode starts fall in 17 Static Code regions
   (observed-merge.json; mostly bank-13 kind 22-25 handlers from the forced defeat of all four area-6 kinds). This is L-019.
5. **Commentary defect:** the AreaExitCheck header in bank08.asm says the countdown is in `$050F`. The operand is $051F. Fix this under L-019.

## Rejection review — L-018 (compact; artifact analysis/redirect/r2-a1/l018-review.json)
- **Claim tested:** that file 0x1C5AE-0x1C5AF (bank 14, fixed at $C000) is code (an alternate entry or leftover instruction)
  or generic data. The bytes decode as `LDA JoyPressed` and sit between `JMP ($0000)` (JumpTableDispatch) and HideUnusedSprites $C5A0.
- **Decisive evidence:**
  - Across all 16 PRG banks there is no reference: no word $C59E/$C59F, and no RTS-1 word $C59D. The single $C59D word hit is inside bank-6 MapNN area data, which is not a pointer table.
  - No LDA/LDX/LDY immediate pair forms #$C5 with #$9D/#$9E/#$9F.
  - No branch encoding targets it.
  - It is not fetched in any trace: pass-001, r1-a1, r1-a2 or r2-a1. The r2-a1 traces include a fresh GameFrameLoop run.
  - The bank-2 copy of the jump engine is followed directly by code, so there is no layout analogue for a 2-byte trailer.
  - **New check:** an entry at $C59E would load A, but HideUnusedSprites overwrites the flags (LDX/LDY) and A (LDA #$F4). A survives only when OamFree=0, and its only caller reloads A after the JSR.
- **What this rules out:** reachability through any static reference, and a *meaningful* alternate-entry contract.
  Accepted orphans (ObjKind9Fall_placeAt, SetupBossSlotsArea17, CsAnimateCommitFinish) all have a prefix whose result the
  fall-through consumes. This candidate does not, so the round-1 competitor "entry with A=joypad" is weakened.
- **What it does not rule out:** a dead leftover instruction (its operand is a real ZP variable, which suggests code origin) versus stale or filler bytes.
- **Strongest competing interpretation:** a dead remnant instruction from a removed code path.
- **Why no proposal is made:**
  - Code: a legal decode alone is not code, and there is no contract.
  - Generic data: the content is a plausible instruction, not uniform, repeated or non-instruction content.
  - So code versus data stays materially ambiguous, which is the policy condition for keeping bytes unknown.
- **What would reopen it:** an observed fetch or edge, a computed target producing $C59E/$C59D, or another build of the engine with this layout.
- **Consistency:** the same standard rejected unreferenced return opcodes and pointer-like sequences, and the round-2 orphans were accepted only with
  consumed dataflow. No prerequisite was added beyond the policy.

## Other avenues triaged (deferrals with reasons)
- **Bank-15 sound-format block $F4F1-$F7FF, D-SNDX, bank-3 period-16 records, duplicate tails of banks 4/8:** these are already
  classified Data, so no classification is at stake.
  - A raw word scan for pointers into $F4F1-$F7FF gives hundreds of coincidental hits in every bank ($F4-$F7 are common byte values), so it cannot discriminate.
  - No consumer or selector is known for these ranges.
  - Deferred: there is no discriminating method, and the result would be semantic only.
- **Natural (unpatched) return to exit ledges:** this needs skilled play or wall-jump routing. It is coverage only, already covered artificially, and has no byte gain. Deferred.
- **Orphan-code reachability** (bank-2 cutscene helpers, CsReadJoypadsAlt, QueuePpuBlock, etc.): reference scans in r1 found no
  caller. Nothing in this round's traces fetched them, and there is no new selector evidence. Their classification is already accepted. Deferred: negative evidence.
- **Progression checklist:** S1-S7 are now all visited, including the boss-area clears and the ending. No unvisited milestone remains.

## Leads
leads.json pass entry r2-a1:
- Added **L-019** (ready, low priority): observed-provenance merge plus two commentary corrections.
- Updated **L-018** (exhausted-unknown) and **L-003** (visited).
- scenarios.json S5 is updated.

## Toolkit
No toolkit issues found in this attempt (analysis/toolkit-issues.md unchanged).
