# Round 1 commentary — phase complete

All currently classified source has been inspected and brought up to the current naming/comment standard. The complete1436-interval scaffold is reviewed:1053 Code,33 Data,1 Text,1 Image and348 Unknown intervals.1073 routine/structure inventory units retain physical extents and execution views. No classified-source readability backlog remains; pending-ranges.json is empty. Unknown bytes remain unknown. Whole-ROM classification is still incomplete at65.50% (171722/262160;90438 unknown). Final editorial acceptance is reserved; editorial_audit remains pending and no final audit was run.

Input and unchanged current map/state SHA256:d8c5f7e719b59cb825ed5679edfa957367d48bd91ecf98947d097bf4b26e95c7 (pass-004). ROM SHA256 remainsa2ae47c68eae003698dfee57ab29e64560785e1637699e05ce99d913e646ac39. Fresh commentary verification:ok=true,byte_identical=true; final game.asm SHA256:9d44b872ef550cee9393431169e2a2a5b48161893132fe80ee7d4ce797770989. Reading-after.asm was refreshed from that source. Source/record links were checked without invoking the reserved final acceptance audit.

Scope includes every classified code slice in all populated PRG banks, header/CHR and data formats. Contracts cover reset/NMI/controller latches, MMC1 writes and bank context, party/inventory and actor records, transition hooks, hero action/animation, tile/attribute rendering, menu/list/message/portrait/health bars, introduction/title/fades and audio bytecode/channel ownership/DMC. Added629 internal entry/branch labels; no numeric conditional branches remain.121 opaque or bank-switched call operands intentionally retain encoded CPU addresses with local contract/uncertainty prose. Named RAM, hardware, shared fields and audio-record offsets are used in operands; CPU data aliases are distinct from physical incbin labels.

Important corrections: $FF2A projects actor columns to screen X with carry as visibility result, rather than reading cell flags. $E5FD's inner JSR returns into the same loop and therefore emits288 bytes (32+256), not32. Fade-out completion loops overwrite their accumulator and test the final nibble, rather than proving every palette byte is black. Descriptions preserve these actual mechanics without inventing intent.

Unresolved meanings are local, not skipped source: creature/animation names, many cell/category bits, spell/item selectors, precise resource units, artwork/tile identities, sparse raw callee continuations and audio music/sample identities. Complete unknown intervals retain boundary notes. Numeric banked calls do not assert an unsupported target bank after mapper-affecting helpers. See uncertainties.md for the map$2B animation-boundary/state$11 lead and consumer-table questions for the next redirect phase.

No toolkit issues found; no toolkit edits, archive, cleanup/deletion, publication, ROM patches or structural map changes. Source/assets/traces remain local. Writer quiescent after final handoff.

## Historical preparation and batch records

The earlier counts/hashes below are historical checkpoints, superseded by the complete scope and final source hash above.

# Round 1 commentary — active, incomplete

Input: frozen pass-004, map SHA256 d8c5f7e719b59cb825ed5679edfa957367d48bd91ecf98947d097bf4b26e95c7. Full current-source review remains active. No redirect or final audit has run.

Completed: every non-code scaffold interval (348 unknown intervals, 33 Data, one Text and one Image) now has a local boundary/format/uncertainty note. Unknown bytes remain unknown. Generic data uses prior bounded boundary/reference inspection rather than invented formats. Header, CHR, party/health/auxiliary tables, transition hooks, debug tile table and flag retain useful established names and gain format/consumer prose.

Reviewed code: fixed-bank reset/NMI/controller initialization through physical $01C212, plus selected arithmetic/MMC1/PPU helpers physical $01E94D..$01EBF4. This is a discontinuous set; review.json, not this shorthand, identifies exact reviewed intervals. Shared hardware registers and well-supported controller/OAM RAM bytes are symbolized with explicit addressing widths retained. Bank-select values are distinguished from effective 8 KiB indices. Inline table dispatch, finite controller resampling, 64-sample repeat, optional OAM DMA, multiplication/scaling and sprite-zero synchronization now have contracts.

Remaining:256 Code fragments /6551 bytes, exact intervals in pending-ranges.json. Continue banks five and six; complete branch/internal-entry naming before phase handoff.

Verification after coherent batch: commentary/verification.json ok=true, byte_identical=true; source game.asm SHA256 49a75228215012d8a4eb784438d917830bff77de7558a455bceb757454f12341. Reading copy is not yet refreshed after these edits; refresh at completed phase handoff, or at any interruption. Final editorial audit is reserved. No toolkit issue encountered.

### Second coherent source batch
Inspected 146 additional fixed-bank fragments spanning CPU $C234..$CD47 (physical $01C244..$01CD57; sparse gaps excluded). Main-menu Select path, modulo-eight work scheduler, party refresh wrappers, transition graphics setup, hero metadata pointers and counted composite-sprite format now have local contracts. Shared main/substate, frame, party, button latch and sprite scratch operands are symbolized. Pending Code fragments:866, exact list refreshed in pending-ranges.json. Fresh verify:byte_identical=true, game.asm SHA2561837f5782955592888a5a10e7d664733c90bdf195eb381ca6112933a517161fe. No final audit or redirect.

### Third and fourth coherent source batches
Reviewed another218 fixed-bank fragments. All currently classified fixed-bank Code is now inspected (physical $01C010..$01FF70, unknown/data gaps excluded). Actor record load/store, hero control/action dispatch, transition destination/entrance, room pointer setup, column tile/attribute buffers, palette output and save-checksum handling are described. Corrected the earlier provisional $FF2A interpretation: it projects actor column to screen X with carry for visibility; it is not a cell lookup. Added internal entry labels for routines packed inside annotation slices. Confirmed a non-obvious $E5FD JSR/return loop writes288 bytes, and documented it without inferring original intent.

Pending:648 Code fragments /15753 bytes, all switchable-bank code; exact physical intervals are in pending-ranges.json. Latest fresh verification:ok=true,byte_identical=true, game.asm SHA256a1a6a8588d10ed56bb5dba7164d603225dd475c55784b79a1277f38e405e77e6. Next inspect switchable low banks and then menu/audio banks.11GiB free/scratch432KiB at last check. No final audit/redirect; no toolkit findings.

### Low-bank, actor and menu batches
Reviewed40 low-bank,136 actor/effect bank-two and216 menu/status bank-four fragments. Party initialization/inventory scan, scroll phase, actor sequence/render/motion records, damage/interval comparison, menu input/list/text/highlights, party grid, status portraits/bars and local support helpers now have mechanical contracts with uncertainty. Full classified Code in physical banks0/1/2/4/7 is inspected; banks3 has no classified code despite promising raw debug-display evidence. Unknown table/gap bytes remain unknown.

Latest verification:byte_identical=true,source SHA2561416d8ced04d91a18981fe29e7199988aadfb67b44a8c46829259950a5a42601. Pending256 fragments/6551 bytes in banks5/6 only. Continue current phase; final internal-entry/branch symbol consistency and fresh handoff reading remain.
