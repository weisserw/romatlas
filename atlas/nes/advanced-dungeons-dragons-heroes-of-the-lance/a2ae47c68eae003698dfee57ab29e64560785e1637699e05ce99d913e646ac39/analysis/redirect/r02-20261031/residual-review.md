# Four-byte residual frontier after proposed batch

This is an ordinary ready-batch handoff, not a terminal exhaustion assessment. Current map still has40660unknown bytes; proposed40656 are unaccepted. If the complete batch is accepted, four isolated `$60` bytes would remain:

| Physical | CPU / effective PRG slots | Independent boundary and remaining ambiguity |
|---|---|---|
| $008345 | $8335 / [4,5,14,15] | Immediately after JMP$83D3, before separately entered$8336 actor handler. Earlier BCC skips this byte. Could be an empty return entry or noncode separator; no independent empty-handler contract. |
| $011CF8 | $9CE8 / [8,9,14,15] | Between exact four-word AND3 dispatch table and first handler$9CE9. No table word selects$9CE8. Could be a spare return stub or noncode separator. |
| $018267 | $8257 / [12,13,14,15] | After JMP$814B, before separately entered$8258 command-class decoder. No direct target or bank-correct pointer established. |
| $018D4D | $8D3D / [12,13,14,15] | Second consecutive RTS after a complete accepted audio writer returns at$8D3C. Could be spare empty routine or duplicate return/padding. |

`residual-reference-check.json` records a bounded scan of all current classified instructions plus the new graph and all-alignment PRG word occurrences. No direct reference targets any four bytes. Only raw `$8257` pointer occurrence is physical$C13D in already accepted bank3 sprite/sequence data; primary neighbors are `$8251,$8253,$8255,$8257,...` under bank3 (effective[6,7,14,15]), anchored by fixedCC24 selectingPRG3. It points into bank3 assets, not bank6 audio, and does not establish an audio code entry.

The actual missing fact is structural: a supported empty-handler contract, bank-correct derived pointer/writer or related data-format evidence distinguishing these isolated bytes. Artificial execution starting at an RTS would merely return and discriminate nothing. Longer gameplay repetition has no specific prospect of reaching them after broad representative routes and the now closed ordinary graph; missing execution is not used as a data proof. No obvious neighboring multi-byte uniform/data extent remains to split off. This rationale does not reject a future localized independent inference, nor claim every selector or natural gameplay path was exhausted.

Other apparent residual barriers were resolved rather than carried forward: reset/BRK prefixes through a supported mapper/vector contract; reset80 constants through canonical read and library framing; checksum/opaque trailer through two independent exact sum constraints plus known title/vector boundaries. Their old tentative ambiguity is not inherited as a rejection.
