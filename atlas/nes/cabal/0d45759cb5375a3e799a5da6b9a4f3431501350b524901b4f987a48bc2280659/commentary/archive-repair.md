# Archive repair

The archive rejected changed non-assembly asset `mapping.json`. Inspection found that
`commentary/source/mapping.json` retained the round002 inventory: it lacked only
region 9630 (physical [6428,6430), CPU $990C in PRG context [0,1,2,3]) and had
next_id 9630 rather than 9631. All 9628 shared region records were identical.
The accepted round003 frozen baseline already classifies that cursor prefix as Code;
annotated assembly and final review already describe it. This was stale build-tree
metadata, not a new classification decision.

Reconciled only that metadata file with the existing frozen `before/mapping.json`.
No assembly, labels, comments, include paths, review credit, conclusions, evidence,
ROM bytes, current maps/state or frozen baselines were edited or regenerated.
The original source-side metadata and archive options are preserved locally under
`analysis/archive-repair-backups/`. Archive options now include this final report;
all existing final/history selections and six historical scaffold omissions remain.

Verification: the supported `commentary.py audit` returned exit 0, ok=true,
byte_identical=true and editorial_audit=true, with 9629 reviewed regions and 9560
embedded unit references. Annotated assembly hashes still match final acceptance:
`game.asm` b5b1a0eff39a02bfa2d5be83fd7e5244e019d87f6b88395629d4363b0cd39303;
`symbols.inc` 2c3b8d14704e9ffc58b573f2d722f301ba7e6232065a54145cdd1d6744231606.
The reconciled metadata equals the frozen map/current accepted map byte for byte
(SHA256 6568e1188ec98ee68f3c5110286d1b938688e9eaa3267e31238272aacf40557a).
The affected source range and immediate advance/compare continuation were inspected;
they already agree with region 9630, so no source or review-location edit was needed.

The requested archive probe targets only
`/tmp/romatlas-archive-probe-3r19vk9e/archive`, using the configured release binary
and every current archive-options document/omission argument. Its returned status
is retained locally in `analysis/archive-repair-probe.json`. The archive contract
requires fresh source verification, ROM/map/state validation, regeneration of a
separate temporary baseline, exact reconstruction of the final tree, and a fresh
byte-identical rebuild of that restored tree before publishing the disposable probe.
The real atlas destination is left to the independent runner. Historical gameplay/semantic/hardware limitations
in the terminal handoff and uncertainty documents remain applicable. In particular,
natural later combat/boss/ending remains unresolved and artificial terminal evidence
is not natural victory. No new research or new semantic review credit is claimed.
