# Round 6 redirect handoff (attempt r06-redirect-a1)

Status: COMPLETE. The input checkpoint is pass-006: 100.00% classified, 0 unknown bytes, byte-identical (map sha256 333b439b...).
- Only research and lead records changed. The map, source, ROM, state.json, scenarios.json and toolkit were not touched.
- All work was static. There were no emulator runs.
- Evidence is in analysis/redirect/r06-redirect-a1/: findings.json (F1-F5), runs.json, splitscan.json, islandscan.json and tools/{common,splitscan,islandscan}.py. Run the tools with `python3 -I tools/X.py $WS OUT.json`.
- Disk: 3.9 GB free. The attempt dir is 60 KB, and there is no scratch.

## Unknown complement
The unknown complement is empty: 0 bytes in every bank (pass-006). The residuals below are classified generic data or Static code. Their only open questions are role and reachability, not code versus data.

## Question tested this round
Is there any reference class not yet scanned that could reach the remaining generic data blocks or the 22 no-caller islands?

The candidate classes were:
- split lo/hi tables
- pointers built by arithmetic
- addresses carried inside decoded byte-code streams

A hit attributed to a real consumer would change a role or reachability note and could produce a lead.

## Findings
- **F1, corrected premise.** The r03 `refs.py` declares a `split` result key but never fills it, so every block reported "split 0" by construction. This means split tables had never actually been scanned.
- **F2, split-table scan (tools/splitscan.py).** The scan starts from the consumer side. It finds pairs of indexed ROM-base loads whose results are stored to N/N+1 or are both pushed. There are 77 such pairs in total.
  - No table entry points into a residual data block.
  - The Static-code hits are all already-accepted dispatch tables: ScriptOpTable to $AFA7; 6:$88F7/$8904 to $8911/$89E3; SfxWriterTable to $F8F7.
  - The F:$F8B2 hits into LegalScreenTiles are a spurious pairing. The scan matched SfxPtrTable+1 (stored to Tmp0F) with SfxWriterTable (stored to Tmp10).
- **F3, the 22 islands (tools/islandscan.py).** The scan looked for immediate lo/hi pairs (in any order, within ±12 instructions) and for data-side split bytes at strides 2-128. Both start and start-1 (rts-dispatch) were tested.
  - There is one immediate hit: 4:$AC7B/$AC82 gives $D536. It is a cross-pairing of two separate text-pointer setups (X=hi, Y=lo) and is coincidental.
  - The data-side hits do not line up with any consumer pair from F2.
- **F4, byte-code streams and arithmetic.**
  - ObjectScripts operands are 1 byte, and op $03 is an offset within the script (the r02 parse covers 1,302 of 1,302 bytes).
  - Sound order lists and patterns use indices and offsets (the r05 parse covers $8708-$9C85 exactly). Story and palette/TitleReveal pointers were attributed in r02 and r05.
  - Hi-byte immediates with adc were checked by r03 hiscan. Base-before-table indexing was checked by r04 idxscan.
  - The only length-arithmetic pointer, the sample at $8000 + $2474, ends exactly at the start of 4:$A474.
  - DMC_ADDR ($4012) is never written.
- **F5.** The map evidence text for region 742 (2:$85C6-$860F) still says "SongHeaders". This is a text-only correction for a future disassembly pass, if one occurs. It is not a ready lead.

## Ready leads
None. leads.json has a new pass entry `redirect-round-06` with an empty result.

## Rejection review (terminal)
The decisive artifacts re-read this round:
- r03 refs.py/refs.json, hiscan.py and findings.json H2
- r04 idxscan.json
- r05 wordscan.json and the round-05 rejection review
- this round's splitscan.json and islandscan.json

The review is per residual class:

- **Generic blocks with no reader:**
  - 0:$8F84
  - 0:$BDE8/$BE00 (PieceTableUnreadA/B)
  - 2:$AC37
  - 2:$B059 (SpareScriptPtrs)
  - 3:$AF42
  - 4:$A474
  - 5:$8388 and $88F4
  - 6:$8400 and $8701
  - F:$E9AC and $FF2C

  - Claim: generic data with an unresolved role.
  - What was ruled out:
    - absolute, immediate-pair, indexed-base and hi-immediate+adc readers (r03, r04)
    - word pointers attributed to their containing structures (r05)
    - consumer split tables (F2, new this round)
    - byte-code operands (F4)
  - Together these rule out every static reference form that 6502 code here uses to address ROM.
  - Strongest competing interpretation: code. This was already rejected, because no flow reaches these blocks and their content is fill, colour, nibble or zero patterns.
  - A reader could still exist through runtime-computed state. No such state is evidenced. B059 stops at `cpx #$0E`, and the debris tables use Y = 0..23 from their own bases.
  - Verdict: retain as generic. Further work is semantic only.
- **LegalScreenTiles/Attrs, 2:$B0E7-$B4E6.**
  - Its format is known.
  - The only hits from all scans, including F2, are spurious or fall inside the nametable itself.
  - It was unread in every run.
  - Competing interpretation: it is read by an attract or power-on path not yet traced. No attract demo appeared within about 820 idle frames (r05). A longer watch would be semantic only.
  - Verdict: retain.
- **StoryTileRecords $9149/$9154 not observed read.**
  - The extent is static: the frame-record pointers and format are consistent (L-029).
  - A longer intro watch would only add reachability, not structure.
  - No claim.
- **22 no-caller Static islands (650 B) and the low-confidence arms or residue (0:$B1E2, 6:$A61E, 0:$8395, F:$D827).**
  - r03 H2 covered word, jsr and jmp references and start-1 words.
  - This round adds immediate pairs and split tables (F2, F3), and both are negative.
  - The code role still rests on idiom and contract evidence, and they are documented as unreached.
  - Competing interpretation: data. This was rejected in r02-r04, and the premises are unchanged.
  - Verdict: retain.
- **Runtime oddities (0:$9102 separator read, $BE18 reads).** No reader PC is available, and they do not bear on classification. No claim.

No revised inference or discriminating test survives. Every reference form has now been checked:
- absolute operands
- immediate pairs and hi-immediate+adc arithmetic
- indexed-base reads
- data words, attributed to their structures
- split tables
- rts-dispatch (start-1)
- byte-code operands

The remaining questions concern meaning or natural reachability, not structure.

## Gameplay checklist
analysis/scenarios.json is current. The natural ending was reached in r02, and no later-game milestone remains unvisited. Nothing was added this round.

## Recommendation
Terminal handoff. Classification is complete (0 unknown), commentary final acceptance is complete, and no ready leads remain.

The only optional item is the region-742 evidence text (F5), for any future map-touching pass.
