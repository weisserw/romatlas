# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.
Round-1 and round-2 lists preserved in analysis/round-1/commentary/ and analysis/round-2/commentary/; this file is the
current state (round 3, pass-004, final editorial acceptance).

## Still open (meaning established only mechanically)
- State bytes: BlobEatState/BlobEatTimer ($81/$02FD: eat/digest timing), FormFlag74, FormEffectSel ($85),
  ObjHover ($39), UnwrittenFlag92 (read by DrawBoy, never written), TorchFlag ($0420: written by
  PickupTorchFlag, no decoded reader).
- Bank 6 set scripts: SetScriptGrid (sets $1E/$1F) and SetScriptHazard are described by mechanics; their
  in-game scenes were not observed.
- Edge markers (DrawEdgeMarkers) and pharmacy offer tables: role inferred from code only.
- Bank 1 $8312/$8313 BIT-skip overlap: source keeps the phase-1 decode; ChanEffectTable proves the hidden entry $8313.
- Reachability of the restart guards SndVecRestart (b1 $8000), ObjVecRestart (b6 $8000) -> RestartViaBank0
  (b7 $C088), SndVecStopSfx (b1 $8009) and AddAToZpWord (b1 $85E0): no caller known; documented as such.
- DrawMetaspriteAlt's A >= $80 path reads $A300 (inside metasprite records); static only, no caller passes such values.
- HummingbirdDYPad (b0 $BA42-$BA43): continues the +1/-1 pattern but outside the index range; generic data.
- DebugTextBoyH (b7 $CBB7 "BOYH",$FF): consumer not found; probably a debug label.
- Frame lists after AnimScriptEntries are assumed to run up to DirStepIndex ($A297); not walked byte by byte.
- RectTileData: blocks from $89E4 interleave with the bank-2 screen-stream bytes; exact ownership of each byte not split.
- Music sequences and screen streams are documented by format, not decoded entry by entry.

## Resolved in round 2
- $0422 = ResetObjectsFlag: ResetPlayerAfterLife sets it on Blobolonia so the next LoadObjectSet uses the empty template.
- All former unknown ranges (L-010..L-022) are now Data/Code/Text and documented at their owning tables.
- SpawnXList1/2 were template fields: GenXList1/2 read the ObjPX field of templates $02/$0E (renamed ObjSetTemplate02_PX/0E_PX).
- BoyLadderFrameTable is indexed by BeanThrowTimer: renamed BoyThrowFrameTable (DrawBoy_Ladder -> DrawBoy_Throw).

## Resolved / changed in round 3 (pass-004)
- b0 $9AB8 is now Code UpdateBoyStop_OrphanRts (Static, structural only): the source says no reachability is claimed and
  why the extra RTS exists is unknown. Competing reading (unread 1-byte datum) is noted in the phase-1 evidence.
- The 24 template $FF runs (ObjAnimTimer/ObjAnimOn slots 3-10 of 12 templates) and TypeBounces $A0B3-$A0BD are Data;
  each run now carries its field name inline; the old 'phase 1 classified these as padding' caveats were removed.

## Structural questions for redirect
- None open. No unknown bytes remain (pass-004). L-006/L-008 (verification/reachability only) and L-009 (partial) are
  unchanged phase-1 leads and do not affect commentary.
