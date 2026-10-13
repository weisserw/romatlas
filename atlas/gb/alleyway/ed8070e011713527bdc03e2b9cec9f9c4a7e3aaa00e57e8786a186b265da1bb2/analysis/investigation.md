# Investigation ledger

## Pass 007 blocked audit — U-ZERO-0426

Revalidated the current ROM, saved map and existing evidence rather than relying on prior status text. Live map.validate passes and analysis.unknown still reports only `[0426,0445)`. Fresh disasm.verify rebuilds the current pass-006 source byte-identically. All 160 classified regions retain role/evidence metadata; both physical banks' counts cover the ROM exactly including the 31-byte unknown gap. No new references or context dependencies appeared: decoded immediate-reference count into this interval is zero; the sole indirect jump is the established dispatcher 054b. The filtered 257-frame startup/play/joypad/reset probe remains error-free with zero matching events and zero drops. No process is pending, so this is not a wait.

The pass-005, pass-006 and current goal turns encounter the same blocker: no evidence choosing dormant NOP code, zero reservation or unused zero data. Earlier static, pointer, semantic, format, runtime and external-source results are preserved below. Pass 006 pursued the remaining concrete new lead and data-read route; neither supplied a structural discriminator. No new concrete route or applicable source artifact is present now. Random repetition and forced execution would not distinguish the competing source roles. Reopening condition: a version-matched source/symbol reservation description or newly evidenced natural incoming reference/access. Goal status is blocked after the third consecutive recurrence, with the whole-ROM objective unchanged and incomplete.

## Pass 006 continuation — U-ZERO-0426 reopened and rechecked

Current unresolved structural range remains physical/CPU `0426-0445`, bank 0, length 31. All other regions retain their annotations; no new bank-context ambiguity. Authoritative checkpoint is pass-006. Previous exhaustion assessment was reopened because a new external lead and a missing runtime data-read check were available.

**New external lead:** [Duo's ASMSchool index](http://gameboy.mongenel.com/asmschool.html) mentions an Alleyway disassembly as a planned lesson. Parsed HTML contained no hyperlink for that lesson. The linked [Super Mario Land notes](http://gameboy.mongenel.com/dmg/smldis.html) concern a different game and contain no applicable zero-block evidence. The pages were retrieved over HTTP after HTTPS hostname validation failed; no archive or ROM was fetched. The site therefore supplies no usable version-matched structure/symbol artifact for this range. This eliminates this specific lead; it does not establish that no such artifact can exist elsewhere.

**Targeted runtime data-read method:** the preceding joypad scanner and following interrupt helpers suggested testing whether a joypad/reset route consumes the zeros as ROM data. Earlier zero-capacity traces could not check that hypothesis. `frame_scenario.py zero-probe` now filters Instruction/Fetch/Read detail to cartridge-rom `[0426,0445)` with capacity 1000 from emulator startup. It repeats the established title/stage/launch/move/pause route and adds Start+Select reset. `zero-probe-results.json` contains all requests/replies. 257 frames and 4,009,009 operations completed without errors; query has zero events and trace has zero detail/aggregate drops. Thus this route does not read/fetch the zero block. Reset returns FFA4 to title setup state 2. Untested gameplay is not claimed to lack accesses.

**Dependency recheck:** reviewed local static instruction boundaries and data consumers while correcting the multiplier symbol to 0454 and clarifying the exact three-byte table strides at 1b71 and 4551. These corrections add no reference to 0426-0445. All 160 role/evidence records survive validation/export; the final unknown remains distinct from portable raw `.data` emission. Existing complete direct-reference scans and rejected pointer hypotheses are still applicable to unchanged ROM bytes and extents.

**Remaining interpretations and path assessment:** A = dormant 31-NOP routine ending at RET 0445; B = reserved zeros preceding a RET stub; C = unused zero data. Their current bytes and tested behavior are identical. Additional random gameplay has no identified consumer/caller or distinguishing route. Forcing execution would demonstrate legal NOP behavior while leaving source intent unresolved; treating zeros as a table would likewise assert an interpretation without evidence. No concrete local method currently discriminates A/B/C. An applicable original/source-derived symbol/reservation description or newly evidenced natural access would reopen the entry. This is an evidence-specific impasse, not universal unknowability or a toolkit limitation. Goal remains incomplete; no completion status requested.

## Pass 001 ledger

## Current pass 003 status

## Current pass 004 status

## Current pass 005 status — U-ZERO-0426 (31 bytes)

All other structural intervals are classified. Bank0 has31 unknown bytes, bank1 none; counts cover32768 exactly. Contexts fixed0/switch1 are resolved. C-HRAM and C-DISPATCH structural ambiguities are resolved; stage-play-results.json confirms HRAM DMA execution and normal launch/play/pause handlers. Required annotations are preserved in state.json and annotations.json. No unsupported classification was introduced for this final interval.

### U-ZERO-0426 — physical0426-0445, bank0 / CPU0426-0445

Evidence: immutable ROM has31 zero bytes; preceding joypad routine ends with unconditional JP0410-loop at0423 (target040d), so it does not fall through. At0445 a RET precedes interrupt-mask helpers0446/044d. Decoder inspection in audit-results.json yields31 one-byte NOPs then RET. Confidence in byte/boundary facts is high; intended role remains unsupported.

Competing structures: (A) unreferenced31-NOP delay/no-op routine ending at0445; (B) zero reservation/padding before standalone RET; (C) unused zero-valued data array with no identified consumer. All reproduce the same bytes; exported raw emission and successful rebuild do not discriminate them. Neither absence of coverage nor apparent unreachability is used as classification evidence.

Methods reviewed and outcomes:

- **Static control/boundaries:** all4830 inspected instruction starts have complete legal nonoverlapping decodes. Direct jump/call targets do not enter0426-0445. No RST instructions exist; the only indirect jump is054b, supported by state table054c and audited state writes0..12. No caller to0445 or any interior zero byte is established. Decoding a plausible NOP sled is insufficient to assert code. Candidate discoveries from legitimate and semantically established detached roots were fully paged; no budget exhaustion.
- **Incoming references/caller semantics:** every decoded n16/a16 operand was scanned; none names this interval. Preceding key scanner branches backward and finishes by return from0415; subsequent helpers operate on interrupt enable and arithmetic. None consumes a zero table or calls a delay here. No semantic dependency from graphics/stage/sound classifications opened a new reference.
- **Pointer/table hypotheses:** full ROM unaligned two-byte scans in both endiannesses found32 little-endian and10 big-endian raw occurrences targeting this interval. Every occurrence is within a decoded instruction, two2bpp tile payloads, a crossing boundary at408c (end of pointer table / first scrolling column), or VRAM command payload at4a5a. Their consumers establish non-pointer meanings. Whole-range pointer hypothesis at0416 in audit-results.json produces0000 targets for zeros and conflicts with neighboring code; no supported pointer consumer. Raw pointer values are not fetch proof. ROM-only context removes bank alternative hypotheses.
- **Runtime instruction/data evidence:** early startup, title, stage entry, launch, movement and pause/resume retain2307 ROM fetch addresses, none here. Detail tracing had capacity0, so no claim is made about comprehensive data-read absence. A ROM data access to these zeros could support a table, but no concrete route/consumer pointing here has been found. Further arbitrary play lacks a discriminator; known input control changes go through the closed dispatch table, and known ROM data consumers address other regions. Forced execution at0426 (not an exposed normal API route) would only demonstrate that zeros can act as NOPs, not original role. Therefore repeating the route or arbitrary snapshot branches is excluded unless a new natural reference appears.
- **Patterns/neighbors/formats:** length31 has no identified alignment boundary or known array stride; adding RET yields a possible32-byte no-op function. Other FF padding has hardware-slot/section boundaries, but that does not justify assigning the same kind to this zero run. No established VRAM, stage, sprite, audio or header grammar consumes it. Matching uniform bytes alone is insufficient.
- **External source:** targeted searches for Alleyway disassembly and0426 disassembly found no applicable original source/symbol evidence. No ROMs or assets were downloaded. This does not prove such material can never be found; it only records that this path currently has no applicable artifact.

Current local-method assessment: no evidenced next experiment can distinguish A/B/C. Assigning a kind now requires guessing original intent. The specific missing evidence is an original/source-derived symbol/reservation description or a naturally sourced reference/access. This is not universal unknowability and not a toolkit execution limitation. Reopen upon a new caller, data consumer, input route that demonstrably targets the range, or version-matched source metadata. Audit dependencies in other regions again if those change. Incomplete handoff is permitted by exhaustion of concrete local paths, but the whole-ROM goal is not complete.

U-SOUND data regions resolved via consumer grammar and checked delimiters in classify_sound.py; no arbitrary byte decoding used for sequence data. U-GFXTABLES resolved 24-delta table using explicit loop bound. U-EMPTY grid at4333-43dc resolved as repeated 14-cell format ending FF; no caller is asserted. Exact unknowns now: 0003-0040,0043-0048,004b-0050,0053-0058,005b-0060,0061-0100,0426-0445,641a-6800,7a7f-7ff0,7ff7-8000.

U-EMPTY0426-0445 remains31 zero bytes before RET. Static decoded code and tables contain no established reference to this interval; competing dormant NOP-return stub and reserved padding remain. Next: bounded pointer hypothesis audit, data-reference analysis and adjacent input helper semantics. This has not been exhausted.

C-DISPATCH remains for state roles and index bound audit; C-HRAM runtime execution now observed in gameplay-results.json. Longer startup passes the first HALT; inputs held only3000 operations are shorter than a frame and may be missed. Next: frame-budgeted input route, inspect FFA4/VRAM to distinguish title and stage states.

U-CORE and U-LEVEL have been resolved structurally through decoded standalone helpers, fixed-stride table consumers and parsed stage/VRAM streams; some unused records remain lower-confidence data with usage explicitly unconfirmed. U-GFXTABLES now retains only 44f5-450d (24 motion deltas) and the neighboring 4333-43dc empty-area ambiguity. See annotations.json and classify_data.py for exact evidence and preserved checks.

Current unknown intervals (half-open): 0003-0040,0043-0048,004b-0050,0053-0058,005b-0060,0061-0100,0426-0445,4333-43dc,44f5-450d,641a-6800,694b-694c,6f17-6f18,6fac-7050,7531-7ff0,7ff7-8000. Data-classification-results.json retains authoritative paged query. None is exhausted.

U-SOUND narrowed to five-byte register/envelope tables 6fac-704c, four-byte duration table 704c-7050, sequence/frequency region 7531-7a7f, FF interval 7a7f-7ff0, FF interval 7ff7-8000, plus unused RET bytes 694b and 6f17. Methods: inspect register-copy and bytecode consumers, follow loops/termination and validate frequency-word structure. Sound test selectors 681a-69e7 have semantic code evidence without execution. Next possible runtime route: normal VBlank invokes sound update, then title/gameplay triggers selective streams.

U-EMPTY: 0426-0445 is 31 zero bytes before RET at 0445; competing interpretations include unused delay NOP routine, padding or empty record array. 4333-43dc is predominantly zeros with one 01 and a final FF, suggesting a dormant display/layout buffer or unused grid; consumer and schema remain unestablished. Inspect reference/pointer hypotheses and adjacent functions, then determine whether these match formats found elsewhere. No automatic zero/padding classification.

U-FF: leading vector gaps,641a-6800 and terminal7ff7-8000 are FF runs adjacent to established code/data boundaries. Need audit of direct/indirect references and alignment/fill semantics before classification; uniform content alone is insufficient. 7a7f boundary depends on sound-parser evidence.

Exact unresolved physical intervals: classification-results.json, final analysis.unknown result (31 gaps, 17662 bytes). Each interval remains unknown. This ledger groups intervals by shared consumers; splitting is required as evidence narrows boundaries. No path is exhausted and no external blocker is established.

### U-VECTORS — gaps below 0100
Hypothesis: FF linker fill between reset and interrupt stubs, including unused restart slots. Evidence: ROM bytes and decode at 0000/0040; valid vector stubs at 0040,0048,0050,0058 and RETI at 0060. Competing interpretation: deliberate RST38 instructions, rather than padding. Next: audit all code references to restart slots and distinguish fill from intended code. No runtime fetches observed in gaps. Unblocked.

### U-CORE — gaps 0302-0358,0426-0453,0472-047c,08c2-08ca,0ece-0edb,0f5d-0f7c,100b-101b
Hypotheses: shared helpers not reached statically, constants and pointer tables. References include n16=0348 from 0989/09d6, 08c2/08c6 from 08a7/08ac, and 100b from 0fd6. Candidate-only decoding is not proof. Next: inspect those consumers, count/index bounds and callers. Unblocked.

### U-LEVEL — gaps 11ee-198c and 1b33-43dc, plus 1aaf-1ac6,1ade-1af5,1b0d-1b1b
Hypotheses: collision/velocity tables, stage layouts and compressed VRAM command streams. Evidence: many LD HL/DE immediates point into these intervals, including 11ee,1b87,1be1,4075,41cd,42b3. 02c1 parses commands with destination and mode/count; this can establish actual stream extents. Competing: detached code among tables. Next: inspect each referenced consumer, parse streams through explicit terminators, locate level pointer and index limits; then target scrolling/descending/bonus states via normal play. Unblocked.

### U-GFXTABLES — 44f5-450d,4551-455d,4599-45bd,45e6-4603,4a2a-4a66,4a8b-4b75
Hypotheses: animation, collision, tilemap or sprite tables near rendering code. Evidence: immediate references to 44f5,4551,4554,4599,45e6,4a3c,4a8b; 4b75 is established tile asset boundary. Next: inspect index/loop consumers and layout semantics; classify exact extents only. Unblocked.

### U-SOUND — 641a-6800,681a-69e7,6f17-6f18,6f8b-6f8f,6fac-7050,7526-7ff0,7ff7-8000
Hypotheses: sound sequence/envelope/frequency data, code and FF fill. Evidence: extensive five-byte-looking tables referenced from 6axx-6dxx and streams at 75xx-7axx; VBlank calls 7ff0 which jumps to 6800. Next: inspect sound consumers and sequence opcodes, exact termination/loop pointers, detached code and FF boundary at 7a7f. Unblocked.

### C-DISPATCH — FFA4 and 054c-056c
16-entry handler table structurally supported; static discovery across all targets has legal nonoverlapping code. Remaining: prove stored indices stay within 0..15, identify each state's role and obtain runtime fetches. Examine every FFA4 write and surrounding logic; continue startup past frame wait before input. Unblocked.

### C-HRAM — FF80 alias of ROM 03b5-03c1
Resolved structurally via exact copy loop and VBlank call; runtime execution not yet observed. No bank ambiguity: ROM-only cartridge has fixed physical aliases. Trace HRAM after normal startup to confirm copy and DMA behavior. Unblocked.
