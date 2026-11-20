# Round 2 disassembly handoff

One disassembly pass complete; ROM incomplete. Next: full current-source commentary,
then redirect. Preserve/carry forward prior completed commentary, not merely new code.

ROM classification: 81.00% (106178 / 131088 bytes classified; 24910 bytes unknown).
Code29636;Data76542.14078 regions, all evidence/provenance;13316Static,762Observed.
Exact per-bank counts and bank-split Unknown complement: `analysis/round02/counts.json`.
Net57940 Unknown bytes classified;219 priorCode->Data corrections. Percentage is
unique physical bytes, not contexts, execution coverage or editorial acceptance.

Accepted current internal state `analysis/state.json` and `maps/current.json` are
byte-identical files, SHA256
`e734fb13fbb234220432d10fd198d4336bdf369b1830ce8dbcb594b25b0fbac2`.
Final immutable checkpoint `round02-disassembly-closure`:
`maps/round02-disassembly-closure.json`, `generated/round02-disassembly-closure`,
`analysis/round02-disassembly-closure-verification.json`; valid, byte-identical ca65
whole-ROM rebuild. Input SHA256 unchanged
`c0637229cffcc7eab627423ebfc47e561ef8c64df06a1a6b22e2b26ed8fb37f7`.
Earlier same-pass `round02-disassembly-final` remains immutable at106153/24935;
closure adds25 bytes. Round01 checkpoint and all historical evidence preserved.

## Accepted batch and independent review

All9 ready leads in `analysis/leads.json` verified with history retained. Initial
redirect batch accepted30318 UnknownData+14500 UnknownCode,219 Code corrections,
15888 existingData refinements. Primary proposed manifests remain immutable under
`analysis/redirect/r01-structural-contracts/`. Independent results and limitations:
`analysis/round02/review-notes.md`, `independent-validation.json`, `graph-audit.json`.

- C01 exact000328..00115C sprite pointers/count+4N records:142 adjacent records;
  preserve genuine000326..328 BCC90F2 (also selectorzero pointer alias).201 wrongCode
  corrected. C02 two copy/delta blocks correct16 bytes; neighboring handlers kept.
  C03 mixed animation data correct2, not3, bytes; generic mixed remainder avoids
  an unsupported overlapping schema. No blanket renderer/handler deletion.
- PPU15 token streams independently parsed through exactFF ends,7F retargets,
  literal/repeat lengths/destinations and source contexts. Stream0 fixedC8CA maps
  fixedphysical01C8DA even tablebank0. Templates36 pointers35 starts/suffix alias,
  FE/FF termination andFD continuation, exactly01CC9F.17+6 chunk-spec/payload unions
  checked against actual clamp/copy consumers. Bank2 payload aliases overlap;
  no ordered disjoint-record claim.
- Dispatch data uses actual C79A consumed-JSR-return contract, independent next
  handler boundaries and coherent selected graphs. Generic pointer prefixes do
  not claim complete original selector domains. ActorAE42=13 entries;sceneC47A=8
  beforeC48A JMP, correcting first-eight heuristic. Tiny masks/deltas/effect arrays
  have direct indexed consumers. All exposed selected tables/handlers followed.
- All6674 unique proposed instruction starts /6789 explicit views /14500 unique
  bytes independently decoded using configured binary, no data/current-region
  overlap.304 ordinary return graphs independently rewalked,647 JSR return edges
  source-byte checked;10 conditional contextual summaries reviewed.762 new starts
  match exact physical+PC+bank actual artificial fetches; Observed assigned only
  when every attached view matched, otherwise Static with matching views explicit.
- Generic checker overlooked absolute-indexed0080,Y reaching page1 in principle;
  actual C6E3/C6E7/CFC6 LDY0F,DEY/BPL loops bound writes0080..008F. Exact local
  loop byte patterns independently audited. No universal indexed-write safety.
- C373/C3E5/C44E wrappers have local saved07E0 nonclobber summaries. C3CB/C3D8
  require valid selectors0..16/0..5 and cursor<=80hex,count<=50hex;last write07D4
  below07E0. C816/C818 saves oldbank on stack, finite stream contract, explicit
  evenX0..28 or inspected04/08/0A lookup. No arbitrary wrapper family certified;
  no universal sound return proof. Per-call static continuation can be supported
  even where another wrapper lacks a universal contract.

## Additional connected gains

`connected-data-validation.json`, `connected-changes.json`, `block-reader-decode.json`:
- Bank5 physical0140DC..015769:5773 newData;10 eight-byte descriptors,12+10 terminated
  scripts,23/50 destination words,6x23+4x50 sixteen-byte tile sets exactly to9759.
- Bank1 physical00785F..007FCA:94 paired-pointer counted PPU/text records;1898newData
  plus1 existingFFtail byte refined. Highbit selects raw queuecopy bypassing command
  prefix, still copies low7bits literal count. A scratch no-payload interpretation
  produced gaps; consumer review corrected it before annotation. Final union exact.
- Bank1 blockpointers/cellmaps/sourcepointer006889..006A1B and tile-indexed patterned
  payload006A1B..00744B:3010 newData. Max selectedtileA2; missing/interstitial indices
  are generic non-code, no complete originaltable claim. Attribute-like continuation
  beyond744B lacks an exact endpoint. Typed readerB778..B7E7=112 StaticCode/60starts:
  complete PPU queue-copy structure, scaling, bounded indices and balancedPHP/PLP.
  Naturalcaller unresolved; no mere pointer/legal-decoding claim.
- `pattern-validation.json`:019F56..01A856=2304 genericData:47 denseLE ROM words plus
  34 repeated65-byte smooth/plateau low-valued records (all<=3F). Pointerstrides41hex
  andA846 boundary independently narrow initial threshold scan by6bytes. No exact
  waveform generator/consumer/original table domain claimed. Differently packed
  continuation staysUnknown for role/boundary research.

## Final local clear contract and stopping frontier

`clear_contracts.py`, `static-contracts.json`, `decode-c68c.json`,
`clear-closure-decode.json` retain the final targeted continuation review.
C68C's indirect store is bounded: clearsZP2C..EF, pointer0300 through06FF, stops
at highbyte07. Disjointpage1/07E0/mapper, ordinaryRTS. Existing-root closure on this
new local proof adds0 bytes; it is useful corrected reasoning, not global relaxation.
A coherent25-byte initializer physical00B79E..B7B7, CPU B78E inbank2[4,5,14,15],
then classifiedStatic: preserveRAM1D onPHA/PLA aroundC68C, set1C=1, ensure2A|2Bnonzero,
call ordinary bank-preservingCC8F,RTS. Exact graphics/code/FFtail bounds discriminate
its role; naturalcaller unproved. Initial scratchA98E was a decimal-offset reading
error rejected by existingData before annotations; corrected B78E is authoritative.
Two isolated CLC/RTS tails remainUnknown; unreferenced return pattern alone lacks
sufficient structure. No inferred mapper wrapper to this initializer exists.

Saved broad scenarios already cover actual second/third periods, arcadeNICE SHOT,
initialized fightpower response, tiedgame shootout input and prior artificial scene7
ending. Exact snapshots/patches/inputs/budgets retained in scenarios/redirect evidence;
no unchanged waits/gameplay replay. Completedfight/shot, natural triggering and all
natural tournament wins/cup route remain unclaimed. Current scenarios phase updated.

This is easy-gain stopping, not terminal research exhaustion. Local sound page1
storage versus maximum nested service/NMI stack reservation remains concrete research;
other saved-slot wrappers/indirect selector domains lack bounded targets/nonclobber.
Patterned sound continuation, bank1 attribute-like remainder and static initializer
caller may be useful narrower inquiries. Missing natural selection alone must not
veto structural claims; generic non-code needs no exact generator/table meaning.
No terminal rejection review or whole-ROM/editorial completion claim.

Prior completed commentary unchanged in `commentary/source` and its original
handoff; generated closure source is the current expanded classification and needs
full commentary/readability pass across all currently classified source. Superseded
wrong-code records/comments preserved in `round02/data-changes.json`; do not carry
incorrect instruction meanings back into data. Progress/investigation/leads/scenarios
updated; no toolkit issues found (`analysis/toolkit-issues.md`). Toolkit/ROM unchanged;
no archive, cleanup, publication or sharing. All state/source/assets/evidence local.
