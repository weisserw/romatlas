# Redirect round 001 — active static contract investigation

Input: pass-001/current map + analysis/state.json; commentary reading-source SHA256 656fb64587d11fae78f4fd79de3d0c1dd1ceac60349483c80372704bc7960172. Classification unchanged 58908/65552, unknown6644. Fixed CNROM execution context [0,1,2,3]. No map, ROM, toolkit or source changes.

Read-only research.py / rpc.jsonl gather authoritative analysis.decode and discover_context; source primary cross-check analysis/prg-review.txt and commentary/reading-after.asm. Discovery is a candidate pool, not automatic code acceptance. Explicit tables: A9E6 four exact movement choices from (record42 AND0C)>>1; A903 nineteen condition entries bounded by following carry-return handler; E2A7 primary behavior entries, B0E7 matching initializer entries; B80E secondary initializer entries; A64D nineteen continuation addresses selected by record47; BF58 six event callbacks followed by first callback BF64. Table word values in tables.json. Initial speculative BAB6 word-root hypothesis was rejected immediately after raw-byte alignment inspection: actual BAB7 starts data-record address table, not code; corrected gather removes those roots. rpc.jsonl preserves both runs. No inference accepted from wrong alignment.

Dispatch closure alone currently supplies 4557 candidate unknown code bytes (dispatch-only-candidates.json). These need independent structural review, especially indirect-table bounds, inline barriers, unofficial opcode at B986 and tail stack contracts. Connected continuation A781 was identified as nineteenth A64D target after inspecting unknown tail; now included. Primary initializer B0E7 is directly consumed at 9FE9/9FEE then JSR A086; primary update E2A7 at A047/A04C. Continuing all initializer/handler paths in same batch.

Independent small routines now being gathered: 8A93,8D7B,92B2,92C6,92CF,9E38,9F32,A35E,AD1A,B05B/B05F,B43D/B443,B4DD,B57F,B923,BA92,C27E,C326,C500,C516,C58C,C773,C7EF,C896. Contracts include frame/button waits, OAM loops, collision bitfield transformations, PPU stream parsers, animation fields. Direct caller not universally required; exact coherent contracts and boundaries must be reviewed.

Independent data lead: C896 loads byte X from072F, increments it, reads C8A0,X, returns; whole 256-byte C8A0..C99F lookup extent follows full byte index and next established audio entry C9A0. Other clear residual tables (A903,A9E6,A64D,B0E7,B80E,BF58,B332,B37D,BAA4..BC0E) need consumer/pattern/boundary evidence recorded. C47C..C4A1 uniform repeated A2 C4 pairs can be generic data; lone C4A2 RTS remains distinct ambiguous byte without a code contract.

Commentary questions: inventory893F packs bitplanes03DA/03D9/03D8 then masks7; 8C36 contains6 bytes followed by existing code8C3C A9 10. Do NOT extend table into code just because index can6/7. Continue writers A785..A7E7 to resolve whether values6/7 natural. 9CA7 bytes25 FC really AND zpFC, unlike immediate29FC at9C94. No direct FC writer found in classified PRG listing; cannot silently repair to immediate. Investigate reset/indirect stores and natural menu state only if useful, otherwise document behavior uncertainty without structural consequence.

Next exact actions: finish current research.py process, subtract candidate intervals from exact accounting complement, inspect coherent contiguous decode against all branch/data boundaries; gather consumer references and bounded raw data formats for residuals. Accumulate one proposal and leads after all actionable independent avenues reviewed. Not exhausted, not a disassembly/commentary handoff yet. Existing NES-001 only; no new toolkit issue.

## Updated checkpoint — all identified actionable avenues reviewed
Combined proposal now5463 Code +1179 Data=6642 bytes. Exact proposal.json,
findings.md, boundary-audit.json, unknown-decode.json/txt and residual-after-discovery.json
persisted. Primary/secondary initializers, all19 effect continuations, event callbacks
and independent helpers/PPU streams followed. Data records/streams and256 lookup
bounded. Proposed remaining bytes only physical2991(A981),44B2(C4A2), both loneRTS
ambiguities. No classification accepted. Preparing lead/handoff with independent
validation recipe, no terminal ROM-completion claim.
