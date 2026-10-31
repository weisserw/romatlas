# Round 001 current-source commentary complete

The SAME interrupted commentary phase is now complete for the current classified
source. No pending readability range remains. This is a current-source readability
handoff, not whole-ROM classification or terminal editorial/archive acceptance.
Final editorial audit remains pending; it was not run.

## Identities and verification

- ROM SHA256: 6a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b.
- maps/current.json and analysis/state.json SHA256:
  c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e (unchanged).
- Final source/game.asm SHA256:
  7c2412a800002f18dbeae885737e0aefad239220b4e2624dd47a8c43651b2130.
- Fresh commentary.py verify passed byte_identical=true; commentary.py read then
  refreshed commentary/reading-after.asm. See commentary/verification.json.
- Classification unchanged: Code26,229 Data109,079 Image262,144 Unknown126,852;
  75.81% classified. Unknown remains structurally unresolved.

## Scope and readability assessment

All 26,229 classified Code bytes /12,994 instruction scaffold records are reviewed.
All 13,488 full-ROM scaffold records and399 meaningful units have reviewed status,
summaries and source locations. All134 Data records, the CHR Image record and359
Unknown intervals retain the previously reviewed role/boundary explanations. The
exact pending complement is empty in analysis/round-001/commentary-frontier.json.
No scaffold region or valid earlier review credit was removed.

Cont04 completed the last4,861 code bytes /2,330 records in banks26..29, adding102
meaningful units. Reused prior sound, fixed-bank, stage, data and unknown-boundary
review; no mandatory whole-source reread or cosmetic rewrite. Classified meaningful
routines/blocks now have functional names, contextual RAM/table/constant operands
and explanations of inputs, algorithms, outputs and constraints. Per-instruction
region labels remain scaffold where they are not meaningful routine/branch entries.
Unknown backing bytes can have consumer-role aliases without a classification claim.

Useful examples in commentary/source/game.asm:

- Bank26_DrawMenuCursor: row-sensitive layout indices, saved palette, horizontal
  tile-$F3 OAM strips, hidden tail and the interrupted alternate-mode path.
- Bank26_ApplyTitleSplit: clear/set sprite-zero hit waits, thirteen-step delay,
  horizontal scroll write and CHR0/1 switch; explicit no-timeout/precondition.
- Bank27_WriteCompressedMenuImage and Bank29_WriteCompressedMenuImage: independently
  inspected zero-terminated repeat/literal streams, 16-bit pointer carry, direct
  PPU destination, and the $80 literal-count wrap to256.
- Bank28_InitializeScene / Bank28_InitializeSceneActors: view/reference positioning,
  thirteen-byte actor records, parallel consumer tables, completion fields and
  nonzero count assumptions; no guessed table extents.
- Bank28_AppendMenuCompositeImage: shared overview/password render producer with
  five-byte records, coordinate flags and full-list omission.
- Bank29_BuildPassword / EncodePasswordLetters / DecodePassword /
  ConvertPasswordLetters: 24 packed bits, reversed field order, three-bit XOR check,
  mode trials, positional alphabet offsets, inclusive bounds and destructive work
  buffer. The editor's player-ready, input-repeat and submit mechanics are explained.

Connected corrections include the stale PrepareRendering uncertainty; using the
palette-copy destination pointer left by CopyPaletteBlockToSlot; contextual menu
coordinate/sort-key scratch, initial reference Y offset, blank-letter count and
password range/check aliases; and removing new unused misbanked cursor equates.
Special-key cursor stores are followed by unknown three-byte tails, so their
control flow and accumulator values are not carried into the separate letter store.
Shared call addresses alone were never used to establish bank-specific semantics.

## Unresolved questions and bounded redirect recipes

Detailed supporting ranges/recipes are in commentary/uncertainties.md. These are
future structural/meaning investigations; none blocks current-source readability.

1. Title four/twelve-direction gates: observed mechanics and carry contracts are
   explained, but their unknown tails' option/mode effects remain unknown. Bank26
   $9C27 lies at physical220215..220248; bank28 $9DC0 is classified. Replay one
   existing title route/pattern and inspect writes immediately after the gate with
   the effective pair preserved; compare only exact supported ranges/dependencies.
2. Password setup/keyboard gaps: bank29 $A176 physical237958..238023, $A331
   238401..238513, $A434 238660..238708; plus three-byte tails $A407..$A40A and
   $A41D..$A420. From password-menu.snapshot use the existing natural entry route,
   then one direction/release and one A/B edit. Bound the trace to that action;
   inspect player/cursor/letter-buffer writes in [28,29,30,31]. No implicit
   delete/blank behavior or unknown-body identity is asserted. Bank27 $A148 lies
   in the much larger Unknown221528..229096 and is not assumed identical.
3. Menu RLE pointers/payloads and scene tables: source names their consumers; exact
   extents remain unknown. One known image's bounded read trace or one scene's
   count/record reads can support localized structural proposals. Keep physical
   storage, CPU pointer aliases and PPU destinations separate.
4. Title waits/panels: bank28 $9D82 physical236946..236965 and bank26 $9BE9 within
   Unknown220126..220156 have classified callers, not known internal contracts.
   Unknown panel $9978 and remaining gate tails stay local uncertainties.
5. Preserve prior fixed/stage uncertainties, particularly unknown $D880, marker
   tails, bank16 row handlers and bank18 $8A5E physical150126..150239. A single
   bounded event read/return-cursor observation remains useful for bank18 with
   effective [18,19,30,31], without treating the unknown payload as already decoded.

## Preservation and next gate

No ROM, toolkit, map/state mutation, redirect execution, final audit, archive,
cleanup, deletion or publication occurred. All restored source/assets, state,
traces and scripts remain local. No worker is active after this handoff.
One-shot commentary-cont04 edit/correction scripts are preserved and MUST NOT be
rerun. commentary-cont04-checkpoint.py only refreshes source locations/frontier and
can be reused. Prior cont02/cont03 scripts and all interruption handoffs remain.

The coordinator can review this completed current-source gate and proceed to its
separate authorized workflow. Reserve editorial_audit and commentary.py audit for
terminal editorial/archive acceptance, as instructed.
