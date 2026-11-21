# Redirect r1-a1 working notes (round 1, input checkpoint pass-001)
Frontier log (append as work proceeds).
1. L-010 DPCM: table tiling verified (tools/rl.py). bank2/3 samples 16464..31313, bank4/5 32848..48336; residue 16437..16464 / 32821..32848 identical in both banks; $AA fill 31313..31372.
2. L-005 sound: tools/sound.py parses SongTable(27 words)->headers->channel entries->envelopes/scripts; 12903+145 bytes, 0 conflicts; spans in sound-spans*.json.
3. Bank 13 $B745..$BFFF byte-identical to bank 1 same CPU range (build residue).
4. L-001 rejection cause: never-taken BNE at $9025 after LDA #$00 (phys 4149) reaching CharToTile table $9028 (43 bytes, $30..$5A).
5. Closures (tools/closure.py, with LDA#imm branch pruning, misalign check) for slot targets, L-012, L-013: clean.
6. Bank 7 tail: TrickShotScripts + 7 scripts tile $B1EF..$B2D1; two 8-byte tables to $B2E1; $B2E2..$BCF4 identical to bank 5 (residue).
7. Bank 0/1: screen objects self-delimiting (word0=size-2, word1=size-4); chain $9D7B..$B943 (tools/screens.py).
8. Residual complement after proposal: residual-ledger.json (7526 bytes), refs into residuals: resid-refs.txt (no control-flow refs).
9. Gameplay: TourStage ($AB) patch 6/5/7 in players menu -> ending / sound test / voice test (artificial). Traced ending run killed at 1500 s job limit (tracing ~10x slow); screenshots kept in evidence/.
Frontier: none open; remaining avenues listed in analysis/handoffs/round-1-redirect.md.
