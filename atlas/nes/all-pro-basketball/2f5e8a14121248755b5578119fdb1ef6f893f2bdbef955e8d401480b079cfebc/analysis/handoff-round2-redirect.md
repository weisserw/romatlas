# Round 2 redirect — combined ready batch, writer released

Input checkpoint **pass-002-reviewed**. Actual classification remains **259,219 classified / 2,941 unknown bytes in 94 intervals**. Map/state SHA256: `c729ce4e85bf60f52a390ee488c35c291e915a0fa27ab927b23d8e0501adb176`. ROM SHA256: `2f5e8a14121248755b5578119fdb1ef6f893f2bdbef955e8d401480b079cfebc`. ROM, map, state, source and toolkit are unchanged. The current-source readability gate remains met; final editorial acceptance is pending. No live server operations remain.

**Ready for independent disassembly: 2,897 proposed bytes — 2,768 Code / 129 Data, all Static.** Redirect accepted no classifications. If accepted in full, the prospective complement is **44 bytes in 10 intervals**.

- [Exact proposal](redirect/r2-contracts-20261105/proposal.json), [verification](redirect/r2-contracts-20261105/verification.json), [structural contracts and reasoning review](redirect/r2-contracts-20261105/contract-review.md).
- [Exact 94-interval ledger](redirect/r2-contracts-20261105/unknown-ledger.json) includes CPU/bank contexts, accepted neighbors and proposed/residual splits.
- [Persistent leads](leads.json) preserves all history. New research is confined to `analysis/redirect/r2-contracts-20261105/`, apart from this handoff and the required leads update.

| Ready lead | Proposed bytes | Finding |
|---|---:|---|
| R2-RESIDUAL-CONTRACTS | 1,410 Code | Graphics in banks 8/10; fixed AI, trajectory, pairing and score paths; bank wrappers and indicator stack contracts |
| R2-UPPER-COPY-CONTRACTS | 1,354 Code | Localized upper-F execution retaining original lower banks 10/11, with prefix exceptions excluded |
| R2-SOUND-RESERVED-BRK | 4 Code + 2 Data | Two DEY/BRK sound stubs and their skipped signature bytes |
| R2-RESIDUAL-DATA | 127 Data | Consumer-backed base words, selectors, coordinates, encoded returns, probabilities, AI words and velocity fields |

The [boundary audit](redirect/r2-contracts-20261105/boundary-review.json), [neighbor excerpts](redirect/r2-contracts-20261105/residual-neighbors.txt) and per-range rationales explain the ordinary paths. Important localized results:

- Physical `1EE5A` balances the PRG-latch PHA preceding the accepted threshold branch: PLA, C755, restore X, then the existing action continuation. The earlier rejection had stopped at PLA without reviewing this contract.
- `1EDE8` restores a saved half-court flag across the partner call. Its BMI selects the formerly isolated RTS at `1EE16`; the preceding three-byte JMP remains unknown.
- Bank 8 contains a direct-PPU column loop, a `24 * tile_row + base` helper and a cell reader. **C0CF selects the direct service at 8006; C0CC selects queued service 8003.** The final proposal corrects the earlier queue shorthand and records the safe-rendering precondition.
- Bank 10 closes render returns, saved-elevation overlays and mirrored composite OAM emission. The fixed pairing graph splits around the 16-byte velocity table `[1F4F9,1F509)`; no linear decoding crosses it.

The decisive prior late-copy rejection was retrieved: different prefix bytes and the missing B53C lookup invalidate wholesale lower-B transfer under `[14,15,14,15]`. The new method uses **upper CPU F with original lower banks `[10,11]`**, retaining original B callees/data. Relative branches remain in the matching suffix. [Instruction comparison](redirect/r2-contracts-20261105/late-comparison.json) found 680 complete original instructions covering 1,381 unknown target bytes, but excluded the dangling operand at `1F7C9` and the entire 27-byte routine whose BEQ enters the differing prefix. The remaining 1,354 bytes have localized contracts. Earlier accepted lower-alias findings remain intact.

A new [decoder comparison](redirect/r2-contracts-20261105/late-runtime-comparison.json) restores the initialized editor for each variant, selects latch 5, writes `BEOLEHPPHRMR`, and runs BE2F versus FE2F to a synthetic return at 0700. Both take 364 operations and produce identical CPU/output RAM and stage 34; only the expected stack return-address high byte differs. This corroborates **artificial mechanics**, not natural entry or every copied routine. The first 300-operation setup stopped inside a pending NMI; the [mapper probe](redirect/r2-contracts-20261105/mapper-probe.json) identifies it. Revised shadow/budget setup drains it deterministically. The failed attempt is preserved.

The [sound test](redirect/r2-contracts-20261105/sound-trap-test.json) combines the accepted 32-word command table, parser DEY convention and IRQ RTI with the artificial path `84DE → 84DF → C2AD → 84E1`. Byte 84E0 is a skipped BRK signature, rather than a fetched RTS. Both physical copies retain Static provenance.

Verification decoded **1,309 proposed instructions** and checked **317 direct control edges**, all resolving to accepted/proposed Code with explicit selected contexts for wrapper calls. Hash/overlap/extent checks pass. These checks supplement structural evidence; they do not replace it. All 129 proposed Data bytes received practical consumer/neighbor inspection; complete natural selector domains are not required for non-code classification.

Prospective residual half-open physical ranges:

`[2410,2413) [659D,659E) [A410,A413) [E59D,E59E) [1C253,1C254) [1D357,1D35A) [1DF41,1DF42) [1EE13,1EE16) [1F7C9,1F7CA) [1F7DE,1F7F9)`.

These contain two JMP-0000 stubs, three isolated returns, an unreferenced JMP-ED2C, a single `1C` byte with an overlapping-NOP alternative, an unreferenced JMP-EEAC, the dangling copied operand and the copied routine with a concrete bad branch. [Localized rejection review](redirect/r2-contracts-20261105/unresolved-review.json) and the contract review explain the exact claims, decisive artifacts, scope, counterinterpretations and revised-test assessment. No RAM-code contract or `0190 != 2` precondition was invented. These are **claim-local deferrals, not terminal exhaustion**.

All identified productive connected and independent opportunities in this frontier were followed before handoff. Next: independently review the combined batch in disassembly, then complete current-source commentary and redirect. No productive experiment is being interrupted. Broad gameplay already covers artificial finals, championship terminal and Start exit; unchanged replay offers no new discriminator.

[Identity checks](redirect/r2-contracts-20261105/handoff-identities.json) confirm the input/map/state and commentary hashes are unchanged. The attempt occupies about 11 MiB; 8.6 GiB was free at the last check. No toolkit issue, deletion, cleanup, archive or publishing occurred. Writer released after records and verification completed.
