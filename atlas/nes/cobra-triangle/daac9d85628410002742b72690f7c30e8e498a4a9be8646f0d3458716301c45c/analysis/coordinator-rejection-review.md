# Coordinator rejection review

Checkpoint: `maps/round-005-disassembly-final.json`. Classification remains
131063/131088 bytes (99.98%); 25 bytes in ten physical intervals remain unknown.
This review accepts a terminal incomplete research assessment, not ROM completion.

I inspected `redirect/round-005-a/rejection-review.md`, its freshly retrieved
`primary-source.txt` and `unknown-ledger.json`, and the actual current source
contexts for the collision boolean exits, triangular-phase builder, descriptor
skip/threshold arithmetic and tile-decoder/selector boundary. Current source
SHA-256 is `fdc6762e2f0a9640869a503900292a59e08a0c2787fcab2586412bbe75daa5e7`.
The evidence is local; it is not part of a ROM-free portable mapping.

| Physical interval | Primary constraint and competing interpretation |
|---|---|
| 277..278; 30710..30711 | Each RTS follows a tail JMP, so ordinary fallthrough is excluded. Address-word matches are independently consumed graphics payload. An empty callback and a stored scalar remain possible; a return-frame precondition supplies no independent callback role. |
| 811..813 | RTI would require an interrupt-shaped frame; the following BVS would consume an opcode at an independently established entry. Reviewing each byte separately does not establish an interrupt or overlapping-branch contract. Vector absence is not proof of non-code. |
| 2801..2804; 3140..3143 | LDA zero/RTS fits nearby boolean idioms, but established branches bypass these islands. Existing boolean consumers belong to other entries. Repeated opcode shape alone does not establish these return entries; stored triples remain competitors. |
| 11951..11954 | JMP through scratch18/19 matches a real pointer representation, but the preceding empty-slot finder returns without forming that pointer. The representation does not independently identify this alias as a dispatcher. |
| 14251..14253 | CPY flags are overwritten by the following phase builder; a valid B7C0 pointer has no identified container/consumer here. Neither redundant code nor two stored bytes is excluded. |
| 25939..25942 | JMP E225 has a real destination and exact instruction length, but no independent parameter, field, control producer or table role distinguishes a tail alias from stored bytes. |
| 106162..106167 | ROM bytes are 10 B0 F4 E4 E0, correcting the old transcription. The BPL target is real decoder code, but its input/run/PPU state is not independently related to this prefix. Alternate decoding overlaps the selector. Eleven preceding zeros plus these bytes have tile size, but no source/count relation identifies that union as a tile; the apparent address reference is audio bytecode. The accepted zero run is not rejected or extended by this uncertainty. |
| 113405..113407 | SBC scratch21 precedes a known contact threshold, but current directional handlers build their own differences and jump to threshold entries. Scratch21 has several unrelated roles; no independent input/carry representation connects this subtraction to distance. |

These are localized structural ambiguities. Neither legal decoding, pointer
values, missing literal references nor unsuccessful artificial entry is treated
as a code/data proof. Generic data is not withheld for missing game semantics:
these exact residuals remain instruction-like or otherwise materially ambiguous.

I compared the strongest rejected claims with accepted small variants: A645
initializes a separately established motion endpoint consumed on a concrete frame
path; CFEF supplies the value consumed by two hardware scroll writes; B63A advances
an independently established record stride; B8D4 supplies a known speed-store
endpoint. The remaining aliases have no corresponding independent relation.
This distinction does not require an actual caller or natural reachability.

The review also considered the remaining shared00/collision51 bank theorem,
postcredits replay and graphics-container inference. None supplies a discriminating
role for a residual interval under the current evidence. Arbitrary entry would
demonstrate selected effects rather than intended structure. No concrete improved
inference or test emerged. Reusing this sound assessment on unchanged premises is
appropriate; a renamed investigation or another startup replay is not warranted.

Broad gameplay milestones and failed/corrected shortcuts remain documented in
`scenarios.json` and the phase handoffs. Natural full progression, boss defeat and
full hardware compatibility are not established. Future independent field/table
contracts or accepted instruction fetches can reopen a localized interval. This
is a terminal incomplete handoff under current evidence, not blocked work or proof
that the residual bytes are dead. Final editorial acceptance and verification
remain separate requirements after this reasoning review.
