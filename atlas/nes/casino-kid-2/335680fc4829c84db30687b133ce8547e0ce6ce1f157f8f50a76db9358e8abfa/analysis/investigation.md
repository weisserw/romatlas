# Investigation ledger

## Round 1 / disassembly (checkpoint pass-001)

Method (reproducible: analysis/tools/round01/, run with the bridge in that folder;
scripts hard-code the scratch path /root/rascratch and the workspace ROM path):
1. Traced boot->title and gameplay (scenarios S01-S08) with aggregate tracing; dumped
   observed contexts/edges/coverage (traces/round01/*.json.gz).
2. Local recursive descent (`analyze.py`, `run_an.py`) from: every observed opcode with its
   observed PRG context; reset/NMI/IRQ vectors ($C000/$EA74/$EA82); fixed-bank API jump table
   $C010-$C120 (consecutive `jmp abs`); each bank's entry jump table ($8010.. banks 3-6,
   $8012/$8015 bank 2); inline dispatch tables (below).
   - Bank contract: UxROM switch routine $E30E (alias jmp $C025): `txa/pha; lda $14; and #7;
     tax; sta $E31B,x` (bus-conflict table $E31B = 00..07). Fixed-bank paths track
     `lda #n; sta $14; jsr $E30E` to resolve later `jsr $80xx`; any other JSR clears the
     known bank. Code in switchable bank N resolves $8000-$BFFF to bank N (assumes callees
     return with the caller's bank mapped).
   - Inline-parameter callees (callee pops return address and resumes past k bytes):
     $D91F/$D961 (aliases $C0C4/$C0C7) k=3 (jmp ($78) after +4); $DAA5,$DAC2,$DADF,$DB4C,$DBDB
     (aliases $C0CD-$C0D9) k=5 via $DC03 which adds 5 to the stacked return address;
     bank 3 $91BA k=2, $91D8 k=4; bank 5 $8B62 k=2, $8B80 k=4 (byte-identical copies);
     generic detector for `pla/sta zp/pla/sta zp+1 ... adc #k ... rts|jmp (zp)`.
   - Dispatchers $E8DD/$E8F5 (aliases $C070/$C073) and routines that `pla;pla` without
     returning: fallthrough not followed; word tables after `jsr $C070/$C073` parsed, bounded by
     first out-of-bank word, observed code bytes, or reaching the lowest target. 13 tables, 79 entries.
   - Rejection rules: unobserved instruction rejected if unofficial/BRK, overlaps an observed
     instruction or another static start, overlaps a code-referenced data address or inline
     parameter, or (transitively) falls/branches/calls into a rejected node. 85 nodes rejected.
3. Gap passes (`build_data.py`, `build_win.py`): header, vectors, bank-start stubs
   (`jmp $C019` at $8000 of every switchable bank) and zero fill; exact-fill gaps (>=16 bytes
   decoding exactly into official instructions ending in rts/rti/jmp; 4 gaps, e.g. $C208 OAM clear);
   uniform runs >=16 bytes; 32-byte windows with >=35% illegal/BRK opcodes and no JSR/JMP to a
   known entry (>=20% if code reads an address inside) -> generic Data; data tails that decode
   cleanly into following code were trimmed back to unknown.

Results: 87.31% classified (see progress.md). Observed code 19,637 B; static code 17,335 B;
generic data 67,556 B; padding 9,919 B.

Reachability vs structure: Static code regions are structurally inferred (not executed in
round 1). Data regions are generic (role unknown) unless the evidence text names a consumer.

Known weaknesses / review targets for redirect:
- Window rule can mislabel a short unreached code head at the start of a data window that
  follows code (only data tails were trimmed). Low-rate data windows (24-35%) are listed by
  `build_win.py` (e.g. bank 4 $B761 pointer-like words, bank 5 $844D, bank 6 $8C72).
- Dispatch table extents are heuristic (confidence 0.8).
- Bank-2 word at $8010 (= $8018) left as low-confidence data (0.6).

Remaining unknown: 16,641 bytes in 326 ranges; largest: bank 7 $C62E-$CD8D (1888, lead R1-L001),
bank 4 $8DDA-$90D0 (759, R1-L002), bank 5 $9ED8-$A057 (384), bank 6 $A3C1-$A540 (384),
bank 4 $A06D-$A14C (224), bank 6 $A561-$A640 (224), bank 7 $DEFD-$DFD7 (219), bank 4 $B8E1-$B9B3 (211).

Lead outcomes this pass: no prior leads (fresh workspace). Produced R1-L001..R1-L008 (leads.json).
Gameplay obstacles: poker locked ("BEAT AILTON FIRST!"); opponent bankroll variable not found
($07AD is not the displayed bankroll); roulette spin input unknown.

## Round 2 / disassembly (checkpoint pass-002)

Method: scripts in analysis/tools/round02/ (originally run from a deleted scratch dir; paths need
adjusting). r2.py wraps map.annotate (fills only unknown sub-intervals; asserts no overlap); applyc.py
applies a desc.py closure (always-taken-branch rule, inline callees) with Observed provenance for
offsets in traces/*/cov_*.json.gz; entries.py = all branch/jsr/jmp targets from classified code.

Lead outcomes: R1-L009..R1-L020 all **verified** and annotated (details in leads.json); each recipe
re-checked (descriptor/stride/chain walks, closures with zero issues, consumer reads re-found).

Always-taken-branch re-review: with the corrected rule no control-flow target from classified code
lands in any remaining unknown byte (xref.py), so no further transitively rejected routine recovers
beyond R1-L018. Residual review (R2-D01):
- Unreferenced code fragments (Static, conf 0.5-0.6; structure only, reachability unresolved, not
  claimed dead): b4 $B17D-$B180, b5 $A81D-$A820, b6 $87C8-$87CE, b7 $D209-$D219 (each bounded by a
  preceding jmp/rts and joining an established entry exactly), b5 $90E5-$9107 (sibling routine; its
  ldx #$FF/rts tail had been inside data region 2247, now narrowed to the $9108 value table read by
  lda $9108,x), b7 $C1B3 jmp $C1FD (jump to the ending tail of FinalMatch), b2 $BCFF lone rts after
  jmp. Word/rts-pointer search for all of them: none.
- 28 small data pieces: consumer reads (lda tbl,x), immediate pointer pairs passed to $C0A0 sprite
  layouts/$18-$19 VRAM strings, 32-bit constant 100,000,000 (b3 $8FD6, b4 $832B tail), pattern
  continuations ending at established entries.
R2-D02: 89 remaining unknown windows (4,545 B) all lay between Data/Padding (or before an established
entry), with no control target and no fixed-bank API call pattern; signatures (word tables,
game-text, tile runs, music-command bytes) recorded per region; generic data, conf 0.75. Specific
roles (bank-6 music streams, bank-2/5 BuildVramStrings blocks, name lists) remain for redirect.

Remaining unknown: 2 B, bank 4 $BB30-$BB31 (A5 7F between rts and entry $BB32), lead R2-L021.
Weak spots for review: low-confidence fragments above; R2-D02 windows are structural-only data
(code excluded by context, not by a proven consumer).
Gameplay: S11 retried (artificial); $06D2 is not the roulette result (R2-L022). Ending still only
artificial (S12).

## Round 3 / disassembly (pass-003)

Scope: apply ready redirect leads R2-L023..R2-L026 (role/bounds/provenance refinements, no
classification change). Script analysis/tools/round03/apply.py re-verifies every extent claim from
the ROM (asserts) before calling map.remove/map.annotate; log applied.jsonl.gz; summary changes.json.
Per-byte kinds before/after identical (Code/Data/Padding), classified 131,086 / 131,088.

- R2-L023 verified (bank 5, prg_banks [10,11,14,15]). Wheel position table $947D (permutation 0..37);
  round init block $94A3 (80 B copied to $0400 by $8CD8) + five 4-byte items $94F3-$9506 referenced by
  its record words (new finding); board row pointers $952A + cell rows $953C-$960D; class/multiplier
  tables $9EB8/$9ECA; 208-word cell list pointers $9ED1 (18 null) + 158 lists to $A327 (every list
  referenced); eye frames $A88F; opponent name pointers $A897 (adc #$97/#$A8 at $809B) + names
  $A89F-$A8B4 + unreferenced label strings $A8B5-$A8D5; tier money records $A8D6; winning-number list
  pointers $A8F6 + lists $A946-$AA74 (shared byte $AA4B). Extended downstream (new): roulette message
  pointer table $AA75-$AB5C (zpMessageTable = $AA75 + 58*StakeTier at $80C4-$80E7; 4 x 29 words)
  and 114 message strings $AB5D-$B634 tiling exactly to code $B635 (2 targets $AC38/$AF14 are shared
  string tails). The three R1-L020 "text" windows are now inside number-list regions.
- R2-L024 verified (bank 4). $A06D-$A0C0 7 menu strings; $A0C1-$A0EC are poker **hand-name** records
  (pointers to "HIGH CARD".."ROYAL FLUSH" at $A249-$A2DD) — the lead's "chip-label" wording and the
  commentary name B4_DrawChipStack ($9449) look wrong (it draws hand names); $A0ED tier bytes; $A0F1
  4 x 43 message pointers -> exactly the 172 strings $A46C-$AD9B; $A2DE-$A46B and $AD9C-$ADC2 further
  $FF-terminated strings (direct consumers not mapped). Credits: $B761-$B7C4 50-word line pointer
  table (read at $B57E/$B6C6; new), staff lines $B7C5-$B8ED (21 incl. empty line $B8ED), banner rows
  $B8EE-$B9B3 (6 x 33 B).
- R2-L025 verified (bank 6). B6_SoundHeaderPtrs $8C8A (28 words) == walker headers; headers tile
  $8CC2-$8DE6; 113 distinct stream entries, union exactly $8DE7-$9F6A, no issues. Streams overlap
  (later entries inside earlier streams), so the map uses one region per entry, cut at the next
  entry; each region notes its refs and where its stream actually stops. $8DE7 is a 1-byte $FF
  silent stream shared by 39 tracks. Generic remainders kept: $8C72-$8C89, $9F6B-$9FF1.
- R2-L026 verified: 7,037 opcode addresses, 0 in non-code; 167 Static code regions hit. 104 fully
  executed regions -> Observed in place; 63 partially executed regions split into executed runs
  (Observed) and unexecuted runs (original Static provenance/evidence). Observed code bytes 19,759 ->
  24,315. Caveat recorded in each region: artificial RAM-patched scenarios (S12-S14), not natural
  reachability.
- No gameplay this pass: ending already reached (S13); no open question needed new runtime evidence.
- R2-L021 ($BB30-$BB31) unchanged: no new evidence; rejection review in round-02-redirect.md stands.

## Round 4 (disassembly, checkpoint pass-004)

- **R3-L027 independent review: accepted.** Claim: physical 80704-80705 (bank 4 $BB30, ctx [8,9,14,15]) is the
  official instruction `lda zpLocal3` ($A5 $7F). The claim is structural only. It sits between rts $BB2F and the
  established entry $BB32 (B4_TravelPlaneFrame, jsr from $BAA4 and $BB2D), whose first instruction `lda #$01`
  overwrites A and the flags.
  - My own checks: no 2F/30/31 BB byte pair occurs anywhere in the 131,088-byte image, so there is no word
    pointer, jsr/jmp operand or rts-pointer to $BB30. No #$BB immediate in bank 4/7 has a nearby #$2F/#$30/#$31
    partner. The dispatcher at $BA81 loads zpLocal3 itself, and the $BB32 body reads, ORs and stores zpLocal3.
  - I re-read the redirect's gap census (analysis/redirect/r03-a1/gapreaders.txt). Every other reader-less gap
    has positive non-code evidence. The dead-load precedent is the executed bank5 $8883.
  - Strongest competitor: an orphan 2-byte data pair. Nothing supports it (no reader, format, pattern or
    precedent). The code reading has operand coherence and exact tiling into an entry.
  - Decision: per evidence-policy.md this is not materially ambiguous, but the evidence is weak, so it is
    classified as Static code at confidence 0.5. The evidence text states that no caller is known and
    reachability is unresolved (it is not claimed dead).
  - Reopen it if a reader of $BB30/$BB31 or data evidence ever appears.
- **R3-L028 verified.** Regions 2190+2783 were merged into one Data region $9DB2-$9DB5 (suit permutation
  0,2,1,3; Static 0.9). The bounds are fixed by the base operand and the observed entry $9DB6, independent of
  the index domain.
  - Side observation (not pursued): in B4_CompareHighCards the second unrolled half stores the card to $80
    ($9D5B), but the inline DivideMulti parameters still name $8A/$89. The second lookup index is therefore the
    previous quotient divided by the previous remainder, which may divide by zero. The result is discarded and
    the extent is unaffected. Commentary may note this as a possible original bug after checking the
    Api_DivideMulti contract.
- **R3-L029 verified.** Region 2245 was split into $8798-$879B "YOU"+$FF and $879C-$87CB, 8 x 6-byte panel-label
  records [ptr, col?, row?, len, $01].
  - The consumer is $8734: an 8-bit mask loop that computes adc #$9C/#$87 + 6n and copies y=5..0 to zpWork0..5.
  - Record pointers: $06E7, $A8BB, $A8B5, $8798, $A8BB, $A8C2, $A8C7, $A8CE. The length bytes match the string
    lengths. Records 0 and 3 are measured at run time.
  - The $A8B5-$A8D5 strings (5) are exactly the record targets. Region 2890's evidence was updated and its
    confidence raised 0.8 -> 0.9.
- Script with assertions: analysis/tools/round04/apply.py; log analysis/tools/round04/applied.jsonl.
  No gameplay this pass.

## Round 5 (disassembly, checkpoint pass-005)

All four ready redirect leads were re-checked from the ROM with assertions in analysis/tools/round05/apply.py
(log applied.jsonl, summary changes.json) before annotating. The result is 0 classification-byte change and 0 kind
change. Structural evidence and reachability stay separate: the new data regions are Static; only R4-L033 changes
provenance.
- **R4-L030 verified, with a boundary correction.** Bank 3 $9270-$9974 (ctx [6,7,14,15]).
  - $9270 is 4 name pointers. The consumers are the B3_SetupTable loads at $8122/$8127 (`lda $9270,y`), copied to $07C2.
  - The 4 names tile **$9278-$9290**. The redirect proposal's $9291 end was off by one: $9291 starts "MONEY".
  - $9291-$92ED is 5 label/menu strings. "MONEY" is unreferenced (round-4 commentary already noted it has no
    record). $9297, $929C, $92CD and $92D5 are the pointer words of the panel-label records at $90C7/$90CD/$90D3/$90D9.
    The $9291 words found elsewhere in bank 3 (at $B0E3, $B5A7, $B619, $BA4F) are tile-sequence bytes inside data,
    not references.
  - $92EE-$9335 is 4 x 18-byte tier records: lda #$12 multiply at $80E2, and the copy loop `cpx #$12` at $80FD.
  - $9336-$9465 is 4 x 38 message pointers. The evidence: `adc #$36` at $816E, `lda #$93` at $8173, and stride
    `lda #$26`/`asl` at $8153.
  - $9466-$9974 is 56 strings tiling exactly to code $9975.
  - 35 phase-1 windows were replaced by 6 regions (ids 3225-3230).
- **R4-L031 verified.** B2_AttractSetup stores zpLocalF = $46 (70 bytes = 35 words) for the list at $B58F and $1E (30
  bytes = 15 words) for the list at $B6D1. This bounds each list independently of the string tiling.
  - The strings tile $B323-$B58E and $B5D5-$B6D0 exactly.
  - Ids 3231-3234 are new. Region 2054 was narrowed to $B6EF-$B702: it is the base word read at $B2BB by
    B2_AttractDrawScene, and its layout is still unresolved.
- **R4-L032 verified.** Bank 5 $BAEA-$BC21:
  - $BAEA: 6x6 block, 38 bytes (`lda #$EA`/`#$BA` at $B96F).
  - $BB10: 3 words (read at $B839/$B83E). Its blocks are at $BB16-$BB59.
  - $BB5A: 3 words (`lda #$5A`/`#$BB` at $B827).
  - $BB60: sub-lists of 2+3+4 words, which match the zpLocal4 counts.
  - $BB72-$BC21: 9 blocks, ending at padding $BC22.
  - Ids 3235-3240 are new.
- **R4-L033 verified.** traces/redirect-r04/cov_gameover.json.gz has 0 opcodes in non-code. It hit 42 Static code regions
  (656 opcodes):
  - 36 regions were upgraded whole to Observed (conf 1.0).
  - 6 regions were split into executed (Observed) and unexecuted (original Static evidence) instruction runs:
    b5 $8C55, $A328, $A36E, $A381 and b7 $C1C8, $C292 (new ids 3241-3256).
  - Every evidence string carries the artificial-scenario caveat (S16 money/number RAM patch): this is not proof of
    natural reachability.
  - Observed code went from 24,315 to 25,671 B.
- No gameplay this pass. The natural ending (S13) and game over (S16) were already visited, and no lead needed new
  runtime evidence. scenarios.json is unchanged.
- Still unresolved (unchanged): the internal layout of $B6EF-$B702; no reader for "MONEY" $9291; the low-confidence
  fragments, including $BB30 (no caller); the meanings of the generic R2-D02 windows.

## Round 6 (disassembly, pass-006)

Boundary/role batch for the round-5 redirect leads; map edges and kinds only (0 classification-byte change, 0 code change).
Script with ROM-derived assertions: analysis/tools/round06/apply.py (dry mode simulates on a copy and checks invariants:
no overlaps, identical classified byte set, identical Code regions, exactly the 65 padfix Padding regions = 2,973 B
become Data). Log/changes: applied.jsonl, changes.json, new-regions.txt (id -> range) beside it.

- **R5-L034 verified.** B2_AttractDrawScene reads lda $B6EF..$B6F2,x (x = scene*4) and lda $BACF/$BAD0,x (x = scene*2).
  The 8 BuildVramStrings blocks [w $14, h 6] tile $B6FF-$BACE exactly and are exactly the 8 pointer targets (scene 1
  upper/lower, 0, 2, 3 in address order). Attribute rows [22, 1] at $BAD7/$BAEF/$BB07 (scenes 1, 0, 2) and $BB4B
  (scene 3, ends at $BB62 before code $BB63). $BB1F-$BB4A (44 B) has no pointer and no valid header: kept generic data
  (conf 0.7; role unresolved). 21 windows (2054, 2055-2064, 2796-2805) -> 13 regions 3257-3269 (one per picture block).
- **R5-L035 verified (with the optional re-split).** Graphics-set extents recomputed from the ROM descriptor tables
  (bank 0 $8010 x 28, bank 1 $8010 x 8, bank 2 $8018 x 9 via word $8010; count 0 = 256; 8 B/tile for sets < $0A else 16)
  and asserted equal to every GfxSet declaration in game.asm. 45 sets give 39 distinct extents that tile each bank's
  graphics area contiguously; plus NametablePicture0-5 (LoadNametableFromBank fixed 1024-B copy from $A4+4n:$80).
  Each extent is now exactly one Data region (Static, 0.95; ids 3283-3304, 3306-3328). 344 phase-1 windows removed;
  regions straddling the outer edges were narrowed to exactly the descriptor tables (1877 = b0 $8010-$809B,
  1926 = b1 $8010-$8037). All 65 Padding regions inside (2,973 B of uniform $00/$FF runs = blank tile rows/cells
  uploaded to CHR-RAM/nametables) are now Data. The other 61 Padding regions (bank tails, bank-6 $A14D-$BDE7 series,
  bank-5 $85A8/$85D2) are unchanged: no consumer contains them.
- **R5-L036 verified.**
  - Bank 2: $B0FD-$B13A = palette-block pointer table + three 18-B 16x1 blocks + B2_MenuCursorXTable (2 B, index 0/1;
    the old evidence's "4 bytes" was wrong). B2_TitleMetasprite $B13B-$B156: lda #$3B / lda #$B1 at $AFF8/$AFFC, header
    word +6 = $B147, 4x4 map to $B156 before code $B157 (ids 3270-3271).
  - Bank 6: B6_InstrumentPtrs $9F6B-$9F7E (10 ascending words, first $9F7F); B6_InstrumentData $9F7F-$9FFF (also read
    via lda $9FEE,x at $86DB/$8813/$8821/$8828). $A000 starts a table of 22 ascending words ($A02C.., step 12), which
    supports the $9FFF end; region 2861 narrowed to $A000-$A031 (ids 3272-3273).
  - Fixed bank: VramStructLists $CE2E-$CE55 (2 x 10 words) + ChipPanelVramStructs $CE56-$CF51 (walking the
    [lo, hi, n, n bytes, $00 $FF] structs visits exactly the 20 list targets; ids 3274-3275). RouletteSpriteSetPtrs
    $ECDE-$ECE7, RouletteSpriteSets $ECE8-$ED05 (5 x 5 B, = the 5 targets), FadeMaskTable $ED06-$ED0D (cmp/and $ED06,y at
    $EDA7/$EDAF; ids 3276-3278).
  - DPCM (DMC table bank 6 $8C72, addr $C000+64A, length 16L+1): $FC00-$FC30 sample 0 (49 B); $FC31-$FC3F gap to the
    next 64-B-aligned address (generic, 0.7); $FC40-$FDF0 = union of record 1 ($FC40-$FDE0, 417 B) and record 5
    ($FD40-$FDF0, 177 B). Correction: record 5 ends one byte past DpcmSample1, so the commentary's "lies inside" is
    wrong; one union region avoids cutting either sample. Region 2456 ($AA run) narrowed to $FDF1-$FDFF (ids 3279-3281).
    Records 6-11 of the table decode to out-of-range lengths/addresses (not samples; SndSampleNumber < 12 only bounds
    the index), unchanged.
- Regions 2,946 -> 2,636. No gameplay this pass; scenarios.json unchanged. No new leads.
- Still unresolved (unchanged): $BB1F-$BB4A role; "MONEY" $9291 reader; $BB30 caller; meanings of generic windows;
  whether DMC records 5+ are ever selected.

## Round 7 (disassembly, pass-007)

Boundary/role batch for the round-6 redirect leads R6-L037..R6-L040; map edges and kinds only (0 classification-byte
change, 0 code change). Each lead was re-verified independently before applying:
- analysis/tools/round07/refs.py decodes every map Code region and lists the absolute operands and immediate lo/hi pairs
  for a base;
- analysis/tools/round07/apply.py asserts each consumer reference (via refs.py) and every pointer/record/extent bound
  from the ROM, then annotates. Its --dry mode checks: no overlaps, identical classified byte set, identical Code
  regions, and no Data->Padding.
- Logs: applied.jsonl, changes.json, new-regions.txt (id -> range).

- **R6-L037 verified.** Fixed-bank consumers found independently:
  - $A000/$A001 at $F018/$F01D, and #$4D/#$A1 (zpArg2/3) at $F022;
  - $A1FD/$A1FE at $F266/$F26B, $A211/$A212 at $F25C/$F261, $A225/$A226 at $F382/$F38B;
  - #$3B/#$A2 at $F4C9/$F684 and #$61/#$A2 at $F4BE/$F679;
  - $A287 $F54B, $A315 $F573, $A48B $F711, $A513 $F74C, $A645 at $F3DB/$F443/$F665, $BDBB $F5A0.

  MapWorldMapBank sets zpBankRequest/zpBankHome = BANK_SOUND (6) before these run. That matches the r06-a1 read trace
  context [12,13,14,15].

  The ROM checks all pass:
  - 22 marker pointers at step 12, and the marker tiles end at $A14C;
  - MarkerAttrs values are all in {0..3};
  - the 6x6 portrait layouts hold tiles $B0-$D1 and $D2-$F3;
  - the four pointer tables each end at their first target, with records of [w,h] stride 14/15;
  - the portrait CHR pointers step $220 and end at $BDBB;
  - $BDE7-$BFF9 is all zero.

  $A000-$BDE6 is now 30 Data/Static regions (conf 0.9): 19 structures, plus the portrait CHR as 11 x 544 B regions.
  103 old regions were removed (34 Padding, 944 B -> Data). The bank tail stays Padding.
- **R6-L038 verified (all five tables).** BuildMetasprite_StoreAttr/FlipStore read lda (zpArg2),y with Y = tile code.
  Bases are immediate pairs:
  - b5 $A60F at $8CF6/$9D70/$A578;
  - $A70F at $A582;
  - $BAC8 at $B871/$B9D3;
  - fixed $DFD8 at $DDF0;
  - b4 $BD60 at $BC80.

  End bounds:
  - $A60F table: the $A70F base;
  - $A70F table: observed code $A7FF;
  - $BAC8 table: the anchored frame box $BAEA;
  - $DFD8 table: observed code $E0D8, which is exactly base+255;
  - $BD60 table: the last statically read cell, $BDDF (the $BD09 headers use tile codes $3E and $70-$7F).

  **$DFD8 was accepted at conf 0.8 on its own merits.** The extent bound is exact for the full 8-bit domain. All 256
  values are in {0..3}, in tile-group-aligned runs. The bytes were already non-code Data/Padding, so only the
  fill-vs-cell role changes. Its read cells are still not derived statically.

  286 B Padding->Data; new ids 3359-3363. b4 $BDE0+ stays Padding.
- **R6-L039 verified.**
  - Readers: lda $844D,x / $844E,x at $82CC/$82D1, and lda $845F / $845F,y at $82E0/$82E7. B5_BuildMessageRow reads
    [rows, width, tiles].
  - The 9-word table ends at its first target, $845F. The lists and blocks decode contiguously to $86BC, before observed
    code $86BD.
  - $85A6/$85D0 are 4x10 blocks of $FF, pointed from lists $859D and $8657.
  - New regions: the pointer table (3364), plus 8 regions of one list with its blocks (3365-3372). 80 B Padding->Data.
- **R6-L040 verified.**
  - Readers: lda $8BF0,x / $8C31,x at $869F/$86A9. The 65 periods decrease strictly, and hi bytes 37-64 are zero. The
    DMC table follows at $8C72.
  - New region 3373 covers $8C31-$8C71. Region 2317 was removed (28 B) and 2316 narrowed to $8BDC-$8C30.
- Regions 2,636 -> 2,534 (Padding regions 61 -> 18). No gameplay was needed, since no lead required runtime evidence,
  so scenarios.json is unchanged. No new leads.
- Still unresolved (unchanged):
  - the meanings of the world-map PlaneTableA/B fields;
  - the $BB1F-$BB4A role, the $BB30 caller and the "MONEY" $9291 reader;
  - whether DMC records 5+ are selected;
  - the read cells of the $DFD8 table.

## Round 8 (disassembly, pass-008)

Provenance and evidence-metadata batch for the round-7 redirect leads. There is 0 classification-byte change, 0 kind
change and 0 bank-context change. The only boundary changes are instruction-run splits of partially executed Code
regions. Script: analysis/tools/round08/apply.py. Its `--dry` mode simulates the batch on a copy and asserts the
invariants: no overlaps, an identical byte set per kind, identical per-byte bank context, and Observed code growing
only inside the traced regions. The live log was identical to the dry log (176 operations). Outputs are in the same
directory: applied.jsonl, changes.json and new-regions.txt.

- **R7-L041 verified.** I re-read the three traces/redirect-r07 coverage files (cov_deal25_ranges, cov_deal40,
  cov_scripted3):
  - 4,431 opcode offsets, none in Data or Padding;
  - every hit is an instruction start of its region (linear decode with the round-2 m6502 lengths);
  - every hit region has bank 4 at $8000 (prg_banks [8,9,14,15]);
  - covalign.py reproduces the redirect list exactly: 53 regions, 1,662 B, 571 opcodes, 38 fully hit.

  Result (same method as R2-L026 and R4-L033):
  - 38 regions were upgraded whole to Observed (conf 1.0).
  - 15 regions were split into Observed and Static instruction runs (71 new ids, 3374-3444): 3046, 308, 309, 313, 314,
    351, 3082, 3090, 3127, 473, 474, 475, 496, 508, 516.
  - Unexecuted runs keep their old Static evidence and confidence, with a round-8 split note.
  - Observed code went from 25,671 to 26,839 B (+1,168 B). 494 B of the split regions stay Static.
  - Every Observed evidence string carries the S17 artificial-scenario caveat: f_pk0.snap with PkrHandCount $0746 =
    $18/$27 and King money patched. This is not natural reachability.
  - Data 2187 B4_ScriptedCards: the evidence now names the reader (lda $984E,x at $97FC) and the S17 observed reads;
    conf 0.8 -> 0.9.
  - The runs were not replayed. The saved coverage was sufficient and the conditions were unchanged.
- **R7-L042 verified (metadata only).** 51 Data regions were re-annotated; 2187 was handled under R7-L041.
  - The new evidence is: the round-7 commentary role sentence(s) for the same physical range, then the consumer
    operands found by a round-8 operand scan of map Code regions (same bank or fixed bank; jsr/jmp excluded), then the
    previous evidence.
  - Kinds and bounds are unchanged. No map labels were added, so commentary labels stay authoritative.
  - Confidence was raised to 0.85 for 40 regions that have a confirmed consumer at the region start. That includes
    fixed-bank LoadTileGraphics:
    - ldx #$10 / #$80 at $C4F1 for the bank-0/1 descriptor tables;
    - lda $8010,x at $C4E3 for the bank-2 pointer;
    - the bank-2 word $8010 = $8018 for GfxDescriptorTable2.
  - Confidence was not raised for these regions:
    - no direct reader: 3268, 2071, 2446, 2192, 2460, 2642;
    - 2189: only its trailing B4_MoodStrength bytes $9A28-$9A2C are read; the first 13 bytes have no reader;
    - 2433: layout unconfirmed;
    - 2457: selected by DMC address byte $F8 in records 2-4;
    - 2066: the operand hits only the contained B2_WipeTiles;
    - 2755: already 0.85.
- No gameplay this pass. No lead needed fresh runtime evidence, and S01-S17 already cover the milestones, including the
  ending (S13), game over (S16) and the final-match scripted deal (S17). scenarios.json is unchanged.
- Still unresolved (unchanged):
  - the meanings of the world-map PlaneTableA/B fields;
  - the $BB1F-$BB4A role, the $BB30 caller and the "MONEY" $9291 reader;
  - whether DMC records 5+ are selected;
  - the read cells of the $DFD8 table;
  - selectors for PkrHandFlag 2 and 3: no classified code stores them.

## Round 9 (disassembly, pass-009)

Metadata-only pass for redirect lead R8-L043. There are 0 classification-byte, 0 kind, 0 boundary, 0 provenance and
0 bank-context changes. analysis/tools/round09/compare.py confirmed this byte-wise against pass-008: only
`evidence`/`confidence` differ, on exactly the 20 lead ids.

- **Method.** analysis/tools/round09/apply.py re-checks each structure extent directly in input/game.nes. It also runs
  its own operand scan of map Code regions: absolute operands, `lda #lo`/`lda #hi` pairs and `adc #lo`/`adc #hi`
  pairs. Before applying, it asserts:
  - each of the 20 regions is Data, with the proposal's extent and confidence and unmodified pass-008 evidence;
  - each region lies inside its structure extent;
  - no non-Data region lies inside any checked extent.

  The `--dry` log and the live log were identical.
- **Checks** (all true):
  - PaletteSets: 44x16 records start $0F, the slot at $BF40 does not, and consumers at b7 $C46A/$EE67 form
    $BC80+16n.
  - B4_TableFrameStructs: the B4_TableFramePtrs words are $8653/$867F/$86AB/$86D7, with four 44-byte structs ending
    $00 $FF.
  - B5_WheelStrip: 84x5 columns, number tiles >= $80, two rim triples, the last 8 columns equal the first 8, the next
    slot breaks the pattern, and the base imm pair is at b5 $9CC0.
  - B5_ArrivedAtText: [19,1] header; base imm pair at b5 $B65F.
  - CardBlockTable: word k = $C638+37k; read at b7 $C5C0.
  - CardBlocks: 53 headers 5,7.
  - DpcmSampleCopyTail: equals $FD01-$FD6F, and no B6_DpcmSampleTable span reaches $FEC1-$FF2F.
- **Result.** Evidence is now the proposal prefix, the round-9 check, then the previous evidence. Confidence went to
  0.85 for 17 regions. It is unchanged for:
  - 2833 B5_PortraitTileLists (0.75): the list extent rests on pointer selection only;
  - 2434 and 2438 (0.8): these segments contain several sub-tables whose extents were not re-checked.
- **Leads.** R8-L043 verified. R8-L044 was not attempted (commentary prose, for the commentary phase). No new leads.
- **Gameplay.** None. Structural research was judged terminal in r08-a1, and the lead needs no runtime evidence.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). The per-bank counts are unchanged
from pass-008 (table in analysis/progress.md "Round 9 / disassembly").
