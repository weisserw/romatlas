# Round 4 commentary handoff

Current-source readability complete. Classification remains **99.96% (65525/65552)**, 27 unknown bytes; no structural changes made during commentary.

Fresh configured-release [verification](r4-commentary-final-verification.json): byte_identical=true. Source SHA256 **dfdf77b5953921c6d64a6330e7db53e40639388df0e8ac8f8ba75ce2ed5bef1a**. [Reading copy](../commentary/reading-after.asm) refreshed. [Consistency](round-4/final-consistency.json): input unchanged, current map/frozen baseline and state/frozen state byte-equal; source references and full partition valid. Entire previous commentary preserved under round-4/commentary-r3-preserved with b8380ee7… source hash confirmed before carry.

[Scope](round-4/scope.json): all 5613 scaffold entries and 518 meaningful units reviewed; 14538 code bytes, 18219 data/header bytes and 32768 CHR bytes readable. Full current inventory checked; 5612 unchanged physical records retain valid Round 3 review/prose. No broad reread or cosmetic rewrite; only changed entry and affected dependencies revisited. No current-source readability gaps.

WriteSetupRectangleTwice (physical 0x1A4E..0x1A51, CPU $9A3E, slots [0,1,2,3]) now explains two finite literal 131-by-132 writes, each source $A2FE..$E68B (end exclusive $E68C). First return resumes setup, with X=0 and PPUDATA-derived A selecting second destination $00xx; second returns to original caller. Nearby setup/body explain actual literal bounds, read/write counts, source carry and stack/latch/interference requirements. ClearBothNametables comparison correctly calls $A3A7, returns $A3A2, then falls through. No same-coordinate, natural caller, attribute-only effect or original-purpose claim. Prior answer/feature, drawing, geometry, catalog and audio prose retained.

Residual half-open physical ranges **0x2707..0x2708**, **0x5E61..0x5E7A**, **0x5E83..0x5E84** retain explicit notes. [Uncertainties](../commentary/uncertainties.md) distinguish remaining structural ambiguity, natural caller/purpose and unresolved gameplay meanings. No terminal exhaustion or whole-ROM classification claim.

Final editorial audit remains pending for terminal acceptance and was not invoked. No toolkit edits/issues added, runtime replay, publication, archive, cleanup/deletion or commits. No live writer/RPC; worker did not launch next phase.
