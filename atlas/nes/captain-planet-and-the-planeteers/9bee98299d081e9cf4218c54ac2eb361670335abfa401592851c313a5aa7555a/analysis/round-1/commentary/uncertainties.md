# Commentary uncertainties (round 1, checkpoint pass-001)

## Interpretation limits
- Game-level meanings of most Geo-Cruiser enemy/event objects (banks 3 and 6) and of the
  Captain Planet objects (banks 7/8/13) are inferred from code behaviour only; names such as
  ChaserTask, DamTask, FlockerTask, TankTask describe observed mechanics, not confirmed game
  identities.  Power numbering (CurrentPower 0-4 / SelectedPower 1-5) is mechanical; which
  number is Fire/Water/Wind/Earth/Heart was not confirmed.
- Several RAM bytes are reused with different meanings per stage type or bank (e.g. $C0
  IrqPhase / SpawnX / CrawlerHits, $057B PasswordDigits / SpawnLog, $F5 EventPhase also stepping
  the palette cycle, $0778, $078C, $0794).  Bank-specific aliases are used where known
  (ramalias in tools/ann); other shared names carry a comment.
- PauseState/PauseRequest: Select ($20) sets $07C2, HudBlinkStep raises PauseState and the NMI
  freezes tasks until Select; "pause" is the probable role (not confirmed by gameplay test).
- CHR: PPUCTRL is written as $A8/$AC (8x16 sprites, background table $0000); ChrBanks+4/+5 feed
  R0/R1 and +0..+3 feed R2-R5.  Sprite-vs-background use of individual banks is not asserted.
- Bank 13 StepObjectAnimation13 reads its list table from $A0ED, inside its own code: a stale
  copy of the bank 8 routine; whether it is ever called is unknown.
- Bank-dependent absolute operands left numeric on purpose: $8000-$9FFF data-bank reads (bank
  selected at run time) and the fixed-bank JSRs to $A000+ entries of CpStageBank (8 or 13).

## Unclassified ranges (phase 1 Unknown; 71 ranges, 19288 bytes) with commentary notes
- phys $06064-$0606E (11 B, bank 3): $A054-$A05E: the first five bytes (00 11 00 00 01) are an empty 1x1 sprite definition used by HideSelf ($A05F passes $A054 to SetSpriteDefInline); the last six decode as 'lda #0 / sta ObjXLo,x / sta ObjYLo,x', the same prologue as HideObjectAtOrigin in bank 14, falling into HideSelf. Left unknown by phase 1 (redirect question: split into 5 data + 6 code bytes).
- phys $064C6-$068C4 (1023 B, bank 3): $A4B6-$A8B4 (1023 bytes): contains the SpecialObjectHandlers targets $A4D6, $A4DD, $A507, $A55D, $A6D1 and $A777, so it is almost certainly code for special stage objects; never observed in the traced scenarios and left unknown by phase 1 (redirect: classify from the JMP table).
- phys $06A5D-$06B26 (202 B, bank 3): $AA4D-$AB16 (202 bytes): SpecialObjectHandlers entry 2 points to $AA4D, so this starts with handler code; unclassified (redirect question).
- phys $06BD3-$06C6F (157 B, bank 3): $ABC3-$AC5F (157 bytes) between SpecialObjectShotReaction and CheckLedgeEither; unclassified, probably more special-object handler code.
- phys $06DCF-$06DD7 (9 B, bank 3): $ADBF-$ADC7 (9 bytes): the remaining 9 of the 22 LandingTileIds bytes read by IsLandingTile (cpy #$16); left unknown by phase 1.
- phys $06E19-$06E36 (30 B, bank 3): $AE09-$AE26 (30 bytes) between ReadSpawnRecord and LoadSavedPosition; no reference identified.
- phys $06E57-$06E5C (6 B, bank 3): $AE47-$AE4C (6 bytes) between SaveObjPosition and SetPriorityBit; no reference identified.
- phys $0C064-$0C068 (5 B, bank 6): $A054-$A058 (5 bytes) is the empty 1x1 sprite definition passed by HideSelf6 to SetSpriteDefInline (.word $A054); data, left unknown by phase 1 because only the inline pointer refers to it.
- phys $0C185-$0C1B0 (44 B, bank 6): $A175-$A1A0 (44 bytes): sits where bank 3 has BobVertically; probably the same routine (unreferenced in this bank). Left unknown.
- phys $0C23E-$0C243 (6 B, bank 6): $A22E-$A233 (6 bytes) between RestorePosition6 and SetPriorityIf; no reference identified.
- phys $0C6F5-$0C994 (672 B, bank 6): $A6E5-$A984 (672 bytes): holds EventObjectHandlers targets (jumped to through JMP (Ptr0) at $A6E2), so it is very probably handler code; never observed and left unknown by phase 1 (redirect question, see L002).
- phys $0C9CE-$0C9E8 (27 B, bank 6): $A9BE-$A9D8 (27 bytes) between wave handlers; probably another handler (unclassified).
- phys $0CA3D-$0CA86 (74 B, bank 6): $AA2D-$AA76 (74 bytes) between WaveSwoopHandler and WaveObjectCommon; probably more handler code (unclassified).
- phys $0CB04-$0CB0D (10 B, bank 6): $AAF4-$AAFD (10 bytes); no reference identified (unclassified).
- phys $0CB19-$0CB21 (9 B, bank 6): $AB09-$AB11 (9 bytes); no reference identified (unclassified).
- phys $0D7AB-$0D7B8 (14 B, bank 6): $B79B-$B7A8 (14 bytes) is the sprite definition passed inline by CrawlerTask (.word $B79B); data, left unknown by phase 1.
- phys $0D995-$0DB25 (401 B, bank 6): $B985-$BB15 (401 bytes) before the stub table; no consumer identified (possibly more event handlers). Unclassified.
- phys $0E022-$0E024 (3 B, bank 7): $A012-$A014 decodes as 'jmp $0000': an unused slot of the bank 7 jump table (the same filler pattern as the bank 14 vector table). Left unknown.
- phys $0E028-$0E02A (3 B, bank 7): $A018-$A01A: another 'jmp $0000' unused jump-table slot. Left unknown.
- phys $0EA4B-$0EA4F (5 B, bank 7): $AA3B-$AA3F (5 bytes) is the sprite definition HidePlayer passes inline (.word $AA3B) - an empty 1x1 definition; data, left unknown by phase 1.
- phys $0F348-$0F377 (48 B, bank 7): $B338-$B367 (48 bytes) is the idle-animation pointer table read by CpSelectSprite (lda $B338,y / $B339,y) followed by definitions; data, left unknown by phase 1.
- phys $0F765-$0F77C (24 B, bank 7): $B755-$B76C (24 bytes) is the Captain Planet palette table read by SetCpPalette (lda $B751,y with y = power*4, powers 1-6); data, left unknown by phase 1.
- phys $0FB86-$0FBA5 (32 B, bank 7): $BB76-$BB95 (32 bytes) are the two sprite definitions CpVictoryPose passes inline (.word $BB76 / $BB86); data, left unknown by phase 1.
- phys $0FD7D-$0FFFF (643 B, bank 7): $BD6D-$BFEF (643 bytes) holds Captain Planet's sprite definitions: every SetSpriteDefInline argument in this bank ($BD91, $BDD1 ... $BFD1) points inside it. Data in role; left unknown by phase 1 because its internal extent was not proven (redirect question).
- phys $100E2-$10219 (312 B, bank 8): $A0D2-$A209 (312 bytes) after the stub table. Starts with task-entry shaped code ('lda #2/3; sta ObjPalette,x; jsr ...', handoff L001); probably object tasks created from the spawn tables. Unclassified (redirect question).
- phys $10251-$10503 (691 B, bank 8): $A241-$A4F3 (691 bytes): starts with the animation list table read by StepObjectAnimation (lda $A241,y / $A242,y), followed by lists and sprite definitions. Data in role, internal extent unproven; left unknown.
- phys $1060E-$106FB (238 B, bank 8): $A5FE-$A6EB (238 bytes): begins with task-entry shaped code (handoff L001); probably spawned object tasks. Unclassified.
- phys $10719-$109F1 (729 B, bank 8): $A709-$A9D3 (715 bytes): begins with task-entry shaped code (handoff L001); probably more object tasks. Unclassified.
- phys $10A30-$11101 (1746 B, bank 8): $AA20-$B0F1 (1746 bytes): begins with the initial-object tables read by CreateInitialObjects (index table $AA20, entry words at $AA23/$AA24); the entry words point at object task code inside this range. Mixed code and data; unclassified (redirect: follow these task entry words).
- phys $1123C-$113CB (400 B, bank 8): $B22C-$B3BB (400 bytes): starts with task-entry shaped code (handoff L001); probably spawned object tasks. Unclassified.
- phys $11476-$117CF (858 B, bank 8): $B466-$B7BF (858 bytes): starts with task-entry shaped code (handoff L001). Unclassified.
- phys $118DC-$11FD2 (1783 B, bank 8): $B8CC-$BFC2 (1783 bytes): starts with TransformFrames (4-byte records read by SetTransformFrame) followed by sprite definitions; probably all data, but its extent and any embedded code are unproven. Unclassified.
- phys $18013-$18028 (22 B, bank 12): $A003-$A018 is an 11-word pointer table ($ADF4, $AEDF, $AFB5, $B0A2, $B1D5, $B30B, $B373, $B3F8, $B472, $B51B, $B5C8) to the briefing texts in this bank; TextWindowSelectText (bank 15) maps bank 12 and reads it with 'lda $A003,y'. Data; left unknown by phase 1 (redirect question).
- phys $186AA-$186AE (5 B, bank 12): $A69A-$A69E (5 bytes) is the empty sprite definition HideShip passes inline (.word $A69A); data, left unknown by phase 1.
- phys $18889-$188B9 (49 B, bank 12): $A879-$A8A9 (49 bytes) decodes as a copy of SetHitBoxB6 from bank 14 ('lda #6 / sta Tmp93 / sta Tmp94 / ...' ending in rts); unreferenced in this bank. Left unknown (redirect: probably dead code).
- phys $18B13-$18B1D (11 B, bank 12): $AB03-$AB0D (11 bytes): the remaining 11 of the 24 ShipLandingTiles ids read by IsShipLandingTile (cpy #$18). Left unknown by phase 1.
- phys $18CF8-$18CFB (4 B, bank 12): $ACE8-$ACEB (4 bytes: 20 26 16 06) between GeoLevelSetup's RTS and LoadGeoPalette; no reference found. Left unknown.
- phys $18EEE-$18EEE (1 B, bank 12): Single byte between two briefing text runs; most likely the text terminator/control byte (redirect: classify with the texts).
- phys $19329-$19329 (1 B, bank 12): Single byte between two briefing text runs; most likely the text terminator/control byte (redirect: classify with the texts).
- phys $19382-$19382 (1 B, bank 12): Single byte between two briefing text runs; most likely the text terminator/control byte (redirect: classify with the texts).
- phys $19407-$19407 (1 B, bank 12): Single byte between two briefing text runs; most likely the text terminator/control byte (redirect: classify with the texts).
- phys $1942A-$1942A (1 B, bank 12): Single byte between two briefing text runs; most likely the text terminator/control byte (redirect: classify with the texts).
- phys $19481-$19481 (1 B, bank 12): Single byte between two briefing text runs; most likely the text terminator/control byte (redirect: classify with the texts).
- phys $194CD-$194CD (1 B, bank 12): Single byte between two briefing text runs; most likely the text terminator/control byte (redirect: classify with the texts).
- phys $1952A-$1952A (1 B, bank 12): Single byte between two briefing text runs; most likely the text terminator/control byte (redirect: classify with the texts).
- phys $1954C-$1954C (1 B, bank 12): Single byte between two briefing text runs; most likely the text terminator/control byte (redirect: classify with the texts).
- phys $195D7-$195D7 (1 B, bank 12): Single byte between two briefing text runs; most likely the text terminator/control byte (redirect: classify with the texts).
- phys $19F29-$19FFB (211 B, bank 12): $BF19-$BFEB (211 bytes): repeating 8-byte records (00 14 00 0F xx xx xx xx) after the last routine; looks like data (possibly unused sprite or column tables) but no consumer was found. Unclassified.
- phys $1A01C-$1A01E (3 B, bank 13): $A00C-$A00E holds 00 00 00 instead of a JMP: the jump-table slot used for the level 1 switch-tile animation in bank 8 is empty here (never called on levels 3-4). Left unknown.
- phys $1A117-$1A25E (328 B, bank 13): $A107-$A24E (328 bytes) after StepObjectAnimation13: probably animation lists and sprite definitions like bank 8 $A241; unclassified.
- phys $1A36D-$1A391 (37 B, bank 13): $A35D-$A376 decodes as 'tax / ldy CurrentTask / copy ObjX/ObjY from slot Y / lda #$0D / sta TaskBank,x' - the bank 13 copy of InitChildAtSelf8 - running into the per-level tables read at $A377+ (indexed by Level 3-4, so the first bytes of those tables are never read). Exact code/data boundary not proven; left unknown (redirect question).
- phys $1A3D0-$1A624 (597 B, bank 13): $A3C0-$A614 (597 bytes): starts with the initial-object index/word tables read by CreateInitialObjects13 ($A3C0, $A3C5/$A3C6); the words point at task code in this bank. Mixed; unclassified.
- phys $1A72E-$1A736 (9 B, bank 13): $A71E-$A726 decodes as 'tax / lda #7 / sta TaskBank,x / ldx CurrentTask / rts' (a SetTaskBank7 copy); unreferenced. Left unknown.
- phys $1A747-$1B4D1 (3467 B, bank 13): $A737-$B4C1 (3467 bytes): begins with task-entry shaped code ('lda #2 / sta ObjPalette,x / jsr B13_SetSpriteDefInline ...', handoff L001); the bulk of the level 3-4 object tasks are expected here. Unclassified (redirect question: follow the MapSpawnTable13 / init-task words).
- phys $1B5A1-$1C00F (2671 B, bank 13): $B591-$BFFF (2671 bytes): starts with the DebrisTask frame and speed tables ($B591/$B592, $B599); the rest is unclassified (likely more data and object code).
- phys $1C0BB-$1C0C3 (9 B, bank 14): Unused vector-table slot(s): the bytes decode as 'jmp $C000' (4C 00 C0) like the other filler entries. No caller is known; left unknown by phase 1 (redirect: these are almost certainly 3-byte JMP instructions).
- phys $1C0E2-$1C0EA (9 B, bank 14): Unused vector-table slot(s): the bytes decode as 'jmp $C000' (4C 00 C0) like the other filler entries. No caller is known; left unknown by phase 1 (redirect: these are almost certainly 3-byte JMP instructions).
- phys $1C0EE-$1C0F0 (3 B, bank 14): Unused vector-table slot(s): the bytes decode as 'jmp $C000' (4C 00 C0) like the other filler entries. No caller is known; left unknown by phase 1 (redirect: these are almost certainly 3-byte JMP instructions).
- phys $1C100-$1C105 (6 B, bank 14): Unused vector-table slot(s): the bytes decode as 'jmp $C000' (4C 00 C0) like the other filler entries. No caller is known; left unknown by phase 1 (redirect: these are almost certainly 3-byte JMP instructions).
- phys $1C24E-$1C262 (21 B, bank 14): $C23E-$C24D decodes as the mirror of AddToPtr0 (sta $06FB / Ptr0 -= A / rts), a plausible unreferenced 'SubFromPtr0' routine; no caller or vector entry found. Left unknown by phase 1 (redirect question).
- phys $1C649-$1C744 (252 B, bank 14): $C639-$C734 (252 bytes): MetatileLookup reads a word table based at $C639 (lda #$39 / lda #$C6, index = id*2), so this range is very probably a 126-entry metatile offset table. Left unknown by phase 1; extent not proven (redirect question).
- phys $1C870-$1C8CA (91 B, bank 14): $C860-$C88A (43 bytes) precede HitSizeTable; probably the first entries of the same size table (indexed from (A+1)*2) but the lowest index is not proven. Left unknown.
- phys $1D262-$1D27A (25 B, bank 14): $D252-$D25A (9 bytes) is copied by HudScoreTask into $0548 as a sprite definition template (lda $D252,y), so it is data; phase 1 left it unknown because only the indexed base was seen.
- phys $1D90D-$1D912 (6 B, bank 14): $D8FD-$D902 (6 bytes) after StopSound; no reference found. Left unknown.
- phys $1E824-$1E873 (80 B, bank 15): Bytes $E814-$E863 are the inline sprite definitions selected by BriefingPortraitTask ($E814 and $E842 via SetSpriteDefInline). Phase 1 left the range unclassified; the consumer is clear but the exact extent of the second definition is not proven.
- phys $1E9BB-$1EAD0 (278 B, bank 15): $E9AB-$EAC0 (278 bytes) sits between BriefingSpeakerTask and TextWindowInit. It is probably more briefing/speaker data or an unreached task, but no consumer has been identified.
- phys $1F1F6-$1F249 (84 B, bank 15): $F1E6-$F239 (84 bytes) is the task entry that GameOverScreenSetup passes to SpawnTaskGroup (lda #<$F1E6 / ldy #>$F1E6), so it is very likely code; it was never observed and phase 1 left it unknown (redirect question).
- phys $1F3E9-$1F4AE (198 B, bank 15): $F3D9-$F49E: SpawnCreditsSprites creates three tasks at $F3D9 (lda #$D9 / ldy #$F3), so this range starts with task code; never observed and left unknown by phase 1 (redirect question).
- phys $1F4C2-$1F70A (585 B, bank 15): $F4B2-$F6FA (585 bytes): unclassified bytes after a run of $07 fill; no code or data consumer has been identified yet.
- phys $1FB79-$1FB79 (1 B, bank 15): Single byte $FB69 between PpuBufferBegin's RTS and BuildOam; never executed or referenced (likely padding).
- phys $1FDD0-$1FDD0 (1 B, bank 15): Single byte $FDC0 after FlushPpuBuffer's RTS; no reference (likely padding).
