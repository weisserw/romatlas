# Round 5 / disassembly handoff

## Scope
One disassembly pass on pass-004 that applied the four ready redirect leads, R4-L030..R4-L033. These are role,
boundary and provenance changes only.
- analysis/state.json was restored. The baseline was not re-created.
- input/game.nes is unchanged, and there were no toolkit edits.
- I used the binary named in workspace.json through a temporary socket bridge. Scratch dir /root/rascratch-r5d has
  been removed and my server stopped. No other process was touched.
- No gameplay: the natural ending (S13) and game over (S16) were already visited, and no lead needed new runtime
  evidence.

## Checkpoint
**pass-005**:
- maps/pass-005.json (sha256 c79c8cdf2ad8208b2fc21bfd3d00ada8c8dd544c4efb094409068012946bc298)
- generated/pass-005
- analysis/pass-005-verification.json

## Verification
- map.validate ok.
- analysis/state.json == maps/current.json == maps/pass-005.json (same sha256).
- `run.py checkpoint pass-005`: ok=true, byte_identical=true.
- Local byte-wise comparison with pass-004: no overlaps, 0 kind changes, 0 classification change.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Per bank (code / data+padding / unknown): b0 3/16381/0, b1 3/16381/0, b2 1546/14838/0, b3 4689/11695/0,
b4 10102/6282/0, b5 7124/9260/0, b6 3023/13361/0, b7 11436/4948/0, header 0/16/0. Observed code is now 25,671 B
(was 24,315). Regions went from 2,977 to 2,946 (1,614 code, 1,206 data, 126 padding).

## Findings
Details are in analysis/investigation.md, "Round 5". The script with assertions is analysis/tools/round05/apply.py,
with applied.jsonl and changes.json beside it.
- **R4-L030 verified, with one correction.** The opponent names occupy **$9278-$9290**, not $9291. "MONEY" begins at
  $9291 and has no located reader. The label/menu strings are therefore **$9291-$92ED**, not $9292. Everything else
  matches the proposal.
- **R4-L031 verified.** The zpLocalF byte counts ($46/$1E) independently confirm 35 and 15 words.
- **R4-L032 verified.**
- **R4-L033 verified:**
  - 36 regions were upgraded whole to Observed.
  - 6 regions were split into Observed and Static runs: b5 $8C55, $A328, $A36E, $A381 and b7 $C1C8, $C292.
  - Every evidence string carries the artificial-S16 caveat.

## Leads
Consumed and verified: R4-L030, R4-L031, R4-L032, R4-L033. No new leads were produced.

## Regions changed (for commentary carry-forward)
All of these are Data/Static unless stated otherwise.

**Bank 3** (ids 3225-3230 replace the 35 windows 2076-2093, 2687-2699 and 2806-2809):

| Range | Content | Suggested name |
| $9270-$9277 | name pointers | B3_OpponentNamePtrs |
| $9278-$9290 | names | B3_OpponentNames |
| $9291-$92ED | label strings ("MONEY" unreferenced first) | B3_PanelLabelStrings, moved from $9292 to $9291 |
| $92EE-$9335 | 4 x 18 tier records | B3_TierParams (rename Bank3_Data_92EE) |
| $9336-$9465 | 4 x 38 message pointers | B3_MessagePtrs |
| $9466-$9974 | 56 messages | B3_Messages |

- Render $816E/$8173 as #<B3_MessagePtrs / #>B3_MessagePtrs.
- Drop the false "no code reference" notes.

**Bank 2** (ids 3231-3234 replace 2047-2053, 2682-2686, 2794 and 2795):

| Range | Content |
| $B323-$B58E | strings, list A |
| $B58F-$B5D4 | pointer list A, 35 words |
| $B5D5-$B6D0 | strings, list B |
| $B6D1-$B6EE | pointer list B, 15 words |

- Region 2054 was narrowed to $B6EF-$B702 (Bank2_Data_B6EF).
- Render the immediates at $B192-$B1BA symbolically.

**Bank 5** (ids 3235-3240 replace 2311-2314, 2756, 2757, 2791 and 2838):

| Range | Content |
| $BAEA-$BB0F | 6x6 frame box |
| $BB10-$BB15 | outcome table |
| $BB16-$BB59 | 3 blocks |
| $BB5A-$BB5F | sub-list table |
| $BB60-$BB71 | sub-lists |
| $BB72-$BC21 | 9 blocks |

- Render the immediates at $B96F/$B827 symbolically.

**Provenance only** (byte content unchanged):
- 36 Code regions are now Observed. The full list is in changes.json; they include bank-3 $9975-$9C0C (the GAME OVER
  screen code), b7 $C1A3 (MainLoop_AfterGame) and $C394/$EC37.
- The 6 split regions above now carry Observed/Static runs. New ids 3241-3256 are split instruction runs at the
  same CPU addresses, so labels carry over unchanged.

Editorial notes from the redirect:
- The StakeTier in-game range is 0..3.
- GameOutcome 2 = lost everything (confirmed in S16).

## Unresolved / deferred
- The internal layout of $B6EF-$B702 is unknown.
- No reader is known for "MONEY" at $9291.
- The low-confidence fragments, including $BB30, have no caller. Their structural claims stand, and their
  reachability is unresolved.
- The generic R2-D02 windows still have no known role.
- Credits scroll S15 is still deferred. It is not structurally needed.

## Next
Commentary round 5 (rebuild from pass-005), then the final editorial acceptance. Toolkit: no new toolkit issues
found (see analysis/toolkit-issues.md; TK-001..TK-006 unchanged; one TK-005-family note was added).
