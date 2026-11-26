# Round 4 — redirect handoff (attempt r04-a1)

**Recommendation: (a)** Run one short disassembly → commentary round on the ready leads L-021 and L-022, then
(c) final editorial acceptance.
- Both leads are low priority and trivial in size. They split, relabel or change the kind of already-classified
  Data. Code is unchanged and the count stays at 100%.
- They need a map edit plus a checkpoint. They may therefore be batched with final editorial acceptance, if that
  step is allowed to apply map/label edits and rerun a checkpoint.
- No other actionable avenue remains.

## Scope and checkpoint
- Input: pass-004 (maps/current.json, analysis/state.json).
- 100.00% classified: 262160/262160, 0 Unknown. The unknown complement is empty.
- Not modified: map, state, source, ROM and toolkit. Nothing was published or archived.
- Static work only: no serve or emulator session was used, and no processes are left running.
- Record: analysis/redirect/r04-a1/notes.md. Scripts and outputs are in the same directory: consumers, codeprobe,
  period, coursemap, srtail, regions.

## Findings
1. **No role-unknown block has a consumer or code evidence.**
   - Blocks checked:
     - bank 0: $A108-$A147, $A8CF-$A9AB and $ADBF-$AE7E
     - bank 2: $9420-$944B, $9C25-$A7E0 and $ACDF-$AFFF
     - bank 3: $B375-$BEFF
     - bank 4: $B348-$B369
     - bank 7: $D881-$D922
   - consumers.py resolved every operand, immediate-pair and data-word hit as a false positive. Examples:
     - bank 7 JSR $B9xx runs with bank 0 mapped (sound driver).
     - The bank 3 delay loop and the OAM immediates are not pointers.
     - $943F/$9445,X are the DR_Light tables.
     - The STA $9FFF writes are MMC1 register writes.
   - No branch from Code enters any block.
   - The longest clean official-opcode run is 13 instructions, and the blocks are full of BRK and unofficial
     opcodes (codeprobe.txt).
   - New independent non-code constraint for bank 3 $B375-$BEFF (period.txt): an address-aligned pattern of
     8 low-popcount bytes then 8 high-popcount bytes (about 1.1 vs 7.3 set bits). This looks like an uninitialised
     memory image and does not fit instruction streams.
2. **DR_CourseMap extent is proven: $944B-$954A, 256 B (new lead L-022).**
   - The pointer is fixed at $944B.
   - The only reads are `lda (MtileMapPtr),y` with an 8-bit Y, at Y and Y+$0F.
   - Independent check: the finish (course $0F00) gives Y=$F0, so the look-ahead column is exactly $FF.
   - $954B-$9560 (22 B) uses the same index values but is outside the consumer's window.
   - This resolves the round-4 "unproven extent" uncertainty. Records 76-103 of the metatile set are unchanged
     (the window's maximum index is 75).
3. **L-021 confirmed.**
   - Bank 7 $D881-$D922 is exactly MenuMetatiles records 41-49 plus 9 zero bytes. The Text region $D899-$D8A3
     is record 42's tiles plus record 43's attribute byte.
   - Bank 0 $A92E-$A94A belongs to the $FF-separated tile-number lists. It is binary data.
4. **Weak role hints (commentary notes only, no claim):**
   - bank 4 $B348-$B369 fits exactly 2 more 17-byte records.
   - bank 2 $9420-$943F matches the 16-byte pose-frame layout, but no pointer refers to it.
   - bank 0 $A108 is TitleMapTop plus extra indices; its set is not established.
   - Map nit: the bank 2 region $ACDF-$AFFF begins with the $80 terminator of the last SFX stream. The fill proper
     starts at $ACE0. An optional split is noted in L-021.

## Ready leads (analysis/leads.json)
| ID | Action | Gain |
|---|---|---|
| L-021 (low) | Change kind Text → Data for bank 7 $D899-$D8A3 (unused MenuMetatiles records) and bank 0 $A92E-$A94A (tile-number lists). Optional: split bank 2 at $ACE0 and add the commentary notes from finding 4. | 40 B kind fidelity |
| L-022 (low) | Split $944B-$9560 at $954B. Label $944B-$954A DR_CourseMap with Static evidence (conf 0.9), and keep $954B-$9560 generic Data. Fix the commentary "end is not proven" note. Run a checkpoint; the build should stay byte-identical. | Exact 256-B table extent |

## Deferred avenues (concrete reasons)
- **L-004, 7 unexecuted static roots:** this is a reachability question only.
  - No new reference was found (anchors.py in r03, consumers.py now).
  - No selector is known, and the inference has not changed, so a rerun would be an unchanged replay.
- **L-005 remaining roles** (bank 0 $A108/$A8CF lists, bank 2 $9420/$9C25, bank 3 $B375, bank 4 $B348):
  - No consumer was found by four independent reference scans.
  - No code-vs-data question remains: every block is non-code by BRK/unofficial-opcode density, the periodicity,
    or the record layouts.
  - Recovering their meaning would need a consumer that does not exist in the static code, and an artificial run
    cannot supply one.
- **DR_BackgroundMetatiles records 76-103:** these rest on stride evidence only.
  - No map indexes them: the window maximum is 75 and DR_BackgroundMap's maximum is 64.
  - They are already Data, so nothing further is needed.
- **S16, natural full Games run:** the reason is unchanged.
  - All progression milestones in scenarios.json are visited except S16, which is deferred with a reason.
  - No classification question depends on it.
- **Runtime read-watch of $954B-$9560:** optional. The 8-bit indirect-indexed proof is decisive.

## Rejection review (compact)
- **"Role-unknown blocks might hide code."** Rejected.
  - Decisive evidence: consumers.txt (no branch, call or operand from Code, every hit resolved), codeprobe.txt (no
    clean run longer than 13 instructions; BRK and unofficial opcodes throughout) and period.txt.
  - Scope: this rules out code for these exact ranges. It does not establish their meanings.
  - Strongest competing interpretation: stale leftover code from an older build. This is contradicted by the
    decode statistics and, for bank 3, by the address-aligned bit pattern.
- **"DR_CourseMap end cannot be proven"** (round-4 disassembly left it unmerged). Overturned by newly combined
  constraints:
  - the fixed pointer
  - the 8-bit (ind),Y addressing
  - the finish look-ahead index $FF

  These give the exact 256-B extent (L-022). The 22-B tail stays generic Data. No claim is made about why it exists.
- **"Unused metatile records / map variants have specific roles."** Not claimed.
  - Only one constraint exists for each (stride, or similarity to another map). No consumer exists.
  - They stay generic Data with explicit uncertainty notes. This is sufficient under evidence-policy.md.
- **"100% + clean audits ⇒ ready now."** Narrowly true for code/data, but there are two small role corrections
  (L-021, L-022). These are worth applying before or within final acceptance.

Toolkit: no new issues found (analysis/toolkit-issues.md unchanged).
