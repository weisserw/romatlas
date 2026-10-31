# Coordinator rejection review and research stopping decision

Research may stop structurally incomplete, subject to final editorial, metadata,
fidelity and orchestration acceptance. This does not classify the ROM complete.
The exact residual is six bytes: physical `[245776,245779)` in fixed bank 30 at
CPU `$C000`, and `[255289,255292)` in fixed bank 31 at CPU `$E529`.

Inspected the [worker review](../redirect/round002-20261031-a/rejection-review.md),
[avenue results](../redirect/round002-20261031-a/results.md), current annotated
source neighborhoods, raw input bytes, current map, current references, descriptor
representation/equality evidence and exact ledger. Independently recomputed direct
control targets and raw-word context in `rejection-primary-check.json`.

## Reset-shaped prefix

`4C 98 FE` at `$C000` is exactly bounded by a physical bank start and independently
classified following resources. Its target is the established reset routine.
Those facts favor a veneer. They do not establish an executable bank-start header:
the other banks' four-byte identity/header and API patterns do not apply to this
fixed-bank prefix, and the hardware vectors already name `$FE98` directly.
The preceding uniform bytes end at another bank's boundary and cannot make this
prefix data. The resource consumers do not establish membership of these three
bytes in a non-code structure. Code veneer versus embedded header/resource bytes
therefore remains a material distinction.

## State-setter-shaped gap

`4C 14 E6` at `$E529` targets the accepted state-$20 setter. State-handler neighbors
and the target's calling contract favor an orphan veneer. Actual source confirms
the preceding `$E526` instruction jumps unconditionally to `$E619`; it is not a
return as an older review described. The local transition's conditional outcomes
already bypass this gap, so they supply no fallthrough or finite selector binding.
This observation only rules out that local fallthrough claim; it does not prove
the bytes dead or non-code. No bounded data format establishes the alternative.

The accepted static utilities had additional API/data-flow/arithmetic/template
constraints and independently anchored bounds. Applying their acceptance to every
single jump with a coherent destination would discard that independent evidence
requirement. A known natural caller is not a universal prerequisite; the deficit
here is a structural discriminator for either claimed role. Arbitrary PC execution
would reproduce the known destination contract rather than supply that fact.

## Primary scope checks

The independent check found 4,322 classified contextual direct branch/JSR/JMP edges
and no direct target at either residual CPU alias. This is not proof of universal
unreachability. The apparent external `$E614` word at physical 251294 crosses the
`BCS` displacement `$14` and following `INC` opcode `$E6`; it is not a pointer.

The worker's raw-word absence statements are PRG-scoped. Whole-file checking also
finds coincidental CHR image matches (including one `$E529` word); these are image
bytes, with no supporting CPU entry/copy/consumer contract. They do not reopen a
code lead merely by matching an address. PRG `$C000` raw matches are in classified
Data, not independently established executable pointers.

## Descriptor avenue and other continuations

Inspected the actual `$D0D2` calculation and `$D40E` composite consumer. The odd
descriptor byte is a base plus category/phase arithmetic. The composite lookup
uses the resulting full byte directly through the initialized pointer planes;
there is no index mask or automatic per-type alternate table in that path.
The raw type-25 pair at physical 121295 is flag `$41`, base 199. At phase zero it
produces 199 or 200 for the two category-bit cases, outside both bank-14 spans
181/189. This falsifies blanket all-types semantic validity under the tested
contracts, not the independently supported paired Data layout. It establishes
neither natural selection nor dead/unused types. Shared descriptor provisioning
and narrower local graphics remain compatible with restricted use.

The avenue was investigated through its real representation and consumers, rather
than inheriting the failed raw-byte criterion. No selected-type route, observed
defect, changed resource contract or connection to either residual yields a useful
next discriminator. Artificial insertion repeats the arithmetic already known.
Optional song/graphic identities and original mixed-data subarrays do not prevent
generic Data completeness or current-source mechanical explanations. Accepted
dispatch/copy/ending/utility evidence and representative gameplay, including
scrolling credits, remain reusable; unchanged replay has no new discriminating
prospect here.

Each residual was evaluated separately. No counterexample was broadened to reject
another interval, and no missing natural caller was imposed on generic Data or
the accepted utilities. No useful corrected inference or concrete test survives
this review. New independent entry/template or bounded-data evidence may reopen
the corresponding claim. Current lead histories contain no ready structural
proposal. Stop research incomplete; proceed to separately assigned final editorial
acceptance, preserving the six Unknown bytes and all evidence.
