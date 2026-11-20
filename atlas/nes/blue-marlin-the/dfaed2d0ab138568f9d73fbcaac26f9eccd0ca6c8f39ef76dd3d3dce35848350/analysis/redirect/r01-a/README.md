# Redirect round 1 — attempt r01-a (2026-11-20)

Input checkpoint: pass-001 (maps/pass-001.json = maps/current.json, sha256 da6cea30...; 17,738 unknown).
No map/state/source changes were made; this directory holds research evidence only.

## Tools (run from the workspace root)
- probe.py — rpc helpers (snapshot import/export, RAM poke, aggregate tracing, merge coverage into scratch/observed-r01a.json).
- newobs.py — observed opcode addresses that are not Code in maps/current.json (-> newobs.txt).
- script_parse.py — script-engine/PrintText parser (-> scratch/script_parse.json; copy here).
- static_rd_r01a.py — copy of analysis/tools/static_rd.py with OBSIN/RDOUT env (logic unchanged); roots in static_roots.txt.
- rd_delta.py — static_rd output vs current map (-> rd_delta.txt). project.py — projected residual (-> projection.txt).
- native_islands.py — $2F ... JSR $F363 islands (-> native_islands.txt). frontier.py, imm_ptrs.py, cls.py — checks/viewers.

## Evidence
- observed-r01a.json.gz — coverage contexts/edges from r01-a traces (fight entry from bite, landing+weigh-in, keep,
  fight event 4, day end, results with round win + all-tournament ending, Hawaii round 1 to sea). Same format as
  scratch/observed.json (round 1).
- static_rd_r01a.json.gz — static descent using round-1 + r01-a observed roots plus static_roots.txt.
- script_parse.json — runs [start,end,role] (roles script/text/native/switch-table), switches, refs, issues.
- frames/ — selected screenshots of the milestones.

## Reproduce
    gunzip -c analysis/redirect/r01-a/observed-r01a.json.gz > scratch/observed-r01a.json
    python3 analysis/redirect/r01-a/script_parse.py
    OBSIN=scratch/observed.json,scratch/observed-r01a.json RDOUT=scratch/static_rd_r01a.json \
      python3 analysis/redirect/r01-a/static_rd_r01a.py $(cat analysis/redirect/r01-a/static_roots.txt)
    python3 analysis/redirect/r01-a/rd_delta.py scratch/static_rd_r01a.json
    python3 analysis/redirect/r01-a/project.py -
(scratch/observed.json is round 1's merged coverage: gunzip analysis/evidence/round-01/observed-coverage.json.gz if absent.)
