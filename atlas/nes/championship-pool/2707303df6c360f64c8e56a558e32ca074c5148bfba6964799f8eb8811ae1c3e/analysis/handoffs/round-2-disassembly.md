# Round 2 - disassembly handoff

Status: complete-pass. Every byte now has a structural classification. The editorial/commentary gate is still open
for the newly classified code.

## Scope
- Restored analysis/state.json (pass-001-final, 2,225 unknown). Consumed ready leads L-001..L-006 from
  analysis/handoffs/round-1-redirect.md and analysis/leads.json. L-008 was skipped because it is commentary-only.
- Before annotating, re-checked each lead: walked the code statically from the entries the screen scripts decode to
  (exact tiling, official opcodes, no overlaps), took Observed provenance per instruction start from the saved
  redirect traces with the same effective bank, and re-counted the in-bank absolute reads for every table.
- Upgraded fully observed Static code regions to Observed (analysis/tools/upgrade_r2.py), using the redirect traces
  and a new traced ending route (S14).
- Tools added in analysis/tools/: annotate_r2.py (dry run by default, `--apply`; idempotent), upgrade_r2.py,
  r2play.py, r2_ending.py.
- Fixed a workspace tool: the rpcd.py daemon no longer crashes when a client is killed mid-call.

## Checkpoint and verification
- `pass-002-final`: maps/pass-002-final.json, generated/pass-002-final, analysis/pass-002-final-verification.json
  -> ok: true, byte_identical: true. This is the output checkpoint.
- `pass-002`: the lead annotations only, before the S14 provenance upgrades. Also ok and byte_identical. Superseded.
- map.validate ok. State saved to analysis/state.json and exported to maps/current.json (same content as pass-002-final).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Bank | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 |
| PRG 0 | 0 | 16165 | 219 | 0 |
| PRG 1 | 0 | 16384 | 0 | 0 |
| PRG 2 | 3495 | 12755 | 134 | 0 |
| PRG 3 | 15023 | 1361 | 0 | 0 |
| PRG 4 | 0 | 16369 | 15 | 0 |
| PRG 5 | 13298 | 3045 | 41 | 0 |
| PRG 6 | 2309 | 13705 | 370 | 0 |
| PRG 7 | 11619 | 4676 | 89 | 0 |

By provenance:

| Kind | Provenance | Bytes |
|---|---|---|
| Code | Observed | 23,556 |
| Code | Static | 22,188 |
| Data | Static | 84,476 |
| Padding | Static | 868 |

Bytes gained this pass: 1,628 code (1,254 Observed, 374 Static) and 597 data. Provenance upgrades: 38 regions
(971 bytes). Execution coverage is a separate metric: the S14 ending trace alone gave 4,976 unique starts, and none
of them fall off classified instruction starts.

## Lead outcomes (analysis/leads.json)
| Lead | Outcome |
|---|---|
| L-001 | verified. $830E-$8325 and $FC65-$FC78 are Observed (natural boot). $F885-$F8AC is Observed on the opponent-message route, which had one RAM patch. $FB32-$FB3F is Static code at confidence 0.6, inferred from the trampoline contract; no caller is known. |
| L-002 | verified. Code $832B-$8644 (767 B Observed, 19 B Static). Data $8645-$8664 (4x8 records, 8 consumer reads) and $8665-$866B (7 reads). |
| L-003 | verified, with a corrected boundary. Code $95F6-$977C and $97BB-$981F (273 B Observed under the ARTIFICIAL GameCmd $19 patch, 219 B Static). Data $977D-$978F, $9790-$97A2, $9820-$987F and $9880. Correction: the word table is $97A3-$97B0 (7 words, not 8). The wait loop is code at $97B1-$97BA (Static, 0.6) because its beq targets $97B1. |
| L-004 | verified as proposed. Tables $88A8-$899D, code $899E-$89D6 and $8A51-$8A74, generic data $8A76-$8AFF, panel data $8B00-$8B13. |
| L-005 | verified. Code $E3A3-$E3E1 (59 B Observed). Data $E3E2-$E3E6 (Reset signature). |
| L-006 | verified per family. All 86 bytes are Static code at confidence 0.5-0.7, with no reachability claimed. Families: dead else-skip JMPs, dead JMPs, dead epilogue RTS (supported by 6 reachable rts;rts pairs), bank-5 return stubs, prologues $DE1C/$D9B0, and title dead arms. $8A48 (bank 5) is an ordinary RTS after jsr RestoreSavedBank, not dead code. |
| L-007 | unchanged (verified in redirect). |
| L-008 | not consumed (commentary-only). |

## Remaining unknown ranges
None.

Lowest-confidence classifications (878 bytes of code at confidence < 0.75). Revisit these first if a data consumer
contradicts them:
- The round-1 exact gap fills (0.7).
- The L-006 fragments (0.5-0.7).
- $FB32-$FB3F (0.6, uncalled trampoline).
- bank 3 $97B1-$97BA (0.6, uncalled wait loop).

$8A76-$8AFF is generic data only: no reference to it was found.

## Scenarios (analysis/scenarios.json)
- New: S14-ending-traced (analysis/tools/r2_ending.py).
  - Route: boot, then the S8 route to manual-break aiming, then RAM shortcut $EF=$09, $E6=$00, $DD=$02.
  - Visited: CONGRATULATIONS screen, the Joe Nozerak interview, and the return to title/TOURNAMENT GAMES (new tournament).
  - Evidence: analysis/observed-r2-ending.json and analysis/r2-captures/*.png. Status: artificial entry.
- Visited earlier and not repeated: Freestyle, Party 9-ball and round 5. They have no unknown bytes left, so tracing
  them would only upgrade provenance.
- Deferred: completing Challenge mode (natural World Championship unlock).

## Unresolved questions (for commentary or redirect; none block classification)
- No caller is known for $FB32 (bank-3 $9CF7 trampoline) or bank-3 $97B1 (wait-for-button loop). Their reachability
  is unknown.
- $8A76-$8AFF: possibly an unused tile rectangle. It has no consumer.
- Natural reachability of the intermission code (the trace used a GameCmd $19 patch) and of the opponent-message
  route (the trace used an earlier $D9 patch).
- 28 partially observed Static code regions were left unsplit so commentary anchors stay stable. They could be split
  later to tighten provenance.
- Commentary is needed for the new code: title/credits animation ($832B), portrait/name screen ($899E, $8A51),
  scoreboard ($95F6), string printer $E3A3 and trampolines. L-008 script labels are also pending.

## Toolkit summary
No toolkit issues found in this pass. analysis/toolkit-issues.md is unchanged. TK-001, TK-002 and TK-003 from
round 1 still apply; TK-002 (trace slowdown) made the traced ending run about 0.37 s per frame. The rpcd.py crash was
a workspace-tool bug and is fixed locally.

## Disk
Scratch /tmp/ra-2707-r2d removed after copying 3 PNG captures into analysis/r2-captures. Server daemon stopped by
its own PID. About 2.5 GB free.
