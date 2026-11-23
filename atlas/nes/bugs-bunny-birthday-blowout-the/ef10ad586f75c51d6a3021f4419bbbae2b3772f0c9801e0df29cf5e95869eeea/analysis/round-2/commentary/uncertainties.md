# Commentary uncertainties (rounds 1-2)

- Object types: behaviour is described mechanically (walker, hopper, platform, shooter...);
  which Looney Tunes character/enemy each type number is was not established.
  Boss behaviours in bank 13 are named BehaviourA-F for the same reason.
- Bonus games (bank 9): the panel game and the second target game are described at the
  flow level; exact prize rules and symbol tables are not traced.
- Special object $0510-$051B (bank 13 UpdateSpecialObject*): round 2 showed it is drawn
  into the map as a 2x2 metatile block stepping downward (metatiles $68/$69, then $54);
  what it represents in the game is still unknown.  SpecialObjectBounce is mechanically a
  diagonal tile sweep ($7F/$78); its visible effect is unknown.
- PowerUp/StageSpecial handlers ($83E6 table, $0422): only entries 1 and 8 do anything;
  their game meaning (invisibility, timed effect) is tentative.
- Generic data banks: sound data in bank 10 is not split per song; map-block, metatile and
  spawn data extents per stage are not separated; text strings in bank 8 not decoded.
- Pass 2 classified the L-001/L-002/L-003 ranges; the new routines have no known caller
  (unused variants: vertical mirroring, second-nametable RLE screen, vertical column
  writer / attribute painter, WRAM protect/disable, split scroll).  WrapColumnToTop's
  low-byte EOR looks like an original bug in unused code.
- Remaining unknown: PRG14 $C43F-$C440 ($01,$00) and $C587 ($60) (L-015).
- Resolved in round 2: $A024-$A025 are StageSelectLimits data (L-014).
- Bonus game 1 prize families: MediumToSmallA/B relate each medium symbol to two small
  symbols (tentative reading); the top prize awards 50 lives via AwardLives.
- Write-only variables (no reader found): WarpFlagD0, FlagE0, FlagE1, FlagE2,
  ScrollSpeedXCopy, Unused045E, BonusFlag7A89, HitFlag7A8C; PartGroupSpread's later use is
  not traced.  Zero-page Tmp63..Tmp84 are multi-purpose scratch.
- 278 short helpers/secondary entries have descriptive names but no header comment.
