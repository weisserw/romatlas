# Round 3 — redirect handoff (COMPLETE) — recommendation: TERMINAL

- Attempt directory: analysis/redirect/r03-a/ (static: sweep.py/.log, sweep2.py/.log, covcheck.py/.log, covdiff.py/.log;
  runtime: exp_ce.py, exp_cf.py, exp_cfjack.py; evidence/ logs, PNGs, cov-ce/cov-cfjack.json.gz, after-ce.snap).
- Input checkpoint: pass-004 (maps/current.json == analysis/state.json, sha256 119004fa...d55690) and commentary-003
  (commentary/source/game.asm sha256 3cdcb947...7131). No map, state, source, ROM or toolkit edits. Backups:
  analysis/leads.pre-redirect-r3.json, analysis/redirect/r03-a/scenarios.pre-r03.json.
- Classification: 100.00% (262160/262160; 0 unknown). Proposed bytes: none. Accepted this round: n/a.

## 1. Boundary sanity sweep (Static, read-only) — no contradiction
- sweep.log: all 808 Code regions decode linearly with only official opcodes; no instruction overruns into a non-code
  region. 1685 branches + 602 JSR + 303 JMP abs: 2567 land on Code instruction starts; 15 are `jmp StreamJmpOpcode`
  ($04AA RAM trampoline, documented); 8 bank-7 far-call edges looked wrong only under the nominal view context.
- sweep2.log: resolving those fixed-bank calls with the preceding `lda #n / jsr SetPRGBank16K` gives 11/11 Code
  instruction starts in the selected bank; the 2 without in-region select ($CA7D after the $CA6F select in the previous
  region -> bank 2 $BEEE, same target as $CA33; $FFF7 reset stub -> bank 0 $8000) are also consistent.
- covcheck.log: all ROM fetches in the 7 prior coverage exports (10125+...+4353 entries) are Code instruction starts;
  covdiff.log: the 2 new r03 exports (274 newly fetched addresses) likewise, all in Static-provenance code; none in Data/Padding.
- Low-confidence items: b0 $9AB8 (0x1AC8) and b1 $85E0 (0x45F0) still never fetched; nothing branches/jumps into
  b0 $BA42-$BA43 or any other data byte. No evidence changes L-023/L-018b/L-022 conclusions.

## 2. Natural ending trigger (RAM contract derived from commentary-003 source)
- Contract: UpdateEndingTrigger_CE (BoyRoom $CE, BlobForm KEY with CheckFormTimerDone, BoyHolding, BoyX>=$C7, BoyFlags 0
  -> BoyRoom $FE, BlobRoom=SpecialRoom $CF, BlobX $8D) then JackUpdate_Special (BlobRoom==SpecialRoom, FormCounter>=$76
  -> EndingCountdown=1).
- E1 (exp_ce.py, ce.log): teleport to $CE only; key bean thrown/eaten/picked up/carried by game play -> transition reached;
  boy scripted through $FE into $CF. New milestone S14 visited (artificial start).
- E2 (exp_cf.py, cf.log; run hit its wall-clock limit after V2 t13, coverage not exported): boy blocked at X=$33; thrown
  jack bean sets BlobForm=5 but the caged blob never eats it (with or without a B call first). Same barrier as r02 S13.
- E3 (exp_cfjack.py, cfjack.log; try 1 left BlobFormStep=1 -> eat counted down, cfjack-try1.log): artificial eat-state
  injection ($86=5, $83=0, $81=$60). Game code then applies JACK, raises it in the cage, JackUpdate_Special starts
  EndingCountdown, collapse sets SpecialRoom $60, rooms $FF, EndingTimer, ending scene ($FD, GameOverState $F8).
  S13 -> visited (artificial). The natural way to deliver the jack bean into the cage remains unknown.

## Leads
- Added: none. Updated: L-006, L-008 (legacy 'ready' -> deferred: verification/reachability only, no unknown bytes).
  No ready leads.

## Compact rejection review (terminal)
| Candidate claim | Decisive evidence | What it rules out | Strongest competitor / remaining reason |
|---|---|---|---|
| b0 $9AB8 = Code (orphan RTS), conf 0.6 | r02-a l023-review.md, rtsrefs.log; r03 sweep/covcheck | consumer, branch target, fetch: none found again | unread $60 datum; no static/runtime method can observe intent for an unreferenced unread byte (accepted pass-004 with explicit limits) |
| b0 $BA42-$BA43 generic data (0.7) | pass-003 L-018b record; r03 sweep (no edge into it) | code entry into it | code fragment requires entry into ORA ($FF,X) running into another routine; generic data needs no specific role |
| b1 $85E0 AddAToZpWord Code (0.75) | r02-a l022_check.log; r03 covdiff (not fetched) | references of any form | data with code-like bytes; structure (CLC/ADC/BCC/INC/RTS contract) independent of caller; only reachability open |
| Natural ending trigger | r03 E1-E3 | $CE contract and jack->ending mechanics now observed | bean delivery into the cage unknown; affects provenance of already-classified Static code only (0 unknown bytes) |
| Semantic uncertainties (commentary/uncertainties.md) | commentary-003 audit | — | meanings only; documented mechanically with explicit uncertainty; no bytes depend |
Independent constraints were combined for each (consumer exclusion, control flow, layout idiom, coverage). No revised
inference or discriminating test survives that could change a classification or boundary.

## Gates
- Classification: 100.00% (262160/262160) — maps/current.json, analysis/state.json (pass-004).
- Byte-identical rebuild: analysis/pass-004-verification.json (ok, byte_identical) and commentary/audit.json.
- Commentary final editorial audit: commentary/audit.json (ok, byte_identical, 1365 regions / 1453 units, editorial_audit true);
  copies analysis/commentary-003-audit.json, analysis/commentary-003-verification.json.
- All three gates satisfied at commentary-003; this redirect changed no map/source.

## Optional (non-blocking) notes
- Commentary '-- static decode only' markers in UpdateEndingTrigger_CE / JackUpdate_Special / UpdateBlobEating could cite
  the artificial r03 observations; provenance only, not a reason for another cycle.

## Recommendation
TERMINAL: classification, editorial audit and byte-identical gates satisfied; no evidence-backed ready lead.
Disk ~3.5 GB free; r03-a ~5.2 MB (after-ce.snap 5 MB kept as reusable route state); scratch removed.
