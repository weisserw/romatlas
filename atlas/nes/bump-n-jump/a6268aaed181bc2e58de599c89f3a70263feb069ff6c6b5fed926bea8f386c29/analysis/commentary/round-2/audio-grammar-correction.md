# B9 stream-width correction

Source handler CPU $BFA5 (physical $3FB5) always reads four register-value bytes and one duration byte. For X=2 (triangle), CPX #$08 follows TXA/ASL/ASL/TAX; the branch skips only STA $4001,X. Its preceding LDA ($F7),Y / INY still consumes the reserved sweep value. Thus five parameter bytes follow B9 for all channels, although triangle performs only three register writes.

The historical audio-streams.py used `4 if ch==2 else 5`, an incorrect shorter triangle width. The corrected bounded static walk is preserved in audio-grammar-corrected.py/json. Its output is byte-for-byte equivalent as parsed JSON to the old grammar result: 33 headers, 3382 tokens, zero invalid/out-of-bound tokens and the same seven gap intervals. The walked graph contains no B9 tokens, so neither the corrected triangle branch nor any B9 parameter width changes a walked boundary. This correction affects the handler contract and parser's unexercised triangle branch, not the classified container, current token boundaries or zero-invalid claim. No runtime replay or classification mutation was needed.

Source also corrects the low-byte period-add explanation: it sets dirty bit $40 only when carry updates the high byte; without carry it returns directly. RAM $040D is shared pitch/call scratch, not an exclusively pitch-dirty field.
