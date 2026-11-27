# Round02 commentary maintenance records

The authoritative deliverable is commentary/source plus its constants/local image,
review.json, uncertainties.md, progress.md and verification.json. The complete
pass001 commentary is preserved at analysis/round-2/commentary-pass001.

migration.json records exact carried ranges and the original pending frontier.
units.json records92 new explained mechanical units with corrected boundaries and
current source anchors. Current inventory-check.json and checkpoint.json supersede
that initial frontier. Prior177 units retain valid credit; affected dependency prose
was revised in maintained source and review.json. Source locations are refreshed
from physical headers and actual labels, not stale pass001 line numbers.

The Python scripts and new-code.asm are historical local edit/inspection recipes.
They are not idempotent and must not be rerun over maintained source. In particular,
later boundary/prose inspections corrected draft thresholds, branch polarity and
speaker-template semantics. Use final source/review/units.json as authoritative.
No toolkit edits, structural annotations, runtime replay, cleanup, archive or publishing
were performed. Final editorial audit remains pending.
