# Round001 commentary progress

Current-source readability complete for pass-001-final; whole-ROM classification remains incomplete.

Inspected all 9,197 original assembly lines. Reviewed all 798 physical scaffold entries: 589 code, 8 data, 4 image and 197 unknown. All 12,434 classified code bytes have reviewed explanations across 173 subsystem ranges; all data/image includes carry their retained evidence/role notes and unknown inclusions carry explicit boundary uncertainty. Exact inventory and source locations are in review.json, subsystems.json and coverage-round001.json. There are no remaining unread or unannotated classified ranges.

Source now has physical-bank-qualified routine/branch labels, named hardware registers, persistent RAM state, object fields, pattern-transfer descriptor fields and consumer-proven table aliases. Neutral Temp00..Temp0F names preserve caller-owned scratch reuse. Array fields are base addresses indexed by byte offsets; strides are documented and used as constants in pool loops. PrgN is a physical 16 KiB grouping/mapper selector, not an effective 8 KiB API bank index. Each subsystem inventory supplies the applicable effective bank pair or fixed-slot constraint.

Examples of meaningful control explanations: audio base/overlay ownership and counted stream loops; inline JSR-table tail dispatch and frame-unwinding allocation failures; background priority requests and nametable wrap; 8.8 enemy/player movement and collision bounds; score decimal carry and resource borrow; upgrade repeat/eligibility and confirmation; gameplay frame sequencing and uncertain result ratio/success handling. These explanations are in assembly beside the routines, not only this record.

Consistency pass corrected four initially proposed entry addresses that were not instruction boundaries and a misplaced sprite-order label, named previously merged audio command entries, covered the one three-byte effect branch and added detailed bank-4 sprite/metatile/direction formats. No buildable byte sequence was changed. Existing structural evidence, frozen before/, baseline map, phase1 state and ROM were preserved.

Final verification: commentary/verification.json reports ok=true and byte_identical=true; source game.asm SHA256 516546b9a822d3be95a01fc28c173266326ebdde5607bc19f1928157fca6da5b. reading-after.asm refreshed after that verification. ROM SHA256 remains 2fd596ac4563b910a4d0a36d4a3658910fcfa822afd4f78bd62f73552e17ea00. baseline.json equals maps/current.json and phase1-state.json equals analysis/state.json.

Remaining semantic uncertainty is in uncertainties.md: inline selector/table bounds, banked command contracts, late progression/result meanings, partial special handlers, data schemas and specific upgrade/audio identities. No remaining readability gap is deferred. Redirect leads R001–R004 remain available; no redirect work was executed in this phase. Final editorial/archive acceptance remains pending and no audit was run.

Toolkit: two pre-existing confirmed documentation/API issues, zero suspected issues, zero missing features, no blockers; no new toolkit issue or modification. See ../analysis/toolkit-issues.md.
