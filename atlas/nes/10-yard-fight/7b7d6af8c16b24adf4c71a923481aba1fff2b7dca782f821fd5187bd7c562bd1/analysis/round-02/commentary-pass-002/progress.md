# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round01 full-source semantic batch
Surveyed all 6,022 baseline lines across all 117 physical scaffold regions. Added 217 named/commented supported entries, 705 routine/control-flow labels, 126 zero-page aliases, player-record field/stride symbols, hardware constants and 102 ROM-table/stream entries. Source comments explain NMI dispatch, stack-unwinding control entries, 18-player records, contact matrix, scrolling buffers/strip decoding, decimal conversion, ball/AI mechanics, audio commands, title/skill selection and data formats. All 19 unknown intervals have explicit opaque-boundary notes. These are static mechanical interpretations; unresolved football event identities remain tentative.

First coherent source batch verified byte-identically with configured binary via commentary.py verify. Source SHA-256 2f089b926d41fa02fb2e7acec4dc89ac35ee03af8bf6f5740167bb6064aa44ba. Phase remains in progress: reviewing generated names and comments for mistaken mechanics, incomplete symbols, and all pending scaffold/unit records before final verification/read refresh. Structural map and input unchanged. Initial ca65 expression grouping and physical-versus-CPU label alias mistakes were corrected before successful verification; no toolkit defect.

### Consistency cleanup
Replaced every residual address-only branch fallback with a functional block name and identified the shared four-byte record advance helper. All supported direct jump/call/branch destinations now have descriptive symbols; indirect targets retain explicit dispatch contracts. Replaced every numeric direct absolute/zero-page operand with hardware, RAM, record-field or table expressions. Adjacent pointer initializers now use low/high-byte expressions for named ROM structures. Added VRAM queue schemas, contact-matrix stride, OAM layout and shared stack-page scratch notes. Corrected the NMI pause/handshake epilogue description during source comparison.

Fresh verification still passes, source SHA-256 7685e3a7f6e442e1e929ea05d0baea2b7641c0e699902c8239f88921f448c560. No new toolkit issue. Finishing complete physical scaffold/unit accounting and compact redirect questions; final reading copy pending.

## Round01 commentary complete — current-source readability
All 117 physical scaffold entries are reviewed with source locations and 279 compact reviewed units. Covered all 11,290 classified code bytes, 19,394 data bytes (including header/vectors) and 8,192 CHR bytes; all 2,100 unknown bytes retain explicit opaque boundary notes. No current-source readability range remains pending. The final editorial audit remains pending intentionally; ROM classification remains 94.88%, incomplete.

Final source has 218 named behavior entries, 705 semantic routine/control labels, 126 RAM aliases and 102 table/stream symbols. Purpose/algorithm/state/stack contracts are in source, with data formats and generic-data limitations nearby. Full physical/unit accounting and identity checks: analysis/commentary-round01/readability-accounting.json. Fresh documented verify passed byte-identically and read refreshed reading-after.asm. Final source hash 7685e3a7f6e442e1e929ea05d0baea2b7641c0e699902c8239f88921f448c560. Input, maps/current.json, analysis/state.json and frozen commentary baseline hashes are unchanged. No final audit/archive/publish action taken.

Remaining meanings/structural questions are collected in uncertainties.md; none is a skipped readability range. Toolkit: no new findings during commentary; the persistent report retains one confirmed NES input protocol mismatch, with tested buttons-array workaround and no blocker. See ../analysis/toolkit-issues.md.
