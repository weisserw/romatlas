# Round028 independent disassembly review

ROM classification: 100.00% (65533 / 65536 bytes classified; 3 bytes unknown).
Rounded percentage does not establish completion. One disassembly pass complete;
whole-ROM classification incomplete. Accepted R27-RUNTIME0-INLINE-ORIGIN-EMPTY-HANDLER:
physical3420 length1 Static Code, fixedROM bank0 CPU3420. No natural occupiedtype0
or accepted instruction fetch claimed. [Primary review](primary-review.json) records
fresh configured-release ROM reads/decode and preannotation map ownership.

Independently checked CALL3340→33a8 leaves3343 on stack.33a8 loads1;33aa calls32dc
which reads actor field1 through saved c416/c417 pointer and returns normally.
Occupancy test at3300 concerns field0 and does not exclude a supported occupiedtype0
precondition. RST00 at33ad pushes33ae; POPHL at0000 removes exactly that word.
RST08 at0001 pushes0002. ADD A at0008 with type0 clears carry; JR0009→0013,
JRNC0013→000b; RST28 adds zero to33ae. Reads000c/000d obtain20/34;000f returns0002.
JP(HL) selects3420 retaining3343; C9 returns3343 to axis integration. This is a
complete ordinary return contract under the stated type0 precondition.

Inline origin independently anchors pair33ae..33af, beyond arbitrary pointer search.
The independently supported pointer0 interpretation is recorded here without changing
its existing Static genericData annotation or claiming the whole57-word directory.
Accepted six-byte wrapper-pointer region341a..341f ends before3420; separate Static
wrapper region3421 begins LD B,2/CALL3444, then its mapped handler. RET provides the
exact one-byte endpoint. Diagnostic decoding through inline words is not Code evidence.

Strongest alternative is reserved/stale zero-index storage for a nonzero-only runtime
domain. Mechanically possible, but consumer-defined origin, exact fixed-ROM word,
retained outer CALL return and endpoint together discriminate an empty handler more
strongly than opcode/pointer resemblance alone. Neither natural selection nor a
whole-directory proof is necessary for this narrowed structural claim.

Connected caller, actor field reader, horizontal/vertical continuations and bank2
wrappers already classified; no additional connected byte gain. Three isolated C9s
remain Unknown:2f09,34af,ad6a(bank2CPU6d6a). Reused scoped primary-evidence review in
../redirect/R27-20261027-origin-and-epilogue-v1/assessment.md and primary-audit.json:
all accepted tail/RET analogs have independent entry; no detached epilogue generation
contract. Prior tails preserve original caller return rather than entering these
bytes. Encoded-pointer absence excludes that mechanism alone. Redundant epilogue and
stored byte remain concrete alternatives; generic non-code inference also unsupported.
No productive revised test, artificial RET call or unchanged replay pursued.

Broad Round026 initialized stages1..4, ending and visible STAFF credits remain reusable
in ../scenarios.json and ../gameplay.md. Boss and final held-A milestones/natural late
game reachability remain unclaimed; no changed residual-linked gameplay route. This
is an easy-gain phase handoff, not final editorial or terminal archive acceptance.

Per-bank Code/Data/Padding/Unknown:0 10280/6029/73/2;1 8413/7904/67/0;
2 2992/13354/37/1;3 227/15468/689/0. Structural annotation65533bytes; execution
coverage unchanged and separate. Fresh configured-release map validation/conflicts,
save/export and round-028-disassembly checkpoint pass, byte-identical65536bytes.
[result](result.json),[final verification](final-verification.json). ROM/config/release
and every commentary file protected hashes match. Prior input state/map copied here;
history untouched. No commentary edits or toolkit changes. Existing confirmedT001
only;zero new findings,no blocker;[report](../toolkit-issues.md). Disk14GiB free;
no bulk scratch directory. All internal state/source/evidence remain local.
