# Round 1 - disassembly phase handoff

Scope: fresh workspace. Established verified baseline, broad gameplay traces, observed+static code,
observed-read / uniform / text / dense-non-code data. Single pass, finished (easy gains taken); ROM incomplete.

## Output checkpoint
- maps/round01-final.json (map sha256 b4a8fa61eed48625a58da3d0af895c73cb61bce2d3ff43eae33f5868a6c437e2)
- generated/round01-final; analysis/round01-final-verification.json -> ok:true, byte_identical:true
- Internal store analysis/state.json == maps/current.json content (map.save/map.export after final edit).
- Baseline: maps/baseline.json, analysis/baseline-verification.json (byte-identical).

## Classification
ROM classification: 83.73% (109762 / 131088 bytes classified; 21326 bytes unknown).

Per bank (code / data / unknown): header 0/16/0; b0 11379/2880/2125; b1 448/13694/2242; b2 1021/13252/2111;
b3 173/14799/1412; b4 103/16052/229; b5 673/9912/5799; b6 1733/9705/4946; b7($C000 fixed) 9156/4766/2462.
Provenance bytes: Code observed 19,940, static 4,746; Data observed-read 45,190, static 39,886.

## Findings
- Game: California Games (Epyx/Rare/Milton Bradley), UxROM. Bank 0 = main switchable "home" bank;
  bank 7 fixed: NMI $FF88, RESET/IRQ $FFB0; idle loop jmp $F27E. Controller read at bank 0 $85D2 ($27 raw, $0B/$0C).
- BIT $2C skip idiom (overlapping entries) at 0x87FA.. and 0x1F2D0.. ; cross-window instruction at $BFFE->$C000.
- RAM code executed at $0310-$0485, image at ROM 0xFC54-0xFCD9 (bank 3); default high-score table at 0xFCDA.
- Read evidence caveat: dummy reads and sound-engine reads through mis-banked pointers make short read runs
  unreliable (0x3A2F example); only runs >= 8 bytes were used.
- Tools: analysis/tools/{rpcbridge,rpc,play,closure,m6502,annotate_code,readscan,data_plan,annotate_data,dense,annotate_dense,stats}.py
  (bridge socket /tmp/ca_rpc_8e32.sock; NES input needs `buttons` arrays).

## Scenarios (analysis/scenarios.json)
Visited: boot/title, main menu, practice list, all 6 events (half pipe, foot bag incl. summary, surfing,
skating, BMX, flying disk incl. throw recipe), high scores, compete registration, trophy results screens.
Attempted: compete circuit ending (looped back to half pipe). Deferred: compete-some, ending/credits.
Durable snapshots (same build, local): scratch/snap_{title,menu,practice,practice2,r7,fb2end,trophy}.json.

## Leads
Consumed: none (fresh). Produced: L-R1-01 (blocked, toolkit TK-04), L-R1-02 (verified), L-R1-03..08 ready
(RAM image mapping, jmp ($002D) dispatch tables, short-read runs, ending RAM shortcut, residual gaps / stronger
non-code test, sound tables $D745/$DB6D). Details: analysis/leads.json.

## Toolkit issues (analysis/toolkit-issues.md)
TK-01 input button/pressed silently ignored (confirmed); TK-02 discover budget/cpu_address doc mismatch (confirmed);
TK-03 UxROM static discovery stops at every JSR (missing feature); TK-04 cross-window instruction (missing feature);
TK-05 tracing ~13x slowdown (suspected limitation). None blocking.

## Unresolved / deferred
- 21,326 unknown bytes in 784 gaps (traces/round01-unknown-gaps.json); many small mixed gaps, some likely
  unexecuted code (e.g. bank 2 0x9F1A after 8x$FF), some data with long accidental decode chains.
- Static data regions rely on the K=8 non-code test (dense.py); redirect may audit samples.
- Ending not reached; event/round RAM contract unknown.

## Status
Pass finished (easy gains exhausted for this round). No resumption needed; next: commentary/redirect.
