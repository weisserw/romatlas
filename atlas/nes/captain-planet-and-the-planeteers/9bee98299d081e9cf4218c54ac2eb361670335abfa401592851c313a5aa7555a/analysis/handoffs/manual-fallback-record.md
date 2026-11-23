# Manual fallback record (user-directed batch exception, 2026-11-23)

**This is a user-directed fallback.** It is not an observed or statically proven data role.

## Authorization
The disassembly goal ended with a justified terminal-incomplete handoff: 4 bytes unknown after
the round-3 rejection review (analysis/handoffs/round-003-redirect.md,
analysis/handoffs/final-coordinator.md). The batch runner does not accept that state without the
final audit. The user then explicitly authorized a batch-only exception for at most 10
residual physical bytes: mark them as generic Data with unknown meaning, Manual provenance. The
instruction overrides the rules requiring further structural proof, for these bytes only.

## Verified complement before the change
On pass-003, analysis.unknown on analysis/state.json returned exactly 4 one-byte ranges, matching
the authorized list:

| File offset (dec) | File offset (hex) | CPU (bank) | Byte | Context |
|---|---|---|---|---|
| 49541 | $0C185 | $A175 (bank 6) | $60 | after a routine's RTS, before the APU pulse-1 init routine |
| 49563 | $0C19B | $A18B (bank 6) | $60 | after that routine's RTS, before the APU noise init routine |
| 129913 | $1FB79 | $FB69 (bank 15, fixed) | $60 | between two classified routines |
| 130512 | $1FDD0 | $FDC0 (bank 15, fixed) | $60 | between two classified routines |

## Prior ambiguity (unchanged)
- **Rejection review findings:** no references, including RTS-trick and computed-jump tables. None
  of the bytes was fetched in any traced route. They are not alignment, and they are not the
  ROM's $00/$FF fill.
- **Code reading:** a lone unreferenced RTS "lacks sufficient structure on its own"
  (evidence-policy.md).
- **Data reading:** nothing positively supports it.
- **Conclusion:** the code-versus-data decision remained materially ambiguous.
- **Reopening:** only a new reference, a new fetch, or a model of how the stubs were generated
  would discriminate between the readings.

## Action
- tools_local/pass004_manual_fallback.py first asserts the exact complement. It then calls
  map.annotate for each byte:
  - kind Data, provenance Manual, no label;
  - comment "Unknown meaning; user-directed manual fallback (not evidence-backed)";
  - an evidence string disclosing the fallback.
- No semantics were invented and the ROM was not modified. The prior state is kept as
  analysis/state.pass-003.json.gz.
- Checkpoint pass-004 (python3 run.py checkpoint pass-004): map.validate ok, verification ok,
  byte_identical true. The map is maps/pass-004.json = maps/current.json, sha256 3debb849...

## Classification statement
ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

| Basis | Bytes |
|---|---|
| Evidence-backed (Observed/Static) | 262156 |
| User-directed Manual Data fallback (meaning unknown) | 4 |

Provenance counts by region: Static 1326, Observed 610, Manual 4.

## Commentary and audit
- **Preserved history:** the round-3 commentary is preserved in analysis/round-3/commentary/.
- **Fresh commentary:** commentary was re-prepared on pass-004, with names, comments and review
  carried by physical address. The source notes for the 4 bytes and their review summaries
  disclose this fallback.
- **Strict audit:** commentary.py audit passed: ok, byte_identical, 1940/1940 regions and
  2184/2184 units reviewed, editorial_audit true. The coordinator re-ran it independently, and
  archive.editorial_status reports complete.
- **Details:** see analysis/handoffs/round-004-commentary-final.md.
