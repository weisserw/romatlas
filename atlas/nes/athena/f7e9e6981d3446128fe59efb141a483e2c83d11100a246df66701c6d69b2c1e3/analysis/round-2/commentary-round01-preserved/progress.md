# Round 1 commentary — current-source readability complete

Baseline: round01-disassembly-final; maps/current.json and analysis/state.json were frozen by supported preparation into commentary/baseline.json and phase1-state.json. Before/source separation preserved. No classification changes or runtime experiments during commentary.

Reviewed all 6,483 code bytes (3,149 emitted instructions), 36,148 data bytes including the header, and explanatory boundaries for 88,457 unknown bytes. All 514 full-ROM scaffold entries are reviewed. Added 213 functional-unit records and 222 used RAM, hardware and table-alias constants. Unit extents stop at current classified-code boundaries; unknown continuations remain unknown. The source retains physical ranges and original provenance separately from semantic comments.

Names/explanations cover title/HUD rectangles and digits, OAM object expansion, reset/NMI and the 128-byte transfer ring, cooperative task stack frames/yields, six-voice audio arbitration, scrolling/metatile/attribute caches, mapper wrappers, stage graphics scripts, scene setup and object entry/exit motion. Understood branch destinations inherit functional names and continuation suffixes; unresolved control-flow aliases remain explicitly unresolved. Shared RAM names describe mechanics, with temporary reuse documented.

Concrete progression/scheduler, sprite-format, audio-command and stage-descriptor questions are in uncertainties.md. No readability ranges remain deferred. Classification and final editorial acceptance remain incomplete; editorial_audit is pending, and the final audit was intentionally not invoked.

Verification: fresh supported commentary.py verify reported byte_identical=true for the entire owned ROM; commentary.py read refreshed reading-after.asm after source completion. See verification.json for exact source/ROM hashes. No toolkit issues found; ../analysis/toolkit-issues.md. All artifacts remain local. Free disk 8.5 GiB; scratch 44 KiB; no cleanup performed.
