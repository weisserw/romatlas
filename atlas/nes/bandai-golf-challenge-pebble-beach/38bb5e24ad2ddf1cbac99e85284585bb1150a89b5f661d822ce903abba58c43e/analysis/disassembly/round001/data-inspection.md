# Non-code inspection

Physical PRG unknown complement was inspected after barrier-aware control-flow
classification, in every 256-byte window and at exact adjacent boundaries.
All available accepted-code direct/indexed references were reviewed. Header and
CHR extents come from validated iNES metadata. No residual was classified merely
because it was unexecuted or exported raw.

The largest interval physical $3261..$8009 begins directly after sound RTS $B250:
16-bit decreasing frequency values, short audio scalar arrays/pointers, repeated
note/duration/command records, then uniform FF bytes through physical $370F.
At $3710 CPU $B700 a monotonic pointer list addresses repeated compressed
course/tile streams. $9266..$927D reads adjacent pointers to derive input size;
$9281 copies the stream into RAM and downstream drawing interprets its repeated
commands. The pattern continues across the inspected windows. The $F3C2 table
(physical $73D2) is separately read by $92A4..$92B6 into $05C0..$05FF.
Late material/tile arrays have repeated fields and runs. Final uniform FF bytes
physical $7F6D..$8009 end exactly before independently validated vectors.
These independent boundary/consumer/pattern constraints discriminate data from
executable code throughout; per-record semantics and original organization are
not required and are deferred. The large combined annotation is Data, not a
claim that every byte belongs to one specific compressed table.

Small intervals are explained individually in internal map evidence. Particularly
$9CF1/$9CFF pointers point to sprite records consumed by $8BD6, not code.
$ACB5..$ACDA pointers are executable terrain-response dispatch targets, and their
connected official-opcode handlers were separately inspected and classified Code.
Standalone display-mask helper $8B2B..$8B2F has independently supported complete
LDA #$1E / STA $1F / RTS contract; natural caller remains unresolved.
