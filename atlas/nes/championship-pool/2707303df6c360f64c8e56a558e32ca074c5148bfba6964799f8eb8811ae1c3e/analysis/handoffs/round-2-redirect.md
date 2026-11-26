# Round 2 - redirect handoff

Status: complete (review pass). No classification correction is needed. Recommendation: **(ii) proceed to final
acceptance (editorial audit)**. Two low-priority non-structural leads are ready: L-009 (map metadata) and L-010
(commentary). Both can be folded into final acceptance.

- Attempt dir: analysis/redirect/r2-q1/. It contains notes.md, the tools (xref.py, idxscan.py, datacheck.py,
  run_q.py, src.py) and their evidence JSON and PNG files.
- Input checkpoint: pass-002-final (byte_identical).
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
- Not modified: the map, analysis/state.json, the commentary source, input/game.nes and the toolkit. The state.json
  timestamp predates this attempt.

## Methods
1. **xref.py → xref-lowconf.json.** For each of the 56 code regions with confidence below 0.75 (878 bytes), it
   collects every static reference that lands inside the region:
   - absolute and indexed operands of all classified instructions (bank-aware);
   - branch, jsr and jmp targets;
   - pairs of `#imm` loads, in both byte orders;
   - every unaligned word inside Data regions.
2. **idxscan.py → idxscan.json.** Finds every `abs,X` / `abs,Y` read whose base is in Data and whose 256-byte reach
   covers a low-confidence region. Each consumer's index domain was then checked by hand.
3. **datacheck.py → datacheck.json.** Looks for code evidence inside data:
   - control-flow targets that land in Data or Padding;
   - observed instruction starts (from all saved traces) that fall in Data or Padding;
   - references to $8A76-$8AFF.

   It also linearly decodes the 40 Data regions in code banks.
4. **Observed-provenance audit.** Checked every instruction start of all 558 Observed code regions (23,556 bytes)
   against the saved traces, and searched the evidence texts for natural-reachability claims.

## Review results per family (decisive evidence: review-families.json, xref-lowconf.json, idxscan.json, datacheck.json)

| Family | Regions / bytes | Verdict | Decisive evidence |
|---|---|---|---|
| Round-1 exact gap fills (0.7) | 20 / 768 B | keep | Each starts after an RTS or JMP (974 after a returning jsr) and ends in RTS/JMP or known code. No data reader lands inside. Neighbouring tables have bounded domains. |
| L-006 dead JMP / else-skip / rts;rts / bank-5 stubs / title arms | 32 / 84 B (incl. $8A48 rts) | keep | No references. Each lies between an unconditional JMP/RTS and the next label. Repeated compiler shapes match live code in the same bank. |
| $DE1C pha prologue | 1 B | keep (strengthened) | VramBufFlushNow pushes 2 bytes and pulls 3. Both branch-in users push A first, so a direct entry needs exactly one preceding pha. |
| $D9B0 txa prologue | 1 B | keep (weakest) | See the rejection review below. |
| $FB32-$FB3F trampoline | 14 B | keep | Same trampoline layout as the verified ones. Its jsr target $9CF7 is exactly the start of gap-fill routine 969, which begins after an RTS. The two boundaries agree independently. |
| bank 3 $97B1-$97BA wait loop | 10 B | keep | The $97A3 word table is indexed by ScoreboardLayout*2. The values in $977D/$9790 are at most 6, so the maximum offset is 13 and the table ends at $97B0. The beq self-target is $97B1. |
| bank 3 $8A76-$8AFF generic data | 138 B | keep generic data | Width/height header (16,8), then 128 tile bytes, then 8 attribute bytes (16x8 tiles = 4x2 attribute cells). No operand, data word or screen script references $8A76. No flow into it. |
| Other data in code banks | 40 regions | keep | No jsr/jmp/branch target or observed fetch lands in any Data/Padding region. Only $9790 decodes as clean code, and it is a table with a known consumer. |

Counts: **56/56 low-confidence code regions kept, 0 boundary fixes, 0 reverted to unknown.** All generic data blocks
in code banks are kept.

Provenance (c):
- All Observed code is backed by traces.
- All 31 regions observed only in artificial-state traces say so in their evidence.
- Two wording nits:
  - The S14 evidence says "then natural interview/menus". It means unpatched execution after an artificial
    shortcut.
  - The opponent-message evidence says "natural reachability is likely". This is a labelled judgement.

### Stale metadata (L-009, no byte or kind change)
- **Region 989, $F9A9-$FA3E (DrawBigNumber).**
  - Five static callers exist in bank 3, at $96E8, $970B, $972E, $974C and $9772.
  - 74 of its 77 instruction starts are observed in the artificial S13 trace.
  - Its evidence still says "without a known caller".
- **Regions 974 and 1112, bank 5 $8A3D-$8A48.**
  - These bytes are the fall-through of AimAtCalledBall after a returning `jsr SelectBank5_Temp`.
  - They are therefore statically reached, which supports confidence 0.9.
  - Their evidence says "no caller".
- **Region 1122 ($DE1C).** Add the stack-contract support to its evidence.
- **S14 evidence wording.** Correct it as noted above.

## Compact rejection/acceptance review (evidence-policy.md) for the weakest kept claims
1. **$D9B0, 1 byte, txa: Static code at 0.5.**
   - Evidence: it sits between LoadPalette's rts and UploadPalette.
   - It rules out a data role: no reference reaches it. The nearest table (PaletteTable, 7 words, script argument
     domain 0..6) ends 34 bytes earlier.
   - Constraints considered together:
     - $8A is not a fill value.
     - The fall-through target's contract takes its argument in A, so txa gives a "send X" entry.
     - The same one-byte-prologue idiom is independently proven at $DE1C.
   - Strongest competitor: a stray data or padding byte. Nothing supports it (no consumer, no fill run).
   - Decision: keep. The ambiguity is one byte with no evidence for the alternative, so it is not material.
   - Revisit only if a data consumer is found.
2. **$97B1-$97BA and $FB32-$FB3F (no caller).**
   - Each has a self-consistent control and stack contract plus an independently fixed boundary (the table domain
     ends at $97B0; the trampoline target is the start of region 969).
   - Competitor: data tail of the preceding table. This is excluded by the bounded index domain at $97B1. At $FB32 no
     table is adjacent.
   - Decision: keep. Reachability is still unknown and is not claimed.
3. **Gap-fill routines with no caller (e.g. DisableNmi/EnableNmi, DrawTileRectDirect/Mapped, Div32Alt,
   VectorLengthCalc).**
   - Exact tiling between known code boundaries, official opcodes only, coherent register/RAM contracts that mirror
     live routines, no data readers.
   - Competitor: data embedded between routines. No consumer exists and no table domain reaches them.
   - Decision: keep.
4. **$8A76-$8AFF generic data.**
   - The decision rests on non-code content (tile/attribute structure; the linear decode is non-code) and on the
     absence of any flow into it.
   - Competitor: code. There is no flow, and the decode is not code-like.
   - Decision: keep. The specific "tile rectangle" role remains unclaimed.

## Ready leads (analysis/leads.json)
- **L-009** (ready, low). Metadata refresh for regions 989, 974, 1112 and 1122, plus the S14 wording.
  - Recipe: run map.annotate on the same ids, offsets, lengths and kinds; then map.validate and a checkpoint.
  - Byte identity is unaffected. No disassembly round is required; this can be done at final acceptance.
- **L-010** (ready, low). Commentary findings, listed below.
- No lead needs a disassembly round. L-008 (bank-6 script labels) was already applied by commentary in round 2;
  its status is left as recorded.

## Findings for commentary (L-010; details in r2-q1/notes.md)
1. **NameCharMap ($CD43).**
   - It is the character map of window frame style 0: WindowFrameTable record 0, bytes +2/+3 hold $CD43.
   - RunMenu's full-screen paths load the literals #$43/#$CD, which should be rendered as NameCharMap.
   - Observed indices are $20-$6A, so the map body is $CD63-$CDAD.
   - The real BigFontBottom is 26 bytes ($CD49-$CD62). The 891-byte block therefore holds BigFontBottom, then
     NameCharMap's body, then further tables.
   - The 1218-byte AsciiTileTable block is likewise read in several separate sub-runs.
2. **Credits pages.**
   - Both credit scripts start with menu op $07, which sets the map to NameCharMap and the tile offset to $80.
   - The entry values (page 1: Tmp08=0, DstPtr=$0000; page 2: Tmp08=0, DstPtr=$CD43) are therefore irrelevant.
3. **PortraitFrame values.**
   - Frame 1 is shown when the opponent lost a game or match: "STALKS OFF", and the random "CONGRATULATES / LUCKY /
     VOWS REVENGE" comments.
   - Frame 2 is shown when the opponent won: "RAN THE TABLE" and "WINS THE MATCH".
   - This comes from the message contexts. The pictures were not visually confirmed, because the $DD=2 shortcut
     bypasses EndOfMatch (S18).
4. **RandomMessageTable entry 0** ("SO, YOU THINK YOU'RE GOOD...") is never chosen by PickRandomComment: the index is
   Random & 3, and 0 is replaced by 1.
5. **Misnamed delay path.** DelayThenEnding → WaitFramesThenEnding → EndingScreenLoop ($B9AB) is really "run
   GameFrame for N frames, then return", because $B9AB is a bare RTS. $B9AC-$B9C0 is a disabled countdown loop.
6. **RunMenu reads of $2000/$2001.** On hardware these return the PPU I/O latch (the last value written to the PPU),
   not CPU open bus. Not traced.
7. **Not investigated:** what the scoreboard rolling wheels count. It has no structural impact.

## Gameplay milestones (analysis/scenarios.json)
| Scenario | Result | Kind |
|---|---|---|
| S15 World Championship unlock | Writing $EE=1 at the mode menu leads, through WELCOME regional, to TournamentRound 5 and the BCA World Championship welcome. Confirms the unlock consumer. | artificial |
| S16 credits entry state | See finding 2 above. | natural, traced |
| S17 char-map read trace | See finding 1 above. | natural menus plus artificial ending |
| S18 EndOfMatch portraits | The shortcut skips the target screen. | attempted |

M-challenge-play is still deferred. A natural Speed Pool clear under 141 needs many accurate manual shots, and it has
no structural value now that every byte is classified.

Already visited and not repeated: boot, menus, Freestyle, Party, round 5, ending.

## Deferred, with reasons
- Natural reachability of the opponent-message and intermission routes. This needs a natural won or lost game under
  the manual call-shot contract. It is reachability only, so it is low value.
- Visual confirmation of portrait frames 1 and 2. This needs EndOfMatch, which the shortcut cannot reach. The meaning
  is already answered statically.
- Scoreboard wheels. Commentary only; needs a natural intermission.
- A runtime data-read sweep of hot code ranges. TK-003 (Read events include instruction fetches) makes it
  impractical, and the static xref plus the bounded-domain proofs already cover it.

## Disk / processes
- Scratch /tmp/ra-2707-r2q1 was removed after copying 2 PNGs into the attempt dir.
- The serve process (2157130) and its rpcd daemon (2157128) were stopped by their own PIDs.
- About 2.5 GB free.
