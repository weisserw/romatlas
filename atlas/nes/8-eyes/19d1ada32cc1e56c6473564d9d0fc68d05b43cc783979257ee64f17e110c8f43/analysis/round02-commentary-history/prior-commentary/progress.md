# Commentary progress

Round01 full current-source readability batch applied and verified byte-identical.
All 636 scaffold intervals have present-source review notes: 478 Code fragments,
29 Data regions, one CHR Image region and 128 explicitly unresolved intervals.
193 purpose/mechanics units cover all 5,523 currently classified instructions;
852 code-entry/branch symbols and shared RAM, object-field and hardware constants
are present. No code/data boundaries or phase-1 maps changed.

Coherent audio/binary-only batch verification passed first; full classified-code
batch verification also passed (source hash 048d022249b3ffc163f382225c756c737a24bf1d3b52643b39a879e4c6917f54).
Current frontier: final consistency fixes to BIT operands, known shared buffer names,
comment wrapping and source-location refresh, then handoff verification/reading copy.
No classified region remains unread. Final editorial audit is deliberately pending.

Full review resolves the F85A wait contract: it is the STA handshake interior entry
of F858. A=$3C waits for 60 interrupt decrements, then increments frame phase once.
The continuation/ending and valid split-screen interrupt scene remain unresolved.
Other unresolved meanings stay in nearby source prose; no original names, complete
boss coverage, full hardware compatibility or terminal game progression is claimed.

Final consistency fixes completed: known BIT/shared-buffer operands named, long
purpose prose wrapped, all scaffold/unit source locations refreshed. Fresh
verification passed with source SHA256
`d0e1fb0cdc7d722a499e4863e8b347c616d5eeb35b13e7c2836ced65d3dcf102`;
reading-after.asm refreshed. Scope reconciliation confirms zero pending readability
regions and unchanged baseline state/map. New structural checkpoint
round01-commentary also passed validation and byte-identical generation; edited
commentary source remains the independent readable deliverable. No readability
frontier remains. Redirect questions live in uncertainties.md; editorial_audit stays
pending. No new toolkit issues, no new execution evidence, input/toolkit unchanged.
