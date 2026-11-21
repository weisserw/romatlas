# Static code and inline data contracts

Input pass-003. No annotations applied. Proposed bytes remain unknown until independent
disassembly review. Candidate regions are in proposals.json; source of every graph
instruction, including accepted predecessor/root or direct JSR, is in graph5.json.

## Supported entry edges, beyond legal decoding

Accepted source Code provides instruction boundaries and recorded effective PRG slot
views. Non-call fallthrough, conditional branch and direct jump connect unknown bytes
to these instructions with unchanged context unless decoder reports a bank barrier.
Direct JSR provides target entry in the pre-call view; its continuation is NOT assumed
unless the separately audited contract below applies. Each graph node records the
specific supporting edge. Finite dispatch roots are derived from selector arithmetic,
loads, pushed words and RTS semantics, not merely plausible pointers:

* Bank8 $8006: ASL then CMP #$48 admits exactly even X=0..70. Thirty-six LE return
  words at physical66597..66669 ($8415..$845D); high then low pushed, RTS adds1.
  High-bit original selectors alias through the shift; no natural selector domain
  claim. All36 handlers inspected through their initializer calls and returned
  continuations; repeated score blobs occupy gaps between these supported handlers.
* Bank12 $9A3A: initial zero returns, AND#$7F then CMP#$1F permits0..30 (zero from
  raw$80). Thirty-one words physical105059..105121 ($9A53..$9A91). All31 entries,
  shared helpers and supported continuations in graph5, not a subset of actors.
* Fixed bank14 $C50C: CMP#$06/BCS fallback; ASL -> X0..10. Six LE return words
  physical116011..116023 ($C51B..$C527). Distinct entries and all connected supported
  continuations inspected. Table starts/end independently bracket code.

## Return and bank contracts

normal-return-contracts3.json records199 conservative normal-return contracts under
explicit incoming effective banks. contracts.py/contracts2.py/contracts3.py reproduce
bounded control/stack proofs: inspect every reachable instruction, balance PHA/PHP
against PLA/PLP; every RTS balance0; reject TXS, RTI/KIL, indirect JMP, unknown indirect
writes, possible mapper/stack-page writes and unproved nested calls. Zero-page indexed
writes wrap within zero-page; absolute indexed writes retain full possible +255.
False rejections are retained as barriers, not conclusions of dead/noncode status.
Queue calls use the independently audited inline contract below. Recursive/infinite
paths are not claims of termination. No callee-return bank propagation across arbitrary
JSRs; mapper wrappers, nonlocal yields and unsupported stack effects remain barriers.

$CEDF is STA $0010,Y; RTS. Its seventeen caller sites visibly have LDY#2/#3/#4;
LDA#0/#1; JSR. Under this preceding LDY contract the store cannot touch the stack,
no bank changes occur and continuation is ordinary. graph5 records these call-site
preconditions explicitly. A universal arbitrary-Y contract was NOT inferred.

Sound veneers$81FE/$8205/$820C set X/Y then JMP$8217; unknown noise veneer$8213
is LDX#$E7; LDY#$0C and falls into$8217. Common initializer JSR$C4C3 reads caller's
saved frame at SP+3/+4 (skips its own inner JSR), reads return+1 and+2 into$00/$01,
then adds2 to that saved return. Thus every direct sound-initializer call owns exactly
two adjacent Data bytes and resumes at call+5. Priority rejection happens after the
word extraction and cannot bypass this consumption. Result is a phrase pointer;
no complete phrase extent inferred from one selected pointer.

Fixed$FA04/$FA13 call$F922, the same outer-frame two-byte reader. Their own caller
inline word is exactly2, irrespective of row/control data length. Queue capacity
helper$FA6E on overflow PLA/PLA removes its own immediate return frame then RTS
returns directly from the queue helper; queue helper's caller return remains intact.
Thus row helper normal and overflow exits both consume2 and preserve mapper banks.
$C4E9 includes an internal FA04 call with inline$0084; this internal word is not
an inline operand of C4E9's caller. No blanket call+3 rule crosses these operands.

## Local counterexample / explicit exclusion

Raw graph5 contains one rejected byte: physical98662 (bank12 CPU$8156) decodes KIL
and overlaps the last byte of supported LDA $0203,Y at physical98660. Its sole graph
edge is fixed CPU$D68F JMP$8156 (physical120479) under [12,13,14,15]. A state/bank
relationship is missing on that branch (possible bank10 entry alias); this does NOT
support executing bank12's operand as code. proposals.json EXCLUDES this one-byte
candidate; upstream fixed code remains structurally coherent, and the banked transfer
is unresolved. No dependent continuation exists because KIL is terminal. This is a
localized rejected target, not a rejection of the independently supported caller or
all graph results. Official decoding/no overlap are checks, not primary code evidence.

## Scope limits

Structural Static proposals do not claim natural execution, complete bank/game
compatibility, original source layout or exact game meaning. Actual fetched roots
from ending/middle experiments are separate artifacts; edits alone establish nothing.
No map, source, binary or ROM changes in redirect. Disassembly must review the
contracts, bank aliases, inline barriers and proposal intervals independently.

## Corrected bank10 alias continuation (final graph7)

The bank12 KIL rejection above is localized. Fixed$D68C reads$0462 then conditionally
JMP$8156. Accepted bank10 initializer$8221 sets$0462=1, fixed initializer$C624 clears
it, and confirmed boss snapshot has effective[10,11,14,15]. Under this explicit boss
entry contract, bank10$8156 is the third member of an independently observed JMP
veneer group$8150/$8153 and JMPs$905A. This is the appropriate bank alias, not the
rejected bank12 operand. graph7 adds exactly this supported conditional entry and
follows it: save X/Y in$84/$85, increment zone-enable bytes$0465/$0466, queue two
bounded rows through$9073, restore X/Y and RTS. $9073 itself is LDA#$20; JSR$FA13;
inline word$907B; RTS, so its normal/overflow bank-preserving return contract is
already covered by the audited queue-helper contract. Connected continuation closed;
no arbitrary roots for other veneer members were invented. Final package11226Code,
6189Data=17415proposed new bytes. Raw graph7 retains the rejected bank12 KIL, while
proposals excludes it. All candidate overlap checks use this filtered package.
