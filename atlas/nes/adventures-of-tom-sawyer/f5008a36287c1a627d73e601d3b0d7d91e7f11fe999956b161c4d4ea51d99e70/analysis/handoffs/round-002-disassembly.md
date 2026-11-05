# Round 002 disassembly — accepted combined batch

ROM classification: 95.95% (251552 / 262160 bytes classified; 10608 bytes unknown).

Incomplete. Fresh checkpoint `pass-002-final` validates and rebuilds byte-identically: [verification](../pass-002-final-verification.json). Map SHA256 `2789b29f7594c8358c26ea716742b8445dfaaf1c4e971e3c1012f5f1bdd4403f`. ROM hash unchanged. Internal state/current portable map saved. Code40793 +Data79687 +Image131072; Observed code8887, Static31906. Exact [per-bank counts](../round-002/counts.json), [65 residual intervals](../round-002/unknown-ranges.json).

Accepted **28156 Static Code +17304 Static Data** new bytes. Redirect's original28125 Code proposal exactly reproduced, all48 data intervals independently reviewed. Configured decoder independently checks12269 unique new instructions,140 grouped spans and every proposed context. [Audit](../round-002/review.json), [annotator](../round-002/accept.py), [primary review](../round-002/contract-review.md), [validation](../round-002/validation.json). All837 regions carry evidence. Code boundary/alignment alone was not the role proof; accepted roots, bounded handler contracts, balanced stack, consumer semantics and independently encoded data origins supply it. New code is Static even if some saved artificial fetch contexts could support Observed; no new execution claim needed.

Six scene header origins/type/resource/state partitions checked directly against ROM; mapper42/palette50 pointer entries and count-prefixed record union cover exact bounded corpora. Pointer aliasing means table order differs from serialization. Generic movement/animation/sound/envelope corpora have repeated command/parameter patterns, consumers and code boundaries without claiming all individual schemas. Seven-entry sound table and E59D event boundary corrections preserved. Bank14 final6 vector-like bytes remain unknown.

**Connected continuation beyond proposals:** E349 descending copy now has a narrow supported RAM-destination contract at four known callsites (0400/0440/04C0/04CC bases, Y3F/20/0B). Alternate ABB0 entry sets04CC before joining common copy path. Bases unchanged and destinations0400..04D7 cannot overlap stack/mapper/pointers. Adds31 Code bytes/14 unique starts; [new instruction/context records](../round-002/copy-continuations.json). Reproduction scripts read frozen round-002/input-state.json, not future current state. This refines arbitrary-pointer rejection without declaring all indirect stores safe.

## Next commentary and redirect

Existing fully reviewed `commentary/source`, reading copy/review/history preserved. New baseline is `generated/pass-002-final`, which needs full current-source commentary, including new classified regions and existing material. No archive execution, deletion, publication or toolkit modification was performed. This is not editorial completion or terminal exhaustion.

All five ready inherited leads marked verified with appended history. New R002-CONTRACT-FRONTIER captures localized unfinished research. [Extended frontier](../round-002/extended-frontier.json): dynamic EA15/EA2A PRG outputs require local post-bank contexts; nonordinary E4F3/E75E/RTS trampolines require stack/dispatch summaries; auxiliary indirect selectors need bounded extents; sound indexed pointers need destination/source domains. These are actual missing contracts, not code disproof or global rejection. Derive local summaries in redirect rather than repeating broad gameplay. Easy static copy gain was followed to its fixed point (855 summaries); further primitive closure needs these new facts.

Prior six-stage, ending/results/title-return and initialized/exercised boss experiments remain reusable; no unchanged replay. Boss-room redraw fidelity after camera jump remains unresolved and separate from classification. No terminal rejection review asserted: this is a productive accepted batch and next research handoff.

Toolkit: **no toolkit issues found**, no blocker ([persistent report](../toolkit-issues.md)). ~9GiB free, no storage pressure; all cited evidence retained local.
