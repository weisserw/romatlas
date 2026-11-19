# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Bank-zero sound interpreter

The current Code byte at physical$576 (CPU$8566) is BRK, while the supported
pulse control helper at CPU$8515 reads$8566,Y as an alternate duty table.
This does not prove BRK reachability. Redirect should inspect its discovery
predecessor/entry contract and the bounded four-byte neighbor interval before
changing Code/Data classification. See source DecodedBrkAtDutyTable and
SelectAlternatePulseDuty. No structural edit was made in commentary.

Unknown spans interrupt primary request comparison, note-envelope initialization,
period output and duty-counter update. Source comments explain only classified
mechanics; bank-zero pointer/command tables remain unknown, despite consumers.
Sound period offset and sweep logic are preserved literally where the conventional
signed-add or channel-two exclusion interpretation would contradict instructions.

## Fixed frame services and runtime helpers

Controller reads and OAM/CHR/PPU handshakes are mechanically explained. Numeric
path/action/terrain selectors, special HUD tiles, persistent counter gameplay role
and some stage raster modes retain limited names. Raw dispatch tables and gaps
interrupt several path, response and rendering routines; comments describe their
classified fragments without asserting the missing bodies or table bounds.

The interval consumer at physical128231 uses only$03E5 as a frame duration; its
records are flags/count pairs. Earlier two-byte interval interpretations in banks8,
13 and fixed helpers were corrected after this direct consumer review.

Actor render scans at CPU$FDD6/$FE07/$FE3A compare negative active type bytes
literally against positive$1E/$1F without masking bit7. The equalities cannot hold
for the same loaded byte. Comments preserve and explain this apparent ineffective
special-type selection rather than silently repairing it. No structural change is
implied, and this is not a toolkit issue.

The forward actor render scan retries its active slot on a zero motion byte; the
backward scan advances instead. This asymmetry is explained literally in source.
