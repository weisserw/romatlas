# Round 1 redirect handoff

- Attempt id: **r1-a1**. All files are in analysis/redirect/r1-a1/ (notes.md, proposal.json, residual-ledger.json, tools/, evidence/).
- Input checkpoint: **pass-001**. maps/current.json is identical to maps/pass-001.json, and the map sha256 is d19c663a...815.
- Classification is unchanged at 65.78% (107,792 of 163,856 bytes; 56,064 bytes unknown). This redirect did not edit the map, state, source, ROM or toolkit.
- Recommendation: **NEXT ROUND** (disassembly).

## Proposed bytes
**48,538 of the 56,064 unknown bytes**, listed in proposal.json. The breakdown is approximate:

| Group | Unknown bytes |
|---|---|
| DPCM (L-010) | ~26.3k |
| Sound (L-005) | ~12.9k |
| Bank 0/1 screens (L-015) | ~4.8k |
| Bank 7 (L-014) | ~2.8k |
| Code (L-001/007/011/012/013) | ~1.6k |
| Small tables | ~0.1k |

Checks run on the proposal:
- No proposed range conflicts with any non-Data classification.
- No proposed byte is claimed both as Code and as Data.
- The only optional item is 64720..64773 (marked "Data?").

The residual complement after the proposal is 7,526 bytes in 222 ranges (residual-ledger.json).

## Ready leads (recipes in analysis/leads.json)
- **L-010 (DPCM samples).** VoiceTable1/2 each hold exactly 7 entries.
  - The DMC address is $C000 + 64 * ((word << 2) >> 8). The length is 16 * L + 1.
  - The samples tile contiguously over 16464..31313 and 32848..48336. The last sample ends at the first byte of VoicePlayer2.
  - Also proposed: the $AA fill at 31313..31372, and the 27-byte residue at $C025-$C03F, which is identical in banks 2 and 4.
  - Voice test selections 0-10 match the 11 names on the voice test screen.
- **L-005 (sound data).** tools/sound.py parses SongTable (27 words) through the headers, channel entries, envelopes and scripts. It covers 13,048 bytes with zero conflicts.
  - Command lengths come from the handler code.
  - NotePeriodTable has 13 words (the highest tonal note nibble is 12). ChannelOffsetTable has 6 bytes. SoundCommandTable has 9 words.
  - Command 7's handler at $82EC-$8315 is code.
  - Bank 13 $B744-$B943 is byte-identical to bank 1 at the same CPU addresses. It is build residue with no consumer in bank 13.
- **L-015 (bank 0/1 screens, 7563..14676).** The data is a chain of palette (32 bytes), metatile set (5-byte records) and compressed screen objects.
  - Screens describe their own size: word0 = size - 2, and word1 = byte 2 + 4 as read by DecompressScreen.
  - Every labelled object ends where the next one starts. Two unreferenced screens chain in the same format.
  - All metatile set sizes are multiples of 5. One byte ($AD28) is unexplained.
- **L-014 (bank 7, 61951..64773).**
  - TrickShotScripts (8 words) and 7 scripts tile exactly. They are followed by two 8-byte per-PracticeShot tables.
  - $B2E2-$BCF4 is byte-identical to bank 5 (DPCM tail).
  - The last 53 bytes are a copy of VoicePlayer2 that nothing in bank 7 references. The reviewer should decide whether to classify them as data or leave them unknown.
- **L-001 (reopened with a corrected premise).** The rejected closure reached phys 4152 only through BNE $9028 at $9025, which comes right after LDA #$00 and so is never taken.
  - Phys 4152 is CharToTileTable: 43 bytes for chars $30-$5A, ending at $9053.
  - With that one edge pruned, the closures from $83C2, $82E8 and $8E39 are clean. They show no illegal opcodes, no non-code overlaps and no misaligned overlaps.
- **L-011 (unknown FarCall/library JMP slots, banks 0/6/8/10/14).**
  - Proposed as code: the unknown slots, and the 6-byte reset stubs at $8000 of banks 2-12.
  - Static closures from the unknown slot targets are clean: $9CD1, $8E39 (stage-won screen), bank 8 $8966, and bank 14 $C3C3, $CA1D, $CDCD and $CE25.
- **L-012 (bank 6).** Proposed as code: $95FF-$9622, $818D-$81FD, $820E-$8222 and $8228-$8233.
  - Also proposed as data: tables $81FE (index $C7) and $8223 (phases 9-13), CpuThinkTable (15 words) and 8 non-code bytes at $8185.
- **L-013.** $C33E-$C34D is a clean 16-byte closure in the fixed bank.
- **L-007** is covered by the L-001/L-011 closure.
- **L-008.** Residual bytes next to Text follow two patterns:
  - a single $00 terminator (54 cases);
  - '00 col row', a terminator followed by the next string's PrintString header.
  - Together these cover about 1,300 bytes.
- **L-016 (residual ledger).** There are 42 consumer-anchored table ranges (4,808 bytes), and the reading instruction for each is listed.
  - The 1,303 bytes in gaps between code have no JSR/JMP/branch reference from classified code.
  - L-009 and L-002 are partly resolved through L-015, L-005 and L-012.

## Gameplay (scenarios.json)
- RAM shortcut tested: write $00AB = N (TourStage) while the players menu is open, then continue the games menu. Context is [0,1,14,15].
- N=6 runs EndingSequence. The ending text "YOU HAVE FINALLY BECOME AMERICA'S NO.1 HUSTLER!..." appeared (SC-008).
- N=5 opens the sound test (SC-012) and N=7 opens the voice test (SC-013).
- These are **artificial** states and do not show natural reachability.
- The traced ending run was killed when it hit the 1500 s job limit, because tracing runs about 10x slower. Only the screenshots were kept.

## Deferred avenues and reasons
- **Later tour cities 1-3 and the natural stage-won screen ($8E39):** the RAM contract for a match win has not been identified. The next step is to find the writer of MatchGames/RoundState for a win, or patch TourStage before a tour match and force the result.
- **Practice/Trick Shot coverage (L-006):** the scripts are now decoded statically, so the value is lower. Route SC-009 is still valid.
- **Bank 10 staff credits display:** no consumer has been located. The EndingSequence loops a WATCH demo instead.
- **Bank 14 JMP ($0030) bound (L-002 remainder):** not re-examined.
- **L-015 screen decompression:** the max-metatile check was not implemented. The chain proof does not depend on it.

## Rejection review
Not required, because this is not a terminal handoff.
