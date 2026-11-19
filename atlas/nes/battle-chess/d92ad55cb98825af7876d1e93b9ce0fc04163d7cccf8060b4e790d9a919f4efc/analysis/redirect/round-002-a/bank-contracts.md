# Static conditional normal-return contracts

EA02: current source EnterEngineOverlay, exact mapper argument13 to SelectPrgPair produces effective26/27, fixed30/31. ExchangeEngineOverlayPage swaps RAM zero-page$04+X (wraps) and absolute$0560+X, balanced PHA/PLA periteration, Xcycles256; RTS. Restricted NMI enabled first.

EA32: LeaveEngineOverlay swaps samepage, imports fixed RAM fields, selects mapperregister0, clears restrictedNMI, RTS; result effective0/1/30/31.

C931: LoadAndUploadPpuResource loads immutable seven-byte resource descriptor, WaitForFrame, selects descriptorbank, UploadPpuResource raw/packet paths converge C97C restore-tail selecting register0 then RTS. No ROM/CPUstack writes except nested JSR return stack. Data source reads/PPU writes do not touch CPUreturn stack. Descriptor14 and packet/raw bounds independently reviewed pass002 independent-review.md, prior resource-review.json. Contract is conditional on normal completion; no termination/naturalreachability claim.

C8C0: LoadDisplayScene pushes selector, balanced PLA before descriptor resolution; each of four optional calls to C931 resets pair0. On all-skipped path retains incoming pair, so pair0 return guarantee requires incoming pair0. All actual roots processed by known transition below are pair0 unless explicitly constrained. Fixed pointer metadata remains physical30/31. Scene6bounded descriptor records establish normal upload endpoints. Final palette fade helpers do not select mapper bank.

Guard: only apply C8C0 with incoming0/1; C931 always resets0/1. New traversal must enforce this.
