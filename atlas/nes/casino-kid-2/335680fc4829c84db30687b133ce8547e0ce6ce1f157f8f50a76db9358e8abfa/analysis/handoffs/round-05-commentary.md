# Round 5 / commentary handoff

Status: **complete** for the round-5 current-source readability gate. Classification: 131,088 /
131,088 bytes (pass-005, 0 unknown). Editorial audit (final acceptance) not run; review.json
editorial_audit is pending.

Baseline: the map changed (pass-004 -> pass-005). Previous commentary was preserved in full at
analysis/round-4/commentary/ (diff -r identical; the round-1..3 copies are untouched). A fresh
baseline was then prepared from pass-005 (state == current.json == pass-005.json, sha256
c79c8cdf...8bc298; prepare verified byte-identical).

## Scope
All classified source: 1,614 code regions, 1,206 data regions and 126 padding regions.
- Names, comments and review were carried forward by CPU address per bank (= physical range).
- Records invalidated and rewritten: bank 3 $9270-$9974, bank 2 $B323-$B702 (and the scene data
  $B6FF-$BB62 that it points to), bank 5 $BAEA-$BC21 and its consumer B5_MatchResultScreen, and
  the headers of the provenance-only regions (bank-3 GAME OVER code, MainLoop_AfterGame).
- review.json: 2,946/2,946 entries reviewed (summary + valid source), 1,872 units, and every code
  region has units.

## Deliverable and reproduction
- Buildable source: commentary/source/game.asm + symbols.asm (with the local image.nes and
  layout.cfg).
- Generator: analysis/tools/round05-commentary/build.sh (gen.py + ann/*.txt), then review.py.
  - This round's patch inputs are in patches-r5/. Round 4's are in patches-r4/ and round 3's in
    patches-r3/.
  - New inputs: ann/consts.txt (numeric constants) and ann/ptrpairs.txt (curated immediate
    pointer pairs).
  - New helpers: ramprose.py (RAM-symbol prose pass), immscan.py and immscan2.py (pointer-pair
    scans).
- Reading copy: commentary/reading-after.asm (refreshed).
- Verification: `commentary.py verify` ok, byte_identical true (commentary/verification.json).
  game.asm sha256 58a89e7c...5c, symbols.asm sha256 7840bf95...16. input/game.nes is unchanged.

## Changes and examples (game.asm line)
- **Bank 3 data:**
  - B3_TierParams 6633: 4 x 18-byte records. The first amount is the opponent's starting money,
    copied by B3_LoadTierOpponentMoney (was B3_CopyBytes8295).
  - B3_MessagePtrs 6643: 4 sets of 38 messages, one voice per opponent.
  - B3_Messages, B3_OpponentNames ($9278-$9290) and B3_PanelLabelStrings (from $9291).
  - B3_SetupTable 2726: `adc #<B3_MessagePtrs` / `lda #>B3_MessagePtrs`, plus the constants
    B3_TIER_PARAM_SIZE and B3_MESSAGES_PER_TIER.
- **Bank 3 routines:**
  - B3_BetAllMoneyMenu 3862 was misnamed B3_PrintAmount. Its grids are B3_BetAllMenuGrid,
    B3_YesNoGrid and B3_DecisionGrid.
  - B3_PostGameScreen 6668 is the GAME OVER screen (observed in the artificial S16 scenario). Its
    data is B3_GameOverBanner and B3_BlankTile2x2.
- **Bank 5:** B5_MatchResultScreen 23041 was "B5_OpponentIntro".
  - It shows the caption by GameOutcome: 1 quit, 2 lost, 3 won.
  - Then it shows one of three closing texts. X = 0 means game over.
  - The money threshold is 100, 1,000 or 10,000.
  - Its tables are B5_ResultFrameBox, Caption*, Closing*, PortraitFrames, PaletteByOpponent and
    GfxByOpponent.
- **Bank 2:**
  - B2_PrologueText/Ptrs (35 lines) and B2_EpilogueText/Ptrs (15 lines; X bit 7).
  - B2_AttractScenePtrs 1821 points to the structure B2_AttractScenePictures: eight 20x6 blocks
    that tile $B6FF-$BACE exactly. This resolves the round-5 disassembly's open "$B6EF-$B702
    layout".
  - B2_AttractSceneAttrPtrs and B2_AttractSceneAttrRows.
  - B2_TitleMetasprite. B2_MenuCursorXTable is narrowed to its 2 real bytes.
- **RAM comments:** StakeTier is 0..3 in game (0 = final match). GameOutcome is 1/2/3 =
  quit/lost/won. New symbols: BjTierStartMoney and BjHandRecords.
- **Editorial fixes from the round-4 list:**
  - 28 immediate pointer pairs are now symbolic (NMI hook vectors, grid and VRAM-string
    pointers).
  - About 75 raw RAM addresses in prose were replaced by symbols.
  - The false "no code reference" wording was removed (the new wording is "no classified code
    operand addresses it directly").
  - Phase-1 evidence notes that used raw "$30/$31 ... $C0A0" wording were replaced by named
    grids.
  - DATA notes are wrapped.
  - Register summaries no longer stop at local branch labels (a generator bug that truncated
    Calls/Reads/Writes).
  - Review summaries are clipped at a sentence boundary.

## Readability assessment
Gate scan results:
- 0 raw-address instruction operands.
- 0 routine headers without a description.
- Every routine entry is named and explained. Uncertain readings are written as tentative prose.

Mechanical data labels: 104 (BankN_Data_/Fixed_Data_/Bank4_Msg_). Generic-data notes: 115.

## Unresolved meanings
See commentary/uncertainties.md, "Round 5" (earlier sections still apply):
- B3_TierParams bytes 8..17, and its second amount (probably a betting limit).
- The OpponentResultBits meaning behind the result-screen thresholds.
- $BB1F-$BB4A: an unreferenced repeat of attribute row 3.
- The one-byte overlap of the result palette/gfx tables at index 9.
- CHR meaning of B3_GameOverBanner and B4_PasswordFieldEnds.
- Api_BuildMetasprite's zpArg2/3 argument when it points at padding.
- "MONEY" at $9291 has no reader.

## Structural note for redirect / final review (no lead filed)
In bank 2, the phase-1 data windows inside $B6FF-$BACE and $BAD7-$BB62 do not follow the decoded
picture and attribute block ends. The region kinds are correct (Data). Commentary documents the
structures; it did not change the map.

## Known editorial issues remaining for the final audit
1. About 3,980 numeric immediates remain. Only pointer pairs, two bank-3 constants and RAM/ROM
   operands are symbolic.
   - Some immediate pairs stay numeric on purpose: 24/32-bit constants (b4 $90AA/$90B7, $8416,
     $E2C8) and uncertain targets (b4 $A004/$8204 via PrintTextWindow, b5 $99A2).
2. About 94 RAM symbols are still undecoded GameVar/PkrVar/RltVar/BjVar/FixVar names.
3. 104 mechanical data labels and 115 generic-data notes remain. Some new mechanical labels
   (e.g. Bank4_Data_BCE5/BCF7, Bank5_Data_B72D/B7A5) are pointer targets whose formats have not
   been decoded.
4. 101 lines exceed 96 columns, mostly register-summary lists (at most 100) and end-of-line
   comments.
5. Many comments still cite ROM CPU addresses in prose (about 750 lines). Most are legitimate:
   ranges after names, or the API slot $C019.
6. Bank 6 sound: undecoded stream command bytes are still annotated as unknown.
7. Raw PPU/VRAM addresses in prose ($2086, $23C9, ...) are left as hardware addresses.

## Remaining gaps
None for the round-5 readability gate. Next: final editorial acceptance (record
editorial_audit, then run `commentary.py audit`) and archive options.
