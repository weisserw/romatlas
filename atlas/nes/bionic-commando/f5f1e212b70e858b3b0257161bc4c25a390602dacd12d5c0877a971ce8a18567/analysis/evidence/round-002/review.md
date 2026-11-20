# Round 002 independent disassembly review

Input round-001-final-tables, map/state93cee40f9b4155b0230113cdb24f0bc550ddb2964c957f5fac942125a498cef8.
One bounded disassembly pass; no unchanged gameplay replay. Saved artificial broad
progression includes later-room/boss visits and terminal 2010.8.2/Joe signature;
natural final shot and full timed credits remain unresolved. Reuse scenarios.json
and round-001-a snapshots/recipes. No ROM/toolkit changes or publication.

Fresh configured-binary decode matched instruction bytes, assembly, lengths,
control successors and terminal flags for all19393 primary graph instructions in615
bounded contiguous chunks. All local direct calls/jumps and relative branches land
on independently decoded starts; no instruction self-overlap, known-data conflict or
inline-table conflict. BRK E000 remains excluded. Cross-slot calls were reviewed
against fixed helper contracts rather than the fixed decoder's default28/29 view.
All new fixed code views explicitly leave lower slots unknown; effective30/31 slots
remain stable under established MMC1 mode. Lower-bank graphs use their supported
entry contracts, not claims that every selector occurs naturally.

D7E1 saves X, doubles A, pops JSR return low/high, reads two bytes at return+1+2*A,
restores X and JMPs through0019; it never returns to inline words. D6A0 vectors
there. Audio8556 similarly consumes its return address but indexes with X. Table
bounds combine preceding dispatcher and independent following handler origins,
consistent object-state increments/returns, or explicit actor<$7F guard. The actor
extended table's FFFF sentinel is Data, never a code root. Indexed handlers use
consistent object fields, motion/animation APIs, timers and control transitions.
DCA8 restores remembered65; DCAA serializes temporary selection; DCBE updates65
then serializes. E995 selects3 before AC0D; CB34 selects3 before AC00. CC24 selects13
before800C; CA5F/67/6F select13 before A401/A9A5/AAD6. These are physical pairs6/7,
26/27 and fixed30/31. No fixed-graph lower-slot references are promoted blindly.

## Correction: executable body inside proposed metasprite shelf

The redirect's generic28010..2BB70 proposal contained a coherent executable
island, visible in the block2B110 and already hinted by gameplay.md. Independent
review traced fixed CA0D: area!=7 -> LDA#0A/JSR D7CC -> JMP B07A. Effective banks
are20/21. New transfer-graph.json/.txt retain108 instructions and one two-entry
inline table at B088..B08C. B07A calls sprite initialization, loadsCC, calls local
B085 dispatcher and tail-calls renderer. State0 B08C initializes PPU/resources,
clears camera through C6AF->D421, builds area digits and optional display string,
then INC CC. State1 B11E installs animation and position, decrements6B/6A timer,
updates area completion/equipment fields and changes CB/CC=1 with CD/CE=0. It
returns B16E. Thus241 proposed shelf bytes must be Code, not Data. Four inline
bytes remain Data; following B16F records remain noncode. C6AF physical3C6BF..3C6C2
is an additional three-byte JMP D421 vector, absent from the original proposal.
All transfer fixed callees are in the validated fixed graph except this vector;
D421's eight-byte B6 clearing loop is independently supported and already included.
No animation firstframe overlaps the corrected executable island.

## Correction: unused audio table constants

Audio script preserved eleven unused cinematic table constants after the two
actual8556 callsites85C2/87C0. Packaging emitted their words as pointer evidence,
but the bytes do not have that role. They are already within independently inspected
audio-resource generic Data; retain that classification without pointer-table claim.
Only85C5 seven words and87C3 ten words receive dispatcher-pointer explanations.
This correction changes evidence granularity rather than classified byte counts.

## Data and extent review

All233 retained256-byte-or-smaller data blocks were reread against immutable ROM.
Independent boundary triage checked code-shaped call clusters, established control
transfers and neighboring code. The corrected B07A island is carved out before
annotation. The B9xx shelf cluster is repeated sprite tuples, not a coherent code
entry. Fixed PPU resources contain encoded lengths/addresses and display command
bytes; their frequent20-byte values do not create consistent instruction boundaries.

Both112-pointer room tables end exactly at their minimum pointed record origin.
Independently checked91 five-byte FF-terminated and40 seven-byte signed-terminated
lists: every row/terminator matches ROM, no byte cursor wraps, consumers AC10/AC6E
copy the corresponding fields. Use specific record/pointer annotations here. The
surrounding coordinate/timing resources remain generic Data after consumer review.
141 constant A:Y animation installations agree with pointer bytes and positive
count1..42, four-byte sprite records and duration bounds; DA8D, DA7D and D99x
establish record representation. Three cinematic streams close using negative
duration pointer grammar. Other shelf gaps retain generic Data, not complete stream
claims. Motion descriptor sources are exact five-byte contracts; dual-use code takes
precedence. Generic room/scene/dialogue/audio/display/collision/numeric blocks have
clear repeated pointer/low-alphabet/tuple/quantized-component structure corroborated
by consumers in primary research; no exact original generator/schema claim needed.

Classification is structural Static provenance. It neither claims new execution
nor dead/unused code. Historical rejected premises do not veto supported contracts.

## Refreshed residual pass

After the connected batch was saved and byte-identically rebuilt, bounded residual
inspection accepted375 more generic Data bytes in eight exact spans, retained in
residual-data-results.json: uniformFF tail2BF94+28, palette shelf3D8B8+128,
small-domain/player lookup blocks34DB5+64,34312+32,3595B+16,373D7+56,3652F+19,
37B0F+32. Absolute indexed consumers, adjacent returns/code and full immutable
bytes were inspected; the signed-word shelf at373D7 is visibly repeated word-pair
resource data, with no original table schema/domain assertion. These classifications
do not require natural selection. Final exact201 residual intervals/5625 bytes are
in unknown-ranges.json; this is a nonterminal research frontier, not exhaustion.
