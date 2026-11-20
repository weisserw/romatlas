# Round 5 redirect handoff (attempt r05-redirect-a1)

Status: COMPLETE. The input checkpoint is pass-005: 100.00% classified, 0 unknown bytes, map sha256 184532e6..., byte-identical.
- Only research, lead and scenario records changed. The map, source, ROM, state.json and toolkit were not touched.
- Runtime work was one bounded natural-input Read watch: the tour intro, 640 frames, with no RAM or register edits and `serve` without `--store`.
- Evidence is in analysis/redirect/r05-redirect-a1/:
  - findings.json (F1-F5)
  - runs.json
  - storyrec.json, orderlists.json, patterns.json, wordscan.json and storywatch.json
  - tools/*.py. Run them with `python3 -I`.
- Fast check: `python3 -I analysis/redirect/r05-redirect-a1/tools/checks.py $WS`. It asserts and needs no emulator.
- Disk: 4.0 GB free; the attempt dir is 92 KB; there is no scratch left over.

## Corrected inference (why this is not a repeat)
The r03 refs.py did find little-endian *word* hits into the residual blocks. For example, it found 142 hits into 0:$9087-$9180 and hits at 2:$8070 into 2:$83D4. But it truncated the list to 12 hits and never attributed them. r04 idxscan.py looked only at indexed operands.

The new premise: look for pointer words held in a field of an already-accepted record structure or pointer list. Then follow the consumer's format to an extent proof. The method is to attribute the word hits to their containing structure (tools/wordscan.py), then parse that structure.

## Ready leads (0 new bytes, no kind change; recipes in analysis/leads.json)
| Lead | Priority | Claim |
| --- | --- | --- |
| L-029 | medium | **0:$90B2-$9180 (phys $010C2-$01190, 207 B) = StoryTileRecords.** The data-pointer fields of the StoryTextBlocks frame records point to 27 records. Each record is a header byte (rows<<4 \| width) followed by rows*width tile ids, and NmiHandler10 copies them to the PPU. Each record is followed by an unread separator byte (29 B in total). The 10 blocks exactly cover $9181-$949A. The first record starts right after MapTruckPath, and the last one ends at $917F. A natural intro read watch saw 159 of the predicted bytes read; the two records at $9149 and $9154 were not reached within 640 frames. One extra 7-read hit at separator $9102 is unexplained, and no claim is made about it. The recipe merges regions 2026, 665 and 1726 and splits 666 at phys $01191. |
| L-030 | medium | **Bank 2 sound structure.** SongChannelOffsets (2:$8070) holds the *absolute bases* $83D0/$8430/$8490/$84F0 of four 48-word order-list pointer tables. StartChannel indexes them with SoundRequest<<1, and together they fill exactly $83D0-$854F.<br>The 188 pointers rise monotonically from $8708 to $8C14. The parsed order lists cover $8708-$8C14 with no gaps and end just before the first pattern-offset table at $8C15.<br>Every pattern uses only pitches 1-12, so $83D0 is not a NotePeriods entry. Instrument ids run 0-30, so InstrumentTable is $8610-$8707, 31 x 8 B.<br>Patterns end at $9C85, right before code at $9C86. This resolves the "2:$83D4 sound data, no direct consumer" residual and corrects the "InstrumentNoiseBytes 1329 bytes" description. |
| L-031 | low | 2:$B65D-$BFFF (2467 B, after the 32-byte champion palette) and F:$FF57-$FFF9 (163 B) are uniform $FF fill. The source says "the rest is read through other offsets", which is unsupported; the recipe fixes it. Disassembly may split region 764 at phys $0B66D. |

Recommendation: run one short disassembly/commentary cycle that applies L-029, L-030 and L-031. These are relabels and region splits/merges with 0 new bytes. Commentary should also update uncertainties.md: remove 0:$90B2-$9180, 2:$83D4 and 2:$B63D+.

After that cycle, the coordinator may accept the next redirect as terminal, unless review rejects a lead or the new labels expose a consumer. The word-attribution pass (wordscan.json) has now been applied to every residual block, and the remaining hits show no pointer-table structure.

## Rejection review (residuals after these leads)
Decisive artifacts re-read: the r03 refs.json, findings.json and runs.json; the r04 idxscan.json, findings.json and runs.json; and this pass's wordscan.json and findings.json.

- **0:$BDE8-$BE17, PieceTableUnreadA/B.**
  - Claim: generic data, role unresolved.
  - Checked: abs, immediate-pair and indexed-base windows (tools/refs_bde8.py and idxscan_bde8.py) found 0 readers. The word hits are coincidental bytes inside unrelated data.
  - Debris consumers use Y = 0..23 at their own bases, so none can reach these tables.
  - Strongest competing interpretation: an earlier debris variant. That is semantic only; the data role is not in question.
  - Verdict: no discriminating test exists. Retain.
- **2:$B059-$B068, eight spare script pointers.**
  - The TitleRevealB loop stops at `cpx #$0E` (14 entries), and no other pointer setup references the area.
  - The wordscan hits are TitleRevealScriptPtrs targets ($B069/$B07A), and those scripts are read.
  - Retain as generic, unread.
- **Remaining small or uniform blocks.** These are 0:$8F84 (colour bytes), 3:$AF42, 4:$A474 (1 KiB of nibbles past the sample length), 5:$8388 (640 zero bytes), 5:$88F4, 6:$8400, 6:$8701 and F:$E9AC/$FF2C.
  - They have no abs, indexed-base or immediate reader (r03, r04).
  - Their word hits (wordscan.json) are scattered single hits inside tile, terrain and velocity data, with no pointer-list structure.
  - The F:$FF2C and 6:$8701 reads are rts dummy reads (r03).
  - The competing code interpretation was already rejected (no flow reaches them, and the content is fill or colour/nibble patterns).
  - Verdict: retain as generic. Further work would be semantic only, with low prospect.
- **LegalScreenTiles reader.** Unchanged: no static reference of any kind, including word attribution (the hits fall inside the nametable itself), and it was unread in every run. A long attract watch remains semantic only and slow. Deferred.
- **22 no-caller static code islands (650 B) and 27 partly executed Static regions.**
  - The premises are unchanged. Word attribution adds no new jsr/jmp/rts-trick target, because the round-5 pointer finds are data pointers into data.
  - Verdict: retain, as reviewed in r03 and r04.
- **0:$9102 extra read and $BE18 reads (r04).** These are runtime oddities with no reader PC available. They do not bear on any classification. No claim.

## Gameplay checklist (analysis/scenarios.json)
The checklist is current. Two entries were added this round: the intro-story read watch and a checklist review.

Visited:
- boot and title; the 1P/2P menus; the intro story and map
- cross-country races at stops 0, 4, 7 and 8; the shop; all five event types
- out-of-cash GAME OVER; the secret stop select
- the natural championship ending (r02)

Not visited:
- an attract demo (none appeared within about 820 idle frames; semantic only)
- a natural truck-1 crash and repair (the static extent is enough)
- a 2P race played to the finish (no residual needs it)

No later-game or ending milestone remains unvisited.
