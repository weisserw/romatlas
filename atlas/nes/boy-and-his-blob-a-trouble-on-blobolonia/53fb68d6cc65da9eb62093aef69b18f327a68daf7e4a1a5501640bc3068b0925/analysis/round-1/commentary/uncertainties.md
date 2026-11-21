# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1
- Purpose of several state bytes is mechanical only: $81/$02FD (BlobEatState/BlobEatTimer),
  FormFlag74, FormEffectSel ($85), ObjHover ($39), UnusedFlag92, TorchFlag ($0420), $0422.
- Bank 6 set scripts: SetScriptGrid (sets $1E/$1F) and SetScriptHazard ($0F-$11) are described by
  mechanics; their in-game scenes (probably late Blobolonia / fortress) were not observed.
- Edge markers (DrawEdgeMarkers) and pharmacy offer tables: role inferred from code only.
- Bank 1 $8312/$8313 BIT-skip overlap: source keeps the phase-1 decode; ChanEffectTable proves
  the hidden entry $8313.
- Bank 1 $8000/$8009 and bank 7 $C088: decode as jmp vector entries / unreferenced block, no caller.
- Unknown ranges inside tables, DPCM/PCM sample ranges and stream/music gaps: see leads L-010..L-016.
- Room-link table naming (RoomRightTable $A8C1 vs RoomLeftTable $A9C1) follows MoveObjectRight /
  MoveObjectLeft usage.
