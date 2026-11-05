# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round 001 active readability pass

Full source inventory inspected. Editing functional units across player motion/actions, scene event commands, entity behavior/collision, HUD, NMI PPU queue, OAM, audio sequencing, scene streams, terrain and animation. Shared hardware/RAM symbols and bank-aware control flow names pending installation. All classified units must pass readability review before handoff. Frozen baseline unchanged; preparation verified byte identity. No completed review credit claimed yet.

## Round 001 current-source readability complete

All 503 classified entries (176,201 bytes) reviewed, including all 13,090 code bytes. Added 134 functional code units spanning player actions/motion, event commands, entity dispatch/collision, HUD/score conversion, PPU queue and OAM, audio sequencing/envelopes, scene streams, terrain, animation, pause/presentation and MMC3 raster callbacks. 618 entry/control-flow names and 328 shared RAM/hardware names replace understood numeric operands; descriptor-table CPU aliases are separate from raw-file labels. 138 unknown spans (85,959 bytes) have honest boundary/uncertainty notes. No pending readability entries remain.

Fresh whole-file byte-identical verification passed after source edits; final report commentary/verification.json. reading-after.asm refreshed at handoff. Frozen maps, phase1-state, before and reading-before remain unchanged. Current-source review is separate from final editorial/archive audit, which remains pending. See uncertainties.md and analysis/round-001-commentary-readability.json. No runtime/disassembly/redirect, cleanup, publication or toolkit modification performed. Existing NES-001 confirmed input-schema bug remains the sole toolkit finding; workaround available, no blocker.

## Round 002 active

Prior phase preserved under analysis/round-002-commentary/prior-commentary before rebase. Physical-range merge in progress; new code/data and changed dependencies pending. No successful handoff claimed.

### Round 002 merge checkpoint

All 503 prior classified fragments carried by exact physical range. Source now includes all 93 new code fragments with 146 functional-unit explanations and all 1017 new data fragments with consumer/format or explicit bounded-fallback notes. Rebased baseline/current map SHA256 d6af065d7fd00334045ea58046b2704e3d4a51caa5f3b6be8db0abc7461ea863. Fresh verification passed byte-identical (source c789e35d7b96a514386acf8e27d4c9a1e204fd7b4a9d3fb36fb540e1bd256858). Final symbol/dependency consistency and source-location check pending; reading copy not yet final.

## Round 002 current-source readability complete

All 1613 classified scaffold entries / 262157 bytes are reviewed, including all 19471 code bytes; the sole three-byte unknown entry also has an explicit local uncertainty note. Carried 503 earlier classified fragments and their valid review credit from the preserved prior phase. Added 146 functional code units across 93 new code fragments (6381 bytes), plus specific consumer/format or bounded fallback explanations for 1017 newly classified data fragments (79575 bytes). The 134 earlier functional units remain credited; updated only delta-affected source prose, understood targets and concrete inconsistencies.

New explanations cover pickup rewards and saturated healing, retry/loss handling, event scene transitions and stack unwinding, entity spawning/patrol/contact behavior, mirrored sprites, sound-bank wrappers/pitch stepping, pause/exit/opening screens and calibrated raster callbacks. Shared hardware/RAM names are reused in new operands; note-period and platform-height lookup CPU aliases distinguish effective addresses from raw-file data labels. Data notes state column count/polarity, 26-row decode bounds, attribute stride/copy limits, bootstrap/replacement event domains, terrain geometry, sprite/descriptor formats, packed screens and sound records. Generic fallback retains actual bank-specific inspection and unresolved selection.

Fresh final verification: byte_identical=true, source SHA256 78ccd6f610af65d0d86b5200571d77ec5083cd5b9f9208994e95ef710c9d79b7. reading-after.asm refreshed after these edits. Current frozen baseline/map/state hash d6af065d7fd00334045ea58046b2704e3d4a51caa5f3b6be8db0abc7461ea863; prior frozen baseline/source/history retained under analysis/round-002-commentary/prior-commentary. Review scaffold identities, reviewed-unit ranges and source locations checked without running final audit. No pending readability entries or deferred ranges. Unresolved game meanings are documented in source and uncertainties.md. Final editorial audit remains pending.

No redirect/runtime/disassembly work, toolkit changes, archive, deletion, cleanup or publication performed. ROM remains unchanged.

## Final editorial acceptance complete

Targeted acceptance reused round-002 complete readability credit. Corrected FinishNegativeTargetVelocity shared zeroing-exit prose and stale GetVerticalSeparation review summary. Normalized review locations to the source-root-relative paths required by supported audit; verified 620 named units against actual source names and 1017 generic units against exact boundary comments. All 1614 scaffold entries / 1637 units reviewed, no deferred ranges. Supported final audit passed, byte_identical=true, reading-after refreshed after last assembly edit. Source SHA256 3469e121290d07901143b0f305d10e3f7af56ca4c2a4cf731baf882569eb39b6. Exact physical [$3327,$332A) remains Unknown; classification is still incomplete. See editorial-audit.md and audit.json. No map/toolkit/runtime/archive/cleanup/publication work. Existing NES-001 only, zero new toolkit findings, no blocker.
