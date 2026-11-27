# Round001 commentary handoff

Current-source readability complete. ROM classification remains incomplete:
101725/131088 bytes classified (77.60%),29363 unknown. Frozen checkpoint/map and
analysis/state.json are unchanged. Final editorial acceptance remains pending.

Read [../commentary/reading-after.asm](../commentary/reading-after.asm); canonical
editable source is [../commentary/source/game.asm](../commentary/source/game.asm)
with [../commentary/source/symbols.inc](../commentary/source/symbols.inc).
[../commentary/verification.json](../commentary/verification.json) freshly confirms
byte_identical=true. Input hash is unchanged. Frozen before/ and manifest retain
exact round001 baseline identity; review.json retains the full3495-entry scaffold.

Scope: all3303 classified instruction records, all classified data/image blocks
and all unknown boundaries reviewed;282 functional units and540 flow labels.
Established RAM/hardware/pool/field operands are named consistently. Explanations
cover audio channels, player movement, terrain, projectiles/thrown object,
stack-based yielding/inline dispatch, controllers, OAM streams, spawning, tile-row
uploads and two-bit terrain cells. Tile format/vector words and generic data limits
are in source. No classified-source readability ranges remain pending. Game-facing
meanings and natural reachability remain qualified where unsupported.

Concrete next questions, with exact supporting source in commentary/uncertainties.md:

- Physical $01C22B..$01C24B (CPU $C21B..$C23B) is independently read as32 palette
  bytes by UploadFlashPalette, yet currently classified Code. Review static branch
  fallthrough/fetch evidence before changing its role. The preceding BEQ follows a
 16-iteration DEY loop and takes its branch. The following decoded span through
  physical $01C254 has weaker palette extent evidence; do not expand the32-byte claim.
- Projectile spawn scans seven16-byte candidates at $0200..$0260; ordinary update
  and hit loops visit six. Inspect missing initializer/call contracts before naming
  the seventh slot or assuming a bug.
- Retain the stage-initialization/continue question for MissionIndex=$049D and
  AreaIndex=$049E, existing failed patch evidence, banked dispatch contracts and
  bank0 sequence/table boundary questions from the disassembly handoff. No new
  runtime progression or redirect research was performed here.

Toolkit: no new commentary findings. Existing NES-001 remains one nonblocking
confirmed documentation mismatch/missing schema capability; see toolkit-issues.md.
No toolkit edits, ROM changes, publishing, archive or final audit occurred.
