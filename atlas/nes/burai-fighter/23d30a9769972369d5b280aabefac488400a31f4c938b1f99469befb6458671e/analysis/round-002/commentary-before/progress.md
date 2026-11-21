# Commentary progress — round001 active

Full mechanical reading of every classified instruction is complete, physical $37FF..$8010 (with classified data gaps and unknown $5725 excluded from instruction decoding). Reviewed source metadata is being applied; this is **not yet a successful handoff**. Prepared baseline verified byte-identical.

Durable authored routine inventory/prose: analysis/round-001-commentary/edit.py and units.json. Covers camera/tile expansion, all actor update and spawn paths, player/weapon/bomb, sound bytecode, collision geometry/dispatch, runtime frame/stage controller, uploaded CHR-to-RAM helpers, PPU/NMI/IRQ, HUD and fixed RAM ABI. Full instruction inventory: compact.txt; original parsed source: parsed.json. The frozen commentary/before remains untouched.

Remaining work: apply names/comments/constants and branch explanations across **all** source code physical [$37FF,$8010) excluding existing data gaps and sole unknown [$5725,$5726); annotate all classified data [$0000,$37FF), embedded gaps and CHR [$8010,$10010); fill review scaffold source locations/meaningful units; check symbol consistency and derived comments; fresh verify; refresh reading-after. No source edits yet, so no new source verification claim.

Concrete redirect issue found: physical [$6A4F,$6A50), CPU $EA3F, classified Static BRK, is also read as first byte of two-sprite offset table at CPU $EA3F,Y with Y=0..3. Preserve classification pending structural review. Primary consumer is CPU $E9E7/$EA0D in identity PRG view [0,1,2,3]. Exact lead will be retained in uncertainties and handoff.

Final editorial audit remains pending for coordinator acceptance. Whole-ROM classification remains 65551/65552; unknown $5725 is unchanged. No toolkit/input/map edits, deletions or publishing.

## Source application milestone

Applied complete first readability pass: 21 functional assembly modules, shared symbols, 287 routine/shared-entry explanations, 100 PRG data explanations plus header/unknown/CHR units, and all 952 encoded control destinations named by their established roles. All 8588 code instructions retain their original encoding. No generated region labels remain in maintained source. All 8617 scaffold entries now carry readability review credit and source locations; 390 meaningful units recorded. Editorial acceptance remains pending.

Fresh commentary verify after this coherent edit batch: **byte_identical=true**, input/source linked SHA256 23d30a9769972369d5b280aabefac488400a31f4c938b1f99469befb6458671e. Remaining before handoff: consistency/prose pass on applied symbols/aliases and unit summaries, durable uncertainty/redirect lead, final fresh verify and refreshed reading copy, compact handoff/coordinator progress update. No phase/map/classification changes.

## Completed current-source readability gate

The consistency pass finished. All 8588 classified instructions, 287 meaningful code/shared entries and 952 encoded control destinations now use explained mechanics and consistent symbols. All 16-bit instruction operands have meaningful symbols or exact interior aliases; reusable RAM is described by context (including the OAM back buffer versus uploaded RAM). Corrected the row-transfer description to exactly32 bytes and clarified directory prefixes versus unresolved following records. Source has21 functional modules and shared symbols. Final inventory:391 meaningful units, all reviewed;8617/8617 scaffold records reviewed with source locations. No remaining current-source readability ranges.

Fresh final verification passed byte-identically; exact report analysis/round-001-commentary/final-verification.json and commentary/verification.json. Linked/input SHA25623d30a9769972369d5b280aabefac488400a31f4c938b1f99469befb6458671e. Refreshed reading-after.asm SHA256b49b4cd9fc7b5839711b17f2bcfca00f0e9feb84087fe9b883d47c302e0786ff. Map/current, state and frozen baseline all retain SHA256f41710f57e8666a751108587e1af0095a64a890466f1a045c55360ce82158201. Completion ledger/final unit inventory are under analysis/round-001-commentary/. Last disk check3.2GiB available; no deletion performed.

Unresolved meanings and exact structural questions are in uncertainties.md and analysis/round-001-commentary/redirect-leads.md. Unknown remains physical[$5725,$5726); C001 preserves current classification at[$6A4F,$6A53) while documenting a finite four-byte offset consumer and always-taken clipped-tail branch for independent redirect. No classification, ROM, toolkit or publication changes. Final editorial audit remains pending for coordinator/final acceptance; it was not run here. Whole-ROM classification remains65551/65552, separately incomplete.
