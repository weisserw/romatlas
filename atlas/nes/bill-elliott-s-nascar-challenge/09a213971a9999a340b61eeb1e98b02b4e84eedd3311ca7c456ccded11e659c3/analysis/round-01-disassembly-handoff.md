# Round 01 disassembly handoff

Pass complete; ROM incomplete. ROM classification: 69.13% (271832 / 393232 bytes classified; 121400 bytes unknown).

- Code6034; Data/Image265798; unknown121400. All333 regions evidence annotated; per-bank counts [counts.json](round-01/counts.json), exact residuals [unknown.json](round-01/unknown.json). CHR entirely image; residuals all PRG.
- [pass-001 verification](pass-001-verification.json): validation successful, byte-identical=true, original ROM SHA matches workspace. Map/state SHA256 `53b31ef28a2c0d0397d16245d8813a40f28cc4a9c6478c1fdd4f24fafb3e193a`. Frozen maps/pass-001.json; generated/pass-001/game.asm. No commentary yet.
- Initial empty lead list; verified vector roots, maximal uniform data slices, bounded menu RTS dispatch and supported direct-call entries. Sampled natural opcode starts2285, artificial1254, artificial-only0; these are separate from structural classification.
- Reproducible natural routes visit menus,10-mile Daytona overview/setup/cockpit, Sears Point and practice setup. RAM0557/0519=3 initializes Watkins via ordinary UI, explicitly artificial. scenarios.json/round-01/route.py preserve presses/releases/budgets/effective banks and captures.
- Prerace remains after bounded600+1200 no-input frames; latest frontier round-01/prerace-later.snapshot. Banks8,9,14,15; RAMBF5/B8=14, NMI pointer19/1A=BFA4. Holding-A-only explanation rejected by release experiment. Need exact handler/state transition before pit/results/championship/ending shortcuts; no arbitrary RAM/PC brute force. Clean natural menu frontier main2.snapshot.
- Related useful lead: seven menu handler return-minus-one words at physical4AC1..4ACE, selector9D domain0..6, consumer/selector proofs saved; callee contracts/continuations remain available for redirect. Full source commentary is the next phase.
- [Toolkit issues](toolkit-issues.md): one suspected NES input-schema/documentation issue (buttons-array workaround); no confirmed or blocking bug. Configured binary unchanged. Input unchanged; all assets/state local, no publishing. Disk~4GB free; scratch12KB.

This is the ordinary easy-gain handoff, not terminal exhaustion. See investigation.md for attempted/rejected/superseded lead scopes; do not repeat failed input schema or long prerace waits without a changed contract.
