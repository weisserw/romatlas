# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 disassembly — pass-001

ROM classification: 60.31% (158102 / 262160 bytes classified; 104058 bytes unknown).

Validated vectors: CPU $FFFA..$FFFF (physical file $2000A..$2000F) map to NMI $E9E4, RESET $FF80, IRQ $EA26 in fixed PRG banks 14/15. Static discovery combined vector paths and actual bank-aware successor edges; all 4,209 accepted instruction starts occupy 8,914 bytes without overlaps. Of those 3,383 starts had observed ROM fetches and 826 were statically justified connected control flow. Observed-decodes initially includes RAM roots decoded against ROM during exploration; **only CPU>= $8000 and actual ROM-space contexts** entered code-proposals, whose physical mapping validation rejects wrong identities. RAM decodes are not ROM evidence.

CHR file [$20010,$40010) is exact header-defined PPU pattern storage; 131,072 Image bytes. Header [0,16) and vectors are Data. Uniform-runs.json records exact maximal $00/$FF intervals >=100 bytes, adjacent bytes and no code conflicts. Boundary/reference/static-neighborhood inspection found non-code repeated records and bank suffixes; classified generic Data without inventing fill purpose. Header+vectors+uniform data total 18,116 bytes. Remaining mixed PRG intervals stay unknown.

Exact per-bank counts are in progress.md. Primary proposals/results, contextual samples, discovery diagnostics and traces summarized under round1/. Map validated; baseline and pass-001 source byte-identically rebuilt. No toolkit issues found (toolkit-issues.md).

Leads R1-RAM-UPLOAD, R1-COURSE-SELECT and R1-CHAMPIONSHIP retain evidence and retry conditions in leads.json. Broad gameplay checklist is in scenarios.json. This is an ordinary substantial-gain handoff, not terminal exhaustion. Further state-contract research belongs to redirect; do not repeat blind course/menu sequences or long races on unchanged premises. No arbitrary RAM edits or register edits were used; ROM backing unchanged.

## Round 2 disassembly — pass-002

ROM classification: 68.20% (178795 / 262160 bytes classified; 83365 bytes unknown).

Accepted nine ready leads after independent exact structural review; +20693 unique bytes, one Code/BRK->Data correction. Static provenance distinguishes source structure from artificial/natural gameplay. Evidence: [review](round2/review.md), [annotations](round2/annotations.json), [accepted decodes](round2/accepted-decodes.json), [counts and residual ranges](round2/counts.json). Current map/state saved and validated; checkpoint maps/pass-002.json generated/pass-002 rebuilt byte-identically, verification analysis/pass-002-verification.json. Map SHA256 bbefab9cc9e3a09a05180c2ce5dd3ba3e01dbdb402c00094dd93bb3b34876f9d.

| Bank | Classified | Unknown |
|---|---:|---:|
|Header|16|0|
|PRG00|884|7308|
|PRG01|296|7896|
|PRG02|0|8192|
|PRG03|2859|5333|
|PRG04|8192|0|
|PRG05|8167|25|
|PRG06|2167|6025|
|PRG07|7417|775|
|PRG08|296|7896|
|PRG09|2256|5936|
|PRG10|2511|5681|
|PRG11|2000|6192|
|PRG12|1323|6869|
|PRG13|940|7252|
|PRG14|4921|3271|
|PRG15|3478|4714|
|CHR00|8192|0|
|CHR01|8192|0|
|CHR02|8192|0|
|CHR03|8192|0|
|CHR04|8192|0|
|CHR05|8192|0|
|CHR06|8192|0|
|CHR07|8192|0|
|CHR08|8192|0|
|CHR09|8192|0|
|CHR10|8192|0|
|CHR11|8192|0|
|CHR12|8192|0|
|CHR13|8192|0|
|CHR14|8192|0|
|CHR15|8192|0|

No repeated gameplay: saved normal Spain/final-course driving and artificial qualifying/final-results/losing-ending/pit-refuel records reused; repair and winning variation unvisited. Fresh connected traversal yielded no new unknown candidates (434 visited, no budget exhaustion). Incomplete ordinary easy-gain handoff; remaining83365 bytes need next source survey/redirect. Commentary must review full fresh source; old complete review applies only to prior map. Two documented missing features persist: uploaded execution views and PRG-RAM debugger editing; see [toolkit issues](toolkit-issues.md). No toolkit edits.

## Round 3 disassembly — pass-003

ROM classification: 80.90% (212079 / 262160 bytes classified; 50081 bytes unknown).

Accepted six ready leads: +33284 unique unknown bytes (11142 Code,22142 Data). Independent review includes5167 fresh decodes,86 context-preserving code groups, explicit mapper-return views and17 compressed-course endpoint proofs. Prior classifications/provenance/views all preserved exactly. Structural claims are Static; no new observed execution. Evidence: [review](round3/review.md), [annotations](round3/annotations.json), [decodes](round3/accepted-decodes.json), [course audit](round3/course-audit.json), [exact counts/residuals](round3/counts.json).

|Bank|Code|Data|Image|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|Header|0|16|0|16|0|
|PRG00|0|884|0|884|7308|
|PRG01|0|296|0|296|7896|
|PRG02|0|8192|0|8192|0|
|PRG03|0|4721|0|4721|3471|
|PRG04|0|8192|0|8192|0|
|PRG05|0|8167|0|8167|25|
|PRG06|0|6936|0|6936|1256|
|PRG07|0|7417|0|7417|775|
|PRG08|296|0|0|296|7896|
|PRG09|0|2256|0|2256|5936|
|PRG10|2176|413|0|2589|5603|
|PRG11|0|2000|0|2000|6192|
|PRG12|6016|1981|0|7997|195|
|PRG13|4011|4050|0|8061|131|
|PRG14|6353|52|0|6405|1787|
|PRG15|3959|2623|0|6582|1610|
|CHR00|0|0|8192|8192|0|
|CHR01|0|0|8192|8192|0|
|CHR02|0|0|8192|8192|0|
|CHR03|0|0|8192|8192|0|
|CHR04|0|0|8192|8192|0|
|CHR05|0|0|8192|8192|0|
|CHR06|0|0|8192|8192|0|
|CHR07|0|0|8192|8192|0|
|CHR08|0|0|8192|8192|0|
|CHR09|0|0|8192|8192|0|
|CHR10|0|0|8192|8192|0|
|CHR11|0|0|8192|8192|0|
|CHR12|0|0|8192|8192|0|
|CHR13|0|0|8192|8192|0|
|CHR14|0|0|8192|8192|0|
|CHR15|0|0|8192|8192|0|

Map/state validated and saved; [pass-003 verification](pass-003-verification.json) confirms byte-identical rebuild in generated/pass-003. Map SHA25660b360155365d7207d52ddb82c33b8d5058fdd8f4f61b172cc28648326f77f35. All241 new regions have evidence/comment annotations;678 historical readability-review records/source/comments remain untouched and need rebase plus full fresh-source review. Current commentary credit is for pass-002 only; final editorial gate remains pending.

Fresh connected traversal visited757 candidates, no newly unknown candidates, no budget exhaustion. Two indirect diagnostics122487/122843 correspond to the reviewed finite rival state/render tables; native traversal does not enumerate these automatically. Saved representative natural/artificial gameplay and repair attempt reused; no repeated broad replay, new execution or winning/natural-completion claim. Remaining50081bytes and localized entry-less fragments remain unknown. Next step is full current-source commentary, then source-derived redirect; no terminal exhaustion claim. Two documented missing toolkit features persist (uploaded execution views and PRG-RAM editing), no new issues; [report](toolkit-issues.md). No toolkit/ROM/source edits, cleanup or publication.

## Round 4 disassembly — pass-004

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).

**Incomplete:** the rounded100.00% is not full classification; four ranges retain5Unknown bytes. Accepted all six independently reviewed ready R3 leads, +50076 unique bytes (6884 Code,43192 Data) in367 new regions. All earlier regions/labels/comments/evidence/provenance/views preserved exactly. No new Observed execution or natural reachability claim. Primary independent evidence: [review](round4/review.md), [fresh decode audit](round4/audit.json), [finite formats](round4/format-audit.json), [data references](round4/data-reference-audit.json), [positive pattern rows](round4/generic-chunks.txt), [accepted annotations](round4/annotations.json), [counts/residuals](round4/counts.json).

109 context-preserving groups/3193 instruction views freshly decoded. Independent finite consumers, exact stream endpoints, branch/stack/sibling contracts and broad positive Data patterns support classification. The33-byte9D72 body is the exact stack-argument modulo correction/clamp; 99+65=FE then+6 wraps04, so no general saturating-decimal-add correctness claim. Four narrow residuals are isolated00, overwrittenA51B, isolatedRTS and isolatedFF; [localized rejection review](round4/review.md) retains genuine structural ambiguity without requiring live callers for complete static routines.

|Bank|Code|Data|Image|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|Header|0|16|0|16|0|
|PRG00|0|8192|0|8192|0|
|PRG01|19|8173|0|8192|0|
|PRG02|0|8192|0|8192|0|
|PRG03|19|8173|0|8192|0|
|PRG04|0|8192|0|8192|0|
|PRG05|19|8173|0|8192|0|
|PRG06|0|8192|0|8192|0|
|PRG07|19|8173|0|8192|0|
|PRG08|4105|4087|0|8192|0|
|PRG09|19|8173|0|8192|0|
|PRG10|2194|5997|0|8191|1|
|PRG11|19|8173|0|8192|0|
|PRG12|6211|1981|0|8192|0|
|PRG13|4136|4056|0|8192|0|
|PRG14|7638|551|0|8189|3|
|PRG15|5297|2894|0|8191|1|
|CHR00|0|0|8192|8192|0|
|CHR01|0|0|8192|8192|0|
|CHR02|0|0|8192|8192|0|
|CHR03|0|0|8192|8192|0|
|CHR04|0|0|8192|8192|0|
|CHR05|0|0|8192|8192|0|
|CHR06|0|0|8192|8192|0|
|CHR07|0|0|8192|8192|0|
|CHR08|0|0|8192|8192|0|
|CHR09|0|0|8192|8192|0|
|CHR10|0|0|8192|8192|0|
|CHR11|0|0|8192|8192|0|
|CHR12|0|0|8192|8192|0|
|CHR13|0|0|8192|8192|0|
|CHR14|0|0|8192|8192|0|
|CHR15|0|0|8192|8192|0|

[pass-004 verification](pass-004-verification.json) confirms validated byte-identical rebuild in generated/pass-004. State/current/checkpoint map SHA2563a5de61434a4e0bb43aa7e3bc3ec2b46a4e88a2262b8c3c65ffd53c6928c7381. Connected traversal1165candidates, no newly unknown candidates, no budget exhaustion; sole diagnostic83315 is an already classified Data barrier, not a new unresolved entry.

Saved representative natural Spain/final-course and artificial qualifying/finalrace/results/losingending/pit-refuel plus repair attempt reused; scenarios progression checklist reviewed, no unchanged broad replay, new execution, winning or naturalchampionship-completion claim. The four residual byte-role questions do not gain evidence from unchanged gameplay replay.

All893 prior current-source review records and commentary/source/history preserved; their credit remains for pass-003. Full fresh pass-004 source commentary/rebase is pending for367 new structural regions plus verification of preserved context; [readability backlog](round4/readability-backlog.md). Final editorial gate remains pending. Next: commentary then redirect residual review; this is not terminal universal exhaustion. Two documented missing toolkit features persist (uploaded execution source views and PRG-RAM editing), no new issues; [report](toolkit-issues.md). No toolkit/binary/ROM changes, archive, delete, cleanup or publication.

## Round4 redirect current-source residual review

See redirect/r4-current-residuals/rejection-review.md and currentremaining-ranges.json. Full readability supplies named neighbor/control contracts but no positive data pattern or sufficient isolated code contract for83139,117377..117379,121095,125983. Prior primary evidence compared with current source; no unchanged replay. No ready lead or productive unfinished frontier; terminal-incomplete recommendation for coordinator review. Future reopening requires new evidence or concrete corrected inference.
