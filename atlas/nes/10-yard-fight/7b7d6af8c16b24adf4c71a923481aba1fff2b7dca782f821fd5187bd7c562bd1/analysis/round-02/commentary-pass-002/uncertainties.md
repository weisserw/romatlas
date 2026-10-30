# Current-source limits and redirect questions

Readability covers every classified byte and gives opaque-boundary notes for all
19 unknown intervals. These are limits of the recovered meaning/structure, not
pending commentary ranges. Physical intervals use exclusive ends and include the
iNES header. NROM execution context is [0,1,2,3]. The structural baseline remains
pass-002, map SHA-256 ce45be0835a8cfcc257414a9ba00a884e3bfb25b46c8c9474a2159b14b959e04.

- Player state names describe established mechanics. Exact football event names
  for packed BallFlags/PlayResult bits, some scoring paths and alternate kick
  states are tentative. Source describes their tests and effects without claiming
  natural runtime observation. Audio selectors identify their channel/pointer
  contracts; event names remain generic where callers do not establish them.
- The large compressed-field, animation and steering-data pools have named
  consumer addresses and established decoder formats. They remain generic Data
  structurally; source does not claim each whole pool is one table or that every
  possible selector is valid. Unreferenced bytes retain a bounded non-code note.
- RAM scratch pairs and state-local player timer/auxiliary bytes have multiple
  meanings. Names deliberately describe storage contracts rather than imposing a
  single global game meaning. Absolute CPU table equates are distinct from
  `_Physical` byte-emission labels, which identify backing positions.

Concrete next structural questions for redirect:

1. `DispatchNextPlayer`/`PlayerHandlerPointers` establish the caller mechanics:
   X and CurrentPlayer select an eight-byte player record; $10/$11 contain the
   handler address; a synthesized RTS return resumes at CPU $9ED0, where sprite
   rendering consumes animation/palette in $12/$13. Several table targets lie in
   unknown islands (CPU $A46B, $A7F3, $A94F, $A99B, $AB20, $AD1A, $AE86, $AEA8,
   $AF46, $AF77). Inspect each target with this contract, its state-local fields
   and nonlocal stack exits; distinguish coherent handler boundaries from
   pointer-only evidence. A bounded state-selector trace or static contract
   analysis may resolve connected branches and their data together.
2. Physical $11C2..$128B follows `CheckHalfTimer` and precedes score sprite helpers;
   $12CC..$12E3 lies between sprite helpers. Test whether supported callers or
   interior entry contracts establish a scoreboard path; do not infer a complete
   routine from plausible decoding alone.
3. Physical $0C6B..$0CB4 and $0CF9..$0D38 sit among down/progress calculations and
   field-message tile consumers; recover entry/bounds before assigning code or
   message data. $0FE6..$100C follows decimal conversion and precedes high-score
   logic; its placement alone does not establish a numeric helper.
4. Physical $48BE..$48FF is next to steering/action selection and skill setup;
   $4C1E..$4C34 is a code-like prefix before independent uniform FF fill. Check
   their caller/stack/selector contracts independently of the adjacent data.
5. Four bytes at $3EAF..$3EB3 and thirteen at $4150..$415D interrupt the audio
   code. Check consumers and command/entry boundaries; proximity to the audio
   engine does not establish a stream or executable fragment.

Exact residual inventory remains analysis/evidence/round01/unknown-ranges.json
(2,100 bytes). Preserve prior failed/accepted reasoning in investigation.md and
leads.json rather than repeating unsupported pointer or decode arguments.
