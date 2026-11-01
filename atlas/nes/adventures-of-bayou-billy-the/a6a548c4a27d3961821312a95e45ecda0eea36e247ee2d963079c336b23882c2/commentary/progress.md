# Commentary progress — round 03

Full current-source readability is complete for pass-003-disassembly-20261031-b.
Covered 39,605 Code bytes (18,219 instructions), 91,479 Data bytes and 131,072 CHR
bytes, plus honest boundary notes for four Unknown bytes in four intervals.
All 8,271 scaffold records are reviewed; 678 code units and 409 named data/image
structures retain useful explanations. No readability ranges remain deferred.
Final editorial/archive audit remains pending for the coordinator after redirect.

Preserved all 39 previous commentary files in analysis/round-03/commentary-round02/
before fresh preparation. Retained independently maintained source and symbols,
useful prose and 8,270 valid physical-range scaffold reviews. Current baseline,
state, source mapping and phase1 scaffold metadata were freshly generated; old
mapping metadata was not copied over them. All 16,865 carried source references
still identify the exact same lines after the edits. Current full inventory was
checked for reviewed status, summaries, locations, partition coverage and bounds.

Added BrawlEnterGameOverState at physical $C1E2..$C1E5, CPU $81D2 with effective
PRG slots [6,7,14,15]. It delegates game state 3/substate 0 to the existing fixed-bank
CPU $E84B overlapping LDA #3 entry and preserves an ordinary caller return frame.
The independent structural review establishes Static Code, without observed/natural
entry claims. Bounded preceding BrawlRestartStage review ownership at $C1E2 so the
new entry has its own explanation. Preserved sprite/footer ownership and physical
bank-tail names; no broad naming transformation was performed.

Repaired ContinueRestart's stale unknown-following-bytes claim: the shared counter
reset now explicitly falls through to ContinueDecrementLives' stage-transition
entry. Its separate life-exhaustion path retains the documented extra-frame rule.
Updated all affected source links, summaries and unknown notes. Exact comparison
tails remain physical $1E32F and $1EAF2; $1EAE2 is not that tail.

Semantic uncertainties remain explained in uncertainties.md and near source:
audio identities/schemas, some object flags, projection generator, font meanings
and fine generic constant grouping. All prior sound consumer/format inspections
remain valid. Four isolated $60 bytes lack a discriminating entry/consumer/template;
no new structural question was invented. No current-source readability work remains.

Fresh verification.json passes byte-identical rebuilding against unchanged input;
reading-after.asm was freshly expanded after the final source edits. Source hashes:
- game.asm: 1dbaafc64e68e62c8bb4f226dda909cd051f00fa4ce34cc28a1be2fa7f945709
- symbols.inc: 582db20960dcf3c79b8195328b855f5652a88675cd4765be80de2b8682ce300f

Map/state SHA256: 8fac0ec90e59abe29b84a73cee9875b8e89ff5d7c5841e626579807c6a56138c.
Whole-ROM classification remains INCOMPLETE: 262156/262160 bytes, 100.00% rounded.
No toolkit edit/new issue, map/state mutation, archive execution, publication,
cleanup/deletion, ROM modification or external asset download occurred.

## Final editorial acceptance — 2026-10-31

Reused full current-source review credit; corrected all 24 redirect stale dependency claims, three equivalent nearby headers and the countdown PPU-script consumer/name mismatch. Updated affected review summaries without shifting source links. No readability ranges deferred or known naming/commenting gap remains. Semantic limitations and four structural residual bytes remain explicit; whole-ROM classification INCOMPLETE. Final audit and verification evidence: analysis/final-editorial/20261031-a/; editorial assessment: editorial-audit.md. No map/state change.

Final audit exited 0: ok=true, byte_identical=true, editorial_audit=true; 8271 regions and 8596 unit records accepted. Current game.asm SHA256 22ea01c06e0e62e6b5bfacfb258dcdb79d8cadec9435546f8560fc36060bd967. Reading copy refreshed; configured release store/map validations both passed.
