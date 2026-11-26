# Round 1 - redirect handoff

Status: complete (ready batch for disassembly). Attempt dir: analysis/redirect/r1-a7/ (notes.md, scripts.py,
swalk.py, cov.py, cmp.py, lab.py, bank6-screen-scripts.txt, observed-*.json).
Input checkpoint: pass-001-final (98.30%, 2,225 unknown). No map/state/source/ROM/toolkit change.
Workspace-tool change only: analysis/tools/rpcd.py and nes.py read the scratch path from env ROMATLAS_SCRATCH
(default unchanged) so servers of other workspaces are not shared.

## Avenues investigated and outcomes
1. Frame-hook / far-call installers (commentary Q1): decoded the bank-6 screen-script block $BB48-$BE8D with the
   RunScreenScript command set. It parses cleanly and gives explicit entries ("op 01 far call addr,bank") for every
   large unknown block: $832B (boot script $BB6A), $830E/$831A ($BB92/$BBB4), $F885/$F899 bank 7 ($BDF9/$BE32,
   chosen by 11 bank-2 opponent-turn messages through menu op 08), $95F6 ($BE46, run by ShowIntermission).
2. Runtime confirmation (bounded traces): boot frames 140-1340 (natural) observed $830E-$8325, $832B-$8644,
   $FC65-$FC78; opponent-turn message observed $F885-$F8AC, $899E-$89C0, $8A51-$8A74, $E3A3-$E3E1 (route had a
   1-byte RAM patch, CalledPocket); artificial GameCmd $19 observed 130 starts in $95F6-$981F. 0 observed starts
   off the static walks; bank context [6,7,14,15] for bank 3.
3. Table extents with consumers: $8645 (4x8 records), $88A8-$899D (8-entry portrait tables, 11 name pointers ->
   11 strings, 11-entry portrait index with values 0-7), $8A76 (16x8 tile rect), $8B00 PanelLayout, $977D-$97B2,
   $9820-$987F (12x8 digit font), $9880, $E3E2-$E3E6 (Reset signature).
4. Q5 TitleSkipWord: written by the $832B title routine ($835A/$835F, decremented $8458/$845B), not by a script.
   Q6 RackPreviewHook: installed by the GAME MODE menu scripts ($BC45/$BC70). Q3: party NINE BALL -> GameType 1
   confirmed at runtime; Freestyle GameType $13.
5. L-007 shot contract and progression: resolved (see milestones).
6. L-006 small fragments: grouped into structural families (leads.json L-006).

## Ready leads (analysis/leads.json; exact sub-ranges in each lead's refined_ranges)
| Lead | Ranges | Proposed role | Bytes |
|---|---|---|---|
| L-001 | b3 $830E-$8325; fixed $FC65-$FC78, $F885-$F8AC; $FB32-$FB3F | code (first three Observed; $FB32 Static by trampoline contract, no caller) | 98 |
| L-002 | b3 $832B-$8644 minus two 4-byte dead fragments; $8645-$8664, $8665-$866B | code (Observed) + data (consumer-anchored) | 825 |
| L-003 | b3 $95F6-$9880 | code $95F6-$977C, $97BB-$981F (+ $97B3-$97BA Static wait loop); data $977D-$97B2, $9820-$987F, $9880 | 651 |
| L-004 | b3 $88A8-$89D6, $8A51-$8B13 except $8A75 | data tables/strings, code $899E-$89D6, $8A51-$8A74, data $8A76-$8AFF, $8B00-$8B13 | 497 |
| L-005 | fixed $E3A3-$E3E6 | code $E3A3-$E3E1 (Observed), data $E3E2-$E3E6 | 68 |
| L-006 | ~40 fragments (families) incl. $8469-$846C, $8476-$8479, $8A75 | low-confidence code (compiler dead code / alternate prologues) or leave unknown | 86 |
Total addressable: 2,139 bytes with strong evidence + 86 low-confidence = all 2,225 unknown bytes.
Recipe: annotate per refined_ranges; Observed provenance from the observed-*.json files (check with
`python3 analysis/redirect/r1-a7/cmp.py <json> <roots> <start> <end> <bank>`), rest Static with the evidence text;
note artificial-route status for observed-oppmsg / intermission. L-008 is commentary-only (split bank-6 scripts).

## Rejected / blocked
- None rejected. Reachability not claimed for: $FB32-$FB3F (no caller), $97B3-$97BA (no reference),
  L-006 fragments (unreachable by construction), $8A76-$8AFF (no reference; classification as generic data only).
- Q4 (RunMenu reads of $2000/$2001) not pursued: commentary-only, no classification impact.

## Gameplay milestones (analysis/scenarios.json S8-S13)
- Natural: boot/title trace; tournament manual break shot; Freestyle (shot, ball-move); Party 9-ball break.
- With RAM patches: opponent-turn portrait message; intermission scoreboard; World Championship (round 5);
  tournament win + Joe Nozerak interview ending "WISHING YOU A GOOD NIGHT" -> title.
  Shortcut: at aiming write TournamentRound $EF=n-1, CurrentPlayer $E6=0, MatchExitCode $DD=2.
- Not visited: Challenge challenges completion (WorldChampUnlock path) - deferred, round 5 reached directly.

## Recommendation
Run another disassembly round with L-001, L-002, L-003, L-004, L-005 (and L-006 per its family rules), then
commentary (new code: title animation, credits pages, portrait/name screen, intermission scoreboard; L-008 script
labels). Not terminal.

## Toolkit issues
No toolkit issues found in this phase (analysis/toolkit-issues.md unchanged; TK-002 slowdown still applies).

## Disk
Scratch /tmp/ra-2707-r1a7 removed; attempt dir ~0.5 MB.
