# Round 3 redirect handoff (attempt r03-a) - COMPLETE, TERMINAL (no ready leads)

- Input checkpoint: pass-003 (maps/current.json = analysis/state.json sha256 08865b21...76a3), round-3
  commentary complete. ROM (8bfff390...), map, state, source and binary unchanged (hashes re-checked).
- Classification: 131088/131088 classified, 0 unknown -> unknown complement empty (no ledger refresh needed).
- Research dir: analysis/redirect/r03-a/ (observations.txt, rtspairs.txt, tools/, frames/, snapshots/
  m2_start.snap + m2_objective.snap, e3*.out, leads-before.json, scenarios-before.json). r01-a/r02-a reused by reference.
- Records: leads.json pass r03-a (updated L002, L005, L007; none added/ready); scenarios.json S13 added.

## Work this round
1. Review of the r02-a rejection review (premises vs primary artifacts) - table below. One premise corrected:
   pass-003 classified 3:$D76E as Code "L007-consistent", but L007's only supporting constraint is a lone RTS
   after a routine-ending tail JMP (3/3); $D76E follows an RTS. Whole-map scan (rtspairs.txt): no other
   unreferenced RTS after a terminal insn. New combined fact: 3:$D76E-$D7EE (129 B) == 1:$D5FF-$D67F, where
   1:$D5FF is the bank-1 routine's RTS -> the bank-1 "rts + BlockSubstTables" lines were duplicated into bank 3,
   so the byte is a copied RTS instruction: Code role stands, rationale changes. Padding reading predicts the
   same byte (not discriminating) -> no classification proposal; editorial note for final audit only.
2. Optional $6B check (uncertainties item 9) - answered, artificially (S13, observations.txt B):
   records 1/5 (missions 2 and 6) map $F7/$F8 -> $69; $F7 lies in section $24 row 11, $F8 in $28 row 8.
   Replays with/without BlockSubst+7/+8:=$6B differ only where $F7 is drawn: $69 = dark recessed opening in a
   raised platform, $6B = closed flat panel (frames/e3_block69_vs_6b_038_zoom.png); RAM trace shows the ground-
   height path reads BlockSubst+7 over the block. Objective $42 sits in ObjType slot 15 for the section.
   Commentary fix (final editorial audit): replace "probably the destroyed-target picture" in
   ObjectHit_ObjectiveHit / uncertainties item 9 with the observed mechanical effect (opening -> closed flat
   panel, height entry changes; game meaning interpretive). Natural missile hit not achieved (E6, stopped).

## Rejection review (r02-a items re-examined + r03-a)
| Claim | Decisive artifact | What it actually rules out | Strongest competitor | Disposition / reopen |
|---|---|---|---|---|
| No code<->data misclassification | r02-a/audit.txt, datascan.py (pass-002); pass-003 diff only L009 (data merge + 1 RTS) | references into data, misaligned code, unofficial opcodes, unexplained fall-through | clean-decoding unreferenced data | premise sound; pass-003 change does not affect audit scope. Closed; reopen on a reference into data |
| 13 no-caller code ranges Code (L006-L008, +3:$D76E) | r01-a/l001-fragments.json, evidence/round-02/frag-verify.json, r03-a/rtspairs.txt | nothing falsified; classification is structural (bypass jumps, byte-identical copies, internal targets), reachability separate | lone RTS = padding | L006/L008 sound (multi-constraint). Lone RTS: L007 sound for 3 after-JMP bytes; 3:$D76E rationale corrected (copied block), role unchanged. No runtime test discriminates code vs padding for an unexecuted byte. Reopen only if a data consumer reads them |
| No-reader data 3:$D76F copy, 3:$D90A, 2:$FE65, 3:$F1C3, 1:$FE16-$FE21 | r02-a/datarefs.txt, dupscan.py | same-bank abs/indexed readers, imm-pair pointers, duplicates | unread leftovers | Data (generic is sufficient per policy); meaning-only. Reopen if a pointer/descriptor targeting them appears |
| RLE overlaps (1:$80C9, 0:$CA01/$CAF8) | uncertainties item 6 decoded lengths | n/a - both readings data | shared bytes vs miscount | no classification effect |
| L002 RAM images Data | toolkit docs/nes.md:91-92 (re-checked r03-a) | uploaded execution views | Code-with-upload-view | blocked on missing toolkit feature; images correctly Data in ROM |
| Block $6B meaning | r03-a/observations.txt B, zoom frame | "unobserved" status | "destroyed-target picture" wording | answered (editorial wording change only) |

No useful revised inference or discriminating test with classification effect survives the review.

## Gameplay milestone review
Visited (S01-S13, mostly with RAM shortcuts): boot/attract/title, briefing, missions 1-8 flight, game over +
name entry, objective -> climb-out -> space battle -> docking -> shop, carrier landing (missions 4/8), final
base attack -> ending/congratulations, controller-2 debug codes, mission-2 objective area (S13).
Not visited (deferred, meaning-only, no unknown bytes): natural Maverick hit on objective $42/$43 (E6 failed;
needs object screen position / missile geometry), A/B star-speed bits in space, natural unpatched completion.

## Recommendation
Terminal for redirect: no ready leads. Proceed to final acceptance (editorial audit); fold in the two editorial
notes above (3:$D76E rationale; ObjectHit_ObjectiveHit/$6B wording). Disk: ~3.0 GB free; r03-a ~4.3 MB.
