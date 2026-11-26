# Round 6 redirect handoff

**Recommendation: terminal.**
- **Structural classification is complete:** 100.00% (65552/65552 bytes; 0 unknown).
- **Every Static Code byte has a disposition backed by evidence.**
- **No open provenance question remains.**
- **Next step:** final editorial acceptance (the commentary audit). It must apply the L-023 comment correction and
  settle L-010 (formatting).

## Scope and identity

- **Input:** checkpoint **pass-006** (512 regions, map sha256 865f4d5a...ee973cf93) and the round-6 commentary
  (byte-identical, all 512 regions).
- **Proposed bytes:** none. Accepted change: none.
  - Code stays Observed 11922 / Static 1937.
- **Not modified:** map, state, commentary source, ROM, toolkit (TK-001..TK-005 kept) and the configured binary.
- **Attempt:** analysis/redirect/r6-a1/ (README.md, tools/, evidence/; 88 KB).
- **Housekeeping:**
  - No runtime experiment, and no server was started.
  - Scratch was removed.
  - Disk: about 2.5 GB free.

## Review of the round-5 rejection review

The only item round 5 left open was $FC23. Its premises were checked against the source and the ROM.

| r5 premise | Check | Result |
|---|---|---|
| "Top-edge clip" | Read PlaceSprite @addY. The path runs when the high byte of ObjY + dy' is nonzero, where dy' is the offset, mirrored as -(dy+7) under vertical flip | **Wrong scope.** The path covers any Y sum outside 0..255, at the top or the bottom |
| "No constant or table bounds ObjY + dy" | Census of every write to the ObjTable Y, flag and frame-table fields in all Code regions (objy_writers.json), plus the dy ranges of all 26 ROM frame tables (frame_dy_ranges.json) | **Wrong.** ObjYHi is $00 in all 21 templates and in the Reset clear, and no other code writes it. ObjYLo writers are template constants, bounded loops (pinsetter $2F-$48; perfect award $87-$B9) or bounded arithmetic (close ball >= $2C; bowler $D4-$F0; lane pins > $30) |
| "No discriminating test" | Per-object range check (fc23_bounds.json, 17 contexts) | Y + dy' stays within **9..232**. The tightest top case is deck pins lifted by the pinsetter: $11 - 8. The tightest bottom case is the bowler after the throw: $F0 - 8. **No counterexample** |

**Outcome (L-023):** $FC23 (15 B) is **dormant by data flow**.
- This is the same standard accepted for the fades (r5) and for the gated jobs (r4).
- It stays Code/Static. No map change is needed.
- **Side result:** frame-table pointers come only from ROM (the templates, BowlerFrameTables $8CDD and
  PerfectFramePtrs $BFD2). No sprite list is ever built in RAM. This removes the scope limit ("ROM lists only") from
  the $FB8F $FD disposition.

## Other dispositions: premise spot checks

| Group | Premise | Checked this round |
|---|---|---|
| Gated NMI jobs | No reachable store opens the gates | The indexed and indirect store census from this round (objwriters.py, using the same decoder) agrees: the indirect targets are the APU, TileBuffer ($041B+, every WordToDecimalDigits caller passes a destination >= $041B) or the dormant VRAM-read job |
| $FD jump | No ROM list encodes it | Scope extended to every list, since no list lives in RAM. dyrange.py found no $FD at any record start in the 26 frame tables |
| Others | (r5 table) | Unchanged artifacts. Nothing new contradicts them |

## Static ledger: 1937 B, every byte accounted for

Source: analysis/redirect/r6-a1/evidence/static_ledger.json (46 regions).

| Remaining item | Bytes | Reason | Decisive artifact | Scope of the negative evidence | Why no corrected inference or test remains |
|---|---|---|---|---|---|
| L-001/L-002 runs | 1461 | Outside the closure rooted at the vectors | r3-a1 closure.json, plus the computed-transfer census (r3-a1 README S1) | Every computed transfer (RTS dispatch, stack writes, indirect jumps) in the decoded closure | Classification is complete. A natural caller would need a transfer that does not exist in the ROM |
| Gated NMI/library jobs | 264 | Their gate variables are never set | r3-a1 gate_writers.json, r4-a1 indexed_stores.json, r6-a1 objy_writers.json | Every direct, indexed and indirect store in all Code regions. The map is complete, so no store is missed | Forcing a gate open by a debugger edit proves nothing about natural play |
| FillNametable2/3 $F656 | 6 | No reference. The only fall-through is an always-taken bne | r4-a1 residual_preds.json | Predecessor analysis over the whole ROM | Structural fact |
| Fade variants | 160 | FadeDelay is $FF only in AnnounceBowler; other callers are dormant or unreachable | r5-a1 fadedelay_flow.json, fade_closures.json | Every FadeDelay writer and every caller closure | Data flow is closed |
| Sound restart, carry and clamp | 8 | ROM sound data never selects them | r5-a1 residual_static.json, clamp_sim*.json | All 41 envelopes and all 64 headers (with SndDuration 0 and 255) | ROM content is fixed |
| Page carries | 6 | Impossible by arithmetic | r5-a1 residual_static.json | Constant bases and counts | Arithmetic |
| $FB8F $FD jump | 14 | No list encodes $FD | r5-a1 residual_static.json, r6-a1 frame_dy_ranges.json and objy_writers.json | Every list, since none is built in RAM | Closed |
| $FC23 Y-range clip | 15 | No object's Y + offset leaves 0..255 | r6-a1 fc23_bounds.json | Every ObjTable Y and flag writer, and all 17 drawing contexts | Closed this round |
| $A84E | 3 | Impossible by frame logic | r3-a1 README; checked against the source in r5 | An open 10th frame sets TenthFrameBall to 3 | Logic |

Totals: unreachable 1467, dormant 461, impossible 9 (1937 B).
- **Strongest competing interpretation:** in every group, "reachable through an unseen natural route".
- **Why it fails:** the closures above are complete over the ROM data and every store. Each one excludes that route
  within the scope stated in the table.
- **What would revive a group:** only a missed writer or a missed transfer. Both censuses ran on a fully classified
  map.

## Leads (analysis/leads.json)

- **L-023 (new), resolved-static-disposition.** Editorial follow-up for the final audit, with no map change:
  - Replace the $FC23 comment "Nothing in the data rules it out". The new comment says this is the out-of-byte Y path,
    dormant by data flow, with Y + offset within 9..232 (fc23_bounds.json).
  - Update commentary/uncertainties.md: $FC23 is no longer open.
  - Note in the DrawObject $FD comment that no list is built in RAM.
  - Region 348's map evidence text ("unobserved, deferred") stays true, but it is incomplete. Refresh it only if a
    later map pass happens.
- **L-010:** open, low priority (formatting only; editorial).
- **Ready disassembly leads:** none. Pass record round-6-redirect added.

## Final unvisited-gameplay-milestone review (scenarios.json S34)

- The game has no levels, bosses or credits.
- S1-S33 visited the following:
  - the title (including the unskipped animation) and the menus;
  - selection for 1-4 players and lanes 1, 2, 4 and 5;
  - full natural games of 300, 299 and 267;
  - every celebration variant;
  - spares, gutters, open frames and the 10th-frame patterns;
  - results, the three award scenes, the high-score ranks and the return to title.
- Pause is unavailable (S30).
- No practical milestone or RAM shortcut target remains.

## Statement

Structural classification is 100% (0 unknown). The remaining Static bytes are explained, and every reachability
question has a closed disposition. The next step is **final editorial acceptance (the commentary audit)**, including
the L-023 correction. That audit and its verification remain required.

## Markdown files created this phase

- this file
- analysis/redirect/r6-a1/README.md
