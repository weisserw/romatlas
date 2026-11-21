# Final editorial acceptance

Final targeted editorial review completed on the pass-004-final whole-ROM candidate.
Reused the four rounds' sound review credit and inspected the full 742-range / 697-unit
inventory for missing review, incomplete source locations and concrete contradictions.
The source was preserved first in analysis/final-editorial/source-before; no regeneration.

Resolved findings:
- E004: AllocateSecondPoolRecord at physical14414..14486 / CPU$B83E in effective
  context[0,1,14,15] no longer claims an unknown copy remainder. Its comment and both
  inventory descriptions now identify FinishSecondPoolCopy14486..14506 and the
  established restore tail. Its $00/$01/$10/$11 operands consistently use the existing
  allocator scratch aliases, matching the first allocator and copy continuation.
- The older command-$14 replacement producer now uses its established temporary
  record field names; two projectile emitters consistently name their kind/Y/X
  arguments instead of unrelated clipping-field aliases or raw $48/$49 coordinates.
- Ending actor setup and expansion at103271..104151 now consistently name the scoped
  sprite Y, selector, world-X pair, metasprite pointer and OAM cursor. Their meanings
  were checked against the producer/consumer instructions and ChooseEndingOamGroup;
  unrelated users of these reused scratch locations retain their separate meanings.

Acceptance scope and limitations:
The uncertainty/comment scan, address-only routine-name scan and known numeric-alias
scan were checked against prior source-lead review and the affected code. Generated
branch suffixes retain descriptive routine prefixes. Numeric scratch in unrelated
map, mapper, reset and decimal/HUD code is scoped differently; aliases for object
records or spawn templates must not be applied there simply because addresses match.
Existing short algorithm/format comments and meaningful units remain valid. Generic
storage retains bounded consumer/format inspection and honest unknown subdivisions;
per-byte semantic speculation is unnecessary. No new missing meaningful structure,
wrong code boundary, contradictory routine name or unsupported purpose claim found.

CompareAccumulatorWithDf remains a mechanical A-to-N/Z/C helper with preserved
A/X/Y and an ordinary return frame; caller, intended sharing and natural reachability
remain unresolved. SHX$9E,$00,$07, selector Data103810..103813, ending dispatch, odd
sound-rule trailer, audio pointer bounds and documented natural-ending/second-loop
limitations retain the earlier corrections. No inferred live caller was added.

Checks in analysis/final-editorial/checks.json compare canonical instruction operands,
control flow and explicit width prefixes to the uniquely preserved source. All frozen
effective .org views match; all source locations were refreshed and checked against
physical headings. The complete scaffold and all697 meaningful unit identities remain
intact and reviewed. Final audit performs a fresh exact-byte rebuild and refreshes the
reading copy; its actual result is recorded below after execution. Semantic quality
was reviewed here independently of that mechanical audit.

No classification, map, ROM, toolkit or frozen-baseline changes. No cleanup, deletion,
archive or publication. Whole-workflow completion remains the coordinator's decision.

Supported final command `python3 /root/romatlas/scripts/commentary.py audit .` passed:
byte_identical=true, editorial_audit=true,742 reviewed regions/697 reviewed units.
reading-after.asm refreshed. Source SHA256
002f17948e94bfaf4120f484ab222696ee47913073870a3c71a5556d64fbf918.
ROM, current state/map, frozen baselines and binary assets remain unchanged.
No outstanding editorial action; ordinary semantic/reachability limits remain above.
