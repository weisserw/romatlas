# Round 4 commentary reconciliation + final editorial audit (input checkpoint pass-004)

Status: COMPLETE. `commentary.py audit` PASSED (ok, byte_identical, 1940/1940 entries,
2184/2184 units, editorial_audit reviewed). archive.editorial_status (read-only check):
complete=true, reasons=[]. Classification note: 0 Unknown bytes, but 4 of them are a
user-directed Manual fallback, not evidence-backed (see below).

## Scope
- The map changed from pass-003 to pass-004. maps/current.json == maps/pass-004.json
  (sha256 3debb849...); the old baseline was ba45ade7... (pass-003).
- I moved the whole round-3 commentary/ unchanged to analysis/round-3/commentary/
  (its game.asm sha256 is c74a7949...). Rounds 1-2 stay in analysis/round-{1,2}/.
- `commentary.py prepare` made a fresh byte-identical baseline (1940 scaffold entries).
- I copied tools/, uncertainties.md and progress.md from round 3.

## Carry method
- tools/mapdiff.py now compares against the round-3 baseline. It reports exactly 4
  one-byte changes, from gap/Unknown to Data:
  - $0C185 (bank 6 $A175)
  - $0C19B (bank 6 $A18B)
  - $1FB79 (bank 15 $FB69)
  - $1FDD0 (bank 15 $FDC0)
- All other scaffold ids, offsets and kinds are identical. Names, comments and
  annotation modules therefore carry by physical address.
- I regenerated review.json with tools/review.py. Units match round 3 in name, offset,
  length, status and summary text before the editorial fixes; only source line numbers
  shifted.

## Treatment of the 4 bytes
- I removed the round-3 unknown() notes (tools/strip_unknown.py). The new notes live in
  tools/ann/r04.py.
- Source note for each byte (no label): "Generic Data, meaning unknown. Classified only
  by the user-directed manual fallback of pass-004 (Manual provenance); this is not an
  observed or statically proven data role. Prior ambiguity: a lone unreferenced,
  never-fetched $60 after an RTS - code (dead duplicate RTS) vs data (padding) was
  unresolved (see analysis/handoffs/round-003-redirect.md; lead L009/L009b)."
- Each note also gives the location: after PlaceShot6, UnusedSq1Sound, PpuBufferBegin
  or FlushPpuBuffer's RTS.
- The block header now says "Manual provenance (user-directed fallback, not
  evidence-backed)".
- review.json summaries for regions 1947-1950 carry the required disclosure verbatim.
- Stale "Unknown" wording removed from:
  - the source header line;
  - the 4 notes;
  - uncertainties.md: the section is now "Former Unknown bytes: pass-004 user-directed
    manual fallback". The full history of the ambiguity is kept there.
  - metrics: review.py now reports unknown_entries 0.

## Final editorial audit (what I checked)
- I read all 4 changed ranges.
- I read a spread sample in every bank: header; data banks 0-2; bank 3 banner and
  special objects; bank 6 boss tasks; bank 7 jump table and CP palette code; bank 8
  DropPod; bank 12 trigger tables; bank 13 Level3BossTask defeat; bank 14 hit boxes and
  ObjKill; bank 15 briefing, NMI and PPU buffer.
- I ran whole-source scans for:
  - raw RAM in operands and in prose;
  - raw code targets;
  - auto or address-only labels;
  - generic locals;
  - stale Unknown text;
  - "no caller" claims checked against caller counts;
  - review summaries that quote provenance tags.

## Findings and fixes
- About 125 comment lines named RAM by its $0100-$07FF address even though a symbol
  existed. build.py prose() now renders those addresses with the same bank-aware
  symbols the operands use; review.py does the same for summaries.
- I fixed zero-page prose by hand: ActivePower, ShotX/ShotXHi/ShotY/ShotActive,
  PlayerFacingX.
- $E6-$E8 were Unk_* symbols even though MapScrollRight's note already gave their roles.
  They are now MapScrollLeft/Down/Up, checked against CpScrollUpdate and
  ObjApplyMapScroll.
- $057B was rendered as PasswordDigits inside the stage spawn-log code. It now uses the
  aliases SpawnLog (bank 12) and SpawnLogLo (banks 7, 8, 13, and ObjKill in bank 14 via
  an operand override). Bank 14 PasswordTask keeps PasswordDigits.
- $0795 in bank 15 is now BriefingSpeakerIndex.
- SetPriorityIf070A is now SetPriorityIfInWater.
- I rewrote the SelectPower comment. It described the wrong operands (it said "player
  type $E4"); it now uses CurrentPower rows and the PowerLevel column.
- I rewrote the ObjKill comment and gave PasswordTable its own note.
- The inline `.word $0751/$0548` operands are now HudPowerSpriteDef/HudScoreSpriteDef.
- The inline SetSpriteDefInline comments said "script/animation pointer"; they now say
  "sprite definition", matching the routine.
- 373 Data and 149 Code entry summaries changed vs round 3, plus 206 unit summaries:
  - inline words now name their helper and target;
  - phase-1 provenance tags are no longer quoted;
  - first_sentence no longer cuts at "e.g.".
- No toolkit issues were found. The toolkit, the ROM and archive-options.json are
  untouched.

## Verification
- `commentary.py verify`: ok, byte_identical=true.
- `commentary.py audit`: ok, byte_identical=true, reviewed_regions 1940, reviewed_units
  2184, editorial_audit true (commentary/audit.json).
- source/game.asm sha256 c3041b93c6d7796f8573cb8d56f66de8b5edf73b6bf33539c8f71d1d41b514ff
- review.json sha256 242f2b04489cf302d678655ebf5046df38a4296c285632221fdfc901f97c60a9
- reading-after.asm sha256 91b4623ccbb84279c8675c1a3d58642b650d75e02864d19c4a41e809b52b281e
  (refreshed by `read` and `audit`)
- ROM sha256 9bee9829... (input/game.nes unchanged)

## Counts (tools/metrics.py)
- 31185 lines.
- 1399 code labels, 727 with routine comments.
- 0 address-only globals, 0 raw RAM operands, 0 raw code targets, 0 generic locals
  (29 flag-only).
- 106 Unk_* RAM symbols, each with a "role uncertain" note (109 before).

## Examples
- Line 4773: the bank 6 $A175 Manual Data note.
- ObjApplyMapScroll now reads MapScrollRight/Left/Down/Up.
- ObjKill: `sta a:SpawnLogLo,y`.
- HudPowerTask: `.word HudPowerSpriteDef`.

## Residual uncertainties and remaining gaps (honest)
- The 4 Manual Data bytes:
  - Their meaning is unknown.
  - Their code-vs-data ambiguity is still unresolved.
  - The classification is the user's fallback, not evidence.
- 106 Unk_* RAM roles. SetPriorityIf0745/_6/0746 and VecSpare*/CallCpStageBank_A0xx keep
  address-based names because the role is unknown or the bank is variable.
- 941 uncommented branch-entry units are summarised only through their parent routine.
- Data banks 0-2 and 9-11 and many sprite-definition blocks have boundary/format notes
  only.
- Object game identities, AccelDelay values, the Level3BossTask defeat path (static only)
  and Gaia texts are unchanged from round 3 (uncertainties.md).
- Phase-1 evidence lines in the source still quote raw addresses; this is intentional,
  as they are records.
- The audit was a sample plus scans, not a whole-source reread. Rounds 1-3 review credit
  was reused.
- analysis/progress.md table row for this phase: left for the coordinator, along with
  archive-options.json.
