# Round 1 redirect: combined structural research

Input: pass-001; current commentary gate complete. Attempt r1-20261105-a.
Accepted map remains218781 classified/43379 unknown. Proposals are NOT annotations:
[proposals.json](proposals.json) contains43371 proposed bytes (15227 code,28144 data),
leaving8 only IF independently accepted. Graphs include research hypotheses; this
file and corrected versions below govern review. No new checkpoint or rebuild is
needed for unchanged accepted state; pass-001/current commentary verification applies.

## Method and evidence limits

Read immutable ROM, documented NES decoder/discovery, source, accepted map and
current uncertainties. Toolkit discovery conservatively stops at calls and many
indexed RAM writes; bounded custom graph traversal resumes these from supported
contracts. Legal decoding alone is never the evidence: finite dispatch domains,
matched frame/behavior domains, coherent actor field operations, reference/consumer
contracts, nonlocal stack effects and bounded record syntax supply the independent
structure. Graphs' call-return fallthrough is a hypothesis, NOT proof all calls return.
Do not map graph files wholesale. Review proposed exact instruction starts and
local contracts; preserve data barriers. All proposed code is Static, even where a
runtime experiment visited the same system. No natural reachability is inferred.

`package.py` intersects candidates with exact accepted unknown complement, preserves
all accepted annotations, verifies renderer copies byte-for-byte, rejects code/data
conflicts, and checks overlapping instruction starts bytewise. Current conflicts=0.
This is consistency evidence, not structural proof or an accepted near-completion claim.
`unknown-ledger.json` inventories the exact unchanged43379-byte complement with bank,
canonical CPU alias, neighbors and proposal/disposition counts. Canonical aliases are
bookkeeping, not execution context. Proposed regions carry explicit processor views.

## Bank/control contracts and corrections

- Screen code/resources use slots[10,11,14,15]. C008 maps Y/Y+1 via MMC3 registers6/7.
  Calls with Y=10 establish screen pair; E08E writes register7=13 to establish gameplay
  table slot. FDA1 demo helper later maps10/11 again before returning to screen code.
- C5C6 actor tick maps12 into slot0 only for frame lookup, advances animation and
  stages pointers, then restores LevelResourceBank before behavior dispatch. Exploratory
  `actor-connected-graph.json` used[12,13,14,15] to decode bank13/fixed code and also
  encountered a bogus8000 KIL via a later screen transition. Its bank12 target is
  EXCLUDED. Its bank13/fixed proposal views are normalized to representative supported
  stage0 context[0,13,14,15]. Never use exploratory slot12 to infer behavior data reads.
  Actor rendering itself uses[12,13,14,15]; level renderer copies use[0/2/4/6,13,14,15].
- `mapper-fixed-continuations` resumes inspected reg6 slot0 changes while retaining
  slots13/14/15; C008 pair helper is a boundary, and CPU< A000 code is excluded there.
  Inspected examples: C7AF/CD2E/CDA4/B714 select#6; subsequent data writes map12 or
  LevelResourceBank. E090/E095 and E16B..E17D explicitly select7/write13. E9F3 selects6;
  EB3F selects6; F521..F57F temporarily map12 then restore level; FA6A..FA7A switches
  tile/collision resource banks and restores level. E000/C000/PPU/APU stores do not
  imply PRG changes. Low-slot data references need their OWN effective-bank proof.
- Screen IRQ9873 is installed by source98C4 storing73/98 to IRQ pointer. Its MMC3
  select1/data54 changes CHR only. Correct `screen-irq-v3` resumes9891,9894,989C;
  v2 accidentally used physical suffixes as CPU addresses and is EXCLUDED.
- `source-prefix-closures-v2` is authoritative. First exploratory version accidentally
  rooted B29B/E57D/DC20 inside instructions; it is EXCLUDED. Correct B29C is a current
  accepted source instruction entry; DC26 is a coherent short timeout-setting/nonlocal
  abort sequence, E58B a self-contained player-to-selected-actor Y convergence loop.
  Neither corrected standalone contract claims a natural caller. Original failed
  graphs remain as history and must not be replayed/annotated.
- `residual-inspection.json` originally gathered references by CPU alias only. Its
  reference arrays are TRIAGE ONLY and include cross-bank false matches. Its bytes
  and physical ranges are sound. Confirm references using named graph banks and
  source; e.g. audio8386 is not a screen8386 consumer. No proposal depends on an
  alias-only reference. Body bytes and accepted explicit views are authoritative.
- PLA/PLA/RTS and four-PLA aborts are intentional in frame/collision, title and
  transition helpers. Inspect callers, do not treat them as conventional returns.
  Source-prefix and queue closures recover actual source call fallthroughs separately.
  Scan found no TSX/stack-page return-pointer reader or inline-argument consumer in
  selected graphs; still preserve exact table barriers and nonreturn contracts.

## Screen graph and data (R1-L03)

8027..803D is11 words, independently bounded by803D title entry; fixed callers
E050/E055/E058/E06A/E08B/E155/E15D/E259/E25E/E27C/E289/E28E select0..10.
Targets in tables.json cover mission/title/logo/loss/ending/score/bonus modes.
Followed foreground handlers, bonus numeric carry helpers, ending scrolling,
NMI screen commands and IRQ; source missing targets8172/839E/8D60/8D6B/8E19/994B
were decoded with their actual callers. `screen-connected`, `screen-nmi-connected`
and corrected IRQ graphs are the reusable recipe, not another runtime sweep.

Resource consumer9682 selects11 tile pointers973F..9755, bias/CHR columns9755..9776,
attribute pointers9776..978C, palette pointers978C..97A2, OAM pointers97A2..97B8.
`screen-data.json` parses ALL11 address/run/FF tile streams and OAM streams; attribute
copy is64 bytes; palette resources have32-byte blocks consumed by fade code. Tile
runs have a two-byte PPU address; FE switches to text minus20; FF ends run/stream.
OAM payload lengths are multiples of4 (or0) and sentinel bounded. Tables/strings,
subtitle words891E..8993, title animation masks8386..839E, sprite movement90CA..9164
and entire generic resource/animation tail99BB..B342 have no competing code evidence.
Where exact animation schema is unresolved, propose generic non-code, not a fabricated
specific table. Ending text91B1..9581 is975 printable bytes plusFF; consumer95FD
advances rows32 bytes and sets0716 onFF. Source subsequently enters concrete IRQ code.
Obvious connected consumers/handlers were followed; no screen avenue deferred merely
because its first entry or initial range ended.

## Actor graph, frames and renderers (R1-L04, improved R1-L06)

Frame roots A599..A5B7, behavior roots A5B7..A5D5, inactive initializer roots
A5D5..A5F3 each have15 entries. Distinct frame-state and behavior-state array endpoints
agree for EVERY type, counts[37,4,6,1,1,9,4,2,3,2,2,1,3,4,1]. `actor-data.json` gives
all exact endpoints and contents.74 distinct sequences terminate FFFF followed by
replacement-state word; every replacement is within that type's independently
matched domain. Their155 distinct frame pointers all parse with11-byte header,
four-byte components atoffset11 andFF component sentinel, zero failures.
Referenced records cover6672 bytes; three gaps8C68..8CEC,9318..9334,9880..98E0
contain4 more exactly fitting frame records, preserving11+4*n+1 format and fields.
Together bank12 8000..9B10 is record data;9B10..9C00 is already accepted zeros.
No frame pointer is asserted to prove code. Record structure comes from consumers
C5FC/C62A and9DA4/9E73/9EDB plus exact syntax/adjacent endpoints.

Followed all15 inactive initializer targets and the matched behavior families,
movement, probes, contact, weapon/effect and guardian transitions, helper calls and
mapper continuations. No arbitrary choice of only two actors. Unknown natural
selection remains separate. `actor-renderer` closes both flip bodies and camera
projection; no mapper/indirect frontier in that graph. `level-renderer` closes
9C58/9D16/9E84 and associated constants for all4 identical banks. All cloned instruction
slices compare exactly and preserve separate storage identities.

Local occlusion routines9EB0..9EED in each level bank and9F7A..9FB7 in actor bank12
have coherent independent contracts: preserveX on stack, quantize screen+scroll to
nametable cell, combine row remap, inspect0600 grid, restoreX, then either ordinary
return onFF or discard one caller return address on nonFF. Level variant gates45;
actor variant corrects vertical carry with10. They have no direct caller found;
propose STATIC PARAMETERIZED CODE from full control/data/stack consistency, not
observed/live code. This evidence is stronger than the isolated RTS residuals.

A000..A72D bank13 is motion/state/frame/pointer/initializer/repeated-record data;
A72D uploader is independently anchored by source pause JSR at physical1B3EB. Other
fixed tables include C019..C1CC (stage pointers/banks/motion offsets), C427..C497
probe flags and short velocity/direction/HUD/glyph tables. Use specific supported
parts; retain generic non-code where precise original schema is not established.

## Audio (R1-L05)

`audio-connected` establishes complete consumer flow8000..841F with three vector
entries, no unresolved indirect/mapper frontier. It recovers25 initial unknown code
bytes;841E isRTS before frequency data. Reads841F/8474 select split note timers;
84C9 word tracks;856D timing pointers;8622/86A2 split128 pattern pointers;
8722/8742 envelope,8761/8769 pitch,8771/8779 duty,8782/87C2 note offsets;
8E3C four-byte instruments. Tables and encoded streams through ADCE are plainly
non-code after practical origin/consumer/pattern inspection. `audio-data.json`
gives origins/pointers, not an assertion every guessed table extent is authoritative.
The full surrounding domain is suitable generic data even where exact stream
packing or serializer layout is unknown. No generator or composer-format prerequisite.

Fixed EE07..EF06 is5 adjacent51-byte columns (priority plus4 channel tracks),
independently matched by EDB1 consumer and title sound-test selectors: music selector0..13 adds4 ->4..17; effect selector0..30 adds19 ->19..49;
startup/checkpoint command50 is also present. Column origins and adjacent code give
51-byte extent; values are00/FF or valid track selectors up to81. Track/timing82-word
candidates and all128 pattern pointers stay in audio banks. Full selector-domain
reachability is not required for generic non-code or structural column boundaries.

## Completion and representative progression (R1-L07)

`exit-test.py` imports final-guardian.snapshot.json, releases buttons, saves clean
snapshot, patches player Xlow/high0420/0440=00/04, Ylow/high0460/0480=08/02 and
PauseFlag0064=0. Source stage11 exit wordsFE24/FE3C give03F8/0200 with unsigned
window<32 on both axes.12frames reachesE26B completion fade with no PC edit;
372frames visibly WELL DONE. Replay input and budgets in exit-test.json; bank6/13/14/15
changes to10/11/14/15. This bypasses final guardian defeat artificially; it confirms
rectangle/completion mechanics, not natural defeat. Initial12frame coverage output
retained only first1000/2240 entries, so do NOT claim every later offset as a fetched
instruction from that incomplete page. State/capture and static source support the
mechanical conclusion. Snapshot export is persistent and local.

From that bonus snapshot,1092/1824 total frames show escape/ending, captures retained.
`ending-terminal.py` uses earlier ending-confirmed snapshot; source endFF is9580.
Patch pointer0098/99=9560, cursor03C6=31, scroll0370=7, raster phase0371=5.
Next update advances pointer32/readFF;0716 changes0->1 in12frames.612frames displays
HIGH SCORE/FINAL SCORE. Full12frame coverage pages retained here. Mode4 returns to
E28C mode6, then E291->E05D inventory/title restart statically; waiting through the
repetitive high-score timeout has diminishing prospects and is not needed for the
structural continuation. No natural terminal route or every guardian defeat claimed.
Existing scenarios already visit ordinary and guardian stages across all4 level banks.

## Narrowed zero proposal and residuals

Prior round rejected a368-byte zero suffix because a vector instruction occupies its
first byte. Decisive artifact: accepted FE82 JMP indirect is3 bytes, endingFE85;
FE84 zero is its operand. Corrected narrower claimFE85..FFF4 is367 uniform zeros,
independently bounded by that vector end and rootedFFF4 JMP FDA1. It has no competing
fetch/decoder evidence. Propose generic non-code, not original linker fill purpose.
This localizes the prior rejection rather than retrying it unchanged.

After all proposals, only bank14 physical1D0FD..1D100 (D0ED JMP CF65),1D4F5..1D4F6
(D4E5 RTS),1D59B..1D59E (D58B JMP CF65),1DACE..1DACF (DABE RTS) remain8 bytes.
No direct reference or matched state entry found in current finite tables/graphs.
Nearby routine branches either jump over them or terminate; valid short encodings
alone do not discriminate code from non-code fragments. No arbitrary execution or
semantic label is proposed. This is a PRODUCTIVE HANDOFF, not terminal exhaustion;
independent classification review may change exact residuals, and fresh redirect
must assess accepted outcomes before any terminal rejection review.

No other identified actionable avenue was deferred as independent/outside scope.
Remaining low-slot runtime semantics need specific resource-bank contracts; generic
non-code and fixed-slot code proposals do not depend on those semantics. More
natural boss combat would revisit already explained mechanics with little structural
prospect. Proposed indirect families, missing source callees, assets, IRQ/scroll
and return continuations have been inspected as one batch.

Toolkit: no new findings; existing report has2 confirmed documentation/API mismatches,
0 suspected,0 missing features, no blockers. Binary/ROM/map/source unchanged. Disk
hygiene checked; small durable records and selected captures retained, no cleanup
of prior evidence, publication, archive or toolkit edits.
