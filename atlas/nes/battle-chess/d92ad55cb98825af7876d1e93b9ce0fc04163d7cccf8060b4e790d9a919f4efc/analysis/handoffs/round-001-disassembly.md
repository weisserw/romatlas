# Round001 disassembly handoff

One productive bounded easy-gain pass completed; **ROM incomplete**, ready for full current-source commentary then redirect. No other phase prompt read/executed.

ROM classification: 71.17% (186573 / 262160 bytes classified; 75587 bytes unknown).

Code4899 (2689natural observed+959Static+1251later observed artificial), Image139264, Data42410. Header16 counted; no gaps/raw fallback counted.1144regions, all evidence-bearing. Per-bank counts: [JSON](../evidence/pass-001/bank-counts.json), tables in [progress](../progress.md) and [investigation](../investigation.md). Exact residuals: [unknown complement](../evidence/pass-001/unknown-complement.json).

Exact checkpoint: `pass-001-final`, map `maps/pass-001-final.json`, SHA256`77d79b7545def6fc5d9738059f783b53336ee32af10757ce39ae0a1e8d84ee59`. [Verification](../pass-001-final-verification.json) confirms byte-identical generated/pass-001-final assembly. Internal state analysis/state.json and portable maps/current.json saved/validated; original input SHA remainsd92ad55cb98825af7876d1e93b9ce0fc04163d7cccf8060b4e790d9a919f4efc. Earlier baseline/pass-001-initial/pass-001-structural checkpoints preserved. Readability is commentary's next phase, not asserted complete.

Lead outcomes ([ledger](../leads.json)):
- R001-L01 verified: accepted reset/startup/board/menu instructions plus supported ordinary static branches; exact bank-aware decodes in observed-instructions.json/static-candidates.json/later-instructions.json. Conservative mapper/call barriers retained. Static does not imply natural execution.
- R001-L05 verified: physical bank2-18 planar raster slices, bank19graphics/nibble transition, bank20-23smooth nibble streams; exact uniformFFruns in mixedbanks; validated header/vectors. Whole asset/table extents and nibble semantics unresolved; generic noncode counted without inventing meanings. Supporting contact sheets, boundary/distribution summaries, code conflict checks retained.
- R001-L02 attempted/open: Battle Chess (DataEast1990) initial3Dboard, options and2DSetup editor visited. Practical near-endgame patch visibly accepted **in editor**, not game. Arrays RAM$04BE/$04FE each64: zeros exceptindex0=$86,17=$05,18=$06 (bluekinga8/redqueenb6/redkingc6). Built-in Setup→Done acceptance remains unresolved; rapid bounded presses during redraw/delay can be ignored. Capture/AI/checkmate/ending not visited. Do not repeat unchanged tap sequence; redirect should inspect ready/cursor/Done handler first, then use supported transition for dependent initialization andQb6-b7mate/capture experiment.
- R001-L03/L04open: bank0engine and bank28setup ordinary calling contracts, mixed scripts/pointers in banks1/24-29 and fixed-bank gaps. Many bank0absolute$04BE accesses provide useful consumer anchors. Needs consumer/return/bank inspection; wholesale unknown decoding not justified. No terminal rejection review warranted for this nonterminal handoff.

Reproducibility: [scenarios](../scenarios.json) has exact requested presses/releases/budgets, actual timeout/budget stops, milestone status, patches and evidence. [gameplay](../gameplay.md) cites NESmanual/FAQ source hypotheses. Good resume snapshots under evidence/pass-001:
- options.snapshot.json frame647, natural,slots[0,1,30,31].
- setup.snapshot.json frame875, artificial only from known countdown RAM$0030=0 (CPU$C3B7-$C3CC contract), editor with initialboard.
- near-mate-done.snapshot.json frame1200, artificial, confirmed near-mateboard editor; preferred continuation snapshot.
- near-mate-game.snapshot.json finalframe1485,artificial,CPU$D063,slots[0,1,30,31], remains Setup submenu despite filename; no game/terminal claim.
Import requires new emulator.start; reassert buttons after restore. Named snapshots are not assumed persistent. Final roots/state/summary saved; RPC server terminated. No trace streams; capacity0drops detail intentionally, aggregate_dropped=0;2795unique instruction starts,2912contexts,3181contextedges. Final one new root after final map batch is saved but unclassified pending inspection; no forced gain.

No toolkit issues found; [persistent report](../toolkit-issues.md). No toolkit edits, publishing, download of ROM/assets, archive or destructive workspace actions. Disk remains healthy; scratch reusable and small. Preserve all cited evidence and historical checkpoints.
