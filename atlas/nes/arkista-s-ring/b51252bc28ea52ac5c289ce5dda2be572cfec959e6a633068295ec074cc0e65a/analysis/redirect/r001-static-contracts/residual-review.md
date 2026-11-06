# Localized residual review (after proposed batch only)

Accepted unknown remains6644; this review concerns hypothetical two-byte residual
if all proposals survive independent disassembly. It is not a terminal full-ROM
assessment and cannot supersede the next disassembly/commentary/redirect cycle.

- physical0x2991, CPUA981, fixed[0,1,2,3], byte60. Previous callback's CMP result
  returns atA980; followingA982 begins coherent entity movement routine. No direct
  branch/call/dispatch word in examined code tables selectsA981. Strong competing
  interpretations: spare return routine, redundant delimiter/padding byte. An
  arbitrary JSR-style debugger setup would executeRTS but prove neither intended
  role nor a supplied return-stack contract. StandaloneRTS plus legal decoding is
  specifically insufficient under evidence-policy.md. Retain unknown absent new
  encoded-entry/control/stack evidence; do not infer dead/unreachable.
- physical0x44B2, CPUC4A2, fixed[0,1,2,3], byte60. Nineteen precedingA2 C4 pairs
  are proposed genericData from pattern/bounds, but also look like little-endian
  pointers to this byte. No established consumer supplies a callback contract and
  no direct edge selectsC4A2. Alternative: no-op callback after inert dispatch
  array; alternative data/padding byte. Pointer pattern and oneRTS alone do not
  establishCode. Keep this one byte separate; missing consumer does not reject
  the preceding repeated-pair non-code proposal. No artificial execution ofRTS
  would discriminate these interpretations. A concrete new consumer/static entry
  would reopen, otherwise reuse this localized review on unchanged evidence.

Primary artifacts: proposal.json raw data boundaries, tables.json table targets,
unknown-decode.json and discovery.json control records; boundary-audit.json scans
all6422 accepted-map region boundaries (Code region count is separately recorded)
and new candidate successors. analysis/prg-review.txt contains neighboring bytes
for direct visual review. None of these absences proves natural non-use.
