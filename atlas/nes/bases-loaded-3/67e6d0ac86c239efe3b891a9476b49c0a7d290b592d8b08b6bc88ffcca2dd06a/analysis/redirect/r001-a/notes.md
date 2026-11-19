# Redirect r001-a research notes (round 1, input checkpoint pass-001)

Scratch: /tmp/ra67r (disposable). Tools: tools/ (paths fixed to this workspace / scratch).
Static pipeline (pure Python over input/game.nes + analysis/state.json):
  python3 farcalls.py            # FarCall sites, per-bank used indices and table words
  python3 jt.py                  # all jump-engine call sites and raw following words
  python3 allclosure.py          # -> /tmp/ra67r/closure.json
  python3 residual.py            # -> /tmp/ra67r/residual.json
  python3 datarefs.py > /tmp/ra67r/datarefs.txt   # needs /tmp/ra67r/residual.json
  python3 orphans.py             # -> /tmp/ra67r/orphan_roots.json (copy in evidence/)
  python3 candidates.py          # -> ../candidates.json (main / orphan / bank0/4-copy groups)
  python3 segment.py             # -> ../residual-after-candidates.json
  python3 bank6.py 820d          # bank 6 partial closure
Closure rules (closure.py): bank context [N,N+1,30,31]; roots = linear decode of pass-001 Code regions,
FarCall entries (counts FCN), jump-table entries; JSR to $F292/$ED2C/bank-local engines = inline table
(bound: min entry >= start, stop at Code/Padding, reject words $C000-$E7FF); complementary branch pairs,
`lda #imm`+beq/bne and ALWAYS set {26:$9B63,$9BC0, 28:$894D} are always-taken; JSR targets >= $C000
are not walked from banked closures (fixed code is pass-001-classified).

Runtime (artificial, RAM patched; daemon.py + rpc.py over run.py serve):
  gend.py/gend2.py/gend3.py loss path; gwin.py/gwin2.py win path; gnext3.py NEXT GAME; p2.py (went to EDIT);
  wtrace*.py attempts to locate the HomeTeam writer (unsuccessful: see handoff).

Findings summary: handoff analysis/handoffs/round-001-redirect.md.
Negative results:
- Stadium->bank 0/2/4 hypothesis refuted (Tbl_26_82AB all bank 2; no other $3C writers; R6/R7 only via FarCall/reset).
- No caller anywhere in PRG for bank 31 $F208/$F2A9/$F2D5/$F322/$F365/$F3CA/$FF00, bank 26 $93BA/$96AB/$996C.
- $0356 is not a games-remaining counter (scoreboard stat; read for hit power).
- HomeTeam writer: no absolute/abs-indexed store except initialisers; Write trace event lacks PC (TK-004);
  replay-to-cycle attempts did not reproduce the write within 300 frames (input timing), stopped.
