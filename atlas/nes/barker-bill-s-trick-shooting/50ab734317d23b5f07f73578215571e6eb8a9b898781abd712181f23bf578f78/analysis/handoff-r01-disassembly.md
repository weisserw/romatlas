# Round1 disassembly handoff

Incomplete. ROM classification: 73.43% (144389 / 196624 bytes classified; 52235 bytes unknown).

Checkpoint pass-001-final is validated and byte-identical; maps/pass-001-final.json SHA256 db252197fe7e5c3fb4e479eaee7888427f006a35a798a9899b20f09f1a665cda. Current state analysis/state.json; export maps/current.json. ROM SHA256 unchanged. Baseline and intermediate pass-001 also preserved.

Scope: validated header/CHR/vectors,8770 observed code bytes,3706 static code bytes, four exact uniformFF runs. 12476 code; 131913 data/image; 52235 unknown. Per-bank counts analysis/evidence/bank-counts-r01.json; exact gaps analysis/evidence/unknown-r01.json; progress/investigation hold full totals.

4065 distinct physical ROM opcode fetch addresses observed, with complete effective8KiB bank context. Natural startup displays separated from artificial gallery visits. All base galleries, Bill/Trixie introductions/animation, slots, menu, title-return and actualR99 balloons/R9 later-gallery displays reached. No Zapper hits or finite ending established. Recipes, failures and milestones analysis/scenarios.json; selected frames/snapshots local under analysis/evidence. Earlier0625 round interpretation was falsified; correct field06F2 is independently used by HUD and capped99.

Ready leads: R01-L001 C52C stack-pop inline dispatcher/table boundaries; R01-L004 remaining PRG consumer/format structures and bank-switch continuations. L002/L003 verified initializer/round contracts. Inspect existing static candidates/call contracts rather than repeating discovery: analysis/evidence/static-new-reviewed.json, call-new-reviewed.json, call-entry-contracts-r01.json. Exact main table extent has not yet been annotated; do not infer code from pointer values. Connected branch and direct-call static batches were classified, not just original lead bounds.

Most PRG unknowns have not received dedicated structure research; no terminal exhaustion claimed. Next commentary must cover all current classified source, then redirect should investigate coherent dispatcher/data batches and late-gallery transition contracts. Bills Thrills target throwing still pending; simply replaying long startup is low value. Normal gun-dependent scoring unavailable.

Toolkit findings: one documented missing feature NES-001, Zapper input; no confirmed/suspected bugs. See analysis/toolkit-issues.md. No toolkit changes made. All evidence remains local; only supported ROM-free maps are portable deliverables.
