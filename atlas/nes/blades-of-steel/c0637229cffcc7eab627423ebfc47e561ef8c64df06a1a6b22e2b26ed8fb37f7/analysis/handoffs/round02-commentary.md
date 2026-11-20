# Round 2 commentary handoff

Current-source readability complete; ROM classification remains incomplete at 81.00%.
Checkpoint: round02-disassembly-closure. Map/state SHA256 unchanged:
`e734fb13fbb234220432d10fd198d4336bdf369b1830ce8dbcb594b25b0fbac2`.

Covered all 29,636 Code and 76,542 Data bytes, plus exact uncertainty notes for
24,910 Unknown bytes. All 14,274 scaffold records reviewed; 820 meaningful units
and 1,002 descriptive source labels. No readability gaps: pending-ranges.json is
empty. Final editorial acceptance remains separately required even for a terminal
incomplete ROM; review.json editorial_audit is pending. No final audit run now.

Entire prior commentary preserved intact at
`analysis/round-2/commentary-round01-preserved`, with hash manifest
`analysis/round-2/commentary-preservation.json`. Supported fresh preparation used
the current map. Exact kind/extent/view carry retained 7,251 valid records; four
contradicted old units were invalidated. All 27 old dependency questions rechecked:
`analysis/round-2/commentary/dependency-dispositions.json`. Useful prior prose and
names retained; changed boundaries and new mechanics inspected explicitly.

Editable source: `commentary/source/game.asm` and `symbols.inc`. Names distinguish
CPU table addresses from physical data labels. Fight/shootout RAM aliases are
scoped to their consumers because scene storage is reused. Direct transfer symbols
preserve frozen physical-instruction destinations and source-bank context.

New work explains actor contacts, paired states, targeting/control transfer;
camera map tiles/attributes; tournament bracket traversal and simulated outcomes;
shootout attempt scoring/physics; fighter input, AI, animation, contacts and effects;
intermission activity; bounded tile/PPU uploads; sound loops/envelopes; fixed state,
angle, nearest-candidate and software-speech helpers. Original asset, team, phrase
and instrument names, tactical categories/scales and some natural callers remain
uncertain. Source states established mechanics without inventing those meanings.

Concrete old defects repaired:
- PPU helpers $C00A/$C107 repeat one loaded byte: the backward branch targets
  the PPUDATA store. Helper $C15D instead copies literal bytes.
- Presentation templates at $E11D copy twelve bytes, replacing the six-byte claim.
- A counted record's high header bit bypasses the queue prefix; it is not repeated fill.

C01/C02/C03 are historical and structurally resolved by disassembly; source now
explains the actual data formats. New bounded redirect question C04:
- Physical $012853..$012857, CPU $A843, banks [8,9,14,15], bytes `20 00 E0 FF`
  are read as signed +$0020/-$0020 effect deltas by $A824/$A82D with Y=0/2.
- Predecessor `LDA #0; STA $05AC,X; BEQ $A838` cannot fall through. Current map
  nevertheless retains the first three bytes as Static Code JSR $E000, fourth Unknown.
- Inspect accepted root/edge or dual-use evidence for that exact physical entry
  before correction. Do not derive a valid $E000 caller from possible table bytes.
  Evidence: `analysis/round-2/commentary/conflict-C04.json`. Exact decoding and
  competing interpretation remain in source and uncertainties.md. No map edit made.

Fresh final verification passed byte-identical; reading-after.asm refreshed.
Report: `commentary/verification.json`. Compact scope, examples and hashes:
`commentary/scope-assessment.json`.
Source SHA256: `57445c04e4506fac3ff000c3f3565b702c99c7a563fe2d49d6c6e4141dce4ee8`.
Symbols SHA256: `79cb86a997a62b1ceff9ba4369f2904c5f5e983458d39a61bb11b49948962c58`.
ROM SHA256 unchanged: `c0637229cffcc7eab627423ebfc47e561ef8c64df06a1a6b22e2b26ed8fb37f7`.

All source, assets, state and evidence stay local. No toolkit change, disassembly,
redirect, archive, cleanup, deletion, publication or sharing performed.
