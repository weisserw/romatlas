# Conan gameplay research, round 1

ROM title screen confirms Conan, System 3/Mindscape 1990; matches US NES game.
[Manual controls](https://www.digitpress.com/library/manuals/nes/Conan%20-%20The%20Mysteries%20of%20Time.pdf)
and [nikomaru's NES walkthrough](https://gamefaqs.gamespot.com/nes/587196-conan/faqs/65409)
provide hypotheses: Up jumps vertically, Down forward, A/B attack, Start pauses,
Select opens weapon selection. Stage progression includes cave, ruins, hydra,
ship, forest, castle and tomb. Confirmed locally: Start on title enters cave;
right moves, down attempts forward jumps, A/B attack inputs exercised, Select tested.
Exact effect of each attack and weapon menu remains frame-inspection dependent.

[libretro primary cheat database](https://raw.githubusercontent.com/libretro/libretro-database/master/cht/Nintendo%20-%20Nintendo%20Entertainment%20System/Conan%20(USA).cht)
suggests level=$52, lives=$53, health=$06AD, inventory=$06D3-$06DF.
These are hypotheses, not ROM structural evidence. Local code independently
confirms $52 selects eight parallel stage tables and $DB15 guards index >=8.
$DE8F-$DEA7 consumes completion flag $04AA==1, increments $52, resets scheduler
and schedules $DB15 full stage initialization. Thus write stage-1 to $52 and 1 to
$04AA from clean cave snapshot; do not jump PC or patch ROM.
Local test stage3 initializes Viking ship and clears flag; later milestones ongoing.
No commercial ROMs/assets downloaded. Forum primary cheat author page returned403;
libretro source was available, no guide limitation blocks work.
