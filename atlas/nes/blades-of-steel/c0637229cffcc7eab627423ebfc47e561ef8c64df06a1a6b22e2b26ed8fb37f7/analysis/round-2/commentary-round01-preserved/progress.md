# Round 1 current-source commentary

## Round 1 — commentary active

Prepared once using supported toolkit commentary.py prepare. Frozen baseline and
phase1-state retained in commentary; initial source reconstruction byte-identical.
Current phase is commentary, not redirect. Readability frontier at preparation:
all classified code/data and unknown boundary notes pending; no source edits or
semantic review credit yet. Full scope is 15,218 code +33,020 data bytes and
82,850 unknown bytes, 7,626 review scaffold entries. Source game.asm has44,458
lines because each accepted instruction has its own physical region. Work will
review meaningful contiguous routines/blocks, beginning fixed-bank initialization,
interrupts and mapper/dispatch helpers, then every switchable-bank code range.
Exact remaining scope is all review.json entries with status pending.

### First coherent readability batch

80 hand-inspected units now explain 2,762 physical bytes: bank-zero OAM renderers,
metasprite expansion and actor sorting; fixed-bank PPU commands/uploads, reset/NMI,
mapper wrappers, scene fragments, input sampling/edges, clock countdown, arithmetic,
inline dispatch and banked PPU-stream decoding through physical $01C971. Symbols
replace understood hardware/RAM operands consistently across all classified source.
Fresh commentary verify passed byte-identical with the new symbols include.

Structural concern C01 is recorded in uncertainties.md: $000328..$0003F1 overlaps
a consumed pointer table yet is frozen Code. Its comment preserves that conflict
instead of inventing a routine purpose. No classification changed. Remaining exact
scope: review.json entries still status=pending, including header/data/unknown notes,
all switchable-bank code beyond $0003F1 and fixed-bank code at/after $01CC9F.
Current phase remains commentary; review is incomplete, no handoff/phase gate claimed.

### Readability batch 2

170 explicit hand-inspected units now persisted. Reviewed bytes by kind: {'Code': 6089, 'Data': 33020, 'Unknown': 82850}.
All bank-zero accepted Code through physical $0035E9 is explained, including
actor contact windows, control dispatch, motion scripts, team input paths and
animation mechanics. Generic data notes reuse the retained boundary/consumer
inspection, and every unknown interval now has an honest exact-boundary note.
Fresh verification again passed byte-identical. Current phase remains commentary.
Exact remaining classified Code intervals: analysis/round01/commentary/pending-ranges.json
(also status=pending in review.json), beginning bank1 $004011 and bank2 $008011,
then banks4/5/6 and fixed-bank code after $01CC9F. No phase handoff yet.

### Readability batches 3 and 4

276 hand-inspected units persisted; reviewed bytes {'Code': 10601, 'Data': 33020, 'Unknown': 82850}. Bank1 camera/strip,
menu choice/highlight and presentation mechanics, smaller bank2/4/5 PPU queues,
and all classified bank6 sound-engine code are now explained. Two further frozen
code/data conflicts are recorded in uncertainties.md as C02. Fresh verification
passed byte-identical after each coherent batch.

The earlier bank-zero blanket statement omitted the three-byte DEC at physical
$002DD8; it remains pending and will be covered with the next fixed-code batch.
Exact remaining scope is pending-ranges.json: that instruction plus fixed-bank
code from $01CC9F onward. Current phase remains commentary; no handoff yet.

### Round 1 commentary phase complete

All current-source readability work is complete:15,218 classified Code bytes,
33,020 classified Data bytes and exact uncertainty/boundary notes for82,850 Unknown
bytes. All7,626 scaffold entries reviewed;381 meaningful hand-inspected units with
source line locations and493 descriptive routine/branch entry labels. No pending
readability ranges remain (pending-ranges.json is empty). Symbols now consistently
cover hardware, shared scratch, actor fields, sound-channel arrays and known targets;
reused scene bytes60/61 have neutral global aliases to avoid false permanent roles.

Examples: OAM metasprite expansion/sorting, PPU token decoding and queue formats,
reset/NMI bank protection, double-sampled controller edges, actor motion/drag and
packed collision probes, angle quantization and scaled squared distance, plus
sound arbitration/envelopes/DMC setup. Uncertain game purposes use mechanical names.
C01/C02/C03 in uncertainties.md preserve concrete frozen code-versus-inline-data
conflicts for redirect. No map/classification or ROM change was made.

Final fresh commentary verify returned ok=true,byte_identical=true. Source hashes:
game.asm68b8914ded989b70c0debd79399f6c01de17bf159616bab9ea69cdb346d6efd4;
symbols.incb5cc634f03a7bdb8b9b58cbaa3292a4af57cbf9f775e34bbfb22c787414110c9.
reading-after.asm refreshed successfully afterward. Scope/fidelity assessment is
commentary/scope-assessment.json; exact verification is commentary/verification.json.
Frozen/current map hashes remain ecde055d22cf42971290adbfd33853bdcb7abe0d42907ed8ad330a9693f68c62.
No final audit was run; editorial_audit remains pending for final acceptance.
ROM classification remains incomplete at36.80%; readability completion does not
resolve the unknown byte complement or establish full game/hardware compatibility.
No toolkit issues found; report analysis/toolkit-issues.md preserved.
Current worker phase is commentary, completed; coordinator may now select redirect.
Handoff: analysis/handoffs/round01-commentary.md.
