# Round001 redirect complete — combined proposal for independent disassembly

Research is complete for the identified promising actionable avenues; this is not
an interruption, terminal acceptance or classification change. No live probe remains.
Frozen research: analysis/redirect/round001-20261030-a/. Read its plan.md for method
history/corrections; do not rerun one-shot package/finalize scripts.

## Protected checkpoint and actual scope

- ROM SHA2566a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b.
- Map/state SHA256c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e.
- Source SHA2567c2412a800002f18dbeae885737e0aefad239220b4e2624dd47a8c43651b2130.
- Actual classification unchanged:397452/524304 classified (75.81%);126852 unknown
  in359 exact intervals. Code26229/Data109079/Image262144. Accepted new bytes0.
- Current-source readability remains independently accepted,0 pending for unchanged
  source; final editorial audit remains pending. New classifications require full
  current-source commentary after independent disassembly/rebuild.

## Reviewable combined batch

annotation-proposal.json proposes126846 NEW bytes: Code22645 (10759 instructions),
Data104201. All are exact currently unknown intervals, with source locations and
physical CPU/bank views. Of new Code,143 actual keyboard opcode addresses/295 bytes
are Observed; all other new Code/Data are Static. Contexts and underlying evidence
objects must be reviewed; decoder agreement alone is not the structural basis.

Audit: audit.json and decode-validation.json. Configured binary cross-checked all
10759 instructions in244 groups. No candidate instruction overlap, Code/Data
conflict, unofficial candidate or physical-view mismatch. Explicit CHR-only store
patterns/constant dominators and palette RAM-pointer stores were audited. Fresh
protected hashes above passed unchanged. No map/state/source/toolkit/ROM write.

Data union ranges in the manifest are accounting slices: use underlying objects
for specific roles/bounds and split as needed; do not claim one original table
merely because adjacent Data merged. Review/reject locally, preserve accepted map
records and history, annotate only supported unknown slices, then validate,
save/export and make a byte-identical checkpoint. No unchanged experiment replay
or natural selection of every table entry is required.

Useful primary artifacts:

- closure.json/closure.py, dispatch-tables.json, mmio-proofs.json: supported direct
  control, indexed RAM domains, bank/stack/return contracts, language APIs, all14
  existing indirect dispatches plus exposed direction helpers. Normal MMC3 mode0,
  valid palette-slot/input/direction preconditions are explicit. Fixed31 callers
  EC04/EEEE/F023 establish useful fixed30 entry contracts. Mapper-changing unknown
  effects, stack manipulation and data/mid-instruction barriers stop traversal.
- stage-data.json: two count4 directories per stage,32 scene slices, map IDs,
  16-byte pattern and1-byte attribute constraints agree;0 invalid IDs.
- stage-records.json, composites.json, movement-data.json, object-descriptors.json,
  demo-data.json: finite sentinels/counts/strides and independently anchored ends.
  Eight relocated47-script libraries parse exactly1029 bytes each. Sprite aliases
  retained; descriptor gaps were never swept in. Shared bank14 render IDs exceeding
  its local sprite span reject universal semantic-use validity, not paired Data.
- menu-data.json: eighteen finite RLE streams all expand exactly1024 bytes; two
  nine-entry pointer planes bounded by loader/writer code. menu-scene-data.json,
  menu-parameter-data.json and menu-composites.json prove24-scene fields, actor
  countsum70/parallel arrays, palette slice sums, keyboard maps and sprites.
- generic-data.json, class-data.json, small-data.json, residual-data.json: practical
  consumer/nearby code/pattern/boundary inspection supports generic Data where
  exact semantics/original layout remain unknown. Generic Data is complete
  structural classification; no invented labels or full selector-domain demand.
- utility-contract-review.md and preserved residual-reasoning-before-contract-review.json:
  localized corrected inference for37 Static utility bytes (already in Q002,
  not extra gain). Missing caller is not a universal prerequisite. Exact second
  input-helper entry is F276, not first helper's RTS F275.

Persistent analysis/leads.json preserves all history: Q001(stage Data), Q002(Code),
Q004(menu Data), Q005(fixed Data), Q006(utility subset) are ready for independent
review. Q007 records two local bare-JMP ambiguities. Q003 is legacy optional sound
semantic naming, not an unclassified structural blocker. Source locations/ranges,
contexts, recipes, expected gains and pass checkpoint are included.

## Concrete behavior/progression resolutions

Read resolved-behaviors.md and ending-data.json before new commentary:

- Four-input gate opens a two-row sound test:12 track choices and16 effects.
  Twelve-input gate toggles0601 bit0, clears its other bits, fades and warm resets
  into opposite26/27 versus28/29 language banks. Static contracts, no newly claimed
  natural cheat activation.
- Bank18 8A5E..8ACE converts NUL text to tile bytes and restores callerY; it is not
  an opaque event decoder advancing the cursor. Caller advancesY2. Exact ending
  layout is21 row/column pairs,21 word pointers,258 NUL text bytes,88CF..8A25.
- First password probe observed initialization only; its edit inputs were too
  early. password-active-probe.json independently waits60 ready frames, then right,
  A and B:0106 becomes1,0112 advances1 then returns0 without erasing the letter.
  observed-fetches.json merges143 actual new addresses,295 bytes,0 conflicts.
- New idle continuation of ending-home.snapshot visibly reaches scrolling designer,
  art, music, sound and special-thanks credits. Selected captures600..3000 and
  bounded routes/checkpoints through4200 are recorded in scenarios.json/E006.
  No new RAM/register patch; inherited artificial Queen success route is explicit.
  Final06F5=42 exhausts21 events, but distinct visible THE END was not captured.
  Trace-enabled100/600-frame calls hit15s; trace-disabled10-frame discriminator
  succeeded, then bounded300/600-frame calls advanced normally. No new toolkit bug.

## Exact residual and stop rationale

unknown-ledger.json retains the ACTUAL359-range126852-byte complement, neighbors,
bank/CPU identities and evidence-linked dispositions. If every proposal is accepted,
only6 unknown bytes remain: physical245776..245779 (fixed30 CPUC000 JMPFE98) and
255289..255292 (fixed31 CPUE529 JMPE614). Both remain materially code/data ambiguous;
raw mentions of C000 include mapper writes/numeric fields, not caller proof. Bare
jump decoding alone does not distinguish intended trampoline from embedded bytes.
The compact primary reasoning is in utility-contract-review.md. Hypothetical final
counts would be Code48874/Data213280/Image262144/Unknown6; none is accepted yet.

Connected stage/actor/menu/handler continuations and independent whole-table,
copy/count/sentinel/graphics constraints were followed while productive, not stopped
at first lead. All remaining non-code families were practically inspected. Exact
song names/generators, packed patch-attribute sublayout and glyph identities now
add semantics without structural gain; no unsupported table extent is required.
Further natural mode activation/rare mission permutations have no credible structural
link to the two remaining bare jumps. Representative early/middle/late/Queen/home
coverage is preserved and scrolling credits added; more unchanged idle capture is
not needed for this review batch. No promising actionable avenue remains deferred
merely as independent/outside scope. This is a supported handoff, not terminal
exhaustion; if independent review rejects a subclaim, localize its actual barrier.

No new toolkit issues found; existing confirmed NES-001 input documentation/schema
issue remains in analysis/toolkit-issues.md, complete buttons-array workaround used,
no blocker. No archive, cleanup, deletion, publication or toolkit change occurred.
