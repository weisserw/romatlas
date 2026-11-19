# Round 2 — disassembly phase handoff (pass-004)

Status: INCOMPLETE (245 Unknown bytes). One pass; easy gains and all ready leads taken.

## Scope
Input maps/pass-003.json (sha256 0408226e..., 9483 Unknown). Applied redirect leads L-R2-01/02/03/05 after
independent re-verification (reproduced proposal, root references, decode alignment, inline/table contracts,
runtime alignment), L-R2-09, then residual re-scan and L-R2-06/07 with boundary + control-flow checks.
One new natural gameplay route S17 (VS 2PLAYER). Tools/artifacts: scratch/r2/ (batch1.json, batch2.json =
exact region edits; resid.txt; cov-s17-2p.json; p2pregame.snap same-build snapshot).

## Checkpoint and verification
maps/pass-004.json (sha256 c1ceb883b29f4cc8425b459a8c16b0f92bc8d354ba75f481afef252e6ee148db),
generated/pass-004, analysis/pass-004-verification.json: ok=true, byte_identical=true.
analysis/state.json == maps/current.json; map.validate ok. input/game.nes unchanged (a71e4cc8...).

ROM classification: 99.91% (261915 / 262160 bytes classified; 245 bytes unknown).
Per-bank table: analysis/progress.md (pass-004 entry). Code 62,051 B (Observed 10,804), Data 68,792 B,
Image 131,072 B.

## Findings
1. Redirect batch verified and applied: 7,848 bytes (code/inline/tables) + 69 Data->Code corrections
   (b2 $8A78-$8A90, b3 $BC04-$BC2F). Proposal artefact fixed: $8A40-$8A90 overlapped existing code, split.
2. Corrected premise: b3 $B39E-$B3AE is the return continuation of far call `jsr $C18F` (Y=$0E -> $CB30,
   which returns via `jmp ($35)` = return+1). Classified Code; observed 7/7 starts in S17 2P team select.
3. Residual data (118 ranges, 1,469 B) classified with per-range consumer/pattern evidence; b2 $8CE5 is a
   RAM routine template copied to $0742; b10 $9B09 table extent closed by index domain 0..4.
4. S17 2-player game: 9,687 starts, 0 in Unknown/Data, 0 misaligned vs pass-004.

## Unresolved (245 bytes)
- L-R2-08 remainder (240 B, deferred): b3 $AC36-$AC46, $AC73-$AC97, $B496-$B4C4; b7 $B820-$B83A;
  b9 $ACC2-$ACD4; b12 $8DE7-$8DFD; b14 $C126-$C16B. Code-shaped, no reference/far-table/fall-through
  entry, never fetched. Rejection review in analysis/investigation.md (Round 2 section).
- L-R3-01 (4 B, deferred): b10 $9B08 ($60 before table), b12 $9026-$9028 (`jmp $8DB8`, unreachable).
- L-R2-04 (1 B, blocked): b13 $A07A overlapping `bcc` — role established, map cannot represent it (TK-003).

## Deferred
Commentary for the new code (MainStates 30-41 menus, pre-game/result pages, 2P team select) — next phase.
Gameplay: S01-S17 visited; no remaining unvisited milestone expected to touch the residuals.

## Leads
Consumed/verified: L-R2-01, -02, -03, -05, -06, -07, -09. Blocked: L-R2-04. Updated: L-R2-08.
Produced: L-R3-01. Rejected: none.

## Toolkit
TK-001, TK-002 (unchanged), new TK-003 (overlapping instruction streams not representable). See
analysis/toolkit-issues.md. All servers started in this pass were stopped.
