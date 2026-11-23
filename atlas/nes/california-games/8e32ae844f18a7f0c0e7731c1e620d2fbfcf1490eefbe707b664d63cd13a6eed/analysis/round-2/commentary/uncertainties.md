# Commentary uncertainties (rounds 1-2)

- Event overlay RAM $91-$E2: only clearly understood variables are named (scoped per event).
  Foot bag and flying disk variables remain numeric. Round 2 replaced all address-based routine names
  (FootBag_xxxx, Disk_xxxx, HalfPipe_xxxx, Bank6_xxxx) with mechanical/tentative role names; the
  flying-disk actor roles (runner vs catcher) and foot bag pose meanings remain tentative.
- $07-$09 are read only by RandomByte. Round 2: the reset code (PowerOnReset/ClearRamAtReset) clears
  only $21-$FF and $0200-$07FF, so $07-$09 are never written - uninitialised RAM used as entropy.
- PauseOrHoldFlag ($0D), StatusMode ($55) and EventTries ($87) have event-dependent uses; the
  names describe the dominant mechanical use only.
- SprFlags/SprAttr/SprBankFlag: the bank 5 renderer uses SprFlags bit 0 for horizontal mirroring and
  SprBankFlag to select vertical placement direction; other bits not decoded.
- Bank 4 entry 0 (PickRandomMessage) is called by BMX through BmxCallBank4 with X = message group;
  the message texts themselves are in still-generic data.
- Skating SkateFrame_Finish compares the player's name with two 5-character strings ($D17E/$D184)
  before a palette flash - looks like an easter egg; not tested.
- Fixed-bank code reading $8000-$BFFF tables (e.g. PrintPlayerScore -> $A5FF) assumes bank 0 is
  mapped, which holds for every traced context; other contexts are not excluded.
- Phase-1 structural doubts carried forward: cross-window instruction $BFFE/$C000 (L-R1-01, TK-04).
  Reset code and the indirect-table targets were classified in round 2 and are now commented.
- Round 2 residual unknown bytes (L-R2-01, 149 bytes in 18 gaps) carry decode-for-reading notes in
  the source; none has a known caller, so code/data stays undecided.

## Round 2 additions
- ShowEventScore (name kept from round 1) is only reached from SurfRideEnd, so it is the surfing final
  score screen; the five rating digits $D1-$D5 are probably the judges' cards (not confirmed on screen).
- $61 and $4D are written (reset, menu kind 4 actions) but no classified code reads them. Menu kind 4
  rows are read as yes/no on the confirm-players screen and kind 5 as an event list; the screen texts
  for those kinds were not matched to the menu kinds.
- Course script commands (bank 6 CourseCommandTable): $0A/$0C adjust $93/$94 and $10-$16 set $BE;
  the game meaning of these values (and of flag byte $95) is not established.
- Sound command 2/3 semantics (envelope mode / yield) are mechanical readings; command 9 preset
  records were not listened to.
- SurfState5/6/7 interpretations (manoeuvre, airborne, sinking after wipe-out) are tentative.
- BuildSurfScoreBackdrop: the picture content (judges' stand) is inferred; the two pattern copies at
  CHR $0310 and $0810 and the $FF-filled plane for tiles < $64 are exact mechanics.
