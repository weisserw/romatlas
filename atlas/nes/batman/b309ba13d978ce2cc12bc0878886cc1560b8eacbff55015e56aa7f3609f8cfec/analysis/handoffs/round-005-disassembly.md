# Round 5 — disassembly handoff (user-authorized manual fallback)

- Scope: narrow, user-directed. Classify the residual unknown bytes as generic Data of unknown meaning under the user's
  batch-only exception. No gameplay, no redirect research, no other map changes.
- Input pass-004 (analysis/state.json). Output checkpoint **pass-005**: maps/pass-005.json, generated/pass-005,
  [analysis/pass-005-verification.json](../pass-005-verification.json) ok=true, byte_identical=true,
  map_sha256 a320e4a34c4f4fdc231fce9157507779675cf0ddc2b9bd400cb5aef9552b2dba.
- analysis/state.json is identical in regions to maps/current.json (2,195 regions); map.validate ok.
- Tool: analysis/tools/r5_apply.py. It aborts unless the unknown complement is exactly the expected range and <=10 B.
  Pre-edit backups: scratch/r5/state-pass004-before.json, scratch/r5/current-pass004-before.json, scratch/r5/leads-before.json.

## Authorization
The user explicitly authorized a batch-only exception: when at most 10 physical ROM bytes remain unclassified, mark them
as generic Data of UNKNOWN MEANING with Manual provenance. This is a user-directed fallback. It is not observed data and
not statically proven data. It adds no semantics and no label.

## Range
- Verified unknown complement over the whole 262,160 B file (analysis.unknown, plus an independent map.get coverage
  sweep with 0 overlaps): exactly file offset 116142 (0x1C5AE), length 2. This is PRG bank 14 (fixed at $C000), CPU $C59E-$C59F.
- Neighbours unchanged: Observed Code 0x1C527+135 ($C517) before it, Observed Code 0x1C5B0+166 ($C5A0, HideUnusedSprites) after it.
- New region id 2265: kind Data, provenance Manual, confidence 0.5, no label, no views. Its evidence text states:
  user-authorized manual fallback, not observed, not statically proven, and refers to the L-018 reviews.
- State diff against pass-004: 0 regions removed or modified, 1 added.

## Prior ambiguity (L-018, unchanged)
Reviewed in analysis/redirect/r2-a1/l018-review.json and analysis/redirect/r3-a1/rejection-review.json:
- No static references: no word, RTS-1, immediate-pair or branch encoding targets it, in any bank.
- It was never fetched in any trace (pass-001, r1-a1, r1-a2, r2-a1, r3-a1).
- It sits directly after the jump engine's unconditional indirect JMP ($C59B).
- It decodes as a zero-page load whose result is dead on every path into HideUnusedSprites, so it has no entry contract.
- The bytes are not a uniform, repeated or non-instruction pattern.
- Therefore neither a code claim nor a fill/pattern data claim was supported. That remains true: the Manual annotation
  resolves the classification gap only by user direction, not by evidence. L-018 status stays exhausted-unknown (history
  entry appended, `map_classification` added). Its reopen conditions still apply and would supersede the Manual region.

## Completion
- Unknown complement: **0 B**. ROM classification 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
- Evidence-backed (Observed/Static): **262,158 B** (99.9992%). Manual fallback: **2 B** (0.0008%). Unknown: 0 B.
- Per kind: Code 41,123 B (28,697 Observed / 12,426 Static); Data 88,611 B (88,609 Static + 2 Manual);
  Text 1,354 B (Static); Image (CHR) 131,072 B (Static); Unknown 0 B. Per-bank counts unchanged from round 2 except PRG 14 (+2 B Data/Manual).

## For commentary (map changed)
- Preserve the round-4 commentary in analysis/round-4/ (same procedure as rounds 1-3). Then rebuild the baseline from pass-005,
  carrying names, comments and review records forward 1:1 by physical range. Only the new 2 B region at 0x1C5AE ($C59E) is new.
- Comment it as "generic data of unknown meaning (user-authorized manual fallback; not observed/not proven; see L-018)".
  Do not give it a meaningful name or format. Do not describe it as code or as a joypad entry.
  Update any existing comment that calls these bytes "unknown/unclassified" so that it says the same thing.
- Refresh the reading copy and verification, then do the final review and the strict editorial audit.

## Leads / toolkit
- L-018: history entry appended; status kept exhausted-unknown ([analysis/leads.json](../leads.json)). Pass record round 5 added.
- No toolkit issues found. TK-001..TK-003 unchanged in [analysis/toolkit-issues.md](../toolkit-issues.md).
