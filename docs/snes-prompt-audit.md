# SNES workflow and evidence audit

2026-10-18. Read-only audit of current generated agent instructions and their
supporting scripts. No active Game Boy workspace or production prompt was changed.
This specifies the required platform split before production SNES integration.
The later continuous-redirect workflow retired the final-investigation prompt
named below; its methods now belong to redirect. The audit rows remain historical.

## Preserve the evidence contract

The opening policy in workflow-template/disassembly.txt, redirect.txt and
final-investigation.txt, the repeated policy in goal.md and disassemble.py,
already separates structural classification from reachability. Preserve it:
static control/data flow, routine contracts and bounded formats can support roles
and exact boundaries without executing every path. Preserve provenance and the
independent extent requirement for tables. A new console does not justify stronger
universal evidence requirements or abandoning productive independent avenues.

65816 decoding needs *only the state relevant to the instruction*: E/M for
accumulator immediates, E/X for index immediates. Many instructions have fixed
length regardless. DB/direct-page state affects references, not those immediate
lengths. Bank identity and encoded CPU aliases are separate from processor width.
SPC700 has its own fixed instruction formats and direct-page/reference context;
do not apply 65816 width rules to it or SM83.

A statically established entry contract or sound flag propagation is useful
context. Runtime observations are another source, not a prerequisite. Branch
merges, PLP/RTI/XCE and unknown incoming state may leave alternatives; preserve
those alternatives where relevant. Do not infer that all downstream bytes are
unknowable, require a gameplay trace for all code, or reject an otherwise sound
role solely because natural reachability is unresolved.

## Audit findings and required changes

| Surface inspected | Current assumption | Required SNES behavior |
|---|---|---|
| scripts/disassemble.py AGENTS/WORKFLOW and prepare | Owned .gb/.gbc, stock .sym, GB analysis commands and bank-organized source | Select a platform profile; use its map, backend, safe address context, per-CPU commands and source products. Preserve existing GB profile text/behavior. |
| workflow-template/disassembly.txt | Effective CPU/bank context; one main ROM instruction view implicit | Add SNES-only explanation of width-dependent views and sound static context. Confirm roles independently of whether execution was observed. |
| workflow-template/redirect.txt | Callers/writers/bank selection/dispatch provide leads | Investigate state transitions, CPU aliases and SPC uploads where the question needs them. No universal trace or caller requirement. Continue unaffected hypotheses. |
| workflow-template/commentary.txt and scripts/commentary.py prose | Replace understood numbers with symbols; explain mechanics/uncertainty | Preserve forced operand widths and encoded aliases when naming. Distinguish ROM storage from ARAM execution and CPU identities. Explain unresolved width/context locally. |
| workflow-template/final-investigation.txt, goal.md and coordinator text | Exact unknown complement, static evidence and separate classification/editorial gates | Keep those gates; do not make “all widths observed in gameplay” a new completion prerequisite. Account for unresolved interpretations explicitly. |
| scripts/commentary.py preparation/checkpoint | baseline.sym, GB file extensions, RGBDS/rgbfix, source inventory and transform assumptions | Backend-owned source inventory, patch/export and fidelity commands. Do not globally substitute assembler instructions in GB text. |
| scripts/archive.py | Explicit GB platform gate, .sym, GB assembly transform/source products | Separate SNES archive schema/backend validation; retain metadata-only archival and independently supplied-ROM rebuild. No generic acceptance before a backend exists. |
| scripts/port.py and port-template | GB-only archive gate; demonstration recipes/scaffold; native behavioral port | Platform-selected asset recipes/scaffold and reference handling. Preserve entire-game behavior accounting and ROM-at-startup loading. |
| romatlas-core execution/address/decode traits | Opaque context and wide addresses; state types adapter-owned | Existing extension points are usable. SNES adapter owns CPU identity and width/reference state; do not import it into shared crates. |
| romatlas-analysis discovery and map Store | Physical claims vs traversal contexts are separate; address context is opaque | Supply adapter-specific observation/context keys. A physical code role may have multiple valid CPU views/width interpretations; do not accidentally merge or reject them as overlapping physical claims. |
| romatlas-map stock .sym API | Explicit GB 16 KiB bank limits, 16-bit addresses/directives | Keep it GB-specific; add SNES portable schema without weakening stock .sym compatibility. |

Readability still covers all currently classified source. Unknown state receives
an honest local explanation, not a blanket excuse to stop commentary or redirect.
Do not rewrite or migrate existing game workspaces during platform implementation.

## Proposed SNES-only instruction text

> Decode each candidate using the processor context that affects that instruction.
> Static entry contracts and supported flag/control-flow reasoning may establish
> that context; runtime observations can supplement them. A runtime trace is not
> required for statically justified classification. Many 65816 instructions have
> fixed lengths and need no M/X assumption. When relevant state remains unknown,
> retain the alternative interpretations or leave the affected boundary unresolved,
> record the specific missing fact and continue work on unaffected ranges. Keep
> structural evidence and reachability separate. Preserve encoded CPU aliases and
> distinguish ROM storage from WRAM/ARAM execution locations and upload lifetimes.

This text is proposed, not installed into any current prompt. It does not require
agents to prove irrelevant flags or execute every branch. Shared evidence text
remains the current policy; the SM83 profile gains no width-context requirement.

## Validation required when prompts become active

- Snapshot existing GB prompt output before refactoring and compare the resulting
  GB profile; investigate any wording or command change. Existing prompt substring
  tests alone cannot establish semantic equivalence or agent behavior.
- Add SNES fixtures for static REP/SEP propagation, fixed-length instructions under
  unknown flags, divergent entries, restored-status ambiguity and SPC uploads.
  Check that unresolved context stays local and does not block independent work.
- Exercise disassembly/commentary/redirect, archive restoration and native-port
  preparation separately for both platforms. Keep stock GB portability gates.
- Audit generated instructions as prose, including implied requirements and stop
  conditions; tests do not prove that live agents interpret evidence guidance well.

## Implementation follow-through

New SNES workspaces now apply scripts/snes_workflow.py only when SNES is selected.
The shared phase templates are reused with platform paths/tool names and the
SNES-only context guidance. Baseline JSON maps, state scaffolds, commentary, Asar
source includes, archive reconstruction and native-port scaffolds have platform
branches. Existing Game Boy constants/profile text and game workspaces remain
unchanged. Runtime roots/edges retain relevant observed processor context even
with zero detail capacity; static-only traversal remains available.
Instruction tests do not establish live-agent judgment, research quality or
semantic completeness.
