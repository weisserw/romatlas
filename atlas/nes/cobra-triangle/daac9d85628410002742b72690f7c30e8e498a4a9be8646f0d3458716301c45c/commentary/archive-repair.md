# Archive repair

The prior archive rejected `commentary/source/mapping.json` as a changed
non-assembly asset. Inspection of the archive and commentary contracts, parent
job error, final progress and handoffs found a stale source-side map: it lacked
accepted region 1481 (physical 9813..9818, CPU $A645, effective PRG banks
[0,1,2,3]) and retained next_id 1481 instead of 1482. All other map content
matched. Current portable map, frozen baseline map and before/mapping.json
already agree byte for byte on the accepted final classification.

Reconciled only source/mapping.json with that existing final baseline map.
This preserves classification rather than introducing a new decision. Reviewed
the exact metadata diff and source/game.asm lines 5367–5383: the existing labeled
clear of SharedMotionTarget and common-frame join already implement and explain
this accepted region, including unresolved natural selection and frame/stack
requirements. No assembly, labels, comments, review credit, frozen baselines,
ROM bytes or historical findings were changed or regenerated. Mapping metadata
is not consumed by the NES assembler's verification path.

Original edited source metadata, archive options and prior audit/verification
records are backed up under analysis/archive-repair-backups/. Preservation
hashes there cover the ROM, annotated assembly, frozen map/state and review.
archive-options.json retains every prior document disposition and adds this
final document. No new research or toolkit changes were needed.

Verification results are recorded below after running the strict editorial audit
and the required disposable archive probe. Classification remains incomplete:
25 unknown bytes in ten intervals; natural full progression and full hardware
compatibility remain unproved. Existing historical final conclusions and their
qualifications remain authoritative. Archive restoration requires the owned ROM
and pinned toolkit; internal evidence remains local. Only the disposable probe
at /tmp/romatlas-archive-probe-ix61553g/archive is authorized here.

Strict commentary audit passed after the repair: byte_identical=true, 1491 reviewed
regions, 1568 reviewed units, editorial_audit=true. Annotated assembly SHA-256
remains 8a75a69947ed859e4302fca913deeea5451fc6d4b96aff5c5a62a502c1aad2f1.
Preservation hashes and the exact single-region metadata delta were checked.

The required disposable archive probe returned ok=true, exact_source=true,
byte_identical=true and editorial_complete=true. It reconstructed the exact
authoritative source tree and rebuilt against the original ROM. The result
and exact argv are saved locally in analysis/archive-repair-probe.json. This
report is included as a final document; the probe is rerun after this addition
so its archived copy contains the verification outcome. No real atlas destination
was written. There are no remaining archive repair blockers.
