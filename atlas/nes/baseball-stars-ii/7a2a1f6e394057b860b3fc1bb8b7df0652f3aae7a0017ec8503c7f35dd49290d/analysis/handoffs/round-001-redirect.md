# Round 1 redirect handoff (attempt analysis/redirect/r1-redirect-a) - COMPLETE

Recommendation: **next: disassembly round 2** (one combined proposal batch, leads L014-L020 + L022 ready; L021 open).
Continuation 2026-11-16 (same attempt): post-handoff probes folded in, L022 added, numbers finalized (finalize.py run once).

## Scope and checkpoint
- Input checkpoint: maps/pass-001.json (= maps/current.json), analysis/state.json unchanged, 91.90% classified,
  31,853 unknown bytes. Commentary handoff round-001-commentary.md (complete) accepted as input.
- Resumed attempt r1-redirect-a from its groundwork (ledger.py -> unknown-raw.json exact complement, 796 runs;
  refscan.py -> refs.json); no new baseline. Redirect changed research/lead records only (ROM, map, state, source
  and configured binary untouched; the emulator ran on a /tmp copy of the store, no map.save).
- Leads investigated: L001-L013 (all), plus independent avenues (stale copies, screen/metatile formats, trampolines,
  RTS/JMP() dispatch tables, inline-call misclassification, runtime menus/stadiums).

## Proposal batch (exact ranges: analysis/redirect/r1-redirect-a/proposals.json; residue: remaining.json)
Proposed unknown bytes: **29,037** (Code 17,007, Data 12,030) of 31,853; residual unknown 2,816.
Reproduce: `cd analysis/redirect/r1-redirect-a && python3 write_props.py` (deterministic from maps/pass-001.json).

| Lead | Group (proposals.json ids) | Kind | Unknown bytes | Core evidence |
|---|---|---|---|---|
| L015 | A-code / A-inline / A-jt | Code (+inline/JT data) | 8,309 | closure from mode handlers 16-19/30/31/36/37/40/41, FarCall entries 7/14, $EAC3 step 3, trampolines $FC04/$FC2A/$FFD4, stadium picker, FarCall Y=26 fall-through, JMP($0014) tables bank 24, sound RTS tables bank 0 |
| L014 | contradicted_data_regions (15) | reclassify Data -> Code/inline | (already classified) | phase-1 dense sweep misread JSR $C6D6/$C6FC/$C32D inline params; 6 regions have observed fetches |
| L016 | B-* | Data | 4,730 | stadium field map 75x128 ($8000), attributes $A580, metatiles $AF80, set $B380 (pairs 2/11/13/14) |
| L017 | C-*, H-* | Code (+tails) | 9,127 | live scroller copies banks 23/27/29 (observed, artificial stadium pick); stale identical copies banks 15/17/19/21/25 and bank 11 (= bank 13 code) |
| L018 | D-screens, D-metatiles, F-screentab | Data | 2,108 | 320-byte screen records + 4-page metatile sets from 86 FarCall Y=4/8 sites and stride-$140 ScreenDataTable runs; table ends at $FFD3 |
| L019 | E-* (108 ranges) | Data | 1,357 | ranges read by indexed/absolute loads in classified code (L011/L013/L009) |
| L022 | l022.json (separate optional batch, not in totals above) | Code/Static, no known entry | 569 (+4 Data bytes reclassified) | 4 residual islands that are complete routines with contract-consistent bounds (see Continuation) |
| L020 | G-*, I-* | Data | 3,406 | ASCII text runs, screen-grid islands, pair-3 (never executed) islands, islands between data without references |

Key byte ranges (physical, inclusive):
- Bank 30: $CE09-$CE0E (0x03CE19-0x03CE1E) FarCall entry 7 `lda $002D / jmp $C16A`; $CE52-$CF03 (0x03CE62-0x03CF13)
  FarCall entry 14 (inline-record routine, ends JMP ($0035)); $CE05-$CE08 table (lda $CE05,x at $CDC3).
- Bank 31: $EB2B-$EB34 mode 16 (timer wait, back to 15); $EAC3-$EACA Mode15 table (4 words; word 3 = $EACB) +
  $EACB-$EAD4 code; $FFCF-$FFD3 ScreenDataTable tail.
- Bank 2/3 (pair 1): modes 17 $9A77, 18 $9AC4, 19 $9D2C (+JT $9D32 5 words, in Data region) and their closure
  (~2.3 KiB incl. $9D75 10-word JT, $9DAE, $9FBF, $9FE5, bank 3 $A189 JTs).
- Bank 24/25 (pair 12): modes 30/31 REMOVE TEAM ($8011/$8000, JT $8009), 36/37 TRADE PLAYER ($9491/$9664, JT $966A);
  JMP ($0014) tables $8E57 (3), $8FB8 (2, targets $8FBC/$8FE2), $9060 (3).
- Bank 11 (pair 5): modes 40/41 $B428/$B450 (+JT $B456) with $B421-$B427 table; $A995/$AA1E/$AA7F/$AB11 after FarCall Y=26;
  $B60B (trampoline $FFD4); $B9B2 orphan byte + $B9B3-$BBE1 stale copy of bank 13.
- Bank 14/15 (pair 7): $9BA1 (trampoline $FC04) .. $9BDC picker set-up, bank 15 $A11D picker loop (writes StadiumPalette).
- Bank 9 (pair 4): $B18F (trampoline $FC2A) closure. Bank 0: sound command handlers from $81E4 (16) and $86AE (10).
- Banks 4/5, 22/23, 26/27, 28/29: map/attr/metatile structures ($8000-$B6BF of each pair).
- Banks 16-21: screen records pair 8 $8000-$BABF, pair 9 $8000-$A6BF, metatile sets pair 10 $8400-$AFFF (+ islands).
- Scroller copies: banks 23/27/29 $BA02-$BAFF/$BB50-$BBA5/$BCAD-$BFF8 (live); banks 15/17/19/21 $BAC1-$BAFF/
  $BB50-$BBA5/$BCAD-$BFF8 and bank 25 $BCBE-$BFF8 (stale); $BB48-$BB4F table tail; $BFF9-$BFFF fill.

## Findings (new facts)
1. Phase-1 static closure never continued after FarCall sites with Y not in {4,8}; only Y=26 (entry 13
   FarCopyPlayerRecord, returns via `lda FarCallSavedPair / jmp SelectPrgPair`) left code behind (bank 11, 591 bytes).
2. Phase-1 generic data heuristic misclassified inline-parameter code (14 regions) and the start of a big bank-14 region
   (stadium picker set-up) - L014.
3. Field scroller exists as live copies (pairs 11/13/14) and as stale build residue in banks 15/17/19/21/25; bank 11
   tail is a stale copy of bank 13. L001's far-call premise is rejected (premise checked: $FF28 lies in ScreenDataTable).
4. Data formats: stadium map/attribute/metatile layout from RowTiles/RowAttributes; screen record = 256+64 bytes,
   metatile set = 4 pages, ScreenDataTable = 105 words $FF02-$FFD3 (pair-8 and pair-9 stride-$140 runs).
5. Pair 3 (banks 6/7) is never mapped for execution (no GameModePrgPair/trampoline/JSR use); data only.
6. Main menu: MainMenuModes = [0,28,0,22,34,20,36,32,0,30]; cell 8 = REMOVE TEAM (mode 30), cell 5 = hidden TRADE PLAYER
   (mode 36, not offered with one edited team). StadiumPalette ($0755) is written only by the bank 15 picker from the
   cursor $0756; in COMPUTER PLAY the picker auto-confirms after $80 frames.

## Continuation (2026-11-16): post-handoff L021 probes and static island review
- check_obs.py on r1-observed.json (incl. the 14:39-14:49 time-out / PLAYER SELECT (mode 23) / ACQUIRE (mode 35) probes):
  0 observed starts in residual unknown bytes or Data proposals. Time-out probe (timeout.py, logs timeout.log/menus2.log):
  mode 15 time-out text/menu and return to modes 3/5; the bank 2 $8C2A island (inside time-out code) was not fetched.
- New bounded run (acq.py, artificial $0552 menu-cell writes; scenario S19): RELEASE PLAYER (untraced, frees a roster place)
  then traced ACQUIRE PLAYER ROOKIE through all 10 phases (kind/class/hand, generate, SIGN CONTRACT, bonus allocation,
  welcome) and ACQUIRE STAR (62,000 > 34,000 funds: contract refused). 1,105 new observed starts, all in classified Code or
  L015 proposals (15 in bank 24 $8EE9-$8F06, an A-code JMP ($0014) target - confirms L015); none in residual bytes.
- Static review of the largest residual islands (lib.probe closure from island start, boundary/contract check; l022.json):
  bank 2 $8C2A-$8D1E (245), banks 24/25 $9F83-$A02A (164 unknown + 4 bytes of phase-1 Data region $9F73+20, i.e. entry
  `sty $10 / lda #0 / sta $07`; $9F73-$9F82 remains a 16-byte data table), bank 30 $DC55-$DCAE (90; sets PhaseTimer=8 and
  falls into HomeRunScoreboardAnim, phase-table entry 3) and bank 30 $C2C4-$C309 (70; inline-record PpuBuffer writer with
  PLA/PLA, helper $C2A9 and JMP ($0035), like FarCall entry 14) are complete routines whose closure fills the island exactly,
  official opcodes, no conflicts, bounded by RTS/JMP/classified code. No code reference (JSR/JMP/RTS-1 word/immediate pair) exists;
  the only matching byte pairs ($C2C3/$C2C4, $8C2A) sit inside screen-record/stadium-map/other-pair data, not pointer tables -> packaged as L022 (Code/Static, reachability unknown; reviewer may keep any item unknown).
- Excluded from L022: bank 25 $A040 (starts with PHP right after table $A02B-$A03F of unproven extent: start ambiguous);
  $A120/$AD8C/$B82B/$B8DD, bank 3 $B0B3, bank 1 $A3B4, bank 15 $ADEC, bank 11 $AFCE, bank 9 $B929, bank 10 $8F10, bank 3 $BAE2
  (closure from the start does not cover the island, leaves the island, or hits data/bad paths).

## Verification performed
- Static: every closure path ends in official opcodes, no BRK/unofficial; overlaps with existing code are aligned;
  data conflicts are only the 15 listed L014 regions (each inspected: inline-param calls or table/entry boundaries).
- Runtime (bounded, aggregates-only tracing; r1-observed.json): 2,448 instruction starts in previously unknown bytes,
  all inside proposed Code groups (check_obs.py: 0 observed starts in data proposals or residue). Banks 23/27/29 scroller,
  modes 17-19, stadium picker, REMOVE TEAM, TRADE PLAYER (artificial GameMode=36), RELEASE/VIEW TEAM/ACQUIRE menus.
- Artificial state recorded (scenarios S15-S17): $0756 picker cursor writes, $0552 menu cell writes, $0484=36. These prove
  execution in that scenario only, not natural reachability (except where noted natural: modes 17-19, picker, REMOVE TEAM
  via cursor+A path).

## Unresolved questions / deferrals
- L021 (open): 2,816 residual bytes (2,247 if L022 is accepted); largest code-like islands without any found reference: bank 2 $8C2A (245),
  bank 25 $A040-$A0F8/$A120-$A1D3/$AD8C/$B82B/$B8DD, bank 24 $9F87-$A02A (crosses into bank 25), bank 3 $B0B3, bank 1
  $A3B4 (table-like), bank 15 $ADEC (digit conversion + table), bank 30 $DC55/$C2C4, bank 11 $AFCE. Missing fact: entry
  (no JSR/JMP/word/immediate-pair reference; likely sub-step handlers of unvisited menu/time-out states or dead code).
  Continuation outcome: time-out sub-steps, PLAYER SELECT, ACQUIRE ROOKIE/STAR traced - none of these islands fetched;
  four of them moved to L022 on static grounds. Deferred continuations: league editing (modes 28/29; pair 5, while the
  remaining islands lie in pairs 0/1/4/5/7/12 with no reference - prospect low, menus share already-observed code),
  time-out relief/pinch sub-menus in depth (bank 2 $8C2A has no reference even as RTS-1/immediate pair, so the dispatch
  cannot reach it; diminishing prospect). Remaining missing fact: an entry or an independent extent proof per island.
- Deferred gameplay: 2P mode, ERROR ON, '?' speciality, >2-team league (shared engine, low expected structural gain).
- Stale copies: reviewer may prefer generic data over Code/Static (no reachability) - both are defensible; bytes are
  instruction-aligned copies of classified code at identical CPU origins.

## Rejection review (compact)
- Claim "bank 17 $BCAD is far-call target $BD1D (L001)": decisive evidence = $FF28 is inside ScreenDataTable (consumer
  $CB25 indexes words), static FarCall Y domain {0..12,24,26,30,32,34}; rules out the far-call entry, not code: identity with
  bank 5 observed code (independent constraint) supports Code (stale copy). Outcome: reformulated, proposed in L017.
- Claim "$86AE table has 11 handlers": word 10 ($8976) lands in sound data and $86C2/$86C3 are read as a pointer at $86D1;
  rules out entry 10 only; table = 10 words. Seed removed.
- Bank 31 $F6C2-$F6D9 kept unknown: 12-byte decodable fragment (CLC/LDA/ADC/STA/RTS) + copy of table $F6B6; only a
  word in bank 6 data references $F6C2; strongest competing reading = stale duplicate; insufficient for code or data.
- Bank 15 $ADEC-$AE4B kept unknown: routine-shaped code + digit table $AE1E (read by classified code); no entry for
  $ADEE/$AE28; table part could be split as data in a later pass once the routine entry is known.
- L022 vs L021 split (continuation): islands accepted for proposal only where the start follows a terminator/classified
  boundary or a contract-coherent entry ($9F83) and the closure ends exactly at the next classified region; $A040 and
  partial-closure islands rejected for proposal (start/extent ambiguous), not classified as data.
- Residual code-like islands (L021): not rejected; missing entry only. Generic-data fallback not applied because they
  decode as coherent routines (code-versus-data materially ambiguous).

## Artifacts
- analysis/redirect/r1-redirect-a/: lib.py, proposals.py (sections A-I with evidence strings), run_all.py, write_props.py,
  proposals.json, remaining.json, refs.json, reftab.txt, screen_sites.json, r1-observed.json, covcheck.py, check_obs.py,
  copies.py, islands.py, stad.py, menus.py, rdaemon.py/rr.py (emulator on store copy), frames/*.png, leads.before.json,
  scenarios.before.json, update_leads.py, finalize.py; continuation: acq.py, l022.json, timeout.log, menus2.log,
  frames/acquire_bonus_alloc.png, frames/acquire_star_insufficient.png.
- Snapshot reused/added: scratch/snaps/stadpick_cpu.json (COMPUTER PLAY stadium picker).
- Toolkit: no new issue (TI-001 slowdown again observed: traced menu frames ~1 s each).
