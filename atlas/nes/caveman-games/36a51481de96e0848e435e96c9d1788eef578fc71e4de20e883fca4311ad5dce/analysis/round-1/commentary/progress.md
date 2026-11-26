# Commentary progress

Prepared verified baseline (commentary.py prepare; before/ frozen, byte-identical).

## Round 1 commentary (2026-11-25) -- first pass (superseded by the continuation below)

Input: checkpoint pass-001b (maps/current.json == maps/pass-001b.json; analysis/state.json).

### How the source is maintained
- source/ is rebuilt from the frozen before/game.asm by `python3 commentary/tools/build_source.py`
  using the notes database commentary/notes/ (bank0-7.txt manual notes, bankN.auto.txt propagated
  sound-driver/MMC1-stub notes, ram.txt symbols). Instructions and .incbin ranges are copied unchanged,
  so the rebuild is byte-identical by construction; edit the notes, not source/.
- Notes are keyed by 16K bank + CPU address (physical offsets derived), so they carry by physical range.
- tools/propagate.py copies notes to aligned copies of the banked sound driver (>= 8-instruction
  matching blocks) and the $FF00 stubs; tools/update_review.py refreshes review.json units/status.
- Reading aids (not deliverables): tools/summ.py, tools/compact.py, tools/hx.py, tools/dups.py.

### Scope done
- Layout: game.asm includes hw.inc, ram.inc and one file per 16K PRG bank (bank0.asm-bank7.asm).
- All branch/call/jump targets labelled (bank-aware; fixed-bank operands into $8000-$BFFF stay numeric
  unless the bank is established); RAM, PPU/APU and MMC1 operands symbolized (326 RAM symbols, global
  and bank-scoped); pointer tables emitted as .word; text blocks carry decoded strings.
- Fixed bank 7, menu bank 0, Mate Toss bank 1 and all banked sound-driver copies: routine headers,
  named branches and data structures (IRQ split, NMI, PPU buffer, metatile upload, metasprites,
  sprite text, results/ranking, records, dispatcher, both sound drivers).
- Event banks 2-6: every routine named; main loops, frame handlers, input, scheduling/bracket and
  setup routines explained; key per-player variables named.
- Unknown ranges: notes at $95E6 (bank 2), $9179/$A60C (bank 3), $957C (bank 5).

### Remaining (exact)
- 192 code units in banks 2-6 have only a one-line mechanical summary (status pending in review.json);
  list with file offsets: commentary/pending_units.json. Ranges: bank 2 file 0x810D-0x9663,
  bank 3 0xC215-0xD42F, bank 4 0x1010D-0x11787, bank 5 0x1411F-0x16928, bank 6 0x18068-0x1B793.
- Inside those units many work-RAM operands ($02xx/$04xx/$05xx) are still numeric and branch labels
  use @loop_/@skip_ address forms; event-bank data tables keep Bn_DataXXXX names.
- editorial_audit stays pending (final acceptance only).

### Verification
- `python3 /root/romatlas/scripts/commentary.py verify <workspace>` -> byte_identical true
  (commentary/verification.json); reading copy refreshed (commentary/reading-after.asm).

## Round 1 commentary continuation (2026-11-25) -- status: readability scope complete

Input unchanged (pass-001b; maps/current.json and analysis/state.json not modified). Classification unchanged.

### Scope done
- All 192 pending units in banks 2-6 now have full headers (inputs, algorithm, state changes,
  tentative meanings), descriptive branch labels and symbolic RAM operands. pending_units.json is empty;
  review.json: 824 code units reviewed, 0 pending (update_review.py).
- Whole source: every @skip_/@loop_ auto label in banks 0-7 replaced by a descriptive local name
  (duplicates inside one bank keep an _ADDR suffix added by the builder); no numeric work-RAM operand
  remains in any bank; every Bn_Data/Text/Pad address label in banks 1-6 renamed to a role name, with
  lo/hi partner bytes folded into objects ("=" extents) so they read Name+1.
- New tool: tools/merge_notes.py (merges a batch of notes / RAM symbols into notes/ by address).

### Names/interpretations changed
- Bank 2: FsCpuDelay -> FsCpuPlayer, FsCpuVar -> FsCpuReactTimer, FsVar04A9 -> FsCpuPatIdx,
  FsVar04B1 -> FsSmokeFrame, FsVar057B -> FsTileBlock; FS_AmbientPos split into FS_AmbientX/Y;
  FS_SmokeRanges split into FS_SmokeOamOfs/XBase/FrameRange/Sprites.
- Bank 3: DrLap/DrVar04C2 -> DrCourseHi/Lo (16-bit course position, finish $0F00), DrFinished ->
  DrBoostTimer, DrSpeedTarget -> DrClubHit, DrRiderState -> DrReversed; address-named handlers renamed
  (DR_StateJump, DR_StateCrashed, DR_StateConfused, DR_CheckObstacle, DR_StateBackUp, DR_ClubSwing,
  DR_JumpInput, DR_BackInput, DR_AdvanceAction, DR_StepSpeed, DR_AdvanceCourse, ...).
- Bank 4: SR_DrawRunners/SR_DrawChaser were swapped -> SR_DrawActors/SR_DrawRunner; SR_Runner*,
  SR_Chaser*, SR_Camera*, SR_Obstacle* helpers renamed by role; SR_ObstacleTick -> SR_SoundTick
  (it only chooses sounds).
- Bank 5: ClStepCount -> ClDazed; CL_Sub*/CL_State*/CL_Check* helpers renamed (CL_ResolveAttack,
  CL_Damage, CL_IsGuarding, CL_IsDown, CL_IsClinched, CL_CommandToState, ...); next-state matrices
  CL_NextStateDpad/CL_NextStateA; action and script tables emitted as .word.
- Bank 6: DV_SkipComputer -> DV_PrepareRun (plain per-run reset), DV_VaultStep* -> DV_VaultFlight/
  DV_VaultMove/DV_VaultPhysics/DV_MoveCavemanX, DV_OutcomeADEE/AE50 -> DV_OutcomeOnActor/DV_OutcomeEdges.

### Unresolved / deferred
- See uncertainties.md (continuation section). Lead L-011 raised for unreferenced Data bytes that decode
  as dead code (banks 3, 5, 6). editorial_audit stays pending (final acceptance only).

### Verification
- commentary.py verify -> ok, byte_identical true (commentary/verification.json); reading copy refreshed
  with commentary.py read (commentary/reading-after.asm).
