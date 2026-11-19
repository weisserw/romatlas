# Round 3 redirect handoff (attempt r03-a) - TERMINAL INCOMPLETE recommended

- Input/output checkpoint: pass-003 (maps/current.json, state, source, ROM, binary unchanged). Classification
  100.00% (393229 / 393232 B; 3 B unknown). Proposed bytes this pass: 0. Accepted: n/a.
- Changed records only: analysis/leads.json (L017 attempt + pass record), analysis/scenarios.json (password
  milestone attempt), analysis/toolkit-issues.md (TK-003 addendum), analysis/progress.md, this file.
  Prior attempts r01-a/r02-a untouched. Research dir analysis/redirect/r03-a/ (tools/, evidence/,
  leads.before.json, scenarios.before.json).
- Ready leads: none. L018 blocked (unchanged; no caller exists). No server left running.

## Exact residual (unknown ledger, evidence/residual-static.json)
| Phys | Bank / CPU | Byte | Before | After | Disposition |
| --- | --- | --- | --- | --- | --- |
| 0x14CCA | 10 / $8CBA (ctx 10,11,14,15) | $60 | JMP $9873 @0x14CC7 (Observed, IntroStep12_Text) | IntroStep13_Objects (Observed) | unknown, terminal |
| 0x17BC6 | 11 / $BBB6 (ctx 10,11,14,15) | $60 | JMP BossW6_Hit @0x17BC3 (Static) | BossW6_Fight (Static) | unknown, terminal |
| 0x1FD5A | 15 / $FD4A (fixed) | $60 | RTS @0x1FD59 (Observed, SetScrollAndCtrl) | DisableNmi (Observed) | unknown, terminal |

## Methods tried in r03-a (each changes an inference from round 3; all negative / non-discriminating)
1. Fall-through: each predecessor is an unconditional JMP abs / RTS ending exactly at the byte -> no sequential
   entry possible (residual-static.json). Would have made the RTS reachable code; excluded.
2. Following-record base: both neighbours are Code starting/ending exactly at the byte -> not the first byte of
   an indexed data record.
3. Anti-tamper checksums (logo $8159..$8181, $FA7F+8k, bank-12 $9A6B..$9A77 / $999A..$99AC): no window covers
   any residual -> no checksum consumer (would not have discriminated anyway).
4. RTS-trick / computed dispatch: no pha/pha/rts dispatcher; BankPushAndMapY/BankPopAndRestore re-push the
   unchanged return address; jmp (ind) targets come from tables already in the r02-a word/word-1 audit.
5. Relocation-tolerant twin search (operands masked, immediates kept) over all classified code: longest match
   3 instructions, no K>=4 copy of any surrounding routine (evidence/twins.json; tools/twins.py).
6. Runtime access watch (trace detail filtered to the physical byte, bank-aware):
   - 0x14CCA: natural power-on -> intro -> stage 1-1, 6786 frames, no RAM writes; positive control: preceding
     JMP fetched once in ctx 10/11/14/15; 0 accesses to the byte, 0 dropped (evidence/readwatch-14CCA-intro.json).
   - 0x1FD5A: 1748 natural frames: 1744 Reads, 0 fetches; every Read is the 6502 RTS dummy read of PC+1 one
     cycle after the RTS opcode at 0x1FD59 (297/297, evidence/dummyread-1FD5A.json) - bus artifact, not a
     consumer (TK-003 addendum).
   - 0x17BC6 not watched: predecessor is JMP (no dummy read); a consumer would need a pointer that the static
     audits already exclude; stage-6 boss run cost (~0.3 s/frame traced) not justified by that prospect.
7. Precedent consistency (residual-static.json `accepted_short_unreferenced_precedents`): see review below.

## Rejection review (evidence-policy.md)
- Claim: each byte is an assembled RTS (Code, Static, dead tail; reachability not claimed).
- Decisive evidence: round-3 disassembly rejection (analysis/handoffs/round-03-disassembly.md; table in
  analysis/investigation.md "Round 3 disassembly"; analysis/tools-round3/l017-span.json empty for these 3) plus
  r02-a audits (analysis/redirect/r02-a/evidence/l017-refs.json, l017-reads.json).
- What it rules out: reachable code (no reference, fetch, fall-through) and consumed data (no read, no checksum,
  no record base). It does not rule out either dead code or a stray byte; the rejection is a sufficiency judgment,
  not a counterexample. Scope checked: it applies only to these 3 bytes (the 6 accepted bytes are unaffected).
- Independent constraints combined: unconditional predecessor; Code on both sides; no consumer of any kind;
  clean 1-byte decode (no overrun, not illegal); not a fill value; JMP->RTS / RTS->RTS idiom; ROM-specific
  edit-leftover precedents (0x14D00 `inc $05AF; rts` orphan duplicate 54 B after 0x14CCA in the same bank-10
  intro module; 3 accepted JMP;RTS dead tails).
- Consistency with accepted standards: every accepted unreferenced short Code region carries a per-instance
  constraint (operands naming shared RAM vars / in-routine branch targets: 0x14D00, 0x1D054, 0x0F4C8, 0x1EA94,
  0x1C2E6; enclosing branch span: 0x08743, 0x0B1BE, 0x0B7A2; validated JMP operand: 0x1C1E2); every accepted
  unconsumed short Data region carries non-instruction evidence (0x094A8 illegal opcodes, 0x0C2C9 / 0x1EB7A
  overrun, 0x1F0E6 pattern). The residual $60 bytes carry neither, so keeping them unknown is consistent.
- Strongest competing interpretation: stray/unconsumed byte (no positive support). Strongest revised inference
  for code: the ROM-specific edit-leftover base rate above. It is a population prior restating the same idiom,
  not a per-instance constraint, so it does not meet the policy bar for "an unreferenced return opcode alone".
- Remaining reason: no per-instance discriminating fact exists or can be produced - no execution path (unconditional
  predecessors, no pointer/stack path), no consumer, no twin. Reopen only with a reference/pointer, fetch, data
  consumer, enclosing control structure or a relocated copy of the surrounding routine.

## Deferred avenues
- L018 reachability (442 B, Static 0.7): no caller exists in the fully classified ROM; no new method.
- Optional semantics (enemy identities, mechanical dispatch names, wave-script terminators): no classification
  effect; belong to final editorial audit.
- Password screen milestone: attempted, arrival unconfirmed (GameMode stays 4); needs the menu-cursor RAM
  contract and a frame capture; classification-irrelevant.
- Next step: final editorial audit / verification on pass-003 with the 3 B residual reported as unknown.
