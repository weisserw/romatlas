# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round1 disassembly

ROM classification: 73.43% (144389 / 196624 bytes classified; 52235 bytes unknown).

Code 12476 bytes (Observed8770; Static3706). Data/image 131913 bytes. Unknown 52235 bytes, all in PRG. There are505 classified regions with structural evidence. Annotation semantics/readability are pending commentary; structural evidence is not an editorial completion claim.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG0 | 1883 | 0 | 6309 |
| PRG1 | 1063 | 372 | 6757 |
| PRG2 | 2448 | 0 | 5744 |
| PRG3 | 826 | 0 | 7366 |
| PRG4 | 1696 | 0 | 6496 |
| PRG5 | 625 | 0 | 7567 |
| PRG6 | 2604 | 79 | 5509 |
| PRG7 | 1331 | 374 | 6487 |
| CHR0–CHR15 (each) | 0 | 8192 | 0 |


Verified baseline and checkpoints pass-001 and pass-001-final rebuild byte-identically. Latest exact map maps/pass-001-final.json, export maps/current.json, internal analysis/state.json, generated source generated/pass-001-final/game.asm; report analysis/pass-001-final-verification.json. Input SHA256 remains50ab734317d23b5f07f73578215571e6eb8a9b898781abd712181f23bf578f78. Configured release binary used throughout; no toolkit edits.

Evidence: contexts-r01.json records4065 distinct accepted ROM opcode offsets across natural startup and artificial scenarios. Observed byte count8770 includes complete instruction operands. Static branch discovery added1873 bytes; direct JSR entry-contract discovery added1833. Static code reachability is unresolved separately. Trace detail capacity0 intentionally discarded details; aggregate_dropped0 in all inspected summaries. No aggregate loss reported. Unt raced waiting frames do not contribute execution evidence. A bridge process ended; targeted replay recovered only affected evidence, preserving earlier saved roots.

Gameplay/scenarios: natural title and top-five display observed; all five galleries, bonus slots, mode menu and title-return initializer visited artificially. Actual R99 Balloon Saloon and R9 later galleries confirmed through06F2. Bills Thrills target throwing remains unobserved beyond its animated introduction. Zapper scoring cannot be exercised; manual describes a repeating score game with rounds capped99, no finite ending. Detailed recipes/statuses analysis/scenarios.json. Earlier0625 round hypothesis rejected and corrected, not promoted as successful progression.

Lead outcomes: R01-L002 and L003 verified; L001 dispatcher/table extents and L004 remaining PRG structures ready for redirect. Most remaining bytes need bounded consumer/format investigation rather than more blind play. No terminal exhaustion claim: this is an ordinary first-pass handoff with useful ready leads.

Toolkit: one documented missing feature NES-001 (Zapper), no confirmed/suspected bugs. See analysis/toolkit-issues.md. Does not block structural work. Disk checked at handoff: about6.6GiB free; scratch under0.3MiB; cited evidence/snapshots retained.

Static claim details: validated reset/NMI/IRQ vectors, accepted instruction boundaries, supported bank views, and uniform FF non-code ranges are preserved in evidence. Entire CHR section is image/pattern storage by validated header/hardware mapping. No pointer-only code promotion. Runtime RAM edits do not establish natural reachability. No ROM upload origin claimed.

## Round 1 redirect — completed research proposal, acceptance pending

Attempt analysis/redirect/r01-20261113-a/ investigated all concrete commentary dispatch/graphics/stream/audio questions, expanded the new scene7 and trigger handlers and associated records, audited accepted-source return continuations, and visited Bill's Thrills throwing via source-backed artificial gates. Findings/primary artifacts and methods are in the attempt's findings.md, handler-contracts.json, code-proposals.json, data-candidates.json and unknown-ledger.json. Proposal audit:12077code+40151data unknown bytes, zero instruction/data conflicts; actual52235unknown remain unchanged pending independent acceptance. Seven isolated bytes have localized primary evidence and no current discriminating method; not terminal exhaustion. Stable lead histories preserved in analysis/leads.json. Reuse successful snapshots, exact patch recipes and semantic/structural constraints rather than replaying unchanged experiments. Full handoff analysis/handoff-r01-redirect.md.

## Round 2 disassembly — completed independent review

Current phase: disassembly complete, round2; handoff to commentary pending. Final checkpoint pass-002-final. ROM classification: 100.00% (196617 / 196624 bytes classified; 7 bytes unknown).

**Incomplete:**100.00% is rounded from99.9964399056%; seven bytes remain materially ambiguous.24553code bytes (Observed9412, Static15141);172064data/image bytes;944classified regions. Gain52228bytes:12077code and40151data. New observed code642bytes/271instruction starts uses saved artificial opcode fetch contexts, not natural reachability. Remaining new code11435bytes is Static. No old classifications were removed.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG0 | 3200 | 4992 | 0 |
| PRG1 | 1551 | 6640 | 1 |
| PRG2 | 5225 | 2963 | 4 |
| PRG3 | 2083 | 6108 | 1 |
| PRG4 | 3534 | 4658 | 0 |
| PRG5 | 3329 | 4862 | 1 |
| PRG6 | 2898 | 5294 | 0 |
| PRG7 | 2733 | 5459 | 0 |
| CHR0–CHR15 (each) | 0 | 8192 | 0 |

Input pass-001-final map/state db252197fe7e5c3fb4e479eaee7888427f006a35a798a9899b20f09f1a665cda. Final map/state SHA256 0adf8a365f40d54161c7e483866e2aac98e110b59154e3e64da2ac334243d6e8. Baseline and all historical checkpoints preserved. Fresh pass-002-code, pass-002-data and pass-002-final checkpoints validate and rebuild byte-identically; final report analysis/pass-002-final-verification.json and source generated/pass-002-final/game.asm. ROM SHA256 unchanged50ab734317d23b5f07f73578215571e6eb8a9b898781abd712181f23bf578f78. Configured release binary only; no toolkit edits.

Independent review: final5202instructions redecoded in5337bank views, exact bytes and physical identities checked. Inline table barriers, per-scene effective bank pairs, C52C return-pop contract, returning-callee continuations and finite RAM/APU indirect-store contracts were inspected; unbarriered discovery is excluded. Count/stride/terminator checks cover471metasprite proposals,87palette records, VRAM records and paired animation streams. Review notes, primary decodes, exact acceptance membership, annotation results and counts: analysis/disassembly/r02-review/. These are local internal evidence.

Narrowed claims: scene5 always replaces default grid5 with11..18, invalidating consumed4x4-role proof for28exclusive blocks, including119bytes overlapping a valid metasprite pointer table. Retained generic signed movement/frame patterns and independent pointers; no bytes accepted solely on the false tile interpretation. Generic roles also retain uncertain parameter, pointer-suffix, phase-pattern and audio-container grouping. E816/E837 are triangle-channel paths (4008/400A/400B), correcting redirect's DPCM wording. Ten isolated coherent routines accepted using field/helper/control/boundary contracts, not legal decoding alone. Scene9 unusual FF execution is preserved separately. No ending claim from B147.

Lead outcomes: R01-L001 verified structural code/dispatch batch; R01-L004 verified data with narrowed specific roles; L002/L003 successful routes reused without replay; L005 retains seven unknown bytes physical2A21,403A,41AC,42E8,4999,60CE,A1B2. Primary address-word occurrences at2A21/4999/A1B2 do not establish a valid consumed object or caller; loneRTS/BRK does not discriminate code/data. The residual and competing-role reasoning is preserved in seven-byte-primary-review.json and review-notes.md. No new useful direct gameplay test identified: all galleries/high rounds and Bill throwing already visited, target hits unavailable under NES-001; finite ending unestablished. Saved traces/snapshots reused, no unchanged startup replay.

Current maintained commentary/source/game.asm is preserved unchanged (SHA2565f6f3b59d74116f019b9c3254548026087beaf07ba7458c591f868b39fbf5e7a). Prior readability was complete for pass001 only; full current-source commentary and final editorial audit remain pending after this gain. Next commentary must cover all resulting source, correct reel36versus24 and triangle-channel wording, and retain current bank/reachability distinctions and preservation credit.

Toolkit report analysis/toolkit-issues.md remains one missing feature NES-001 (Zapper), zero confirmed/suspected bugs; no structural blocker or new issues. Failed local request-schema attempts were corrected before persistence and are not toolkit defects. Disk6.6GiBfree; shared scratch208KiB; no cleanup, archive, deletion or publication. Compact handoff analysis/handoff-r02-disassembly.md.

## Round 3 disassembly complete

Current phase: disassembly complete, round3; input pass-002-final; authoritative final checkpoint pass-003-accepted. ROM classification: 100.00% (196618 / 196624 bytes classified; 6 bytes unknown).

Incomplete: exact percentage 99.99694849051998%; six unknown bytes remain despite rounded100.00%. Gain1generic Data/Static byte02A21, code24553/data-image172065,945regions. Independent combined pointer/consumer/storage-grammar review accepts only generic storage, not safe empty metasprite or natural selection. Exact checks/reasoning: analysis/disassembly/r03-review/. Six residual physical offsets403A,41AC,42E8,4999,60CE,A1B2 retain localized ambiguity; no unchanged runtime replay.

| Bank | Classified | Unknown |
|---|---:|---:|
| header | 16 | 0 |
| PRG0 | 8192 | 0 |
| PRG1 | 8192 | 0 |
| PRG2 | 8188 | 4 |
| PRG3 | 8191 | 1 |
| PRG4 | 8192 | 0 |
| PRG5 | 8191 | 1 |
| PRG6 | 8192 | 0 |
| PRG7 | 8192 | 0 |
| CHR0 | 8192 | 0 |
| CHR1 | 8192 | 0 |
| CHR2 | 8192 | 0 |
| CHR3 | 8192 | 0 |
| CHR4 | 8192 | 0 |
| CHR5 | 8192 | 0 |
| CHR6 | 8192 | 0 |
| CHR7 | 8192 | 0 |
| CHR8 | 8192 | 0 |
| CHR9 | 8192 | 0 |
| CHR10 | 8192 | 0 |
| CHR11 | 8192 | 0 |
| CHR12 | 8192 | 0 |
| CHR13 | 8192 | 0 |
| CHR14 | 8192 | 0 |
| CHR15 | 8192 | 0 |

Map/state SHA256f7c49fce4ad15bbe7ad6b7d51354e5cf3aeb46760784f9902c148b9f198eaab1. Validation and fresh byte-identical rebuild passed: analysis/pass-003-accepted-verification.json. Preacceptance pass-003-final checkpoint preserved separately; it is not authoritative final R03. ROM unchanged; current maintained source remains SHAa866725b5d57f4cdd4a0ea3d1397d8d1bd35e29ff604cfa8b67c578f41af6c48. Next commentary incorporates one datum, preserves full prior work, and corrects stale Bank4AuxiliaryMotionStep gaps wording at10463. Full current-source readability and final editorial acceptance remain separate gates. Lead histories updated; compact handoff analysis/handoff-r03-disassembly.md.

Representative progression/scenarios reused: galleries/high rounds/Bill throwing covered; optical hits unavailable NES-001; no supported finite ending. Toolkit report analysis/toolkit-issues.md unchanged:1missing feature,0confirmed/suspected bugs, no new issue or structural blocker. Configured release binary only, no toolkit change/archive/deletion/publication. Disk6.5GiBfree; scratch208KiB.
