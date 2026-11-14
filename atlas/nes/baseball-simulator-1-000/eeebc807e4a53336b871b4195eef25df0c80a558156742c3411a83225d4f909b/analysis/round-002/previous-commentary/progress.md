# Commentary progress

Round 001 current-source readability is complete. Whole-ROM classification remains incomplete at 62.45%; final editorial/archive acceptance is pending and no final audit was run.

Inspected and explained all 21,899 classified code bytes, all 10,746 Data bytes, 131,072 CHR bytes and boundaries/uncertainty for 98,443 Unknown bytes. All 3,959 original scaffold entries retain their physical ranges and execution views; 267 meaningful routine/structure units add compact semantic review. Fragmented code entries link their functional source context; unit lengths bound reviewed classified spans without claiming recovered original routine extents.

Source explains the audio engine, menu/save/team setup, text/layout/decimal formatting, presentation objects, role input/automatic decisions, live match/batter/pitcher state machines and fixed NMI/PPU/MMC1/RAM/checksum runtime. Added descriptive hardware/RAM/field/constant symbols, replaced 1,159 generated entry labels, named 436 numeric branch targets and explained 67 additional routine entries inside grouped regions. Unknown game meanings remain ordinary tentative prose.

Examples in `source/game.asm`: `MusicInitializeSong` line 282; `B2_TextFormatNumericField` line 10561; `B6_MatchBeginHalfInning` line 15932; `B6_PitcherHandleInput` line 18600; `AudioRequestInlineByte` line 20149; `CallBankedInlineSelector` line 26520. Header's misleading unknown-audio note was corrected. A later source-label consistency check removed an unjustified handedness direction name. A local branch-label collision was repaired; the fresh rebuild below supersedes intermediate failed checks.

Fresh final verification (`verification.json`, copied to `../analysis/round-001-commentary-verification.json`) passes byte-identically. game.asm SHA256 `0dbb4dcbb524a2935a45a14db7c99f6b933660942c9d9a45128116baa6450d57`; readable-symbols.inc SHA256 `01d0477718faffa94102931285af869c925f69fd3af600bd65841308692a4aa1`. `reading-after.asm` was refreshed after this verification. Baseline map SHA256 remains `38c23b1d127cbeb734575c70ff102751d98f0811575fb8ae7876bbf701ce6bae`; input and phase-1 state are unchanged.

No readability ranges remain deferred. C001 (apparent twelve-byte audio tables still classified Code) and C002 (distinguish one-byte inline audio from two-byte banked calls), plus unresolved game/table meanings, are documented in `uncertainties.md` for redirect. These limitations preserve map classification and do not assert later-inning/terminal runtime coverage. Final editorial audit remains pending.
