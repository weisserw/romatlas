# Round02 commentary — full current-source readability complete

Current pass002 map/state SHA256:
de61d9a48d3713139a2fb103019075d49b8740ce5986b1c42bc5c0a96e905dd1.
The entire pass001 commentary, including source, frozen baseline, review and reading
copies, is preserved at analysis/round-2/commentary-pass001 before fresh preparation.

Carried636 unchanged classified physical regions by exact bounds, kind and evidence,
including valid prior names/comments/review and177 mechanical units. Read all445 new
Code fragments (3817 bytes) and64 new Data/Text entries; added92 mechanical units.
All972 Code regions (18409 bytes) and173 non-code regions (174865 bytes) have source
explanations and current review records. All233 unknown intervals (68886 bytes) have
explicit honest boundary notes. The1378-entry full-ROM scaffold remains intact;
no current-source range or readability review is pending. No cosmetic whole-source
rewrite was needed for sound carried prose. Inventory check and current unit anchors:
analysis/commentary-round02/inventory-check.json and units.json.

New explanations cover map flight, stage introduction/mission screens, player action
buffering/drag/gravity/terrain/item/scripted-state handlers, actor dispatch/culling/
contact/metasprites, decimal score/extra-life handling, dialogue data/controls, final
scene loops, fixed-bank frame services and reset completion. Bank-aware symbolic
references and contextual RAM/field/table names preserve exact addressing modes.
Concrete full-inventory inspection corrected semantic-unit boundaries, a redundant
countdown branch, the dialogue templates' PPU descriptor role, the constant-ROM
condition's fallthrough polarity, descriptor counts and a24-pixel state threshold.

Affected prior prose now incorporates L007's84-candidate/54-record password overread,
L008's immutable$a2 & $08 ==0 condition, and$f8ce's twenty-frame transition wait.
L011/L012 remain explicit, bounded semantic/record-boundary questions in uncertainties.md:
resource selector versus effective PRG bank, stage7 PPU address alias, and bank10/11
checkpoint streams plus bank0 room/object records. No further runtime replay was needed.

Final verification.json: ok=true, byte_identical=true. ROM SHA256 unchanged:
0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d.
Final game.asm SHA256:
2300ffa6d4c5118f7f6ca716846b7d022a07a888d6c54a1c0734f77ae53cc169.
Final constants.inc SHA256:
29cbf23061b12040a3a61ed209f8e29898826f6598f065d26b9e08939e2d8b2a.
reading-after.asm refreshed after the final rebuild. Final editorial audit remains
pending and was not invoked. Whole-ROM classification remains73.72%, not complete.
Overall goal remains active. No map/ROM/toolkit modification, cleanup/deletion,
archive or publication. Disk had3.3GiB available; helpers228KiB before final notes.
