# Round 1 redirect handoff — supported batch ready

Input checkpoint: pass-001-final. ROM, maps/current.json, analysis/state.json, checkpoint and current commented source remain unchanged. Map/state SHA256: b4cbd4690d6850d5155f0bc94dafcf48607ac1fab283289ff2d6c080043e9189. Source SHA256: 82974701ab5a7bef74fffdc36b920ea20caaba5a41b9274d9802a53b7342ee12. Original ROM SHA256 retained.

Classification remains **25,107 / 131,088 (19.15%)**, with **105,981 unknown bytes**. Redirect accepted zero new bytes. This is a productive handoff for independent disassembly, not terminal exhaustion.

The combined proposal contains **57,246 new unknown bytes**, 303 candidate records, with zero conflicts against accepted kinds and zero internal Code/Data conflicts. Candidate read views can overlap; use the authoritative union rather than summing candidate records. All proposals use Static provenance and include physical offsets, CPU aliases, physical PRG banks and effective AxROM context.

| Ready lead | Proposed unknown bytes | Evidence resolved |
|---|---:|---|
| L-R1-GFX | 41,609 | All three identified graphics branches: finite bit-tree and escape-run decoding, literal endpoints, 65 source slots with null/RAM exclusions |
| C-R1-MAP | 9,254 | Bounded run sources, property/metatile domains and map-related noncode spans; executable initializer split out |
| L-R1-DISPLAY | 4,082 | 33 display scripts, 149 ROM text roots, 55 dictionary slots and seven PPU-run selectors |
| C-R1-SPRITE | 1,550 | 152 nonnull pointer entries / 96 unique terminated descriptor views and renderer continuations |
| L-R1-STATIC | 388 | Selected neutral-return continuations and exact BRK operand barriers |
| L-R1-ROOM | 363 | Corrected NMI prerequisite, transition routing/code, 23 cursors and four 38-entry transition columns |

Authoritative research artifacts: analysis/redirect/r1-20261119-a/report.md; proposals.json; validation.json; unknown-ledger.json. The ledger preserves all 369 current unknown ranges (105,981 bytes), effective context, accepted neighbors, evidence and candidate intersections. Persistent analysis/leads.json retains old failed/verified history and marks all six IDs ready, with bounded verification recipes. The attempt directory is local research evidence, not a portable map.

Independent disassembly should review the exact proposals, not blanket-classify bounded linear decode outputs. Some exploratory roots hit operands or mid-instruction bytes and were explicitly rejected/corrected. Retain inline two-byte BRK barriers and return/tail-bank contracts; split candidates against existing accepted regions. Decoder scripts distinguish bytes read from bits consumed and independently encoded endpoints. In particular, selector26 overlaps other bit-tree source views, two RLE tails remain unresolved, and property/metatile read views may share bytes. These do not assert an original serializer layout. Root-tree flag bit1, the nine-byte 8349 stream, the narrowed 54-record metatile span and the executable A497 initializer are corrected premises preserved in the report.

The failed room shortcuts were corrected by replacing the active task pointer $8E15 with task0's $FEB6 RTI before entering the initializer. Bounded artificial runs now visited overhead rooms9/18, outdoor sideview room1 and compact platforming room2, with actual instruction execution and movement/action evidence. Measured upward movement: room9 19 pixels; room18 22 pixels. Room1/2 captures show airborne players after 25 accepted action frames. No unsupported boss, sewer or graveyard identity is assigned.

Scene31 rendered a search gate, not an ending. Text-derived victory script24 was rendered and completed through bounded timer shortening: completion $062C=1 at frame1404, followed by prior-room resource loading. This is artificial farewell dialogue completion, not a natural ending, credits or terminal-state claim. analysis/scenarios.json preserves all earlier failures and adds current individual milestones and actual missing prerequisites. Successful routes, screenshots, snapshots and exact calls are retained locally.

Next commentary must correct two narrow existing mechanical descriptions alongside the new ranges: sprite FF is the flipped-header prefix and 80 the command terminator; text command1 returns/unwinds and commands2..7F enter dictionary streams. Prior unrelated readability review remains valid; the current source itself was not edited here.

The identified graphics branches and selected map/sprite/display graphs have been followed through their supported continuations. Further natural progression requires actual room/scene identity and gate/flag contracts; repeated scheduler-only edits are no longer the method. Other pointer families and remaining opaque intervals need new concrete consumers or entry/boundary evidence, not an arbitrary actor/room sweep. These are remaining frontiers for the next evidence-informed redirect, not a whole-ROM rejection or exhaustion finding.

All runtime servers and probe sessions have finished; no live writer remains. Configured toolkit binary was used without modification. No ROM/source/map/state edits, toolkit edits, deletion, cleanup, archive or publication occurred. Disk checked: about4.4GB free; attempt about66MB before final small handoff records.

Next step: independent disassembly acceptance/classification, byte-identical checkpoint, full current-source commentary including the two corrections, then redirect with refreshed residuals. Preserve prior evidence and these exact frontier distinctions.
