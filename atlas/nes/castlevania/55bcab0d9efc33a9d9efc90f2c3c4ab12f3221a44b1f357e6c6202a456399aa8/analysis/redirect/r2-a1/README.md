# Redirect round 2, attempt r2-a1 (input checkpoint pass-002)

Run scripts from this directory (symlinks input/maps/scratch -> workspace; lib.py copied from r1-a1).

- refs.py -> refs_out.txt: bank-aware reference/consumer census for the 6 residual bytes
  (branch/JSR/JMP targets, absolute operands, indexed-read windows, word and word-1 values in all non-code bytes).
- indirect.py: census of indirect transfers (only JumpTableDispatch `jmp ($0000)` at $CA39; no pha/pha/rts).
- idiom.py: rts directly after an unconditional transfer in classified code (precedents $E3B2, $FAFE) + padding fill values.
- secrts.py: referenced sec;rts / clc;rts exits (paired carry-exit idiom in bank 6).
- frag.py / l003.py: context listings and target-alignment check for the L-003 fragments and residuals.
- leads_before.json: copy of analysis/leads.json before this pass.
- emu/: own bridge (socket /tmp/ra55bc_r2redir.sock, stopped), stage-6 boss scenario SC-BOSS6
  (s6a.py route, s6fight.py fight, snapshots, screenshots, cov_s6.json observed starts).
