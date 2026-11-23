# Round 3 / redirect handoff (attempt r03-a1)

Status: **complete, proposal ready** (not terminal).
Checkpoint: pass-003 (maps/pass-003.json sha256 4f2a310b... == analysis/state.json == maps/current.json).
Classification unchanged here: 131,086 / 131,088; unknown = bank 4 $BB30-$BB31. Proposed: +2 B (R3-L027).
No map/ROM/source/toolkit changes; static analysis only (no runtime, no scratch left).
Research: analysis/redirect/r03-a1/ (findings.json, unknown-ledger.json, gapcensus.jsonl,
gapreaders.txt, deadload.txt, strrefs.txt, tools/gapcensus.py, gapreaders.py, deadload.py, strrefs.py).

## Scope
Coordinator tests (a)-(d) on R2-L021; commentary questions 2 ($9DB2 overrun) and 3 (unreferenced strings).

## Findings
1. **R2-L021 reopened -> R3-L027 (ready).** Corrected premise plus combined constraints on unchanged evidence:
   (a) the $BB32 body is unique ROM-wide. Six routines share its opening, and none has a zp-load prefix
   (negative, neutral); (b) of 30 rts/jmp-terminated non-code gaps, every data gap has a code reader (24) or
   positive non-code evidence (5). Reader-less clean fragments ending on an entry are accepted as Static code
   (7 at 0.55-0.6, $BCFF 0.5). $BB30 fits only the code criterion; (c) $7F = zpLocal3, the travel-scene
   state byte that B4_TravelLoop dispatches on with `lda zpLocal3` and that the following routine itself
   modifies. No A5 7F occurs in any data region; (d) the game has an executed redundant load (bank 5 $8883
   lda #$01 overwritten by lda #$06), and B4_HandAnalysis_9DAD's result is discarded. So "null instruction"
   is no argument against code.
2. **$9DB2 no overrun (R3-L028).** Index = card div 13 (DivideMulti quotient), cards 0..51 from B4_DrawCard
   (only writers of $071C-$0725). Table = 4 bytes $9DB2-$9DB5 (suit permutation 0,2,1,3), ends at entry
   $9DB6. Both callers discard the result.
3. **Panel strings have a reader (R3-L029).** Generic block $8798-$87CB = "YOU"+$FF plus 8 six-byte
   records [ptr, col, row, len, 1] used by B5_PrintMessageValue (really a mask-driven panel-label printer;
   B5_ChoiceMenu mask $80 = YES NO). The pointers cover every B5_PanelLabelStrings start.
4. **B4_LuckyStrings $AD9C-$ADC2: no reader.** Aligned word and immediate-pair search found only misaligned
   or operand coincidences. The role stays string data (negative, no lead).

## Ready leads
| ID | Target | Gain | Recipe |
| R3-L027 | b4 $BB30-$BB31 phys 80704 len 2 | 2 B | review F1, then map.annotate Code/Static conf 0.5, "unreferenced leftover lda zpLocal3; structure only, reachability unresolved" |
| R3-L028 | b4 $9DB2-$9DB5 (regions 2190+2783) | 0 | merge/re-annotate as one 4-entry suit table; commentary rename, note discarded result |
| R3-L029 | b5 $8798-$87CB (2245), $A8B5 (2890) | 0 | split string/records, role notes; rename B5_PrintMessageValue -> B5_PrintPanelLabels |
Expected bytes after R3-L027 = 0 unknown. The tools rerun statically in about 1 s each.

## Deferred / rejected
- Bank5_Data_8268 (per-message label masks?): read at 3 sites with different uses. Roles only, diminishing.
- B4_LuckyStrings consumer: the word, immediate and inline-parameter search is exhausted. A runtime read
  watch would need a scene that prints them, and none is known (missing fact).
- Low-confidence fragments: unchanged (no new reference evidence).

## Rejection review (R3-L027 / former R2-L021)
- Claim: physical 80704-80705 (bank 4 $BB30, ctx [8,9,14,15]) is a 2-byte official instruction
  `lda zpLocal3`, an unreferenced leftover. It is a structural claim only, with no claim of reachability or
  deadness.
- Decisive evidence: gapreaders.txt (census), deadload.txt ($7F uses; the $8883 precedent), and the
  B4_TravelLoop/B4_TravelPlaneFrame source.
- What it rules out: it rules out the old premise that null semantics makes code implausible. It also rules
  out the data reading under this ROM's own precedent, where data gaps always have a reader or positive
  non-code evidence. It does not prove that the bytes ever executed or why they exist.
- Combined constraints: an exact instruction boundary into an established entry, plus the operand being the
  adjacent routine's state variable, plus no data evidence of any kind, plus in-ROM precedent for leftovers.
- Strongest competitor: an orphan 2-byte constant (for example a coordinate pair). Nothing supports it, and
  it would be the only reader-less, formatless gap in the ROM.
- Result: **supported proposal** at low confidence for independent disassembly review. The reviewer may
  reject it. If rejected, the exact residual is these 2 B, and no further test remains without a reader.

## Recommendation
Run **disassembly round 4** (R3-L027 review, R3-L028, R3-L029), then commentary. If R3-L027 is accepted, the
ROM is fully classified, so proceed to final editorial audit/acceptance.
