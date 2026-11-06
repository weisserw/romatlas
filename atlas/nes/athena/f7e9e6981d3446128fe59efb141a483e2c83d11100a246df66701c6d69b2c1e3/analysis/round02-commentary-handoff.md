# Round 2 commentary handoff

Current-source readability is complete for the frozen round02-disassembly-reviewed-final classification. This is the continuation of the interrupted round-two phase, without preparation or regeneration. The full round-one commentary remains preserved at round-2/commentary-round01-preserved.

Classification remains 99.98% (131,061 / 131,088 bytes): 33,017 Code, 98,044 Data including the header, and 27 Unknown bytes in nine intervals. No map, classification, toolkit, ROM or evidence boundary changed. maps/current.json and analysis/state.json still match the commentary frozen baselines; map SHA256 is 6b3b9c949a34093a1c23d94fcb4ba421c10049f3b5d8bf93946862397b5aa39f.

The maintained source now explains all currently classified routines, control-flow families and data structures. The last 108 Data intervals received practical byte/neighbor/consumer review and source explanations in small applied batches. Existing sound terrain, audio, sprite and earlier code prose was retained. All nine Unknown intervals retain honest boundary/competing-role notes. The scaffold frontier is empty; all 698 regions and 473 compact units are reviewed, with current source locations. Carried units were checked against physical bounds and same-kind spans; code units carry explicit physical 8 KiB bank contexts. See round-2/commentary-consistency-final.json and commentary-location-refresh.json.

The consistency pass named all remaining numeric conditional branches, understood RAM/object fields and ROM lookup operands, and every numeric/unresolved direct JSR/JMP operand. Historical generated/CPU aliases remain compatible definitions, with no live unresolved_cpu operand. Context-specific scratch names distinguish attack reach from contact damage and terrain probe side from graphics count. Meaningful table views are named without asserting unknown original extents or disjoint asset layouts.

Examples of substantive reconciliation:
- The saved-selector wrappers at CPU $e702/$e710/$e722/$e728 select bank six and call its graphics upload wrappers. Prior bank-zero descriptions were commentary errors; accepted execution views/bounds are unaffected.
- Scene $b7fa increments the packed-decimal timer; $b72f decrements immediately while bypassing the timed gate/counter reset. Wrappers and the increment entry label now identify those mechanics.
- Explicit fixed-bank consumers establish bank-two input/jump/attribute tables, bank-one score addends and bank-three auxiliary/counter views. A former bank-six association for $b4f7 was corrected. The current-cache view at CPU $d001 selects the final two entries of the four-pointer backing at $cffd.
- Stale first-round statements about unclassified continuations were reconciled with the now-explained scheduler, audio, sprite, health and player bodies.

No known current-source editorial gap remains in this phase. Remaining uncertainty concerns actor/game identities, natural selector domains, some internal data/asset subdivisions, padding purposes and the nine unresolved byte intervals. These are explained locally and in commentary/uncertainties.md. Natural defeat/ending/credits progression and full hardware/game compatibility are not established. The bounded historical direct-reference index omits immediate/indirect and unresolved fixed-bank consumers; empty entries never established no consumer. No evidence from the commentary corrections invalidated accepted Code views/roles/bounds, so no new structural redirect lead was created.

Fresh supported verification passed byte-identically and the reading copy was refreshed using commentary.py read. Source SHA256:
- game.asm: 3e3d0e3867ee6a81072e9e640e729b13758303975a747a511728001452e23826
- constants.inc: 3dbd69c2a6e52732e793843266402817a1bacdb1d436203c3da562f659dd46b8
- reading-after.asm: 6be42a0f6066eb14c776f933831203cf60caa7d21bb0e27aa640207d0a0038bc

Input ROM SHA256 remains f7e9e6981d3446128fe59efb141a483e2c83d11100a246df66701c6d69b2c1e3. Verification is recorded in commentary/verification.json. All source/assets remain local. No archive, cleanup, deletion, publication or final audit was performed. editorial_audit remains pending for the coordinator's terminal editorial/archive acceptance; this handoff does not establish whole-ROM classification completeness.
