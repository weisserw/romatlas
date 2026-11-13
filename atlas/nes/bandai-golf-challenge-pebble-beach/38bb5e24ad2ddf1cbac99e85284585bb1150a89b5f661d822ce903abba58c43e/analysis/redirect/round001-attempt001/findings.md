# Redirect round001 attempt001

Input pass-001-final; current-source gate accepted (84 regions,261 units,no unread gaps).
Fixed CPU PRG slots [0,1,2,3]. CNROM changes CHR, not PRG. No map/state/source edits.
Initial hashes and exact empty unknown complement: input-checkpoint.json and unknown-complement.json.
Free disk 6.6GiB; existing analysis/scratch80KiB; no new capture/trace scratch.

## Connected setup correction — R001-R01 ready

Physical0556..0557 (offset1366,length2), CPU8546..8547, bytes44 6A.
Source game.asm219..239 and276: after incrementing shotPhase, load from8546,X,
TAY, then store menuSelection to0000,Y. These are RAM destinations44/6A,
not coordinates. Source symbol SetupChoiceDestinations=8546; local declaration
SetupChoiceDestinationBytes. Correct map/internal-state label to
SetupChoiceDestinations and evidence to destination semantics, retaining Data/Static,
identity/extent/views/confidence. Independent disassembly review must apply it.
No new code or natural execution claim. Consumer and menu continuation inspected;
no conflicting boundary or additional setup defect found.

## Connected sound bounded schemas — R001-R02 ready

The broad generic tail already has sound-role evidence, but three exact nested
structures now have independent consumer/boundary constraints:

* physical3261..3278, CPU B251..B268: 12 little-endian period words. PlaySoundNote
  masks low nibble, rejects values>=12, doubles accepted semitone index and reads
  B251,Y/B252,Y. Consumer shifts word by octave, writes APU period; words are
  [1710,1614,1524,1438,1357,1281,1209,1141,1077,1017,960,906]. This proves the
  consumed table span, not historical generator or natural semitone domain.
* physical32D1..3328, CPU B2C1..B318: 22 four-byte descriptor records. StartSound
  computes B2C1+4*A, reads slot,mode,stream low/high; high OR20 is literal.
  All22 slot offsets are multiples15hex belowA8, modes0..3; decoded stream bases
  begin B319 exactly after descriptor22 and increase toB64C. The immediate15hex
  bounce sound selects last record; title selects0..3 through returnA+1 helpers.
  Exact88-byte extent follows stride, last supported record, first stream boundary,
  and consistency of every record. Do not claim every descriptor naturally selected.
* physical3329..3664, CPU B319..B654 inclusive: 22 streams, each four-byte header
  followed by two-byte events, except terminal FF is a single consumed byte.
  Bases/endpoints retained in sound-verification.json. First6 streams have a B0,02
  loop followed by FF immediately before next header; FF is not claimed reached.
  Remaining16 have terminating FF at last event, next header begins next byte.
  Final FF B654 is first byte of previously noted uniformFF run: do not incorrectly
  treat that entire run as exclusive fill. Stream boundary proposal ends at B655
  exclusive; remaining uniformFF starts physical3665 for this refined interpretation.

verify_sound.py independently checks bounded abstract cursor/marker paths. Counted
loops explore jump and fallthrough, not exact repeat timing; no runtime execution
claim. Max1024 states/stream, bounded B319..B6FF address fence. No out-of-range
path. An initial exploratory fence at physical3664 excluded last FF; corrected
fence recognizes FF single-byte consumption. This was a test-boundary mistake,
not a ROM inconsistency. sound-static-paths.json is supplemental; reproducible
sound-verification.json and primary source are decisive. Recommended single combined
Data refinement preserving generic gaps3279..32D0 and3665..8009 unless separately
supported. No Code/Data counts change. Sound stream IDs/names remain mechanical.

## Independent avenue assessment and honest limits

Read current uncertainties, source relevant consumers, prior data-inspection and
structural-review evidence; searched all source uncertainty markers. Broad generic
blocks are incbin declarations, with consumer descriptions and bounded window
review already available; not skipped merely because generic. No explicit unread gap.

SetRenderingMask CPU8B2B..8B2F has complete five-byte contract. Source contains only
its definition, no symbolic direct call. Existing dispatch/entry review provides no
missing concrete source caller. Repeating arbitrary execution cannot establish a
natural caller; retain static classification and uncertainty without another lead.

Terrain prefix/unused selector extents: TranslateTerrainTile conditions and index
transformations inspected; ReadHeightSample takes byte from dynamically selected
strip and indexes82D4 without clamp. These do not independently bound every original
lookup subdivision. Existing generic Data remains supported; no new conflicting
fetch/boundary. Global CHR/strip-value enumeration would require source provenance
and complete copy-domain proof beyond current consumer constraints; no promising
classification contradiction identified. Do not require it for generic Data.

Third ball sprite: inspected AppendBallSprites through count selection. Third Y is
3A minus height scaled4, common X, tileF9; two ordinary sprites plus third only when
zoom=0 and suppression=0. Mechanics already fully described. A tile's original
intended display name does not follow from arithmetic; arbitrary height edits add
no discriminating name evidence. No new source/map defect.

Physical units/spin/rounding/original generators: mechanical widths, shifts,
saturation and effects are explicit; no independent calibration to choose original
units. Fitting generators would overstate intent. Wide division's unusual high
subtraction is literal; preserve consumer-restricted contract, no generic algorithm
repair. RAM45/81 original intent likewise does not affect readable mechanical roles.

Remaining sound ramps/envelope/vibrato tables use additional pointer/index paths;
not assigned full record domains from legal pointers alone. Their consumers already
explain mechanisms, generic Data is sound; no concrete contradictory extent or needed
behavior experiment identified after the connected stream graph closed.

Progression checklist reviewed: title,menus,first/later9/final18 courses,club choice,
flight,putting,terminal scorecard->title have actual evidence, with artificial patches
explicit. No unvisited later/ending shortcut remains. Terrain collisions/two-player
variants are representative permutations of classified/commented graphs; no specific
new missing structural/behavior contract justifies redoing them. Natural manual
swing timing failed previously; no timing-specific new evidence to reopen it.

This is a productive ready-lead handoff, not terminal exhaustion or final acceptance.
No toolkit issues found. Next phase independently reviews both ready leads together,
then commentary reconciles source and conducts required readability/fidelity gates.
