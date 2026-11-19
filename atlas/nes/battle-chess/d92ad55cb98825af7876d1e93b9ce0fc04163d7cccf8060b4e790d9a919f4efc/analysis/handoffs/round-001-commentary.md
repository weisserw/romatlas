# Round 001 commentary handoff

**Current-source readability complete; ready for redirect.** No unreviewed classified backlog or deferred readability ranges. Whole-ROM classification remains incomplete: 186573/262160 (71.17%), 75587 unknown. Final editorial audit remains pending; not invoked.

Checkpoint unchanged: `pass-001-final`; current/frozen map SHA256 `77d79b7545def6fc5d9738059f783b53336ee32af10757ce39ae0a1e8d84ee59`. Input ROM SHA256 remains workspace identity `d92ad55cb98825af7876d1e93b9ce0fc04163d7cccf8060b4e790d9a919f4efc`.

Covered every current Code/Image/Data span plus honest boundary notes for all unknown spans: 4899 code bytes in banks 0/28/30/31, 139264 planar raster bytes, 42410 data bytes including header/vectors. All 1246 scaffold entries reviewed; 174 code entries/control blocks inventoried in [review](../../commentary/review.json) and [units](../../commentary/units.json), with exact classified slices rather than invented full-routine bounds. [Coverage validation](../round-001-commentary-review.json).

Examples: six-byte menu records and indirect callbacks; row-major board squares and cursor formulas; held-mask/repeat handling; seven audio stream slots and instrument/event mechanics; stack-page PPU queue and literal/repeat resource decoder; palette fades; four logical/rendered object arrays and script callback/count records; clipped/mirrored three-byte composite sprite records. Hardware/RAM/fields/branches and consumer table aliases are named consistently, with conditional scratch reuse and bank constraints explained. Generic non-code retains only supported structure; raster-to-nibble bank19 is explicitly mixed. No classification map mutation.

Final fresh [verification](../../commentary/verification.json) passes byte identity using configured release binary. [Reading copy](../../commentary/reading-after.asm) refreshed after final source edits. Preserve buildable commentary/source with its local image asset, symbols and frozen before baseline; the portable map cannot reproduce prose. Maps/current.json, analysis/state.json and input/game.nes unchanged.

Redirect questions ([details](../../commentary/uncertainties.md)):
- Setup Done: inspect bank30 CPU transitions $C292/$C295/$C2AF and $C2F0/$C2F3, physical unknown spans `$03C2A2..$03C2D3` and `$03C300..$03C353`; resolve selected menu record/callback and readiness/validation before further input. Prior near-mate snapshot is editor-only, artificial; no accepted game/endgame claim.
- Menu bank0 $8C9A consumes six-byte records through RAM $89/$8A; recover base-pointer producer/menuSelector 1/3 contracts and bound actual selected records.
- Fixed-bank script decoder $E2EB uses callback-word/count-byte records at $E42D (physical `$03E43D`). Use supported opcode domains/handlers to bound tables and streams.
- Resource descriptor consumer $C9C7 indexes pointer words at $CB41 (physical `$03CB51`); direct resource IDs and raw/literal/repeat upload semantics provide concrete consumer anchors.
- Audio bank28 table consumers $842E/$8446/$845E are named but bounds/commands/request meanings remain unresolved. Reused raster/noncode evidence supports banks2..23; asset grouping, nibble semantics and original generator remain open.

No terminal rejection review/exhaustion claim. No agents spawned, toolkit changes, downloads, publication, archive or destructive operations. All source/evidence remains local. Preserve existing checkpoints and gameplay provenance.
