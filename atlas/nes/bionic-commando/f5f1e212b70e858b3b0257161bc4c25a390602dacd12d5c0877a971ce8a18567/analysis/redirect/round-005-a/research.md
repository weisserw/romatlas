# Round005 corrected arithmetic contract

Input round-005-classified, map92909a271b71405418235187a087b923aa6930b0c5ffd9260d7d70f9f7d2df85; current-source commentary complete. This is a ready proposal, not terminal exhaustion. ROM/map/state/source preserved.

## Retry declaration and decisive evidence

Target two independently bounded arithmetic operations at physical3E9C2..3E9D7 and3E9D9..3E9EF, fixed physical8KiB slots30/31 (CPU E9B2..E9C7 and E9C9..E9DF). Prior round003 rejected whole64byte arithmetic and round004 retained downstream46 because BCC and final STA cross accepted vector boundaries. Primary [round004 decode](../round-004-a/fresh-decode.json), [round003 roots](../round-003-a/narrow-roots.txt), and exact immutable bytes/decode copied in [arithmetic-primary.json](arithmetic-primary.json) were retrieved. Changed inference: accepted shift now independently establishes exact numerical order29,28,27,1C,1B,1A; subtract and commit share that precise high-half order, not merely legal instruction syntax. Localize corrupt branch and truncated store without requiring a routine RTS. Discriminating check: all three-byte arithmetic chains must agree in byte order, carry propagation, producer/result storage, and exact complete operation boundaries. Stop if any mismatch; no runtime manipulation needed.

## Supported first21bytes

3E9C2 loads1F, ORs1 and stores20. SEC then computes low1C-20, mid1B-1E-borrow, high1A-1D-borrow, preserving low inX, middle inY, high inA and carry for no-borrow. Incoming carry is immaterial. It is a complete24bit subtraction with trial operand Q=(1D:1E:(1F|1)), input R=(1A:1B:1C); outputs (A:Y:X)=(R-Q) mod2^24, C=(R>=Q). NES decimal mode does not change arithmetic. Independent accepted preceding shift produces exactly R in this byte order; downstream STA1A/STY1B/STX1C consumes exactly these register outputs. This combined producer/arithmetic/consumer relationship distinguishes a preserved arithmetic block from numeric resources. Endpoint before BCC is the complete result/flag boundary, not an arbitrary opcode window.

## Supported following22bytes

3E9D9 stores A,Y,X into1A,1B,1C, exactly inverse of subtraction's saved register ordering. Then LDA20/ADC0/STA1F; LDA1E/ADC0/STA1E; LDA1D/ADC0 yields the three-byte Q+C result with high byte inA and low/mid stored. Carry flows unmodified through loads and stores. At the conventional subtraction-success fallthrough C=1, this commits R-Q and computes Q+1; more generally the block has a defined incoming-C precondition and computes Q+C. No claim is made that an actual caller supplies it. Final output in A is complete before3E9EF, whose dangling85 would consume vector4C; exclude that byte. A complete high-byte store is not required to identify the preceding three-byte carry calculation, exactly as the subtract produces its high result inA.

The strongest alternative is stale arithmetic partially overwritten by vectors. It explains the damaged enclosing graph but supports the intact suboperations' Code role. No independently consumed numeric schema competes for these precise carry/register/storage relationships. We do not assert an original square-root algorithm, a repaired executable routine, ordinary reachability or successful complete execution. Proposal43bytes Static Code only.

## Exact conflict and continuation limits

Physical3E9D7/CPU E9C7 BCC+1A reaches CPU E9E3=physical3E9F3, accepted JMP operand. Exclude2bytes; do not silently retarget it. Physical3E9EF85 overlaps accepted vector4C as operand; exclude1byte. Prior leading3E9B017 also excluded. These4bytes remain unknown; no invented branch repair or partial store claims. The two proposed straight-line blocks are connected by dataflow, not a claim that the intervening corrupt control edge is supported. Independent disassembly can validate each separately.

## Other identified avenues

Reused sound primary review at [round004 review](../round-004-a/rejection-review.md), after inspecting its source-contracts and decode. Counter50 retains actual branch conflict3D6A5→3D6B5 and truncated load crossing3D6B0. Narrow3D6A7..AF clears low5 after decrement, but unlike proposed arithmetic has no independently established corresponding numeric output/producer contract beyond the same damaged counter sketch; no new discriminating constraint. Wrapper four calls corroborate a sketch, not successful helper semantics. Earlier pointer/mapper-shaped prefix includes unsupported E000 BRK entry; no supported contiguous extension.

Renderer prefix4 has competing alignment consuming proposed STA operand; no independent predecessor. Tail7 PLA/TAX requires external stack and branch targets physical3D5E6 unrelated OAM. Already accepted inner48 does not fix those specific defects. No new corrected target or stack fact remains. BC00 wrapper11/vector3: retrieved [primary dictionary evidence](../round-002-a/bc00-conflict-primary.json), exact wrapper A90B20AADC2000BC4CA8DC and serializer0B effective22/23. Independently consumed collision dictionary at physical2FC10 remains actual invalid callee; bank-prefix LDA/JSR alone has no additional independently bounded operation. No alternate-bank invention. Trap00/35 lacks BRK ABI or repeated noncode evidence. Isolated24 retain exact primary flanks from round003 inspection and round002 neighbors: single returns, three JMPFFFF placeholders, JMP820D and JMP9C32/TAX have no additional independent contracts distinguishing code/data. Generic-data fallback cannot resolve these because code-shaped material remains a concrete competing interpretation.

Reviewed current commentary handoff/scaffold and existing uncertainty/generic-range contracts; no uncovered commentary gap or additional noncode shelf emerged. Existing graphs and accepted data extents do not produce new residual consumers. These are candidate-specific stops, not universal missing-caller/RTS/schema requirements. Ready43 combined batch is proposed; no terminal claim.

## Gameplay review

Reviewed analysis/scenarios.json. Broad later rooms/bosses, rescued helicopter/base explosion and artificial terminal signature are already visited. Final cockpit defeat/natural credits remain unproved; primary final-room-fields and boss-graph previously distinguish room101→96 from actual cockpit contract. No changed RAM contract or new discriminating route arose from numerical fragments. Replaying the unchanged artificial terminal or final-shot tests would repeat evidence. Retain those milestone limits and their recipes unchanged; this ready structural handoff does not declare gameplay exhausted.
