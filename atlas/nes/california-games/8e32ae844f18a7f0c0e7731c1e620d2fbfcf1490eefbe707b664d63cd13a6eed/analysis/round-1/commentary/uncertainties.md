# Commentary uncertainties (round 1)

- Event overlay RAM $91-$E2: only clearly understood variables are named (scoped per event).
  Foot bag and flying disk variables remain numeric; several routine names there are address-based
  (FootBag_xxxx, Disk_xxxx, HalfPipe_xxxx) because their roles were not established.
- $07-$09 are read only by RandomByte; no classified code writes them (probably uninitialised RAM
  used as entropy, or written by still-unknown reset code at $86ED).
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
- Phase-1 structural doubts carried forward: cross-window instruction $BFFE/$C000 (L-R1-01), reset
  code and indirect-table targets left unknown (see leads L-R1C-01..03).
