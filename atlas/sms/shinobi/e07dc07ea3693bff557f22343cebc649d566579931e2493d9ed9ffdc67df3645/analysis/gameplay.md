# Shinobi SMS gameplay familiarization
ROM title visually confirmed (title.png), SMS export-region header product 28681 v1;
precise commercial revision identity remains unconfirmed.

External hypothesis: SMS Power forum https://www.smspower.org/forums/14870-ShinobiMasterSystem
reports DOWN + Button 1 at title for mission/level select. Search also found
GameFAQs entries mixing other platform cheats; those were not used.
Down+button1 tested in step2 with correct SMS input shape; screen transitions black,
selection/gameplay result pending in step3. Prior button/pressed tests were ineffective
because SMS requires `buttons` array. They do not falsify the cheat.

Runtime controls supported: arrows/button1/button2 and console Pause NMI. Attack,
jump and progression behavior remain to be confirmed visually. Do not assume the
mixed-platform GameFAQs TG16 Select route applies to this ROM.

Confirmed against this ROM: title down+button1 starts gameplay, but does not by
itself display level selector. Mode3 code (mode-52a.txt) checks C019&3==2 before
showing selector, bounds stage C092 to0..18. Exact built-in cheat sequence not yet
recovered; direct initialized-stage RAM recipe is reliable.
Natural city gameplay visible after-release.png. Artificial stage3 shows elevated
platform/enemies,6 helicopter,10 mask-wall,14 rotating mandala,18 ninja in dojo.
These are visual descriptions, not verified canonical level names. Attack/jump
buttons were exercised with recorded inputs but individual animation semantics
have not been exhaustively audited. Press/release contract is buttons array.

Round002: bonus shooting visited artificially using C000=86 stage-clear/bonus
flags from retained clean gameplay. Long door animation requires more than420
frames; continuation150 frames reaches input/wave/actor loop. Left/right/button1
exercised; first-person hands/targets visible. See round-002/bonus-review.md.
This corrects earlier final-boss/ending interpretation of58C4. Genuine credits
remain unproven; no claim that they are absent.
