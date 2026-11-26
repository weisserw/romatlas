# Round 4 redirect handoff

**Recommendation: another disassembly/commentary cycle.** Two leads are ready:
- **L-019** (provenance, 3 bytes, plus one evidence-text note);
- **L-020** (commentary).

The cycle is small. If both leads are accepted and nothing new comes up, the next redirect can reuse the
dispositions below for a terminal review.

## Scope and identity

- **Input:** checkpoint **pass-004** (512 regions) and the round-4 commentary (byte-identical).
- **Classification:** 100.00% (65552/65552), 0 unknown, so the unknown complement is empty.
- **Proposed, not yet accepted:** region 49 ($8A2D, 3 bytes) Static -> Observed. Code Observed would go
  11917 -> 11920 and Static 1942 -> 1939. No boundary or kind change.
- **Not modified:** map, state, commentary source, ROM, toolkit, binary.
- **Attempt directory:** analysis/redirect/r4-a1/ (README.md, tools/, evidence/; 0.24 MB).
- **Cleanup:** the server was stopped and the scratch directory removed.

## Findings

1. **E17: natural left gutter.**
   - The round-3 premise "left aim clamped above $C0" was wrong. Aiming stops at $C4, and HookCurve0-3 then
     drift the ball left.
   - Run: snap_lane1, Left held 110 frames, spin 0, power 16. LaneBallX was clamped at $C0, and the traced
     replay covers $8A2D.
   - Evidence: r4-a1/evidence/e17_leftgutter_coverage.json, gain_r4.json.
2. **Indexed and indirect store census** (indexed_stores.json).
   - None of the 72 reachable indexed or indirect stores can set the gates AttrRowPending, VramReadMode,
     FillPending, WaitSecondCount or SpriteCycleMode 1-$EF. This closes the r3-a1 gate_writers caveat.
   - Every reachable StripMode store writes 1, so the NmiCopyStrip column mode is dormant.
   - Result: 264 bytes of gated jobs are dormant by data flow. They are still code.
3. **Constant-flag branch** (residual_preds.json).
   - FillNametable2/3 ($F656, 6 bytes) have no reference.
   - Their only predecessor is the `bne` after `ldx #$24`, which is always taken.
   - So they are unreachable from the vectors and still code. The r3-a1 closure is conservative here.
4. **Sign and meter labels** (E18, natural).
   - Bowler 2's sign reads "2PLAY".
   - SignGirlTiles' first tile is the bowler digit ($84/$94/$95/$96 = 1-4 in CHR bank 2, sprite half $0000;
     e18_sign2.png, e18_sign_tiles.txt).
   - The arc gauge with the spin marker is labelled CONTROL on screen.
   - This resolves two open uncertainties.

## Ready leads (analysis/leads.json)

- **L-019 (disassembly):**
  - Re-decode region 49 against e17 coverage and set it Observed.
  - Add a region 315 evidence note: "unreachable; preceding bne always taken".
- **L-020 (commentary):**
  - left gutter clamp in UpdateLaneBall;
  - sign digit tiles;
  - CONTROL label;
  - dormant gated jobs;
  - FillNametable2/3 unreachable;
  - update uncertainties.md.

## Remaining residuals (478 reachable Static bytes after L-019) and dispositions

Static code is a structural claim already supported by the closure and the contracts. All of these
residuals are provenance only.

| Group | Bytes | Disposition |
|---|---|---|
| NMI/library jobs gated by $54/$46/$2B/$5D/$6C, plus the StripMode column ($F2E6, $F319, $F330, $F4D8, $F520, $F588, $FDD4, $FE4C) | 264 | Rejected for natural coverage. Negative evidence: no reachable direct store (r3 gate_writers.json) or indexed store (r4 indexed_stores.json) opens these gates. Observing them would require debugger mutation, which proves nothing about natural behavior. |
| FillNametable2/3 $F656 | 6 | Unreachable (S2). Commentary/evidence note only. |
| Fade variants ($F938, $F982, $FA22, $FAB8, $FB0D) | 160 | Deferred for reduced prospect. These are FadeDelay/group-mask combinations; callers are known (residual_preds.json). Natural coverage would need a caller-by-caller FadeDelay data-flow trace, which gives provenance only. |
| DrawObject $FD list jump $FB8F, top-edge clipping $FC23 | 29 | Deferred. Data-dependent: needs a sprite list with $FD, or an object above Y 0 in a reachable scene. None was seen in all the natural play so far (S1-S32). |
| Page-crossing carries ($AAF7, $AB07, $BDEB, $E267) | 8 | Deferred. Each needs a specific buffer or pointer crossing a page; these values come from fixed addresses. Provenance only. |
| Sound envelope restart $E2BE, attenuation clamp $E3D4 | 6 | Deferred. Depends on the stream data. Provenance only. |
| $A92B clamp (`cmp #$0a`), $A84E impossible open-10th jmp | 5 | $A84E is impossible by the frame logic (r3-a1). $A92B needs an out-of-range value; deferred. |
| L-001/L-002 (unreachable) | 1461 | Resolved-unreachable (r3-a1 closure.json). Unchanged. |
| L-010 sprite-record splitting | - | Formatting only; left to final editorial. |

Other open uncertainties are semantic, carry no structural question, and are deferred:
- character handedness;
- ball weight units;
- names of the selection objects;
- portrait pose B;
- UnrefTilePairs and UnusedBestScoreData;
- the ClearScores ,y slip;
- sound id names.

## Gameplay milestone review (S32)

- The game has no levels, bosses or credits.
- All terminal screens have been visited (results, three awards, hi-score ranks, return to title).
- Pause is unavailable (S30).
- New this round: the natural left gutter (E17) and the bowler-2 announcement (E18b).
- No practical milestone remains unvisited.

## Toolkit

No new issues; TK-001..TK-005 kept.

## Markdown files created this phase

- this file
- analysis/redirect/r4-a1/README.md
