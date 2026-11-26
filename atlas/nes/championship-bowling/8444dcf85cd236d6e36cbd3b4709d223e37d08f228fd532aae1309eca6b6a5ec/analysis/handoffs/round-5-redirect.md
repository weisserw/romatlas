# Round 5 redirect handoff

**Recommendation: another disassembly/commentary cycle.** It is small. Two leads are ready:
- **L-021:** 2 bytes of provenance plus evidence text for 188 more bytes;
- **L-022:** commentary.

If both are accepted, the next redirect can go **terminal**, reusing the rejection review below. Only one
provenance question is left open (top-edge clip, 15 B), and it has no discriminating test.

## Scope and identity

- **Input:** checkpoint **pass-005** (512 regions, map sha256 951ffb73...) and the round-5 commentary
  (byte-identical).
- **Classification:** 100.00% (65552/65552); the unknown complement is empty.
- **Proposed, not accepted:** region 386 ($A92B, 2 bytes) Static -> Observed. Code Observed would go 11920 -> 11922
  and Static 1939 -> 1937. No kind or boundary change.
- **Not modified:** map, state, source, ROM, toolkit (TK-001..TK-005 kept) and binary.
- **Attempt directory:** analysis/redirect/r5-a1/ (README.md, tools/, evidence/; 0.3 MB).
- **Cleanup:** the server was stopped and the scratch directory removed.

## Review of the round-4 dispositions

Round 4 split the 472 reachable Static bytes (after L-019) into groups. Rechecked against primary evidence:

| Group (bytes) | Round-4 disposition | Premise check | Round-5 result |
|---|---|---|---|
| $A92B clamp (2) | "needs an out-of-range value", deferred | **Wrong premise.** ScoreBall @firstMiss stores $D0; nibble $D >= $0A is normal for an open frame after a missed first ball | **E19 natural: covered** -> L-021 |
| Fade variants $F938/$F982/$FAB8/$FA22/$FB0D (160) | reduced prospect; "needs a caller-by-caller FadeDelay trace" | The trace was cheap and decisive. It was a missing analysis, not a missing prerequisite | **Dormant by data flow** (fadedelay_flow.json, fade_closures.json): FadeDelay is $FF only inside AnnounceBowler, whose two FadeInSpriteGroups calls take the instant path; BrightenGroups and TakeNextFadeGroup have only dormant or unreachable callers; nothing in the $FF window or the NMI reaches FadeOutAll; BrightenColor's cap needs nibble 3, but FadeInAll brightens twice from nibble 0 |
| Sound $E2BE / $E3D4 (6), plus $E267 carry (2) | "depends on stream data", deferred | Stream data is fixed ROM content, so it can be checked statically | **Dormant by ROM data:** all 41 envelopes end with a $FF hold; no pointer low byte is $FF; simulating all 64 headers (initial SndDuration 0 and 255) never takes attenuation below the envelope value (clamp_sim*.json) |
| Carries $AAF7/$AB07/$BDEB (6) | "specific page crossing", deferred | The bases and counts are constants | **Impossible by arithmetic:** digit pointer low byte <= $92; StripPpuAddr low byte is always in $x6-$x8 (residual_static.json) |
| DrawObject $FD jump $FB8F (14) | "needs a sprite list with $FD", deferred | Checkable against the data | **Dormant by data:** none of the 7 $FD bytes in sprite/pair data precedes a ROM address. Scope: lists in ROM only; no RAM-built list is known |
| DrawObject top-edge clip $FC23 (15) | data-dependent, deferred | Correct: depends on runtime object Y and record offsets; no constant bounds it | Unchanged; see rejection review |
| Gated NMI jobs (264), FillNametable2/3 (6), $A84E (3), L-001/L-002 (1461) | dormant / unreachable / impossible | Rechecked r3/r4 artifacts; premises hold (r4 indexed_stores census, residual_preds, r3 closure) | Unchanged |

The 472 bytes now break down as follows. Total: 472.
- Observed (proposed): 2
- Dormant by data flow or data: 264 + 160 + 8 + 14 = 446
- Impossible: 6 + 3 = 9
- Open: 15

## Ready leads (analysis/leads.json)

- **L-021 (disassembly):**
  - Set region 386 Observed against e19_open_miss_coverage.json (natural, no RAM writes).
  - Evidence-text notes on regions 405, 407, 333, 335, 337, 270, 272, 276, 166, 168, 403 and 340. No kind or
    provenance change for these.
- **L-022 (commentary):** a one-line note at each site, plus the header paragraph and uncertainties.md.

## Commentary uncertainties: discriminating-test review

| Item | Cheap test? | Disposition |
|---|---|---|
| Character 0 handedness | No. Only the art can be judged; ROM data cannot say whether the bowler is "left-handed" | Semantic, deferred (no structural question) |
| "PND" weight units | No. The on-screen text is already the evidence | Keep |
| Celebration and selection-object names | Possible, by matching ObjTable slots to the screen. The yield would be names only | Optional editorial work; not needed |
| Portrait pose B | Screenshots would only show the expression; the name stays interpretive | Deferred, semantic |
| UnrefTilePairs, ScorecardData head, UnusedBestScoreData | No reference exists (r2 label-ref search), so there is no consumer to test | Generic data is already sufficient |
| ClearScores ,y slip | Static: no visible effect, because totals are recomputed | Keep as a comment |
| Ignored 4-frame B press | A single anecdote with no structural relevance | Deferred |
| Sound id meanings | Partly named via call sites; the rest are not requested by the game | Keep |

## Rejection review (for the terminal decision after L-021/L-022)

**1. $FC23, DrawObject top-edge clip (15 B; Code, Static, reachable).**
- **Claim:** the clip executes in natural play.
- **Decisive evidence:** none in either direction. Natural runs S1-S33 never covered it. Unlike the other groups,
  no constant or table bounds ObjY + dy.
- **What the evidence rules out:** nothing. The absence of coverage is not negative evidence.
- **Competing interpretation:** a defensive clip for objects that enter from above, which this game never uses.
- **Why there is no test:** it would need an object list with dy below ObjY on a visible frame. Forcing that by RAM
  edits would prove nothing about natural play.
- **Remaining reason:** reduced prospect. This is provenance only; the structure (Code) is already supported.

**2. Gated NMI jobs (264 B).**
- Negative evidence: r3 gate_writers.json and r4 indexed_stores.json.
- Scope: every reachable store. This holds unless a store is missed by the linear decode of the Code regions. The map
  is complete, so no store is missed.

**3. Fade, sound, carry and $FD residuals (188 B).** Negative evidence from this round (see the table above).
- Scope for $FD: ROM lists only.
- Scope for sound: the stream grammar as implemented in SoundCmdDispatch.

**4. L-001/L-002 (1461 B) and FillNametable2/3.**
- Unreachable by the r3-a1 closure and the computed-transfer census, plus r4 residual_preds.
- No new references were found this round. FadeInBgGroups and FadeOutBg/SpriteGroups callers are again
  unreachable only (fadedelay_flow.json callers).

**5. $A84E.** Impossible by the frame logic (r3-a1). Rechecked against the source: an open 10th frame sets
TenthFrameBall to 3 after ball 2.

None of these needs a corrected inference or test beyond L-021. Classification is complete, so no structural
question is open.

## Gameplay milestone review (S33)

- The game has no levels, bosses or credits.
- All terminal screens have been visited (results, three awards, high-score ranks, return to title).
- Pause is unavailable (S30).
- New this round: an open frame after a first-ball miss (E19).
- No practical unvisited milestone remains.

## Markdown files created this phase

- this file
- analysis/redirect/r5-a1/README.md
