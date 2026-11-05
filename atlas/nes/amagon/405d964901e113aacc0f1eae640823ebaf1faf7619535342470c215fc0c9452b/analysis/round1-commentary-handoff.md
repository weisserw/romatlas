# Round 1 commentary handoff

Current-source readability complete. Classification remains **50.20%**:
65,807 / 131,088 physical bytes classified, 65,281 unknown. The suspect vector-root
classification described below is preserved for independent structural review.

Authoritative phase input: [final disassembly map](../maps/round1-disassembly-final.json),
[internal state](state.json), [current map](../maps/current.json).
Current map still exactly equals the final disassembly map; ROM SHA256 unchanged.

Maintained local deliverables: [buildable source](../commentary/source/game.asm),
[CPU/RAM symbols](../commentary/source/symbols.inc),
[refreshed reading copy](../commentary/reading-after.asm),
[review inventory](../commentary/review.json),
[verification](../commentary/verification.json),
[scope assessment](round1-commentary/scope-assessment.json),
[phase progress](../commentary/progress.md).

Coverage: all 10,873 classified Code bytes / 5,239 instruction starts; all 52,000
Image bytes and 2,934 Data bytes; 193 explicit unknown-slice notes covering 65,281
bytes. All 1,552 inventory records and 265 routine/structure units reviewed.
No current-source readability ranges remain pending. Final editorial audit remains
pending; this is neither full-ROM classification nor final editorial acceptance.

Examples: seven audio voices and their local phrase stack; sign/magnitude object
motion and parallel field arrays; atomic PPU command publication; bank-preserving
UI/collision wrappers; scene state, demo, actor AI and animation dispatch; five-byte
graphics upload records and pointer lists; scene selector arrays and correct CPU
vectors. Constants/aliases preserve explicit assembler addressing widths and bank
context. Generic data retains prior bounded-inspection rationale; unknown gaps stay
opaque. Routine prose explains algorithms and contracts rather than every opcode.

Fresh final byte-identical verification and reading refresh succeeded.
Source hashes: game.asm `7bad001a4dbb336ea4de3ebe9cf2bfd899ed0e77fce4e97a76028399b7c9d0f0`;
symbols.inc `fa3330160c2ce80789b2c4c373fd7f948a62b398296e9a8d3a64091b0c46b6d9`.
ROM SHA256: `405d964901e113aacc0f1eae640823ebaf1faf7619535342470c215fc0c9452b`.

**Priority redirect:** raw vector bytes physical [131082,131088) are
`FE D8 6C C0 F9 FF`; true little-endian NMI is **$D8FE**, not the phase-1
annotation's **$FED8**. Discovery rpc.jsonl line 23 explicitly roots physical
130792 / CPU $FED8. Static tail annotations [130792,130876) and [130877,130892)
total 99 bytes and contain tile-like repetition/unofficial opcodes/KIL. Preserve
classification until checking independent callers/fetches/entry contracts and
neighboring consumers. Wrong vector-root evidence does not reject unrelated code
or by itself prove data. See [compact decisive evidence](round1-commentary/vector-root-review.json)
and [uncertainty/review guidance](../commentary/uncertainties.md).

Unresolved semantics: audio parameter $0528; exact UI decimal-display role; some
actor/boss game meanings; seven naturally unselected upload descriptors; generic
uniform-data purpose. Established mechanics and limits are explained in source.
Further classification/consumer research belongs to redirect, including the prior
three leads in leads.json. No redirect or disassembly work ran in commentary.

Toolkit: one existing confirmed issue NES-001, no blocker or new issue; see
[toolkit report](toolkit-issues.md). No toolkit edits. Disk 8.9 GiB free; no evidence
cleanup, publication, archive or final audit performed.
