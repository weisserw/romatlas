# Scope closure and actual remaining barriers

This is a productive combined proposal handoff, not terminal exhaustion. No map
bytes were accepted here. Every inline C79A dispatcher exposed by the chosen
current-source/static/runtime graph was inspected through its selected handlers;
`nested-frontier.json` is empty. Dispatching helpers themselves still appear in
`static-contracts.json.blocked`: that records their nonordinary return contract,
not an uninvestigated selected-handler frontier. All inspectable tables and
associated tiny masks/effect arrays are in data-proposals.json; all connected
supported code is in code-proposals.json. Generic pointer-pattern prefixes do not
claim original complete table layouts or natural selector coverage.

## Why the remaining return continuations were not inferred

- Fixed C31F/C340 sound wrappers: the service uses page-one sound storage
  ($0102+X, $016E, $0179) and several nested helpers. Ordinary stack depth arithmetic
  without an address reservation bound cannot show these writes are disjoint from
  the actual nested caller return bytes. Primary examples are bank6 CPU8003 and
 803F, and fixed EC72's $016E writes. The current proof deliberately rejects page1
  and unresolved indirect stores. Actual post-call fetches already captured are
  proposed individually; a broad universal sound return/bank summary is not.
  Missing fact is the reserved page1 window versus maximum service/NMI stack depth,
  not a missing natural caller. Deriving it is substantially different from the
  ordinary balanced-leaf proof used here; selected gameplay snapshots do not bound
  every service/interrupt path. No arbitrary forced call is proposed as a substitute.
- C2F1/C2F7/C304 and arbitrary save-slot wrappers: these really select a different
  switchable bank. Immediate descendants only continue when an explicit wrapper
  contract proves the final effective bank and preserved saved slot. C3CB andC3D8
  have that conditional proof with encoded chunk sizes; C816/C818 have a stacked
  old-bank/finite-stream proof. Unresolved other wrappers may call code that writes
  the shared saved slot or performs indexed/indirect writes with unbounded addresses.
  Caller continuation in the old bank would be a false physical identity. These
  are actual bank/nonclobber prerequisites, not an instruction-fetch requirement.
- Other indirect JMP/script-controlled transfers and dynamic writes: exact pointer
  provenance/range or write destination is absent from the conservative graph.
  Reachable numeric pointers alone do not establish intended code, and arbitrary
  execution would not resolve that structural ambiguity. The graph includes the
  supported prefix and direct callees; unmatched indirect destinations are omitted.

These barriers are localized in `static-contracts.json.blocked` with source, bank,
callee and reason. They do not reject adjacent data or another independently
supported entry. Remaining unknown byte groups are exact in unknown-ledger.json;
there is no whole-gap or whole-bank negative inference. No original generator,
full semantic schema or natural selector requirement is imposed on generic data.

## Other gameplay routes

Second/third periods, arcade intermission, fight display/controls and tied-game
shootout are now represented. Inputs moved arcade to NICE SHOT and changed fight
power; the bounded shootout controls did not demonstrate a completed shot. Winning
every fight/shootout, all tournament rounds and naturally earning the cup remain
unvisited. Extending them now would repeat representative control systems without
a concrete unresolved ROM consumer/boundary discrimination; their scene/handler,
transfer and graphics structures already belong to the combined batch. Ending
scene7 was already artificially visited in disassembly and is preserved. This is
reduced prospect of additional distinct coverage, not a claim those paths are dead
or impossible. A later concrete state/consumer inconsistency can reopen them.

## Method corrections that must not be inherited as rejections

- The first-eight-pointer heuristic misestimated actor AE42 as55 entries and scene
  C47A as lacking an even endpoint. Full local data and independent code boundaries
  give13 actor entries endingAE5C and8 scene entries before C48A JMP. This falsifies
  the sampling heuristic, not either data proposal. Original triage is retained.
- PPU templates also end onFE through a tail return; an FF-only scan crossed code.
  Corrected consumer semantics yield the exact814-byte region.
- Stream0 points at fixed C8CA even when mapper table value0. Its initial parser
  bank-boundary error came from a wrong address-space formula, corrected before any
  proposal. All15 final streams terminate and retain exact effective banks.
- First long progression process exited143 before its final snapshot; completed
  RAM samples/screens survive. The optional66=7 terminal-handler shortcut and
  terminal_probe.py then created valid replacement continuation snapshots. Do not
  reuse vanished process-local handles or claim the unexported last snapshot exists.
- Timer edit in later_game.py's batch named3 occurred in summary, not live third
  period. Third-period/tie evidence is the later third-live.snapshot and actual
  counter34=2->3 transition, separate from filenames.
