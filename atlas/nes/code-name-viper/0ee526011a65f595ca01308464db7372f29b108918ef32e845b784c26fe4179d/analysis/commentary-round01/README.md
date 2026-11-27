# Round01 source edit records

units.json preserves the 177 reviewed mechanical explanations. source-locations.json
points to their labels in the final source. labels.json records the initial symbol
pass; branch-renames.json records concrete subsequent refinements. The final source
and verification hashes are authoritative.

The helper scripts are historical edit recipes, not idempotent commands.
annotate.py expected the freshly prepared pass-001 baseline source. Do not rerun
them over the maintained commentary/source. Preserve the current source, frozen
before/ and review credit when carrying work into later rounds.
