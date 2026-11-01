# Coordinator evidence corrections

## Round 02 redirect exact game-over tail citation

Immutable ROM inspection confirms `4C 4B E8` at physical $C1E2, $1E32F and
$1EAF2. Physical $1EAE2 instead begins `35 4C 9F`.

The completed redirect handoff and point 3 of combined-contract-review.md correctly
identify $1EAF2. Some surrounding recipe/avenue prose uses EAE2 when referring to
that accepted transition tail. Use physical $1EAF2 for the exact opcode comparison;
distinguish the surrounding helper/source unit from the tail instruction. This is
a citation correction, not a classification change. Completed attempt records are
preserved verbatim; independent disassembly must read the exact primary bytes.

## Editorial carry

Round 02 redirect identifies the ContinueRestart comment near source line 64891
as stale: it calls following bytes unknown although ContinueDecrementLives is now
classified and explained. Next commentary/final editorial acceptance must repair
this concrete dependency statement and refresh verification/source links.
