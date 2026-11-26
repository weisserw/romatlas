# Round 5 commentary handoff

- Phase: commentary, round 5, on checkpoint **pass-005** (maps/current.json = analysis/state.json
  = maps/pass-005.json, sha256 951ffb73...c3d700e64f; 512 regions).
- Status: **complete. Current-source readability covers ALL classified source**: all 512 scaffold
  entries, file range $0000-$FFFF (physical 0-65551), header and CHR included. **No ranges remain.**
- Final editorial audit / archive acceptance was intentionally **not** run; editorial_audit stays pending.
- ROM classification is unchanged and reported separately: 100.00% (65552/65552, 0 unknown), structural only.

## Baseline change handling

1. The map changed (pass-004 -> pass-005: region 49 $8A2D Static -> Observed; evidence text added to the
   8 gated NMI-job regions and region 315). No boundary or kind change.
2. The whole round-4 commentary directory moved unchanged to **analysis/round-4/commentary/** (source,
   review, frozen baseline, notes, reading copies). It is the only copy; earlier round copies untouched.
3. `commentary.py prepare` made a fresh frozen baseline (before/ = generated pass-005 source), byte-identical.
4. Carry by physical ranges (analysis/tools/commentary-round5/carry.py, round-4 method): round-4 game.asm
   copied, 1 marker rewritten (region 49 provenance), no insert or .incbin split; marker list equals
   before/game.asm; verify byte-identical right after the carry.
5. review.json regenerated with `cm.py review`: 509 entries identical to round 4; region-48 (UpdateLaneBall
   unit) and region-315 (FillNametable2/3) summaries follow the new comments; region-512's hand-written
   summary carried. 512/512 reviewed, 988 units.

## Edits this round (analysis/tools/commentary-round5/batches/01-l020.txt, applied by replace.py)

- **L-020 (resolved):**
  - UpdateLaneBall: both X clamps are gutters ($C0 left, $E8 right, SFX_BALL_EDGE); the left one is
    reached by a full-left aim plus a left-drifting hook; `jmp @gutter` at $8A2D commented.
  - Concrete defect fixed: AimAndWaitForThrow said the aim range was $C5-$E4; the code stops at $C4.
  - RunSpinMeter: the arc gauge is labelled CONTROL on screen (POWER is the vertical bar); hedge removed.
  - SignGirlTiles: the sign reads "nPLAY"; first tile = bowler digit ($84/$94/$95/$96 = "1"-"4",
    CHR bank 2, sprite half $0000).
  - Header paragraph on dormant NMI jobs and FillNametable2/3. Each site now says dormant by data flow:
    NmiUploadAttrRow (and UploadAttrRow8), NmiVramRead, NmiFillRect, NmiCopyStrip column path,
    CopySpritesToDmaPage reversed copy, NMI WaitSeconds countdown; StripMode/SpriteCycleMode symbol notes.
  - FillNametable2/3: unreachable (no reference; FillNametable1's bne always taken). Still code.
  - commentary/uncertainties.md: CONTROL label and sign number moved to resolved.

## Readability assessment

- One source file, commentary/source/game.asm (11.4k lines, 802 global labels).
- No auto/address-only labels apart from deliberate unknown-role labels (UnrefTilePairs_CDE7,
  UnrefSound*_xxxx, Stream_xxxx); no "uncalled"/"no known caller"/"discovered code"/"not checked" wording
  left for resolved items (grep).
- The rest of the source is carried unchanged from round 4, whose full-source assessment still applies:
  the pass-005 changes are provenance/evidence text only, and every affected region and its dependencies
  were rechecked (UpdateLaneBall/AimAndWaitForThrow, the NMI and its jobs, FillNametable0-3).

## Verification

- `commentary.py verify`: ok=true, **byte_identical=true** after the carry, after the batch and fresh at
  handoff (commentary/verification.json, source sha256 e161cb21...f356).
- Reading copy commentary/reading-after.asm refreshed with `commentary.py read`.

## Unresolved meanings

See commentary/uncertainties.md: character 0 handedness; "PND" units; celebration and selection-object
naming; portrait pose B; UnrefTilePairs_CDE7, ScorecardData's first 16 bytes and UnusedBestScoreData;
ClearScores `,y/,x` slip; one ignored 4-frame B press; sound id meanings; purposes of unreachable routines
and dormant jobs.

## Structural questions / leads

- No new structural question; commentary proposes no map change.
- L-020 resolved (leads.json attempt entry round-5-commentary). L-010 stays open/low (sprite record
  splitting, formatting for the final editorial audit). No new lead IDs.

## Housekeeping

- Markdown created this phase: this handoff, plus the moved round-4 copy under analysis/round-4/commentary/.
  commentary/*.md come from prepare (progress.md and uncertainties.md rewritten).
- Tools: analysis/tools/commentary-round5/ (cm.py copy, carry.py, replace.py, batches/). No scratch left.
- No server started; no toolkit change; no new toolkit issue.
