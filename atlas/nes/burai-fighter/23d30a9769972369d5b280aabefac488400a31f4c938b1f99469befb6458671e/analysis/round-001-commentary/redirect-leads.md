# Commentary structural leads for independent redirect review

## C001 — two-sprite offset table starts one byte before its current data annotation

Candidate physical [$6A4F,$6A53), CPU [$EA3F,$EA43), physical 8 KiB PRG bank 3, effective PRG slots [0,1,2,3]. Current map: [$6A4F,$6A50) Static Code (BRK), [$6A50,$6A53) Static Data. **No classification was changed in commentary.**

Primary source: commentary/source/actor-support.asm:103 (RenderTwoSpriteFrame), :121 (CPU $E9E7), :139 (CPU $EA0D), :162 (clipping tail), :171 (boundary). Frozen original has the same instructions. review.json region-3357 retains phase-1 evidence: generic connected-static path, not a captured BRK fetch. The two LDA consumers are Observed from analysis/round-001/ending-0-contexts.json, recorded in scaffold regions3312/3330. Map hash remains f41710f57e8666a751108587e1af0095a64a890466f1a045c55360ce82158201.

Independent constraints worth combining:

- The renderer initializes Y=0, reads $EA3F,Y for Y offsets, increments Y, reads the same base for X offsets, increments Y again and repeats until CPY #4. These paths consume exactly offsets0..3 (clipped paths advance the same two-byte stride).
- Thus $EA3F itself supplies the first offset; the already classified following three bytes complete two Y/X pairs. Exact bytes can be inspected locally from input/game.nes or source/image.nes.
- Normal completion returns at CPU $EA35. The only immediate linear predecessor of $EA3F is BMI $EA2B at $EA3D. Its preceding LDA #$EF at $EA38 sets N and intervening STA $0200,X at $EA3A does not change N, so this branch is always taken under the established path. Treating its syntactic fallthrough as reachable may have admitted the BRK.
- No direct call or branch destination to $EA3F occurs in the maintained classified instruction set. This absence alone is insufficient, but the exact finite table consumer plus independently established return/always-taken tail discriminates data from the simplistic fallthrough route.

Bounded independent check: inspect the original connected-static root/edge that admitted region-3357, confirm all entry/control contracts at $EA38/$EA3A/$EA3D and table index bounds, and review whether any separate supported entry establishes an executable BRK. If none conflicts, propose the four-byte data table with exact bounds and preserve Observed provenance for its executing consumers separately. This is a stronger structural argument than only noting a read/no fetch; original author/linker intent is unnecessary. No gameplay rerun is requested solely for this question.

## D001 — unchanged actor-allocation extra return ambiguity

Physical [$5725,$5726), CPU $D715, PRG bank2, effective [0,1,2,3]. commentary/source/stage-commands.asm:987 onward preserves the scan, reset failure jump, isolated unknown60, and separately targeted success RTS at $D716. Existing decisive investigation: analysis/round-001/orphan-rts.txt and residualdecode-0x5700.json. Independent redirect must review the rejection's exact scope under evidence-policy.md; commentary adds no discriminating entry contract and does not convert this byte to code/data.
