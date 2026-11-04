# Required next current-source commentary corrections

Current source hash is unchanged; this redirect edits no assembly. Reconcile these concrete new findings while commenting the independently accepted next checkpoint:

* MatchSpecialPassword93E2 resets Y=0F at93E4 for every pointer candidate; current prose claiming Y is not reset after mismatch is incorrect.
* CF0E currently labelled DeactivateSelectedEntity writesFF to ActorScriptWait[ActorScriptSlot63], not ObjectType. Explain persistent wait/revisit mechanics and use actor-mode alias, not puzzle ObjectUpdateDelay.
* D77B encoded operands are group, pointerlow, pointerhigh; PHA/PLA rearrangement supplies A/X pointer and Y group. Wrong-order exploratory palette claims are explicitly rejected, not input to classification.
* D433 mode1 has destinationword,width,height,single repeatedtilebyte. D439 mode2 inherits destination and uses width,height,width*height payload; D43F mode0 includes destinationword. The32x32 PPU2800 descriptor contains full1024-byte nametable image including attribute area, not1024 visible tiles.
* Selectorphysical71665 encodes count10 but seven independently supported pointerwords precede first script target71681. Recorded16-byte prefix is a classification coverage claim, not proof that the native interpreter's default skip consumes only16. Explain out-of-domain overlap/defaultskip uncertainty without inventing original layout or global value domain.
* Composite template fields can point to following script/native code placeholders; do not instantiate all31 nominal frame slots. Name finite sprite/table bounds and preserve bank context.
* Bank8 palettegroup45 fourthbyteFF is a raw basepalette value; attenuation/output clamp semantics differ from direct6bit NES palette indices. Do not force every inputbyte<=3F or invent its original meaning.
* New scalar/helpers, callback states, node/coordinate fields, replay pointers/streams, DMC samples and generic patterned gaps need useful mechanical labels/comments. Reachability/observed/artificial provenance remain separate; all this pass's new proposals areStatic.
