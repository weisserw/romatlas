# Round 001 commentary handoff

**Current-source readability complete.** No remaining source ranges; final editorial
acceptance remains pending. No final audit, redirect research, publishing or archive
was performed by this phase.

Input checkpoint: **disassembly-001-final**, frozen structural map SHA256
`71f91f57da5affd908db67474f695f700a6275f1130d1e261c14d63f50b9f736`.
Supported `commentary.py prepare` established frozen before/source copies; source is
now independently maintained. Classification remains **65552/65552**, zero unknown:
12,664 Code; 20,054 Data; 66 Text; 32,768 Image. No structural map/state edits.

Output commentary checkpoint: **commentary-001-final** (maintained source plus
[verification](commentary-001-final-verification.json), no regenerated map checkpoint).

- [Readable current source](../commentary/source/game.asm)
- [Refreshed reading copy](../commentary/reading-after.asm)
- [Frozen reading baseline](../commentary/reading-before.asm)
- [Review inventory](../commentary/review.json)
- [Readability scope check](commentary-001/readability-check.json)
- [Progress](../commentary/progress.md), [uncertainties](../commentary/uncertainties.md)

Covered all 135 full-ROM scaffold entries and 335 meaningful routine/structure units.
The complete 7,056-line baseline was reviewed in address order. Current 9,573-line
source explains 240 routine/handler contracts and names 437 internal control targets,
385 RAM/object-slot aliases, hardware registers, common object fields and applicable
button/stride constants. Source comments cover reset/NMI and synthetic RTS dispatch;
input/attract/continue; terrain/CHR/palette; actor motion, steering, drawing and damage;
player weapons/armor UI and deployment; bosses/base parts; items, score/HUD and audio.
No address-only Code labels or numeric memory operands remain. Numeric immediate
values with unresolved or local mechanical meaning are retained honestly.

Examples: `DispatchActor` explains the pushed return into common motion/rendering;
`GateTerrainHalfRate` documents its two-return unwind; `ClassifyDirectionToPoint`
and collision helpers preserve carry contracts. RTS tables now visibly contain
symbolic high/low target-minus-one expressions, distinct from little-endian hardware
vectors and pointer directories. Small data arrays have visible bytes; broad encoded
blocks retain prior bounded inspection evidence, format notes and uncertainty rather
than invented schemas. Interior consumer labels/include splits do not claim original
source layout or independently proven full table extents.

Unresolved meaning is local, not unread source. Actor names describe established
mechanics while exact artwork/natural selection remains open. Shared scratch and
auxiliary fields retain generic mechanical names where meanings vary by handler.
Standalone $80D2 has no established caller. Recorder helpers $82B9/$924D/$9265 write
CPU $CBxx, ROM/mapper space in CNROM; comments do not claim a working RAM recorder.
The shipped recorder hooks return immediately. $9578 is explained as the armor
RTS dispatcher's synthetic expiry continuation; no separate natural caller is claimed.

**Preserved redirect question D001-L05:** `BeginFinalBaseEncounter` ($8893) sets six
parts and BaseActive without allocating those parts. `SpawnTerrainRecords` ($8976)
reads six three-byte spawn records through progress-derived $FCAA pointers;
`HandleBasePartDamage` ($B00E), `SpawnBaseAuxiliary` ($B0DD) and
`HandleBaseCoreDamage` ($B113) require existing records and source/damage fields.
Inspect the specific spawn records, progress match and free slots before another
bounded replay. The prior artificial final-base scenario remains unsuccessful and
ending/loop completion unobserved. Reuse [disassembly handoff](handoff-disassembly-001.md)
and its before/after RAM/recipe rather than retrying unchanged patches. Generic data
subformats may be refined later, but no new structural ambiguity was found here.

Fresh final `commentary.py verify` passes **byte_identical=true**; reading copy
refreshed afterward. Maintained game.asm SHA256:
`62718df9373e8ec7e1f7d06ad1f2c4d9487fafe6656ad042ff86a6a5db3d86b3`.
Input SHA256 unchanged:
`9f915b15984b93637f3155a6830db3430b0aae4eed3d09852930bb30f1786981`.
Current map and frozen commentary baseline SHA match the input checkpoint exactly.
The readability check accounts for all 12,664 code bytes and zero pending scaffold
entries; it is explicitly not the final editorial audit. `editorial_audit` stays pending.

Toolkit findings: existing one confirmed documentation/API-validation issue NES-001;
no new bugs, suspected bugs, requests or blockers. [Report](toolkit-issues.md).
No toolkit modifications or live servers. Input, source, assets and all evidence
remain local; only the supported ROM-free portable map may be shared.
