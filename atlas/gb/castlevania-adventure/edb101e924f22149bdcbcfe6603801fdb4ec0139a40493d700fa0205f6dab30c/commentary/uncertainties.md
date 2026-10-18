# Current archive status

Current checkpoint: round025; 64955/65536 bytes classified, 581 Unknown in 85 ranges. All 2807 source records and 3000 units reviewed, zero pending readability. Final editorial review is recorded in editorial-audit.md. Earlier counts and pending-review statements below are historical and superseded. Research stopped at FI025 method exhaustion; classification remains incomplete.

# Commentary uncertainties

Current round021 source readability remains incomplete:845scaffold records/8073bytes
pending in analysis/round-021/remaining-review.json. Older completion counts below
are historical and do not establish current-source completion. Current checkpoint:
analysis/round-021-commentary-handoff.md. Structural Unknown and natural-runtime
claims remain separate from editorial review.

- Main state $c006 and other inline selectors still need writer/domain checks. Prior ten-word table interpretation is unchanged.
- Transfer list bounds and bank-selected payload extents from round 001 are resolved by round 002. Sources below $4000 remain fixed bank 0; upper flag semantics beyond current zero-valued lists remain unresolved.
- Intro stream $1221..$1265 is now classified and reviewed; its six-run interpretation is unchanged. Adjacent $1266 remains unknown. Bank-3 addressed streams include a shared suffix at $7cea; their raw callers remain unclassified.
- Transition $c405 uses low two bits for the four masked phases and bits 3..4 for initial substeps. Window movement and block copy/cache mechanics support a scrolling-transition interpretation; exact scene context is unobserved.
- ResolveTransitionBlockTiles ($2c82) selects stage table $3505, nested $c412, row of five entries and column, then selects a 16-byte block with the low six bits. Threshold- and transition-sourced bounded stage/nested pairs now have exact widths, scrolling cells and supported raw-ID payload records. Broader $c412 bounds remain unresolved. Transition coordinate applicability is separate from scrolling bounds. Scrolling uses raw IDs, while transition masks to six bits. Stage-3 raw IDs >= $40 therefore require independent interpretation and remain withheld. PackFourThresholdClasses ($2d0d) now has a closed 0..3 class domain because each four-byte stage vector ends FF. This does not bound the payload sources or establish later-stage runtime reachability. Seven raw indirect-store continuations remain unknown, as do tile arrays $2e01/$2e11. These are bounded redirect questions, not proven new substantial leads.
- Bulk tile payload roles are supported structurally; individual scene/art identities and overlapping stage substitutions remain pending editorial work. No need to classify those roles again.
- Nineteen transition-sourced descriptor cases are reviewed. New list fifth fields include selectors 52, 54, 55, 60 and 61; check their source stage, descriptor reload and exact downstream extents independently. Successful gate passage and natural later-stage reachability remain unobserved.
- Bank-2 sound $405b/$4706 command and stream semantics remain deferred.

## Round008 affected scope
Continue initialization/selection were fetched only under constructed clean state. Confirmation's retry reset and decline06ed tail remain static; natural callers and input acceptance are untested. Further transition fifth fields need independent source-stage and descriptor reload proof. Existing stage3 flagged raw-ID exclusion remains. Final editorial audit and607 scaffold entries remain pending; unchanged unknowns were not re-reviewed.

## Round009 affected scope
Selectors56/57/65 exposed in exact source lists still need independent descriptor/context proof. Stage3flagged IDs, global selector/command domains, stream grammar/extents and natural sound/transition reachability remain unresolved. Concrete audio follow-up: inspect one selected stream with its channel decoder and termination/branch rules before proposing any extent. Pending605scaffold entries and final editorial audit remain; no unchanged unknown sweep.

## Round010 corrections and remaining dependencies
Supersedes earlier statements that selected5/6/7/21 stream extents are wholly unknown, and pending selector56/57/65 context: exact bounded slices now reviewed. This does not close either domain. Prior Code/fallthrough assumptions in81a9..81b8,8269..827a,82ad..82bf are invalid. Eight82af..82b6 bytes remain unknown despite pointer shape; prove actual nested selector bounds and consumers before classifying. Selectors58/59 require independent source-stage/descriptor reconstruction. Other audio streams, natural sound execution, stage3flags, bank1objects and global closure remain pending. No final editorial audit.

## Round011 affected interpretation
Supersedes round010's eight-byte unknown and pending selector58/59 statements: four82af..82b2 dispatch bytes now supported, four82b3..82b6 still Unknown;58/59 reload stage1/nested12/13 width5. EA16 endsJP41a2 and never reads8aa9. EA20 wraps its eight-bit index toFE and reads8b9d. Word targets/intervening entries, selector66 descriptor context, music40/48 call/loop paths, stage3flags, bank1objects and natural reachability remain open. No final audit;599scaffold entries pending.

## Round012 affected dependencies
Selector66 descriptor context is resolved: stage1 nested14 width ten. Music40/48 stream roles have closed static cyclic state graphs; natural playback, duration, other command domains and target4ac5 extent remain unresolved. EA20=8b9d;8aa9 and82b3..82b6 remain Unknown. Earlier round-specific deferred notes are historical.

## Round013 changed evidence
Earlier82b3..82b6 Unknown notes are historical: fresh EA3x/4x source events now support these exact dispatch words. Stage05aa table is conditional on0..3 without a local mask;0a0e masks locally. Stage graphs establish structural cycles, not duration or natural playback. EA20=8b9d,EA16 no fallthrough,8aa9..8aaa Unknown; selected target extents and other command sources remain unresolved.

## Round014 placement scope
The selected directory and record slices support forty descriptor-backed contexts only. E000 bounds the distance scan for nonnegative DE below 2000. Stage 1/nested 12 is nonmonotonic; its second row has structural support but no natural selection proof. Equality does not spawn. Object type meanings and wider index domains remain unresolved; raw pointer targets do not prove execution. See analysis/round-014/commentary-review.json for affected consumer anchors.

Round017: finite first-state body fetches supersede prior wrapper-only limitations for the accepted18 cases. State1-body execution, types25/33/45/46 contracts and ordinary selection remain unresolved; see analysis/round-017-commentary-handoff.md.

## Catch-up010 correction: persistent contact selector
The earlier separate actor+9 interpretation is superseded for the normal2fc9 path. HL moves from actor+0b to+0c, then adds6 to+12 and9 to+1b. Thus the2f7e lookup/inline dispatch reads the same stored type field+1b. Historical conversion tests remain valid and did not execute this contact consumer. No new selector-domain/natural-reachability or classification claim follows. Exact original bytes and static derivation: ../analysis/commentary-catchup-010/contact-selector-correction.json. Frozen phase1 evidence retains historical wording; this editorial correction supersedes it without changing maps.

## Catch-up014 audio interpretation corrections
EA3x targets work byteC111, then CPU45e6 writes group register1 or copies a waveform; this is duty/wave selection, not envelope register2. Source and EA3/4 word editorial summaries now reflect that. Frozen phase1 naming/history stays unchanged. Pitch sign is set-only in the local decoder, FE equality precedes its FF special case, and channel work byte24 overlaps the pause wrapper marker. No global selector/domain, natural playback, execution or new classification claim. Exact source/changes: ../analysis/commentary-catchup-014/.

## Catch-up016 current limitations

All classified code now has functional labels/contracts, shared RAM and contextual
scratch names, and reviewed data formats. This does not resolve the 19,820 Unknown
ROM bytes, globally unbounded selectors, inherited ROMX bank assumptions, natural
later-state/transition/audio reachability, unnamed artwork identities, or the game
purpose of actor-type-specific fields. Those limitations are stated at their source
consumers and boundaries. No additional gameplay or disassembly pass was run.


## Round018 current scope and correction

Types 25, 33, 45/46 and children 37/49 now have finite initializer/state/parser,
bounds and class-cache/tilemap explanations. Historical wrapper-only limitations
for these paths are superseded. Natural scheduling/placement, the type24
predecessor's ownership, the writer/timing of $ca03, and allocator safety when its
unbounded slot scan finds no empty slot remain unproved. Neither constructed
calls nor sprite tests establish natural timing, visibility or artwork identity.

The accepted five-byte stream $7a04..$7a2b supplies speeds 0,0,0,0,4,3,2,1.
Existing child-tests rows5/6/7 read $7a7e/$7a7f (0300), $7a7c/$7a7d (0280),
$7a7a/$7a7b (0200) respectively, in effective bank1. Redirect017's claim that
only source speeds0/4 occur is incorrect. Six step bytes remain Unknown because
this commentary pass preserves the structural map; this is a concrete evidence
omission to reconcile next, not a newly tested route or an editorial backlog.
Exact finite rows, values, consumer and bounded recipe:
[follow-up](../analysis/round-018/speed-step-followup.json).

All currently classified source is readable and reviewed. ROM classification is
47,063/65,536 (71.81%); 18,473 bytes remain Unknown. Earlier unknown counts and
pending commentary statements above are historical. No map, ROM, toolkit or
emulator state was changed by this phase.


## Round019 affected dependencies
The historical speed0/4-only interpretation and six-byte map omission are resolved:
source rows produce0..4 and actual saved child reads support the newly classified
steps1/2/3. This correction supersedes older Unknown notes without changing their
historical records. No speed5 domain follows. Companion subtype state1, type31
coordinate spawning and type7 states0/1/2 have finite local contracts; seven
constructed actor calls,20 renderer cases and16 audio calls do not establish
ordinary ownership, scene timing, allocation pressure, natural contact or playback.
Only spawner selector2 is source-produced here; other7fa4 words remain Unknown.
The nonlocal bound helper deliberately discards its immediate return. Type7 artwork
and sound26's audible identity remain unresolved; source explains exact mechanics.
Odd-tile sprite components reuse the last attributes byte. The prior renderer
oracle mistake is corrected locally and is not a toolkit defect. Persistent31ee
and spawned741b eight-byte seeds now have classified source-backed split-copy roles.
All remaining Unknown intervals are explicitly bounded; current-source readability
has no deferred range. Whole-ROM classification remains incomplete (17622 Unknown).


## Round020 finite terrain/marker machines
Type6 states0..4 and type8 states0..8 have source-produced finite local graphs.
Comments name exact latch, delay, FF marker, probe-class and motion-field mechanics;
they do not identify enemy artwork, natural placement or contact timing. The type8
contact path writes horizontal magnitude0200 and leaves +07 untouched: the historical
redirect prose claiming +07 was cleared is contradicted by `LD [HL+],A; INC L`.
Animation36 timer is5; animation42 timer is7. Counts are renderer invocations, not
calibrated time. Sound12 is a finite channel1 stream, with unresolved audible identity.
Unknown neighboring tables/code are bounded explicitly; adjacent5c2b is not selected
by these state/call graphs. Other wrappers need their own source-produced state domains.
No current-source readability range is deferred. Whole-ROM classification remains
incomplete:17127Unknown bytes. Historical constructed routes remain local contracts.


## Round021 correction and pending scope
CALL1a54 targets1a6f after POP1a6e; ordinary state-setter RET appears to return
to Unknown1a57. Prior nonlocal assertion is contradicted. Keep map unchanged until
bounded structural follow-up; see analysis/round-021/barrier-correction.json.
Round025 supersedes the former withheld width/tail claim: width358c is53, and
25 newly accepted sparse cells complete265 consumed cells in a275-byte envelope.
The transition bound remains48*5; natural ordinary nested1 execution remains unproved.
Runtime17 parent-window logic checks occupancy, not runtime parent identity.
Full commentary still pending; exact inventory analysis/round-021/remaining-review.json.

## Round021 alternate actor static caveats
- Runtime21 normal retirement6b3b increments C431. Initial20 is a replenishable
  spawn quota, not a fixed total20spawns. Which normal removal paths deplete quota
  remains a gameplay question; no runtime tests were performed during commentary.
- Runtime22 state5 gate6dae changes A/HL when6ddb is called. Unmatched spriteX7
  also satisfies the final [HL]==7 and tail writes X0/Y32 instead of count0/delay32.
  This exact static path is explained in source; ordinary occurrence is unproved.
  A bounded later check could compare matchedX48 count6 with unmatchedX7 under
  the same actor pointer, recording registers/fields without inventing natural entry.
- Runtime22 state6 at6df8 tests predecrement1c via OR A; initial32 acts on call33.
  Do not simplify this into an ordinary decrement-to-zero delay.
- Runtime20 states7/8 alias existing handlers; no local seed installs those values.
  Their natural entry is unproved. Sparse seeds end in fields15/1c (flags/countdown).
- Actor9 primary-Y helper6f6a has a zero-displacement low-byte conditional branch.
  Field11 comparison does not resolve equality; only sprite-Y comparison changes mode.


## Round022 superseding resolutions and current limits
The earlier pending-round021/Unknown1a57 notes are historical. CALL1a54 returns
ordinarily to classified1a57; its jump initializer ends with POP1a6c discarding
the CheckA caller. POP1a6e is a separate entry skipped by that CALL. Direction
left clears sprite bit5. All ordinary tail gaps, selected animation1/3/4/7/60
graphs, ground/air action tables, runtime1 dispatch word and two nine-byte seeds
are now explained. Eight type7 sprite headers use ignored second byte05, which
does not establish actor shape5. Shape12 contact height is literal192.

Incoming contact requires shape bit5, separately from auxiliary bit6. Amounts
use post-conversion runtime7/15/17, not direct initializer41/42/43. These local
static and prior constructed observations do not establish natural contact,
artwork, timing, visibility or late-stage routes. Other unknown selectors,
nested1 ordinary width/tail, stage3 flag meaning and dynamic45d6 prerequisites
remain as recorded. Earlier pending-source statements/counts are historical;
current commentary scope is tracked in round022 artifacts. Whole-ROM
classification remains97.55%, with1605Unknown bytes; no classification changes.

## Round023 retained mechanical limits

Serial routines do not prove a working link feature: the software bit1 handshake
writer and natural callers are absent. DEC remaining/JR Z repeats only when zero;
SC-busy helper returns immediately. Palette selector3 shares c016 with serial count;
final rotation count0 wraps256 iterations. No intended caller domain inferred.
Animation29/37 and sound4/25 natural selection/playback remain unproved. New pitch
loops close under ordinary timed-note cursor0/count0, excluding arbitrary inherited
255. Explicit transition high-E0 stop differs from placement E000 coordinate bound
under DE<2000. Descriptor output contexts/type34/35 setup structure do not prove
natural gate success or occupied child scheduling. Remaining636bytes retain honest
Unknown status. All are explained in source; no readability range is deferred.

## Round024 credit extent correction

Current remaining Unknown total is607bytes/82ranges. The final credit pointer and
count3 block now have static structural proof. State6 phase1 draws index8 BEFORE
its phase17 exit, rather than first drawing it in a later state; state9 phase36
erases retained8. Destinations9b45/9b89/9bc7 contain9/2/6 tile bytes; final FF ends
at physicalad35Code. Natural ending reachability/artwork identity remain unproved.
Earlier historical totals are superseded, not additional current Unknown ranges.
