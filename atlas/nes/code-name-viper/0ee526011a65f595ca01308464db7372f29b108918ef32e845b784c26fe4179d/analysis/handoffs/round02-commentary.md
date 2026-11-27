# Round02 commentary — current-source gate complete

Pass002 map/state unchanged: de61d9a48d3713139a2fb103019075d49b8740ce5986b1c42bc5c0a96e905dd1.
ROM remains0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d.
Whole-ROM classification73.72% (193274/262160);68886 unknown bytes in233 intervals.
This is a full current-source readability handoff, not full classification or final
editorial acceptance. Overall goal stays active. No redirect/disassembly performed.

Prior full commentary preserved at analysis/round-2/commentary-pass001. Fresh baseline
prepared for pass002; all636 unchanged classified entries carried by exact physical
bounds/kind/evidence. Prior177 mechanical units and valid review credit reused.
All445 new Code fragments (3817 bytes) read and explained in92 additional units;
all64 new non-code entries reviewed and233 unknown intervals explained honestly.
All1378 scaffold entries remain intact/reviewed with refreshed physical source anchors;
all972 Code/18409 bytes and173 non-code/174865 bytes covered. Exact remaining
current-source readability frontier: empty. Final editorial audit remains pending.

Authoritative maintained source: commentary/source/game.asm plus constants.inc and
local image; commentary/review.json, progress.md and uncertainties.md. Helper unit
inventory: analysis/commentary-round02/units.json; inventory-check.json and checkpoint.json.
Historical scripts are not idempotent; do not rerun them over maintained source.

Source now explains map flight/introduction/mission flow, player input/airborne/item/
scripted-state mechanics, actor dispatch/contact/culling/metasprites, score/life addition,
dialogue scripts/pointers/templates, terminal wait, reset and frame helpers. Concrete
inspection corrected unit boundaries and branch polarity/counts/field roles. The two
24-byte dialogue speaker templates are three PPU transfer descriptors each, not OAM.

L007 resolved in source:84 matcher candidates but54 stored numeric records; all30
extra candidates contain non-digit bytes and cannot match editor digits. L008 resolved:
$c6ab is immutable$a2; AND$08 is zero, so BNE is not taken and pending-column
attributes upload. Original intent remains unknown. $f8ce waits X marked NMI frames;
transition calls use twenty, not mapper selection. Natural victory/encounter eligibility
is not established by artificial routes or static behavior contracts.

Live questions for the next authorized phase:
- L011: establish descriptor/resource-selector to effective-PRG-bank association before
  specific graphics/terrain names. Follow SelectRoomResources124955..125158 and the
  eight nine-entry split pointer arrays126812..126884; do not assume equal indices.
- L011: SaveMapPpuBytes15623..15654 has only14 stored address bytes15654..15668.
  Stage7 reads opcode$a5 as high byte and masks toPPU$2500. Original shared-byte intent
  versus tolerated overread unresolved; never extend the array into following code.
- L012: RestoreCheckpointRoom126174..126305 selects bank10/11, stage*8 OR checkpoint
  bits$06 indexes$b746/$b747 into$a2/$a3. Follow actual consumers/finite stream bounds,
  plus bank0 six/eight-byte spawn records and fixed$d3c4 room row pointers. Reuse saved
  evidence; unchanged gameplay replay does not answer these consumer/boundary questions.
Other unknown item/terrain/audio/artwork meanings remain explicit in uncertainties.md.

Fresh final verification commentary/verification.json: ok=true, byte_identical=true;
reading-after.asm refreshed afterward. game.asm SHA256
2300ffa6d4c5118f7f6ca716846b7d022a07a888d6c54a1c0734f77ae53cc169;
constants.inc29cbf23061b12040a3a61ed209f8e29898826f6598f065d26b9e08939e2d8b2a.
No toolkit issues found or edits made; no ROM/map mutation, cleanup/deletion, archive,
publication or commercial asset download. All generated source/assets remain local.
