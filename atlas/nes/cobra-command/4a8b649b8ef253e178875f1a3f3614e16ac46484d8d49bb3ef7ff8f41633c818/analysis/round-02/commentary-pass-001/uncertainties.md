# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Inline sprite-zero descriptor classification
CPU $cecd..$ced1, physical $1cedd..$1cee1, effective fixed banks [14,15] in upper slots. ReserveSpriteZero ($ceb4) copies four bytes indexed Y=3..0 to OamShadow, while a BNE skips the descriptor and the next callable routine starts at $ced1. This independently anchors a four-byte Y/tile/attribute/X descriptor; phase1's single Code region includes it as PHA/ADC encoding. Source now explains the distinction but retains byte-identical assembly. Redirect question: verify exact instruction/control boundaries and propose the four-byte Data split; no live execution is required for the consumer-anchored role.

Other reviewed fixed-bank uncertainties: $82 inhibition purpose, sound selector/event names, overlay-patch game trigger, player-terrain flag semantics and projectile type names. Source states exact mechanics and leaves game names tentative. Unknown gaps, including the 88 bytes following BeginSharedPlayerContactEffect, remain structurally unknown and explicitly opaque.

## Scene sound tables semantic correction
Physical $1ec83..$1ecb3 / CPU $ec73..$eca3: QueueSceneSelectedSound ($ec58) chooses one of three 16-byte tables, then QueueSoundCommandPreservingPrg selects MMC1 PRG register six and calls its $8003 driver command entry (physical $18013 JMP $811c). Thus these are sound-command bytes, not CHR selectors as phase1 evidence said. Structural Data extent remains supported. Source and review corrected; map evidence remains frozen for later reconciliation.

## Player reach / actor slot seven
Reviewed $a660..$a93d (physical $e670..$e94d) establishes an active player-linked reach animation and alignment flags for actor slot seven. A rescue interpretation is tentative. Its sprite emitter has a 32-byte unknown continuation. Precise proximity thresholds and state cells are published as R1-C003 in analysis/leads.json, refining R1-L004 without repeating the failed boss patch. Missing prerequisite: source/scene contract for a correctly initialized actor seven.

## Specific bank-return contracts
The reviewed fixed wrappers distinguish saving selector shadows, writing hardware and intentionally selecting an exit bank. $ec2f has a narrow potential preserving contract through sound bank six command acceptance; $fce4 exits in bank four. R1-C004 refines R1-L003 with these specific postconditions. Lower-slot bank identities still require the MMC1 control-mode contract; pointer values and generic callee success do not establish it.
