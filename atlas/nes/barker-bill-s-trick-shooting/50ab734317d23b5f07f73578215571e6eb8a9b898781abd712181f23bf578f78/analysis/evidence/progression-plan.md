# Round 1 bounded state initialization experiment

Supported main dispatcher: fixed PRG CPU C0DF loads $0600, calls C52C, followed by pointer words. C52C multiplies selector, pops return pointer into $C1/$C2, reads inline low/high target and jumps. Reset initializes $0600 and $0624. The observed title-state routine at bank0 CPU8000 consumes $0624 as initialization countdown. Initial experiment $0600=0,$0624=1 returned title and bank0.

Source snapshot analysis/evidence/title-shortcut-snapshot.json: frame1821, context [0,1,6,7], title state, artificial from this initialization. Every following experiment restores that file, releases all buttons, writes RAM $0600=selector and $0624=1, then executes 45 frames with 2,500,000 tick budget and 15s timeout. Inspect rendered frame, effective bank and instruction coverage, retaining actual fetched instructions only. Selectors1..9 are the finite inline dispatcher hypothesis; do not classify target addresses based on pointer values alone. No CPU-register or ROM edits.

Preliminary 15-frame probes: selector1 balloons context[4,5,6,7]; 2 flying-saucers transition context[2,3,6,7];3 window gallery context[0,1,6,7]. These establish artificial visits. Full results in progression-experiments.json.
