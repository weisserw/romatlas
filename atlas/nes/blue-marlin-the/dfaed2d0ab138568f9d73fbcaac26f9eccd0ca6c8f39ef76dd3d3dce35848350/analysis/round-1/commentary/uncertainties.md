# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1
- Bank context of fixed-bank calls into $8000-$BFFF: resolved only where the mapping is
  established (xb directives in annotations.txt). Fight_BuildScreen ($E6C1) calls $ADC0
  and $92A7 which are data bytes under banks 0/1 -> real mapping unknown (Q3).
- Bank 1 $BA2E static region is a misaligned decode of the routine at $BA2A (Q1).
- Bank 0 unknown ranges $8B81-$8D15 are called from fixed-bank wrappers (Q2); not
  reclassified in commentary.
- Meanings marked tentative in source: TimeOfDayScene values, LineType ($0762),
  PlayerStat0760/0761, FightCounterC5/C7, FightGauge, fight events 1-9, outcome codes
  $80/$81 (LineOut 0 / >= $200), sea events from BoatCrashCount, Sun_Update object,
  title/name-entry script natives in banks 12/13, sound commands $FB/$FC.
- $06E1-$06E4 RAM code: Sound_ResetAll stores $8D/$40/$60 there and Snd_ShadowWrite
  stores the register index in $06E2, i.e. a self-modifying "STA $40xx / RTS" helper
  (resolves the upload-origin part of lead L-006; execution of $06E1 was observed).

## Round 1 continuation
- Fight event script (bank 15 $E5DC-$E5FD, still unknown): decodes as ScrCmd_Switch on
  SeaEventFlag + 15-word table; target scripts in bank 12 print texts that match the
  event triggers. FEV_* constants and the FightEvent_* names rely on this reading and on
  bank 12 being mapped at $8000 during the fight (consistent with L-011 but not traced).
  Lead L-012. FEV_ROD_BROKEN ($0A) has no direct writer found.
- GloveHeat / ReelHeat (were FightCounterC5/C7) and FightGauge as hook-hold/exhaustion
  meter: names from the event texts their overflow/limits raise; tentative.
- Fight fish model: renamed by the value each helper updates (Strength/Stamina/Fatigue/
  Direction terms). The game meaning of the four values and of individual table terms is
  inferred only. Two likely original quirks are documented in source (FightFish_DirectionTerms
  uses `tya` for `tay`; FightFish_PassiveFatigue indexes with X).
- Sound driver: Snd_SweepUpdate was really the music-pulse volume envelope (renamed
  Snd_EnvelopeUpdate, SndEnvLevel/SndEnvPhase); SndTempoScale -> SndVolumeScale,
  SndScale -> SndGateScale, Snd_WriteVolumeReg -> Snd_WriteSweepReg (writes shadow byte 1).
  Command semantics remain code-derived, not verified by ear ($DF two-note effect unclear).
