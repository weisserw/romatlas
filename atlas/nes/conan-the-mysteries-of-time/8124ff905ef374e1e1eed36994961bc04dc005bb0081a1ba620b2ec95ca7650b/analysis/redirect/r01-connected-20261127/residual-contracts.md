# Static residual engine contracts (not execution claims)

These proposals use the decoded control/data/stack effects, exact adjacency to known
entries or distinct RTS/tailcall boundaries, and repeated API conventions together.
Legal decoding alone was not treated as sufficient. Preconditions are caller
contracts, not claims of known natural callers. All fixed code uses permanent
physical banks14/15; B1B7 uses bank pair12/13. Records are in residual-decoded.json.

- B1B7..B20A: object task entry with CurrentTask<18; set action7B/reset facing,
  signed movement(19,20), private timer/state and animation flag; compare player
  relative position, invoke movement/animation helpers and terminate via D822.
  Branch targets land on complete task operations; animation resumes with bank6
  restored by F7A3. Natural producer unresolved, but coherent complete object task
  and independent established helper contracts support structural code.
- CB6C/CB76: disable display, choose PPU0000/1000, then CB7D selects PPU address
  and extracts caller's inline stream pointer through D611 before falling into
  existing CB8A counted upload. Inline stream pointer belongs after the caller's
  JSR, not after CB80's nested JSR D611.
- CCD7: A/Y screen pointer, disable/select nametable, tailcall known CCBE1024-byte
  decoder. CCE4: A/Y destination coordinates and external inline source pointer;
  read width/height/token, then decode one bounded row at a time. Calls CDB6 with
  existing run-token state to retain runs across rows; restores CurrentTask X.
- CE87/CEA4/CEC3/CEFB: decimal image helpers. CEC3 takes A number and X/Y tile
  position; divide by100/10 using CEFB's repeated unsigned subtraction, build
  RAM image header3x8 and digit tile bytes; CE87 renders all3, CEA4 suppresses
  leading blank digit columns. CE9C calls inline-image renderer D360: physical
  1CEAF..1CEB1 (CPU CE9F..CEA1) is inline RAM pointer05A7, NOT LAX code.
- CF0A: inline palette writer manually pops caller return pointer, reads palette
  offset and nonnegative values, queues writes, pushes adjusted return past high
  terminator. Stack path independently matches existing palette API behavior.
- D252: extract inline image pointer into00/01 then fall through known image setter.
  D26B: pop caller return address, consume inline X/Y followed by NUL-terminated
  text, write characters minus20 through D3E9, then resume after string terminator.
  D290 is A/Y pointer variant, D296 shared NUL loop. X/Y refer tile coordinates.
- D351 A/Y coordinates plus pointer00/01; D360 A/Y coordinates plus caller inline
  image pointer. D36D shared renderer reads image width/height/palette, rounds
  dimensions to tile rows/columns, sets palette quadrants, writes tile bytes,
  preserves X and temporary dimensions on balanced stack. Exact branch targets
  and known PPU helpers jointly constrain this image format.
- D465/D478/D4A0/D4B3 are two-byte LDA#1 alternate entry prefixes for existing
  wait/sleep helper bodies; do not sever them from the next classified instruction.
- D4E9 clears active slots among all18 while preserving inactive/suspended slots.
- D557 is inline-word discard wrapper, explicitly recognized by prior classification
  recipe; D611 pushes/extracts inline value, two PLA discard it, RTS returns after
  two-byte external argument. No claim that following bytes are local inline data.
- D649 extracts inline CHR source pointer then falls into existing D652 upload.
  D77E extracts inline image pointer then falls into existing D787 prepare helper.
- D7C5 inline-image setter with X object slot. PrepareImageTiles carry decides
  install or retry; failure subtracts5 from caller return address (JSR3+word2),
  re-pushes it and yields via D5B9, causing exact retry. This independently confirms
  both calling convention and instruction/data boundary.
- D80F argument42 ->Y, tailcall existing controller helper CC70; D881 arguments
 42/43/44 ->X/A/Y, tailcall existing D87A position primitive.
- D8FA sets PPU0000 and tailcalls counted decoder CB8A with42/43 pointer; D923
  passes coordinates42/43 and image pointer44/45 to D351; D953 passes text pointer
 42/43 to D290. These wrappers have independent known consumer semantics.
- DEF2 waits for controller byte06==80 through existing D80C; falls into the known
  following task code. Input reachability is not asserted.
- F69D..F6A5 four little-endian scale words1000,100,10,0 (lastzero not selected
  by decoded conversion). F6A5..F788 converts16-bit BE/BF into4digit C2..C5,
  divisors decrease1000->100->10, remainder finalunits; emits4tile/attribute pairs
  at076F with leadingblank suppression. Counter limits independently4digits.
- F926 checks all3music task IDs E1/E2/E3 equalFF and returns carry; F938 resets
  those IDs. F95B silences4APU channels via FB3D, terminates each valid task ID,
  restores CurrentTask X and resets task IDs toFF.

Unresolved one-byte CPU CAE1 / physical1CAF1 is RTS between the NMI tail JMP and
IRQ entry. No reference or independently supported entry was found; unlike the
multistep API wrappers it lacks a discriminating routine contract. Do not classify
it solely from valid opcode, adjacency or absence of execution. This is a localized
ambiguity, not rejection of the surrounding supported engine batch.
