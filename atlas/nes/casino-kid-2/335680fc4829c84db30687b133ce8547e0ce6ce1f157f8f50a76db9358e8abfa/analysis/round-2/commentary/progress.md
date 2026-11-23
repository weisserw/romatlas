# Commentary progress

## Round 2 commentary (2026-11-23)

Baseline: map changed pass-001 -> pass-002 (maps/current.json == analysis/state.json, sha256
a228e814...0d36). The whole round-1 commentary/ was copied to analysis/round-1/commentary/
(diff-verified identical) before `commentary.py prepare` built the fresh frozen baseline
(before/, reading-before.asm, baseline.json, phase1-state.json, review.json scaffold 2,878 entries).

Method: round-1 pipeline copied to analysis/tools/round02-commentary/ (round01-commentary kept
unchanged as the historical record). `build.sh` regenerates source/game.asm + symbols.asm from the
new before/ plus annotations (ann/bank0-7.txt, ann/ram.txt, ann/structs.txt, generated
ann/structs-gfx.txt); `annpatch.py` applies annotation patches; `review.py` fills review.json.
Names/comments carried forward by CPU address within each bank (= physical range); every carried
name was revalidated by the generator (names must land on instruction starts; build byte-identical).

Generator changes: bank-4 2-byte inline callees (message pointer / grid-menu descriptor) handled
like the other inline helpers (continuations no longer become fake routines); inline message
strings decoded into comments; 32-bit RAM symbols (Name+1..+3); bank-scoped RAM symbols; immediate
pointer pairs to named tables rendered as #<Name/#>Name; structure-range notes (graphics sets,
pictures, palettes, DATA ranges) for every data piece; condensed phase-1 evidence notes for new
data regions; raw inline RAM addresses replaced by symbols (0 raw RAM operands remain).

Scope: all classified source (1,469 code regions, data and padding) plus the 2 unknown bytes.
- New code from pass-002 named and explained: poker chip selector/bet validation $8DEA-$90D0,
  hold-card selector $914D-$91C4, B5_UpdateWheelAttributes $831B, MainLoop_GameFinished $C197,
  CallFarInline_Return $E93B, and the 7 low-confidence fragments (B2_UnusedReturn,
  B4_UnusedCursorReset, B5_UnusedRoundEnd, B5_UnusedMouthFlip, B6_UnusedClearChannelField,
  MainLoop_UnusedEndingJump, FetchTextChar_UnusedMarkMap) with reachability caveats.
- New data named: CardBlockTable/CardBlocks, VramStructListTable/Lists/ChipPanelVramStructs,
  NmiRouletteJobTable, ChipValuePtrTable, MetaspriteTileLists, SpritePaletteStruct,
  RouletteSpriteSet*, WorldMapMarkerYTable, YourPasswordText, DpcmSample0-2/CopyTail,
  B4_HoldCardGrid/YesNoGrid/SeeFoldGrid/SelectMenuGrid/DebugMenuGrid/TierRuleTable,
  B5_AttrRow*/B5_ChipValueTable/B5_BoardCellTable/B5_MouthTilePairs, bank-6 sound tables,
  GfxDescriptorTable0-2, B2_TitlePaletteBlocks/B2_MenuCursorXTable; 45 graphics sets, 6 pictures
  and the palette block described by range notes; phase-1 text-window/data-window signatures quoted.
- Concrete round-1 defects fixed: bank-4 mis-segmentation (~25 continuations shown as routines) and
  wrong roles (see uncertainties.md "Resolved or revised"), PrintTextById/TextPointerTable ->
  DrawCardBlock/CardBlockTable, PrintTextList -> DrawVramStructList, B5_StateSettle/ToggleState
  (really +/-2 bet-state cases), B5_OpponentIntro (post-match), bank-6 header-table addresses,
  B4 PrintMessage* headers, fall-through notes for ClearTextRows/QueueWorkBufferLine, stale
  UNKNOWN/redirect notes for ranges classified in pass-002.
- Unknown $BB30-$BB31: explicit note with both readings (R2-L021).
- review.json: 2,878/2,878 scaffold entries reviewed with summary + source line, 1,515 units;
  editorial_audit pending (final acceptance only).

Resume (coordinator request): all raw-address operands replaced by symbols (0 left): 120
mechanical bank-qualified labels (BankN_Data_XXXX / Fixed_Data_XXXX / Bank4_Msg_XXXX) with reader
lists in symbols.asm, Name+offset inside named ranges, Bank2_GfxDescriptorPtr, B4_MoneyCapValue.
The 184 generic blocks without a known reader have mechanical notes (size, range, checks made,
nearest named data, untraced indexed/pointer access caveat); blocks reached only by inline operands
say so. Remaining limits: immediates mostly numeric; mode-local RAM partly bank-scoped names.

Verification: `commentary.py verify` ok, byte_identical true (commentary/verification.json,
game.asm sha256 7f9472c5...8c67, symbols.asm fcf0a14b...142d); reading copy refreshed
(commentary/reading-after.asm). Classification unchanged (131,086/131,088).

## Round 1 commentary
See analysis/round-1/commentary/progress.md.
