# Round 001 commentary interruption handoff

**Interrupted, not complete. Resume commentary before redirect.** No prior
commentary existed; prepared the verified baseline once. Do not regenerate source/.
No final editorial audit run; keep it pending for terminal acceptance.

Map/state/current frozen SHA256 c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e.
ROM unchanged SHA2566a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b.
Classification unchanged75.81%: Code26229,Data109079,Image262144,Unknown126852.

Latest byte-identical verification: commentary/verification.json. Source
commentary/source/game.asm SHA256 ecd59d5151001b85fd28444a1409b7f5a3e1b188499ac1db53f1fc30d7eebb44.
Reading copy refreshed: commentary/reading-after.asm. Single buildable ca65 source
has instruction-level .org/.reloc scaffold; preserve exact addressing modes a:/z:.
Source edits/comments/symbols and hardware names only; no classification edits.

Readability:2,811 code bytes /1,271 records reviewed;23,418 bytes /11,723 records
pending. Every pending Code record in commentary/review.json remains work.
Exact pending contiguous physical ranges: analysis/round-001/commentary-frontier.json
(278 ranges). Reviewed code ranges, exclusive ends:
20..29;15442..16130;16404..16413;31335..32023;
258546..258618;258661..258680;258705..258938;259129..259145;
259145..259521;259523..259654;261421..261991.
All134 Data records,359 Unknown intervals and CHR Image record reviewed for
boundary/role notes, reusing prior sound/uniform/reference evidence. Generic Data
requires no invented semantic records. Unknown bytes stay undecoded with ambiguity
notes. Scaffold preserved; meaningful compact units added and source locations current.

Examples: both bank-local five-channel sound drivers now name event decoders,
countdowns, channel arrays, presets, period tables, commands and effects. FE selects
a preset, FF uses alternate track/channel restart pointers, FC/FB stop, E2 stores
argument-minus-one. AudioPriorityOrIndex at E6 is also initializer scratch; $F2D2
proves priority comparison for requests. Corrected early arithmetic wording: the
second ASL determines carry, clear for track<64; no incoming caller-carry dependency.
Fixed31 mapper routine distinguishes register values OR$20/$21 from physical
indices and verifies header identities. NMI/PPU queue/serial controllers/reset
selective RAM clearing are explained. Scene frame marker stack and F503 carry1
completion contract match recorded artificial progression. Pause Start masks and
APU silence bit mechanics are explicit.

Unresolved meanings live beside source and in commentary/uncertainties.md. In
particular the outgoing actor/render helper meanings and bank-dependent APIs need
continued review; opaque calls are explicitly named by address until reviewed.
Sound song/effect identities, E2 consumer and exact streams remain unresolved.
PPU producer/alternate-update contracts remain pending. Do not confuse generic
sound Data completeness with per-track schema requirements.

Next useful frontier: connected fixed31 $F636 completion handlers, $F715/$F77D/
$F78D, $FA98/$FBEF/$FC5F/$FCCE, then earlier fixed-bank/physical30 routines and
stage/menu banks. Pending bytes by bank:4=1387,6=794,8=758,10=933,12=830,13=148,
14=2182,16=1250,18=1195,26=676,27=310,28=1827,29=2048,30=3335,31=5745.
Use source compact instruction extraction for bounded regions; comments explain
mechanics without inventing game meanings. Preserve valid reviews and enrich their
dependency names when callees become understood.

Detailed progress: commentary/progress.md, analysis/progress.md. Existing toolkit
report analysis/toolkit-issues.md:1 confirmed bug NES-001,0 suspected,0 missing,
no blocker; no new finding or toolkit edits. No redirect, archive, cleanup, deletion,
publication or ROM edits. All source/assets remain local.
