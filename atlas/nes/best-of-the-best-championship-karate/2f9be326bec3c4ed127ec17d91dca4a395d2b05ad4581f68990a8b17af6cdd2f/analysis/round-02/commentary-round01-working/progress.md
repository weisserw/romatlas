# Commentary progress

Round 01 current-source readability complete. Reviewed all 11992 classified code bytes, 59787 classified data bytes and every unknown interval boundary. The source has 5796 instructions, 412 specific mechanical code anchors and 711 named flow targets. All 370 code scaffold intervals carry reviewed semantic units; the full 503-interval scaffold is intact, with 415 units including header, fighter records and vectors. There was no previous commentary credit to invalidate. No current-source range is deferred.

Names and prose cover fighter action requests/animation, interval and six-byte box collision, resource/damage and referee event logic; title/menu selection and dispatch; reset/NMI, PPU uploads and VRAM queue formats; music/sample/noise drivers; sparse pose decoding, CHR staging, OAM and HUD; signed comparison, division, multiply and fixed-bank veneers. Hardware, stable RAM fields, arrays, table addresses and selected field/constants are used in operands. Mainline scratch lifetimes and shared RAM aliases are explicit; unresolved callees remain numeric.

Accuracy checks corrected the shared IRQ/reset target $CDDC, sixth-frame audio skip mechanism and shared renderer/frame-snapshot RAM alias. Direct ROM inspection confirmed secret-name bytes `  LAYER   ` at physical 251649..251659. The classified consumer sets second fighter resource to ten; ending/progression is not claimed. VRAM copy consumer reads bank from the first command's unindexed $6A despite per-record bank storage, now stated accurately.

Fresh final verify passed: commentary/verification.json; byte_identical=true; original ROM SHA256 2f9be326bec3c4ed127ec17d91dca4a395d2b05ad4581f68990a8b17af6cdd2f. Source game.asm SHA256 d05fe2c3b261a49ab2b51c96dc1345404a5db24ba7628e4d760d55221d806992. Reading copy refreshed with commentary.py read after final source edit. Source links and all code-unit presence checked separately; no final editorial audit invoked. Editorial/archive acceptance remains pending.

Structural map/state unchanged. ROM classification remains 27.38% (71779/262160), with 190381 unknown bytes. Unresolved meanings and concrete consumer/dispatch/graphics leads are in uncertainties.md. Prepared frozen baseline and historical evidence preserved. No toolkit changes, publishing, archive or cleanup. Disk has 4.4 GiB available at handoff.

## Round 02 active

Input round-02-disassembly, map SHA256 f3e913592366f0db58098a8979d579c519b26589952491f666f31d8cb22b7a0b. Preparing fresh baseline; entire round01 tree preserved in analysis/round-02/commentary-before. Frontier: carry previous code/data by exact physical ranges and byte identity; all 13630 newly classified code bytes and 176744 data bytes pending review. Seven Unknown bytes remain explicit. No phase completion or editorial acceptance claimed.
