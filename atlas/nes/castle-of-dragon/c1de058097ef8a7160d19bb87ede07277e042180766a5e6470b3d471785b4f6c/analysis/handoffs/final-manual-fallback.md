# Final record: user-directed manual Data fallback (2026-11-24)

**Authority.** Explicit user instruction for this batch run: when at most 10 physical ROM bytes
remain unclassified after the terminal incomplete handoff, mark them as generic Data with unknown
meaning using Manual provenance. The instruction overrides workspace rules requiring further
structural proof, for these residual bytes only. This is a user-directed fallback. It is
**not** an observed or statically proven data role.

**Exact range (verified before the edit).** After `map.load analysis/state.json`,
`analysis.unknown` returned exactly one range: physical offset 115749 (0x1C425), length 1.
That is CPU $C415 in fixed bank 7. It matches the user's validated list (1 byte, which is 10 or fewer).

**Prior ambiguity (preserved).** The byte is a single $60 that sits after ClearSpritesAndObjects'
RTS ($C414) and before InitFramePool ($C416). No absolute, split, RTS-trick, branch, indirect or
dispatch reference to it was found, and no trace fetched it. It is not part of any fill or alignment
run. It could be an orphan RTS (dead code) or a stray/padding byte, and the ROM evidence does not
decide between them. Lead L-001 was blocked in rounds 1-3. The round-1/2/3 redirect rejection reviews
are in analysis/handoffs/round-{1,2,3}-redirect.md. The coordinator accepted the review in
analysis/handoffs/final-coordinator.md.

**Action.** `map.annotate` created region 1245 with offset 115749, length 1, kind Data, provenance
Manual and confidence 1.0. It has no label. Its evidence and comment text say this is a user-directed
fallback with unknown meaning. After the edit, map.validate returned ok and analysis.unknown returned
total 0. The state was then saved with map.save and exported with map.export. The pre-fallback state is
kept at analysis/evidence/final-fallback/state-pass-003-before-fallback.json. The RPC log is
analysis/evidence/final-fallback/apply-log.txt. ROM bytes were not modified (sha256 c1de0580... unchanged).

**Checkpoints.** pass-005 is the fallback checkpoint: byte-identical, map sha256 1c95fbac...
(analysis/pass-005-verification.json). pass-004 is a no-op checkpoint. It was created after a
rejected first annotate call and is identical to pass-003. It is retained but not cited.

**Separate accounting.**
- Evidence-backed classification (Observed/Static): 131087 / 131088 bytes (99.9992%).
- Manual user-directed fallback (generic Data, meaning unknown, no proven role): 1 byte, $C415 / 0x1C425.
- Map coverage: 131088 / 131088 classified, 0 unknown. This does **not** mean every byte has
  evidence-backed classification. goal.md's whole-ROM "complete" criterion (every byte evidence-backed)
  is not claimed.

**Commentary.** The round-4 commentary was re-prepared from pass-005, and the round-3 commentary is
preserved at analysis/round-3/commentary/. The final editorial audit passed
(commentary/audit.json: byte_identical, 752 regions, 855 units, editorial_audit true). See
analysis/handoffs/round-4-commentary.md and analysis/handoffs/final-editorial-audit.md.
