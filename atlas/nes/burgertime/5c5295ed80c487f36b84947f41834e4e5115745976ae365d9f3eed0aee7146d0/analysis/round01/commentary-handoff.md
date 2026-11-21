# Round 1 commentary handoff — current-source readability complete

Checkpoint **commentary-round01-final**, based on structural **pass-001-final**.
[Fresh verification](commentary-final-verification.json): byte-identical rebuild
passed; [reading copy](../../commentary/reading-after.asm) refreshed afterward.
[Maintained source](../../commentary/source/game.asm),
[symbols](../../commentary/source/symbols.inc),
[review inventory](../../commentary/review.json),
[progress](../../commentary/progress.md).

Reviewed full current inventory: 61 code intervals / 11,296 bytes, 48 non-code/image
intervals / 13,281 bytes, three explained unknown intervals / 15 bytes. All 112
scaffold entries and 322 compact units reviewed. Source includes 274 routine/block
contracts, 727 control labels and 387 address symbols. Every meaningful subsystem
has purpose/control/data-format explanations; shared scratch and differing audio
field meanings are explicit. PRG data/unknown bytes are visible as byte/word
records, dispatch words name handlers, and CHR remains a bounded local tile asset.
No readability ranges remain deferred. Final editorial/archive audit is **pending**;
this does not declare full-ROM classification or hardware/game compatibility.

Examples: NMI handshake/PPU mailbox ordering; software three-byte BCD scores and
extra-life thresholds; stage modulo-six versus saturated difficulty; ingredient
record/state/queue mechanics; enemy direction priorities; music/effect interpreters
and precise processor flag contracts. Code was read across the complete generated
inventory and checked against saved decode/physical dispatch words; no gameplay
replay was needed for this editorial phase.

Structural residuals unchanged: [005D,005E), [3485,3488), [3609,3614), tracked as
R01-001..003. Unknown bytes were preserved and explained, not reclassified.
ROM classification remains **24,577 / 24,592 (99.94%)**. Map/state SHA-256 remains
75d8105e19a68e00cad655743bcbc11b76ec970b0aad9b97fafe83352f60c04f.
Input ROM hash remains the workspace identity.

Read [uncertainties and redirect questions](../../commentary/uncertainties.md).
Concrete follow-up evidence:

- Audio request carry survives ASL(request-1) through BIT(header); BIT provides
  N/V only. Use this precise contract when reviewing the eleven-byte priority
  alternate path and negative request IDs, without inventing a caller prerequisite.
- Selector-one/two ingredient wrappers fall through to selector three because
  their BEQ is false after nonzero LDA. Source explains the actual row-Y update;
  intended animation semantics remain tentative. No byte repair is proposed.
- Reconcile stale phase-1 semantic annotations at physical 1278..128E (five pepper
  offset pairs plus twelve direction remaps), 0B7B..0B7F (extra-life initialization,
  not demonstrated enemy pacing), 26B6..26BE (eight pepper stun durations with
  one-based table-minus-one addressing), and 0F74..10F4 (bonus stride eight versus
  separate player-mode stride seven). Physical boundaries/non-code classification
  do not change. Original evidence remains preserved; corrected source is explicit.

Other unresolved meanings are documented locally: two movement constants, duplicate
ingredient selectors, some internal layout/audio stream extents, fill purpose,
negative audio request domain and CHR picture meanings. These do not leave source
mechanics unexplained or generic non-code unclassified.

No toolkit issues found; [report](../toolkit-issues.md). No toolkit modifications,
archive, publication, cleanup or redirect execution. Frozen commentary baseline and
historical evidence retained. Disk 3.3 GiB free; scratch 80 KiB, commentary 1.5 MiB.
