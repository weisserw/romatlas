# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 — disassembly

ROM classification: 63.91% (83773 / 131088 bytes classified; 47315 bytes unknown).

- R01-STARTUP verified: validated bank0 reset $ffdc -> $8001, observed startup and ROM opcode contexts; RAM banking trampolines retained separately without source-origin claims. Static graph code comes from executed roots and supported direct JSR entries, not arbitrary legal decode. No candidate boundary conflicts.
- R01-GRAPHICS verified: bank1 $8001 upload loop, $8075 pointers and recognizable planar graphics support physical $80cb-$fb13; bank3 $8001 loader and $8067 pointers support $18122-$1ad02. Additional planar slices $1ae65-$1bac5 and $1bbca-$1ffec stop at code/reset boundaries. Primary Code role wins wherever uploaded bytes also execute. Local sheets and loader decodes retained.
- R01-DATA verified: maximal uniform intervals >=32bytes; code conflicts checked. Bank1 $fb47-$ffec repeated records/planar runs inspected with neighboring $fb03 OAM-copy consumer, generic Data because complete record semantics are unresolved. Header/vectors and bounded selector ramps classified separately. See uniform-data.json and state evidence.
- R01-PROGRESSION verified: clock digits $059e-$05a1, quarter $05e6, divider $05d4 consumers in clock-decode.json. Reset clean snapshot before each patch. Quarter-end0/1/2/3 reached states7/11/9/16 respectively. Halftime continuation initialized quarter2 court with reversed sides; movement/A/B actual execution sampled. Artificial reachability only.
- R01-INPUT superseded: initial button/pressed requests silently cleared NES input. Their evidence is no-input attract execution; no requested-action or court claims survive. Corrected buttons-array route verified. NES-001 retained.
- R01-ENDSTATE attempted/deferred: final score verified; state17 blank after bounded Start/A/B and1200+frames wait. No proven cause; redirect should inspect state17 initialization and dependency on score/high-score fields rather than repeat unchanged waits.
- R01-CALLS partial verified: direct JSR targets rooted with caller's effective banks (entry preserves mapping); no assumption of bank-preserving return. Final repeated callee-target expansion yielded0 bytes. Missing return summaries/indirect tables need source/dataflow inspection, not more long attract play.

Unknowns and next questions: 47,315 physical bytes; largest PRG-bank gaps in banks0-3 and8-11, likely mixed code/tables. Follow supported consumers and repeated record/image formats before generic fallback; non-code meaning is not required. Concrete redirect frontiers: two-entry NMI dispatch table near physical$00d1 and indirect dispatch paths; main state$fc consumers; bank0/bank2 table-reading loops and indexed targets; post-final state17 blank-screen contract. Read exact unknown complement and latest current source. No terminal rejection review claimed or required for this ordinary productive pass.

Checkpoint and map/source fidelity verified in pass-001-verification.json. Full readability/editorial review remains commentary work. Toolkit findings: one confirmed doc/API-validation bug NES-001; no blocker, no toolkit edits.

## Round 2 disassembly complete — incomplete ROM

ROM classification: 99.85% (130891 / 131088 bytes classified; 197 bytes unknown).

Structural totals: {'Data': 37807, 'Code': 30058, 'Image': 63026}; header16 counted separately from PRG. 1101 evidence-backed regions. Added47118 unique bytes (11258 Code+35860 Data); mandatory120 Image->Code correction is zero coverage gain. Original combined45599 accepted after independent review;1519 connected/narrow-contract continuation bytes also classified. No proposal rejected; all five ready lead IDs verified.

| PRG8 bank | Code | Data | Image | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0 | 7996 | 114 | 0 | 8110 | 82 |
| 1 | 4089 | 4096 | 0 | 8185 | 7 |
| 2 | 160 | 8032 | 0 | 8192 | 0 |
| 3 | 3301 | 4891 | 0 | 8192 | 0 |
| 4 | 116 | 71 | 8005 | 8192 | 0 |
| 5 | 0 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 0 | 8192 | 8192 | 0 |
| 7 | 181 | 1216 | 6795 | 8192 | 0 |
| 8 | 1936 | 6256 | 0 | 8192 | 0 |
| 9 | 4393 | 3799 | 0 | 8192 | 0 |
| 10 | 2710 | 5478 | 0 | 8188 | 4 |
| 11 | 4644 | 3444 | 0 | 8088 | 104 |
| 12 | 205 | 69 | 7918 | 8192 | 0 |
| 13 | 318 | 298 | 7576 | 8192 | 0 |
| 14 | 0 | 0 | 8192 | 8192 | 0 |
| 15 | 9 | 27 | 8156 | 8192 | 0 |

Fresh configured-toolkit exact decodes and independently rooted control graph checked5119 original proposed starts and197 conditional normal-return summaries. Explicit FD/row-count/character-index/EntryPlayerIndex preconditions retained, with inline barriers and bank force/return context. Original actual-fetch artifacts independently matched before Observed provenance; static descendants remainStatic. Details: [review](round-02/review.md), [summary](round-02/summary.json), [unknown ledger](round-02/unknown-ledger.json).

Current state saved and portable map exported/validated. Fresh [pass-002](../maps/pass-002.json), source generated/pass-002, [verification](pass-002-verification.json): byte_identical=true, input SHA unchanged. All prior commentary source hashes unchanged; commentary phase remains separate and must catch up new classifications plus decimal17=$11 historical prose correction. Prior completed redirect records unchanged.

Representative progression reused rather than replayed: coherent artificial statistics, record/initials/AAA50 leaderboard/title and initialized overtime court, plus original natural court route remain in scenarios.json. No new runtime/artificial route claimed. Credits stream17 remains statically classified, naturally unverified. Residual197 bytes in15 localized intervals have code/entry or small mixed-boundary ambiguity, not merely unknown data meaning. R02-RESIDUAL recipe directs actor-update and player-control contracts, isolated JMP triples and tiny data ambiguities to redirect. Easy-gain handoff, not terminal exhaustion.

Toolkit: no new issues; existing NES-001 documentation/input-schema issue remains with workaround in [toolkit report](toolkit-issues.md). No toolkit edits, publishing, archive, cleanup or input changes. Disk about8.3GiB free; scratch/review records retained.

## Round 3 disassembly complete — incomplete ROM

ROM classification: 99.98% (131063 / 131088 bytes classified; 25 bytes unknown).

Accepted R02-STATIC-CONTRACTS172 Static Code bytes/9 intervals after independent role/bank/boundary review. Totals Code30230, Data37807, Image63026; header16 included in Data, separate from PRG. 1110 internal regions.

| PRG8 bank | Code | Data | Image | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0 | 8060 | 114 | 0 | 8174 | 18 |
| 1 | 4089 | 4096 | 0 | 8185 | 7 |
| 2 | 160 | 8032 | 0 | 8192 | 0 |
| 3 | 3301 | 4891 | 0 | 8192 | 0 |
| 4 | 116 | 71 | 8005 | 8192 | 0 |
| 5 | 0 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 0 | 8192 | 8192 | 0 |
| 7 | 181 | 1216 | 6795 | 8192 | 0 |
| 8 | 1936 | 6256 | 0 | 8192 | 0 |
| 9 | 4393 | 3799 | 0 | 8192 | 0 |
| 10 | 2714 | 5478 | 0 | 8192 | 0 |
| 11 | 4748 | 3444 | 0 | 8192 | 0 |
| 12 | 205 | 69 | 7918 | 8192 | 0 |
| 13 | 318 | 298 | 7576 | 8192 | 0 |
| 14 | 0 | 0 | 8192 | 8192 | 0 |
| 15 | 9 | 27 | 8156 | 8192 | 0 |

Fresh configured-toolkit decode, structural review and exact residual ledger: round-03/review.md, fresh-decodes.json, neighbor-contracts.json, unknown-ledger.json, summary.json. Five isolated JMPs15bytes and two short code/data ambiguities10bytes remain unknown; no unsupported residual inference. Connected accepted continuations checked; representative gameplay evidence reused without repetition. R02-STATIC-CONTRACTS verified; no proposal rejected.

Saved state/export validated. Fresh maps/pass-003.json and generated/pass-003 rebuild byte_identical=true (pass-003-verification.json). Input SHA unchanged. Full commentary file-tree hashes unchanged; separate commentary must cover172 newly classified bytes and update dependency/unknown notes. Final editorial audit remains pending. No new toolkit issue; existing NES-001 workaround remains in [toolkit report](toolkit-issues.md). No toolkit edits/archive/publish/cleanup/input changes. Compact handoff: handoffs/round-03-disassembly.md.
