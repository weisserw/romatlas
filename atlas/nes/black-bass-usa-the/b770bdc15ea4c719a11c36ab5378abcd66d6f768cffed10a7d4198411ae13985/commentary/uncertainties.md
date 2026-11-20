# Commentary uncertainties

Round 1 (pass-001 source):
- Song and sound-effect numbers are left numeric; only their call sites are known
  (e.g. music 3 = fishing, 4 = fight, 8 = menus, sfx 5 = looping reel sound).
- Envelope handlers SndEnvelope1-9: mechanics documented, musical roles unknown.
- PowerMeterState 1 -> 2 -> 3 sequence and CastPhase 1..5 names follow the code; the
  on-screen meaning of each phase (rise/flight/fall/land/splash) is inferred.
- Fish reaction names (jump/run away/turn/pause/hold) are interpretations of the
  velocity changes; heading 6 is drawn in front of the background (treated as a jump).
- RetrieveInput codes (1 Up, 2 Right, 3 Left, 4 A, 5 A+B) come from the button table;
  what each retrieve action looks like on screen was not checked at runtime.
- LureActionProfile/ActionScore: the scoring algorithm is exact; calling it "attraction"
  is an interpretation.
- Rank/place logic (ComputeDayResults, PointsForPlace, CompareTotals) is described
  mechanically; the exact tournament rules were not confirmed in play.
- Bank 5 fishing tables were identified from fixed-bank reads made while bank 5 is
  mapped (DrawFishingView/BuildMapRow/ProbeTerrain leave bank 5 selected); their
  internal formats beyond the pointer level are not decoded.
- Unknown fixed-bank blocks (L-001/L-002) carry decode notes only; several look like
  shared-library helpers (MMC1 serial writes, palette fades, signed divide).
- $C040 jmp ($FFFC) matches GoHome's soft-reset idiom (jmp ($FFFC) at bank 6 $B77B) but
  no reference to $C040 was found.

Round 2 (pass-002 source):
- Song/effect constant names (MUS_*/SFX_*) come from their call sites; MUS_SCREEN_END
  (song 1) and SFX_QUIET (effect 1) are guesses (short order list / replaces the looping
  reel effect).  SFX_LINE_BREAK (7) is also played on slack in FightCheckSlack.
- The no-caller fixed-bank library routines are described from their code only; their
  record layouts (StepObjectDirection, DrawMetasprite fields beyond 10 bytes) do not
  match this game's object records, so field names are kept as Obj0OamBase+N offsets.
- Bank 5: FishK*Frames / LureFrames / SplashFrames internal frame boundaries are not
  decoded; TileMapAB35 ($AB35, 40 bytes) and DataB858 ($B858, 134 bytes) have no
  identified consumer; LureMotionTables' last extent is not proven.
- LureActionProfiles: the first byte of each record is $FF (ScoreRetrieveActions skips
  $FF entries), so the profile effectively constrains counts 1-3 plus the tolerance.

Final acceptance (redirect r002-q5m8 results applied; earlier entries kept as history):
- Resolved: rank/place rules.  ParticipantRank (was RankPoints) is the overall place
  1..200; ComputeDayResults, CompareTotals, CompareBestFish/ImprovePlaceForRival,
  SwapBoundaryPlaces and the ceremony's in-class place are now documented as rules
  (static reading; prize texts 1st/3rd/5th confirmed at run time with artificial places).
- Resolved: TileMapAB35 is LandingBoxTilesCopy (byte-identical to bank-7
  LandingBoxTiles; no reader found).  DataB858 is CastAnimFrames/PowerCursorFrames
  (angler casting animation and aiming cursor, drawn through CastAnimObj/PowerCursorObj).
- Resolved: song 1 is silence (MUS_STOP, audio level measured 0).
- Resolved: CastPhase 5 is a bounce off land/obstacle (CastPhase5Bounce/BounceOffsets);
  phase 3 is described mechanically (half-speed final approach), "fall" is interpretive.
- Still open: SFX_QUIET (effect 1) is probably silence but was not measured; on-screen
  look of fight reactions and retrieve actions; per-frame extents inside FishK*Frames,
  LureFrames and SplashFrames (format known, frames not split); last LureMotionTables
  extent; musical roles of SndEnvelope1-9.
