# Round005 actor instruction-pattern review

Research only. Checkpoint maps/round-005-disassembly-final.json/current/state SHA256
289c2300d7444e697915ba512406de08776cdff0d5ddab5749b40510a2118dc4;
source95d55c01310dcf6570dfc279f45016dbd8ecb068a4f94bbb7fa695b20d13a136.
Primary: actor-primary.json (immutable ROM bytes, exact canonical/candidate decode),
dependency-source.txt (current source), and preserved
analysis/redirect/r002-copy-contracts/actor-tail-review.json (actual malformed target
operands). Decoder lengths corroborate boundaries; they are not the role proof.

## Changed claim and discriminating check

Prior r002 whole84-byte rejection correctly falsified a complete contemporary
comparison-handler contract. Prior r004 localized two complete coordinate copies,
but required the remaining fragments to have valid downstream flag-helper contracts.
That requirement is too strong for recognizing machine-code fragments: the needed
claim is Code, not a working current handler, original source grouping, relocation,
old version, actual execution or natural selection. No repair or inferred relocation
is used. Stop Code inference if comparison constants, indexed field, branch skip,
mask setup and call/transfer placement cease to match the independently established
actor instruction family. A lone legal instruction/target does not meet this test.

## P01/P02/P03, 62 Static Code bytes

Physical123199..123229 (30),123239..123259 (20),123269..123281 (12),
CPUE12F..E14D,E157..E16B,E175..E181. Physical8KiB bank15,
effective[null,null,14,15]. These are fragments of recognizable comparison/mask
instruction structure; do not describe their whole handler as functional.

Independent constraints combined:

* Canonical123076..123197 contains independently supported actor handlers and
  shared OR/store/RTS flag helper. It loads coordinate$0428,Y, conditionally stores
  to$06AD,X, tests thresholds60/48, sets A=20/40 and uses JSR followed by JMP to shared
  control paths. Those RAM field identities and mask meanings are established there.
* P01 begins CMP60/BCC+3/STA06AD,X, exactly the seven-byte suffix of canonical
  conditional-copy123178..123185 and the two newly accepted copies. This is an
  instruction-fragment boundary; absent LDA is not manufactured. Under incoming A
  as comparison coordinate, it performs the same local test/store. Branch lands
  exactly at123206, the next CMP48.
* At123206,123239,123269 the same CMP48/BCC+5/LDA20/JSR/JMP layout recurs. This is
  canonical123185..123197 with changed encoded targets. Branches skip exactly the
  mask-load and complete JSR, joining at JMP123215/123248/123278. Field/test/mask
  semantics and aligned local control structure jointly discriminate code from
  incidental numeric bytes. Both latter blocks begin at joins of independently
  accepted coordinate-copy operations, adding an established boundary constraint.
* P01's JSR/JMP pair123218..123224 and LDA40/JSR123224..123229, and P02's
  JMP123251..123254 and LDA40/JSR123254..123259 fit the same call/transfer and
  mask-setup components in this tightly repeated family. The LDA40/JSR components
  end exactly where the independently accepted next LDA-coordinate copy begins.
  Their adjacency, mask value, call width and next field operation supply joint
  evidence; this is not an arbitrary truncation of a decoded byte stream. The
  standalone transfer components are weaker individually but part of the repeated
  bounded actor instruction layout, not isolated jumps elsewhere in the ROM.
* P03 ends with a complete JMP at123281, independently bounded by639 uniform FF
  bytes already classified Data. P01/P02 endpoints are existing accepted Code
  boundaries. All claims exclude the orphan prefix and do not cross Data.

Actual negative scope: encoded JSR E0EA enters byte06 of STA06AD,X;
JSR E10B enters byte20 of LDA20; JMP E102 enters operand03 of BCS+3;
JSR E129 enters operandE0 of JSR E0D5; JMP E120 enters byte06 of STA06AD,X.
JMP E0E1 alone does target a canonical LDA entry. Exact bytes and containing
instructions are preserved in r002 actor-tail-review.json and current actor-primary.
These failures decisively prevent asserting the offered helper results or correct
stack-return behavior. They do not turn the encoded call/branch/mask skeleton into
non-code, nor require discarding recognized instructions until an obsolete origin
can be proved. No invented old helper placement, repaired target, whole-handler
contract, reachability or execution claim accompanies this proposal.

Strongest alternative: orphaned copied/obsolete instruction bytes, or deliberately
stored encoded instruction templates. Obsolete instructions still support the local
Code role without proving an obsolete origin. An encoded-template Data claim has
no observed consumer or independent template-format evidence here; merely possible
storage as Data does not outweigh the field-specific instruction family and exact
branch/established-code boundaries. This comparison uses the same structural
recognition standard as accepted actor copies and service entry families, rather
than demanding exclusion of every conceivable interpretation.

Physical123197..123199=28 04 remains Unknown: missing LDA opcode makes an orphan
operand interpretation plausible, but PLP/NOP-zp overlap has no independent field,
control or entry contract. Its ambiguity is localized to two bytes.

## Other concrete avenues and connected continuations

All actor encoded helper destinations were checked in the primary canonical bytes;
there is no new unknown callee continuation. Existing accepted copies remain
unchanged. Generic Data is not proposed for the orphan or other residuals.

Reuse exact primary neighborhoods from r004 primary-review.json/contracts.md and
r002 actor-tail-review.json; current exact ledger includes18spans/95bytes, contexts
and current neighbors. Actor field/mask/control repetition supplies no new evidence
for the following independent candidates:

*31242..31245 JSR E696: complete instruction and known destination alone do not
 establish a role-specific fragment family or inline/entry contract.
*84533..84536 JMP8B64 and84976..84979 JMP8BAF(F04): isolated transfers remain
 separated from supported contiguous veneer families; matching target alone is
 weaker than the bounded repeated actor instruction skeleton. No new role constraint.
*49276,57360,61891,84594,85147,88076,114661,115706,121465 singleton RTS and
98317..98319 double RTS: opcode/possible return mechanics provide no discriminating
 entry family or meaningful operation beyond a possible exit. Actor recognition
 does not add a constraint to them.
*64986..64989=2CA902: numeric tables end independently before this interval;
 no repeated trailer format. BIT/LDA overlap remains credible code-versus-data
 ambiguity. No new consumer or bound proof permits either role.
*124424..124432=4C78E6EA4C03C4FF: split JMP E678,EA,JMP C403,FF independently.
 First target known but no local family; EA lacks rooted six-byte E672 format;
 second target fails the offered service contract; FF has no uniform-run extent
 and crosses next Code if decoded. The second target failure does not veto the
 first JMP; first JMP target alone does not establish either the remainder or itself.

These unchanged primary judgments are reused; there is no justified runtime test
that would discriminate roles without arbitrary entry/stack assumptions. No
whole-ROM search or exhaustive actor enumeration is warranted by this result.

Broad gameplay checklist reviewed directly: S005 initialized artificial9INBOT,
pitch and GAME SET; S008 defensive MOVE/exit; S009 natural Ultra special pitching.
S006 remains active-widget/parent-transition barrier with actual6007=165; S010
late counter patch did not advance inning. This instruction-family inference adds
no missing Season/UI or live-contact state contract. Thus no unchanged input replay,
arbitrary RAM sweep or rare-interaction forcing is useful here. Later-game and
Season terminal limits remain explicit, not silently counted visited.

## Handoff

Combined ready L010:62 proposed Static Code bytes in3spans,0accepted in redirect.
If independently accepted, remaining Unknown33bytes/16spans (orphan2 plus15other
spans). Current counts remain95/18. No terminal exhaustion claim; full commentary
must follow any acceptance. No map/state/source/ROM/toolkit changes. No new toolkit
issues found; existing TK-001 nonblocking. Disk5.9GiB free; scratch path absent,
no exploratory bulk dumps or cleanup performed.
