# Round 1 redirect handoff (attempt r01-a) - COMPLETE

- Input checkpoint: pass-001 (maps/current.json = maps/pass-001.json, sha256 b253e7de...ee41),
  commentary round 1 complete. Map, ROM, source and binary unchanged by this phase.
- Research directory: analysis/redirect/r01-a/ (tools/, frames/, snapshots/, coverage.json.gz,
  l001-fragments.json, refsearch.txt, unknown-ledger.json, leads-before.json, scenarios-before.json).

## Scope
1. Code-vs-data decision for all 152 unknown bytes (L001) under evidence-policy.md.
2. Commentary questions: 3:$9E4A MulB3_Image (Data), lone RTS bytes incl. 3:$91B9 (Data).
3. Broad progression (L003): mission completion, space battle, docking, shop, final mission, ending.
4. L002 RAM-executed sources, L004 bank-2 coverage, controller-2 debug inputs.

## Findings
- Unknown complement (unknown-ledger.json): 10 ranges / 152 bytes, all between classified Code
  (3:$A067 is followed by Data). Every range decodes with official opcodes, ends exactly at the
  next region start, and all branch/jump targets are internal or established instruction starts
  (tools/fragcheck.py). Reference search (LE words, word-1 RTS dispatch, lda/ldx #lo/#hi pairs,
  refsearch.txt): only operand coincidences in code or numeric tables (e.g. 0:$C8CB sbc table);
  no entry. No fragment fetched in any round-1 or r01-a trace.
- Strong independent constraints for 150 bytes (L006): 3:$9DBD is byte-identical to code at
  3:$8901 and 3:$9D03; 0:$9A1A is bypassed by "lda SpacePhase / bne $9A54 / jmp $9A54" (both
  outcomes skip exactly the block whose exit is $9A54); 0:$B091 is skipped exactly by StarRespawn's
  "jmp $B093" and falls into sta-sequence; 3:$A067 is a complete stack-balanced routine (push X/Y,
  bullet loop, two pop/RTS exits) mirroring CheckBulletHits; 0:$9843 jmp to GameOverFadeStart;
  0:$98C6 / 2:$A1EB branch to established entries; 0:$9E30 is "stx FramePhase" (same encoding
  as Flight_PhaseStep) falling into $9E32.
- 0:$844C, 0:$E0B7 (unknown) and 3:$91B9 (Data) are each a single $60 right after a routine-ending
  JMP: a repeated authoring pattern but weak evidence (policy: lone return opcode) -> L007, low.
- 3:$9E4A-$9E8D (68 bytes, Data "MulB3_Image") is byte-identical to Code 0:$D518 (MulSigned) and
  2:$84E9, relative branches only, between two RTS-ended routines -> L008 (Data->Code).
- L002: RAM PCs observed (round 1 + r01-a) are exactly $0700-$0763 and $079B-$07FF, i.e. only the
  two known copied images; the "$06F0" premise is absent from coverage. Blocked on toolkit views.
- L003 progression (all via game code after documented RAM patches, see analysis/scenarios.json
  S07-S10): MissionStage $066B := MissionObjectiveStage[Mission] (0:$F9C0) sets Landing=2 and allows
  climb-out (PlayerY<$14 -> code $88) -> space battle -> SpaceWave $066E:=$40 or controller-2
  B+Up -> results -> docking -> PlayerX $3B=$67/PlayerY $3D=$3F + B -> shop (buy Phoenix, NEXT
  MISSION) -> departure -> mission-2 briefing. Mission 8: base attack; $066B:=$40 -> base explosion
  -> CONGRATULATIONS / FLEET DESTROYED -> GAME OVER -> ENTER YOUR NAME. CONTINUE? screen also seen.
- L004: r01-a traces added 868 bank-2 (1126/212/270 banks 0/1/3) instruction starts; all inside
  classified Code (0 conflicts). No classification change.
- Controller-2 B+Up confirmed (PauseState=2 in space battle); B+Down showed no effect in 30 frames.

## Leads (analysis/leads.json; pass record added)
- L006 ready/high: 8 ranges, 150 bytes -> Code (Static, no known entry). Ranges/bytes:
  0:$9843-$9845 4c4d84; 0:$98C6-$98CC a546f0034c139a; 0:$9A1A-$9A53 (58 bytes a50b29fc...8c7d05);
  0:$9E30-$9E31 862a; 0:$B091-$B092 a900; 2:$A1EB-$A1F0 c9ff9002a200;
  3:$9DBD-$9DC9 857ea9398526a9f3852720b898; 3:$A067-$A0A1 (59 bytes 8a489848...9d380560).
- L007 ready/low: lone RTS 0:$844C, 0:$E0B7 (+ consistency with Data 3:$91B9); disassembly to choose
  code (confidence ~0.6) or retain unknown with this disposition.
- L008 ready/medium: 3:$9E4A-$9E8D Data -> Code (MulSigned copy), no unknown-byte change.
- Updated: L001 superseded by L006/L007; L002 blocked (toolkit); L003 verified; L004 verified.
- Unchanged: L005 (data semantics, not classification-blocking).

## Recommendation
Another disassembly pass is warranted: apply L006 (+L008, decide L007). Expected 150-152 unknown
bytes resolved (99.88% -> 99.998-100%). Then commentary (rename UnknownB*/MulB3_Image labels,
add progression facts above) and a short redirect/final audit. No terminal rejection review needed
(ready leads exist).

## Deferred avenues (reasons)
- Data with no consumer (1:$D649=3:$D7B8, 3:$D76E, 3:$D90A, 2:$FE65, 3:$F1C3, 1:$FE16..): already
  classified Data; only semantics missing; no code evidence, not classification-relevant.
- Overlapping RLE streams (bank-1 $BA, bank-0 $CA01/$CAF8): semantics only; would need decoder
  count proof; no effect on classification.
- L002 RAM views: toolkit lacks uploaded-execution views (missing feature, not a ROM question).
- Natural (unpatched) completion of missions: patches suffice for coverage; manual play is slow
  and yields no new classification evidence (all observed code already classified).
- Controller-2 B+Down ($44) ground effect: no visible effect in 30 frames; low value.

## Toolkit
No new toolkit issues. Note: tracing (capacity 0) slows stepping to ~35 s per 100 frames in
docking/results states; use untraced runs for navigation. TK-001/TK-002 unchanged.
