# Round 2 disassembly handoff (pass-002)

- Scope: one pass consuming ready redirect leads L006, L007, L008 (static verification and
  annotation). No gameplay needed (S01-S10 already reach the ending). L002 not retried (blocked).
- Input: pass-001 / analysis/state.json (all prior classifications preserved; two 1-region Data->Code
  replacements: 3:$9E4A-$9E8D, 3:$91B9). Pre-pass state copy: scratch/state-before-pass002.json.
- Checkpoint: pass-002 (maps/pass-002.json, generated/pass-002).
- Verification: analysis/pass-002-verification.json ok true, byte_identical true,
  map_sha256 6fd86fe7...c0df; map.validate ok; maps/current.json == maps/pass-002.json.
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

## Per-bank counts
| Space | Code | Data | Unknown |
|---|---|---|---|
| iNES header (16) | 0 | 16 | 0 |
| bank0 | 21199 | 11569 | 0 |
| bank1 | 1365 | 31403 | 0 |
| bank2 | 3897 | 28871 | 0 |
| bank3 | 8845 | 23923 | 0 |

## Lead outcomes
- L006 verified: 8 ranges / 150 B -> Code (Static, 0.75-0.85; labels NoCaller_*/Bypassed*).
  Structural evidence (exact boundaries, established targets, bypass jmps, byte identity) recorded
  separately from reachability (none found; not Observed; not claimed unused).
- L007 verified: 0:$844C, 0:$E0B7 (unknown) and 3:$91B9 (Data) -> Code, consistent "rts after tail jmp"
  decision, confidence 0.6 (weakest claims in the map).
- L008 verified: 3:$9E4A-$9E8D Data -> Code (MulSignedB3, byte-identical to 0:$D518 / 2:$84E9).
- L002 blocked (toolkit RAM/uploaded execution views); L005 open (data semantics, commentary).

## Unresolved / deferred
- Reachability of the 12 no-caller code ranges remains unknown (no caller, pointer, or fetch found).
- RAM routine images 0:$F796-$F805 and 2:$C730-$C794 stay Data in ROM until toolkit support (L002).
- Commentary should pick up new labels (NoCaller_*, Bypassed*, MulSignedB3) in commentary/ source.
- Toolkit issues unchanged: TK-001, TK-002 (analysis/toolkit-issues.md).

## Artifacts
analysis/evidence/round-02/: frag-verify.json, leads-before.json, tools/verify_frags.py,
tools/apply_l006_l008.py.
