# Round4 final dependency/coverage frontier

All planned semantic batches through fixed-other are applied/running verification. Remaining work is concrete source/metadata refinement, not a new phase:
- Correct generic four-byte scalar suffix1D503; actual response arrays are earlier1D4FB/1D4FF and the new suffix has no identified consumer.
- Correct bank5 FFAA as positive-extension adder (not collision-probed/signed extension), FFB3 as X-only retirement (not movement), and preserve exact old/current physical aliases.
- Correct A4BB routine's actual inline callsite to A4BE; correct older CHR0 BFFF first-write comment/alias usage and NMI busy epilogue prose.
- Normalize CPU targets with explicit bank contexts; keep physical Data inclusion labels separate from CPU data aliases; preserve width hints.
- Reconcile all18,668 scaffold records and discovered units: fresh source locations, sound prior credit, no pending/deferred current ranges; retain editorial_audit pending.
- Review source numeric operands/stale claims/duplicate labels and concrete descriptions, check map/state/ROM/frozen-baseline identities.
- Fresh byte-identical verify, refreshed reading copy, scope report and compact commentary handoff. No final editorial audit.
R4-C01 remains queued for redirect. Source notes retain index1 icon pointerEC2E overlap honestly; no structural/map edits.
