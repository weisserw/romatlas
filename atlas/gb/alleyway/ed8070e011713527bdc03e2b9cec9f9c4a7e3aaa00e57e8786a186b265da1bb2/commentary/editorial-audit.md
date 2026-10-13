# Phase-2 editorial audit — 2026-10-13

The whole-game semantic/editorial review is complete. This is a phase-2 result;
phase 1 retains its 31 unknown bytes at physical/CPU $0426-$0444. No original
game behavior or instruction value was changed. Final machine-gate results are
recorded in audit.json and verification.json after the closing audit command.

Closing result: exit 0, ok=true, byte_identical=true, reviewed_regions=161,
reviewed_units=1077, editorial_audit=true. ROM SHA-256 remains
ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2.
The audit freshly rebuilt current source and refreshed reading-after.asm.

## Requirement-by-requirement findings

| Requirement | Current evidence and editorial result |
| --- | --- |
| Every ROM interval reviewed | review.json retains all 161 frozen scaffold entries, covering 32768 bytes without overlap/gaps; every entry is reviewed with summary/source locations. |
| Meaningful routines, loops, branches, alternate entries and shared tails | 1077 reviewed units; every assembly label has an inventory unit. Code-unit union covers every classified code byte. Units intentionally overlap for enclosing algorithms/shared entries. |
| Accurate physical boundaries and source links | Every unit start matches a fresh RGBDS linker symbol; every source pointer names its exact label. All code-unit ends coincide with instruction/region boundaries; all unit extents lie in ROM. |
| Descriptive names and useful variables/constants | Whole-source scan finds no generated code_/Call_/Jump_/jr_ labels or numeric direct control transfers. Conditional request-gate calls and the HRAM DMA entry are symbolic; branch/record names identify their roles. |
| Functional comments, game meaning and hardware effects | Reviewed startup/frame/input, state graph, scrolling/physics, scoring/bonus, UI/metasprites/graphics and audio caller/callee/data relationships. Comments explain interfaces, why synchronization/layout matters and actual effects, rather than per-opcode translations. Small return/preset leaves use descriptive names without repetitive headers. |
| Thorough data explanation | Header fields/checksums; VRAM mode/count/endian/sentinel grammar; stage flags/pointers/cells/HP; signed 8.8 velocity records; paddle maps/thresholds; relative sprite records/attributes; graphics ID aliases/bitplanes; audio register presets, frequency/duration tables, note/rest/end/restart streams and waveform nibbles are documented close to consumers/data. |
| Graphics review | Inspected all three local PNG atlases and matched their 2048-byte build assets to exact ROM slices. Documented 128-tile blocks, blank slots, Mario/paddle/ball/font/title content and the $8800 font's BG/sprite dual use. |
| Contradictory or misleading interpretations revisited | Corrected the returned sequence value's demo/angle uses; FFC5 is a scrolling threshold index, not relaunch delay; speed-display calls hit a RET, not the detached writer; music has pulse2/wave voices plus separate pulse1 effects/noise; tonal mute does not mute noise. |
| Original quirks preserved accurately | Bonus points wrap while brick points saturate; threshold equality delays extra-ball award; short-layout reveal cursor wraps; width-reset/demo clamp bounds differ; cue8..12 retain restart ID6; pulse music end writes NR12 rather than NR22. These effects are explained, not silently fixed. |
| Investigated ambiguity | uncertainties.md records observable behavior, alternatives, methods, limits and concrete reopening evidence for the zero interval, serial accessory identity, collision tail, unused angle choices/grids/presets, speed-display stub, legacy sound-test ABI/flags and pulse shutdown intent. No further useful local discriminator is identified; unknown history is not presented as fact. |
| Phase-1 preservation | Scaffold id/offset/length/kind/phase1 fields match freshly reconstructed frozen scaffold. Original ROM, frozen/current phase-1 map/state hashes match preparation manifest. reading-before.asm matches before/ expansion; notices and assets are preserved. |
| Final fidelity/readability artifacts | commentary.py audit performs a fresh unmodified-header rebuild/byte comparison and refreshes reading-after.asm. Source remains the maintained build target; reading copy retains constants/hardware includes and binary references. |
| Toolkit/local-artifact rules | Toolkit repository/binary were not changed; commercial source/assets/ROM/traces stayed local. T001 remains a nonblocking metadata-API missing feature. No new toolkit issue was encountered. |

## Editorial corrections and verification scope

The final pass replaced address-bearing priority-return names, removed stale
restart-opcode wording after DS conversion, gave vector padding its own source
anchors, inventoried the paddle tile-table alias and made mixed audio-region
summaries identify the complete algorithms rather than just their first helper.
It refreshed every source pointer. One multiply-loop unit had ended inside a
jump operand; its end is now $046d. The auxiliary width scanner omitted JR Z's
second byte; corrected decoding gives 4845 classified-code starts, with all4488
distinct starts in the older capture included. This correction does not change
the no-incoming-reference result for unreferenced grids or any ROM bytes.

Whole-source review also checked generic names, omitted structures, boundaries,
numeric aliases, contradictory comments, unjustified certainty and redundant
comments. The closing automated checker supplements this judgment; status and
byte identity alone were never used as semantic proof. Assembly routines/data
were reviewed statically; no whole-game execution or full hardware compatibility
is claimed. Existing phase-1 execution evidence/replay routes remain unchanged.

## Preserved structural counts

These are internal phase-1 kinds, not execution coverage or phase-2 unknown work.

| Physical bank | Code | Data | Image | Padding | Unknown | Total |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 4550 | 11563 | 0 | 240 | 31 | 16384 |
| 1 | 4953 | 2887 | 6144 | 2400 | 0 | 16384 |
| Total | 9503 | 14450 | 6144 | 2640 | 31 | 32768 |

No bank ambiguity remains for this ROM-only cartridge: physical bank0 maps CPU
$0000-$3fff; bank1 maps $4000-$7fff. Keep source/ and its local assets to retain
the final commentary; the stock .sym alone does not carry these comments.
