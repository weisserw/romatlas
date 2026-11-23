# Round 3 / disassembly handoff

Scope: one disassembly pass on pass-002 state applying ready redirect leads R2-L023..R2-L026
(role, bounds and provenance refinements). Baseline not re-created; input/game.nes unchanged; no
toolkit edits; no gameplay (natural ending already reached in S13; no question needed new runtime evidence).

Checkpoint: **pass-003** — maps/pass-003.json (sha256 4f2a310bf6ebc8a8ca7e9105d0050027b8ac46a6afea8b1ea308041fd12bb149),
generated/pass-003, analysis/pass-003-verification.json.
Verification: map.validate ok; analysis/state.json == maps/current.json == maps/pass-003.json (same sha256);
`run.py checkpoint pass-003`: ok=true, byte_identical=true. Per-byte Code/Data/Padding kinds identical to pass-002.

ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).

Status: **incomplete** (2 B unknown, bank 4 $BB30-$BB31, R2-L021; exact 99.9985%). Per-bank counts in
analysis/progress.md ("Round 3 / disassembly"). Regions 2,877 -> 2,976 (1,603 code, 1,247 data, 126 padding);
Observed code 19,759 -> 24,315 B.

## Findings (details analysis/investigation.md "Round 3")
- R2-L023 bank 5: roulette tables re-segmented on proven bounds, plus new downstream structures:
  message pointer table $AA75-$AB5C (4 tiers x 29 words via zpMessageTable) and 114 strings $AB5D-$B634;
  $94F3-$9506 item block referenced by the $94A3 init records; opponent name ptrs/names $A897-$A8B4.
- R2-L024 bank 4: $A0C1 records are poker **hand-name** records (HIGH CARD..ROYAL FLUSH), not chip labels
  -> commentary name B4_DrawChipStack ($9449) is probably wrong (draws hand names). Credits line
  pointer table $B761-$B7C4 identified (new).
- R2-L025 bank 6: streams overlap, so one region per stream entry (113), cut at the next entry; each
  region's evidence names its header refs and actual stop address.
- R2-L026: 104 regions Observed in place; 63 split into Observed and Static runs (artificial-scenario caveat).

## Regions changed (for commentary carry-forward; full list analysis/tools/round03/changes.json)
Data, removed ids -> new segments (CPU, bank):
- b5 $947D-$9506 (id 2248) -> $947D-$94A2 wheel pos | $94A3-$94F2 init records | $94F3-$9506 items
- b5 $952A-$960D (2680,2249,2826,2250) -> $952A-$953B row ptrs | $953C-$960D cell rows
- b5 $9EB8-$A327 (2258..2261, 2725, 2831, 2832) -> $9EB8 class | $9ECA multipliers | $9ED1-$A070 cell list ptrs | $A071-$A327 number lists
- b5 $A88F-$B634 (2275..2307, 2726..2754, 2834..2837) -> $A88F eye frames | $A897 name ptrs | $A89F names |
  $A8B5-$A8D5 label strings | $A8D6 tier money | $A8F6 list ptrs | $A946-$AA74 lists | $AA75-$AB5C msg ptrs | $AB5D-$B634 msgs
- b4 $A06D-$ADC2 (2811..2813, 2193..2219, 2701..2724) -> $A06D menu strings | $A0C1 hand-name records | $A0ED tier bytes |
  $A0F1-$A248 msg ptrs | $A249-$A2DD hand names | $A2DE-$A46B strings | $A46C-$AD9B tier msgs | $AD9C-$ADC2 strings
- b4 $B761-$B9B3 (2223, 2814, 2224, 2815) -> $B761-$B7C4 credits line ptrs | $B7C5-$B8ED staff lines | $B8EE-$B9B3 banner rows
- b6 $8C8A-$9F6A (2318..2340, 2839..2860 range) -> $8C8A-$8CC1 header ptrs | $8CC2-$8DE6 headers | 113 stream-entry
  regions; leftover generic $8C72-$8C89 and $9F6B-$9FF1 (narrowed, original evidence kept).
Code: 63 Static regions split at executed/unexecuted instruction boundaries (observed-plan.json); 104
upgraded in place. No code boundary outside these splits changed; no labels added to the map (names stay
in commentary; suggested names in lead recipes and investigation.md).

## Leads
Consumed: R2-L023, R2-L024, R2-L025, R2-L026 (verified). R2-L021 not retried (no new evidence).
Produced: none new; commentary items below.

## For commentary (round 3)
- Rebuild from pass-003 (map changed); carry names by CPU address. Region headers changed as listed.
- Apply R2-L023 renames (B5_PickWinningNumber, B5_SettleBets, B5_SpinStart, RltWinningNumber $0718,
  RltWheelPos $0719, RltBetCount/RltBets); rename B4_DrawChipStack -> hand-name printer; name the new
  tables (roulette msg ptrs $AA75, credits line ptrs $B761, B6_Stream_XXXX); document the sound
  command grammar (analysis/redirect/r02-a1/findings.json F5, sndwalk.py).

## Unresolved / deferred
- R2-L021 $BB30-$BB31: unknown (rejection review in round-02-redirect.md stands).
- Consumers of $A8B5-$A8D5 (b5), $A2DE-$A46B and $AD9C-$ADC2 (b4) strings not mapped (roles text; structure proven by $FF tiling).
- Meaning of $94F3 items and $A0ED tier bytes; remaining R2-D02 generic windows (see round-02-redirect deferred list).

Artifacts: analysis/tools/round03/ (apply.py, covplan.py, observed-plan.json, sound-streams.json,
changes.json, applied.jsonl.gz). Toolkit: analysis/toolkit-issues.md — new TK-006 (minor API/doc mismatch);
TK-001..TK-005 unchanged; none blocking.
