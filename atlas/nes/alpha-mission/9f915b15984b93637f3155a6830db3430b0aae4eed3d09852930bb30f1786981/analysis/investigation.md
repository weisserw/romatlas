# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 disassembly — checkpoint disassembly-001-final

Input: initialized empty store. Output: [state](state.json), [map](../maps/current.json),
[checkpoint verification](disassembly-001-final-verification.json).

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

Code12,664; Data20,054; Text66; Image32,768;135 regions. Static structural claims,
actual accepted fetches, artificial observations and natural reachability remain
separate. No raw baseline/export gaps were counted as classified.

| Bank | Physical interval (hex, end excluded) | Code | Data | Text | Image | Unknown |
|---|---|---:|---:|---:|---:|---:|
| Header | 00000–00010 | 0 | 16 | 0 | 0 | 0 |
| PRG 0 | 00010–02010 | 6024 | 2102 | 66 | 0 | 0 |
| PRG 1 | 02010–04010 | 6121 | 2071 | 0 | 0 | 0 |
| PRG 2 | 04010–06010 | 519 | 7673 | 0 | 0 | 0 |
| PRG 3 | 06010–08010 | 0 | 8192 | 0 | 0 | 0 |
| CHR 0 | 08010–0A010 | 0 | 0 | 0 | 8192 | 0 |
| CHR 1 | 0A010–0C010 | 0 | 0 | 0 | 8192 | 0 |
| CHR 2 | 0C010–0E010 | 0 | 0 | 0 | 8192 | 0 |
| CHR 3 | 0E010–10010 | 0 | 0 | 0 | 8192 | 0 |

- D001-L01 verified: reset/NMI/IRQ roots and eight-entry state RTS directory;
  [vectors](disassembly-001/vectors.json), [initial listing](disassembly-001/static-listing.txt).
- D001-L02 verified: connected actor/boss/weapon/armor/collision/item dispatch batch;
  [listing](disassembly-001/all-dispatch-listing.txt), [item listing](disassembly-001/item-listing.txt).
  High/low target-minus-one semantics, bounded directories, meaningful object/stack
  contracts and consistent downstream direct flow justify code; pointer validity alone
  was not acceptance criterion. Natural selection of every entry is unresolved.
- D001-L03 verified: header/credits/CHR partition and PRG data complement;
  [tail review](disassembly-001/data-pattern-review.json),
  [residual review](disassembly-001/residual-data-review.json). References, nearby
  routines, finite lookup domains and obvious repeated records checked. Generic Data
  explicitly retains semantic uncertainty. Original generator/table organization not claimed.
- D001-L04 verified: broad progression/first boss/Octo armor. See
  [scenarios](scenarios.json), [gameplay](gameplay.md),
  [boss capture](disassembly-001/boss01.png), [armor capture](disassembly-001/octo-activation.png).
- D001-L05 blocked locally: final-base command0116 activates `$E8=6,$ED=1` but
  synthetic route lacks destruction-part initialization;240 additional firing frames
  preserve frozen state. [Initial state](disassembly-001/base-trigger-ram.json),
  [later state](disassembly-001/base-battle-ram.json). Redirect should derive `$8976`
  event-spawn and `$B0DD-$B152` part-handler prerequisites before another attempt.
  This does not block structural classification or justify unknown bytes. No ending claim.
- D001-L06 verified static helper structure: `$80D2`, `$82B9`, `$924D/$9265`, `$9578`.
  Coherent PPU/recorder/armor-expiry contracts and exact adjacent boundaries support
  code independently of a live caller. [Discovery](disassembly-001/isolated-routines-discovery.json)
  and residual-decode files preserve evidence. Full commentary should review these
  contracts; observed/reachable/unused claims remain unmade.

No failed classification lead was retried unchanged. The failed generic-input route
was corrected using actual NES API contract and preserved as historical evidence.
Toolkit issue NES-001 is a documentation/API validation bug with a working workaround;
see [persistent report](toolkit-issues.md). No toolkit modification.

Internal/export validation succeeded; generated assembly byte-identical, input hash
unchanged. Structural coverage complete; full source readability/editorial review and
terminal gameplay remain separate. Handoff is productive completion of one pass,
not terminal exhaustion of the workflow.
