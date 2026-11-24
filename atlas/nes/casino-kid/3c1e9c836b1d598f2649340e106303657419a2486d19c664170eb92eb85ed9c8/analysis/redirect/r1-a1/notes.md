# Redirect round 1, attempt r1-a1 (2026-11-24)

Input checkpoint: pass-002 (maps/current.json == maps/pass-002.json), commentary round 1
(commentary/source built from commentary/notes/*.ann). Map, ROM, state, source untouched by
this attempt; only files under analysis/redirect/r1-a1/ plus leads/progress/handoff/scenarios
records were written.

All runtime work used the configured binary through `run.py serve` (tools/daemon.py relay,
socket /tmp/ra3c1e.sock). Traced emulation runs at ~1-2.5 frames/s (untraced ~30 f/s), so
every scenario was prototyped untraced and then replayed traced from the same snapshot with
identical inputs (deterministic: identical money/RAM results in both runs, see rr.log).

## Tools written here (all read-only with respect to map/ROM)

- obscheck.py [out.json]: session coverage vs map Code instruction boundaries (linear decode of
  every Code region); saves session coverage in traces/coverage.json format.
- inlinecheck.py [cov...]: L3 check - every executed JSR to an inline-parameter callee
  (tools/walk.inline_consumer) must have its observed return point at S+3+N and no opcode
  fetch inside the parameter bytes.
- targetaudit.py: every JSR/JMP/branch target encoded in Code must be a Code instruction
  start in the same effective bank; Code regions must not fall through into non-Code.
- refs.py BANK LO HI: absolute operand / immediate-pair references into a CPU range.
- provdelta.py cov...: Static code regions that become fully observed with extra coverage.
- rr.py / bjrun.py / bjscen.json / bjsession.sh: blackjack drivers (rr.py: snapshot chaining,
  deck patches at BjDeck+BjDeckPos, prompt detection from JSR return addresses on the stack:
  GridMenuSelect $D10C, SelectBetAmount $D21F, WaitButtonIndex $D41B, BjWaitForA $906F).
- dbg.py: cheat-password / debug-screen / bankrupt-password driver. montage.py: PNG tiling.
- bjbet.json: exported snapshot (blackjack bet prompt vs LISA). rr.log / dbg.log: exact
  replay records (inputs, patches, resulting RAM, banks, PC, frame counts).

## A. Blackjack reached (L1) - artificial position patch, natural game logic afterwards

Static decode of the floor talk check (bank 5 CasinoDecideAction $8800-$8954):
front cell = (ViewCellX+8, ViewCellY+7) + DirectionDeltas($98BA)[Facing] - (1,1).
Seat table ($98C2 ptr -> $98C5, count $98C4 = 9; 3-byte records Y, Xmax, opponent id; match
when Y equal and Xmax-3 < X <= Xmax): ids $18-$1F (8 blackjack seats) and $0E.
People table ($98E1 ptr -> $98E3, 32 ids, person i -> id). EncounterDialogue ($A52A):
ids $10-$17 poker gamblers, $18-$1F blackjack dealers, $00-$0F talkers; willingness
(variant 4 = offer to play) for $10/$18 always, others when the previous opponent's
progress bits = 3 (OpponentProgress). Playing reduces Opponent to 0-7, runs
SelectOpponentLevel then Api_RunPoker ($C0E5) or Api_RunBlackjack ($C0E8) and
RecordGameResult; all $0569-$056C = $FF -> Api_EnterKingMatch.

Patch (from traces/snapshots/casino.json): ViewCellX $0559 = 26, ViewCellY $055A = 35,
Facing $AE = 0 (up); press A 8 frames -> player cell (34,42) faces seat (34,41) = id $18
(dealer LISA, person 24 stands at (34,39)). Encounter scene, "A WINNING BLACKJACK WILL BE
PAID 3 TO 2", bet prompt -> snapshot bjbet.json. Bank 3 executes ($C19F -> bank 3 $8010).

Session (bjsession.sh 1, traced; deck patches at BjDeck $06E0 + BjDeckPos $93; nocheat =
BjCheatActive $07D0 := 0): card code = suit*13 + rank (rank 0 = A .. 12 = K, confirmed by
screen); deal order player, dealer (hole), player, dealer (up card); hits continue in deck order.
Rounds: stand-lose, hit, dealer draws/busts, push, player blackjack (3:2: +$15 on $10),
insurance YES vs dealer blackjack (net 0), insurance NO (push), split 8-8 (both hands won,
+$20), surrender (-$5), double down (+$20), natural hand, BET ALL MONEY (Select menu) with
player blackjack -> dealer $0 ("OOOPS!"), session ends, encounter screen "NOT ANYMORE!",
OpponentLevel $055C = 3, progress $056B = $C0 (blackjack dealer 0 recorded). Money results
match blackjack rules (rr.log).

Coverage: 1659 of 1862 bank 3 instruction starts observed (2025 bytes in 80 regions now fully
observed, 1728 bytes partly, 54 bytes none). 0 observed starts off map instruction
boundaries, 0 in non-Code regions. Bank 3 inline callees $90F2 (2 bytes) and $9110 (4 bytes):
10 and 19 executed sites, all returns at S+3+N (inlinecheck.py). Unobserved bank 3 starts are
short alternative branches (bet at-limit/quit, FINAL HAND, some cheat/soft-ace paths, split
variants) inside observed routines - static only, no conflict.

Observed semantics for commentary (L10):
- Hand record +12 total, +13 card count, +14 soft-ace count, +15 status. Dealer $076C, player
  $077C, split $078C (ram.inc correct). Status 2 = blackjack observed on the dealer; player
  status 9 after a lost stand, 5 after split in progress, 3 after dealer blackjack (from
  untraced probes; values not exhaustively mapped).
- BjCheatActive: in an untraced probe a player HIT whose card (K) would bust 16 was swapped
  with the following deck card (deck[pos] keeps K, player received the next card): the
  routine avoids a bust when the next card does not bust. The name "dealer cheat" may be
  misleading - it helped the player's hand here. Rate depends on dealer record +$11 vs
  RandomSeed.
- Select at the bet prompt opens "BET ALL MONEY / FINAL HAND" (BjQuitMenu). B at the bet
  prompt resets the bet to 0. FINAL HAND was attempted once untraced and the session did not
  end within 900 frames (input timing not investigated further; low value).
- Dealer names (bank 3 $914A): LISA, MARTHA, TOM, HARRY, STEFANY, CLYDE, BO, BRUNO.

## B. Inline-contract check (L3)

traces/coverage.json alone: 155 executed inline-callee sites, 0 mismatches. With the
redirect coverage: 242 sites, 0 mismatches (including bank 3 $90F2/$9110 above). Bank 4
$8B1C and $969C (2 sites each) remain unexecuted. 126 inline jump-table words all point at
Code instruction starts.

## C. Boundary audit (L8)

targetaudit.py over all Code regions: the only control-flow targets outside Code are
JMP $CFF6 (at $CFED) and BNE $CFF6 (at $CFF2); the only Code region falling through into
non-Code is $CFE6-$CFF5 (last instruction LDA #$3D at $CFF4) into $CFF6. $CFF6 = $48 (PHA),
immediately followed by established code @store_in_row ($CFF7), which ends with PLA/STA
(Arg10),y - the remnant's PLA ... PHA bracket matches that stack contract. Fixed->switch
targets (10) are only the documented bank entry slots $8010/$8013/$8016/$8019/$801C.

## D. Data structures with independent extent proofs (labels for data already classified)

- Bank 5 $93DF-$9456: four 30-byte cheat strings at $93DF/$93FD/$941B/$9439 (explicit
  CmpMulti inline operands, length byte $1E), ending exactly at DebugScreen $9457:
  "I AM A RICHMAN", "I WANNA SEE THE KING", "I WANNA GO BANKRUPT", "DEBUG MONITOR SWITCH ON"
  (digit 0 = space). The current 90-byte region $93FD-$9456 says "no direct operand reference
  found" - wrong; it is referenced by the three CmpMulti operands.
- Bank 6 $87DA-$8859: PresetPasswordPtrs (4 words -> $87E2/$8800/$881E/$883C; first target =
  table end) + the same four 30-byte strings; ends at observed code $885A. CheckPresetPasswords
  compares 28 bytes. The commentary's "$87F0 four 28-char strings" is wrong.
- Bank 4 (inside the 7098-byte region $9F06-$BABF): OpponentRecords $9F06 (9 x 9, consumer
  LoadOpponentRecord) -> $9F57 per-opponent 3-byte table (lda $9F57,x, x = 3n+1) -> $9F72
  strings -> HandNameRecords $A038 (11 x 4: two layout bytes + text pointer; PrintHandName
  $A038 + rank*4; rank domain 0-10 from EvaluateHand) -> $A064 per-opponent byte (9, lda $A064,x
  in ShowPokerMessage) -> PokerMessagePtrs $A06D (9 x 43 words; $A06D + 9*$56 = $A373) ->
  HandNameStrings $A373-$A405 (11 $FF-terminated strings, pointer targets) -> other text ->
  message texts $A597.. ending exactly at $BAC0 (max message string end = existing split).
  Hand names: HIGH CARD, ONE PAIR, TWO PAIR, THREE OF A KIND, STRAIGHT, FLUSH, FULL HOUSE,
  FOUR OF A KIND, ROYAL STRAIGHT, STRAIGHT FLUSH, ROYAL FLUSH.
- Bank 3 (inside $913A-$9B87): DealerNamePtrs $913A (8 words; first target $914A = table
  end) -> names $914A-$9176 -> UI strings $9177-$91D3 (MONEY, CHIP, STAND/HIT/SPLIT/SURRENDER/
  DOUBLE-DOWN menu, YES/NO, BET ALL MONEY/FINAL HAND) -> DealerRecords $91D4 (8 x 18; ends at
  $9264) -> BjMessagePtrs $9264 (8 x 38 words, $4C each; ends $94C4 = first pointer target) ->
  message texts $94C4-$9B87 (max string end = $9B88 = existing split).

## E. Poker ace-high straight (L9) - resolved statically

EvaluateHand returns 8 for A-10-J-Q-K unsuited and 10 for the suited case (8 + flush/2).
HandNameRecords entry 8 points to "ROYAL STRAIGHT" (bank 4 $A3D8), so category 8 is an
intentional hand rank in this game, not a bug. Commentary should drop "possibly an original bug".

## F. Debug screen / cheat passwords (L11, L12, L13)

See dbg.log. "DEBUG MONITOR SWITCH ON" RAM-written into PasswordBuffer $05DC on the password
screen, END, pad 1 A+Start held with pad 2 A+Left -> casino entry sets DebugEnabled $AA = 1
(held ~60 frames). Then pad 2 A+Left + pad 1 Start (12 frames) on the floor -> DebugScreen:
"MONITOR VERSION 1.00 / PRESENTED BY HIRO", address/value line. Address page: Right/Left
digit, Up/Down value, A read, B edit page (A writes: $07F0 := $1F observed), Start sound page
("MUSIC n": A next, B previous - not Up/Down as the commentary says), Start again SFX page
("SOUND EFFECT n": A next, B previous, each loop replays the effect), Select returns to the
floor. Results of the traced run and the bankrupt password: see section G.

## G. Traced debug / password results (dbg.log "=== traced run")

- debugpw: $AA = 1 after ~60 frames of pad1 A+Start with pad2 A+Left (casino entry,
  CheckCheatPasswords fourth branch).
- screen: address page read/edit (write $07F0 := $1F), music page, SFX page: indices 0-35 all
  played (A held/released per step); Select returns.
- bankrupt: "I WANNA GO BANKRUPT" -> GAME OVER screen, prg_banks [12,13,14,15], PC $8A84
  (Api_EnterBrokeScreen $C0F4 -> bank 6 $8013). Confirms L12.
- Coverage after blackjack + debug + passwords (coverage-all.json, before poker): 8509
  session opcode starts, 3569 new vs traces/coverage.json (bank 3 1659, bank 5 880, bank 7
  942, bank 6 88); 0 off-boundary, 0 in non-Code. provdelta.txt: Static code fully observed
  now: bank 3 2025 B (80 regions), bank 7 1765 B (22 regions, sound-effect routines), bank 5
  90 B, bank 6 31 B; partly observed 4568 B.

## H. Poker vs a regular opponent (bank 4 non-King paths)

People id $10 (JOSEPH) is always willing (EncounterDialogue variant 4). NPC 22 has id $10
(people table $98E3) and stands at cell (56,39) in casino.json. Patch $0559 = 48, $055A = 33,
$AE = 0, press A -> "THE GAME IS DRAW POKER." vs JOSEPH ($1000). pkbet.json = first bet
prompt (ante paid). pk.py plays a fixed action cycle (grid: A, Right+A, B, Down+A, ...; bet:
Up+A, A, Up+Up+A, Down+A, Select, A) for N prompts; untraced prototype of 30 prompts played
several hands with varying money (pk.log).
