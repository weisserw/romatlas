# R008 independent review: correct unsupported leading byte

Retain Static Code physical [17040,17176), CPU [$C280,$C308), effective PRG banks [0,1,2,3]. Restore physical17039 CPU C27F to unknown. No Data claim, observed execution or reachability upgrade.

Independently inspected original source_claim in analysis/redirect/r001-contracts/proposal-revised.json (C280..C308), original isolated C27F decode in residual-review.asm, round002 review, and decisive r004-terminal-review review/proposal/c27f-primary-chain/contextual decodes. Fresh configured-binary decodes in fresh-contextual-decodes.json confirm timer helper beginning C269 terminates at C27E; C27F is an isolated RTS; palette entry C280 stores X/Y/A, performs finite copy/nibble loops and ends C307 RTS. Original JSRs89E4/8A58 explicitly target C280. These establish the retained contract's bounds without extending it backwards.

The earlier merged physical proposal and acceptance included one byte outside its explicit source_claim. Prior acceptance is no independent structural evidence. C27F could be a deliberate no-effect return stub or obsolete return residue; legal decoding and adjacency cannot discriminate. Absent caller is not a universal prerequisite; the actual failure is unsupported extension beyond the proven entry and control contract. One-byte Data classification is likewise unsupported. This is evidence-scope correction, not a new whole-ROM caller rule.

Full original region451 retained in input-state.json and immutable historical records. Annotation RPC preserves ID/provenance/confidence and corrects range/view/prose. Every other region is exactly equal; maintained commentary source is unchanged for next-phase merge. Next commentary must remove misleading ReturnFromSpawnHelper code treatment and explain raw unknown byte, while preserving readable source.

Connected palette body remains accepted; no other connected classification changes are supported. Residual alternatives and broad progression reuse sound r004-terminal-review primary review and scenarios S001..S007 (flight, late mission19, landing rescue/base, actual caller return, artificial clean ending); unchanged replay cannot distinguish these marginal roles. No obvious productive continuation remains in this single pass; no terminal exhaustion or editorial audit is claimed.

No toolkit issues found. No toolkit edits, commentary/redirect/audit, archival, cleanup or publication. Local artifacts only. Disk approximately9GiB free; no scratch generated.
