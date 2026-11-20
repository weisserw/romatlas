# Commentary progress — round 001 complete current-source pass

Baseline: round-001-final-tables, map SHA-256
93cee40f9b4155b0230113cdb24f0bc550ddb2964c957f5fac942125a498cef8.
Prepared baseline was byte-identical. Frozen before/, baseline.json and
phase1-state.json preserved. Current map/state remain unchanged.

Reviewed the full 6,687-line initial listing, then maintained source/game.asm and
symbols.inc independently. All 747 physical inventory entries are reviewed:
298 Code, 233 Data, 16 Image and 200 Unknown. Added 298 compact code-unit records
with effective bank views and source locations. Classified scope is 8,474 Code,
53,229 Data and 99,328 Image bytes. Unknown scope is 101,129 bytes, honestly noted.
No readability range is deferred. Whole-ROM classification remains incomplete.

Names and source prose cover reset/NMI/controller polling, title/overlay and main
state dispatch, camera/scroll queues, graphics transfers and bit reversal, audio
priority/stream/envelope processing, player/grapple/projectile setup, object
animation/visibility, collisions and MMC1 serialization. 181 exact-value aliases
name hardware, RAM, structure-of-arrays bases, slot offsets, local fade scratch and
audio record fields. Bank aliases remain distinguished by physical label suffixes.
Unclassified callees and scratch roles retain explicit uncertainty, not invented
semantic names. Generic data notes reuse the preserved local inspection evidence.

Coherent batches verified byte-identical after initial naming, audio-field and
format prose, fixed-ABI correction and final slot/field consistency corrections.
Final fresh verification: commentary/verification.json reports ok=true and
byte_identical=true. ROM SHA-256 is unchanged. Final game.asm source SHA-256:
5fc68e7aa45317343e4b3404b80a9147295402d361f2436ca7bda78a959d6220.
Final symbols.inc SHA-256:
15809e9431d6c8cc289e6e077e10e5ee0c43909131bd13202d0ff78b12afb6b7.
reading-after.asm refreshed after final verification. Final editorial audit was
not run: archive/editorial acceptance remains pending by phase contract.

Concrete unresolved questions are in uncertainties.md and the disassembly handoff.
No toolkit edits, ROM modifications, cleanup, deletion, archive or publication.
