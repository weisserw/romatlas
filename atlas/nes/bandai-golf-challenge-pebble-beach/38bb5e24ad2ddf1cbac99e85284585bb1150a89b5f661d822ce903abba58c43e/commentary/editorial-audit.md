# Final targeted editorial acceptance

Current scope is pass-003-parameters: 91 regions, 290 reviewed meaningful units,
65,552 classified bytes and zero unknown bytes. The complete current-source
readability review from round003 remains valid. This final review reused that
credit and examined concrete naming inconsistencies and affected dependencies;
it did not claim a fresh whole-source reread.

ChooseZoomFromPosition used CLUB_PUTTER ($0D) and HOLE_COUNT ($12) for positional
magnitude comparisons. These names falsely connected independent arithmetic to
club selection and course count. All four axis comparisons now use exclusive
ZOOM_LEVEL1/2_X/Y_LIMIT constants (29/13 and 18/9). The rewritten contract states
fraction subtraction, axis levels 0/1/2 and selection of their minimum. It leaves
original coordinate units unknown. No instruction value or control flow changed.

Inspected every named immediate operand in current game.asm, including all other
HOLE_COUNT and CLUB_PUTTER occurrences. Remaining hole constants compare holeIndex
or its loaded X value; remaining putter constants compare clubIndex. PlayerRecordStride
switches the player-record offset. Button masks consume controller edges; PPU flags
modify their corresponding control shadows; HIDDEN_SPRITE_Y supplies OAM Y even
where X temporarily carries it. Main-state symbols feed state selection and sound
slot constants bound the slot iterator. No other value-only semantic mismatch was
found in this named-immediate inventory. Unused symbol definitions are harmless;
no cosmetic replacement of unrelated numeric constants was needed.

Checked affected view-stream selection and nametable consumers: hole bounds still
select playable versus UI streams, independently of zoom thresholds. Inspected
current sound parameter/descriptor/stream boundaries and uncertainty notes against
round003 acceptance: biased envelope base, ID-zero bypass, final consumed terminator
and broad tail start remain consistent. Setup destinations retain their established
RAM-destination interpretation. Existing unknown physical units, optional third-sprite
meaning, unrestricted selectors and finer generic subformats remain explicit mechanical
limits. Prior useful prose and alternate instruction entries were preserved. No
additional missing structure, contradictory certainty or redundant comment required
repair in this targeted scope.

Both source comment lines were rewritten in place; all game.asm line numbers stay
unchanged, so existing region/unit source locations remain valid. The affected unit
summary was reconciled. Frozen baseline, state, map scaffold and all historical
reviews remain unchanged. Fresh supported audit supplies inventory validation,
byte-identical rebuild and current source hashes plus the refreshed reading copy;
those mechanical checks do not establish prose quality or full hardware compatibility.
