# Round 8 / redirect handoff (attempt r08-a1)

Status: **proposal ready, metadata/editorial only**. Two low-priority ready leads:
- R8-L043: map evidence metadata for 20 Data regions.
- R8-L044: three commentary prose names.

No structural change was found: 0 classification bytes, 0 boundary changes, 0 kind changes, 0 bank-context changes.
**I recommend terminal stop for structural research.** Apply the two leads in final editorial acceptance (or one
light disassembly/commentary touch-up), then run the final audit. The rejection review below supports this. Another
redirect should not run unless new evidence appears.

Checkpoint: pass-008 (maps/current.json == analysis/state.json == maps/pass-008.json, sha256 569ab31c...7bbdb).
- Classification: 131,088/131,088, 0 unknown. Regions: 2,590 (1,670 code, 902 data, 18 padding). Observed code: 26,839 B.
- These are unchanged: ROM, map, state, commentary source and toolkit.
- No emulator server was started and no scratch directory was created. Research files are in analysis/redirect/r08-a1/.

## Scope
I started from the round-8 commentary handoff, commentary/uncertainties.md "Round 8", and leads.json (last lead R7-L042,
consumed). Then I triaged:
1. Stream/table boundaries derived from decoders now known to be imperfect: the sound-stream $FE length.
2. Data-note/role consistency between the map and the current source.
3. The remaining weak Static code and low-confidence regions: a census, plus a re-read of each region's decisive
   evidence.
4. Gameplay coverage: scenarios.json S01-S17 already cover boot, all three games, the final match, the ending, game
   over and the debug menu. No milestone with a structural question is unvisited. S15 (full credits scroll) remains
   deferred and no structure depends on it. No gameplay was run, and scenarios.json is unchanged.

## Findings
1. **The sound-stream boundaries do not change.**
   - Handler check: B6_SndCmdFE_Fade is `jsr B6_StartFade` (three `iny` plus operand reads) followed by `iny`, so the
     command is 4 bytes, not 1.
   - I re-checked every other command length against its handler: F0 2 (including the effect-track skip), F1 2, F2 4,
     F3 2, F4 3, F5 3, F6 1, F7-FC 2 each, FD 1, and 1 byte for notes, B0, D0 and E0.
   - Re-walk with FE = 4 (tools/sndwalk_fe4.py): it is identical to analysis/tools/round03/sound-streams.json. That
     means 113 streams with the same starts, stops, ends, calls and jumps, and the same 28 headers.
   - There is no $FE byte anywhere in $8DE7-$9F6A (sndcmdcount.txt).
   - The 113 map Data regions start exactly at the stream starts and end at $9F6A (sndwalk-compare.txt).
   - The commentary statement "no effect on current data" is confirmed.
2. **The map evidence of continuation segments lags the commentary (R8-L043).**
   - The round-7 stale.py matched only notes that begin with "NAME (". It missed "Part of NAME ($A-$B)" segments and
     the "Role/consumer not yet identified" wording.
   - The extended scan (tools/stale2.py) finds 20 Data regions (1,425 B, conf 0.7-0.8). Their map evidence still says
     generic or role not identified, but they lie inside named structures: B5_WheelStrip ×7, PaletteSets ×4,
     B4_TableFrameStructs ×2, CardBlockTable, CardBlocks, B5_ArrivedAtText, B5_PortraitTileLists, DpcmSampleCopyTail,
     ChipValuePtrTable (with ChipAmounts and BetLimitTable) and MetaspriteSlotTable (with its sub-tables).
   - ROM extent checks (extchk.txt):
     - PaletteSets: all 44 16-byte records start with $0F.
     - CardBlockTable: word k = $C638 + 37k for all 53 entries.
     - CardBlocks: 53 records of [5, 7, 35 tiles].
     - B5_WheelStrip: 84 columns of 5 bytes. Number tiles are $80 or above, there are two rim triples, and the last 8
       columns repeat the first 8.
     - B4_TableFrameStructs: all four structs end with $00 $FF.
     - B5_ArrivedAtText: [19, 1] plus 19 letters.
     - DpcmSampleCopyTail: identical to $FD01-$FD6F.
     - In every case the segment ends where the structure ends.
   - This is metadata only.
3. **Three dangling names in commentary prose (R8-L044).**
   - tools/danglingnames.py found 17 hits. 14 are intentional "Named X before round 8" notes or wildcards. These 3 are
     dangling:
     - B6_UnrefRecordPtrsA000 at line 27111: now B6_WorldMapMarkerPtrs.
     - B3_UpdateBankroll at line 1732: no such routine exists.
     - B5_MatchResult at line 22645: the users are B5_MatchResultEntry / B5_MatchResultScreen.
   - The round-1 text in uncertainties.md lines 45-48 (bank-6 "stream boundaries not traced", "$A000-$BDE6 left
     generic") has been superseded. It is historical, but a cross-reference would help.
4. **Weak regions census: no new evidence.**
   - These have no new caller, operand, trace hit or conflict:
     - Code below 0.85: the 7 bank-start stubs, CallFarInline_Return, 1876, 3192, and 2761-2765/2876/2877/3221.
     - The other low-confidence data: 3280, 2777, 2787 and 2788.
   - The $FE path (ToFE, B6_SndCmdFE_Fade, B6_StartFade) and the type-2 header path are Static code reached through a
     complete low-nibble dispatch. Finding 1 shows that no data selects them, so only an artificial stream could run
     them. That would show the mechanics work, but not change the structure or show natural reachability, so I did not
     run it.

## Ready leads
| ID | Target | Recipe | Change |
|---|---|---|---|
| R8-L043 | 20 Data regions (r8-l043-proposal.jsonl) | stale2.py + extchk.py, then map.annotate: prepend proposed_evidence_prefix and set proposed_confidence (0.85 for 17 extent-checked rows; unchanged for 2833, 2434, 2438) | metadata only (1,425 B evidence) |
| R8-L044 | game.asm lines 27111, 1732, 22645 | fix in round08-commentary ann inputs, build.sh, then `commentary.py verify`; re-run danglingnames.py | commentary prose only |

## Rejection review (terminal assessment, per evidence-policy.md)
Each item gives the candidate claim, its decisive evidence, what that evidence rules out, the competing reading and
the verdict.

1. **The round-3 sound walk mis-sized $FE, so stream boundaries could be wrong.**
   - Decisive evidence: the handler (B6_StartFade) and the FE = 4 re-walk, which is identical, with no $FE byte present.
   - What it rules out: any boundary shift from this error, across all 113 streams.
   - Competing reading: none remains.
   - Verdict: closed.
2. **b4 $BB30-$BB31 (3221, Code 0.5) could be data.**
   - Decisive evidence: the round-4 review of R3-L027. The bytes are `lda zpLocal3` (the travel-state byte used by the
     adjacent routine) between an `rts` and the entry $BB32.
   - What it rules out: nothing either way. A 2-byte orphan reading is neither supported nor disproved, and no
     consumer reads it as data.
   - Competing reading: generic data.
   - Verdict: no discriminating test exists without a caller. The current claim is explicitly structural-only at 0.5.
     Keep it.
3. **Unreferenced fragments 2761-2765, 2876, 2877, 1876 and 3192 could be data.**
   - Decisive evidence: round-2 / round-6 review. Each fragment is a coherent official decode bounded by jmp/rts that
     joins established entries at instruction boundaries, and each has a sibling routine of the same shape.
   - What it rules out: misalignment and a data reading by the same consumer. No reader operand points into them
     (rescanned in r07).
   - Independent constraints: layout (bounded between control transfers), sibling equivalence, the RAM/API conventions
     they use, and the absence of any data reader.
   - Competing reading: dead or patch bytes, which are still code.
   - Verdict: keep. Execution, or a reader that turns out to be data, would change this; neither is available, and the
     r07 poker coverage and the S01-S17 traces hit none of them.
4. **The 18 Padding regions and b2 $BB1F-$BB4A.** The r06-a1 review applies with unchanged premises: bank tails, stub
   gaps and alignment fill beyond the last read cell, and an unpointed generic repeat of row 3. Keep.
5. **Static code 0.85 (619 regions, 10.9 KB).** These rest on static descent with bank context. Observing them would
   change only provenance, and no structural claim depends on it. Coverage gains were taken in R2-L026, R4-L033 and
   R7-L041. No unvisited milestone remains that would reach a new subsystem; the remaining Static code is error,
   tie-break and alternative branches inside observed routines. Not a structural avenue.
6. **Semantic questions** (PlaneTableA/B fields, info-block $FC/$FD marks, B5_WheelStrip number order, roulette board
   geometry, nine address-based RAM names, DMC records 5+, instrument list ends). Each lies inside an already
   bounded, consumer-anchored Data region or concerns RAM. Their answers change labels and comments, not
   classification or boundaries. Deferred to editorial work as optional meaning improvements.

Residuals: 0 unknown bytes. Proposed versus accepted classification bytes: 0 versus 0. Readability and the final
editorial audit (review.json editorial_audit pending) remain separate gates.

## Deferred (reasons)
- **An artificial $FE / type-2 sound run:** it would test mechanics only. There is no structural or reachability
  payoff, because no data selects it.
- **Roulette board geometry and the B5_CheckCursorMove tests:** semantic only, inside observed code. It needs a
  screen-to-cell mapping experiment, which affects naming only.
- **The nine address-based RAM names:** commentary naming. This is part of the final editorial backlog, not
  structure.

## Toolkit
No new toolkit issues found (analysis/toolkit-issues.md unchanged). No runtime was used.
