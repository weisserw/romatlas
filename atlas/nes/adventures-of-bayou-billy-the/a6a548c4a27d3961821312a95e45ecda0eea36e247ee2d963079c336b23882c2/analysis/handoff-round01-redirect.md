# Round 01 redirect — ready combined batch

Redirect is complete for this checkpoint. Return the combined batch to independent
disassembly review; do not treat these proposals as accepted classifications.
No productive experiment or connected investigation is left running.

Accepted input remains **pass-001-final**, maps/current.json SHA256
**687e9f29ec9f58f16b94a00c952536b7f9464a679019f8d37fe13100509a097b**:
**151364/262160 classified (57.74%), 110796 unknown in 273 intervals**.
The coordinator-accepted current-source readability gate and byte-identical source
are unchanged. ROM, internal map/state and commentary source were not modified.

The proposal union adds **110789 currently unknown bytes**: **23909 Code** and
**86880 Data**, plus C01 changes **13 existing Static Code bytes to Data**.
If independently accepted in full, this would leave **7 unknown bytes in five
intervals**; it is a hypothetical count, not a completion claim. Fine data semantics
are intentionally unresolved where generic Data is sufficiently supported.

Primary package: `analysis/redirect/round01-20261031-a/`.
- `proposals.json`: exact instruction/data intervals and explicit effective bank views.
- `final-audit.json`: counts, hashes, all fourteen E116 sites and physical-view checks.
- `residual-ledger-final.json`: exact unchanged accepted complement, bank segments,
  neighbors, proposal evidence/dispositions, and localized residuals.
- `findings.md`: preserved research checkpoints, corrections and method outcomes.
- `leaf-contracts.json`, `additional-contracts.json`, `fragment-contracts.json`,
  `localized-reassessment.json`: structural entry/field/stack contracts, including
  unobserved routines. Graph decoding alone is not their evidence.
- `analysis/leads.json`: version 1, prior entries/outcomes preserved, fourteen stable
  leads, eleven ready; R01-L14 records the five blocked narrow ambiguities.

## Combined findings to review

**C01 / R01-L07:** Driving mode 2 selects slots [10,11,14,15] and entry $800F;
$8000 is selected only by the other mode branches. The disputed physical
$14010..$1401F prefix is five three-byte constant arrays, read by $81EF, $805F
and $80D7. Reject the lost mode/bank correlation that supported its old static code.
Remove only the 13 Static bytes before annotating the 15-byte data span; preserve
observed $800F code. Natural use of a third variant is not asserted.

**R01-L01/L08:** Followed the coherent driving, brawl, shooting, ending, text,
audio, sprite, HUD and fixed-helper graphs, including all supported finite actor
handler tables, new brawl subtypes and seven boss phases. External bank wrappers
were traced through push/call/pop under explicit busy=0/balanced-stack contracts.
Fixed code proposals use [null,null,14,15], not the graph generator's placeholder
low bank. All fourteen E116 sites have barriers; new tables are at physical
$F707 (14 bytes), $1F133 (4), $7CA6 (12), and the player table gains its final
word at $C324. Shooting state-table slot 0 often overlaps preceding instructions
and is excluded from Data claims. Four justified always-taken/exhaustive branch
corrections prevent decoding constants as code. No proposed instruction overlaps
another instruction boundary or accepted Data.

**R01-L03/L05/L09/L10/L11/L13:** Complete consumer-linked asset batches include
17 PPU RLE containers; 23 dialogue scripts; 38 dialogue tile strings; 101 PPU
string pointers with 81 unique strings; 109 PPU command pointers and a generic
command pool; five valid bank-2 sprite families; two bank-12 driving sprite
families; eleven background scene origin sets, 48-byte columns and 16-byte
metatiles; six four-byte brawl event streams; three driving event and road streams;
three controller/duration demo streams; audio descriptors/dispatch/envelopes;
and referenced animation, sound-menu, credits and numeric constants.
The scene-7/8 pointer to the next table was excluded from the column claim.
A failed FF-only PPU parser was rejected without rejecting independently supported
non-code pool bounds. Exact files/recipes are indexed by the leads.

**R01-L11/L12:** Driving projection has an encoded origin and consumer-derived
32×128 stride, monotonic quantized rows, and a separately retained FF border/tail.
No exact generator was required. The RAM raster upload has an independent ROM
copy source and length: 153 bytes from $DAE5 to $0600. Propose only the 135 unknown
source bytes as a Data code template; retain the 18 overlapping Observed ROM-code
bytes. Packed audio-like slabs are generic Data. Eleven four-byte DMC register
descriptors do not establish complete sample extents: some apparent sample spans
overlap code under the observed context, so that stronger hypothesis was rejected.

**R01-L06:** Six additional reset footer copies match the independently observed
stub and vectors. Separate pre-INC local views, the actual post-reset final-bank
fetch, and copied JMP template roles. Do not assert local fallthrough after MMC1
reset. Pair 12/13's all-FF footer is not a reset-stub copy.

## Gameplay and remaining uncertainty

Representative artificial progression now includes a moving staff credits roll,
**THE END**, and Start exiting to Stage 1. Exact patches, button release/presses,
20-frame budgets, snapshots and selected captures are in `credits-test-log.json`,
`terminal-test-log.json` and scenarios.json. This does not establish natural boss
defeat or natural ending reachability. Capacity-zero tracing intentionally dropped
detail while retaining aggregates without aggregate drops; saved credit coverage
contains 1584 physical and 1748 contextual accepted roots.

The five hypothetical residuals are $C1E2..$C1E5 and lone $60 bytes at $C7A6,
$14379, $1ACD0 and $1EE00. `residual-rejection-review.md` compares each against
primary neighboring code and competing constant interpretations. A valid jump
destination or isolated RTS encoding does not independently prove code; neither
is generic Data proved. Two earlier overly strict prefix rejections were corrected
using parallel helper/complete continuation contracts. No unchanged artificial
entry experiment would discriminate the remaining five roles. This is a ready
batch handoff, not terminal workflow exhaustion; reassess any changed premise
following independent review.

Final checks: zero proposed Code/Data conflicts, zero partial instruction overlaps,
all proposed execution views resolve to the stated physical bytes. The only
Data-over-accepted-Code overlap is C01's deliberate 13-byte correction. Source and
ROM hashes still match the commentary handoff. New classification/rebuild and full
current-source commentary remain the next phases' responsibilities.

Toolkit report: existing NES-001 only (one missing feature), no new confirmed or
suspected bugs or blockers; see `analysis/toolkit-issues.md`. No toolkit edits,
archive execution, publication, deletion or cleanup. Free space remains 9.7 GiB;
scratch remains 696 KiB. All research and captures remain local.
