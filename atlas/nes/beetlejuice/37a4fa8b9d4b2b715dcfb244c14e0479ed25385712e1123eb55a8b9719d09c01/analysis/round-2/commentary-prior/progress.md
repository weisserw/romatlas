# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round 1 batch A — banks 0–3
Input map pass-001-final remains frozen. Reviewed all classified instruction/data fragments and explicit unknown gaps at physical $000000–$00800F. Added 55 mechanism groups, bank-local entry/control symbols, hardware register names, camera/player coordinate names and scoped audio stream/channel names. Audio engine, object contact/culling, vertical camera, PPU bytecode/run output, random-state mixing and BRK/NMI mechanics are explained; uncertain game identities are stated locally. Groups may span opaque gaps; they do not classify those bytes.
Fresh source rebuild verified byte-identical (source SHA256 d7db841efec9a5e932376210c5fa28d82160e110db24067bce9a86f85476ee75). Final reading refresh remains due after all source reviewed.
Pending same-phase scope: every inventory entry at physical $008010–$02000F (banks 4–15); no review credit asserted for these. Continue commentary, never redirect from this partial batch. Draft procedure and group list are in analysis/commentary-round1/. Final editorial audit remains pending.

## Round 1 batch B — banks 4–11
All classified fragments and explicit opaque gaps through physical $01800F now reviewed. Added graphics/table decoders (bit-tree, two-bit runs, nested PPU streams), map row/subtile/property lookup, dynamic collision boxes, PPU upload coroutine mechanics, room object motion and bank-local interrupt variants. Kept uncertainty on scene identities, payload bounds and coroutine continuation contracts. Source rebuilt byte-identically: SHA256 e6930cd36d2ba7efe292a7953a834a7912af2930e3f74fd78be02c4778cb2f8c.
Pending same-phase scope is now exactly all inventory entries at physical $018010–$02000F, banks 12–15. Continue commentary; final reading refresh and verification remain due.

## Round 1 batch C — banks 12–13 and crossing setup helper
Reviewed through physical $01C0A1 inclusive. Explained room initialization and palette/record loading, frame dispatch, alternate-stack PPU drains, player map probes and inventory interaction, room-specific contacts/events, sprite dispatch and PPU/APU reset helpers. Transition-cell scanner reduces player coordinates to 16-pixel cells, but the opaque matched-index continuation still lacks the safe task/stack contract needed for later-room experiments.
Fresh byte-identical rebuild source SHA256 1e013e0b5b4a0d1a001d8041f3128cea6a29d921d60f5ad99fce212fc5fd306e. Pending same-phase range: $01C0A2–$02000F only; final source reading refresh remains due.

## Round 1 batch D — bank 14
Reviewed physical $01C0A2–$01E00F, including controller polling/new-press computation, support-record scan, timed scene tasks, jump/horizontal/vertical player arithmetic, palette fade, task-memory initialization and sprite command renderers. Added consistent controller, player velocity/phase and PPU upload symbols across reviewed source; condensed repeated group prose to one explanation per mechanism.
Fresh byte-identical rebuild source SHA256 caf3011b68e4b30a271e8ddb3fb998d7339a42abd4f8844eb6a7199c8223f7f8. Pending current-source scope is exactly physical $01E010–$02000F (bank 15); final metadata locator cleanup and reading refresh remain due.

## Round 1 commentary complete — all current source readable
All 1,437 full-ROM scaffold entries are reviewed: 944 Code, 124 Data and 369 Unknown intervals. Code/Data coverage is all currently classified 25,107 bytes (22,647 code, 2,460 data); unknown 105,981 bytes remain structurally unclassified. There is no remaining current-source naming/comment backlog. All detached one-instruction fragments and copied bank-local handlers have descriptive mechanism labels and contextual explanations; opaque intervals retain explicit uncertainty notes. There are 232 mechanism groups and 950 physical code-fragment unit records with effective bank context.

Completed bank15 inventory overlay/mirroring transfers, counter rendering, scrolling marker scans, reset/title tasks and NMI trampoline. Names for controller state, player/camera motion, audio channels/streams and PPU state are consistent across callers. Control targets in classified source receive descriptive mechanism labels; unresolved external targets retain their exact numeric aliases. Source locators are corrected to game.asm:line relative to commentary/source; every Code entry has reviewed units. Frozen scaffold IDs/ranges/kinds and baseline/state hashes are preserved. Classification map/state were not edited.

Final fresh verification passed byte-identically: commentary/verification.json, source SHA256 82974701ab5a7bef74fffdc36b920ea20caaba5a41b9274d9802a53b7342ee12. commentary/reading-after.asm refreshed after the final edits. Scope assessment: analysis/commentary-round1/scope.json. Unresolved meanings and bounded redirect questions: uncertainties.md; compact handoff: analysis/round1-commentary-handoff.md. Remaining readability ranges: none. Final editorial/archive acceptance is pending and no final audit command was invoked; neither readability nor rebuild claims whole-ROM classification.

Toolkit findings unchanged: one existing confirmed documentation/API mismatch NES-001, no new suspected bugs or missing features and no blocker. See analysis/toolkit-issues.md. All artifacts remain local, ROM/toolkit unchanged. Disk at completion: 4.5 GiB free, commentary 7.3 MiB, commentary working records 172 KiB; no cleanup performed. No live server or other writer was started in commentary.
