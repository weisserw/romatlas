# Round 3 — redirect handoff (2026-11-18), attempt r3-a

Status: TERMINAL for classification (INCOMPLETE: 1 byte, 0x2604/$E5F4, L-009). No classification
proposal. One ready commentary-only lead (L-013, 0 bytes).
Input checkpoint pass-003 (maps/current.json sha256 a60ef9e5...ae2d0, 24591/24592 classified).
ROM, map, state.json, commented source and the configured binary are unchanged: re-checked
against analysis/redirect/r3-a/pre-hashes.txt (all OK). Research dir: analysis/redirect/r3-a/.
r1-a and r2-a are unchanged and only referenced. leads.before.json = leads.json at the start
of this phase.

## 1. Structural audit of the current source (no conflicts found)
struct_audit.py -> struct_audit.out, ptr_audit.py -> ptr_audit.out (read-only; decodes all
Code regions: 5317 instructions).
- A. Control targets: every JSR/JMP/branch target lands on a decoded Code instruction start
  (0 exceptions). No instruction crosses a region end.
- Inline-argument routines: GetInlineArgs ($E99A) and QueueInlinePpuBytes ($EA17) are the only
  stack-inspecting entries (scan for PLA before PHA at every JSR target found only these plus
  $EB5B TaskSaveResume, the scheduler). There are 11 consumers, all registered: $C8BB, $C963,
  $C985, $C9A7, $E29B/$E29D, $E9C2, $E9DB, $EA05/$EA07, $EA17, $EA63, $EB0A. I checked each
  exit-Y contract against its Data region length: BuildSpriteFramePtrs 3, SetBallPosition/
  Velocity 6, SetObjPosition 5, TaskCreate/TaskCreateForSlotX 3, SetPpuAddrInline 2,
  DrawPpuStringList 2, WritePpuInline 1+count, QueueInlinePpuBytes 1+count (count read from
  the ROM).
- B/C. JSR sites: there are 83 calls to inline consumers. Each is followed by Data of exactly
  the consumed size, and each resumes in Code. None of the 571 plain JSRs is followed by Data.
  No JMP or branch reaches an inline consumer, so there is no tail-call that would move the
  inline data. The four "plain JSR -> Data" hits are the prelude entries $E29B and $EA05, and
  they are consistent.
- E. Code regions that end on a non-terminal instruction before Data: all of them end in JSRs
  to inline consumers, or in always-taken branches ($CD20 after LDY #$4A, $CE18 BMI after BPL,
  $FD0A after LDY #$10). There is one table-conditional branch, $D337 BNE after
  LDA PickoffDirTable-1,X. Every value it can read is non-zero (table 01 08 02, and the
  neighbouring bytes are also non-zero), so the branch is always taken.
- F. Absolute data operands that point into Code: all 5 are table-1 bases, indexed from 1
  ($CD21, $D338, $D5B8, $DA9B, $DFEA).
- G/H. Immediate hi/lo pairs only produce coincidences. The only hit is PPU $20EB in GameSet.
  The LE-word hits inside the sprite-list Data (regions 749-754) are pattern noise with no
  consumer. Neither result is a lead.
- The "CALLED" class of mis-decode (L-008) has no other instance.

## 2. Residual 0x2604 ($E5F4): the round-2 rejection review is reused
No premise has changed. Round 3 changed only region-648 evidence text and commentary naming,
and the section 1 audit found no new control target, inline resume or reference near $E5F3/$E5F4.
The compact rejection review in analysis/handoffs/round-2-redirect.md ("Updated rejection
review — residual 0x2604") still applies unchanged: claim, decisive artifact
r2-a/l009_tests.out, every ROM-encoded reference route ruled out, $FF padding convention,
no structure inside a single byte. A bare unreferenced RTS is insufficient under
evidence-policy.md. L-009 stays blocked (an attempt was appended). Reopen it only if a future
map correction creates a reference.

## 3. Semantic uncertainties (investigated where cheap)
### SQ-R3-1/2: ForceCheck N flag, resolved -> L-013 (ready, commentary only)
- Exact model of $E246-$E25D (forcecheck_table.py/.out, ROM bytes asserted). CMP sets C when
  A >= M, so the routine counts runner entries with progress <= 2Y-1. Empty $FF entries are
  never counted. N is set when the count reaches Y. Over all 20 occupancy/runner
  configurations this equals the baseball force rule exactly. RunnerRole[0] = 0 (no
  batter-runner) gives N clear.
- Natural runtime (sq_runner.py -> sq_runner_fielding2.out; r1-fielding-start, 1800 frames,
  A pulses to pitch, no RAM writes, Instruction trace of ROM 0x2256-0x226E): 719 calls.
  The single N-set call was RunnerRole [01 03 FF FF] with Y=2: the batter is running and the
  runner on first is forced. Every call after the batter reached first returned N clear.
- So **N set = forced**. The source says the reverse (ForceCheck header, "(N clear = forced)"
  at $D921). Caller comments at Runner_onBase, Runner_ballInFlight, RunnerDecision and
  FielderHasBall inherit the inversion. Static callers agree with the corrected contract:
  TryTagRunnerAtBase records a force out (N set, no walk); OutOnTag/FielderHasBall leave forced
  runners alone on a walk; the CPU throw goes to RunnerBase+1 when forced; CheckTagOrForce
  makes the out on BPL.
- Runner progress encoding confirmed from observed positions: 0 = batter at home (1,-8);
  1 = batter running; 2k = on base k ((8,0) first, (0,8) second, (-8,0) third); 2k+1 = leading
  off or running from base k.
- $D981 Runner_awardedBases, exact arithmetic: the LSR carry feeds ADC and SBC runs with carry
  clear, so A = 2*(ceil(p/2)+3-BasesAwarded). The routine branches to Runner_retreat when
  ceil(p/2) < (CurrentTask/2-3) + BasesAwarded, and otherwise to Runner_advance. The current
  comment's formula is off. The intent is still unverified.
All of this is recorded in L-013 (recipe lists the source lines and symbols).

### Deferred, with reasons
- Movement direction of Runner_retreat/Runner_backToBase and the names PointPrevBase/
  PointNextBase. PointPrevBase(p) selects BasePoints $28+4*floor(p/2), which is base
  floor(p/2)+1, the base *ahead*. PointNextBase(p) selects $34+4*floor(p/2), which is roughly
  home/first/second/third. Taken at face value, the names look swapped, and the awarded-bases
  path uses "Runner_retreat" while every runner must advance. I could not reconcile this with
  the observed sequence of +1 steps (lead +1, leave -1, arrive +1). Missing fact: a
  per-frame trace of one runner's ObjRole and target point through a full advance, ideally a
  home run (BasesAwarded=4). Natural home runs are not available in the existing snapshots. An
  artificial BasesAwarded patch needs an understood entry contract. Only the meaning is
  affected (0 bytes), so this does not justify another cycle. Note it as an open uncertainty.
- HitPower/HitAngle swap: this is a naming choice that the static RotateByAngleBits contract
  already covers (commentary documents it). A rename (HitPower -> elevation bits, HitAngle* ->
  speed) is optional editorial work. No new evidence.
- Song-to-event map and the purposes of the unreferenced fragments (L-005 set, SumOfSquaresXY,
  CameraRecC8 reader): these have no reference, so no runtime check can reach them naturally.
  An artificial call proves no intent. They stay as commentary uncertainties.
- L-006 (static code not executed) and L-007 (generic data consumers) remain open. Neither
  bears on classification.

## 4. Ready leads
- L-013 (ready, low, commentary-only, 0 bytes): ForceCheck contract inversion + progress
  encoding + exact $D981 arithmetic. It can go into the final editorial audit or a short
  commentary touch-up. It does not need a disassembly pass (no map change).

## 5. Assessment
No structural lead remains. Another full disassembly/commentary/redirect cycle is not
warranted for classification. L-013 is commentary-only.
Final checkpoint: pass-003, 24591/24592 (99.996%). The only Unknown is 0x2604 ($E5F4, PRG bank
1 CPU $E5F4, context prg_banks [0,1,0,1]), disposition L-009 blocked (rejection review §2).
Verification artifacts: analysis/pass-003-verification.json (byte_identical true);
commentary/verification.json (game.asm d2739ac3..., byte_identical true).
Commentary: commentary/source/game.asm, commentary/uncertainties.md,
analysis/handoffs/round-3-commentary.md.
Scenarios: analysis/scenarios.json entry r3-runner-forcecheck (visited, natural).

## Continuation r3-b: runner direction (2026-11-18)

Question: do Runner_retreat / Runner_backToBase and PointPrevBase / PointNextBase move the runner
the way their names say? **Result: no. The names are inverted. Runtime-confirmed on a natural
route.** Runner_retreat and Runner_backToBase run the runner FORWARD to the next base, and
PointPrevBase selects the base AHEAD. Runner_advance does not advance anything: it moves the
runner to the PointNextBase point just past the base he has reached, then waits there for the
next decision. Commentary-only lead L-014 (0 bytes).
ROM, map, state.json, game.asm (b89dd66c...) and the binary are unchanged
(r3-b/hashes.txt; leads.before.json is a pre-edit copy).

Route (natural, no RAM writes): import traces/snapshots/r1-fielding-start.json; 1800 frames;
`buttons:["a"]` held for 4 frames every 150 frames (the same route as r3-a/sq_runner.py).
This was run twice with different trace ranges (deterministic replay):
- `dir_trace.py runner 1800` produced r3-b/dir_runner.out: Instruction trace of $D885-$DA5F, with
  hits on runner label entries (addresses resolved statically by labels.py into labels.json).
- `dir_trace.py point 1800` produced dir_point.out and dir_point.runners.out (runner slots only):
  Instruction trace of $E481-$E4B7, logging the Point* routine entered plus Y (the BasePoints
  offset) and X (the slot) at PointSet_store.
Both traces also log RunnerRole $0598-$059B and the slot 6/8/A/C position high bytes once per frame.

Observed (BasePoints: $28 (8,0) first, $2C (0,8) second, $30 (-8,0) third, $34 (0,-8) home;
$38 (7,0.5), $3C (-1,8), $40 (-7.5,-0.5) are the points just past first, second and third
toward the next base):
- Slot 8, runner who started on first. f1186: Runner_ballInFlight exits through
  Runner_flightRetreat into Runner_backToBase. That is the forced-or-two-outs exit; r3-a logged
  the only N-set ForceCheck at RR 01 03, so this is probably the forced case. Progress 3 gives PointPrevBase -> $2C
  (second base). The position runs (6,1) -> (0,8). f1423: Runner_backArrived and then
  Runner_arrivedBase set progress 4, and Runner_advance goes to PointNextBase $3C (-1,8),
  just past second. f1433: Runner_retreat sets progress 5 and runs Runner_backToBase
  (PointPrevBase -> $30, third base). The position goes (-1,8) -> (-8,0) and arrives at
  f1670 with progress 6.
- Slot 6, batter-runner. BatterRunner_onFirst falls through to Runner_backToBase at progress 1
  (PointPrevBase -> $28, first base), and the batter arrives with progress 2 (f344, f1440).
  Runner_advance then goes to $38 (7,0). At f1457, Runner_retreat sets progress 3 and
  Runner_backToBase runs PointPrevBase -> $2C. The position goes (7,0) -> (0,8), and the
  batter arrives with progress 4 at f1695.
- Progress never decreased in this window. Every executed Runner_retreat/Runner_backToBase
  moved the runner one base forward. The decreasing path (Runner_goDelay1 -> PointBaseAlt ->
  Runner_leaveBase dec) was not executed here. Its "return to base k" reading stays Static.

Lead L-014 (ready, low, commentary-only; exact renames in leads.json):
Runner_retreat -> Runner_runNextBase; Runner_backToBase -> Runner_runToBase (with
Runner_backWait/backCheck/backArrived -> Runner_runWait/runCheck/runArrived); Runner_toRetreat ->
Runner_toRunNext; Runner_flightRetreat -> Runner_flightRun; Runner_advance(+Step/Wait) ->
Runner_holdPastBase(+Step/Wait); PointPrevBase -> PointBaseAhead; PointNextBase -> PointPastBase.
Rewrite the "round-1 name; direction uncertain" notes (Runner_retreat, Runner_advance, point
helpers, Runner_ballInFlight, Runner_onBase, Runner_awardedBases) as runtime-confirmed.
Replace the uncertainties.md OPEN runner-direction item with the resolved result and keep the
unexercised goDelay1/return path as Static.
