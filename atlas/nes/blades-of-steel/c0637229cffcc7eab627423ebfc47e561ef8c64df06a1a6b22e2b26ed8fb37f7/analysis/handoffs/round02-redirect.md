# Round 2 redirect handoff

Redirect is complete with five new or materially improved ready leads. Start a
separate round 3 disassembly worker for independent review, followed by full
current-source commentary and redirect. This is productive batch closure, not
terminal research exhaustion.

Accepted checkpoint: `round02-disassembly-closure`. Map/state SHA256:
`e734fb13fbb234220432d10fd198d4336bdf369b1830ce8dbcb594b25b0fbac2`.
ROM classification remains **81.00% (106178 / 131088; 24910 Unknown)**.
Accepted map, source and ROM are unchanged. Current readability remains complete:
820 meaningful units and 1002 labels. The fresh byte-identical input verification
is `commentary/verification.json`; final editorial acceptance remains pending.

All paths below beginning with a filename refer to
`analysis/redirect/r02-structural-frontier/`.

## Combined proposal

`proposals.json` contains 604 distinct physical instruction starts, 1262 Unknown
Code bytes, 12481 Unknown Data bytes, and three prior Code-to-Data corrections.
Total: **13743 proposed Unknown bytes; zero accepted here**. If all are accepted,
11167 Unknown bytes would remain. That is a projection, not current classification.
One already classified FF byte is included in DMC sample 0 without double counting.
All proposals are Static. There are no new Observed or natural-execution claims.

- **R2-C04:** Four-byte signed delta array at physical 012853..012857. Correct
  three Code bytes and classify one Unknown byte. ADC A843/A844,Y uses Y=0/2.
  The LDA #0; STA; BEQ predecessor cannot fall through. No accepted direct control
  entry or observed root exists. The old graph followed both branch outcomes and
  generated JSR E000; that callee itself immediately failed a data barrier.
  Primary evidence: `C04-review.json`.
- **R2-CONNECTED-CONSUMERS, reopened:** 655 Unknown Code and 174 Unknown Data.
  Bank 1 B4DF uses the consumed-return dispatcher with five words ending at its
  first handler B4EE. All five handlers connect through queue, scrolling and
  packed-coordinate operations to the accepted B778 cell reader. There are 335
  new instruction starts, 164 patterned prefix bytes at 744B..74EF, and ten inline
  pointer bytes. Fixed C438 explicitly calls this dispatcher. Queue capacity and
  finite reader domains remain explicit; natural use is separate. Primary:
  `bank1-graph.json`, `research.py`.
- **R1-STATIC-RETURN, reopened:** 607 Unknown Code and 14 Unknown Data. This joins
  511 bytes after 28 immediate sound-request call sites with 96 missing mapper
  wrapper bytes. For each C31F startup call, supported entry conditions are request
  <= $51, S >= $B0, and NMI disabled/nonreentrant operation. Startup installs
  descriptor pointers; it does not interpret complete music scripts. Highest
  page-one storage is $019B. The reset path needs at most seven additional stack
  bytes, reaching $01AA. Actual callers supplying these conditions remain unproved.
  Requests $52..$55 are excluded because nested C34B overwrites saved bank $07E1.
  There is no universal music or NMI summary.

  C340's reset caller C1F5 independently supplies S=$FF and NMI disabled: deepest
  stack write $01F7 is disjoint from storage through $0179. DE1E's old data-barrier
  rejection at 1DF49 was false fallthrough after DF32 LDA #0; STA; DF37 BEQ. Known-Z
  propagation plus bounded C31F requests $0C/$0D supports conditional DE1E/DDF1
  returns. The explicit C459 wrapper corrects the prior statement that no wrapper
  existed. Its accepted B78E initializer calls CC8F. Template 6 emits 44 queue
  bytes; entry cursor $21 <= $B4 bounds the template's last write at $07DF and
  CC8F's at $07D6, below saved bank $07E0. C4D3 explicitly jumps to C459.
  Other wrapper structure does not certify every callee's universal return.

  Small arrays: 12772..12774 contains offsets 0/5; 12774..1277E contains two
  five-frame tile lists; 2F2E..2F30 contains team deltas +4/-4. Consumers, domains
  and code boundaries were inspected. Primary: `sound-zero-continuations.json`,
  `zero_contracts.py`, `zero-contracts.json`, `sound-reset-contract.json`,
  `wrapper-review.json`, `small-tables.json`.
- **R2-SOUND-DATA:** 8186 Unknown Data. The 87 three-byte records at
  18958..18A5D have independent bounds: encoded base $8945 plus three times the
  request, and request 86's $14 header including record 87. The first stream pointer
  $8A4D equals the endpoint. Following command/envelope/pointer patterns continue
  to 19F56; thirteen packed 16-byte records cover 1A856..1A926; descriptor-selected
  command streams continue to 1B0F2. Generic data is used where exact schema is
  unknown, after consumer inspection. Additional 336/16-byte software-speech
  remainders at 1B412..1B562 and 1BAA2..1BAB2 show dense AA/55/AB patterns and
  independent next-speech/FF boundaries. Do not enlarge speech counts or claim
  an original generator. Primary: `sound-data.json`, `data-conflicting-edges.json`.
- **R2-DMC-SAMPLES:** 4106 Unknown Data. Bank 6 $819B uses A=0..4 to index five
  fixed ED2B specifications, writes $4012/$4013, then enables DMC in $4015.
  The existing MIT toolkit implementation, `romatlas-port/src/nes/apu.rs`
  `restart_dmc`, gives address C000+64*field and length 16*field+1. Four unique
  physical spans follow: 1EFD0..1F021 (81 bytes), 1F050..1F2A1 (593),
  1F2D0..1F8C1 (1521; selectors 2/3 alias), and 1F8D0..1FFD1 (1793).
  All are fixed bank 15, without address wrap. Sample 0's last byte is already
  generic FF Data. Separate uniform FF suffixes, 24 unconsumed packed bytes at
  1F2A1..1F2B9, mapper identity bytes 0..7 at FFD0 explicitly consumed by C2F7,
  and identification text/metadata end at the vectors at physical 2000A.
  The 24-byte remainder has its own generic-pattern rationale: initial 4C could
  decode JMP CAAA in isolation, but there is no control entry or neighboring code
  purpose, while repeated AA/55/66 texture supports data. Review it independently
  and localize any counterexample. The unused specification field remains unknown.
  Primary: `dmc-samples.json` preserves raw fields, aliases, overlap and boundaries.

## Closure and deferred prerequisites

This phase followed the productive connected and independent avenues through
handlers, new readers, small tables and the sample tail. `new-reader-frontier.json`
found three unknown ROM operands; all became the 14 tiny-array bytes above.
Newly reached 8EC5's four-word table and its four handlers are already accepted and
readable. C3F0's selected summary handlers and nested intermission tables are also
already accepted. General C3F0 return needs per-state queue bounds below $07E0;
handler 8 additionally needs its nested script-domain contract. Targets are already
classified, reducing prospects for new target discovery. This does not reject
another local contract.

The six remaining blockers in `sound-zero-continuations.json` are that C3F0
wrapper, the already classified 8EC5 dispatcher, and contextual C2F7/C304 bank
protocol steps. These mapper helpers intentionally change the effective bank;
they cannot receive an ordinary same-bank return summary. No uninspected supported
Unknown handler continuation remains. Other indirect/script domains retain actual
pointer, selector or write-context ambiguity; arbitrary successful execution would
not discriminate it. Reassess changed neighbors after independent acceptance.

All four software-speech tails were checked: bank 2 ends at accepted B78E code,
bank 5 at accepted FF data, and both remaining bank 6 patterned tails are included.
No obvious sound payload continuation was reserved merely for another round.

Existing second/third periods, arcade, fight power, shootout controls and artificial
ending recipes were reused. No new distinct milestone or shortcut contract arose.
Completed fight/shot, natural triggers and tournament wins remain unclaimed;
prior best-effort routes and obstacles remain in `analysis/scenarios.json`.

`unknown-ledger.json` preserves the exact accepted complement: 198 bank-split
ranges summing to 24910, with proposals separated from unchanged local ambiguity.
Prior dispositions are reused for unchanged ranges. This is not an exhaustive
per-byte experiment or terminal rejection review. A later terminal decision still
requires evidence-policy.md's rejection review.

## Verification

`validation.json` records unchanged accepted hashes, JSON checks and exact counts.
No source/map mutation occurred, so the fresh input commentary verification remains
valid; disassembly must independently validate and rebuild its accepted output.
All research remains local. No ROM/toolkit mutation, deletion, cleanup, archive or
publication occurred. About 3.7 GiB disk remains available; research occupies about
3.5 MiB, with no new bulk captures or traces. **No toolkit issues found**;
`analysis/toolkit-issues.md` records this pass.

`analysis/leads.json` preserves histories, three new IDs and two reopened IDs with
bounded recipes. `analysis/progress.md` and `frontier.md` retain intermediate
corrections and superseded provisional counts. This completed attempt can now be
kept immutable. Next phase: independent round 3 disassembly.
