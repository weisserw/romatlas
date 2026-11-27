# Round04 current questions and corrections

Current-source readability is complete. Classification remains separate:48 bytes
in21 fragments remain Unknown; exact ranges, bytes and competing interpretations
are in analysis/pass-004/unknown.json. Each retained raw range has a source note.
These are structural questions, not deferred readability.

- Physical $186d..$186f: can a specific scanner-entry constraint distinguish the
  apparent F0 ED branch from payload? The preceding RTS and bypass branch remain
  counterevidence; another broad replay is not a changed condition.
- Physical $7c03..$7c05: alternate palette Y selector versus stored word. Inspect
  a bounded independent entry/value or format constraint; the preceding wrapper
  tail-jumps past the bytes. Physical $7c7b..$7c7d is a separate $00/$09 fragment,
  not an established sprite trailer.
- Physical $18518..$1851a: apparent LDA #7 immediately overwritten at the audio
  successor. Identify a useful value/entry contract or a concrete metadata role.
- Physical $1e59a..$1e5a6: apparent coordinate copying has no exit before tile data.
  The preceding initializer returns. A complete bounded flow or payload constraint
  is needed to discriminate the competing roles.
- Isolated RTS bytes and tiny load/tailcall candidates have plausible mechanics
  but unresolved structural identity. Static supported entry contracts are allowed;
  no blanket natural-caller requirement or terminal exhaustion claim applies.

Meaning limits remain local: exact actor class selection, detailed scene/audio
resources, inner sprite/tile packing, original palette intent and some shared
scratch roles are uncertain. Mechanical state/timer, signed motion, allocation,
script stride/sentinel and PPU/stack contracts are explained in maintained source.
No semantic research limitation is omitted readability work.

Corrected prior metadata interpretation: physical $1fe78 / CPU $fe68 is $04,
not $ea. It completes the eleven-byte sequence
06 0A 04 04 06 06 06 04 04 04 04 immediately before FE69 A0 00.
The ten documented stage selectors remain separate from this generic-data suffix;
there is no established stage-ten domain claim. The maintained source and review
summaries are corrected. Historical commentary is preserved unchanged at
analysis/round-4/commentary-pass003; its old counts/frontiers and $ea statement
are superseded here.

C061/FA5B temporarily maps ten/eleven then explicitly restores two/three.
F5E1 writes one byte through a RAM pointer and advances once, trapping at high byte
$20. Audio86AE consumes the saved count with a balanced stack. PPU fill loops
are finite. These narrow contracts do not permit generic mapper/pointer inference.

Prior semantic questions are retained unchanged in
analysis/round-4/commentary-pass003/uncertainties.md for historical investigation;
its119-interval structural frontier is superseded by the21 current fragments.
