# Local RAM tile clear contract

E1BA resolves cell index throughDF42; negative highbyte skips. Otherwise it writes zero via($32),Y and($11),Y forY=$17 downward, skipping$0F..$08 viaY=$07, stopping onnegativeYornegativecountX. Thus writes remain within source offsets0..23, secondpointer=source+8. CPUstack is never written by these instructions under supported RAM tile-buffer precondition.

Independent producer: CopyFrameTilesToRamE124 startsdestination$0200; copies32bytes perframe record, replaces tileOffsetLo/Hi with that RAMdestination, advances$20. All1510accepted composite records have ≤15sprites(composite-review.json maxrecords15), socachedsources$0200..$03C0 and plane1$0208..$03C8. IndexTilePointersByCoordinatesE0CB transfers these pointers into12gridcells after initializing absenthighbytesFF. DF42 loadsone suchpointer andadds8. Therefore twoSTA atE1C7/E1C9 cannot reach CPUstack/mapper in the pipeline; precondition does not claim arbitrarypatchedpointers safe. All otherindirectwrites remain barriers.

Strong competing interpretation: uninitializedorcorruptcachedpointers couldtarget ROMorstack; excluded by this explicit validpipelineprecondition, not by observedfetches. This is a static routine contract, no naturalreachability ortotalterminationclaim.
