# Round 2 redirect — combined ready batch

Input is pass-004: 150,757 / 163,856 classified bytes (92.01%), with exactly 13,099 Unknown bytes in 155 ranges. Current-source commentary is complete; see analysis/round2-commentary-handoff.md. Its byte-identical verification passed, source SHA256 is 530c7d0404b310ffba3ec0271547890ed590187da93e12e1a6604679632c8047, and final editorial audit remains pending.

The combined Static proposal is **12,957 bytes: 8,371 Code and 4,586 Data**. Nothing has been annotated or accepted. If independently accepted in full, 142 Unknown bytes would remain in 15 subintervals, giving 163,714 / 163,856 classified bytes. This is a ready disassembly handoff, not terminal exhaustion or ROM completion.

## Reviewable package

- proposals.json: exact physical intervals, provenance, effective Code views, local supports and Data evidence.
- proposal-instructions.json: all graph and local finite-bank candidate instruction records. Use this combined record when reviewing views; graph-final.json alone omits the separately audited 23-byte stage-bank continuation.
- graph-final.json: bounded connected graph, exact predecessors, 216 inline sites and remaining actual bank/stack barriers. Traversal queue is empty; no budget interruption is being called exhaustion.
- final-contracts.json / final-contract-decodes.json: conservative ordinary/conditional return summaries and primary contextual decodes. contracts.md explains each manual summary and why universal return, global shadow or natural-selector prerequisites were narrowed.
- extra-code.json, stage-bank-local-primary.json and stage-bank-local.md: exact C5FD..C614 continuation under eight possible selected lower pairs, then the two supported restored pairs after F777.
- data-inspection.md, data-primary.json, data-primary2.json, data-primary3.json and ppu-source-extents.json: practical consumer/boundary work and exact copied source unions.
- unknown-ledger.json: exact original 155-range Unknown complement, neighbors, bank/CPU context, proposed versus unclaimed bytes and localized dispositions. residual.json is the precise 142-byte unclaimed subset.
- handoff-checks.json: unchanged ROM/map/state/source hashes, exact complement, no overlaps/current-map conflicts and exact physical resolution of all proposed Code views.

## Connected and independent findings

FA81 has a static bank-preserving contract when mode 3, a complete serial latch and matching remembered register 04A3 are supplied. F76A provides concrete shadow-producing anchors, including boss register 5 and normal register 6. This is not a claim that every actual caller satisfies a global invariant. The sound dispatch and all its handlers were already independently bounded. Sound return is an ordinary balanced X/Y return under that precondition.

CEE3 has a different contract: it saves status/X/Y and the task stack, transfers to the NMI scheduler, and eventually resumes through the saved JSR return. It is never modeled as an immediate ordinary subroutine. A restored lower-pair condition is explicit where banked resumption matters. Finite task initialization supplied the missed task-0 entry C658: CEFA contains C657, and C1DD seeds index zero after F76A selects register 6. This producer/stack contract opened the startup/title path. No sixth initializer word was extrapolated from six scheduler slots.

Several old rejection reasons were conservative proof-checker limitations, not contrary observations. Decimal score writers have bounded zero-page score pointers; OAM and map copy routines have encoded or caller-produced RAM destinations. Palette and password dispatchers push synthetic return addresses before RTS and are not unbalanced ordinary-return routines. Some collision/actor helpers have separate normal and nonlocal branches: only the explicit normal branch supports post-call code. Tail summaries and fallthrough into already audited helper entries preserve these bounded facts. contracts.md records each correction and keeps its counterexample local.

The independent bank 10 projectile-contact routine at 8879 is supported by its finite OAM proximity loops, balanced owner/projectile stack roles, sound/score award, boss HP and flash updates, burst-mask changes and accepted E1DF projectile tail, in addition to the 8159 veneer. See boss-contact.md and bank10-8879-primary.json. Natural caller selection remains unresolved. The connected ending graph was followed through newly supported handlers, transitions, row helpers, rescues, credits and background consumers; saved gameplay routes were not replayed.

The password-input dispatcher has eight two-byte return words at FEDA, from FE32's finite button scan, doubling and RTS contract. All selected handlers were followed. The title uploader F992 uses the existing F922 outer-frame word reader: C1B6's descriptor 60 and block count 1C select 28 * 16 = 448 source bytes at FACE, ending independently at FC8E. C1C4 supplies two blocks at C1E5, proving 32 palette bytes. Bank 10 993A loads encoded source 99AA and copies 25 * 32 = 800 tile bytes through 9CCA, with a separate twelve-byte palette prefix. Other finite PPU row/header copies add their exact source unions only.

The remaining patterned lookup, sprite, velocity, motion, credits and title/copyright subintervals were inspected against nearby Code and their available consumers. Generic Data is used where clear noncode is sufficient but original table domains or subdivisions are unresolved; meaning uncertainty did not leave those bytes Unknown. Data-specific mechanical descriptions and limits are in data-inspection.md.

Finally, C614/F74A does change the lower pair. Its five shifts bound the full eight-value stage-bank domain. C5FD's fixed continuation is therefore proposed under those actual selected pairs, not a pretend preserved incoming pair. The final C611 JMP uses restored normal/boss pairs after F777. This local proof closes 23 bytes without requiring a global scheduler or natural-stage invariant.

## Remaining avenues and actual deferrals

The exact 142-byte residual is not a list of unexplored large structures. It consists of fifteen short islands. Two six-byte sound-disable candidates, isolated JMP/RTS/LDA/clamp fragments, alternate controller/menu/display helpers and an apparent two-byte FACE word lack an independently supported entry, selected boundary or sufficiently discriminating wider control/record contract. Nearby accepted flow bypasses these islands. The proposal does not call them unused, dead or Data. Legal decoding, an address-like word or a plausible hardware write alone was not promoted.

The ten-byte E902 continuation is a different barrier: its caller invokes C14E, which selects bank 6 and clears task-stack RAM 0110..01CF. Ordinary return needs a concrete safe-SP/return-frame contract; merely restoring a bank tuple cannot establish that. No unchanged reset replay or invented saved-stack prerequisite was attempted. Standalone F76D/F74A/F74D and C614 still change banks; CE8C switches stacks; FA6E can remove a frame. Their conservative graph barriers are not missing Code proposals by themselves, and their actual supported consumers were handled separately.

No already identified promising connected handler or independent patterned block remains pending in this attempt. The supported batch is ready for independent disassembly. Remaining islands can be reassessed after acceptance and full current-source commentary; this handoff does not claim a sound terminal-incomplete assessment or waive the required later rejection review.

## Preservation and verification

ROM SHA256 remains fc521e891d074369d67579fd56617460b9c9949dec67cc4a37d46b4e4bb09d29. maps/current.json and analysis/state.json both remain SHA256 563b559e3c5d2a1a0d676f2f7734997365a4ddd6bd236f11454e305d2a5f2f41. Source and byte-identical verification scope are unchanged. Final checks confirm 12,957 unique proposed bytes, zero overlaps/current-map conflicts, and exact effective-view physical resolution. No source, map, state, ROM, toolkit or configured binary was changed. Research servers exited; no emulator scenario was started. Prior r1 evidence is immutable.

No new toolkit issues found. Existing confirmed documentation/API issue NES-001 remains in analysis/toolkit-issues.md, with a bounded explicit-buttons workaround and no blocker. There were no toolkit edits, cleanup/deletion, publication, archive, live server or goal-completion actions. Final disk check and directory size are recorded in checkpoint.md. All ROM-derived artifacts remain local.
