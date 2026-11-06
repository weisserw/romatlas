# Round 4 disassembly handoff

One productive pass complete; whole-ROM incomplete.

ROM classification: 99.99% (131076 / 131088 bytes classified; 12 bytes unknown).

33025Code+98051Data. Accepted R03-REJECTED-SELECTOR-THUNK: physical01944E..019451, CPU943E, effective[12,13,14,15], Code Static. [Independent review](round04/independent-review.md) combines encoded boundary word81B4=943E, guard's independent >=B2 flags-zero operation, fixedD17E/D399 exact operation/normal-return contract and closed neighboring handlers. [Fresh decode](round04/fresh-primary-decode.json) and [immutable checks](round04/fresh-boundary-checks.json). No179-word table or actual/natural B2 selection claim; guard excludesB2. All supported downstream continuations already classified.

Saved state/export validate. Fresh **round04-disassembly-reviewed-final** [verification](round04-disassembly-reviewed-final-verification.json) confirms whole-file byte identity. Map SHA25660778e26c4705c5eeb146960415dfa7538253107508bfbcd1c811bc36d60e59d; frozen checkpoint equals current export. [Classification](round04/classification.json) records all per-bank counts and residuals.

Five residual half-open intervals total12:00F9C3..00F9C7(4EA),018232..018235(3EA),01967F..019680(RTS),01E609..01E60A(RTS),01F2D1..01F2D4(JSRE108). Reuse precise primary review in redirect/r03-rejected-selector-contract/rejection-review.md. Broad gameplay stage0/2/4/6/13,boss/postboss and final yield-loop evidence reused; natural credits unverified. No changed useful condition warranted replay/arbitrary fragment execution. This acceptance pass is not terminal exhaustion.

Next commentary must preserve valid round3 full current-source review698regions/476units and existing prose, integrating only3new bytes and precise dependencies. commentary/source and reading copy remained untouched. Input hashes in round04/input-integrity.json match round03-commentary-handoff.md: game.asmc6d35aa5714154581d3ef466175990cc602b88f70caf84156373228086de23ac, constants3dbd69c2a6e52732e793843266402817a1bacdb1d436203c3da562f659dd46b8. Replace obsolete943E Unknown note with structural thunk explanation while retaining actual guard exclusion/table boundary caveat. No known need to rewrite unchanged work. Current source integration and coordinator editorial/archive acceptance remain pending.

[Toolkit issues](toolkit-issues.md): no toolkit issues found. No source/toolkit edits, archive, cleanup/deletion or publication. ROM and local evidence preserved. Disk8.3GiB free; evidence compact.
