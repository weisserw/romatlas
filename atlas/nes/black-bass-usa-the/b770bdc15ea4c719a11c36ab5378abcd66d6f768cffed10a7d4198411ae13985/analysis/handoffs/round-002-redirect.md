# Round 2 redirect handoff (attempt r002-q5m8)

Status: complete (phase finished; recommendation below). Input checkpoint: commentary-002
(maps/commentary-002.json, sha256 530ccd61...87; 131088/131088 classified, 0 unknown).
No map, source, state or ROM changes; research/lead/scenario records only.
Records: analysis/redirect/r002-q5m8/ (findings.txt, terminal-rts-survey.txt,
nocaller-duplicates.txt, tools/*.py, frames/*.png). Previous attempt r001-a7k2 untouched.

## Scope
1. Weakest classifications ($C040, lone rts $D76C/$E24A/$E30F, 1302 B no-caller code).
2. Semantic uncertainties: L-009 bank-5 residue, ranking/ceremony rules, cast phases,
   song 1, greeting text; fight/retrieve on-screen meaning triaged.
3. Unvisited milestones: prize text, Lake More, Japan Lake, LOOK AT MY RECORD, HBMAX HUD.

## Findings
- Weak claims: no contrary evidence. $C040 starts the third 16-byte-aligned jump-slot group
  ($C020 / $C030 / $C040, zero-filled gaps) right before ResetMain and is byte-identical to
  GoHome's jmp ($FFFC). Of 16 rts bytes that follow a terminal instruction in code, 13 are
  branch targets ("jmp X / @return: rts"); the 3 lone ones fit that idiom, but 15 routines
  end with a jmp straight into the next entry, so this is weak support only. Keep all
  four as code (0.7/0.6). No-caller code: only known sibling/copy duplicates found.
- L-009 answered: DataB858 = angler cast animation (5-frame table $B858, 14-sprite frame 0,
  frames 1-4 continue via $FD into frame 0) + aim cursor (table $B862, sprite $B864, tile
  $1C); used through the PowerCursorObj/CastAnimObj records ($CF9B/$CFA5, Frames field)
  with bank 5 mapped (confirmed at runtime). TileMapAB35 is a byte-identical copy of
  bank-7 LandingBoxTiles with no reader. Sprite-list entries are attr, dX, dY, tile.
- Ranking rules decoded (RankPoints = overall participant place). Two current comments
  are reversed (PlayerBeatsRivals, CompareTotals). Ceremony uses the in-class place; prize
  texts captured: 1 winner, 3RD, 5TH (artificial RankPoints).
- Song 1 is silence (audio RMS 0 after the request). CastPhase 5 is a bounce off land
  (offsets hop up 32, down 48). DrawGreeting comment order is wrong; "M;." shows "Mr.".
- Gameplay: Lake More (index 1) and Japan Lake (index 2) reached through generated standing
  codes (class picks the lake); LOOK AT MY RECORD and HBMAX debug HUD visited (S-016).

## Ready leads
None. No classification or structural change is proposed. L-009 is answered and needs only
editorial label/comment work (0 bytes). Leads updated: L-002, L-004, L-005, L-009.

## Readability findings for the final editorial pass (14)
R-01 DataB858 -> label/split as CastAnimFrames ($B858), PowerCursorFrames ($B862),
     PowerCursorSprite ($B864), CastAnimFrame0 ($B869), CastAnimFrame1..4 ($B8A2/$B8B1/
     $B8C0/$B8CF); note the consumers PowerCursorObj/CastAnimObj and show their Frames
     field as .word labels (findings.txt B.1).
R-02 TileMapAB35 -> LandingBoxTilesCopy ("byte-identical to LandingBoxTiles $E365; no
     reader found"); remove it from uncertainties.md.
R-03 Sprite-entry order: fix "dy, dx, attributes, tile" to "attributes, dX, dY, tile" at
     DrawObject (game.asm ~7656) and the FishK*Frames note (~1313).
R-04 PlayerBeatsRivals header: A=$FF when PlayerTopTotal < RivalTotal (reversed now).
R-05 CompareTotals header: a deficit worsens the place from class best by
     (RivalTotal-PlayerTopTotal)/LakeTotalDivisor, capped (LakePointCap) at class worst;
     a total >= the rivals' keeps class best.
R-06 RankPoints -> ParticipantRanking (overall place 1..200, lower better; A 1-50,
     B 51-100, C 101-200) or keep the name with that comment; LakePointFloor/LakePointCap =
     per-lake class best / class worst+1 (the lake follows the class).
R-07 CompareBestFish/PointsForPlace: each top-3 fish beating rival best #1/#2/#3 improves
     the place by 3/2/1, floored at class best.
R-08 CeremonyScreen/ShowPrizeText/PrintPlace: in-class place 1-5 (RankPoints minus
     ClassPointSteps); Txt_YouGotA2nd is a template whose "2ND" is overwritten by the ordinal.
R-09 AdjustPointsAtBoundary: promotion/relegation swap (46-50 <-> 51-55, 96-100 <-> 101-105).
R-10 MUS_SCREEN_END -> MUS_STOP (song 1 is silence, measured); update uncertainties.md.
R-11 CastPhase5Splash -> CastPhase5Bounce, SplashOffsets -> BounceOffsets (lure bounced off
     land/obstacle); describe phase 3 mechanically (half-speed advance to Y $50).
R-12 DrawGreeting header order: A ",SIR.", B "M;. (NAME)," (displays "Mr."), C "HEY (NAME),".
R-13 $C040 / lone-rts comments may cite the slot-group layout and the 13-of-16 epilogue
     survey (keep "weak claim" wording).
R-14 commentary/uncertainties.md: mark resolved items (ranking rules, prize text,
     TileMapAB35/DataB858, song 1, CastPhase 5) and keep the open ones (fight-reaction and
     retrieve visuals, SFX_QUIET, per-frame extents).
Plus the commentary-002 handoff's known editorial issues (generic @labels, numeric PPU
immediates, CopyTilesInline raw words, sound-number prose, review summaries).

## Deferred avenues (reasons)
- Natural unaided catch / natural winner ceremony (S-017): needs a full manual day beating
  random rival totals. Every routine on the path is already observed with aids, and no
  question depends on it.
- On-screen fight reactions / retrieve actions: the names already have three independent
  supports (velocity mechanics, SFX_FISH_JUMP, the draw-in-front flag). A jump needs a
  bass in the shallowest band, which happens at random. Only wording would change.
- Per-frame splits of FishK*Frames/LureFrames/SplashFrames: mechanical with the known
  format. This is editorial, not a structural question.
- New caller methods for no-caller library code: r001's four reference methods plus this
  duplicate search found nothing. No further method is available.
- SFX_QUIET identity: probably silence (same role as song 1), but it was not measured. It
  changes a name only.

## Recommendation
Proceed to final acceptance (editorial audit + verification), applying R-01..R-14. No
disassembly/commentary cycle is warranted: there are no ready leads that change
classification or structure. Further redirect has no promising actionable avenue.
Toolkit: no new toolkit issues found (analysis/toolkit-issues.md unchanged).
Servers stopped; scratch /tmp/r002scratch removed.
