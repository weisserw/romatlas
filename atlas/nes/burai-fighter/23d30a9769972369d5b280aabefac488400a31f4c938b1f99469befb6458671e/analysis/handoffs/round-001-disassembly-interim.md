# Round 001 disassembly interim checkpoint

Phase ongoing, not ready for commentary. Baseline verified; current internal state and map hold 34895 classified / 30657 unknown of 65552 total (53.23%). Code 2105 bytes / 969 instructions; CHR 32768, header16, vectors6. Evidence directory analysis/round-001; startup.snapshot frame102; difficulty.snapshot natural attract frame1852. Named snapshots are not required. Replay scripts use configured binary through run.py.

Important correction: NES emulator.input uses {buttons:[...],port:1}, not generic {button,pressed}; initial accepted calls silently passed an empty array. Current play.py uses correct arrays and is capturing natural menu/start and gameplay samples. Initial timeout advanced to frame102; its state was inspected and persisted. No toolkit changes, ROM edits, deletion or publishing. No final stopping decision yet.
