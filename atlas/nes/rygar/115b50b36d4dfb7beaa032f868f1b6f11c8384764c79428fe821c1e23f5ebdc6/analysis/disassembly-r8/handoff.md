# Round8 disassembly handoff

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Revised user policy and attachment independently evaluated. Accepted13538previously Unknown bytes in72regions:13276StaticData/70unlabeled generic regions and262StaticCode/2regions/144starts. All1801prior region objects unchanged;1873total,22881Code/11451starts,108182Data including16header bytes,25Text. Exact Unknown complement empty. No unresolved effective bank contexts in accepted code views; natural entry/use remains unconfirmed for new static routines and many data blocks.

[Structural review](review.md), [exact boundary review](data-boundary-review.json), [routine audit](routine-audit.json), [all11307old-start conflict scan](code-conflicts.json), [accepted annotations](accepted-regions.json), [counts](classification-counts.json), [empty complement](unknown-ranges.json), [preservation](preservation.json). Generic Data deliberately claims only non-code role; no invented format/consumer/padding/reserve semantics. L013verified252-byte9354alternate terrain updater under[12,13,14,15], independent of prior9350failure;1024threshold-selector and33792loop-state cases checked. L014verified generic non-code bulk and interior field gaps. L015verified10-byteC780signed scaling prefix into existingC78Ahelper; all256signed inputs checked, old suffix preserved. See analysis/leads.json.

| Bank8 | Code | Data | Text | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 0 | 0 |
| 1 | 0 | 8192 | 0 | 0 |
| 2 | 0 | 8192 | 0 | 0 |
| 3 | 0 | 8192 | 0 | 0 |
| 4 | 0 | 8192 | 0 | 0 |
| 5 | 0 | 8192 | 0 | 0 |
| 6 | 638 | 7554 | 0 | 0 |
| 7 | 0 | 8192 | 0 | 0 |
| 8 | 0 | 8192 | 0 | 0 |
| 9 | 0 | 8192 | 0 | 0 |
| 10 | 3273 | 4919 | 0 | 0 |
| 11 | 5979 | 2213 | 0 | 0 |
| 12 | 5156 | 3036 | 0 | 0 |
| 13 | 1138 | 7054 | 0 | 0 |
| 14 | 5845 | 2322 | 25 | 0 |
| 15 | 852 | 7340 | 0 | 0 |

Fresh configured `python3 run.py inspect` passed. Fresh map.validate/save/export passed; internal=current=pass008. Fresh `python3 run.py checkpoint pass-008-reviewed` validated/generates/ca65+ld65 reassembles all131088bytes byte-identically, ok=true/byte_identical=true. [Verification](../pass-008-reviewed-verification.json), [checkpoint](../../maps/pass-008-reviewed.json), [generated listing](../../generated/pass-008-reviewed/game.asm). ROM SHA256115b50b36d4dfb7beaa032f868f1b6f11c8384764c79428fe821c1e23f5ebdc6 unchanged. Existing configured release used, no toolkit/source edits/rebuilds or commits. All24prior maintained commentary source hashes match r7; labels, variables, branch comments, crosswalk and347unit/11307start review credit retained.

Broad gameplay checklist persisted in [scenarios.json](../scenarios.json), with [gameplay familiarity](../gameplay.md). Supported MainState6destination pipeline visits12/17/24/3A/47: final numeric areas03/07/0D/16/1B, each distinct rendered active gameplay plus move/attack/jump/release inputs. Exact patches0300..031Ftask set,1Ddestination,41step,CA/CBhealth48, clean exported snapshots, budgets and effective banks retained. Area names not inferred; rope/water scenery is not claimed traversal. Full frame and actual accepted fetch evidence saved. Successful status spell: Mind20/selector1/freshA -> Mind15/CD10. Boss arena47visited and two bounded far/close combat attempts emitted weapon/projectiles and fetched attack initializer; boss health05E2remained15, no hit or victory. Close-position patch0512=B0/0502=00 follows clean restore. Do not force indefinitely.

Ending uses established main10/step0 full22-step initializer, then supported timer acceleration from exported clean step8snapshot (0533=FF;2B=00 only signed-delay states; heldStart at15). Reaches main0/frame685 and visible terminal END. This is artificial progression, not natural completion/boss victory. [END](ending-terminal.png), [ending exact timer/input log](ending-timer-log.json), [spell recipe](spell-recipe.json), [boss recipe](boss-actions.json), [close attempt](boss-close-actions.json). Prior natural routes remain distinct.

All retained complete-paged artificial fetched roots are checked against current Code classifications with no Data conflicts; aggregate_dropped=0. Original long captures contain first1000roots, so all six original patch routes were deterministically repeated for complete pages:3511/3609/3853/3839/4340/2409contextual root records (21561summed records), all zero non-Code conflicts, aggregate_dropped=0 and exact original final CPU/state/RAM equality. No classification correction was needed. [Complete original-route audit](original-route-fetch-audit.json), [additional short/boss/spell/ending fetch audit](artificial-fetch-audit.json). Detail drops at zero-capacity tracing are expected; aggregate drops are checked separately. Snapshots remain local same-build artifacts; no full-hardware compatibility or natural reachability claim.

Structural classification complete, but full current-source readability/final acceptance remain pending. Next phase commentary must integrate72new map regions into preserved maintained source, comment new144instruction starts with variables/branches and proper CPU aliases, and replace/reconcile all71old Unknown scaffolds/prose with short generic-data uncertainty notes where applicable. Preserve previous347functional reviews/11307starts, labels/symbols/crosswalk; generic Data requires no invented names or meaning. Fresh maintained-source rebuild/read/hash verification and final editorial/archive audit still required. This worker did not run commentary or redirect.

No toolkit issues found; [report](../toolkit-issues.md). Only portable NES JSON mapping is distributable; input, snapshots, traces, source/assets stay local.
