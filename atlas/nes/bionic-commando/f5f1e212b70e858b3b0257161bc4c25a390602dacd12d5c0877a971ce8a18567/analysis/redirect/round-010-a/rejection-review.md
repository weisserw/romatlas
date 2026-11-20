# Candidate-specific terminal rejection review

Fresh primary evidence: [ROM bytes, exact current map neighbors, effective execution views and source excerpts](primary.json), [readable primary extract](primary-readable.txt). These are observations; role claims below remain hypotheses. Rechecked actual [counter/vector decode grids](../round-006-a/target-grids.json), [independent accepted counter/branch contracts](../../evidence/round-007/review.md), [original renderer bytes and88-byte copy](../round-009-a/renderer-copy.json), and [allocator/JMP/template occurrence artifacts](../round-009-a/template-tests.json). Original rejection scope is localized, including the now accepted CLC/ADC; no live-caller, whole-routine, whole-RTS or data-schema prerequisite is imposed.

## 2E810+9 (4cffff4cffff4cffff); CPU $A800, physical8KiB bank23

Actual claim: Three placeholder JMP $FFFF slots. Observation: Three identical 4CFFFF records between FF run and resource shelf; real four-entry bank22 vectors start separately at 2E827.

Repetition supports a three-slot object, but is equally specific to obsolete instruction stubs and encoded sentinel records. $FFFF target has no established return/body contract. Generic data is not established by uniform surrounding FF. No supported subdivision removes the same ambiguity.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 34231+3 (4c0d82); CPU $8221, physical8KiB bank26

Actual claim: Isolated JMP $820D. Observation: Preceding live arm ends JMP $81BA; following entry begins LDA #$81. A second 4C0D82 exists at 34209 inside a coherent death-position arm.

Destination is genuine code but the second occurrence supplies no common wrapper/displacement or dataflow. Tail-transfer semantics alone fit any three-byte pointer-like residue. Neither forced entry nor searching the same two occurrences again distinguishes the alternatives.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 34DFC+1 (60); CPU $8DEC, physical8KiB bank26

Actual claim: Isolated RTS after allocator tail-jump. Observation: Allocator A001 A903 4CEED6 ends immediately before 60; following collision entry starts A901.

Allocator already returns through its tail-call. This byte adds no required return path. No allocator duplicate corroborates a retained instruction; a separator/resource byte remains possible.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 359D3+1 (60); CPU $99C3, physical8KiB bank26

Actual claim: Isolated RTS after impact-handler tail-jump. Observation: Previous timed-impact path ends JMP RemoveBothTypeFiveImpactObjects; next state dispatcher begins ObjectState load.

No fallthrough from terminating JMP. Duplicate return shape does not add control/stack consistency to either routine; isolated return and numeric residue remain equally local explanations.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 38AF8+1 (60); CPU $8AE8, physical8KiB bank28

Actual claim: Duplicate RTS between actor handlers. Observation: Previous movement-delay arm already returns at physical38AF7; next state dispatcher begins at38AF9.

No branch needs a second return and no indexed resource contract reaches it. Separate no-op return and boundary residue both fit; similar singleton gaps elsewhere are not independent producer/extent evidence.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 39319+1 (60); CPU $9309, physical8KiB bank28

Actual claim: RTS before grapple lookup shelf. Observation: Previous sprite routine restores X and returns; accepted indexed arrays begin next byte at3931A.

Neither existing restore/return path nor bounded lookup offsets require this byte. Extending either code or data from adjacency alone would choose an unsupported role.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 39EDE+1 (60); CPU $9ECE, physical8KiB bank28

Actual claim: RTS after coordinate tail-transfer. Observation: Previous helper tail-jumps ConvertCoordinatesContinuation; accepted direction-offset table begins39EDF.

Tail-call return does not flow here, and known four-field table origin is next byte. A standalone no-op entry and resource residue both fit without discriminating evidence.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 3B612+4 (4c329caa); CPU $B602, physical8KiB bank29

Actual claim: JMP $9C32 plus TAX adapter or resource suffix. Observation: Consumer-backed lookup at3B60F has exactly values24,76,00; next dispatcher3B616 loads ObjectState,X. No second JMP9C32 copy.

Split JMP3/TAX1 considered. JMP cannot fall into TAX; TAX can prepare slot X only by assuming incoming A has that meaning. No producer, matched adapter or record extension selects those claims; generic data also lacks a compelling pattern. An arbitrary precondition is legal decoding, not an independent contract.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 3D62A+1 (1d); CPU $D61A, physical8KiB bank30

Actual claim: Copied orphan LDA operand. Observation: Current byte1D lies after accepted X-position valueC0; original3D00B is B11D, long88-byte copy begins original3D00C/candidate3D62A. Accepted CLC/ADC follows.

Strong evidence establishes copied operand origin, not a complete present instruction or a current noncode role. MissingB1 cannot be repaired; ORA abs,X would consume accepted18/65 arithmetic. Treating every copied original operand as current Data would conflate historical origin and present role. No smaller subinterval exists.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 3D6AF+1 (b5); CPU $D69F, physical8KiB bank30

Actual claim: Truncated copied LDA zp,X opcode. Observation: Copied counter source has B590 at original3D091; candidate B5 is followed by independently accepted JMP opcode4C.

Historical code origin plausible; present B54C would load unrelated zero-page4C and overlap established vector. Earlier accepted indexed91/mask contract ends before it. No complete instruction contained in singleton and no noncode pattern.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 3DFD3+1 (60); CPU $DFC3, physical8KiB bank30

Actual claim: Duplicate RTS before object-memory clear. Observation: Page-window test already ends RTS; next clear routine starts health LDA/PHA.

No result/stack operation or control edge requires second return. Isolated return vs residue cannot be settled by forcing PC or assuming compiler padding.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 3E010+2 (0035); CPU $E000, physical8KiB bank31

Actual claim: BRK35 trap prefix versus numeric/overwritten bytes. Observation: Previous type scan compares2A/2B; next accepted BEQ and exclusions64/86/6E/2E join decrement loop.

Neighbor scan/filter consistency supports suffix, not BRK ABI. BRK consumes padding and redirects via IRQ; no trap handler/service contract connects35 to scan. Split00 or35 has still less structure. Failure of conventional scan flow rejects intact loop only, not all code in area; accepted suffix retained. Neither a two-byte numeric run nor BRK shape proves generic data/code.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 3E9B0+1 (17); CPU $E9A0, physical8KiB bank31

Actual claim: Arithmetic prefix/orphan17. Observation: Preceding bank-call wrapper terminates JMP restore; accepted next byteA2 begins two-pass48bit shift.

17 is unofficial SLO zp,X and needs operandA2, consuming accepted LDX opcode. Could be orphan operand, numeric residue or overlapping obsolete code; shift semantics establish no prior operation. No complete supported singleton instruction/noncode pattern.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 3E9EF+1 (85); CPU $E9DF, physical8KiB bank31

Actual claim: Dangling STA zp opcode. Observation: Accepted24bit carry-add leaves high result A; byte85 followed by accepted JMP4C0080 vector.

Arithmetic contract predicts a store but cannot supply original destination. Decoding854C would consume independently anchored vector and store to unrelated4C. Original-source damage vs data residue unresolved; requiring a functioning whole routine was not used.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 3F448+1 (60); CPU $F438, physical8KiB bank31

Actual claim: RTS between motion descriptor and Y setup. Observation: Five-byte descriptor ends before60; accepted A0E0 setup follows. Earlier initializer returns before descriptor.

Descriptor consumer reads five bytes, so does not require this sixth byte. Calling singleton60 is plausible but unsupported by additional boundary/control constraint. Could be old return after embedded constants; no justified current data extension.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.

## 3F9E5+1 (60); CPU $F9D5, physical8KiB bank31

Actual claim: Duplicate RTS before room-event dispatcher. Observation: Area-to-four-way mapping branches to existing preceding RTS; next ObjectState dispatcher begins next byte.

All area comparison arms already return one byte earlier. Extra return has no needed stack/control role; event table and mapper do not include it. Same singleton ambiguity remains without a useful discriminating runtime test.

Strongest alternative is the opposing code/data or overwritten-fragment interpretation named above. Missing control evidence alone does not reject static code; here the positive local constraints fail to discriminate those interpretations. Existing context-copy checks, current neighbor bounds and consumer contracts were combined. Further forced execution would validate only imposed mechanics; unchanged reference/template searches or arbitrary states provide no useful revised test. Retain Unknown.


CORRECTION:0035 rejection and terminal recommendation superseded by [BRK/RTI primary correction](brk-correction.md), ready2-byte proposal in proposal.json. All prior records retained for history.
