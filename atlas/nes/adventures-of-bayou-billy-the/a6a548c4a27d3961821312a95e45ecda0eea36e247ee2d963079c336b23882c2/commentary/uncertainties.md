# Commentary uncertainties — round 03

Current-source readability covers all classified source and the four unknown intervals.
This is separate from complete ROM classification and terminal editorial acceptance.
Analysis names describe mechanical roles; they are not recovered original symbols.
CPU RAM $40..$7F and object pages overlay different mode structures. CPU addresses,
effective PRG views, physical ROM offsets, PPU addresses and OAM remain distinct.

C01 is resolved structurally: physical $14010..$1401F contains five three-byte
variant arrays. Mode 2 selects banks 10/11 and enters at CPU $800F. The old
$8000 instruction path lost that correlation; only five Static records/13 bytes
were invalidated. The preserved round01 directory retains its original question.
Consumers at $81EF/$805F/$80D7 and the independent round02 review support the new
source description. This is not proof of natural selection of every variant.

Four bytes remain Unknown: $C7A6..$C7A7, $14379..$1437A, $1ACD0..$1ACD1
and $1EE00..$1EE01 (physical half-open ranges). Each is an isolated $60 byte.
The preceding routines are complete and use other return sites; no supported entry,
consumer, finite table or template resolves code versus stored scalar/padding.
Source notes retain both possibilities. Artificial entry would not discriminate.

Physical $C1E2..$C1E5 is now Static Code BrawlEnterGameOverState. The independent
round03 review combines transition-family placement, exact neighboring bounds and
the complete delegated state-3/substate-0 operation through CPU $E84B. That alias
starts at the overlapping LDA #3 inside the $E84A BIT operand and returns normally.
Matching accepted terminal tails are at physical $1E32F and $1EAF2, not $1EAE2.
No live caller or natural entry is claimed. See analysis/disassembly/round03-20261031-a/review.md.

Unresolved game meanings remain explicit near their mechanical descriptions:

- Audio: precise song/instrument identities, full stream/envelope schemas and exact
  physical DMC sample extents. The eleven descriptors program $4010..$4013, but
  apparent start/length values can cross code in saved contexts. Encoded slabs
  remain generic data after consumer/neighbor/pattern inspection.
- Sprite/terrain: original actor and sprite identities, individual flag meanings
  and natural use of every valid record. Counted/shared metasprite and scene
  column/metatile/attribute formats are bounded and explained.
- Dialogue/PPU: font tile codes are not ASCII; narrative transcription is not
  recovered. The variable command/palette/tile pool is not uniformly FF-terminated.
  Ending dialogue and credits mechanics are explained without claiming a natural
  boss defeat/ending route.
- Driving: exact object identities, projection generator/rounding and real-world
  units are unknown. Projection stride/border mechanics and the explicit 153-byte
  ROM-to-RAM raster upload are established; RAM execution alone proves no other
  source origin.
- Generic small numeric/control pools and repeated inter-asset bytes have bounded
  reference/consumer/pattern/neighbor review, but no invented finer schema. Short
  local boundary/uncertainty notes are sufficient for those fallback blocks.

Banked wrappers require mapper busy zero and a balanced software bank stack.
Unknown low banks stay unresolved. Copied reset templates distinguish the local
pre-INC view, the copied JMP bytes and the actual post-reset fetch in banks 14/15.
Readable static source does not imply natural reachability or complete hardware
or game compatibility. No new discriminating structural lead arose in this pass;
redirect may reconsider the residuals only with a new entry/consumer premise.
