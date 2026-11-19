# Round 2 — redirect handoff (2026-11-18), attempt r2-a

Status: INCOMPLETE (classification) — no classification proposal; residual 1 byte ($E5F4,
0x2604) stays Unknown. Two low-priority ready leads (L-011 map evidence text, L-012
commentary naming), both 0 bytes.
Input checkpoint pass-002 (maps/current.json sha256 97281128...ef9, 24591/24592 classified).
ROM, map, state.json, commented source and binary unchanged (pre-hashes re-checked OK:
analysis/redirect/r2-a/pre-hashes.txt). Research dir: analysis/redirect/r2-a/
(r1-a is unchanged and referenced). leads.before.json = leads.json at the start of this phase.

## L-009 / SQ-R2-1 — $E5F4 (0x2604), revised reference tests
Premise check: r1-a/wordsearch.py already tested contiguous LE words for target and target-1
($E5F4 and $E5F3). The coordinator asked for a wider check, so this pass widened the scope
(script l009_tests.py, output l009_tests.out; decoded 5317 instructions across all Code regions):
- (a) RTS dispatch. Scheduler entries are stored as entry-1: TaskCreate ($EB0A) reads inline
  [slot][entry-1 word], and TaskSaveResume stores the JSR return address (site+2). EB94
  dispatches with PHA/PHA/RTS. Tests run: LE and BE words $E5F3/$E5F4 anywhere in PRG
  (CHR included for completeness), immediate #$E5 near #$F3/#$F4, and every stack-built
  return (RTS at EA3D, EB9C; E9B4 ResumeWithInlineArgs). Only one hit: word $E5F4 at $EC36,
  which is already known and sits inside sprite-list data. No JSR can return to $E5F4,
  because $E5F3 is itself an RTS and so no JSR ends there.
- (b) Split hi/lo tables. I checked every abs,X/Y table base pair (bases at most 64 bytes
  apart, index <64). Only one candidate came up: $D3F3[56]=$E5 paired with $D3F0[56]=$F4.
  It is incidental: BatterStepX/Y have an index domain of 0..4, and index 56 falls on code operands
  ($D428 = the BPL offset, $D42B = the high byte of `JSR Random` at $D429). The other 8 $E5 bytes in Data regions have no
  $F3/$F4 partner.
- (c) Inline arguments: no inline-argument Data region ends at $E5F3. JSR GetInlineArgs sites
  (C8BB, C963, C985, C9A7, E29D, E9DB, EA07, EA63, EB0A, E9C2) resume at site+3, never $E5F4.
- (d) Computed control: no JSR/JMP/branch targets $E5F4. The only indirect jump is
  JMP ($0000) InlinePtr, which resumes after inline data, and the condition for it to land
  on $E5F4 is ruled out by (c). The only stack-built returns are QueueInlinePpuBytes
  (resumes past inline data) and the scheduler (resumes covered by (a)).
- (e) Convention: the byte pair `60 60` appears only at $E5F3. Every other 1-byte Code
  region ($C913, $C958, $CB34, $D360, $DBAF, $C962) is an RTS that has an entry (a branch,
  a fall-through or an inline resume). This ROM pads with uniform $FF (Padding regions 757,
  758, 760, 763), so nothing here sets a precedent for a $60 filler.
- r1 traces: 0x2604 has never been fetched (traces/r1-coverage-offsets.json).
Outcome: negative; L-009 is kept Unknown (status blocked, attempt appended).

### Updated rejection review — residual 0x2604 ($E5F4, 1 byte), supersedes r1-a for scope
- Claim: Code (an unreferenced RTS / empty routine) or generic Data (filler).
- Decisive evidence: l009_tests.out. No direct, entry-1, BE, split-table, inline,
  stack-built or indirect reference exists. The neighbours are the RTS at $E5F3
  (DrawTitleCursor) and Random at $E5F5 (observed). The ROM's padding convention is $FF.
- What it rules out: every ROM-encoded control or data reference route that this ROM's
  dispatch mechanisms (JSR/JMP/branch, TaskCreate entry-1, TaskSaveResume, GetInlineArgs,
  QueueInlinePpuBytes, JMP (InlinePtr)) could use. The $FF convention also weakens the
  filler reading. Neither result establishes the opposite role.
- Independent constraints considered together: the single byte has no internal structure
  (so no tiling or contract evidence is possible); it has no consumer; and it sits between
  two complete routines. The ROM has 13 other unreferenced remnants (L-005), but each of
  those is a multi-byte routine with a supported register/RAM contract. A bare RTS supplies
  no such contract.
- Strongest competing interpretation: a leftover empty subroutine (code, unreachable) versus
  a stray byte from assembly/linking. These are not distinguishable from the available evidence.
- Result: inference insufficient (evidence-policy: "an unreferenced return opcode … lacks
  sufficient structure on its own"). There is no revised test left: every encoding this ROM
  can use to reach code has now been checked. Reopen only if a map correction elsewhere
  creates a new reference.

## SQ-R2-2 — $DF92-$DFC3 (region 648, 0x1FA2-0x1FD3)
- Structure is sound as Code. $DF92-$DFA6 computes X^2+Y^2 into A:Y (two JSR $DFA7, then
  adds), and SquareByte $DFA7-$DFC3 computes |A|^2 by shift-and-add on MulA/MulBLo and
  ends with an RTS directly before the consumer-anchored Data at $DFC4. The official
  opcodes tile exactly, and the internal JSR targets fall on instruction boundaries.
  No competing data consumer reads these bytes.
- Reachability: there is no effective entry. BEQ $DF90 is always taken: Z is set by LDA #0
  (Intercept_ahead) or LDX #0 (Intercept_store), with only STA/STX in between. No outside
  reference exists (sq2_refs.out; the word $DFA5 at $C090 is the operand of `LDA $DF`, so
  incidental). The bytes were never executed in the r1 coverage, while $DF90 was.
- Scan of all immediate-load always-taken branches (flagscan.py/.out, ftrefs.out): apart
  from the documented L-004 stubs, only $DF90 leaves a Code region with no other entry.
  No other region has a misleading evidence string of this kind.
- Correction: region 648's evidence wrongly says "reached by bounded traversal", because the
  r1 traversal did not carry the Z flag across STA/STX and a two-path merge. **L-011 (ready,
  low, 0 bytes)**: a metadata-only map.annotate of region 648 (text is in leads.json). The
  commentary already reads "no other known entry", so no commentary change is needed.

## SQ-R2-3 — camera naming (settled; commentary only)
I ran a bounded runtime check from natural snapshots only, with no RAM writes
(sq3_camera.py, sq3_shots.py):
- Pitching (r1-fielding-start, 900 frames; r1-2p-start, 600 frames): throughout, $CB=$06,
  PPUCTRL nametable bit set ($2800), ScreenSel=$08, DrawFirst=$15. Screenshot
  pitch_f200.png shows the diamond close-up.
- r1-play1-end: once the ball Y hi is $80 (CameraParam $76 >= 0), the view switches to $CB=0,
  nametable $2000, DrawFirst 0. cb0_f120.png shows the full field with the outfield fence
  and scoreboard. The view returns to $CB=6 at frame 218.
- Conclusion: the second CameraTables record (carry set) is the INFIELD view and the first
  (carry clear) is the full-field/OUTFIELD view. INFIELD_DRAW is correct. The SetCamera
  comment and the round-1 "$00 infield / $06 outfield" fact are inverted.
  **L-012 (ready, low, commentary only)** lists the symbols to fix: SetCamera,
  CameraTables, CameraFrameOfs, FrameTask_camera, the $D810 tables and
  SetFirstCameraHideSprY. It also closes the uncertainties.md camera item.

## Commentary corrections (for the next commentary pass)
- L-012 camera naming, as above.
- Optional: the r1 redirect handoff's "$CB $00 infield / $06 outfield" should be read as
  inverted. Historical records are left unchanged.

## Avenues investigated / deferred
- Gameplay/scenarios: reviewed scenarios.json. GAME SET has been visited (artificial
  shortcut). Extra innings are unavailable (C6CC forces game end at HalfInning $12).
  Substitution menus / other teams (L-006) are deferred: the only residual is 0x2604, and no
  runtime state can create a ROM-encoded reference to it. Added a scenarios entry for
  r2-camera-views.
- L-007 (open, from round 1): not touched; it has no bearing on the residual.
- Other commentary semantic questions (HitPower vs HitAngle, ForceCheck N flag, $D981):
  these are meaning-only and do not affect classification. They were not pursued this pass
  because no structural lead depends on them.

## Ready leads / next step
- L-011 (map evidence text, 0 bytes) and L-012 (commentary naming, 0 bytes). Neither changes
  classification, and neither on its own warrants a full disassembly/commentary/redirect
  cycle. Fold them into the final editorial audit, or into a short metadata-and-commentary
  touch-up if the coordinator prefers.
- Classification remains 24591/24592. The residual 0x2604 is honestly unresolvable with the
  available evidence (see the rejection review).
