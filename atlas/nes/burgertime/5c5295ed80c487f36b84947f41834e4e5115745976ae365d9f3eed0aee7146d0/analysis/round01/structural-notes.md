# Initial structural review
NROM-128 immutable physical PRG [0x10,0x4010) maps at CPU $C000-$FFFF
and aliases at $8000-$BFFF. Effective 8 KiB slots [0,1,0,1].
Header [0,16) is validated iNES metadata. CHR [0x4010,0x6010) is
cartridge pattern ROM: 512 16-byte NES two-plane tiles, independently bounded
by header PRG/CHR sizes. It cannot be CPU code under mapper 0.
Vectors [0x400A,0x4010) hold NMI $FF7F, reset $FD24, IRQ $FD24.
Reset initialization and NMI register-save/restore/RTI contracts verified.
715 vector-root static instructions and 1838 merged game-root instructions
were decoded individually (decoded-game.txt/json). Coherent branches, JSR/RTS,
RAM/PPU/APU routines; no inline-call parameter contract seen. Unresolved indirect
barriers are main-state, level-layout command and audio command dispatch.
The main dispatcher at $FD4F doubles RAM $16, reads word from $C000 and
jumps through $0005. Seven words [0x10,0x1E) terminate at independently
reached first code $C00E. Their pointed routines have state-handler contracts;
validate each as code before classification, without claiming natural selection.

The seven handlers were expanded as a connected static batch (4179 instructions,
no overlapping instruction boundaries). Review confirms controller/pepper, entity
sprite processing, enemy spawning/collision/death, ingredient mechanics, stage
initialization, score display, PPU transfers and audio interpreter. Indirect jumps
remain traversal barriers, never presume their fallthrough. All normal branch/call
paths are structurally supported without claiming unexecuted paths Observed.
Main-state table endpoints are [0x10,0x1E) and all seven targets form coherent
state handlers that return to $FD4F. No table word alone is counted as code proof.

Expanded indirect handlers (decoded-expanded.json/txt) reviewed: 1064 additional
instructions have coherent entry contracts and connected flow. Layout-command
handlers consume A command and Y stream index; motion handlers consume X entity,
Y OAM offset and $32/$33 proposed position; ingredient handlers consume Y record;
audio commands consume Y stream index, X audio state. Separate dispatch table
extents are provisional except layout nibble domain (16 entries); do not classify
unproved table extents on target-pointer plausibility alone. These supported
routine contracts justify Static code even when runtime selection is unresolved.
