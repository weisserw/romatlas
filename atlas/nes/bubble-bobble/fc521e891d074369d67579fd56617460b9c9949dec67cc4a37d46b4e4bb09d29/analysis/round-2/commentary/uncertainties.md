# Current-source uncertainties and redirect questions

All currently classified source has mechanical explanations; these questions do
not leave an editorial range pending. Source comments distinguish complete helpers
from decoded fragments separated by unknown bytes. Review coverage is in review.json.

- Boss damage and defeat: bank10 CPU$8FFC (physical0x1500C) tests two enabled fixed
  zones against player position, while$86CA (0x146DA) tests both players against a
  caller-supplied point/radius. Success continuations are currently unknown. Trace
  the confirmed Continue boss route with bounded watch/read experiments to find
  the HP/defeat flag and ending transition. Do not treat a RAM write as a visit.
- Stage records: fixed-bank$CF04 (0x1CF14) unpacks24 rows from336 packed bytes;
  $E919 (0x1E929) selects22-byte actor records with a packed header and3-byte
  entries. These consumers give a useful schema hypothesis for banks0..6. Exact
  per-bank grouping/consumer correspondence and pointer carry paths remain unknown;
  generic data classification stays unchanged.
- Sound streams: bank8$8176/$8217 parse phrase/note pointers and control bytes;
  $C4C3/$F922 extract inline words through nested return addresses. Unclassified
  gaps after calls may contain inline operands. Derive only supported local word
  extents/calling contracts before proposing structural changes. $FE/$FF command
  bodies and complete dispatch/envelope table extents are unresolved.
- Actor kinds, item identities and several global flags have established mechanical
  uses but unresolved game meanings. Names such as RecordFieldN and CpuAddress_NNNN
  retain exact identity without inventing a role. Scratch aliases are deliberately
  shared; boss state overlays actor RAM. Table symbols name consumer bases only.
- Reset writes$4022..$408A resemble disk-system I/O initialization. Their intended
  meaning on this mapper1 image is unresolved; no peripheral support is inferred.

Existing progression, residual heterogeneous-bank structure and tile/record-consumer
leads remain in analysis/leads.json. No terminal exhaustion assessment was performed
in commentary; redirect remains useful. No classification or runtime experiment was
performed by this phase.
