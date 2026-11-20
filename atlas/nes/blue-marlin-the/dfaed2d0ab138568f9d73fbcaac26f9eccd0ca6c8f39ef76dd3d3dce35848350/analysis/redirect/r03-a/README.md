# Redirect round 3 — attempt r03-a (2026-11-20)

Input checkpoint: pass-003 (maps/current.json = maps/pass-003.json, sha256 703aba69...; 5 unknown bytes, lead L-021).
Research only; no map/state/source/toolkit changes. No emulator run (no runtime question discriminates these bytes).

Target claim: each of 0x02DC6/0x032FA/0x03353 (bank-1 RTS) and 0x1E106/0x1E1C3 (bank-15 NOP) is code vs data.
Prior barrier (r02-a): no branch/JMP/JSR/word/word-1/imm-pair(8 B) reference; indexed consumers bounded; "lone opcode".
Changed method (coordinator questions): references at addr-2..+2 in words, consumer-anchored split lo/hi tables at
any separation, imm pairs within 16 B; RTS-dispatch existence; script operand/hook domains and bank-relative encodings;
twin/analogue endings; whole-ROM consistency census (JMP;RTS followers, NOP sites, small inter-code islands, fill value).
Stop condition: either a reference/consumer appears, or the combined-constraint argument is written up for review.

Files (python3 analysis/redirect/r03-a/<script> > <txt>, run from the workspace root, read maps/current.json):
- rtsdisp.py -> rtsdisp.txt   Q1: RTS-trick sites in Code; words/imm pairs/raw split hits for addr-2..+2.
- splitcons.py -> splitcons.txt  Q1: 634 consumer-anchored (lo,hi) indexed-load pairs evaluated idx 0..255.
- bankrel.py -> bankrel.txt Q2: bank-relative / PRG-offset word encodings.
- twins.py -> twins.txt       Q3: preceding-context twins, JMP;$60 and NOP census, same-target analogues.
- refcheck.py -> refcheck.txt Q3: reference status of every JMP;RTS follower in classified Code.
- lonebytes.py -> lonebytes.txt Q3: small non-Code islands between Code and tiny Code regions (consistency).
- proposals.json              Ready proposals L-022 (NOPs) / L-023 (RTSs).
