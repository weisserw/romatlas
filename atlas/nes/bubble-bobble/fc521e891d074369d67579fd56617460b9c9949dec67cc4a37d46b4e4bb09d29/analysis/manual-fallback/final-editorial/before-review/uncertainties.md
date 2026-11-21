# Current-source meanings and conditional contracts

All13507 classified instructions,363 Data regions and CHR have source explanations.
Four Manual fallback Data intervals (9 bytes) have exact local notes; see analysis/manual-fallback/handoff.md.
All163856 bytes are classified including fallback;163847 have evidence-backed classification.
No current-source readability range remains pending. Prior uncertainty records are
preserved under analysis/round-6/commentary/uncertainties.md.

- Four user-authorized generic Data fragments retain unresolved meaning and code/data ambiguity: physical101105..101108
  candidate JMP8B0E;101150..101152 candidate LDA0;101940..101941 isolated RTS;
  103733..103736 isolated JMP9525.
  Bounds of the linked JMP8B0E/LDA0 pair are independently anchored. Their possible
  whole-field clear differs from accepted type7-gated setters preserving low bits;
  the encoded link alone does not establish that role over literal data. The isolated
  return and veneer likewise lack a discriminated role. No universal caller requirement
  is imposed; see analysis/coordinator-rejection-review.md.
- Reset E902 now has a concrete safe return contract: E8DC/E8DE set SP=FF;
  E8FF saves its return at01FE/01FF. C35D/C1D5 clears and task0 seeds012E/012F
  do not overwrite that frame or the inner01FC/01FD frame. This establishes the
  supported producer, not arbitrary incoming-stack safety.
- FA81 restores the incoming lower pair only with complete MMC1 latch, mode3 and
  remembered04A3 matching that pair's register. F76D selects sound register4 without
  changing04A3; F777 reloads it. No global actual-shadow invariant is asserted.
- CEE3 saves caller/status/X/Y on a task stack. CE8C eventually restores that stack
  and resumes it by RTS. Eventual scheduling and preserved effective bank context
  remain preconditions. Five encoded task words do not establish a sixth entry.
- F22D absolute LDA F239 reloads constantF4 and fills0200..020F; it is not indexed
  template copying. Remaining template field identities and a natural caller remain
  unresolved. Stable controller/mask/sound siblings have static mechanical support,
  without proving actual selection of every variant.
- D3A7/DD3E/DD4E/DE99 have distinct ordinary and frame-removing alternatives.
  Bank10 boss887B BNE bypasses collision when6D is nonzero. These conditions and
  static actor/OAM entry contracts remain explicit in source.
- Exact actor, cue/instrument, reward, tile/text and generic-data subdivisions stay
  unresolved where consumers establish mechanics only. Shared scratch RAM aliases
  describe context; unresolved CPU aliases do not imply a bank-independent target.
- Reset4022..408A writes retain literal behavior without a peripheral interpretation
  or full hardware/game compatibility claim.

Physical115187..115189 is now generic Data, without a semantic label. Independent
preceding JSR/RTS and palette C1E5 consumer extent discriminate this interval: CE
requires three bytes and FA a mandatory successor in accepted palette data. Hardware
overlap remains possible but unsupported. Apparent FACE pointer meaning remains
unproven; see analysis/round5/review.md and91 fresh decodes.

Primary independent structure: analysis/round5/review.md and analysis/round4/review.md; earlier conditional
contracts remain in analysis/round3/review.md and analysis/redirect/r3-20261121-a/.
No new structural redirect question arose. The coordinator accepted terminal-incomplete
stopping after independent review of all four islands; unchanged research is not pending.
Prior final editorial acceptance is preserved in analysis/round-6/commentary/editorial-audit.md.
Current final editorial acceptance remains pending after this fallback reconciliation.
