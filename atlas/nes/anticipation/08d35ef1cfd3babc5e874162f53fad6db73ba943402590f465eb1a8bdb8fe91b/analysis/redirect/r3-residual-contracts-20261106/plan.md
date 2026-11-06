# R3 residual contract review
Input r3-disassembly-final; current readable source b8380ee7f0580fb5bd0967bf3686b0ee2aa435261780beea853ce00e6a913d76; 65522 classified,30 unknown. Research only.
Primary prior ambiguous-fragments.json, R2 decode/review and R3 independent review retrieved. Changed method: test residual subintervals against newly accepted PPUMASK suffix rather than inherit full-fragment rejection; inspect scratch/buffer contracts and exact double-write postconditions. Stop each inference when competing storage interpretation remains equally supported and no independently justified contract can distinguish it. No runtime replay planned unless a specific discriminating test emerges.
Frontier: inspect $0702/$07FF field consumers and rectangle-return register/PPU postconditions. Single BPL and isolated RTS remain individually ambiguous; absence of callers is not a veto.
Disk 8.6GiB available; analysis/scratch4KiB; no cleanup authorized.
