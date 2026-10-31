# Generic non-code boundary/competing-interpretation review

These are Static proposals, not accepted annotations. Exact generic subranges are normalized against specific regions in proposals.json. Semantic generator and full original object layout are not claimed. Non-execution is not the classification evidence.

Practical role inspection: fixed DD49..DEB7 consumers read room maps, indexed pointer dictionaries,20+4 column tiles or32 row tiles, and6/8 attribute bytes in selected bank pairs6/7,8/9,10/11. Pair8/9 has a real code-image counterexample and is excluded. Pairs6/7 and10/11 contain sustained repeated tile-index/attribute fields and pointer layouts over their payloads, bounded independently by152-byte bootstrap and six-byte bank vectors. Packed tails display dense recurring bit motifs across their intervals. Audio tail includes referenced stream/pointer/descriptor structures followed by the same visibly non-instruction bit patterns. Specific partitions beyond separately proven tables remain unknown.

Competing interpretation: copied machine-code image or small routines hidden among patterns. Inspected boundaries and all accepted/new code roots; no conflict. Checked every accepted Code region of at least24 bytes against payloads below, excluding known bootstrap/vector boundaries; no matches. This check is supplementary, not proof from absence. The positive pattern/consumer consistency distinguishes these payloads from pair8/9, whose visible state-setting instruction sequences match current code and contain coherent control flow.

Localized uncertainty: byte60 at180A8 could be a standalone RTS or payload prefix; it stays Unknown. Its uncertainty does not extend into the sustained patterned suffix180A9..1C00A. The DMC range1FC10..1FE11 has independent descriptor-derived source/length and overrides generic tail labeling.

## $00C0A8..$01000A (16226 bytes), room graphics bank pair6/7

Distinct byte values: 252; frequent values: $53:3060, $00:2687, $55:515, $02:275, $CE:251, $CF:247, $C3:243, $07:238. Counts describe the observed representation; no fitted acceptance threshold.

Left flank: 8d ff ff 4a 8d ff ff 4a 8d ff ff 60; first32: 00 01 02 03 04 05 06 07 08 09 0a 0a 0b 0c 0d 0e 0e 0f 10 11 68 11 10 13 14 15 16 17 18 19 1a 11; last32: a2 a8 ab ba aa aa 8a 88 a8 88 ea a8 8a a8 aa aa aa a8 a8 8a a8 88 aa ac 8a a8 aa ae aa ac 8a 8a; right flank: 04 01 00 c0 14 03 78 a2 00 8e 00 20.

- $00C0A8: 00 01 02 03 04 05 06 07 08 09 0a 0a 0b 0c 0d 0e 0e 0f 10 11 68 11 10 13 14 15 16 17 18 19 1a 11 1b 1c 11 11 1d 1d 1e 1e
- $00C8A8: 00 7c 7d 00 2b 82 00 00 2f 00 00 00 00 82 81 85 00 82 81 7d 82 7d 00 00 2b 87 00 00 00 00 00 2f 00 00 00 00 00 00 00 00
- $00D0A8: 08 08 28 55 3e 55 55 3d 08 08 00 0f 17 4b 1a 0f 00 53 16 53 53 53 53 15 53 00 1c 1c 1c 1c 1c 1c 1c 10 09 09 09 09 4b 09
- $00D8A8: 53 53 7d 6d 6f 5f 00 53 53 53 53 53 53 53 53 53 53 53 53 53 53 53 07 55 09 5f 00 53 53 53 53 53 53 85 38 53 53 53 53 53
- $00E0A8: 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16 16
- $00E8A8: 36 36 37 53 53 53 53 53 07 55 08 53 53 53 53 53 53 53 53 53 53 53 53 53 53 53 53 53 07 b2 08 53 53 53 53 53 53 53 53 53
- $00F0A8: a1 a1 a2 d0 7a 53 53 53 53 53 52 52 07 07 07 07 07 07 07 07 64 64 64 64 64 07 07 07 07 0e 0f 98 99 99 9a 0e 0f 07 07 07
- $00F8A8: c6 c6 72 c3 c6 c6 c6 c6 c6 72 c3 c6 c6 c6 c6 c2 c3 47 7c 7c 7c 7c 7d c3 00 7c 7c 7c 7c 7d c3 00 7c 7c 7c c2 c3 4a 7c 7c

## $0140A8..$01800A (16226 bytes), room graphics bank pair10/11

Distinct byte values: 241; frequent values: $00:2058, $55:1600, $01:1101, $02:1063, $A8:599, $AA:512, $54:292, $41:279. Counts describe the observed representation; no fitted acceptance threshold.

Left flank: 8d ff ff 4a 8d ff ff 4a 8d ff ff 60; first32: 02 02 02 02 02 02 2d 2e 22 23 02 02 24 25 02 02 26 27 02 02 02 28 29 02 02 2a 2b 2c 02 02 02 02; last32: 88 a8 aa aa a8 8a 9a 8a a8 88 aa aa 8a aa aa a8 88 e8 aa aa 8a 8a cc c8 aa aa 8a aa aa a8 a8 aa; right flank: 04 01 00 c0 14 03 78 a2 00 8e 00 20.

- $0140A8: 02 02 02 02 02 02 2d 2e 22 23 02 02 24 25 02 02 26 27 02 02 02 28 29 02 02 2a 2b 2c 02 02 02 02 13 14 02 15 16 17 02 02
- $0148A8: 01 00 01 00 01 00 01 00 01 00 0d 0c 01 00 0d 0c 01 00 01 00 01 02 03 04 05 06 01 00 01 00 0d 0c 01 00 01 00 01 00 01 07
- $0150A8: 00 01 00 01 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03 02 03
- $0158A8: 00 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 55 00 52 52 00 55 55 55 39 37 39 55
- $0160A8: 0d 0e 0f 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f 00 00 20 00 20 1a 00 20 00 00 00 00 00 00 00 00 00 00 2a 27 21
- $0168A8: 01 18 1a 02 01 ae 00 80 ae 87 88 01 02 01 02 01 02 01 02 01 2a 18 1a 01 2a 18 28 00 ae 00 02 02 01 02 01 02 01 02 01 02
- $0170A8: 05 41 00 00 01 02 01 02 01 02 01 02 01 02 01 02 01 02 05 07 04 40 45 00 01 01 02 01 02 01 02 01 02 01 02 01 02 01 04 06
- $0178A8: 04 05 04 00 01 02 01 02 01 02 01 02 01 02 01 02 01 02 01 07 07 02 01 02 01 02 00 05 04 05 04 04 05 04 05 00 02 01 02 01

## $0180A9..$01C00A (16225 bytes), packed pattern suffix12/13

Distinct byte values: 184; frequent values: $AA:1050, $8A:754, $A8:687, $55:665, $00:620, $44:584, $45:538, $2A:530. Counts describe the observed representation; no fitted acceptance threshold.

Left flank: ff ff 4a 8d ff ff 4a 8d ff ff 60 60; first32: 14 44 40 00 40 04 02 45 45 40 40 44 00 01 44 05 00 00 44 50 00 40 00 44 04 00 40 40 00 44 44 14; last32: 88 aa aa aa aa ae 88 a8 8a a8 aa a8 aa aa aa a8 8a aa a8 88 8a a8 89 88 8a 8a e9 8e 8a aa aa 8a; right flank: 04 01 00 c0 14 03 78 a2 00 8e 00 20.

- $0180A9: 14 44 40 00 40 04 02 45 45 40 40 44 00 01 44 05 00 00 44 50 00 40 00 44 04 00 40 40 00 44 44 14 44 00 44 44 40 00 00 41
- $0188A9: 15 55 11 11 05 01 11 10 00 05 50 05 41 44 01 55 11 11 45 01 11 41 02 54 40 41 34 45 51 50 45 50 41 45 14 51 14 45 10 24
- $0190A9: 04 44 44 44 04 04 04 45 04 14 04 44 40 44 40 04 04 00 44 40 04 44 40 05 44 40 40 05 44 04 00 44 04 02 00 44 00 04 00 04
- $0198A9: 50 44 15 10 40 01 71 51 41 50 40 14 41 14 51 14 14 40 41 55 50 55 90 55 51 04 44 11 14 51 1d 50 55 05 44 11 14 44 d1 45
- $01A0A9: 8a aa 8c 2a 80 a0 8a 82 82 a2 88 a2 80 02 0a 88 22 08 0a 0a 88 40 02 80 02 28 80 ac 28 8a 2a 88 20 0a 0a 00 a2 8a 2a 08
- $01A8A9: a8 88 a8 28 88 a8 a0 08 02 08 a3 2a 2a a2 8a aa a0 2a 8b 00 a2 0a 82 68 2a 8a 80 88 80 80 a2 82 aa a0 a8 aa a2 0a 2b 02
- $01B0A9: 8c 48 28 02 2a a2 8a 22 0a 8a 22 a8 aa 06 0a 22 2a a0 22 a0 a8 00 2e 28 c0 88 22 88 a8 aa a0 40 aa a8 a8 a0 20 08 8a 88
- $01B8A9: 20 82 a8 28 8a 28 a2 02 88 88 06 40 20 88 a2 82 8a a2 28 2a 28 28 a2 80 08 28 00 48 8a a8 82 21 aa 02 a2 22 a8 e8 82 0a

## $00AB37..$00C00A (5331 bytes), audio structures and packed tail

Distinct byte values: 192; frequent values: $08:313, $01:313, $AA:257, $00:205, $60:191, $A8:178, $10:175, $8A:170. Counts describe the observed representation; no fitted acceptance threshold.

Left flank: 85 fd 86 fc a9 00 8d 07 07 4c c5 aa; first32: 20 f0 63 ac 6e ac 82 ac 8b ac 96 ac a5 ac b0 ac bb ac c8 ac d4 ac da ac e6 ac f5 ac fe ac 07 ad; last32: aa aa 8a a8 aa ae aa 8a c8 88 aa 8a a8 bb 8a a8 a8 8a 8c aa b8 aa a8 8a 8a aa 88 8b 88 88 ba 8a; right flank: 04 01 00 c0 14 03 78 a2 00 8e 00 20.

- $00AB37: 20 f0 63 ac 6e ac 82 ac 8b ac 96 ac a5 ac b0 ac bb ac c8 ac d4 ac da ac e6 ac f5 ac fe ac 07 ad 07 ad 09 ad 07 ad 1d ad
- $00B337: 2f 24 01 18 17 80 60 18 81 00 83 00 18 10 1b 08 85 08 13 15 17 86 18 10 1b 08 85 08 13 15 17 86 18 10 1b 08 85 08 13 15
- $00BB37: a8 8a aa a8 ba 8a 8a a8 8a 88 aa 88 a8 8a aa a8 8a aa a8 9b 8a aa 8a ba 88 aa 8a 88 9a aa aa aa 88 8a aa a8 aa aa 8a a8

## $01F70C..$02000A (2302 bytes), fixed-bank packed tail

Distinct byte values: 89; frequent values: $AA:556, $A8:194, $8A:187, $00:168, $FF:134, $88:122, $2A:92, $A2:91. Counts describe the observed representation; no fitted acceptance threshold.

Left flank: 00 00 03 01 04 00 00 01 00 00 00 00; first32: a8 aa aa ae 00 72 aa 08 8a 2a 22 0a 28 a0 22 88 20 aa 22 a8 a2 82 a8 20 ac 8a 28 aa 0a 12 a2 23; last32: 88 aa 8a aa 8a 8a 88 88 a8 aa 8a 8a a8 8a a8 a8 aa 8a 8a ae aa 8a aa aa 8a 8a 8a aa aa a8 9a 8a; right flank: 04 01 00 c0 14 03 ff ff ff ff ff ff.

- $01F70C: a8 aa aa ae 00 72 aa 08 8a 2a 22 0a 28 a0 22 88 20 aa 22 a8 a2 82 a8 20 ac 8a 28 aa 0a 12 a2 23 0a a2 22 a2 aa 02 24 02
- $01FF0C: 22 aa aa a9 aa 88 88 a8 aa aa a8 aa a8 ba 88 8a aa aa 88 aa aa a8 8a 88 aa a8 88 a8 8a a9 8a 88 a8 a8 88 aa ac aa 8a a8

