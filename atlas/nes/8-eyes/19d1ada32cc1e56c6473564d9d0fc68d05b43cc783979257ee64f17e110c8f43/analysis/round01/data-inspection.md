# Round 01 non-code inspection
Physical banks 0–3, $000010–$008010 exclusive, contain repeated low-valued
16-column stage-like grids in 4KiB halves, trailing parameter records and repeated
four-byte NES-palette-like records. Bank4 $008010–$00A010 contains repeated
four-byte tile-index groups, bit-packed $55/$AA attributes and long uniform runs.
Banks5–7 $00A010–$010010 repeat common leading arrays (e.g. 32-byte $8B
rows, ramps at +$100), parallel small-value fields, tile-index sequences, bounded
sparse object-like records, FF separators/tails. Patterns inspected throughout
at each 512-byte slice and boundaries, plus complete repeated-run scan. All
static candidates start at or above physical $010010. This absence is a conflict
check, not the affirmative data proof: repeated grid/record/array organization
supplies that proof. Indexed reader at CPU $8000,Y exists in observed fixed-bank
code (round01-gameplay source); exact individual consumers/schemas are deferred.

Annotate each full 8KiB physical bank as generic Data, not specific table schemas.
No original source grouping, reachability or table-size inference is claimed.
Uniform FF intervals in higher PRG banks are classified independently at maximal
run boundaries; nonuniform neighboring bytes stay unknown. No semantic fill label
is assigned. Review if later instruction evidence conflicts.
