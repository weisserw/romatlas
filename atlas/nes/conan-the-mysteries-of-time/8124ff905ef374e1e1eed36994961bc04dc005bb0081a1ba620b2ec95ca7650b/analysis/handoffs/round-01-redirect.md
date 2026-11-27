# Round 01 redirect handoff

Status: combined ready batch for independent disassembly. Research complete for
this pass; ROM classification remains incomplete. This is not terminal exhaustion
or an accepted classification result.

Input: [current-source commentary](round-01-commentary.md), frozen
maps/round-01-disassembly.json. Input SHA256 unchanged, current map/state byte-equal
to checkpoint, configured binary unchanged. Accepted counts stay19055/131088
(13531code,5524data),112033unknown. No map/state/source/ROM/toolkit edits.

Attempt: [contracts and data evidence](../redirect/r01-connected-20261127/data-contracts.md),
[engine/stack contracts](../redirect/r01-connected-20261127/residual-contracts.md),
[exact proposals](../redirect/r01-connected-20261127/proposals.json),
[exact unknown ledger](../redirect/r01-connected-20261127/unknown-ledger.json),
[research verification](../redirect/r01-connected-20261127/research-verification.json).
Leads R1-Q01/R1-Q02 updated ready; R1-L04 added, previous histories preserved.

Combined proposed gain112032 previously unknown bytes:13318code,98714data.
No kind conflicts with accepted map or between proposals. Hypothetical residual1
at physical1CAF1 / CPUCAE1. These are proposals, not annotations or observed fetches.
All statements remain subject to independent disassembly review; reject/localize
unsupported subclaims rather than counting the aggregate as accepted.

## Connected and independent findings

- Stage records:42seven-byte records, physical1BB9B..1BCC1 exclusive, bounded by
  cursor0,+7,while<0120. Final offset011F and full7bytes end at loaderBCB1.
  All37distinct task initializers inspected, not merely selected encounters.
  [Handler CFG](../redirect/r01-connected-20261127/handler-cfg-v2.json) extends
  through direct callees, branches, inline task words and immediate42/43 task
  producers into11329instruction starts. New earlier evidence and scripts remain
  preserved. Fixed-to-lower edges were not guessed. Mapper-affecting helpers have
  map-bank/image-bank temporary access and explicit bank6 restoration; bank6
  remains a supported calling precondition elsewhere, not a reachability claim.
- Stage maps: all8rooted containers fully pack layout(width*height),26-byte
  tile/palette definitions and pointer tables to next rooted layout. Every pointer
  aligned; every definition referenced; no holes; palette values0..2. Last table
  meets existing FF run.26bytes comes from25tiles plus offset25palette, not a
  guessed stride. [Audit](../redirect/r01-connected-20261127/stage-container-audit.json).
- CHR/screens: eight stage counted streams plus title/inventory/terminal roots
  terminate within selected physical banks. Partial final literal payload is
  distinguished from bytes actually read. Title/terminal/extra consumers and
  exact token walks are in screen-and-extra-chr.json and stage-chr-streams.json.
- Animation/image bank3:246pointer entries; complete zero-terminated five-byte
  sequences plus383dimensioned images exactly tile81EE..B5A8 with zero cross-role
  overlaps or holes. Independent terminal CHR rootB5A8 closes the container.
 118immediate action producers supplement structural evidence; no claim of full
  natural selector domain. [Container](../redirect/r01-connected-20261127/animation-container.json).
- Raw CHR: bootstrap4096-byte copy plus image-derived cache keys establish all128
  mapper4groups and94mapper5groups, each128bytes;28416bytes total. Remaining
  mapper5tail up to rooted inventory stream is already uniformFF. Every group
  has primary image-pair references in chr-tile-groups.json; no arbitrary sweep.
- Other avenues investigated in same pass: all21music streams/three7-entry root
  tables; eight stage palette pointers/sevenunique streams; numeric/coordinate/
  action tables referenced by connected handlers; bounded DMC descriptor/sample
  backing throughCAA5; standalone coherent API wrapper/stack/decimal/image/text/
  audio contracts. These are bundled, not reserved for independent later passes.
  Generic non-code fallback is used where identity or exact subtable meaning is
  unresolved, after consumer/pattern/boundary review. residual-references.json
  includes CPU aliases: distinguish lower-bank references by actual bank context.

## Corrections and independent verification recipe

Read data-contracts.md and residual-contracts.md before applying proposals.
Decoder commentary reverses repeat/literal classes: atCDCD source stays fixed
through the run and increments afterwards (01..3Frepeat); atCDEB source increments
every emitted byte (81..FFliteral).40..7Fzeros,00/80skip. Fix in next commentary.
Additional crucial inline barrier: physical1CEAF..1CEB1 (CPUCE9F..CEA1) is pointer
05A7 consumed by callerJSRD360->D611; the apparent LAX opcode is data, not code.

Recompute bounded scripts in attempt directory if needed: research-v2.py (at most
18000starts, one instruction per decode), assets.py, asset-audit.py, animation.py,
other-assets.py, tile-groups.py, residual-review.py, residual-decode.py, proposals.py.
finalize.py records leads/history and should not be rerun merely to inspect results.
Use configured release binary. Review each exact proposed span, full context
[12,13,14,15] for mapper6engine; asset mapper b uses[2*b,2*b+1,14,15]. Separate
static contracts from observed fetches/natural use. Honour existing and newly
identified inline-word barriers. Generic-data evidence need not prove a particular
original table generator, all selectors, or every semantic identity. Independently
annotate accepted slices, validate/save/checkpoint and byte-identically verify;
then complete the new current-source commentary scope.

## Residual and deferrals

One-byte CAE1 is unreferenced RTS between known NMI tailJMP and IRQentryCAE2.
Primary residual-decoded.txt, current map/source and full CFG/reference review
supply no supported entry or broader routine contract. Valid decoding and absence
of execution are insufficient; exact unknown ledger preserves it. This local
ambiguity does not reject the independently contracted neighboring wrappers.
No concrete improved test for that byte emerged; resolving it may require a
specific reference/entry or independent boundary/data-role fact. This ready-batch
handoff does not assert terminal incomplete; terminal rejection review belongs
after independent acceptance/rejection changes the actual residual frontier.

Other obvious continuations now yield no unclassified targets under recovered
contracts: animation/task producers and all stage initializers included; new
image/text/decimal helpers consume caller/RAM arguments without a known additional
ROM target; music/palette/CHR domains close independently. Natural selection and
actor names remain useful future semantics but do not block structural proposals.
Prior scenarios already visited stages0..7, inventory, title and terminal; reused
that broad progression evidence without repetitive runtime replay. No new execution
coverage claimed. No toolkit issues found; toolkit-issues.md unchanged. Disk~3.4GiB;
no live server, cleanup, deletion, archive, publishing or commit.
