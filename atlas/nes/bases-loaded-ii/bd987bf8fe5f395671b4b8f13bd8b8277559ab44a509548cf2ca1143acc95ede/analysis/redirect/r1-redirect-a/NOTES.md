# Redirect attempt r1-redirect-a (round 1, phase 3) - working notes / frontier
Input checkpoint: maps/round1-disassembly-b.json (sha256 2b95d9dd...; 375320/393232 classified).
Status: COMPLETE (handoff analysis/handoffs/round1-redirect.md; recommendation: disassembly cycle).
Tools: tools/survey.py (unknown-range refs), jt.py/jt2.py (jump-table words), walk2.py (static_walk
engine + proposed tables), residual.py, gapref.py, classify_res.py, cfeat.py, exhaustive.py,
covcheck.py, rt.py (runtime helper), leads_update.py.
Outputs: tables.json, walk2-c.json, proposals.json, unknown-ledger.json, rescat-c(2).json/.txt,
gapref-c.txt, cc-dis.txt, residual-final.txt (20 bytes), runtime.json, observed-in-proposals.json,
snapshots/, experiments/.
Frontier for later redirects: 20 residual bytes; natural callers of L-015 islands; L-009/L-011
runtime semantics (long traced at-bat needed); password-entry flow.
