# Round 2 localized rejection review

Input pass-002-reviewed / round02 completed commentary. Accepted 24,588/24,592
(99.98%); proposed +0; exact four-byte complement in unknown-ledger.json.
Fixed NROM effective banks [0,1,0,1], CPU=physical+BFF0 (8000 alias also valid).
This is a reasoning review of unchanged primary constraints, not a substantive
retry of exhausted reference scans. Prior review remains preserved.

## R01-001: [005D,005E), CPU C04D (alias804D)

Claim considered: Static code, an independent no-op return. Decisive primary
[physical/reference artifact](../round01-contract-review/physical-and-reference-evidence.json)
contains byte60 and neighboring bytes, empty decoded references and exact address
word results. Current [source](../../../commentary/source/game.asm:47) independently
shows AdvanceTitleScroll's conditional return at C04C followed by this byte and
InitializeDemo at C04E. The scrolling branch lands on its own RTS; its contract is
complete without C04D. Thus the counterexample is to an inferred *fall-through or
shared return role*, not to all possible code. Negative reference scans constrain
known explicit entries only; computed entry is not disproven.

Combined constraints establish exact isolated bounds, legal RTS behavior and an
adjacent routine's complete return. They do not establish any additional operation,
state protocol or consumer for this byte. Strongest alternative is one non-code byte
between routines. Conversely no repeated/uniform non-code run or consumer supports
generic data for a lone60. A hypothetical subroutine stack is sufficient to execute
RTS but adds no independent structural evidence. No actual caller is required as a
universal rule. Additional independent role/entry/consumer evidence could decide;
forced PC execution or repeating the same scan cannot distinguish these models.
Retain unknown, without claiming dead/unused code.

## R01-002: [3485,3488), CPU F475..F477 (alias B475)

Claim considered: Static code tail trampoline JMP ServiceAudioFrame(F493).
Primary [bytes/reference/record artifact](../round01-contract-review/physical-and-reference-evidence.json)
shows4c93f4, final ladder pointer F44D, thirteen triples and FF at physical3484.
Current [ladder consumer](../../../commentary/source/game.asm:3056) checks FF before
reading a record and advances three bytes. Current [audio source](../../../commentary/source/game.asm:6173)
places ResetAudioEngine at F478 and ServiceAudioFrame at F493; the latter has a
balanced callable RTS contract. These independent constraints exclude these bytes
from the selected final ladder record, establish a plausible target/boundary and
make a service wrapper credible. They do not prove that every non-record byte in
this gap is code. Counterexample scope is only the ladder-record interpretation;
negative address/reference searches do not refute computed or unused entries.

Strongest alternative is three non-code bytes outside that terminated record and
before the audio engine. No known framing or repeated non-instruction pattern
supports *classifying* that alternative either. The JMP's target and tail contract
are a single operation: copying the target's rich semantics back onto the JMP does
not provide a second candidate-specific constraint. Module adjacency strengthens
plausibility but leaves the code/data alternative materially open. A live caller
is one possible discriminator, not a prerequisite; an independent encoded entry
contract or meaningful repeated wrapper layout would also suffice. Neither is
present in saved primary/current source evidence. Arbitrarily executing it tests
JMP mechanics only. No justified different experiment or narrower supported slice
survives (splitting the operand bytes supplies no independent data role).

## Changed evidence, alternatives and frontier

[R01-003 acceptance](../../round02/review.md) does not weaken these standards:
that eleven-byte slice combines a state-specific test, independently established
mask-helper preservation/carry semantics, the adjacent matching guard protocol,
two exact loader convergences and balanced refusal/loader exits. Residuals have no
analogous multi-operation semantics. Current commentary adds names and corrects
initialization prose/aliases; it adds no C04D/F475 incoming edge, consumer, framing
or new bank constraint. Reusing the prior localized review is sound.

Other explicit [commentary questions](../../../commentary/uncertainties.md) were
assessed against the prior source-grounded [connected investigation](../round01-contract-review/findings.md):
negative audio IDs remain unproved header-domain speculation and offer no supported
path to either residual; ingredient wrappers' exact fallthrough is already code;
remaining data meanings do not reopen classified structure. Four metadata corrections
are now accepted, so no pending ready continuation remains. No new whole-ROM survey
or arbitrary selector sweep is justified.

[Progression checklist](../../scenarios.json) reviewed: first/middle/sixth layouts,
mechanics and GAME OVER visited; failed early patch preserved. Current completion
source at lines380–400 wraps both player layout selectors7 to1, supporting the prior
absence of an identified ending route, not proving no ending exists. No untried
supported ending/boss shortcut or distinct missing subsystem contract is apparent;
layouts2/4/5 reuse the demonstrated initializer without a new structural prospect.
No startup or milestone replay is needed on unchanged premises.

Terminal recommendation: incomplete, no ready leads, no unfinished productive
frontier. Reopen only when independent evidence or a justified discriminating
method changes these alternatives. Final editorial acceptance remains pending;
current-source readability and byte-identical verification are complete as linked
in the commentary handoff. Coordinator must independently inspect this reasoning
and decisive artifacts before accepting termination. No toolkit/map/source/state,
ROM, gameplay or archival changes were made.
