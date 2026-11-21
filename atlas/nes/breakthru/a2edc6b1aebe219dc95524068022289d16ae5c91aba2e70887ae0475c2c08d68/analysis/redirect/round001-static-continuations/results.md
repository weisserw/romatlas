# Round 001 combined redirect results

Input remains pass-001-final. Accepted classification: 256690/262160; exact unknown complement: 5470 bytes/138 ranges. Source readability completed before redirect; no source/map/classification/ROM edits here. Toolkit configured release binary used through run.py serve. No runtime experiments or new observed-execution claims.

Combined independent-review proposal: **5451 bytes**, comprising **4832 Code + 619 Data**. Exact physical slices, CPU aliases, effective contexts and individual rationales are in [proposals.json](proposals.json). Per-lead totals: R001 bank1 3407; R002 banks12/13 1901; R003 fixed banks14/15 143. These are proposed, not accepted counts. If independently accepted, 19 bytes remain unknown; do not report full classification.

## Connected R001 object investigation

Primary evidence: `commentary/source/game.asm` DispatchFirstObjectPool/ProbeFirstObjectRecord (~432), RenderAndRetireObject (~2613), ExpandObjectMetasprite (~2808), AllocateSecondPoolRecord (~3030), ObjectWorldToScreenX and ChooseObjectSoundFromVisibility (~3130); exact accepted instruction records in [accepted-instructions.json](accepted-instructions.json), new decode in [proposed-code-decode.json](proposed-code-decode.json).

The dispatcher pushes $A089 and JMPs the selected handler, so normal RTS enters unknown $A08A, not merely the next textual instruction after JMP. Its four bytes decrement remaining objects and branch to the established dispatcher return. Most other gaps follow `STA ($40),Y`: objectPtr is an established six-byte RAM record in one of the two pools. These stores preserve bank context and ordinary PC continuation, unlike an arbitrary pointer into mapper ROM. Restores of saved Y, PHP/PLP carry arithmetic, bounded phase updates and render calls complete the local contracts. Post-RenderAndRetireObject RTS islands are supported ordinary returns: its own register saves are balanced; it does not discard the dispatcher's frame.

Recovered handler continuations call $B7DC, so the first-pool allocator is included with downstream copy, cursor/count updates and register restoration. Its six-byte-copy/21-record contract parallels the already supported second-pool allocator. $B886 closes the latter's RAM copy. $B649 three-byte CMP/RTS is split off as unresolved; the following $B64C 18-byte descriptor resolver is an exact duplicate of fixed ResolveMetasprite $C97E with the same independently anchored $F000 table contract. No live caller is asserted for that duplicate.

$B8E7..$B919 is non-code selector conversion storage read by projectile allocation. Sound-rule gaps have repeated kind/request pairs and $FF suppression in the established $B995 consumer area. Full dispatch selector/table completeness is not needed for non-code classification and is not claimed. Final six bytes of object PRG pair repeat the established reset transfer address $BA64 three times.

## Independent connected R002 audio/ending investigation

All three explicit ending state dispatches were followed in this phase, including state writes, timer gates, drawing helpers and downstream PPU/map helpers. No handler was deferred merely as another actor. Ending loop $9055 closes through all its explicitly called PPU/background helpers $90CA/$910A/$9136/$9158/$9185. Three unused-context MMC1 writers at $91F4/$920A/$9220 are exact 22-byte duplicates of accepted fixed functions at physical116847/116869/116891: supported static argument/register contracts, actual entry unclaimed, post-mapper context not propagated.

Tertiary helper contains **data at $9572..$9575** (physical103810..103813), independently bounded by modulo3 index and the next called helper $9575. New decode restarts at $9575; initial linear decode across that payload is not used as code evidence.

$9DD8..$9E3A comprises 49 LE dispatch pointers. [flow-check.json](flow-check.json) validates all 49 distinct targets against exact supported instruction starts; all handler bodies show coherent stream-operand reads, channel RAM/APU operations, pointer loops, branches and return contracts. Tokens subtract $90 and could arithmetically select beyond49; full valid token domain and natural selection of every handler remain unresolved. The physical pointer storage and individual static routine contracts are sufficient proposals without claiming that stronger domain. Explicit one-frame discards ($9E3A, $9F0B/$9F81 family) and three-frame discards ($A05D and recovered command bodies) are terminal local stack operations, not ordinary fallthrough. Every proposed instruction has a supported accepted-CFG, dispatch or independently stated static-contract root; 2428 proposed instructions, zero uncovered by the mechanical closure check.

Other non-code support includes consumer-encoded/terminated OAM templates, digit tile lookup, RAM audio-stack pointers, overlapping bit masks, descending LE note period words from initialized origin $9859, and repeated reset-transfer vector words. Note-period role follows actual initialization and WriteMusicPeriod semantics; no original equal-temperament generator or rounding algorithm is claimed.

## Independent R003 fixed investigation

Resolved menu OAM selection with three encoded origins plus four-byte copy count, score digit lookup with decimal consumers, lives template with encoded origin/terminator, and local stage marker pointer/coordinate storage. Generic data is sufficient where a full original schema remains uncertain.

$D8C0 follows balanced RequestGameplaySound and restores pause scrolling/latch. Its actual opcode $9E is **SHX abs,Y**, not STA abs,X; internal loop and incoming return support code, but implementation must retain exact-byte/mnemonic rendering and uncertainty. Do not silently repair the ROM. Fixed $FE03 is the supported return of ObjectSoundBridge; following JSR/RTS service wrappers have supported fixed routine argument/stack contracts. $FEF8/$FEF9 alone is the directly consumed frame-hook pointer; the preceding four bytes are not generalized into that claim. ASCII title physical131056..131072 is non-code text; following ten mixed bytes remain separate.

## Narrow residual review and actual limits

[residual-review.json](residual-review.json) preserves exact 19 bytes as five ranges and scans for literal LE references. This is a compact local uncertainty review for a ready-proposal handoff, **not a terminal exhaustion assessment**.

- physical8714/$A1FA: one $FF between bounded object handler pointer storage and independent $A1FB handler. A lone separator versus overlapping unofficial instruction cannot be discriminated by the local pattern. No encoded pointer/reference found. A positive independent record/sentinel contract would change this; none identified now.
- physical13913..13916/$B649: CMP #$DF; RTS after previous RTS. Legal decoding alone is insufficient for this isolated fragment; no caller, dispatch pointer, duplicate implementation or longer contract found. The following independent 18-byte resolver was rescued rather than inheriting whole-gap rejection.
- physical105610/$9C7A: single $00 after unconditional token-loop JMP and before independent $9C7B entry. Could be separator or BRK. Literal $9C7A hit at21625 is inside classified stage data at21226..22698 (repeated map bytes), not bank-aware code/reference evidence.
- physical130820..130824/$FEF4: two pointer-shaped words $FB6F/$FBFF lack a consumer or independent format. Known last word $D1EC is separately proposed at130824..130826. Do not infer all three words are one table.
- physical131072..131082/$FFF0: ten mixed trailer bytes. Non-code ASCII prefix is separately proposed; remaining bytes do not inherit that pattern. Literal $FFF0..$FFF2 hits are FF/F0 stream/template data coincidences (one in independently known OAM template at120759), not bank-aware code targets. No supported entry or consumer identified.

No concrete productive continuation identified for these tiny residuals with currently supported contracts: arbitrary state sweeps or treating pointer-shaped bytes as proof would not discriminate the hypotheses. They remain reviewable after independent acceptance/rejection of this batch. Post-aircraft next-loop gameplay was previously unobserved; this static batch resolves actor transitions without requiring repeat gameplay, and no remaining local residual has a credible connection that warrants unchanged replay.

## Verification and handoff

[checkpoint-identities.json](checkpoint-identities.json) confirms current state, maps/current and pass-001-final all hash e03b78197b212592dc77dc59832daa4ce172f26e09e0e067218d93a53bed440c; source retains expected hash8292800b9b7ca4fad4f5e65d9f3d8b2e8098c225ace6c07ad771ac34b55448fb; ROM retains expected identity. Exact proposed instruction ends validated independently with configured toolkit. This is not assembly verification of accepted changes (none made), and does not replace independent disassembly review.

No toolkit issues found: zero bugs, suspected bugs, missing features or blockers. Disk at handoff:3.5GiB available; attempt7.1MiB. No cleanup, archive, publish, toolkit edit or global completion action. Next phase: independent disassembly on combined ready R001/R002/R003, then full current-source commentary and redirect.
