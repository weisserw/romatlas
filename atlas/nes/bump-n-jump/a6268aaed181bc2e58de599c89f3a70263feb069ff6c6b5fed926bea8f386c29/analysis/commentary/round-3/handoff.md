# Round 3 commentary handoff

Current-source readability complete: all 14,211 Code and 51,330 non-code bytes covered; 11 unknown bytes have honest local boundary notes. No remaining readability ranges. Classification remains incomplete at 65,541/65,552 (99.98%). Final editorial/archive audit remains pending; no audit invoked.

Entire prior commentary preserved in analysis/round-3/commentary before fresh preparation. Exact physical/dependency reconciliation retains 165 unchanged scaffold entries and all prior 359 routine/57 data-structure units. Current inventory: 167 reviewed scaffold entries, 360 routine units, 57 data-structure units; zero uncovered code bytes. Review source locations relocated by the exact inserted-line delta. [Scope](scope.json), [carried ranges](carried-ranges.json).

ConditionalStartCourse at physical $049F..$04A3 / CPU $848F..$8493 now names its established mechanics: A=controller mask, X=valid course 0..15; BIT pad1_current selects the X-preserving $8495 join or default-zero $8493 path. Actual caller/natural selection remain unknown. ReadControllers, PollTitleStart, StartSelectedGame and ChooseCourseIntro dependencies were checked against pass-003 review and source; their previous explanations remain sound. Preceding $049E RTI remains independently unknown. No structural change or new runtime experiment.

Fresh commentary verify passed byte-identically; reading-after.asm refreshed. Source SHA256 079bf436b43182359aed91c3d1f072da34b0f7a7971bc6358c037d01cf525d61. Current/frozen map and state SHA256 90c4126c01fcd06e3dc687064fda1288f02ec1e022ec37049170ffe359a75c16. Input ROM identity unchanged.

Exact end-exclusive unknowns: $007C..$0080, $0238..$023C, $049E..$049F, $1219..$121A, $3B8A..$3B8B. B9 triangle width, dirty-flag/shared-scratch corrections and prior semantic/reachability limits remain preserved in source and uncertainties.md. No fresh redirect question beyond the existing localized uncertainty; no redirect/disassembly performed.

No toolkit issues found. No toolkit/input changes, deletion, cleanup, archive or publication. Disk free 3.3 GiB; no exploratory scratch/capture batch created. Next phase is coordinator-directed redirect; this handoff does not establish terminal exhaustion, full classification or compatibility.
