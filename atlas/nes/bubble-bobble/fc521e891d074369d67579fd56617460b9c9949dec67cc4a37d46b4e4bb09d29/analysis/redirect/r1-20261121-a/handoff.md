# Round1 redirect handoff — combined ready batch

Input/output checkpoint remains pass-003. Redirect applied NO classifications.
Current133132/163856 classified=81.249389708036% (81.25%);30724Unknown in266ranges.
Map/state unchanged SHA2568c64f4b84194861555db57fa6dc98e0fc64b0349d3fe44e65cdfe8b7731be084.
ROM unchanged SHA256fc521e891d074369d67579fd56617460b9c9949dec67cc4a37d46b4e4bb09d29.
Current-source readability remains complete; commentary/verification.json ok and
byte_identical true, source SHA256e0bbb595a126222574d2b35d1162db4edeba5f61190a0282744e107e7c067d6a.
Final editorial/archive gates remain pending. No archive/publication/cleanup/toolkit
change performed; all source, traces, RAM, snapshots and captures local.

## Ready proposals for independent disassembly review

proposals.json:17625 unique previously Unknown bytes,11226Code+6399Data;
5207 instruction regions,153 data regions. Zero overlap/current-map conflict after
explicitly excluding the one wrong-bank KIL target. These are proposed, not accepted.
If all accepted,13099Unknown remain; do not report this prospective number as current.

Five ready lead IDs in analysis/leads.json (ranges, views, source links, exact recipe,
expected gain and preserved attempt history):

| Lead | New bytes | Evidence/contract |
| --- | ---: | --- |
| R1-residual-structure |11226Code| graph7.json supporting accepted/JSR/branch/dispatch edges; normal-return-contracts3.json; explicit inline/bank contracts |
| R1-packed-records |794Data|336 packed prefix,248 actor sprite lookup,26 headered PPU containers,184 exact copied text; data-inspection.md,ppu-source-extents.json |
| R1-inline-contracts |204Data|102 unique two-byte words through saved outer-return+2; sound veneers and queue helpers |
| R1-finite-dispatch-data |156Data|36sound/31effect/6palette/5task-init return words, finite selector or independent endpoint proof |
| R1-sound-noncode |5245Data|34 score blocks4836, duration16, envelope pointers32/payload361; consumer/boundary inspection |

Read inline-contracts.md and data-inspection.md before annotation. The graph is an
exploratory evidence index, not an annotation command. Each code node records its
specific supporting root/predecessor, exact CPU/physical address and effective slot
banks. The199 conservative normal-return proofs reject unknown indirect/mapper/stack
writes and nonlocal yields; only documented sound/queue inline calls skip2. CEDF
continuations have immediate LDY#2/#3/#4 preconditions, not arbitrary-Y assumptions.
Code/data is the needed claim; live caller/natural selector is not universally required.

## Corrected rejection and connected closure

Raw graph7 retains physical98662 KIL, reached by fixed$D68F JMP$8156 underbank12;
it overlaps an accepted/candidate instruction operand, so excluded from proposals.
This falsifies that alias only. The same fixed transfer is gated by$0462, set in
accepted bank10 boss initializer$8221; confirmed boss context is[10,11,14,15]. Under
that supported boss contract, bank10 third JMP veneer$8156->$905A initializes two
zone rows, saves/restores X/Y and returns. Its$9073 queue helper and all supported
continuations closed, adding34Code+2inlineData. No arbitrary roots for the other
unreferenced veneer members. Exact details and competing alias in inline-contracts.md.

All36 sound handlers and31 global-effect entries were assessed, plus shared helpers,
state-branch fallthroughs, palette dispatch and all existing accepted Code successors.
No artificial two-actor cutoff. Generic bank8 data was inspected block-by-block after
those contracts; no semantics/complete-stream prerequisite left clear noncode unknown.
Independent PPU source bounds use consumer dimension/count fields, not pointer plausibility.

## Gameplay/behavior results

analysis/scenarios.json records routes/patches/captures. R1-ending and
R1-middle-continue now verified, separate from structural annotations:

* middle/stage30.png: initialized Round30, distinct geometry/enemies, movement,
  bubbles and jump. middle80/part3.png: initialized Round80, distinct pink geometry,
  enemies/player bubbles after representative movement/jump inputs. Durable endpoints
  retained; corrected double-confirm Continue route avoids stale stage-only edits.
* From confirmedB4.snapshot, RAM$006D=3 selects source-supported defeat initialization.
  State3->4 falling animation sets$72=1 and enters bank10$94CB. No HP/PC guess.
* ending-b/frame2.png: BAD END requires MAGICAL CRYSTAL BALL, PASSWORD EECJJ.
* From saved defeat snapshot,$049A=1 alone: ending-c/crystal-single.png BAD END,
  TRY AGAIN WITH YOUR FRIEND, PASSWORD EE.
* Also$0042=3 enables the second-player gate: paired rescue animation followed by
  ending-d/frame1.png HAPPY END! with hearts and rescued paired characters. Saved
  ending-d/step1.snapshot is a reusable terminal endpoint. No more ending retry needed.

All these progression routes are artificial; RAM edits alone are not visits.
Actual fetched roots from ending-c and Continue80 establish101 instruction/203byte
Observed subset of the structural proposal, in observed-upgrades.json, with actual
bank contexts and scenario artifact links. Natural pickup/damage/progression not claimed.

Long runtime scripts sometimes ended143 after useful saved endpoints; coordinator
confirmed no manual termination. Trace120-frame call timed out once, and framebuffer
properly refused existing capture filename once. Records/scripts/RPC outcomes retained.
Changing to <=250-frame durable per-process parts completed stage80. These are local
execution interruptions, not a demonstrated toolkit defect. All runtime sessions now
closed; no pending experiment or server required for this handoff.

## Remaining unknowns and actual limits

unknown-ledger.json is exact current266-gap/30724-byte complement, canonical CPU
aliases (odd PRG banks use$A000), effective context descriptions, neighboring accepted
IDs, exact proposed coverage and localized unclaimed spans/dispositions. It has not
subtracted proposed bytes from current Unknown.

The combined batch closes the identified finite dispatch/inline/data-format avenues
as far as evidence presently supports. Further repetitions of the same pure-return
checker mostly stop at the same substantive barriers, not a fixed pass quota:

* FA81 banked sound wrapper /F76D/F777 transient bank restore needs the remembered
  $04A3-to-MMC1-mode/selected-bank invariant to propagate a caller's lower-bank view
  after return; graph does not assume it from one observed slot tuple. A mapper write
  or fixed caller alone is not a bank-preservation proof. Saved contexts support
  local views, not a global shadow-register invariant across all task states.
* CEE3 yields through alternate saved stacks; scheduler restores registers/RTS without
  restoring a task-specific PRG context. Immediate ordinary return propagation would
  be wrong. Existing actor/sound tasks change banks; precise per-task state/mapping
  preconditions remain missing for unobserved banked continuations.
* Some routines use unresolved indirect writes or intentionally pop return frames.
  Their connected instruction bytes are proposed where supported, but the checker
  cannot infer arbitrary pointer domains or shared caller stack contracts. Exact
  callee/call-site barriers are preserved in graph7.json; no dead/unused claims.
* Natural boss damage/contact and natural crystal pickup/secret-road alternate-mode
  route remain unresolved. Ending gates and representative terminal scenes are now
  verified; forcing every hidden-route permutation has diminished classification
  prospects (new actual roots all lay inside the combined static batch). Further
  arbitrary state sweeping or repeating terminal screens is unwarranted.
* Isolated bank8$8035 six-byte apparent disable routine has no supported entry;
  retained Unknown. Other heterogeneous gaps have no safely bounded noncode claim
  after current consumer/CFG inspection. No blanket executable-bank Data fallback.

These are limitations on additional claims, not terminal exhaustion or a demand for
recursive closure of every state. Ready combined findings warrant disassembly now,
then full current-source commentary and another redirect; accepted new source may
resolve remaining contracts. No empty disassembly/commentary cycle is proposed.

Toolkit: no new toolkit issues found this redirect. Existing NES-001 confirmed
input-schema documentation/API-validation bug remains, workaround explicit buttons
arrays; zero blockers. See analysis/toolkit-issues.md. Disk final~3.3GiB free;
analysis/scratch under1MiB. Attempt evidence retained; no cleanup/deletion performed.
