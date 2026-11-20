# Commentary progress

## Round 1 (2026-11-20) — complete current-source pass

Baseline: prepared from checkpoint pass-001 (maps/current.json = maps/pass-001.json,
analysis/state.json); frozen copies in baseline.json / phase1-state.json / before/.

Method: commentary/source/game.asm is produced from the frozen listing before/game.asm
plus the hand-written annotation file commentary/notes/annotations.txt by
analysis/tools/commentary_build.py (labels, symbol substitution with identical values,
`.org`-wrapped data labels, `.word`/`.lobytes` jump tables, comments). Either file may be
edited later; if the source is edited directly, keep annotations.txt in sync or stop
regenerating. review.json was filled by analysis/tools/commentary_review.py.

Scope covered (all classified source):
- Every code region of banks 0, 1, 2, 4, 6, 7, 12, 13, 14, 15 (24,646 bytes, 626 regions):
  every call/jump/table target has a descriptive name (no address-only sub_/loc_ names
  remain); 464 routines/entry points carry header comments; branch labels inside
  routines are named (mechanical @loop/@skip/@return or condition-derived names, with
  hand names for meaningful branches).
- RAM: ~330 symbols (zero page, object arrays, PPU queue, clocks, boat, fight, fish,
  sound driver, script engine); hardware, MMC3 and button constants; MMC3 register
  indices and button masks substituted automatically where the context is certain.
- Data: named tables at every consumer-referenced address in banks 0/1/2/6/7/13/14/15;
  jump tables rendered symbolically; text runs show their string; generic data blocks
  carry a one-line note from their phase-1 rule.
- Unknown ranges (777, 17,738 bytes): each has a boundary note (location, length) and an
  honest role note (text-gap / code-plausible but uncalled / non-code-like / data-bank
  residue, plus static references into the range when known). Bank-0 ranges with static
  fixed-bank callers have hand notes (redirect question Q2).

Names/interpretations introduced: see annotations.txt (grouped by bank). Key systems:
PPU command queue, palette fade tables, metasprite renderer, map streaming, script
interpreter (75 commands) and text printer, map message windows/COMMAND menu, boat
physics/camera, wake/lure/sun objects, fish AI and both bite paths (Fish_CheckBite,
FishShadow_TryBite), fight model (tension, line strength, line out, fish stamina terms,
events), sound driver command/stream format, sound test, area select, round info.

Verification: see ../commentary/verification.json (byte_identical true at handoff).
Reading copy: reading-after.asm (refreshed at handoff).

Deferred / remaining editorial work (not blocking this pass):
- ~280 generic local labels (@skip/@skip2...) remain inside documented routines.
- Fight fish-model term tables (bank 1 $B476-$BA60) are named mechanically; the game
  meaning of individual terms and of fight events 1-9 is not established.
- Sound driver command semantics $C0-$E0 are named from code paths; not verified by ear.
- editorial_audit stays pending (final acceptance only).

## Round 1 continuation (coordinator readability gate)
- Branch labels: commentary_build.py now names local labels from the branch condition
  (button, symbol compare/zero/sign/bit tests incl. shift chains and `lda #m / bit`,
  carry after 16-bit adds, routine carry results) or from what the target does
  (call/goto/store/clear/return value); multi-source targets prefer a condition shared by
  all sources. Lone @loop/@return kept; scopes with several loops get body-derived names.
  ~150 hand names added (annotations.txt, "continuation" section) for the remaining
  generic cases (sound channel tests, Mul8 multiplier bits, metasprite modes, ...).
  Result: 557 local labels, 0 @skip*, 0 numbered @skipN/@loopN/@returnN, 45 lone @loop,
  81 lone @return; promoted locals now take the nearest preceding global as prefix
  (no more Fish_RunBehaviour_swimming_4 style names).
- Fight events 1-9 renamed from the bank-12 event script texts (FightEvent_PropellerCut ...
  FightEvent_RunAfterRest), FEV_* constants for all 14 SeaEventFlag fight values, RollEvent
  chain/carry quirk documented; GloveHeat/ReelHeat/FightGauge explained.
- Fight fish model helpers/tables renamed by the value they update, with mechanical headers.
- Sound: stream commands $C0-$E0 each documented; envelope/sweep/scale misnomers fixed.
- Verification: commentary/verification.json ok, byte_identical true; reading copy refreshed;
  review.json regenerated (3,170 entries reviewed, 864 units; editorial_audit pending).
