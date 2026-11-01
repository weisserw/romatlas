# C1E2: narrower Static transition-thunk proposal

Claim only: physical $C1E2..$C1E5 is one Static `JMP $E84B` instruction,
CPU $81D2, effective PRG slots [6,7,14,15]. It is an input-independent helper
that sets gameplay state 3 and clears gameplay substate before returning on the
existing caller frame. Actual entry, natural selection, original symbol and deadness
are unresolved. Suggested descriptive name: BrawlEnterGameOverState.

Primary evidence: [source excerpts](primary-source-excerpts.txt), immutable
[ROM windows](primary-rom-windows.json), current [ledger](residual-ledger.json).
Full source references are commentary/source/game.asm:11145,12300,12355,12360,
59875,63565,64899; symbols.inc:412 explains the overlap.

The previous review correctly excludes preceding fallthrough: $C1E1 is RTS and
$C1E5 begins a separately supported completion handler. The finite ten-word
BrawlStageDispatch table at $C01C..$C030 selects 8020,8078,8088,80AF,80C4,
80FF,81C3,81D5,8213,8208. It does not select 81D2. This rules out that concrete
entry route, not a structural helper role. It also fixes the exact residual boundary
without deriving it from exporter formatting.

Combine these independent constraints rather than treating a caller as mandatory:

1. Neighboring semantic organization is a stage-transition group. $81C3..$81D1
   disables splitting, sets render suppression and restarts substate 1; $81D5 onward
   preserves equipment, advances ordinary gameplay state or enters a final delay.
   The intervening three bytes implement precisely the missing game-over transition
   member of this local family, without reading arbitrary registers or local state.
2. The destination's actual bytes, not its name, establish the delegated field and
   stack contract. $E84B decodes `LDA #3` inside the BIT operand at $E84A; $E84D
   stores $34, then JMP $E843 loads 0/stores $35/RTS. No mapper write, indexed read,
   unknown selector, extra PLA or helper-specific register precondition occurs.
   The fixed slot remains bank15. Entry requires an ordinary return frame, exactly
   like the neighboring callable state helpers; JMP preserves that frame.
3. Exact `4C 4B E8` also occurs at accepted $1E32F (LoseLife) and $1EAF2
   (ContinueDecrementLives). Those sites use the same terminal transition with
   established life-exhaustion semantics. In the second, extra PLA/PLA is explicit
   before the jump; the candidate has no extra caller-frame claim. These are actual
   contextual examples of this tail convention, not hypothetical destinations.
4. The exact three-byte interval is exhausted by a complete terminating transfer.
   It neither borrows bytes from adjacent accepted instructions nor falls into
   inline data. This is stronger than an isolated RTS and has a specific effect
   derived independently from the shared state's stores.

No single item proves role by itself. Together they support an intentional local
state-transition thunk under the same complete mechanical-contract standard used
for accepted unobserved helpers in leaf-contracts.json and localized-reassessment.json
($1E0D7 timer/menu initializer and $102B3 screen/object reset prefix). Their more
extensive operations are inline; this candidate delegates its entire operation to
a demonstrated fixed-bank entry. The claimed role does not depend on finding an
actual supplier of a return frame. The old no-entry rejection is not discarded:
its negative reachability result remains true.

Strongest competing interpretation: three stored scalar/constant/padding bytes
happen to encode the same complete game-over tail among restart/completion helpers.
No surrounding indexed consumer/record format covers them, no repeated data layout
requires them, and no data boundary pattern distinguishes them from this semantic
code family. This competing role is possible but has less positive support than
the combined code contract. The proposal is an inference, never Observed.

Independent disassembly recipe: read these exact ROM windows with the configured
binary, check current map boundaries/banks and the E84B overlapping execution alias;
inspect E32F/EAE2 and the two previously accepted unobserved helpers. Review the
combined family/delegation/stack argument against the scalar competitor. If sufficient,
annotate only [49634,49637) as Static Code with CPU81D2/[6,7,14,15], then regenerate,
verify byte identity and run full current-source commentary/redirect. If insufficient,
record which combined inference fails; do not demand a live caller by default.
Artificial entry is intentionally omitted because success would not discriminate.
No connected downstream unknown exists: the entire shared state setter and caller
convention are classified, and the finite local transition group is now inspected.
