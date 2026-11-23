# Final commentary phase (round 6 / final acceptance) - handoff

Status: **COMPLETE**. The final audit passed, with a fresh byte-identical verification and a refreshed
reading copy.

## Identity
- **Input map:** maps/current.json = analysis/state.json = maps/final-fallback.json = commentary/phase1-state.json
  (sha256 212cccb66dfad0ff...).
- **Previous commentary:** built on aab2fb6c... with source a2b9634c.... It was moved whole to
  analysis/round-5/commentary/, and the round-5 cmt_*.py tools were copied to analysis/round-5/tools/.
  analysis/round-1 and analysis/round-2 are untouched.
- **Fresh baseline:** `commentary.py prepare` was byte-identical and produced 2293 scaffold entries. The only
  difference from the round-5 before/ is the two new "Data; Manual" region headers.
- **Output files:**
  - commentary/source/game.asm: **e5b27876792c597e770e4169a3e658ddd6927385cd477dd2d0e5e15c2f2e3d5d**, built
    from commentary/annotations.txt (647074e1...) by analysis/tools/cmt_apply.py.
  - commentary/review.json: fdd1d6b0....
  - commentary/reading-after.asm: c04f3ed4....

## Verification and audit
- **Audit:** `commentary.py audit` returned ok:true, byte_identical:true, reviewed_regions 2293,
  reviewed_units 1237 and editorial_audit true. Records: commentary/audit.json and
  analysis/round06-final-commentary-audit.json.
- **Verification:** analysis/round06-final-commentary-verification.json.
- **Reading copy:** refreshed by the audit.

## Carried vs changed
- **Carried by physical range:**
  - All names, comments and review credit from round 5. The annotation keys are bank/CPU addresses,
    which are equivalent to physical offsets.
  - progress.md and uncertainties.md.
- **Invalidated:** only the two records that touch the 3 changed bytes (unknown-00400e and unknown-01c010).
  They are now region-2509 (physical 16398+2, bank 0 $BFFE-$BFFF) and region-2510 (114704+1, bank 7 $C000).
  Both were re-described and reviewed.
- **Changed this round:**
  - TK-04 wording in the overview, in BmxFrameSprites and in both region notes.
  - 84 descriptive branch labels: SurfBuildWave 41, NameEntryFrame 20, FootBagUpdatePose 17 and
    SkateFrame_Finish 6, with short section comments.
  - Stale raw-address prose replaced by symbols:
    - CameraXHi:CameraX, SkateEggNames/SkateEggName2
    - DataPtr, Work38/39, ScrollMode
    - AwardFirst-AwardThird, AwardLayout, PresetScores
    - Temp0/Temp1/Work37
  - DrawTeamPatterns description corrected: it loops over the award list, not over registered players.
  - Long prose lines reflowed.
  - uncertainties.md: superseded and resolved items are marked, and there is a new Round 6 section.
  - Workspace tool analysis/tools/cmt_review.py: Data summaries now include the P note above the header.

## Manual fallback (reported separately from proven classification)
- **What was marked:** bank 0 $BFFE-$BFFF and bank 7 $C000 are generic Data with UNKNOWN meaning. This is an
  explicit user-authorized fallback (Manual provenance, confidence 0.1), not an observed or statically
  proven data role.
- **What the bytes really are:** in execution they are the observed cross-window instruction
  "lda $C149,x" (BD 49 | C1). The opcode is at $BFFE, the last operand byte is at fixed-bank $C000, and
  execution continues at $C001 in BmxFrameTail.
- **Why they are not Code:** TK-04. A Code region cannot span non-adjacent physical bytes.
- **Classification:** proven classification is 131085/131088 bytes; all 131088 bytes carry a region kind.

## Remaining uncertainties (commentary/uncertainties.md, Round 6)
- **Generated branch labels:** about 776 Owner_Skip/Loop/Return labels remain. They are exact and
  consistent, and the convention is explained in the overview header.
- **Tentative names:**
  - the half pipe board/rider records;
  - PipeReact*;
  - BmxLift/BmxWheelA/B;
  - SurfRideFlag;
  - the DiskUncaught double use;
  - AwardLayout and the PresetScores medal reading.
- **Foot bag poses:** the pose meanings are not decoded, so those labels are named by value.
- **Other open items:**
  - The round-3 no-caller code: execution is not established.
  - Raw RAM $0167/$0172.
- **Cosmetic:** the "Referenced by" order in generated headers changes between builds because of set
  ordering. Byte identity is not affected.

## Disk
Scratch is scratch/r06c/ (two small helper scripts). analysis/round-5/commentary is about 12 MB.
