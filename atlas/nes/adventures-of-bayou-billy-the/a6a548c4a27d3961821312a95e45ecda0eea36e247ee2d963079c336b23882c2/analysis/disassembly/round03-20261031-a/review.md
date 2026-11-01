# Independent round 03 structural review

Accepted: R01-L14's narrowed physical [49634,49637), CPU $81D2,
effective PRG slots [6,7,14,15], as Static Code BrawlEnterGameOverState.
No observed fetch, actual caller, natural selection or deadness is claimed.

Fresh configured-binary immutable reads and bank-aware decoding are retained in
independent-primary.json. Current saved regions independently bound the candidate
between C1E1 RTS and C1E5's completion-handler LDY. Source and primary bytes agree:
restart sets split/render/substate fields, completion preserves equipment and
advances state, while the intervening complete terminating transfer implements the
local game-over state transition. E84B is the overlapping LDA #3 entry inside the
BIT operand, then STA $34; JMP E843; LDA #0; STA $35; RTS. The shared fixed-bank
path performs no mapper write and has no selector, index or extra stack-pop
precondition. An ordinary return frame suffices and is preserved by the JMP.
The exact accepted tails at physical 1E32F and **1EAF2** independently demonstrate
this terminal state-setter convention. 1EAE2 is not the matching tail.

The decisive inference combines contextual transition-family placement with a
complete independently meaningful delegated operation and separately established
bounds/return semantics. It does not arise from valid decoding or destination
alone. Accepted unobserved 1E0D7 timer/menu and 102B3 screen/object helpers likewise
have coherent field/stack effects without requiring live-caller proof. Delegation
rather than inline stores does not invalidate that criterion: the candidate's
entire operation is this established fixed-bank state setter.

Strongest alternative: three stored scalar/padding bytes coincidentally encode the
same meaningful terminal transfer among the related state handlers. No bounded
data layout, consumer or padding pattern supports that role. Unlike an isolated
RTS, the candidate selects a specific nontrivial operation independently used for
life exhaustion. The combined constraints discriminate Code sufficiently, while
retaining inference (Static) rather than observation. The ten-word dispatch table
not selecting 81D2 remains a valid negative result for that route only.

Connected slice: E84B/E843 stores and return, restart/completion handlers and both
comparison tails are already classified. No downstream unknown is deferred.
The four separate isolated60 intervals retain Unknown. Reused primary review:
analysis/redirect/round02-20261031-a/residual-review.md. Their preceding routines
are already complete; branches use other returns; no missing state/stack operation
or bounded data/template requires these bytes. Neither generic Data nor Code is
supported by repetition of a single return byte. Artificial entry would merely
execute the hypothesized decode and would not decide these roles.

Gameplay/scenarios and avenue-assessment.md reviewed: three mechanics, later
areas, boss combat, credits/THE END/Start exit already have sound documented routes.
No unchanged replay or arbitrary RAM experiments; none supplies a new residual
selection hypothesis. Broad gameplay evidence is reused with its natural/artificial
limits. This is one completed classification pass, not terminal workflow acceptance.

All 39 commentary files remain unchanged (commentary-preserved.json). Commentary
must carry them forward, add this unit, and repair ContinueRestart's stale
“following bytes are unknown” dependency statement. Existing toolkit NES-001
missing feature only; no new bugs, findings or blockers. No toolkit edits.
