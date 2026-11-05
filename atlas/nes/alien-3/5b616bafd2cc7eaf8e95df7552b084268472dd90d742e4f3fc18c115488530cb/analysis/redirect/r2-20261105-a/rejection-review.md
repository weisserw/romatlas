# R2 residual structural review — coordinator decision requested

Input accepted pass-003, map/state SHA256
4e8db6a04eb66e103f84903691c4c75900553daf826f7edf62392b2e1be2082a.
All bounds below are half-open physical file offsets, bank14 fixed CPU C000..DFFF,
representative effective slots [0,13,14,15]. Structural role and reachability are
separate. No annotation is proposed. This review is not a final editorial audit.

Primary evidence: [ROM/graph extracts](primary-review.json),
[current source excerpts](source-evidence.txt). Original graph instruction objects
include physical offsets, contexts, successors and references. Exact bytes are read
fresh from immutable input/game.nes. Historical r1 findings/residual inventory are
retrieved for comparison, not treated as dispositive rejections.

## Method change and its discriminator

Old barrier: plausible isolated JMP/RTS with no direct references. Fresh inference:
combine accepted neighboring control/stack contracts, 37-state player behavior
and paired frame domain, common transition CF65 semantics, and redundant-epilogue
models. A parameterized contract may establish static code without a live caller.
Expected useful result would be a supported local path/schema or nontrivial routine
whose independently constrained behavior identifies the interval as instructions.
Stop this avenue if the models add only hypothetical entry/return or an assumed
source-generation convention. Do not force execution at these PCs: successful JMP
or RTS behavior cannot distinguish the competing interpretations.

## 1D0FD..1D100 — CPU D0ED, 4C65CF

Candidate: static tail trampoline into CF65, entering player state10. Observations:
D0EA JMP CE62 ends exactly at D0ED; CE62 sets idle state/frame/facing and RTS.
D0F0 is separately entered by D095 JMP D0F0; it JSRs CEA6 then sets motion index18
and RTS at D0F7. Neither predecessor can fall through to D0ED. CF65 independently
sets state10, zeroes frame/motion timer, delay5, speed3, then RTS. Its meaningful
state10 model has an actual use at D658; player state10's behavior entry is D10C,
not D0ED. Under an ordinary return-address precondition the candidate would tail
return with the same stack contract as CF65.

Combined constraints make an intentional redundant state10 trampoline plausible:
exact instruction length, real state10 target, location among support transitions,
and second identical stub at D58B. But the length and target still derive from the
same three opcode bytes; location and repetition do not establish a caller-side
state distinction, bounded dispatch record or other independent structural use of
this entry. Counterexamples rule out fallthrough from these immediate routines and
selection by the known finite player table, not every possible static trampoline.
Competing interpretation: retained obsolete instruction bytes/opaque local residue
rather than a defined executable entry. No positive data pattern justifies generic
non-code either. Missing discriminator is structural organization of the fragment,
not natural reachability. Forcing state10 selects D10C and adds no such evidence.

## 1D59B..1D59E — CPU D58B, 4C65CF

Candidate: independent second state10 trampoline. D575..D58A updates motion index,
early release optionally selects14, and both routes converge at D58A RTS. All
landing branches select D58E/D596; these align Y/set frame2/delay5/audio then RTS
D5A3. The unknown is not a required missing return or branch operand. CF65's
state10 contract and identical D0ED bytes independently supply a semantic candidate,
while the matched state table selects D10C for state10. A natural transition to
state10 elsewhere proves that state exists, not that every CF65 byte pattern is
part of a routine. No claim that D58B is dead follows.

The local control model falsifies only a continuation of D575 or D58E and the known
dispatch-entry model. An unused/alternate trampoline remains possible. The strongest
competing interpretation is the same opaque/obsolete residue. A claim of generic
data would merely negate execution without positive non-code evidence. No shared
record stride, count, independent encoded origin or boundary convention connects
the two three-byte stubs. Their identical encoding alone is not a structural model.
Retain unknown independently of D0ED; neither rejection vetoes the other.

## 1D4F5..1D4F6 — CPU D4E5, 60

Candidate: routine return/empty helper or redundant arc epilogue. D4E0 BCC D530
otherwise D4E2 JMP D501; the JMP consumes through D4E4. D501 either selects
D542, JSRs BBE9 and branches to D4E6, or integrates the vertical step and JMPs
D575. D4E6 loads16/stores motion index and RTS D4EA. Thus the accepted rightward
arc branch has complete outcomes and does not need a fallthrough return at D4E5.
The next helper is independently entered at D4E6, not D4E5. Player state0D selects
D48F. No encoded pointer/reference to D4E5 was found in the inspected evidence.

An empty routine with an ordinary return address is a coherent *hypothetical*
contract, but every isolated60 byte has it. Redundant epilogues are plausible in
hand assembly; no independently established source/layout convention here makes
this a specific epilogue. Lack of a caller is not the reason for rejection: absence
of any behavioral, formatting or control constraint beyond the opcode prevents
identifying it as an instruction. The preceding unconditional jump rules out local
fallthrough only. Competing data/residue cannot be rejected or positively established
from one byte. No padding/alignment pattern is claimed. Retain unknown.

## 1DACE..1DACF — CPU DABE, 60

Candidate: redundant weapon-dispatch epilogue or empty return entry. DAB7 CMP02,
DAB9 BEQ DAB4, otherwise DABB JMP DBCE; DAB4 JMP DC32. DAB1 PLA,DAB2 PLA,DAB3
RTS already closes selector7's nonlocal path. DBCE resource firing and DC32
projectile firing close via explicit stack-removal paths (DC2B etc); they do not
return to the byte after the tail JMP. An ordinary JSR into the dispatcher leaves
the caller's return address on stack; removing it exits another level. It does not
create DABE as a return destination. DABF support-Y helper has its own entry and
returns at DAD2. The similar aimed dispatch at DBC1/DBC4 also ends in explicit
projectile/resource tails or its own PLA/PLA/RTS rejection path.

The stack constraint is independent of the bytes at DABE and decisively rejects a
required post-tail return. It does not disprove an independent empty helper or old
unreachable epilogue. An assumed assembler always emits RTS after a tail dispatch
would be a source-generation hypothesis, not an observed convention with extent
proof. Since the only positive role evidence for DABE remains the one-byte opcode,
that corrected narrower candidate remains insufficient, as at D4E5. Generic data
is equally unsupported: no uniform run or container establishes non-code here.

## Independent opportunities and actual deferrals

The finite player-table re-read is bank13 CPU A45F..A4A9, physical1A46F..1A4B9,
37 words matched to the frame domain in prior actor-data.json. No residual is a
selected handler. This negative result is restricted to that table; arbitrary
out-of-domain state edits would not establish structure and are not a useful test.
All15 domains and accepted graph roots were already expanded in r1; no initializer
with an unclassified selected handler emerges from current uncertainties.

Current source/uncertainties have no unreviewed code/naming backlog or missing
callee frontier; remaining resource stream schemas/visual meanings concern already
classified data and supply no new boundary/reference model for these isolated
fixed-bank bytes. Low-slot resource meanings cannot change fixed-bank predecessor
termination. Arbitrary data-reference sweeps or postulated computed addresses add
assumptions without a supported consumer. All four CPU words are absent even as
contiguous little-endian PRG bytes (fresh bounded scan); split/runtime computed
addresses remain possible and are not excluded by that scan.

Progression history/scenarios reviewed: early/later normal maps, first and later
three guardian milestones, final source-defined exit, bonus, escape ending and
terminal final-score screen already have saved observations and reusable recipes.
R1-unvisited is a historical pre-redirect entry; later R1-redirect-* entries close
its exit/terminal questions. Natural final guardian defeat remains unobserved, but
the now-established attack/state/return graphs do not identify a residual consumer
or discriminating interaction; repeating combat has diminished structural prospect.
No ending replay or arbitrary PC mutation is useful for these claims.

No corrected independent role/extent model or supported next discriminating test
survives this review. Recommendation: stop redirect incomplete, subject to
coordinator inspection of this reasoning and primary artifacts. This is an honest
code-versus-data ambiguity, not a request for an execution witness as universal
proof. A new supported computed reference, source/layout contract or bounded
container model would reopen the appropriate interval without invalidating history.

## Acceptance consistency check

Accepted short F05B JMP F346 is not an isolated stub inference: current source
F058 JSR F300 falls through at F05B, making it a required continuation under the
callee's normal return contract. F098 JSR F346/RTS similarly completes F095 JSR
F27E. FD09's no-natural-caller static model begins with JSR F300 and joins common
queue staging; its parameterized PPU preparation contract has distinct helpers and
shared state effects. Accepted uncalled DC26 sets the independently consumed0381
cooldown then PLA/PLA/RTS; accepted occlusion helpers compute a grid cell from
screen/scroll state, preserve X and choose normal versus nonlocal return based on
0600 content. Those independent data/control contracts distinguish purposeful
routines beyond successful opcode decoding. This does not require every routine to
be long or referenced. The residual JMP pair supplies only CF65's *already existing*
contract and duplicate byte placement, without a constrained distinct role for the
new entry. The isolated RTS pair supplies no state effect/domain relationship at
all. A hypothetical original generator could explain them but is unnecessary for
accepted evidence and is not asserted here.
