# Authoritative final editorial corrections

Checkpoint `pass-003-final-context`; structural classification and all executable source remain unchanged. This document supersedes the two wording/count findings in `analysis/redirect/round-003-structural-frontier/editorial-findings.json` without altering historical outcomes.

At source game.asm:5229, the screen-disable routine ends at CPU $8A0F, physical68128 exclusive. The next physical interval [68128,68134) is the accepted six-byte controller-mask Data table; [68134,68203) is its timed matcher. Replaced obsolete “Unknown bytes” prose with that concrete table boundary. No new structure or boundary inferred.

The decisive `analysis/redirect/round-002-contracts/sound-template.json` candidate/fixed hex strings are each81 bytes. Independent byte comparison gives exactly ten differing positions, zero-based17,18,27,28,40,41,53,54,66,67: two bytes of the local JMP operand and eight bytes of four table-address operands. Historical wording of eleven differences in review/investigation records is a counting error, not an additional relocation. Source game.asm:11214 now records ten; historical records remain intact. Four52-byte candidate arrays versus53-byte fixed-bank stride remain distinct.

R008 is statically relocated code, not a demonstrated sound routine: its bank8 JSR $8003 calls screen initialization. Actual caller, natural execution and safe returning use remain unresolved. No classification, role, bank contract or reachability status changes follow from these editorial corrections.
