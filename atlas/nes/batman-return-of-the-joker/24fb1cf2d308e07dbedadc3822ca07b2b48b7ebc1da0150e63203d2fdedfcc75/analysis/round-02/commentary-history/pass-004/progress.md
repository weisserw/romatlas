# Commentary progress

Round 1 current-source readability complete at pass-004. All 49 inventory intervals reviewed, including all 26 classified regions (276715 bytes, 417 code) and explanatory boundary notes for all 23 unknown intervals (116517 bytes). Eight meaningful routine/structure units recorded. No prior commentary existed; frozen before/ and baseline/state copies retained.

source/game.asm names interrupt saves, pending/applied PPU state, mapper values, IRQ dispatch pointer and selector, reset loops, page-clear setup and startup fields. Comments explain NMI branches, OAM DMA, bank restoration limits, reset setup and partial callee contracts. Uniform PRG data remains generic after reusing boundary/reference checks in analysis/round-01/classification-evidence.json; CHR tile format and vector layout explained. No new classification or natural execution claimed.

Fresh handoff verification: commentary/verification.json byte_identical=true, ROM SHA256 24fb1cf2d308e07dbedadc3822ca07b2b48b7ebc1da0150e63203d2fdedfcc75; game.asm SHA256 57e194928bdd7c0dc11095c3ca4109b769b187adcd4840fa8a086d19e6d14596. reading-after.asm refreshed after final edits. Map and state remain identical to frozen pass-004 identity 13e76833ad4c8350ca4fffcdbc6ac693aa5a100b19bcc07180a8df40db5c5e81.

No remaining current-source readability gaps. Final editorial audit remains pending as required; ROM classification remains incomplete. Deferred meanings and redirect questions are in uncertainties.md. TK-001 remains one documented missing feature, mapper69 debugger admission; no new issue found. Input unchanged, no toolkit changes. At handoff free disk 4.9GiB, scratch 20KiB; no bulk captures created.
