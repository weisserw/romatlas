# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Pass 001 complete: incomplete ROM classification

ROM classification: 62.53% (163919 / 262160 bytes classified; 98241 bytes unknown).

Code 14429; Data 18418; CHR Image 131072. Header16 and all CHR131072 classified; remaining unknown complement is entirely PRG. Unique physical intervals, no raw fallback counted. Counts and exact unknown complement: pass-001/counts.json.

| Physical 8KiB PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 1805 | 35 | 1840 | 6352 |
| 1 | 7 | 2569 | 2576 | 5616 |
| 2 | 0 | 1573 | 1573 | 6619 |
| 3 | 7 | 1306 | 1313 | 6879 |
| 4 | 0 | 1218 | 1218 | 6974 |
| 5 | 7 | 1153 | 1160 | 7032 |
| 6 | 0 | 854 | 854 | 7338 |
| 7 | 7 | 2100 | 2107 | 6085 |
| 8 | 0 | 1233 | 1233 | 6959 |
| 9 | 7 | 1867 | 1874 | 6318 |
| 10 | 302 | 0 | 302 | 7890 |
| 11 | 317 | 2983 | 3300 | 4892 |
| 12 | 1670 | 0 | 1670 | 6522 |
| 13 | 2691 | 224 | 2915 | 5277 |
| 14 | 3998 | 32 | 4030 | 4162 |
| 15 | 3611 | 1255 | 4866 | 3326 |

All annotations include structural evidence and provenance; code includes explicit CPU execution views. Observed and Static remain separate. Execution coverage is a subset of code, not whole-ROM classification; runtime roots and artificial flags retained in *contexts.json. Bounded detail buffers intentionally capacity0 (detail drops), all cited trace summaries aggregate_dropped=0; no dropped aggregate evidence treated as observed.

Broad progression: normal startup/name/castle/hub/SELECT; supported artificial entrances into all five attractions, representative mechanics; artificial six-key parade and final End screen. Full objective wins, Haunted boss and trivia chain remain deferred with missing RAM/script contracts in scenarios.json/L004. Repetitive hub movement stopped at geometry barrier; corrected dialogue A route succeeded. Initial traced180frame request timed out and was replaced with untraced broad progression and1-3frame captures.

Static gains include validated vectors, accepted fetches, connected control flow/direct-call entries, exact six16-byte entrance arrays, inline scene dispatch table, replicated reset wrappers/trailers and explicit generic repeated-data slices after boundary/conflict inspection. Direct-call closure stopped naturally when no new entry roots remained, not a byte ceiling. Unknown unique interstices and inline/bank-return barriers need redirect investigation L003/L004; this is an ordinary easy-gain handoff, not terminal exhaustion.

Verification: map.validate passed; maps/current.json exported from analysis/state.json; run.py checkpoint pass-001-final validated/generated/rebuilt byte-identically, report analysis/pass-001-final-verification.json. Input SHA-256 remains workspace identity. No toolkit issues found; [toolkit issues](toolkit-issues.md). Evidence retained locally, no toolkit modifications or publication. Disk145MB phase evidence, scratch4KiB,9.8GB free. Final handoff: pass-001/handoff.md.

Lead outcomes: L001 verified, L002 verified; L003/L004 ready for redirect. No unchanged rejected experiment retried.

Final execution-evidence split: Observed code 7175 bytes in 3539 physical instruction annotations; Static code 7254 bytes in 3477 instruction annotations. These are map provenance counts, separate from contextual fetch/hit counts; artificial scenario evidence remains marked in saved contexts.

## Redirect r001 combined research handoff

Frozen pass-001-final; commentary current-source gate complete. Research proposes96381 unique unknown bytes (81427 Data,14954 Code candidates), accepted0; actual98241 unknown unchanged. Exact complement/dispositions and prospective1860 residual bytes: redirect/r001-inline-structure/unknown-ledger.json. Conditional L008 Data review and F3CA/BE36 code/state recipes belong to the same independent disassembly batch, not an artificial extra workflow cycle. No terminal exhaustion claim.

Consumer/record/stack/bank evidence now covers all banks2..9 containers, sound/palette programs, Space/title/status/display/spawn structures and connected scene/actor/dispatch paths. Source and map unchanged. Return-witness sketches require independent branch/loop feasibility and43/44/stack review; CBB5 stack-alias precondition explicit. Twenty-eight independent structural/schema/hash checks pass; no core alignment/kind conflict. Exact evidence/recipes, counterexample localization and residual review: redirect/r001-inline-structure/handoff.md, manual-contracts.md, rejection-review.md and validation.json.

Artificial Haunted interior floor5 and trivia terminal key award verified, with exact clean snapshots/patches and bounded captures; no natural full-chain or named-boss claim. Updated scenarios retain earlier route failures and source correction. Leads L003..L008 and all prior histories preserved. No toolkit/source/map edits, cleanup/deletion, archive or publication.

## Pass 002 disassembly result

Incomplete; ordinary easy-gain handoff, not terminal exhaustion. Accepted97584 new bytes:15318 Code and82266 Data, all Static. Existing classifications/provenance retained. Core96381 accepted, conditional448 accepted, tail698 net (one-byte correction), title57 accepted. Exact review, proposal/accepted slices and correction history: pass-002/review.md.

ROM classification: 99.75% (261503 / 262160 bytes classified; 657 bytes unknown).

| PRG8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 2186 | 6006 | 8192 | 0 |
| 1 | 7 | 8185 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 7 | 8185 | 8192 | 0 |
| 10 | 1384 | 6760 | 8144 | 48 |
| 11 | 1528 | 6652 | 8180 | 12 |
| 12 | 5679 | 2485 | 8164 | 28 |
| 13 | 6759 | 1325 | 8084 | 108 |
| 14 | 6720 | 1113 | 7833 | 359 |
| 15 | 5456 | 2634 | 8090 | 102 |

Header16/16 and CHR131072/131072 classified; remaining unknown is PRG only. Code provenance7175 Observed /22572 Static; no new runtime observations. Prior bounded gameplay/snapshots reused: all five systems/ending, Haunted interior and first/final trivia key verified artificially. Named boss activation/natural full quest unresolved; no unchanged replay.

Final state/current map agree; map.validate passed. Authoritative fresh checkpoint maps/pass-002-final-corrected.json and analysis/pass-002-final-corrected-verification.json rebuild byte-identically. Earlier pass-002-final checkpoint superseded only for C9D6 boundary correction. Input SHA256 unchanged. Final exact unknown complement and per-bank counts: pass-002/final-unknown-ranges.json and final-counts.json. Current-source commentary preserved;15318 newly classified code bytes/82266 data bytes await full commentary phase; no editorial completion credit inferred.

LeadsL003/L004/L005/L006/L008 consumed with prior rejected/blocked history preserved; newL009 carries residual657 bytes for localized bank/stack/entry review. Strongest useful questions: C9D9/CDE6 bounded buffer operations; F8 escape synthetic returnCD42 and map-upload continuations; actor query pop-two return contractC669; final status/title and text-return fragments. No broad missing-caller requirement. No terminal rejection review claim.

No toolkit issues found; confirmed0/suspected0/missing features0/blockersnone. See [toolkit issues](toolkit-issues.md). No toolkit edits, publication, commits, archive or deletion. Compact handoff pass-002/handoff.md.

## Pass 003 disassembly result

Incomplete; ordinary one-pass handoff, not terminal exhaustion. Independently accepted651 bytes:618 Code/33 generic Data, all Static, in38 slices. Fresh exact decodes matched saved objects, while structural acceptance used actual local loop/branch/stack/bank/consumer evidence. Prior classifications and histories unchanged. See [review](pass-003/review.md), [accepted slices](pass-003/accepted.json), [decisions](pass-003/decisions.json).

ROM classification: 100.00% (262154 / 262160 bytes classified; 6 bytes unknown).

| PRG8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 2186 | 6006 | 8192 | 0 |
| 1 | 7 | 8185 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 7 | 8185 | 8192 | 0 |
| 10 | 1432 | 6760 | 8192 | 0 |
| 11 | 1540 | 6652 | 8192 | 0 |
| 12 | 5701 | 2490 | 8191 | 1 |
| 13 | 6867 | 1325 | 8192 | 0 |
| 14 | 7079 | 1113 | 8192 | 0 |
| 15 | 5525 | 2662 | 8187 | 5 |

Header16/16 and CHR131072/131072 classified. Exact unknowns: physical104273 (bank12 CPU9741), length1; physical125800..125804 (bank15 CPUEB58..EB5C), length5. EB58 rejection is localized to insufficient independent structural constraints for a tiny zero-store-return body; no universal caller prerequisite. 96D0 five-byte constants accepted as generic Data after indexed palette reader/pattern/boundary check, without claiming exact table layout. LeadL009 partially verified; remaining6 bytes carry into later redirect. No additional concrete supported connected interval found.

Structural annotations cover262154 bytes; all651 newly accepted bytes carry evidence and Code execution views. Code provenance7175 Observed/23190 Static bytes is structural provenance, not full executed-byte coverage; no new runtime capture or execution-coverage gain asserted. Reused all-five-systems/ending/Haunted-floor5/trivia-key evidence; natural full quest/named boss activation unresolved. No unchanged gameplay replay or fabricated spawn contract.

Validated internal/current maps agree byte-for-byte. Fresh checkpoint [map](../maps/pass-003-final.json), [byte-identical verification](pass-003-final-verification.json), [audit](pass-003/verification.json); ROM hash unchanged. Current map SHA256 1a19aa8e9e018537acb0ab6a0029aad96e563a60389982f3e2f0a734ecf1c775. All14892 preexisting region records preserved exactly. Current commentary unchanged; full commentary refresh for newly accepted618 Code/33 Data and whole-source gate comes next.

No toolkit issues found: confirmed0/suspected0/missing-feature0/blockersnone; [persistent report](toolkit-issues.md). No toolkit edits, archive, cleanup/deletion, publication or commits. Compact [handoff](pass-003/handoff.md).

## Redirect r003 residual constraints

Exactly one redirect completed from pass003-final/current-source readability gate. Classification262154/262160,6Unknown unchanged;0proposed/0accepted. Reviewed connected A3 collision/slope and9742 motion consumers, actual indirect/split/stack target constructors, text bank transform, residual subentries/RTSencoded forms and bounded indexed/copy alternatives. Primary evidence and localized competing-interpretation review: [rejection review](redirect/r003-residual-constraints/rejection-review.md); [findings](redirect/r003-residual-constraints/findings.md). No supported additional structural claim or currently credible discriminating method remains; terminal-incomplete recommendation awaits coordinator inspection, not self-accepted exhaustion.

Residuals physical104273/bank12CPU9741 length1(60), physical125800/bank15CPUEB58 length5(A90085A360). Exact complement independently rederived; ROM/current/frozen/state/source hashes unchanged. L009 history preserved and marked blocked/no ready recipe; [checkpoint](redirect/r003-residual-constraints/checkpoint.json), [ledger](redirect/r003-residual-constraints/unknown-ledger.json), [handoff](redirect/r003-residual-constraints/handoff.md). Existing representative five-system/ending/Haunted-floor5/trivia-key evidence reused; no new named-boss activation contract or natural fullquest assertion. Final editorial audit remains pending. Research/lead records only; no toolkit changes, runtime replay, source/map mutations, cleanup/deletion, archive, publication or commits.

Coordinator inspected r003 decisive primary artifacts and accepted localized structurally incomplete stopping: [review](coordinator-terminal-review.md). Final redirect checkpoint/handoff now records coordinator-reviewed status;6Unknown retained, no new proposal. Final editorial audit remains separate.
