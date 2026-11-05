# Round 1 source readability assessment

Current-source readability complete for the unchanged pass-001 structural map.
Every classified source interval is reviewed; none is deferred. Unknown intervals
remain unknown with explicit physical-boundary/role notes. This is separate from
whole-ROM classification, execution coverage and final editorial acceptance.

The preserved scaffold has 1762 entries, all reviewed with useful source links.
All 15111 code bytes are covered by functional units:423 code units across213
separated physical code groups. Four data units describe header, CHR tile planes,
vector words and pause shortcut. Generic data uses concise role/boundary/uncertainty
notes backed by the existing disassembly's pattern/reference reviews; format
uncertainty is not mislabeled classification uncertainty. The source contains973
physical-bank-qualified labels and296 address/field/table aliases.

Representative source entry labels (in commentary/source/game.asm):

- B08_DecodeNoteCommands explains packed command ranges, slide tuples and note state.
- B10_LoadScreenResources explains address-prefixed tile runs, text conversion,
  sentinels, attributes, OAM and initial palette.
- B12_RenderUnflippedFrame/B12_EmitOAMComponent explain four-byte components,
  flip bits, clipping and OAM capacity rather than repeating individual opcodes.
- B14_ResolveActorFrame/B14_AdvanceActorFrame explain nested state tables and the
  $FFFF state-transition record; pointer targets remain independently unproven.
- B15_WorldToMapPointer explains coordinate/page quantization, subcell bits and
  map-bank selection. B15_RequestCameraDirection explains queued direction changes,
  speed convergence and transfer gating; names agree with direction dispatch.
- Actor/weapon nonlocal returns explicitly describe discarded caller frames.

Consistency review checked concrete note boundaries against accepted opcode starts,
corrected camera left/right naming, kept an unknown consumer out of reviewed code,
scoped scratch aliases for distinct subsystems and verified byte-identical rebuilding.
Table-origin equates name consumer mechanics without establishing complete table
extents. Some exact game/weapon/state meanings remain unknown and use neutral names.

Evidence: [scope](round1/commentary-scope.json),
[inventory checks](round1/commentary-readability-check.json),
[review metadata](../commentary/review.json),
[source](../commentary/source/game.asm),
[symbols](../commentary/source/symbols.inc),
[verification](../commentary/verification.json),
[reading copy](../commentary/reading-after.asm).
Frozen maps/state and input are unchanged. Final editorial audit remains pending.
No source readability gaps are left for redirect; its remaining work is structural
and semantic investigation of R1-L03/04/05/07, detailed in uncertainties.md and leads.json.
