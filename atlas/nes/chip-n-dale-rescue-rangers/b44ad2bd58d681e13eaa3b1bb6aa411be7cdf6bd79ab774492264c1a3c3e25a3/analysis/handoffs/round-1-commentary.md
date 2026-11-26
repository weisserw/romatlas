# Round 1 commentary - handoff (COMPLETE)

## Scope and checkpoint
- Input: latest verified checkpoint **pass-001-final** (maps/current.json ==
  maps/pass-001-final.json, sha256 8404126a...; analysis/state.json). Prepared with
  `scripts/commentary.py prepare` (frozen commentary/before, baseline.json,
  phase1-state.json, reading-before.asm).
- Covered: ALL currently classified source - fixed bank 7 (PRG14/15), banks 6 (objects),
  5 (sprite drawing), 4 (spawning), 3 (sound), 2 (screens/title/map/cutscenes/continue),
  1 (demo input), replicated reset stubs/vectors in banks 0-6, data regions and all 121
  unknown ranges (boundary/decode notes). Classification unchanged (97.60%, 6303 unknown).
- Method: source/game.asm + source/symbols.inc are regenerated from the frozen
  before/game.asm by commentary/tools/annotate.py using commentary/notes/*.ann and
  notes/symbols.txt (edit notes, rerun, verify). tools/view.py and tools/skim.py are
  reading aids; tools/review.py refreshes review.json. Progress log: commentary/progress.md.

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify <workspace>`: ok, byte_identical true
  (commentary/verification.json, game.asm sha256 in that file).
- Reading copy refreshed: commentary/reading-after.asm (expands symbols.inc).
- review.json: 948/948 scaffold entries reviewed (summary + source), 642 units;
  editorial_audit intentionally pending (final acceptance only). Audit not run.

## Results / examples
- 1,932 labels, 1,650 hand-named; no address-only labels remain (remaining minor branch
  targets carry <Routine>_Loop/_Skip/_Return names). ~2,080 annotations.
- RAM map in symbols.inc: task scheduler ($80-$93), joypads ($14-$17), PPU shadows and
  VRAM lists ($0300-$03FF), camera/streaming ($20-$36, $FA-$FD), object tables
  ($0400-$059F, 16 slots, slots 0/1 = Chip/Dale), player state ($50-$6E), sound queue.
- Examples: Nmi/Scheduler/YieldFrame/SwitchPrgBank (cooperative tasks, deferred sound);
  MainLoop and the 13 PlayerAction handlers (PlayerGround ... PlayerOut/GameOver);
  UpdateCameraX/StreamColumns/StreamRows and the screen->block->metatile map format
  (DecodeBlock, GetScreenPtr); TerrainProbe collision classes; LoadLevel/EnterSection
  per-level/section tables; ObjType handlers (Hopper, Swooper, Turret, ItemBox, Zipper,
  rideable platform, collapsing floor, segmented boss, scrolling boss); DrawObject
  animation/metasprite formats; spawn lists (InitSpawnLists/SpawnEntry/ObjectDefs);
  Dialogue text engine; sound engine command/track/envelope tables.

## Unresolved meanings (see commentary/uncertainties.md)
- Game identity of most object types; sound data formats (partly decoded);
  track-flag bits; tile classes 7-9 (slopes vs. level-5 conveyors); HUD counter layout;
  role of PlayerConfig+22/26 poses beyond the identified actions.

## Structural questions for redirect (evidence: consumers in current source)
1. Bank 6 $8235-$8489 (597 B) and $85C1-$8605 (69 B) unknown: targets of the type-$01
   sub-handler table MovingItemHandlerLo/Hi ($81FB/$8202, itself unknown) via
   ObjType01's JMP (Temp00): $8235, $839C, $85C1, $85F8. Strong code candidates
   (context [12,13,14,15]).
2. Bank 3: inline jump tables (JSR JumpTableDispatch) at $816E (7 entries), $82E0 (5),
   $8468 (25 entries), $86A4 (5) are unknown data; their targets $81E7/$81EE/$8204,
   $849A-$849D, $84D1/$84D6, $8541, $867E are unknown code reached only via those
   tables. Classify tables as data and targets as code (context [6,7,14,15]).
3. Bank 2 $8900 (in unknown $88E1-$8A60) is the Select-button JSR target from MainLoop;
   bank 2 $A410 (in unknown $A40A-$A447) is the level-clear tally JSR target from
   UpdatePlayers. Both have direct JSR callers with known bank context [4,5,14,15].
4. Data consumed by code but left unknown: bank 6 $8BAA-$8BC7 (item-box tables),
   $8D2A-$8D68 (pickup sound/handler + heart/life indices), $8DA5 (ExtraLifePath),
   $9427 (shot spread), $95C7 (respawn limits), $966D, $9725, $9954, $9B36, $9EE8,
   $A02A, $A0A6, $AE63-$AE96, $B4AA (boss palette), $B2FD-$B341 tables;
   fixed bank $FB01-$FC40 (BlockDrawOffsets/Ids), $C73F (InitTable05A0/05D0),
   $D657-$D6A2 tables, $E723-$E753 section tables, $F586 (AimDirectionTable),
   $F9F2-$FA33 (hitbox tables).
5. Fixed-bank unknown ranges that decode as complete plausible routines without known
   callers: $FEE5 (task free/exit), $C13A (PPUCTRL helpers), $C206 (fill), $C268,
   $C2BE (inline dispatcher), $F0D3 (probe-move variants), $D4A1; bank 6 $8B5F.
   Need a caller/contract before classification (evidence-policy: structure vs reach).
Suggested bounded checks: static descent from the listed table targets in the stated
contexts; a traced run pressing Select in gameplay; finishing a level to reach $A410.

## Remaining gaps
- None in current-source readability scope. Weaker spots (documented, not skipped):
  sound-engine byte formats, object game identities, a few base-offset table bounds.
- Whole-ROM classification remains incomplete (6303 unknown bytes); final editorial
  audit pending.
- Toolkit: no new toolkit issues found in this phase (see analysis/toolkit-issues.md,
  TK-001..003 unchanged).
