# Round 3 disassembly handoff (pass-003)

Resumed from analysis/state.json (pass-002); no new baseline. Leads applied with tools_local/pass003_annotate.py.

## Checkpoint
- maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json: ok true, byte_identical true (map sha256 ba45ade7...).
- analysis/state.json saved, maps/current.json exported, map.validate ok.

ROM classification: 100.00% (262156 / 262160 bytes classified; 4 bytes unknown). ROM INCOMPLETE (rounded figure).
Code 48908 (Observed 18178, Static 30730), Data 80115, Text 2061, Image 131072; 1936 regions.

## Lead outcomes (one criterion)
Unreferenced, never-fetched bytes are Static Code only with an independent structural constraint beyond legal decoding (operand resolving
to an established code entry in the same effective bank, or a coherent multi-instruction routine contract). A lone 1-byte RTS lacks one.
- L009a verified: bank 13 $BA5D-$BA5F ($1BA6D-$1BA6F) `jmp DebrisSpawner` -> Static Code, reachability not claimed.
- L009b rejected: bank 15 $FB69 ($1FB79), $FDC0 ($1FDD0) remain Unknown.
- Consistency: region 1853 split; bank 6 $A175 ($0C185) and $A18B ($0C19B) returned to Unknown. New regions: Code $0C186-$0C19A (APU pulse
  init, $A176) and $0C19C-$0C1B0 (APU noise init, $A18C).
- L010 verified: region 1500 split; new Data $1121E-$11222 label EmptySpriteDef8 (with comment); MapSpawnTable block now $111A7-$1121D.
- L011 not touched (commentary-only).

## Commentary notes
- Re-anchor by address: region ids 1500 and 1853 removed; new ids 1942 (L009a), 1943/1944 (bank 6 APU inits), 1945/1946 (bank 8).
- Commentary: label $B20E EmptySpriteDef8, render TransformFrames words with it, render the $BA5D jmp as code, mark the four lone RTS bytes
  as Unknown (raw .byte $60) in bank 6 and bank 15; apply L011.

## Remaining unknown (4 bytes)
$0C185, $0C19B, $1FB79, $1FDD0: each a lone `$60` directly after a routine's rts and before the next classified routine; no reference
(consumers.py, rtsrts.py), no fetch in any route. Reopen only with a reference/fetch or an independently justified source-pattern model
that discriminates dead code from a 1-byte data remnant. Rejection review: decisive evidence = negative consumer/caller search; it rules
out known data consumers and known callers equally, so neither role is supported.

## Toolkit
No new toolkit issues found (TI-001..TI-004 unchanged, analysis/toolkit-issues.md). Server stopped. Disk ~3.1 GB free.
