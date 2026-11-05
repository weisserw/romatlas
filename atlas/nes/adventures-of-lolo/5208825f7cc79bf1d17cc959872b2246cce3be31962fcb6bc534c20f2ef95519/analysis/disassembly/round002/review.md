# Independent round 2 structural review

Input frozen pass-001; reviewed primary ROM bytes, fresh toolkit decoding, contracts.py/safe.py algorithm and saved graph, consumer references and format records. Research graph alone is not acceptance. Every 3752 proposed instruction was freshly decoded with physical offset and effective PRG[0,1,2,3]; exact assembly and length agree. decode-review.json records this check. These are Static structural claims, not new fetches or natural reachability.

## Code and connected batch

The return-summary worklist rejects underflow, unequal RTS depth, indirect exit, TSX/TXS and arbitrary mapper writes; nested ordinary-return and inline2 summaries provide supported continuation. Existing roots, 21 dispatch roots with coherent object-slot/state mechanics, and six independent helpers anchor the accepted graph. Generic D9D9/DA23 argument effects and C0D9 indirect selector exhaustiveness remain barriers. Tail calls preserve the caller stack contract. Branches are followed without requiring observation.

Independently inspected DCCD..DD03: scroll/control construction ends with LDA$44; three LSR; ORA#$1e; tail serial helper. Low five bits are $1e/$1f; PRG mode3 and register0 preserve slots[0,1,2,3]. CHR serial helpers D9ED/DA08 save A, lock/unlock via D9AE/D9B8, only write BFFF/DFFF; they preserve PRG. These summaries do not authorize arbitrary mapper calls.

B561 uses B631 grid index, four graphics arrays, two-by-two RAM packet and DA5A inline pointer before RTS. B7AE bounded scan12..DC transforms cells then tail-joins B774. D769 computes attribute pointer/quadrant, uses D830 clear mask and restores X/Y; its shift loop assumes A<4 and supported coordinates. DBBA copies queue reader to writer then returns. E3A0 sign-extends five bits with DF6C mask10, independently parallel to E391 four-bit helper. D8C8 calls DA54 with inline D8CE source then returns atD8CD. These contracts establish code without inventing callers. D769..D7B2 is excluded from sprite data.

DB71..DB92 is height-counted column transfer: packet byte writes through DBE3, source width stride except fill, DBEF cursor publication; outer DB93..DBB9 restores source/X and returns. Inline DA54/5A/60 and D8DE contracts skip exactly two bytes. No inline pointer is decoded as a continuation.

## Data and exact bounds

Room metadata 50*2+50+50 independently matches loader and clear threshold32hex. All 50 saved record decodes produce15*16 cells and consume exact recorded length. Row destinations15..239 and five consecutive240-entry arrays fit literal<240/reuse/substitution domain; B606 values176..182 remain within it. Graphics output bytes can use full8-bit tile IDs; a draft review check erroneously constrained outputs<240, failed, and was removed because that is the input-index constraint. No data proposal failed this check.

Audio parser opcode dispatch and note duration consumers were checked against recorded raw events and pointer operands. 34 counted channel descriptors and3960 grammar events have consistent extents. The five isolated FF gaps are generic noncode separator bytes, not executed stops. Fresh extraction of F2 parameters is[0,1,5,7,8,9,10,11,12]; E089..E096 multiplies by6 and E029..E042 adds durationclass0..5, max77, proving78-byte FE47..FE95 slice independently of F6 modulation selectors. Paired modulation descriptors, music/effect arrays, masks and priorities terminate at adjacent independent consumer bases. E3C3 decreasing 16-bit periods and E459 increasing16-bit values have clear noncode representation; the latter is generic data with uncertain meaning, not a recovered historical generator. RAM waveform pointers establish no ROM wave backing or hardware compatibility.

Sprite independent raw-byte check:119 fixed four-byte records pass attribute reserved bits,49 split pointer pairs produce exact contiguous count+4*N union D2FC..D769. Pointer values alone are supplemented by CF75/CFCD/D23A consumer contracts and independently established neighboring code.

Video records use positive dimensions and exact header+payload products; 67 PRG targets count as data,3 RAM targets do not. Palette seeds have32-byte D8DE consumer bounds. Shared record overlaps are unioned once; previously classified bytes are preserved.

Residual-reference-review.txt and actual consumer graph checked for bounded short lookups and aliases. AE18..AE1C 08 01 04 02 has no reference; it is generic four-byte directional mask pattern adjacent to repeated signed movement arrays, with no claimed live selector. Short isolated FF audio separators and the BFD2..C000 / FF09..FFFA tails are generic noncode, after reference/neighbor inspection. Raw tails contain repeated FF/00 and sparse bitplane-like masks, not a coherent supported instruction/stack body; no fill, unused or original organization claim. Mapper writes are not ROM data references. Specific semantic uncertainty does not block code-versus-data classification.

## Local residuals / rejection scope

Physical8938 length2 (CPU A2DA14 3C) follows infinite self-jumpA2D7, before consumed packetA2DC. This removes ordinary fallthrough only; bytes could be two constants or unofficial NOP$3C,X. No independent role/extent/entry contract discriminates. Physical17417 length1 (C3F9=60) follows tail JMP CBE9 and precedes distinct handlerC3FA; literal, empty routine or redundant trailing RTS remain concrete alternatives. No evidence beyond opcode shape supports code or data. Retain these3 bytes unknown. This is an ordinary pass handoff, not terminal exhaustion or final editorial audit.

Accepted L005-L011 supported connected union23238bytes (8135code,15103data), no rejected proposal interval. Reuse prior natural room1/artificial room25/50/ending scenes and scenarios; no replay answers a concrete new structural question in this pass. All source commentary/history preserved for next phase refresh.
