# Round 1 redirect handoff (attempt r01-redirect-a1)

Status: COMPLETE. Input checkpoint pass-001 (94.39%, 14,697 unknown). No map,
source, ROM or toolkit changes; research only.

## Scope
Static, bank-aware investigation of the commentary's structural questions and the
largest unknown clusters. Evidence: analysis/redirect/r01-redirect-a1/findings.json
(F1-F12), exact unknown complement with per-range lead/disposition:
analysis/redirect/r01-redirect-a1/unknown-ledger.json (271 ranges = 14,697 bytes),
tools/ (python3 -I; regenerate /tmp/rd/gaps.json with unk.py first).

## Key findings
- Global audit: no decoded control transfer with matching bank lands in any
  unknown range; all 36 unknown-bank fixed-bank JSR targets resolve to existing code.
- Bank 1: 21 RLE nametable streams (each decodes to exactly 1,024 bytes, contiguous
  $8400-$9EF8) plus raw status-bar table $8000-$83FF (LoadScreen_Tbl -> DecodeRleToPpu).
- Bank 4 $8000-$A473: 9,332-byte 4-bit sample (PlayDigitizedSample count $2474).
- Bank 5 $8000-$B844: complete event-data layout (throttle curves, terrain records,
  pointer tables, 144 metasprite frames, tile blocks, column and scenery records).
- Bank 6: HillNmiJump 13-word table + 643 bytes of code (descent covers all) and
  hill terrain tables/blocks/height profile.
- Bank 3 CPU path lists, bank 0 script tables/scripts and palettes, fixed screen
  records and SFX pointer table, and code islands with structural contracts.
- Bank 0 $8055-$805D is code (warm-start target after bank 0 select).

## Ready leads (expected bytes)
L-003 5,267; L-004 2,035; L-011 1,046; L-002 805; L-006 793; L-005 681;
L-001 646; L-016 478; L-010 459; L-012 363; L-009 157 (+9-byte Data->Code fix);
L-013 95; L-014 64; L-015 735 (low priority). Total ready: about 13,624 bytes.
Open: L-017 (1,073 bytes, residual data-sandwich gaps). Deferred: L-007, L-008.

## Deferred avenues
- L-017: consumers exist for most gaps, but extents weren't proven one by one. This
  needs per-gap generic-data review. Bank 3 track data may be bounded through TrackMapPtrs.
- Bank 2 music format: not decoded. Generic data is enough to classify it; decoding
  the format would only add the specific role.
- Gameplay (two-player mode, later races, natural ending): not attempted. The static
  leads above need no runtime facts.

## Unresolved questions
- Natural reachability of the islands in L-001, and of HillSpecial values 1-12.
- Bank 0 $B1E2-$B1E3 (`pla/tax`, no entry); NmiHandler2 $8592 bank; ScriptOp1D `jsr $0014`.

## Recommendation
Run another disassembly round with the ready leads above. This is not a terminal stop.
