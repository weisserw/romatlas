# Commentary uncertainties — round 01

Current-source readability is complete; semantic and structural investigation is not.
All numerical work operands not named in symbols.inc retain either scratch/stream/
actor-field mechanics in their routine description or an explicitly unknown game
meaning. In particular audio envelope repeat counters, shooting scene/target flags,
ending helper arguments and driving item flag values are not given invented names.
RAM $40..$7F and object pages are mode overlays, not a single persistent structure.
Address aliases and descriptive names are analysis names, not recovered original symbols.

## C01 — driving bank prefix may be a spurious static path

Physical $14010..$1401D (CPU $8000..$800D, effective PRG slots [10,11,14,15])
is currently Static Code. Its NOP/ORA/SLO sequence is anomalous beside the real
DrivingDispatch entry at physical $1401F / CPU $800F. DrivingSelectPresentation
at physical $140E7 / CPU $80D7 reads indexed bytes at CPU $8009 and $800C;
those references overlap the suspicious prefix and adjacent unknown bytes.
This is a concrete competing data interpretation, not proof that unfamiliar opcodes
are invalid. Source flags it as DrivingEntryUncertain and retains the current map.

Primary evidence: maps/pass-001-final.json records at decimal offsets 81936,
81939, 81942, 81944 and 81946; commentary/source/game.asm labels
DrivingEntryUncertain, DrivingSelectPresentation and CallGameplayMode.
The latter selects CPU $800F for driving via the fixed-bank mode route.
For redirect, inspect the saved accepted roots and the static candidate predecessor
that reaches bank10 $8000; reconstruct its effective slots across mapper writes and
banked wrappers. Compare that support against the bounded $8009/$800C indexed
consumer, and isolate the prefix from valid $800F code before any reclassification.
Do not require natural execution to establish a narrower static/data claim, but do
not infer a full table extent just from a few indexed reads.

## Remaining semantic questions

- Text/ending: TextPresentationDispatch and EndingDispatch expose state/command
  mechanics, but not all script/glyph/presentation table extents or the final credits
  transition. Reuse the saved ending/credits scenarios; avoid unchanged long waits.
- Audio: six logical voices, stream forms, pitch/envelope loops, pause ownership,
  DMC descriptors and priority are explained. Instrument/song names, command tables,
  sample bank identities and complete stream boundaries remain open.
- Sprites/terrain: component formats, depth sorting, mirroring, clipping and terrain
  nibble generation/readers are explained. Complete pointer-table extents, sprite
  identities and individual collision-bit meanings remain open.
- Driving: resource, speed, projection, steering, projectiles, road profile and raster
  mechanics are explained. Exact item meanings and the ROM origin/extent of RAM
  raster code remain unresolved. RAM execution alone does not establish upload origin.
- All 273 unknown intervals retain original bytes and explicit local boundary notes.
  Source unit envelopes can span such gaps; unit length never asserts classification
  of the gap or a valid fallthrough across it. Uniform data remains generic after the
  prior boundary inspection; CHR tile identities and arrangements remain unresolved.

Natural versus artificial execution remains as recorded in analysis/scenarios.json.
Static explanations do not assert natural reachability, full compatibility or full
ROM classification. Final editorial/archive acceptance remains pending.
