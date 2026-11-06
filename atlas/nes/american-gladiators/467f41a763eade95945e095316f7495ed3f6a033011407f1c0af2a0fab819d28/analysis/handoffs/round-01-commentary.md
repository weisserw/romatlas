# Round 01 commentary handoff

**Current-source readability complete; ROM classification incomplete.**
Classification remains **54.94%: 144,024 / 262,160**, with **118,136 unknown**.
Code 10,632; data 2,320; CHR image 131,072. Execution coverage remains the disassembly
handoff's 4,739 physical opcode starts / 6,148 effective contexts; commentary added none.

Read the maintained [source](../../commentary/source/game.asm),
[RAM/constants](../../commentary/source/symbols.inc) and refreshed
[reading copy](../../commentary/reading-after.asm). All 278 classified contiguous code
fragments and classified data/image regions were reviewed. There are 689 functional
routine/branch names with explanations, 268 RAM aliases, and honest physical boundary
notes for every unknown fallback. No classified readability range is deferred.
The full review scaffold is preserved; final editorial acceptance remains pending.

Examples: actor position/velocity arrays and candidate collision coordinates; twelve-slot
cooperative scheduler and continuation saving; completion-mask level transitions;
inline pointer/string/palette return-address contracts; bank-six audio command calls,
priority restoration, envelope interpolation and DMC address conversion. Names describe
established mechanics; event-specific flags, scoring units, some audio commands,
CHR artwork identities and unclassified continuations remain explicitly unresolved.

**Redirect input:** [consumer contracts](../round-01-commentary/redirect-contracts.md)
contains exact CPU/physical entries, formats and bounded checks. Prioritize the connected
three-byte actor-spawn records, two-byte pointer consumers, animation/metasprite extents,
inline terminated palette/text records and their callbacks. Sound indirect entries and
DMC descriptors need effective physical bank context. Investigate completion at its
supported actor path before retrying final/ending shortcuts. Prior direct-PC freezes
reject only that incomplete state recipe. [Uncertainties](../../commentary/uncertainties.md)
localize the remaining meanings and dependencies; structural map was not changed.

Fresh [byte-identical verification](../round-01-commentary-verification.json) passed
and reading copy was refreshed. [Checkpoint record](../round-01-commentary/checkpoint.json)
records source/reading/map/store hashes and scope. Original maps/current.json,
analysis/state.json and frozen commentary copies match
4c6a44876fb0b171e856cb54a96e453676de8b6ec8feb5da8b25957ddbc42cdf;
input SHA-256 remains the workspace identity. Existing round-01-disassembly checkpoint
and evidence are preserved. Readability completion does not imply whole-ROM coverage
or full hardware/game compatibility.

Toolkit: **no toolkit issues found**; [report](../toolkit-issues.md), no blockers.
No archive, publishing, cleanup, toolkit changes or further agents were used.
