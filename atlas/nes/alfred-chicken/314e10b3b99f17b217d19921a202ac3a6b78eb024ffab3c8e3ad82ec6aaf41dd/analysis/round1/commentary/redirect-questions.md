# Commentary questions for redirect

Continue the accepted round1 leads; commentary added mechanical names, not new classification proposals.

- Flight wave selector physical $12DDF and script reader $135CC: follow `$072C`/`$0735` and timed group records from the saved final-later snapshot to determine later wave/terminal states. Current source explains spawn delays and record mechanics; boss/ending identity is unresolved.
- Resource decoder fixed CPU $FDEF, physical $1FDFF: connect each caller's selected bank, work_ptr source, work_e/f destination and carry-selected RAM/PPU mode to exact resources. This is a route to bank0 mixed boundaries and bank1/2 graphics; valid decompression alone does not prove a whole table/asset extent.
- Actor scheduler physical $1FB0C and state table CPU $8AF5 in mapper bank5: current handler names reveal patrol, slope, support, timer and spawn contracts. Investigate unclassified indirect targets with those contracts, preserving physical identity rather than decoding all possible state values.
- Persistence mapping fixed CPU $F6E0, physical $1F6F0: row-specific relative pointer lists map cell positions to bit IDs. Combined with stage descriptor/map transitions, this may clarify progression/can/ending flags. Bounded matching-row inspection is preferable to arbitrary flag permutations.

Source/readability completion and byte identity do not establish any of these structural or natural-reachability claims. See `commentary/uncertainties.md`, `analysis/leads.json`, and the original disassembly evidence.
