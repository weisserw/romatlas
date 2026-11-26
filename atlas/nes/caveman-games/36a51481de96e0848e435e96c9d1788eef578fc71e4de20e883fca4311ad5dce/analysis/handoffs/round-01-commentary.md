# Round 1 — commentary handoff

**Status: INCOMPLETE.** Readability is complete for banks 0, 1 and 7 and for all sound-driver/stub
copies. Event banks 2-6 are only partly done: every routine is named and described, but 192 units
there have just a one-line summary.

- Input checkpoint: pass-001b (maps/pass-001b.json == maps/current.json, analysis/state.json). Classification
  unchanged: 99.12% (2318 unknown bytes). Commentary changes no structural classification.
- Phase directory: commentary/ (prepared this round with commentary.py prepare; before/ frozen).
- Verification: commentary/verification.json (commentary.py verify, ok, byte_identical true, final run after
  last edit). Reading copy: commentary/reading-after.asm (refreshed).
- Maintained source: commentary/notes/*.txt -> `python3 commentary/tools/build_source.py` -> commentary/source/
  (game.asm + hw.inc + ram.inc + bank0-7.asm). Edit the notes, then rebuild; see commentary/progress.md.

## Scope and readability assessment
- Whole source: all branch/JSR/JMP/data targets labelled (bank-aware), 326 RAM symbols (global +
  bank-scoped), PPU/APU/MMC1 register names, .word pointer tables, decoded text comments, 1476 named labels.
- Fully commented: fixed bank 7 (IRQ/NMI, pads, pause, clocks, PPU buffer, palettes, metatile screen upload,
  metasprite renderer, sprite text, results/ranking/records, dispatcher, fixed-bank sound driver), bank 0
  (attract, players, character select, bios, main menu, Practice, Caves of Fame, event intro), bank 1 (Mate
  Toss), and the banked SFX driver in banks 0-6.
- Event banks 2 Fire Starting, 3 Dino Race, 4 Saber Race, 5 Clubbing, 6 Dino Vault: main loops, frame handlers,
  input, pairing/bracket and setup are explained. Helpers have one-line mechanical summaries. Names that could not
  be verified are neutral (address-suffixed, e.g. SR_Runner8FCB, CL_StateStep8BBF, DV_VaultStepA9D1).
- review.json: 632 code units reviewed, 192 pending; all data/text/padding/unknown scaffold rows have summaries.
  editorial_audit is still pending.

## Examples
- commentary/source/bank7.asm `IrqHandler`, `NMI_Common`, `DrawMetasprite`, `UploadMetatileScreen`,
  `ReturnToMenu`, `RankSlots`, `SoundUpdate`.
- commentary/source/bank0.asm `MainMenu`, `CharacterSelect`, `CavesOfFame`; bank1.asm `MT_SpinPhase`, `MT_ScoreThrow`.

## Remaining gaps (exact)
- The 192 pending units are listed with file offsets in commentary/pending_units.json. By bank (file offsets of the first..last pending unit start):
  bank 2 0x810D-0x9663 (22 units), bank 3 0xC215-0xD42F (31), bank 4 0x1010D-0x11787 (43),
  bank 5 0x1411F-0x16928 (60), bank 6 0x18068-0x1B793 (36).
- Inside these units, work-RAM operands are still numeric and local labels use the @loop_/@skip_ address form.
  Event-bank data tables keep Bn_DataXXXX names.

## Unresolved meanings
Recorded in commentary/uncertainties.md. Main items:
- Purpose of QuitEventFlag (inferred).
- Controls of Dino Race, Saber Race and Dino Vault (L-006).
- Identity of the third actor in bank 4 and the second actor in bank 6.
- Why Caves of Fame skips event indices 4 and 6.
- Roles of $82/$83, $0111/$0112 and Var04AD.

## Structural questions / leads raised (for redirect; see analysis/leads.json)
- L-007: Some bank-7 bytes classified as Data decode as code. These are the IRQ split handlers and their epilogue
  $C036-$C047/$C080-$C0BD (RTS-dispatch table $C072), plus the QueueSfx body $F26A-$F277 and small stubs at
  $C5F1, $C61A, $C62F and $CEDB.
- L-008: refines L-002, bank 2 $95E6. $95E6-$95E7 are marker X offsets read by FS_DrawMarker. A routine
  starts at $95E8; the phase-1 trace began mid-routine.
- L-009: bank 3 $9179 (part of L-003) decodes as LDX #0 / JSR DR_SetVelocity / INX. No caller was found.
- L-010: bank 5 $957C-$9587 (part of L-003) is the tail of the CL_Bracket6 pair table plus the placement list
  at $9582. Consumer and index-bound evidence support this.
- Consumed: none (L-001..L-006 unchanged and still open).

## Toolkit
- New finding TK-005 (missing feature): NES generate emits no branch/call labels and splits regions into
  per-region .org blocks. Worked around with a local builder. Not blocking. See analysis/toolkit-issues.md.

## Next
Resume this commentary phase on unchanged maps: work through commentary/pending_units.json in banks 2-6. For each
unit, name its RAM in notes/ram.txt (bN scope), rename helpers, write headers and branch names, then run
build_source.py, update_review.py, commentary.py verify and commentary.py read.

## Continuation (round 1, same phase) — status: COMPLETE (current-source readability)

- Input unchanged: pass-001b (maps/current.json, analysis/state.json untouched); classification unchanged
  (99.12%, 2318 unknown bytes). No toolkit change, no publish/archive.
- Scope covered: all 192 pending units in banks 2-6 (Fire Starting, Dino Race, Saber Race, Clubbing,
  Dino Vault). Each has a full header (inputs, algorithm, state changes, tentative meanings), named
  branches and symbolic RAM. In addition, across banks 0-7: no @skip_/@loop_ auto labels remain, no
  numeric work-RAM operands remain, and no Bn_Data/Text/Pad address labels remain. Data tables carry
  role names, lo/hi partners read as Name+1, and action/script/frame pointer tables are emitted as .word.
  commentary/pending_units.json is empty. review.json: 824 code units and all scaffold rows reviewed.
  editorial_audit is still pending (final acceptance only).
- Readability assessment: event-bank mechanics are now explained in the source. Meanings stay
  tentative where only control flow supports them: controls, state numbering, actor identities, Dino
  Vault clearance geometry. Several earlier names were corrected:
  - Saber Race: SR_DrawRunners/SR_DrawChaser were swapped.
  - Dino Race: DrFinished is really a sprint (boost) timer.
  - Saber Race: SR_ObstacleTick only picks sounds.
  - Dino Vault: DV_SkipComputer is a per-run reset.
  - Clubbing: ClStepCount is the daze counter.
  See commentary/progress.md for the full list.
- Examples: bank3.asm `DR_ClubSwing`, `DR_StateJump`, `DR_CheckObstacle`; bank4.asm `SR_RunnerFreeState`,
  `SR_CameraSpeed`; bank5.asm `CL_ResolveAttack`, `CL_Bracket3`; bank6.asm `DV_VaultFlight`, `DV_UpdateBgTiles`;
  bank2.asm `FS_CpuInput`, `FS_UpdateSmokeX`.
- Verification: `commentary.py verify` -> ok, byte_identical true (commentary/verification.json, fresh after the
  last edit); reading copy refreshed (`commentary.py read`, commentary/reading-after.asm).
- Maintenance: notes are merged with the new commentary/tools/merge_notes.py, then build_source.py and
  update_review.py are run.
- Unresolved meanings: see the continuation section in commentary/uncertainties.md. Open items: Fire Starting
  state meanings and reused sound bytes; Dino Race control semantics (L-006) and DR_AdvanceCourse (L-004,
  inferred); Saber Race actor identity; Clubbing move semantics; Dino Vault clearance variables
  ($043E/$0440/$0433) and second-actor identity.
- New lead: L-011. Unreferenced phase-1 Data bytes decode as dead code: bank 3 $8984, bank 5
  $8B53/$8CBD, bank 6 $8DA8/$AE0C. Earlier leads L-007..L-010 are unchanged. No lead consumed.
- Remaining gaps: none in current-source readability. Still open separately: whole-ROM classification
  (Unknown ranges L-001/L-003/L-008..L-010) and final editorial audit/archive acceptance.
