# Commentary uncertainties (round 1)

Investigated limitations; each is explained in source near the code/data concerned.

- Fixed-bank library: many conventions match the Famicom Disk System BIOS ($FF/$FE
  mirrors, $FC/$FD scroll, VRAM buffer at $0300, VramStructWrite format, $4020-$408A
  writes in InitHardware). Names such as VramFill, LoadTileset, Pixel2NamConv follow
  that interpretation; behaviour was read from the code, the FDS origin is a hypothesis.
- $CFE6-$CFF6: unreachable remnant in the text printer (maps marks $3E/$3F to $3C/$3D);
  its jump target $CFF6 ($48 = PHA) is classified Data. Probably code; left unchanged
  (redirect question Q1, lead L8-cff6-remnant).
- EvaluateHand (bank 4 $9CEF) returns 8 for an ace-high straight without a flush,
  ranking it above four of a kind. Possibly an original bug; not verified (L9).
- Bank 3 (blackjack) is Static-only: hand status codes (+$0F of a hand record), the
  9-entry settlement table and the dealer record fields are inferred from comparisons
  only (L10). Treat bank 3 names as tentative.
- Bank 4 poker variables $0714-$075A: roles (pot, bets, streaks, personality nibbles,
  RiggedDeal) inferred from arithmetic call patterns; HandOutcome/HandWinner codes
  partially inferred.
- Bank 5 DebugScreen (memory viewer + sound test) and the cheat-password effects other
  than "I AM A RICHMAN"/"I WANNA SEE THE KING" are read from code only (L11).
- Bank 6 $8013 is the "broke" screen (A+Start to restart with starting money); the
  earlier disassembly note that $C0F4 leads to the King match is contradicted by the
  code: the KING password jumps to Api_EnterKingMatch ($C0F1 -> bank 6 $8016) (L12).
- Bank 6 CheckPresetPasswords: four 28-character preset passwords at $87F0; a match
  just returns from PasswordEntry; what state that produces is not traced (L13).
- Music driver (bank 6 $9D8C-$A47B): per-channel state arrays $0516-$0558 are only
  partly named; command byte semantics beyond notes/rests/end/jump/loop not decoded.
- Data: banks 0-2 graphics, bank 3/4 screens, bank 5 map/metatile data, bank 6 music
  and text data, fixed-bank sound-effect step tables are described at region level
  only; per-record formats remain undecoded (leads L5, L6).
- Branch labels: ~960 local branch targets keep mechanical names (@loop_xxxx for
  backward, @skip_xxxx for forward targets, @case_xxxx for jump-table entries); key
  branches in the main routines carry descriptive names.
