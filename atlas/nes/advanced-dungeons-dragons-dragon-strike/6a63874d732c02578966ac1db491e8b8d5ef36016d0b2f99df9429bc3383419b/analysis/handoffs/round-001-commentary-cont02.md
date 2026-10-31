# Round 001 commentary cont02 interruption handoff

**Interrupted by context scope, not complete. Resume the SAME commentary phase.**
Original round-001-commentary.md is preserved. Do not regenerate source/ or rerun
the one-shot cont02 edit scripts. No redirect, final editorial audit, archive,
cleanup, deletion, toolkit/ROM/map edits or publication occurred.

Current map/state SHA256 c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e.
ROM SHA2566a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b unchanged.
Classification unchanged: Code26229, Data109079, Image262144, Unknown126852.

Fresh byte-identical verification passed in commentary/verification.json. Source
commentary/source/game.asm SHA256
f184d62efe5bbf7ee0120fa3caef8d90a611c090511158d60b2f2ecca4c96c98.
Reading copy commentary/reading-after.asm refreshed. Keep .org/.reloc scaffold and
all exact addressing-mode prefixes. All input/source/assets/evidence stay local.

Reviewed code9,140 bytes /4,520 records; pending17,089 bytes /8,474 records.
Every pending Code record in commentary/review.json remains work. Exact pending
complement: analysis/round-001/commentary-frontier.json (256 ranges). All Data,
CHR and unknown-boundary prior review credit preserved. All currently classified
physical bank31 code is reviewed. Final editorial_audit remains pending.

New mechanics: completion carry predicates and guarded effects; scaled three-sprite
meter and OAM clearing; queue reservation/address headers; nametable filling;
descending render-record selection sort; fair row-band sprite omission; palette
copy/transfer/fades; scene CHR phase/register dispatch; 4x4-metatile row expansion,
attribute nibble merging, tile coordinate/class query; five-byte scene-record
clipping/cache/timers/state/action preparation and rectangular background-patch
queue producer; bounded/wrapping scroll and alternating NMI row halves; eleven-byte
object allocation/reservation/compaction, lifetimes/phases, contact rectangles,
movement scripts80..83 and six-bit fractional movement; fixed30 direction quantizer,
absolute-vector quadrant mapping, restoring quotient; meter subtraction/addition
and scene-reference tile sampling. Names use observed roles, not game identities.

Corrected earlier immediate operands that were accidentally named as unrelated RAM
addresses ($00/$02/$03/$04 and $E2). Immediate symbol sampling now uses actual
AUDIO_* or BUTTON_START constants. Corrected direction-wrapper signs by checking
the previously reviewed F1E2/F1F4/F206/F218 helpers. Preserve source explanations
of exact carry and category behavior rather than normalizing arithmetic: E2FE
class pointer high ADC uses actor-shift carry; movement83 cloning feeds scratch0/2
to an appender consuming bit0. Unknown source meanings remain explicit.

Uncertainties.md stale opaque PPU/CHR/palette helper frontier reconciled. Their
mechanics are reviewed; exact graphics/game identities remain unresolved. DA10 is
now QuantizeDirectionBetweenPoints; EB0D is typed-object allocation; CF23 subtracts
the scene meter. Bank-local APIs and unknown interval D880 still have unresolved
payloads. Do not decode unknown bytes to fill a narrative gap.

Next: remaining fixed30 code, then stage/menu banks. Pending bytes per physical
bank:4=1387,6=794,8=758,10=933,12=830,13=148,14=2182,16=1250,18=1195,
26=676,27=310,28=1827,29=2048,30=2751.
Fixed30 exact remaining physical ranges (exclusive ends; CPU aliases in parentheses):
- 248397..248686 ($CA3D..$CB5E)
- 248720..248831 ($CB80..$CBEF)
- 248836..248935 ($CBF4..$CC57)
- 248972..249145 ($CC7C..$CD29)
- 249193..249334 ($CD59..$CDE6)
- 249340..249346 ($CDEC..$CDF2)
- 249370..249651 ($CE0A..$CF23)
- 249994..250211 ($D07A..$D153)
- 250341..250500 ($D1D5..$D274)
- 250534..250748 ($D296..$D36C)
- 250847..250856 ($D3CF..$D3D8)
- 250860..251417 ($D3DC..$D609)
- 251420..251423 ($D60C..$D60F)
- 251438..251447 ($D61E..$D627)
- 253039..253097 ($DC5F..$DC99)
- 253121..253176 ($DCB1..$DCE8)
- 253252..253503 ($DD34..$DE2F)
- 253514..253598 ($DE3A..$DE8E)
- 253631..253664 ($DEAF..$DED0)
- 253666..253668 ($DED2..$DED4)

Use bounded extraction of .org blocks for instruction review; source is one large
buildable ca65 listing. Review source code mechanics and references, add useful
symbols/comments and compact units, mark only actually inspected classified
instructions reviewed. Update source references/frontier after coherent batches.
The repeatable commentary-cont02-checkpoint.py refreshes source locations/frontier
and checks earlier literal fixes, but all other cont02 scripts are one-shot edits;
do not rerun them. No other writer is active.

Progress: commentary/progress.md and analysis/progress.md. Toolkit issue log still
has only existing NES-001; no new issue was found. Do not modify the toolkit.
