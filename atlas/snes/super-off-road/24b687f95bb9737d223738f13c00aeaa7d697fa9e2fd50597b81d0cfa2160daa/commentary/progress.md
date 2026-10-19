# Commentary progress

## Round27 — current-source readability handoff

Carried the complete round22 commentary source and review credit into the fresh round26 baseline. Integrated the 32 newly classified interleaved bytes in the bank-00 physical window `$068B..$06CB` with mechanical labels and comments. The comments describe only the bounded two-byte-stride storage-window role and observed ROM byte value `$00`; slot-internal semantics remain unresolved. Updated stale adjacent-byte wording in the selected heading entries. No high-field, padding, word, use/write or reachability semantics are assigned.

All 1,527 review scaffold entries are `reviewed`; all current source locations resolve, and every Code entry has a reviewed unit. The one structure unit documents the consumer-bounded 32-slot window and leaves the interleaved-byte interpretation unresolved. The complete carried source now covers all 1,437 classified map regions. The 90 unknown intervals (5,398 bytes) remain unknown and locally documented. The entire prior commentary tree is preserved at `../analysis/round27/commentary-round22-original/`.

Fresh `commentary.py verify` passed byte-identical for all 524,288 bytes using the configured release binary; `commentary.py read` refreshed `reading-after.asm`. ROM SHA-256 is `24b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa`; the local source mapping matches `maps/current.json`. Exact proof: [round27 commentary verification](../analysis/round27-commentary-verification.json). Final editorial/archive audit remains pending and whole-ROM classification is incomplete. No runtime, map, state, ROM or toolkit changes. No new toolkit issues found; see `../analysis/toolkit-issues.md`.
