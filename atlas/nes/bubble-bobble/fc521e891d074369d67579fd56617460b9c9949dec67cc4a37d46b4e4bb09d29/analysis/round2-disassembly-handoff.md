# Round2 disassembly handoff

ROM classification: 92.01% (150757 / 163856 bytes classified; 13099 bytes unknown).
Single pass completed; ROM incomplete. Input pass-003 -> verified pass-004.
Accepted all17625 proposed bytes:11226Code+6399Data. TotalsCode20196/Data97793/
Image32768. Exact per-bank counts: analysis/round2/stats.json and progress.md;
exact155-range residual: analysis/round2/unknown.json. State/current saved/exported.
Checkpoint verification ok=true,byte_identical=true: analysis/pass-004-verification.json.
Portable map maps/pass-004.json SHA256563b559e3c5d2a1a0d676f2f7734997365a4ddd6bd236f11454e305d2a5f2f41.

Primary independent review in analysis/round2/review.md; all5340 proposed views freshly
decoded, exact physical nonoverlap checked, stack/inline/finite dispatch/data consumers
reviewed. Excluded wrong-bank physical98662; accepted bank10 boss-contract8156 veneer
and connected9073 helper.101instruction/203byte actual fetched subset kept Observed,
artificial ending/Continue reachability explicit; two actual fixed-bank views added.
All9643 regions have evidence, executionCodeObserved7676/Static12520 bytes separately.

Five ready R1 structural leads consumed/verified; leads.json preserves history and
adds research R2-bank-shadow-contract,R2-task-stack-bank-contract,R2-residual-consumers.
Frontier in analysis/round2/frontier.json: FA81 bank-shadow invariant, CEE3 alternate
stack/task bank context, pointer-domain barriers and residual bank10 consumers. No
terminal exhaustion; no further supported connected easy gain currently established.
Do not replay completed Round30/80/boss/BAD END/HAPPY END; saved routes remain valid.

Next full current-source commentary must use generated/pass-004 while preserving prior
commentary artifacts. Existing commentary/source/game.asm unchanged SHA256
 e0bbb595a126222574d2b35d1162db4edeba5f61190a0282744e107e7c067d6a;
its previous verification belongs to pass-003, not pass-004. Current readability gate
pending. No other phase executed, no live server, no goal completion claim.
Toolkit:existing NES-001 confirmed bug, no new issues/blockers; analysis/toolkit-issues.md.
No toolkit changes, publication/archive or cleanup/deletion; all internal assets local.
