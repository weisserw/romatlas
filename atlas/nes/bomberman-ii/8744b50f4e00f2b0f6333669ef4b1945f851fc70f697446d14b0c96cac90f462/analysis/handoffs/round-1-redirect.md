# Round 1 redirect handoff

Scope: redirect round 1 on checkpoint pass-001 (maps/current.json == maps/pass-001.json; ROM, map and analysis/state.json
hashes unchanged, analysis/redirect/r1-a1/input-hashes.txt). Classification unchanged: 99.98% (131057/131088; 31 unknown).
Attempt dir: analysis/redirect/r1-a1/ (findings.json, unknown-ledger.json, static-executed.json, read/dmc/cov traces, scripts).
Leads: analysis/leads.json (pass entry "redirect r1-a1"). Scenarios: analysis/scenarios.json. Gameplay notes: analysis/gameplay.md.

## Findings (details and evidence: analysis/redirect/r1-a1/findings.json)
- F1-F5 / L-001: all 31 unknown bytes resolved to a role. Bank-5 $9508 and $A0B1 = 1-piece metasprites (cursor sprites);
  $B8E9 = two 0-terminated metatile lists (6+7 bytes, DrawVsMatchPoint); $B9C7 = text "SC". Consumer reads observed for all
  13+5+5+2 bytes (natural, except $B8E9 needing a WinCount poke). Bank-6 $B027 (6 bytes) = non-code residue after the 255-tile
  ChrFontMenu chain (font load reads stop at $B026), before the $FF fill -> generic data.
- F6 / L-008: BossActionTable $9664 (bank 0) is a 13-entry jmp (PpuDest) dispatch table (bound cmp #$0D) missing from
  tables_spec. Entries 6/7 point to $97A2/$97A4 = `INC $1C` x2 falling into BossActionMove: 4 bytes currently classified as the
  tail of BossBounceOffsets data are code (data ends at $97A1). Entry 12 references BossActionBlink $97EF (commentary says
  "unreferenced"; it runs naturally in the power-on opening).
- F7 / L-004: fixed $E000-$FF67 is DMC (DPCM) sample data addressed by DmcSampleTable (bank 2 $8EAC, 13 records
  rate/level/addr/len -> $4010-$4013). DMC DMA reads observed naturally (title music + sound-room PCM 1-12) cover exactly the
  union of the records: $E000-$EBA0, $EBC0-$EE70, $EE80-$F070, $F080-$F580, $F5C0-$FF60; 5 unread alignment gaps remain generic.
- F8 / L-010: StageTimeTable (96 B) and StageItemChoices (144 B) extents proven by World 0-5 x Stage 0-7 domains; $815B is a
  separate ItemChrIndex table (13 B, read by LoadStageItemChr); VS (World 6) reads record 48 = $815B-$815D (observed).
  Read events at $8167 are RTS dummy reads (verified by cycle, dummyread-8167.txt; reported as TK-005).
- F9 / L-009: 120 Static code regions (6279 B) executed in natural windows (power-on, attract demo, password bonus stage),
  26 more (1305 B) only in artificial windows. No executed opcode in data/unknown anywhere.
- F10: opening cutscene ($B186, scripted kinds 18-21, 23-26) runs naturally at power-on; kind 22 has no known spawner (L-011).
- Gameplay (L-005 verified): sound room (password K3456712), bonus stage (KJH5C13D) - both natural; game over (Lives poke),
  battle arena (GameMode=2 write; Four Score required, emulator has 2 ports), VS match point (WinCount poke), ending -> STAFF
  credits -> END (artificial ending entry), attract demo (natural). Bosses: none in normal-mode stage enemy lists.
  All special passwords decoded (gameplay.md).

## Ready leads for the next disassembly pass (recipes in analysis/leads.json)
- L-001 (high): annotate the 5 unknown ranges (4 specific data roles + 1 generic) -> 100% classified. 31 bytes.
- L-008 (high): add BossActionTable to tables_spec; split data 0x1793+35 -> Data 0x1793+31 + Code 0x17B2+4 ([0,1,14,15]).
- L-004 (medium): relabel/split 0x1E010+8040 into Observed DMC sample intervals + 5 Static gaps (role, not byte gain).
- L-009 (medium): provenance Static->Observed for executed ranges (replay recorded windows; flag artificial ones).
- L-010 (low): split 0x140DB+157 into StageItemChoices 144 + ItemChrIndex 13; StageTimeTable extent note.
- L-007 (commentary): DMC sample map, password names, BossActionTable entry names, BossActionBlink comment fix.

## Rejected / blocked / deferred
- L-002 rejected (scope): no code target into data from classified code; short decodes in generic data are referenced only by
  metasprite/sound pointer tables. Reopen only with a concrete reference.
- L-003 verified: all JMP(ind)/trampoline sites enumerated; only BossActionTable was missing.
- L-006 blocked: $C149,$C21B,$C8DA,$C92A,$A94A have no reference in any bank (missing fact: a caller); $97EF resolved.
- L-011 blocked: kind 22 spawner unknown; not executed from power-on to frame 3428.
- Deferred: enemy kinds 12-14 behaviour and sound byte-code command meanings - commentary semantics only, no structural
  dependency (route if wanted: truck snapshot, poke $4B=3/$4B=4, $4C=0, w650). Remote/fuse passwords not run (static effects clear).
  HiddenP2WorldSelect not executed artificially (would prove nothing about reachability).

## Rejection review (compact; not a terminal proposal)
- L-002: claim "short routines hide in generic gaps"; decisive evidence = codetargets.py (0 code->data targets) and islands2.py
  (candidates only referenced from pointer tables whose other entries point to metasprite/sound data). Rules out referenced
  short routines; does not rule out unreferenced code (unprovable without a caller). Competing interpretation (data) fits all
  constraints. No revised test.
- L-006/L-011: missing caller/producer; ROM word search over all banks and script parsing done. Reachability not claimed dead.
- F5 ($B027): alternative "code" rejected: BCC into font data then unofficial opcodes, no reference, bounded by proven chain end
  and $FF fill; specific "256th tile" role not claimed.

## Recommendation
(a) Another disassembly -> commentary -> redirect round: L-001 completes classification (31 bytes), L-008 fixes a
data/code error, L-004/L-009/L-010 improve roles/provenance; commentary then needs the updated names.
Blockers: none. Toolkit: TK-005 added (dummy reads not flagged; state.pc transient during mid_instruction). Server stopped;
scratch pruned (41 MB; cited snapshots soundroom/bonus/battle/gameover0/ending700 kept).
