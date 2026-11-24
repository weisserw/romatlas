# Round 1 redirect handoff (attempt r1-a1)

## Scope
Input: checkpoint pass-002 (maps/current.json == maps/pass-002.json; ROM 100.00% classified,
0 unknown) and the round-1 commentary source. Redirect changed research/lead records only:
ROM, map, state, commentary/source and the configured binary are unchanged. Exact unknown
complement: empty (nothing to ledger). Research: analysis/redirect/r1-a1/ (notes.md A-K).

Avenues taken: L1 blackjack route (and through it L10 semantics, L2 static-code confirmation,
L3 bank 3 inline contracts), whole-map boundary audit (L8), the data tables that the
blackjack/poker/password code reads (L13, new L14), L9 by static reading, L11/L12/L13 at
runtime, a poker session against a regular gambler (bank 4 non-King paths), and the
status-screen password round trip (bank 5 encoder, bank 6 decoder).

## Findings
1. Opponent selection (L1): CasinoDecideAction's talk check uses the seat table $98C5 (9 x
   [Y, Xmax, id], match Xmax-3 < X <= Xmax) and the people id table $98E3 (32 ids). Ids $10-$17
   are poker gamblers, $18-$1F blackjack dealers; $10 and $18 always accept. Patch from
   traces/snapshots/casino.json: $0559=26, $055A=35, $AE=0, then A gives seat $18 (LISA).
   For poker: $0559=48, $055A=33, $AE=0 next to NPC 22 (id $10, JOSEPH).
2. Blackjack observed (13 traced rounds, rr.log): 1659 of 1862 bank 3 instruction starts were
   observed. Payouts, insurance, split, surrender, double and the dealer-broke exit (OpponentLevel 3,
   progress $056B=$C0) all match the static reading. Card code = suit*13+rank; the deal order
   is player, dealer hole card, player, dealer up card.
3. Coverage (coverage-all.json, in traces/coverage.json format): +4438 opcode starts across
   banks 3-7. 0 starts fall off map instruction boundaries and 0 fall in non-Code regions. 276
   executed inline-callee sites all return at S+3+N, and all 126 inline jump-table words hit
   Code starts.
4. Boundary audit (targetaudit.py): in the whole map, the only control-flow targets outside
   Code are JMP $CFF6 and BNE $CFF6. The only Code region that falls through into non-Code is
   $CFE6-$CFF5. Everything else is consistent.
5. Data tiling with exact extents (all bytes already Data):
   - Cheat strings, bank 5 $93DF-$9456: CmpMulti operands with length $1E.
   - Preset passwords, bank 6 $87DA-$8859: a pointer table followed by the same 4 x 30-byte
     strings.
   - Poker tables, bank 4 $9F06-$BABF: OpponentRecords, HandNameRecords $A038,
     PokerMessagePtrs $A06D -> $A373, hand names, and texts that end at $BAC0.
   - Blackjack tables, bank 3 $913A-$9B87: DealerNamePtrs, names, DealerRecords $91D4,
     BjMessagePtrs $9264 -> $94C4, and texts that end at $9B88.
6. Poker category 8 is the hand "ROYAL STRAIGHT" (hand-name table), not a bug (L9).
7. Debug monitor: "DEBUG MONITOR SWITCH ON" with pad 2 A+Left enables it. The music and SFX
   pages use A/B, not Up/Down (L11). "I WANNA GO BANKRUPT" leads to the GAME OVER screen through
   $C0F4 (L12). The status-screen password decodes back to the same money and progress (S13).

## Ready leads for the next disassembly pass
- L8-cff6-remnant (high). Annotate physical 118790, length 1, as Code (Static) next to
  $CFE6-$CFF5. Evidence: targetaudit.txt, and the stack bracket PHA -> @store_in_row PLA.
  Reachability stays unresolved.
- L2-static-code-confirmation (medium, provenance only). Merge
  analysis/redirect/r1-a1/coverage-all.json into traces/coverage.json (sum the hits, tag
  redirect_r1a1). Then run tools/mkmap.py, map.load/validate and generate/verify. Expected:
  5219 Static code bytes become Observed (provdelta.txt), with no kind or boundary change.
  Re-run obscheck.py first; it should report 0 conflicts.
- L13-preset-passwords (medium, labels). Split and label bank 6 physical 100330-100457 as
  PresetPasswordPtrs (8 bytes) plus 4 x 30-byte strings at 100338, 100368, 100398 and 100428.
  Split and label bank 5 physical 87023-87142 as 4 x 30-byte strings at 87023, 87053, 87083
  and 87113. Fix the wrong rationale "no direct operand reference found".
- L14-bank34-game-data-tables (medium, labels). Split the bank 3 and bank 4 generic blocks at
  the CPU boundaries listed in leads.json/notes.md D. They stay Data; byte identity must hold.
Commentary (next cycle): L9 (ROYAL STRAIGHT), L10 (bank 3 semantics, the BjCheatActive
swap helped the player in the observed case), L11 (debug buttons), L12 (GAME OVER),
L13/L14 labels.

## Other avenues and why they are deferred
- About 9.8 KiB of code is still Static-only or partly observed, mainly bank 4 AI and King
  branches and bank 5 floor/people branches. It is already Code by static flow and has no
  contradicting observation. More traced play would only change provenance, and traced
  emulation is about 10x slower than untraced (TK-004), at roughly 1 h per session.
- L4/L7 unreferenced fragments: no byte-level references were found and no trace shows a
  dynamic call. Their classification does not depend on a caller.
- L5/L6 (graphics and fixed-bank table semantics): labels only, with no code-versus-data
  question. They rank below L13/L14.
- Two small runtime gaps remain: FINAL HAND (timing was not resolved) and the natural
  "all opponents beaten -> King" trigger. The trigger can be tested by RAM-setting
  $0569-$056C and beating LISA. Each gives little new coverage, since the King match itself
  was already observed.

## Assessment
Ready leads exist (one structural byte, provenance, data labels, commentary corrections), so
one disassembly + commentary cycle is warranted. A terminal rejection review is not required:
no rejection is being inherited, and a structural proposal (L8) is pending. After that cycle,
the open questions are provenance and semantics only; no code-versus-data ambiguity has been
found.

## Artifacts
analysis/redirect/r1-a1/{notes.md, rr.py, rr.log, bjsession.sh, bjbet.json, pk.py, pk.log,
pkbet.json, dbg.py, dbg.log, pwchain.py, pwchain.log, coverage-all.json, provdelta.txt,
inlinecheck.txt, targetaudit.txt, obscheck.py, inlinecheck.py, targetaudit.py, refs.py,
provdelta.py, *.png, leads.before.json, scenarios.before.json, update_leads.py};
analysis/leads.json; analysis/scenarios.json (S8, S10-S13); analysis/toolkit-issues.md (TK-004).
