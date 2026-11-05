# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 001 current-source review

All currently classified source has functional commentary. Unknown physical gaps are explicitly identified beside their local binary emissions; their role remains unresolved. Data-pattern inspection from the disassembly evidence is reused rather than relabeled as semantic proof.

- Scene descriptor `$8000..$8009` supplies five stream/metasprite pointer fields. Later scene-header fields are described by their consumers, but their complete underlying tables and object formats are not claimed. Generic resource spans remain generic data.
- CPU aliases in the switchable scene window depend on effective PRG banks. `SceneCpuTarget_*` and `UnresolvedBank*Target_*` retain numeric entry contracts where the destination is not locally resolved. A descriptive fixed-bank function name does not prove mapper preservation after a call. The next investigation should prove individual leaf/call contracts before expanding return continuations.
- Object arrays separate type/state, horizontal and vertical stream IDs/cursors, animation stream/cursor, display flags and coordinates. Slot0 is the player. Exact gameplay meanings of many stream IDs, animation IDs and player flag bits remain unresolved; source explains the established selection and mutation mechanics instead.
- Scratch `$00..$17` is deliberately named by storage identity because rendering, collision, resource upload and dispatch reuse it. Sound `$18..$1F` and sound-record offsets have local mechanical names; remaining sound control bytes retain RAM identity. Song/effect titles and the precise note tuning are unknown.
- `StageResultLow/High` is a two-byte quantity rendered as decimal digits, incremented on a sixty-frame divider and accumulated into stage/player records. Its exact game scoring/time unit is not asserted.
- Classroom split timing and NMI ordering are explained from source, not claimed cycle-exact or fully hardware compatible.
- Ending state code queues text/poses and initializes map/palette state, but the saved artificial route produced malformed composition. Correct CHR/preparation and natural ending reachability remain open. Boss/checkpoint spawning remains open as in the disassembly handoff.
- Source-review unit intervals group readable entries and may include explicitly explained unknown gaps. They are not structural map annotations or claims of original routine extents.

Concrete redirect work remains R001-CALL-CONTINUATIONS, R001-STAGE-HANDLERS, R001-ENDING-PRECONDITION and R001-BOSS-CHECKPOINT in `analysis/leads.json`. These are future structural/runtime investigations, not a readability backlog.
