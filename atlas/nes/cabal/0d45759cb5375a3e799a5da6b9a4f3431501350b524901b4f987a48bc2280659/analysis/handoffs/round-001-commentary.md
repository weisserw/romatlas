# Round001 commentary handoff

Current-source readability completed across all currently classified source:
12980 code bytes / 6404 instruction records, 104424 data bytes, and explicit notes
for all 162 unknown intervals (13684 bytes). Classification remains 117404/131088,
89.56%; commentary does not establish whole-ROM completion or natural reachability.

Local deliverables: commentary/source/game.asm, source/symbols.inc,
commentary/review.json (6575 preserved scaffold entries, 366 semantic units),
commentary/uncertainties.md, commentary/progress.md and
analysis/round001-commentary-assessment.json. Review involved reading the entire
compact instruction inventory in all four physical 32 KiB AxROM images and authoring
mechanical unit explanations; automated counts alone were not the review method.

Examples: Bank0_StartMusicSequence/UpdateAudioChannels explain four interleaved
channels and bytecode/modulation; NmiServiceBody/WaitForNmi explain semaphore and
controller edge behavior; SortActorParallelArrays/RenderMetasprite explain field
identity, sprite ordering and clipping; Bank1_DecodePpuRleCounted distinguishes
compressed input and decoded output counts; Bank2_MergePackedAttributeRun describes
two-bit quadrant preservation; Bank3_ExpandCollisionRectangle names packed bounds.
Descriptive entries/branches replace anonymous labels. Context-specific ending text
aliases avoid confusing reused sprite scratch. Consumer table symbols do not claim
recovered extents. Fragment notes do not bridge unknown control-flow boundaries.

Fresh final commentary verify: byte_identical=true, original SHA256
0d45759cb5375a3e799a5da6b9a4f3431501350b524901b4f987a48bc2280659.
commentary/verification.json records source hashes; reading-after.asm refreshed.
The pass-001 maps/current.json and analysis/state.json exactly match frozen commentary
baseline.json and phase1-state.json. generated/pass-001/maps/pass-001.json and its
verification remain untouched. No pending classified-source readability range.
Final editorial audit remains pending; no final audit was run in this phase.

Unresolved meanings and next questions (details in uncertainties.md):
1. Bank0 $A48A/$A493 versus NMI acknowledgment $A626: compare saved snapshot
   PPUCTRL/mapper shadow, actual interrupt entry and semaphore writes in a bounded
   trace. The wait itself does not prove an input problem or toolkit bug.
2. Bank0 $B165 and common $FFA2/$FFB2 tails: establish exact BRK stack/inline return
   convention and RAM trampoline setup; use independently supported static contracts
   to investigate mixed prefixes/tails without requiring every selector to run.
3. Bank0 $8E04 and Bank2 $8AEC: resolve missing boss setup dependencies and transfer/
   part state before repeating artificial visits. Actor/audio game identities and
   some state nibbles remain unresolved but their classified mechanics are explained.
4. Ending $8ECD is artificial-display-confirmed only; highscore terminal and natural
   selection unresolved. Generic data remains supported by prior full-byte inspection,
   not guessed asset names. RLE copies retain distinct exhaustion semantics.

No toolkit issues found; analysis/toolkit-issues.md. No toolkit changes, publication,
archive, cleanup/deletion or redirect work performed. Input ROM unchanged. Disk about
3.2 GiB free; scratch 632 KiB. Coordinator can now inspect samples and begin redirect.
