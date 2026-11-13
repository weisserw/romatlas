# Round001 redirect handoff

Ready combined batch: R001-R01 and R001-R02 in analysis/leads.json.
No map/state/source changed. Input checkpoint pass-001-final; 65552/65552 classified,
0 unknown;9478Code/23306Data/32768Image,84 regions. Source readability input gate
84 regions/261 units completed, no unread gap. Final editorial acceptance pending.
Exact empty unknown complement and current identities: unknown-complement.json,
input-checkpoint.json, preservation-check.json. Commentary byte-identical rebuild
retained at analysis/commentary/round001/verification.json; no new build claimed.

R001-R01: fix incorrect HandicapCursorPositions label/evidence for physical0556..0557,
CPU8546..8547 fixed PRG[0,1,2,3]. Actual44/6A are RAM destinations written via TAY,
STA0000,Y. Source already corrected. Independent disassembly must update current
metadata/internal record, preserve frozen history and byte extent/Data/Static.

R001-R02: 940 existing Data bytes have stronger exact sound schema constraints:
physical3261..3278 twelve period words;32D1..3328 twenty-two4-byte descriptors;
3329..3664 twenty-two bounded streams. Fixed PRG[0,1,2,3]. Consumer-derived finite
semitone domain, descriptor fields/last ID/first stream boundary, and stream command
paths/FF endpoints independently support refinements. First6 streams loop before
trailingFF; final stream consumes FF at physical3664 (first byte of old uniform run).
No newly classified bytes, no artificial/natural execution claim. Keep unproven gaps
generic Data. Review findings.md, primary-evidence.txt, verify_sound.py and its saved
sound-verification.json; stream endpoint details in sound-stream-boundaries.txt.

Connected setup and sound consumers/continuations inspected. Other identifiable
uncertainties checked against current source and previous primary reviews: standalone
render-mask caller, terrain prefix/index domains, third sprite display meaning,
original units/generators, wide division contract, RAM intent and remaining sound
subformats. findings.md records concrete limits/reduced prospects for each; none
is deferred solely for independence or first-graph closure. Broad gameplay checklist
includes later9/final18,putting/flight,terminal scorecard->title; no missing later/ending
route with an obvious untried shortcut. Prior failed manual timing not repeated.

This is a productive lead handoff, not terminal incomplete or final acceptance.
Independent disassembly should review/apply combined batch, validate and checkpoint,
then current-source commentary and redirect continue. No toolkit issues found;
analysis/toolkit-issues.md unchanged. No toolkit edits,publication,archive,deletion,
commits or remaining writer/server. ROM/assets/evidence remain local.
