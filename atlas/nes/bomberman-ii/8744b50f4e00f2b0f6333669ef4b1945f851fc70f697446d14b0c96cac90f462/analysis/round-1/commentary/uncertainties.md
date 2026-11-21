# Commentary uncertainties

Carried phase-1 ambiguities and round-1 commentary questions.

## Unknown ranges with new consumer evidence (redirect candidates)
- 0x15518+5 (bank 5 $9508): 1-piece metasprite {1, 0,0,0,0}; MsPtr=$9508 at DrawPasswordGridCursor ($94FD) and DrawPasswordEntryCursor ($9526).
- 0x160C1+5 (bank 5 $A0B1): 1-piece metasprite {1, $2A,0,0,0}; MsPtr=$A0B1 at DrawSoundTestCursor ($A0A6).
- 0x178F9+13 (bank 5 $B8E9): two zero-terminated metatile lists ($B8E9, $B8EF) read by DrawMetatileList via TilePtr set in DrawVsMatchPoint ($B899, $B8AC).
- 0x179D7+2 (bank 5 $B9C7): text "SC" queued by DrawIntroScore ($B98C-$B9A1, X=2).
- 0x1B037+6 (bank 6 $B027): no consumer found; possibly the tail of the font CHR block (2977 bytes from $A486) or padding.

## Semantic uncertainties (names are tentative where noted in source)
- Enemy kinds 12-14 and 22-24: behaviour only partly analysed (comments say so).
- Sound: meaning of several byte-code commands ($D4-$EB setters of $0236/$0240/$023B/$02C7-$02E7) is mechanical only; envelope/vibrato/sweep naming inferred from use.
- Special passwords 4/5/6 set BombFuseTime to $2D/$4B/$69 - inferred from PlaceBomb using it as the fuse.
- ChooseStageItem threshold semantics and StageItemChoices layout (3 bytes per stage) inferred from code; table extent not proven.
- Generic data blocks (fixed $D2F9-$D7FF, $DDA9-$DFFF, $E000-$FF67, bank tails) have no identified consumer.
- HiddenP2WorldSelect ($A94A) is unreferenced; its effect (pad-2 Up -> ending) is a static reading only.
