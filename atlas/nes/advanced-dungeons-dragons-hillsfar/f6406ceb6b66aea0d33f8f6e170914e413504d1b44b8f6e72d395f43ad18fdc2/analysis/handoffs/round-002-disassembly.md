# Round 002 disassembly handoff

ROM classification: 68.73% (180185 / 262160 bytes classified; 81975 bytes unknown).

**Incomplete; ready for full current-source commentary.** Exactly one substantive disassembly pass. **Coordinator: print the classification line above to the user before starting commentary.**

Final authoritative checkpoint **pass-002-verified**: [verification](../pass-002-verified-verification.json), [frozen map](../../maps/pass-002-verified.json), [generated source](../../generated/pass-002-verified/game.asm). Internal state and maps/current.json are byte-identical to its frozen map. Rebuild is byte-identical to original ROM. [Integrity](../round-002/integrity.json) confirms unchanged input/game.nes, configured release binary and preserved commentary/source/game.asm and symbols.inc. Earlier pass-002 and pass-002-final are valid intermediate duplicate maps at179,718 classified, before final467-byte continuation; retain them, but use pass-002-verified/current state.

Accepted **90,603 new bytes**: redirect proposal78,675 Data+10,570 Code, then connected tavern1,307 Data+51 Static Code. Final kinds: Code22,103; Text72,754; Data85,328; Unknown81,975. Header16 bytes is Data;32 PRG banks total262,144. Full exact per-bank code/text/data/classified/unknown totals and exact unknown complement: [classification-counts.json](../round-002/classification-counts.json); [progress](../progress.md) and [investigation](../investigation.md) include the per-bank table. Structural classification is separate from execution coverage and commentary readability.

## Independent review and accepted boundaries

[Independent review](../round-002/independent-review.json), [review script](../round-002/review.py) and [code contracts](../round-002/code-contract-review.md) supplement primary redirect evidence, rather than blindly importing proposal claims.

- Reparsed42 graphics streams from84-byte directory and27 screens from54-byte directory, independently checking bank selection, headers, packed repeat/literal grammar, exact terminators, next-pointer endpoints and16-byte/960/144/96 geometry. Resource data labels now identify streams. No all-ID natural-use claim.
- Checked12 script stage graphs, exact pointer directories, parsed bytecode/operand bytes and localized FB00/FB01,FD,FE gaps. F8 failed stage advance continues normally at third stage; advance succeeds only when new selector<13 decimal and stack-aborts.188 text records and45 fixed gossip strings retain accepted Text; only unknown controls/terminators/table bytes filled as Data.34 handler/split arrays and fixed connected action helpers accepted with reviewed stack/control contracts.
- Freshly decoded9,044 unique proposed physical instruction starts, exact effective bank/CPU views, official instructions only, no competing instruction-boundary or Data/Text collisions. Reviewed all37 tavern dispatch targets, local continuation and fixed callee bank preservation. Actual saved contextual roots support Observed only when matching exactly; Static continuations remain Static. Regions may carry additional static alias views without asserting that alias was observed. Roots inherit saved artificial-state provenance, never natural reachability.
- Generic Data applies to short text/control framing, bounded zero/descending records and copied graphics resource suffixes; separate sparse146373..147125 suffix is bounded after coherentRTS and before existingFF. Mixed137821..146373 whole-bank Data inference stays rejected locally; no overly broad rejection of later suffix.

Exact annotations: [data](../round-002/data-applied.json), [code](../round-002/code-applied.json), [fresh code decode](../round-002/fresh-code-decode.json). Evidence strings and confidence1 reflect independently supported role/bounds, not a claim of complete game semantics.

## Connected work beyond original proposal ranges

[Connected tavern review](../round-002/connected-tavern-review.json) and [last complement review](../round-002/tavern-final-review.json) follow actual static consumers under effective[24,25,30,31]:

- Four fighter-tip pointer wordsAB75..AB7D bounded by AND03 and first target; four records close at next directoryB17A.
- Ten split gossip pointersB17A..B18E bounded by random1..10 plusFF =>0..9. Last two entries duplicate targetB889; nine unique records terminate exactly atB923 code.
- Ordinal zero-record walkerB923 anchors160 consecutive prose/control records8A2C..AB75.22 menu labels8847..8978 plus further contiguous menu prose8978..8A2C were independently parsed; whole printable text retained, missing controls/short runs classified Data. No original generator or every selector use claimed.
- Local numeric/pointer/count blocks8204..821B,83EC..840B,869B..86A1,BAD7..BB57,BC0C..BC32,BE89..BE91 have consumer references and coherent code boundaries. Generic Data deliberately avoids overclaiming exact semantic schemas.
- Secondary entry8003 JMP8761 and coherent48-byte callback8761..8791 accepted Static with exact context/precondition and service/stack/control consistency; no execution claim. Known local8836 text service is part of the reviewed contract.
- Seven repeated descending4-byte recordsBF57..BF73 classified generic Data independently of **12 ambiguous bytesBF4B..BF57**, which remain unknown. **40-byte BFD8..C000 reset/trailer also remains unknown**. Those are the only52 unknown bytes in bank24/25. New persistent lead R002-TAVERN-LOCAL-RESIDUAL suggests independently reviewing cross-bank reset/trailer layout; no automatic code claim from plausible pointers, legal decoding or a loneRTS.

A conservative local review assertion initially omitted already accepted local8836 service, so its first attempt terminated without durable annotation. Successful retry explicitly includes that existing service; no evidence or annotation gate weakened. All checkpoints/evidence retained. No toolkit bug.

## Gameplay reuse, remaining contracts and stopping scope

No new runtime experiment or startup replay. Saved snapshots, actual roots, routes and captures already exercise representative horseback completion, town, archery shot/miss, arena entry/controls, spring-lock menu and artificial retirement dialogue. [Scenarios](../scenarios.json), [gameplay](../gameplay.md) and prior handoff preserve exact recipes. Broad progression status was reviewed before stopping: successful lock/tool/maze needs actual object acceptance contract after corrected pick failure; full guild reward needs state2/X0 entry, not A-search(X64). Arena interface does not establish victory; retirement dialogue does not establish natural quest completion/full credits. Repeating unchanged shots/UI/handler patches would not resolve those missing contracts.

All ready leads were consumed after independent review and useful connected continuations; [leads](../leads.json) records outcomes and the new residual lead. This is an ordinary substantive pass handoff, **not terminal exhaustion** of81,975 unknown bytes. Later redirect can investigate remaining gameplay/data consumers, guild/object contracts and residual mixed/code tables with new concrete reasoning, not repeat unchanged failed routes.

## Required next commentary correction and carry-forward

**Existing commentary source was preserved unchanged. Carry its meaningful labels and comments forward into full current-source commentary for pass-002-verified.** New structural annotations do not imply whole current-source readability completion.

Correct stable **R001-COMMENTARY-CONTROLS** during commentary: actual **UP10/DOWN20/LEFT40/RIGHT80**, while existing BUTTON_RIGHT10/LEFT20/UP40/DOWN80 names are wrong. MenuUp/Down prefix labels/comments are correspondingly misleading:10 decrements selection,20 increments. Lock menuX8 selection is$064E, not$0646; verified oneDown moved1->2 (pick). Primary [menu decode](../redirect/r001-a01/menu-control-decode.json), actual town routes and [uncertainties](../../commentary/uncertainties.md) support this localized correction. Disassembly did not edit commentary. Freshly verify edited source byte-identical after correction and full current-source carry-forward/review.

**No toolkit issues found** ([persistent report](../toolkit-issues.md)). All RPC/server/rebuild processes exited. Scratch4KiB; approximately9.8GiB free. Input/state/traces/restored source/assets remain local. No toolkit or ROM changes, deletion, archive or publication. Resume by map.load analysis/state.json; final authoritative checkpoint is pass-002-verified.
