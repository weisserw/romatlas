# Current-source meanings and conditional contracts

All13444 classified instructions,356 Data regions and CHR have local mechanical
explanations. All15 Unknown intervals (142 bytes) have individual boundary/role
notes in source; exact intervals are analysis/round3/unknown.json. No source-review
range remains pending. Earlier uncertainty records remain intact under
analysis/round-3/commentary/uncertainties.md.

- FA81 restoration is established conditionally: complete MMC1 latch, PRG mode3
  and04A3 matching the supported incoming register. F76D selects sound register4
  without changing04A3; F777 reloads the remembered value. This does not prove a
  global actual shadow invariant or natural selection of every sound continuation.
- CEE3 retains the caller return below status/X/Y and records taskSP. CE8C eventually
  restores that stack and pops Y/X/status before RTS. Continuation requires eventual
  resumption with the supported effective bank tuple; immediate return is false.
  CEAF has five encoded entry words, with C1DD initializing task0 after bank6 setup;
  six scheduler slots do not prove a sixth entry word.
- E902's post-C14E bytes remain Unknown: initialization clears0110..01CF, so a safe
  current stack/return-frame contract is needed independently of bank restoration.
  Other residuals are short unselected control/hardware/menu variants and one
  apparent pointer; each source boundary states its actual evidence limitation.
- D3A7,DD3E,DD4E andDE99 have ordinary and nonlocal alternatives. The explicit normal
  conditions support their continuations; frame-removing alternatives do not.
  Synthetic return frames in C50C/FE32 belong to finite dispatch itself.
- Bank10 8879..88FB has a static actor/OAM collision contract, not established
  natural selection. BNE at887B bypasses collision when6D is nonzero. Projectile
  flags, owner selection, finite sprite scans, bossHP and flash updates are explained.
- Stage palette/map routines restore only a matching remembered lower pair.
  C614 selects a transient pair0/1 through14/15; the fixed C5FD continuation remains
  physical14 before F777. F1CA explicitly selects10/11 then restores12/13.
- Exact sound cue/instrument, actor/reward/effect, tile/text identities and some
  generic-data subdivisions remain unresolved. Known formats/consumers, record
  fields and bounds are explained without assigning invented game meanings.
  Shared scratch RAM has contextual aliases; unresolved CPU aliases remain numeric.
- Reset writes4022..408A remain literal behavior without a supported peripheral
  interpretation or full compatibility claim for this mapper1 image.

Primary structural evidence: analysis/round3/review.md and
analysis/redirect/r2-20261121-a/contracts.md. No gameplay replay, structural map edit,
toolkit modification or terminal-exhaustion assessment was performed here.
Final editorial/archive acceptance remains pending.
