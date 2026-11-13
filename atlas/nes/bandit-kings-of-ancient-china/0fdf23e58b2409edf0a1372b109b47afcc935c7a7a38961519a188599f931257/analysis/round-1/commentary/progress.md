# Commentary progress

pass-001 current-source readability complete on frozen pass-001-extended map/state.
All 732 full-ROM scaffold entries reviewed: 513 classified intervals and 219
explained unknown intervals. No prior commentary credit existed to carry forward.
All 3,358 classified native code bytes, 14,162 text bytes, 46,282 data bytes and
131,072 image bytes covered. No structural or classification edits.

Source now explains every native routine family, individual opcode entries and
shared branch mechanics. Consistent RAM names cover VM pointer/value/stack pairs,
PPU and ExRAM transfers, palette/OAM state, stack guard, sprite flash and counters.
All classified main-opcode targets have entry labels; meaningful loops and branch
tails have functional names. Text slices include literal previews; generic runs,
CHR tile banks and unknown intervals have honest role/boundary limitations.

Examples: frame save/restore and inline displacement; MMC5 partial-product multiply
and sixteen-step division; signed comparison booleans; absolute/relative VM branch
operands; bounded row copy/fill; stack-failure sprite construction; guarded PRG
register writes with physical bank ambiguity retained.

Fresh verification passed byte-identical after final source edits; input SHA-256
0fdf23e58b2409edf0a1372b109b47afcc935c7a7a38961519a188599f931257.
reading-after.asm refreshed. Review inventory records source lines and compact units.
No unfinished readability ranges. Final editorial_audit remains pending for archive
acceptance; no final audit run. Structural classification remains 49.56%, execution 0.

Open questions and naming constraints are in uncertainties.md; existing L002–L004
remain for redirect. Mapper 5 runtime limitation remains the sole toolkit finding:
1 missing feature, 0 confirmed bugs, 0 suspected bugs, runtime-only blocker.
No toolkit changes, cleanup, archival, publication or redirect work performed.
