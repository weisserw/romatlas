# Round 001 disassembly handoff

Status: single substantial pass closed; ROM incomplete. Continue with commentary,
then redirect. This is an easy-gain handoff, not a terminal exhaustion assessment.

ROM classification: 61.42% (161031 / 262160 bytes classified; 101129 bytes unknown).

Final checkpoint: `round-001-final-tables`.
- Internal state: `analysis/state.json`.
- Supported portable map: `maps/current.json`.
- Frozen exact map: `maps/round-001-final-tables.json`.
- Readable-source starting point: `generated/round-001-final-tables/game.asm`.
- Verification: `analysis/round-001-final-tables-verification.json` confirms valid,
  byte-identical reconstruction. Input hash unchanged; no toolkit edits.
- Final counts, per-bank metrics and exact unknown complement:
  `analysis/evidence/round-001/final-counts.json`, `unknown-ranges.json`.

Classification: 8474 Code, 53229 Data, 99328 Image bytes; 547 regions all have
structural evidence. All provenance is Static intentionally; natural/artificial
reachability remains separately supported by saved contexts/scenario routes.
Execution evidence union: 2234 physical opcode starts, 2831 contextual starts;
not an execution percentage. Untraced waits excluded. Commentary is still needed
across all currently classified source; do not infer editorial completion.

Strong findings:
- Game is English Bionic Commando, mapper1, 256 KiB PRG and CHR RAM.
- Full graphics banks8..19 inspected as aligned NES bitplanes; packed dialogue font
  separately inspected. Local tile previews and `$D28D` upload consumer corroborate.
- Repeated screen-cell/structure intervals and uniform runs classified after local
  block/flank review; exact schemas deliberately not invented.
- Reset/fixed-bank hardware code, audio interpreter, sprite/room/player handlers,
  serializers, and three bounded inline dispatch tables classified with explicit
  effective bank views. Pointer targets were not promoted solely as pointers.

Broad gameplay:
- Natural startup/title/communications and overworld presentation visited.
- Clean durable map snapshot: `pre-shortcut-snapshot.json`, frame3002, in evidence dir.
- Supported artificial loader recipe: set `$4D=area`, `$4C=room`, `$3F=0`,
  `$CC..CF=[3,0,0,0]`, release input. Initialize100 untraced frames, then exercise
  right+B8 and right+A8 traced. See `analysis/scenarios.json` for budgets/results.
- Confirmed player/platform mechanics in area1; area12 exterior; neutral13 NPC;
  special area12 rooms96/98 and mechanical helicopter room101. Each has durable
  frame, snapshot, accepted contexts and candidate evidence.
- Final cockpit shot remains unproven: normal gun failed; selected weapon5 produced
  a black scene without a verified victory transition. Do not repeat unchanged.
- Main `$CB=4` cinematic shortcut reached base explosion and helicopter rescue.
  Latest `cinematic-late-snapshot.json` is frame6957, selectors[4,0,1,4,3], `$74=0`;
  credits/terminal not verified after bounded waits. `cb4-wait-2.png` and
  `cb4-wait2-0.png` show explosion/rescue. Natural final completion is not claimed.

Redirect questions / missing facts:
1. Resolve bank3 `$B903->$BB23` late-cinematic/credits continuation and required
   sprite/text/timer state. Blind waiting stopped producing useful evidence.
2. Resolve room101 entrance position, selected Hyper Bazooka/projectile and cockpit
   collision contract; use saved boss snapshot and local transition decode.
3. Review exact unknown complement against resource consumers: dialogue script in
   banks22/23, metasprite records in20/21, inline state dispatch in26/27. Mixed banks
   demonstrably contain code; broad blanket annotations would be unsound. Public
   ROM-map resource extents are hypotheses needing local boundary confirmation.
4. Expand supported direct-call/dispatcher structural batches with correct bank
   effects; current easy static calls were inspected, but indirect selectors and
   bank-changing return contracts need redirect research.

Historical observations and attempted-lead statuses are preserved in
`analysis/investigation.md`, `analysis/leads.json`, `analysis/gameplay.md` and
`analysis/scenarios.json`. Persistent RPC hosts exited externally(143); recovered
from durable state. Short self-contained probes retained their own evidence.

Toolkit report: `analysis/toolkit-issues.md`: two confirmed bug/documentation
findings, no blockers (NES discovery budget parameter name; observed provenance
fields). Workarounds used; toolkit unchanged. Disk4.1 GiB free, scratch160 KiB,
cited evidence about100 MiB; no cleanup, deletion, publication or archive performed.
