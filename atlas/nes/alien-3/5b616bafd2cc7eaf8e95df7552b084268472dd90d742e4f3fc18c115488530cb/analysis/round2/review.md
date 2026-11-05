# Round 2 independent structural review

Input pass-001 (218781 classified/43379 unknown). Review uses exact proposal slices,
not wholesale graphs. Accept 43371 bytes:15227 Code,28144 Data, all Static. No new
execution evidence or natural reachability claim. Historical maps/source untouched.
Primary contracts: ../redirect/r1-20261105-a/findings.md; exact inputs proposals.json
and manifest.json there. Independent checks: decode-audit.json, data-audit.json,
decoded-proposals.json and reproducible review.py/data-audit.py in this directory.

All7227 instruction starts were redecoded through configured binary with their
explicit CPU addresses/effective banks: exact text/length agreement; every view
maps to its physical bytes; proposal union43371, no overlap with accepted bytes,
no duplicate bytes. Direct calls/jumps/branches resolve to Code rather than Data or
Unknown under those views. These consistency checks supplement structural evidence;
they are not the reason arbitrary legal bytes become code.

Screen: eleven adjacent dispatcher words end at title entry803D; fixed-mode call
sites supply finite selectors0..10. Resource consumer9682 independently uses the
four eleven-word address columns, tile-bias column, FF streams, FE text conversion,
64-byte attributes and OAM stream. Independently reparsed every resource stream,
checked PPU addresses and OAM multiples of4, palette values<64. Printable975-byte
ending payload and finalFF at9580 independently read. Generic resource suffixes
and animation records have inspected encoded patterns; do not assert original
schema/serializer for the whole99BB..B342 domain. Correct IRQ9873 installed by
98C4; select1/data54 changes CHR only. Register restoration and RTI balanced.
Only screen-irq-v3 used, invalid v1/v2 roots excluded.

Actors: independently read all15 type roots atA599/A5B7, all matched finite state
arrays, all74 sentinel sequences and replacement states. ConsumerC603/C634 supplies
the state/frame pointer traversal and replacement behavior. Independently parsed
ALL159 frame records to exact9B10, including four matching gap records;155 referenced
records agree. Renderer9DD1 starts components at11 and advances four bytes untilFF;
frame records are data, never pointer-as-code assumptions. Behavior/initializer code
operates coherently on0400/0420/0440/0460/0480/04C0/04E0 actor fields and connects to
inspected movement/contact/guardian handlers. C5C6 maps12 only for frame work and
restores8A before behavior dispatch; bank13/fixed behavior views use representative
[0,13,14,15]. Exploratory actor bank12 target/KIL excluded. Physical low-slot data
references cannot inherit those representative views without separate proof.

Renderer: independently compared every selected instruction across banks0/2/4/6.
9C58/9D16 use projected coordinates, clipping and component templates;9E84 emits
four-byte OAM entries using balanced temporary pushes. Tables are bracketed by
routines and template origins. A broader exploratory full1KiB identity assertion
failed because stage initialization tails differ; it is NOT used as evidence.
Instruction slices are identical, while each tail's repeated records/pointers/zero
runs independently support generic non-code without claiming identical tail contents.

Standalone occlusion9EB0(level) and9F7A(actor) preserveX, combine quantized screen
and scroll coordinates with row-remap data, read0600 grid, restoreX and return normally
forFF; otherwise discard one caller return address. Full control/data/stack contract
supports static parameterized code; no caller or natural execution is claimed.
DC26 sets0381 and discards a return address; E58B waits for29 then converges player
Y toward selected actor using signed direction and0711 frame waits/E457 updates.
These are coherent routine contracts stronger than isolated legal JMP/RTS bytes.
Authoritative source-prefix-closures-v2 only; predecessor mid-instruction roots excluded.

Mapper: C008 pair-map is an explicit propagation boundary. Fixed continuation code
uses physical banks13/14/15, with inspected reg6 changes affecting slot0 only and
reg7 writes restoring13. E08E/E16B..E17D explicitly establish slot13; F521 temporarily
maps12 then restores level. E000/C000 and CHR writes alone do not invalidate PRG
identity. Graph call fallthrough does not prove conventional return after PLA aborts;
no TSX/stack-page inline-argument reader supports hidden inline operands. Proposed
boundaries preserve all actual data barriers. No hypothesis that every path is reached.

Audio:8000/8003/8006 vectors enter channel setup/update; APU4000+4X, split track,
pattern and note-timer reads establish interpreters and non-code domains. Code ends
with841E RTS, followed by frequency/pointer/command patterns throughADCD. Exact
composer format/all guessed pointer lengths are not asserted. Five51-byte fixed
columns EE07..EF06 are anchored by EDB1 origins and command selector contracts;
local constants/text/coordinate tables were checked against physically resolved
references and consumer syntax. Alias-only residual reference arrays were not used.
Where precise meaning remains unresolved, generic Data classification is sufficient.

FE85..FFF4: independently read367 uniform zeros; FE84 belongs to accepted three-byte
vector instruction, andFFF4 is rooted JMPFDA1. Narrowed generic non-code claim accepts
no old invalid first byte or original linker-fill assertion.

R1-L03/04/05/06/07/08/09 verified as scoped above; earlier R1-L01/L02 historical
outcomes preserved. Connected code/data beyond original lead ranges accepted where
these contracts establish it. Prior representative early/later/guardian/ending/score
experiments reused; no changed runtime question warrants repetition this pass.

Residual8 bytes remain Unknown in bank14:1D0FD..1D100,1D4F5..1D4F6,
1D59B..1D59E,1DACE..1DACF (half-open physical bounds). Isolated JMP CF65/RTS
encodings have no independent entry/control contract; adjacency alone insufficient.
Fresh redirect should inspect actual accepted neighbors and residual references,
localizing the question to these fragments. This is an easy-gain handoff, not a
terminal exhaustion/rejection review. No accepted proposal rejected. Rebuilding and
whole-ROM readability remain separate gates. Toolkit unchanged; existing two
confirmed documentation/API issues, no new findings or blockers.
