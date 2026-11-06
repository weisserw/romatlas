# Round 3 commentary handoff

Current-source readability is complete for round03-disassembly-reviewed-final. Entire prior commentary was preserved before preparation at round-3/commentary-round02-preserved. Fresh map/state baseline SHA256 is 55a0ad55c4785fc44585933422a031a3e6e284ee2df6c8588dcc13d152cf9ceb.

Classification remains 131073/131088 bytes: 33022 Code, 98051 Data including header; 15 Unknown bytes in six ranges. Commentary changed no classification or map. All 698 current scaffold regions and 476 compact units are reviewed with current source locations, physical bounds, same-kind span checks and explicit Code bank contexts. Full physical byte-to-source location coverage passes. Sound prior whole-source review was carried for the 695 unchanged regions; only three affected spans and their dependencies required fresh editing. No known current classified-source gap remains. See [consistency report](round-3/commentary-consistency-final.json), [location refresh](round-3/commentary-location-refresh.json) and [precise source diff](round-3/commentary-source.diff).

Integrated changes:
- Physical 01C490..01C493 / CPU C480 now names and explains the normal repeat-delay JSR-return jump to the title cursor toggle; the destination is also named.
- Physical 01BE04..01BE0B / CPU BDF4 is explained as generic stage-indexed tile backing. Existing consumer and operand alias remain correct. Seven physical bytes establish no seven- or fourteen-entry schema; higher indices can address following Code.
- Physical 01D4AF..01D4B1 / CPU D49F names and explains the current-cache-index specialization falling into the shared terrain-class/stage-list carry test. Natural caller remains unresolved. All three obsolete Unknown notes are replaced; six residual notes remain honest.

Maintained source retains the round-two routine, branch, RAM, field, constant and data explanations, including the bank-six wrappers, decimal-timer increment/decrement distinction and corrected bank-qualified lookup views. Exact source diff confirms no unrelated prose or instruction rewrite. Constants are unchanged. Remaining semantic limits include actor identities, natural selector domains, original data subdivisions/padding, six structurally unresolved intervals and natural ending/credits progression. These are limitations, not skipped readability work; see commentary/uncertainties.md. No new structural redirect question or toolkit issue arose.

Fresh supported verify passed byte-identically and reading copy was refreshed. Source game.asm SHA256 c6d35aa5714154581d3ef466175990cc602b88f70caf84156373228086de23ac; constants.inc SHA256 3dbd69c2a6e52732e793843266402817a1bacdb1d436203c3da562f659dd46b8; reading-after.asm SHA256 86fac4788333b554dba020605cb663a2b17cbe07fdf61a5e533a856b42e924f8. Input ROM unchanged at f7e9e6981d3446128fe59efb141a483e2c83d11100a246df66701c6d69b2c1e3. See commentary/verification.json.

editorial_audit remains pending for coordinator terminal editorial/archive acceptance. No final audit, archive, cleanup/deletion, publication, toolkit modification or whole-ROM completeness claim was made. All generated source/assets and historical evidence remain local and preserved.
