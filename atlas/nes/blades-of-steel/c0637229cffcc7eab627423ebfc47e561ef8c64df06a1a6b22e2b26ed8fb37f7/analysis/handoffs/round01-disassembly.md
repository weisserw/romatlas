# Round 1 disassembly handoff

Single pass finished; ROM incomplete. No commentary or redirect work performed.

ROM classification: 36.80% (48238 / 131088 bytes classified; 82850 bytes unknown).

15,218 Code +33,020 Data;7,380 regions. Header16 classified separately from16
physical8KiB PRG banks. Per-bank counts/exact unknown complement:
[../round01/counts.json](../round01/counts.json). All classified regions carry
structural evidence and provenance; full current-source commentary is pending.
Execution metric is separate:5,179 unique observed instruction starts,10,819 decoded
bytes across all logged timelines, including artificial states. No unmatched roots.
Static inferred code is not claimed observed or naturally reached.

Verified final checkpoint: maps/round01-disassembly-final.json,
generated/round01-disassembly-final, analysis/round01-disassembly-final-verification.json.
Internal analysis/state.json and maps/current.json match final checkpoint. Map SHA256
ecde055d22cf42971290adbfd33853bdcb7abe0d42907ed8ad330a9693f68c62.
Map validation and ca65 byte-identical rebuild passed; ROM SHA256 unchanged.
Provisional round01-disassembly checkpoint and baseline retained.

Scope/evidence: validated reset/NMI/IRQ C1C5/C204/C266, conservative bank-aware local
paths, observed setup/game/ending roots and direct JSR callee closure. No return
fallthrough assumed across unknown bank/stack contracts; inline dispatcher C79A is
a barrier. Repeated FF/other uniform slices are generic data, with exact boundary
and conflict inspection. Whole bank3 payload (physical C011..FD3F) is generic data
based on repeated compressed graphic-pattern/mask/text content across both halves,
not a specific recovered schema. Four speech streams are specific counted packed
bit data proved by C34B selector bank lookup and EC72 descriptor/DAC4011 consumer;
precise scripts and RPC evidence are under analysis/round01. Final eight new direct
callee targets were followed; resulting79-byte addition is in final checkpoint.

Gameplay: title/game confirmed Blades of Steel; manual/cheat hypotheses sourced in
analysis/gameplay.md. Earlier match/period2/third artifacts were attract mode; a
same-pass correction narrows their reachability claims. Actual College Exhibition
setup and hockey were subsequently visually confirmed. Timer RAM C0..C3 and63
contract tested; actual goal/faceoff held00:01 and did not prove later periods.
Concrete frontier snapshot expiry-frontier.snapshot preserves this obstacle.
Scene7 shortcut from title-ready (18=7,19=0,1F=0,2E=40,2F=02) reached visible terminal
congratulations and A was exercised; this is artificial ending reachability only.
Later periods, intermission minigame, fight/penalty/overtime and tournament rounds
remain deferred with reasons in analysis/scenarios.json. Do not repeat long waits
or early Start timing; title-ready snapshot is at resetframe240 after scene18=1.

Ready/reviewed outcomes in analysis/leads.json: vectors, direct calls, speech,
uniform data and ending verified. Redirect should resolve the concrete countdown/
faceoff gate, regular callee return/bank preservation contracts (fixed code after
calls), and graphic-stream boundaries/consumers. Large residual intervals mix code
and data; unknown meaning alone is not a reason to keep obvious data unknown.
No terminal research exhaustion is claimed: this is disassembly easy-gain handoff.

No toolkit issues found; persistent report analysis/toolkit-issues.md. Rejected
attempt to annotate AA run inside already classified speech data was correct overlap
validation; existing data preserved. No toolkit modifications, publication, cleanup,
ROM changes or archive operations. RPC log retained (~72MiB); avoid repeating bulk
queries. Live PTY46898 remains available; session helper session.py, helper functions
helpers.py. New server must map.load and emulator.start then snapshot.import by path;
named snapshots alone do not persist. Reassert inputs after load.
