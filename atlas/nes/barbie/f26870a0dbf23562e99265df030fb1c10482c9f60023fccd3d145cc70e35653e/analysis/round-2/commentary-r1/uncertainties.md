# Commentary uncertainties — round 1

All currently classified code/data/image/text has source explanations. These are
unresolved meanings or structural investigations, not a naming/comment backlog.
Readability does not establish whole-ROM classification: 75,907 bytes remain unknown.
The frozen map/state and all prior disassembly evidence are unchanged.

- **Resource/RAM execution.** `ReadChrResourcePayload` (physical $01d44b,
  CPU $d43b; fixed PRG 14/15) reads directory bounds through PPU, copies a span to
  RAM $0200 and may jump there. The normal PPU queue shares that RAM. This establishes
  mechanics, not a PRG upload origin or a complete CHR payload schema. Redirect
  should capture the CHR register value, directory index X, copied bounds and accepted
  RAM fetches for one loader invocation; link them by backing identity before
  interpreting any missing code. Continue R1-003 rather than inferring origins from PC.
  **Role caveat:** the current whole-CHR Image annotation inherited a header/section
  argument. The header establishes CHR backing storage, not a graphics role for
  every byte. PPU-loaded directory/data/executable payloads may require structural
  subranges with corrected role/provenance. Redirect must reconcile this distinction;
  commentary leaves maps/state unchanged and makes no full-image-role endorsement.
- **Stage completion and initialization.** `FinishPendingStageLoad` ($01d6a7,
  CPU $d697) is partial, while `RequestSceneTransition` ($01d3f5, CPU $d3e5)
  has a clear request contract. `StageCheckpoint` ($04d2), `StageReturnScene`
  ($04d8) and `StageLoadPending` ($0528) expose a bounded route to inspect boss,
  exit and ending transitions. Scene 5/11 presentations do not prove the true ending.
  Follow R1-002 with a clean snapshot and supported scene/checkpoint request; keep
  artificial reachability distinct from normal progression.
- **World/drawing records.** `ResolveWorldMetatileRecord` ($01c29c,
  CPU $c28c) multiplies the index by **eleven**, not ten: three shifts and three
  additions of the original index. Tile columns expand eleven records to 22 tiles.
  The header parser resolves five parallel tables, relative sections, palettes and
  page ordering. Many consumers/table aliases lie in unknown backing ranges; names
  establish no extent or classification. Bound record domains through the actual
  resource header and page-order consumer before proposing new typed regions.
- **Object contracts.** Sixteen handler IDs, two streamed groups, six visible display
  slots and seven moving entities have mechanically named fields. `ObjectState`
  ($97+X) and `ObjectParameter` ($062b+X) are deliberately handler-specific: linked
  slots, persistence IDs, timers and fractions reuse them. `PlayerRecoveryState`
  ($04d3), scene activity fields and command IDs lack complete game meanings.
  Inline records following spawn/interaction calls retain opaque bytes when their
  immutable structure was not classified. Unknown code bodies are not explained as
  if decoded; classified conditions and bypasses are explained explicitly.
- **Non-code tails and graphics.** Generic data retains prior boundary/pattern
  inspection in analysis/disassembly-r1/data-boundaries.json and map evidence.
  Full specific schemas/consumers are unresolved and are not prerequisites for
  generic non-code readability. R1-001's banks 10/11 consumer investigation remains
  actionable; no reinterpretation/classification was attempted in commentary.
- **Audio/UI names.** Music notes, envelopes, modulation, four effect slots and APU
  ownership are supported by consumers. Track/effect IDs have mechanical names
  rather than guessed sound names. `MeterValue` ($04c5) controls eleven UI drawing
  slots and is capped at nine; its precise gameplay meaning remains tentative.

ROM read aliases explicitly identify physical PRG bank pairs and CPU addresses.
Aliases inside an unknown gap describe how known code reads it, not a new map region.
Mapper register values remain separate from effective 8 KiB bank indices. CHR,
OAM, palette, CPU RAM and immutable ROM identities remain separate. No toolkit
implementation changes or compatibility claim follows from these explanations.
