# Commentary uncertainties (round 1)

- Object types: behaviour is described mechanically (walker, hopper, platform, shooter...);
  which Looney Tunes character/enemy each type number is was not established.
  Boss behaviours in bank 13 are named BehaviourA-F for the same reason.
- Bonus games (bank 9): the panel game and the second target game are described at the
  flow level; exact prize rules and symbol tables are not traced.
- Special object $0510-$051B (bank 13 UpdateSpecialObject*): role (thrown item vs.
  companion) is tentative.
- PowerUp/StageSpecial handlers ($83E6 table, $0422): only entries 1 and 8 do anything;
  their game meaning (invisibility, timed effect) is tentative.
- Generic data banks: sound data in bank 10 is not split per song; map-block, metatile and
  spawn data extents per stage are not separated; text strings in bank 8 not decoded.
- Phase-1 unknown ranges (L-001 fixed-bank orphan code, L-002 small gaps, L-003 duplicate)
  are explained but unresolved; see notes in source and the handoff's redirect questions.
- $A024-$A025 (bank 13) are limit data decoded as code in phase 1 (see handoff).
