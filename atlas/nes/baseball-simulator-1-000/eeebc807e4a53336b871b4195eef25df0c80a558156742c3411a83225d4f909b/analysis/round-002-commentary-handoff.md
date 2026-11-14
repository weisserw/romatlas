# Round002 commentary handoff

**Current-source readability complete. Whole-ROM classification remains incomplete.** Final editorial/archive acceptance is pending; `audit` was not invoked.

Frozen map/state/baseline SHA256 `7dd308535281fec582e2294a84bb62b73b1d9cd258bf94823b9434f84a8bd0f4` agrees across all four artifacts. Original input SHA256 remains `eeebc807e4a53336b871b4195eef25df0c80a558156742c3411a83225d4f909b`. Classification247,503/262,160 (94.41%): Code72,934, Data43,497, CHR131,072; Unknown14,657.

Before fresh preparation, preserved the **entire** previous commentary at `analysis/round-002/previous-commentary/`. Reused3,581 exact physical Code blocks/21,864 bytes, valid names/prose and review credit. New/changed Code51,070 bytes includes51,047 newly classified Code plus C001's23-byte preserved prefix; all83,786 new classified bytes have current source explanations. Prior one-shot edits were not blindly replayed.

- Reviewed all new code across audio; menu/save/team/ability editors; text/budgeted fields; presentation/standings/ranked lists; automatic batting/pitching/defense; runner/fielder/throw/contact mechanics; ball fixed-point flight; fixed camera/map/PPU/palette/arithmetic/runtime. Purpose, control flow, record layouts, bounds, state changes and unresolved mechanics are explained in source.
- All996 raw payload blocks explain their supported formats or bounded unknown role. Pointer/inline scaffold fragments describe complete2/4/5-byte parameter groups; data units are grouped without changing the scaffold.5,128 scaffold records reviewed,1,625 meaningful units, zero pending readability entries. Added2,291 scoped control-target names plus RAM/field/constants. Counts in `analysis/round-002/readability-counts.json`.
- C001 now explains exactly twelve audio-id Data bytes `$018AC3..$018ACF`, retaining the adjacent23-byte Code prefix and its labels. Canonical `$01D0C8..$01D0D0` remains `LDA #$2B; BIT $2CA9; BIT $2DA9`; explicit `ObjectSelectState2C_D0BB`/`ObjectSelectState2D_D0BE` aliases and selecting-cell indices/offsets are beside the22-cell table. BIT reads/flags remain real.
- Fixed concrete stale carried dependencies, including newly classified continuations, the already-Code interior `$CB33` +$10 pointer helper and `$D51F`'s sprite-frame field naming. Existing useful explanations were retained.

**Redirect C003:** physical `$00C590..$00C596` currently renders DEC/STA across a two-byte inline-copy descriptor pointer and the following JSR. `$E68D->$FBA2->$FC1E` consumes a pointer, not six inline bytes. Source and `commentary/uncertainties.md` retain exact bytes, contexts and bounded correction proposal. Frozen classification is unchanged; this caveat does not invalidate other regions. C001/C002 statuses and remaining statistic/ability/artwork/command meanings are current in that file.

Prior gameplay limits remain: natural Ultra pitching/point use is established, defensive MOVE and the34-byte audio branch were artificially reached. No successful special hit, live-contact fielding/catch or natural Season terminal claim. Unknown bytes remain raw. No readability ranges are deferred.

Fresh final byte-identical verification: `analysis/round-002-commentary-verification.json` and `commentary/verification.json`; reading copy refreshed at `commentary/reading-after.asm`. Final source SHA256 `687c97b04b3bf6c0c9f20428543007070ea3964c83bba113c93fba07a9ee86c5`; symbols SHA256 `6b3605c46d4e4d02f673741b6a30aab2eff730ffe5ed2869b73289d13895db6b`. Hash agreement/reading-copy digest: `analysis/round-002/commentary-final-hashes.json`.

Next frontier: redirect C003 and residual supported bank-context leads, preserving this maintained source and its local assets. No toolkit/input/map changes, cleanup, archive packaging, deletion or publication occurred during commentary.
