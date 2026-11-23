# Round 3 commentary + final editorial audit handoff (2026-11-22)

STATUS: COMPLETE. The commentary is reconciled with pass-003, current-source readability is
complete, the final editorial audit passed (`commentary.py audit`: ok, byte_identical), and a
fresh `commentary.py verify` after the audit is byte-identical. No archive or publication was
done. The ROM map is 100% classified: 262157 B evidence-backed + 3 B Manual fallback. This
is not 100% evidence-backed classification.

## Scope
- Input: checkpoint pass-003 (maps/current.json = analysis/state.json = maps/pass-003.json,
  sha256 be088f7905b9e184cdcf32b34a57ec061e39af752fece9848856eb378eb7bc3f;
  analysis/pass-003-verification.json ok=true, byte_identical=true, re-checked after the
  audit). input/game.nes is unchanged (sha256 ef10ad58...).
- Map change since round 2: only regions 1908 and 1909 (L-015 residual, Unknown -> Data,
  Manual provenance). No other region changed.
- Work done: reconcile the commentary baseline; source and review text for the two
  residual ranges; final editorial review of known issues; final audit.

## Preserved copy
- The whole round-2 `commentary/` directory was copied to `analysis/round-2/commentary/`
  (52 files, 8.0 MB). `diff -r` confirmed it was identical before anything was moved.
- The original was then moved aside, `commentary.py prepare` was run, and the carry was
  done. After that, the aside copy was checked identical to the preserved copy once more
  and removed as a redundant duplicate. The round-1 copy in analysis/round-1/commentary/
  is untouched.

## Carry method
- `prepare` built a fresh frozen baseline from pass-003: before/, baseline.json,
  phase1-state.json, manifest.json and a 993-entry scaffold.
- Compared with the round-2 scaffold (id/offset/length/kind), exactly 2 rows changed:
  - unknown-01c44f -> region-1908 (115791, 2, Data)
  - unknown-01c597 -> region-1909 (116119, 1, Data)
- before/game.asm differs from round 2 only by the two new Manual region headers.
- Carried from round 2: tools/, notes/, progress.md and uncertainties.md. The notes are
  keyed by bank + CPU address, which is a physical range. source/game.asm was regenerated
  from the new before/ with tools/annotate.py. It was not copied.
- Check of stale assembly: before any round-3 edit, the regenerated source differed from
  the round-2 source/game.asm only at the two residual ranges (their headers). review.py
  then rebuilt review.json. Apart from source line numbers it matched the round-2
  review.json in 991/993 entries, and in all 1360 units. Only the two residual entries
  changed.
- Workspace-local tool change: tools/review.py gives Manual-fallback data entries an
  explicit summary.

## Residual ranges (manual fallback, reported separately)
| Region | Physical | CPU (PRG14 fixed) | Length | Source |
|---|---|---|---|---|
| 1908 | 0x1C44F | $C43F-$C440 | 2 | game.asm, block after BuildMapColumn, before PutMetatileInColumn |
| 1909 | 0x1C597 | $C587 | 1 | game.asm, block after AttributeIndex, before ApplyCameraScroll |

- Both ranges are emitted as `.incbin` data with no label.
- Their comments say:
  - UNKNOWN MEANING;
  - data only because of a user-directed manual fallback (L-015, Manual provenance);
  - NOT an evidence-backed classification;
  - the prior ambiguity is unresolved (unreferenced alternate entry/leftover code, or
    leftover data);
  - there are no references, and only 6502 RTS dummy reads were observed, never an
    opcode fetch.
- They are not called dead code, padding or tables.
- The review summaries for both entries say the same.

## Final editorial review (findings and fixes)
Valid round-1/2 review credit was reused; there was no whole-source reread. The following
were fixed:
1. 16 address-only `Tbl_` sub-table labels were given descriptive names, with format
   notes taken from their consumers:
   - RoundTileOverridePtrs
   - MediumPrizeRows, SmallPrizeRows, PrizeRowOffsets
   - TargetFramesA0, TargetFramesB321, TargetFramesB0
   - CarriedBobYSpeeds
   - PlayerFrameTablesB, PlayerFrameTablesC
   - EffectYSteps
   - GoalSpriteAttrs, GoalTileOffsets, GoalSpriteTiles
   - PartYSteps, PartXSteps
2. Address-only routine labels: `Sub_F239` -> FinishScreenClear,
   `ClearScreenAndShow_F236` -> FillNametable1AndFinish. No `Tbl_`/`Dat_`/`Sub_` labels
   remain.
3. Misleading name: EffectFrameTables -> EffectXSteps. The table is a 16-bit X step that is
   added to the effect position, not a frame table.
4. Stale comments: references to Tbl_C789, Tbl_04_9510 and Tbl_11_BDCA were corrected, and
   so was the ladder-table note. The PrizeTables note now describes the top-prize rows.
5. Wrong claim: the $E000-$E6F3 fixed-bank data note said it was "read by fixed-bank code
   through absolute,indexed operands (see the Tbl_ labels)". No such reference exists, so
   the note now says the consumer, format and meaning are unknown.
6. Header comments were added to the 20 most substantial name-only helpers (20-23 code
   bytes), for example TakePanel, RecordPanel, ReadSpecialTile, FadeInStep,
   TickChannelTimer, FreeSlot and BeginAttrFill.
7. Spot-checked headers against code: every edited routine plus a random sample of 7.
   No further contradictions were found.

Accepted as is:
- Raw RAM operands: only Reset's eight page-clear stores, which are commented.
- Numeric $8000/$8001/$8020/$BE00/$A980/$9CF0 operands: these are pair-relative
  switched-bank tables read from fixed-bank code, and they are commented.
- 253 short helpers or secondary entries (under 24 code bytes: loop continuations, shared
  tails, one-line setters) rely on their descriptive names.

## Verification
- `commentary.py audit`: ok=true, byte_identical=true, reviewed_regions=993,
  reviewed_units=1360, editorial_audit=true (commentary/audit.json).
- `commentary.py verify` was re-run after the audit: ok=true, byte_identical=true.
- source/game.asm sha256 is
  0ae28422aaa87e039db8c9b9f1d9a468989684724dec8d5dc164af55e5d46bff
  (commentary/verification.json).
- The reading copy commentary/reading-after.asm (27637 lines) was refreshed by the audit.
- review.json editorial_audit is status=reviewed, with notes that include the
  manual-fallback statement.

## Residual uncertainties
Details are in commentary/uncertainties.md:
- L-015's 3 bytes: code vs data is still ambiguous. Manual fallback only.
- Object-type character identities, and boss behaviours A-F.
- Bonus-game prize rules. MediumToSmallA/B is a tentative reading.
- The special object $0510-$051B, and the visible effect of the diagonal tile sweep.
- Formats of the generic data banks. This includes the $E000-$E6F3 fixed-bank block, which
  has no known consumer.
- Write-only variables.
- Reachability of the pass-2 orphan code (L-016).
- Per-object-type indexing of PartXSteps/PartYSteps.

## Toolkit
No new toolkit issues (see analysis/toolkit-issues.md). All fixes were made in the
workspace-local commentary/tools. The stale `run.py serve` (PID 1823535) was not
contacted.
