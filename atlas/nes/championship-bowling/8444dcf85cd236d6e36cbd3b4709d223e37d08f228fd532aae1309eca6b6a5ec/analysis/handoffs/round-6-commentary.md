# Round 6 commentary handoff

- Phase: commentary, round 6, on checkpoint **pass-006** (maps/current.json = analysis/state.json
  = maps/pass-006.json, sha256 865f4d5a...ee973cf93; 512 regions).
- Status: **complete. Current-source readability covers ALL classified source**: all 512 scaffold
  entries, file range $0000-$FFFF (physical 0-65551), header and CHR included. **No ranges remain.**
- Final editorial audit / archive acceptance was intentionally **not** run; editorial_audit stays pending.
- ROM classification is unchanged and reported separately: 100.00% (65552/65552, 0 unknown), structural only.

## Baseline change handling

1. The map changed (pass-005 -> pass-006: region 386 $A92B Static -> Observed; evidence text added to
   fade, sound, page-carry, $FB8F and $FC23 regions). No boundary or kind change.
2. The whole round-5 commentary directory moved unchanged to **analysis/round-5/commentary/** (source,
   review, frozen baseline, notes, reading copies). It is the only copy; earlier round copies untouched.
3. `commentary.py prepare` made a fresh frozen baseline (before/ = generated pass-006 source), byte-identical.
4. Carry by physical ranges (analysis/tools/commentary-round6/carry.py, round-5 method): round-5 game.asm
   copied, 1 marker rewritten (region 386 provenance), no insert or .incbin split; marker list equals
   before/game.asm; verify byte-identical right after the carry. source/mapping.json and layout.cfg come
   from the fresh prepare (mapping.json carries the new pass-006 evidence text).
5. review.json regenerated with `cm.py review`: 512/512 reviewed, 988 units. Summaries changed only for
   region-15 (ShowLaneScreen) and region-272 (EnvelopeRestart); region-512's hand-written summary carried
   (source line updated). All other entries equal round 5 apart from source line numbers.

## Edits this round (analysis/tools/commentary-round6/batches/, applied by replace.py)

- **01-l022.txt, L-022 (resolved):**
  - $A92B: the $D (first ball knocked nothing) code counts as 0; reached after a missed first ball.
  - AnnounceBowler: the only place FadeDelay is $FF, so both FadeInSpriteGroups calls are instant.
  - Dormant notes: FadeInSpriteGroups stepped body, BrightenGroups, TakeNextFadeGroup, FadeOutAll's $FF path,
    BrightenColor's cap (FadeInAll brightens only twice from level 0).
  - Sound: envelope page carry ($E267), EnvelopeRestart (new header; all 41 ROM envelopes end in a $FF hold),
    attenuation floor ($E3D4); StartEnvelope header notes the restart is unused.
  - Impossible page carries: $AAF7/$AB07 (digit pointer from $0456, at most +60), $BDEB (rows $22B6 + $40n).
  - DrawObject: $FD list jump supported but unused by ROM sprite lists (header and site).
  - $FC23 top-edge clip: not seen in recorded play; nothing rules it out (kept as an open possibility).
  - Header paragraph summarising the remaining small Static branches.
- **02-defects.txt (concrete defects in carried prose):**
  - ShowLaneScreen said "draw the lane screen instantly (FadeDelay $0F)"; $0F is a slow fade (15 frames per
    step vs. the usual 6). Rewritten.
  - SetChannelVolume now says the attenuated volume is floored at 0.
- commentary/uncertainties.md: round-6 section (resolved dispositions; $FC23 still open); older open items carried.

## Readability assessment

- One source file, commentary/source/game.asm (~11.4k lines, 1489 labels, 0 auto/address-only labels apart
  from deliberate unknown-role labels such as UnrefTilePairs_CDE7, UnrefSound*_xxxx, Stream_xxxx; 1 raw operand).
- Inventory scan: the 395 global labels without their own comment block are all members of documented groups
  (numbered frame/curve/envelope/sound tables, paired entry points such as FadeInBgGroups / FadeInSpriteGroups,
  the sound command table list, the sprite-helper list); same set as round 5, each checked against its group comment.
- Grep for stale wording (uncalled / no known caller / discovered code / TODO): none; one "not checked" is the
  deliberate selection-object uncertainty.
- The rest of the source is carried unchanged from round 5, whose full-source assessment still applies: the
  pass-006 changes are provenance/evidence text only, and every affected region and its dependencies
  (frame totals, fade library and its callers, sound envelope/volume code, scorecard digits, high-score name
  reveal, DrawObject) were rechecked; the fade check found the ShowLaneScreen defect above.

## Verification

- `commentary.py verify`: ok=true, **byte_identical=true** after the carry, after each batch and fresh at
  handoff (commentary/verification.json, source sha256 cc0f4f95...c0c7). input/game.nes unchanged.
- Reading copy commentary/reading-after.asm refreshed with `commentary.py read`.

## Unresolved meanings

See commentary/uncertainties.md: $FC23 top-edge clip (provenance only); character 0 handedness; "PND" units;
celebration and selection-object naming; portrait pose B; UnrefTilePairs_CDE7, ScorecardData's first 16 bytes
and UnusedBestScoreData; ClearScores `,y/,x` slip; one ignored 4-frame B press; sound id meanings; purposes of
unreachable routines and dormant jobs; undecoded sprite/nametable records (L-010).

## Structural questions / leads

- No new structural question; commentary proposes no map change.
- L-022 resolved (leads.json attempt entry round-6-commentary). L-010 stays open/low (editorial formatting).
  $FC23 remains the only deferred provenance item (no discriminating test). No new lead IDs.

## Housekeeping

- Markdown created this phase: this handoff, plus the moved round-5 copy under analysis/round-5/commentary/.
  commentary/*.md come from prepare (progress.md and uncertainties.md rewritten).
- Tools: analysis/tools/commentary-round6/ (cm.py copy, carry.py, replace.py, batches/). No scratch left.
- No server started; no toolkit change; no new toolkit issue.
