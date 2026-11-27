# Round002 commentary handoff

Current-source readability complete; ROM classification remains incomplete:
130691/131088 bytes classified (99.70%),397 unknown in28 ranges. Final editorial
acceptance is pending. Next phase is redirect, not terminal audit.

Canonical source: commentary/source/game.asm and symbols.inc; refreshed reading:
commentary/reading-after.asm. Fresh commentary/verification.json confirms
byte_identical=true after final edits. ROM and map/state hashes are unchanged;
manifest identifies round002 map SHA256
0572498f2f2c0849b7d65ca3ac0e45d211b1544b91aeaa29c2ab796ff9ce9ee5.

Entire prior commentary preserved in analysis/round-002/commentary-prior before
preparation. Physical-range carry retains198 source blocks and281 prior units.
Only PaletteBytesDecodedAsCode was invalidated, with historical record retained.
All9521 scaffold entries reviewed:9381 code,105 data,7 image,28 unknown. Scope
includes6096 new physical instructions, changed dependencies,41 retyped palette
bytes and all new data. Added372 functional code anchors,35 data units and444
routine-based flow names; shared field/RAM/mapper/table operands are symbolic.
No current-source readability gaps remain; commentary-pending.json is empty.

Examples: RequestAudioSequence and command loops; ClearRamFromInlineArguments
and normal C031 versus inline-consuming C034 contracts; final-area messages;
InitializeAllocatedProjectile and the thrown-object fuse; persistent bitmap/layout
initializers; decimal score addition; terrain rectangles and banked PPU transfers.
Stale prior unknown-continuation claims were corrected. Review source locations
and valid prior credit refreshed. No invented enemy names or selector bounds.

Remaining meanings and concrete next questions: commentary/uncertainties.md.
Allocator seven/update six remains unresolved; main30/secondary16/spawn48/audio8
selector domains remain qualified. Final presentation CPU B08B/B17E and object
readiness B047 now readable; inspect dependent player/object/environment producers
before improving the saved final-stage shortcut. Failed raw-scroll warp is not a
fresh method. Score life-threshold intent, standalone natural callers and individual
data schemas remain tentative. Exact residuals: round-002/final-residual.json;
R002-RESIDUAL remains the narrow next structural lead. No ending visit, natural
reachability, full classification or terminal exhaustion claimed.

No new toolkit issues. NES-001 remains a nonblocking documentation/schema mismatch;
see toolkit-issues.md. No ROM/toolkit changes, publication, archive or deletion.
