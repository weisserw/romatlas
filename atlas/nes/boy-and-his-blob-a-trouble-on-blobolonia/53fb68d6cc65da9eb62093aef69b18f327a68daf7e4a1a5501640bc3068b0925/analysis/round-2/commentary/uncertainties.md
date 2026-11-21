# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.
Round-1 list preserved in analysis/round-1/commentary/uncertainties.md; this file is the current state (round 2, pass-003).

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

## Structural questions for redirect
- L-023: b0 $9AB8 remains the only unknown byte (second RTS; no reference).
- L-024 (new, role precision only): Padding-classified $FF runs inside tables - TypeBounces $A0B3-$A0BD and the 24
  8-byte runs at template offsets $3A/$50 (ObjAnimTimer/ObjAnimOn slots 3-10). Non-code either way.
