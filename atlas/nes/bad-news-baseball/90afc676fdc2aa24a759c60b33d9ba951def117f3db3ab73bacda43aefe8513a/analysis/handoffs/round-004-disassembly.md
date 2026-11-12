# Round004 disassembly handoff


### Round 004 disassembly complete — incomplete classification

Exactly one substantive pass. R010/R011 verified with independent local control/stack/consumer review; accepted180bytes=171 Static Code+9 Static Data. Connected downstream routines already classified; no further supported unknown neighbor. No new Observed or natural invocation claim.

ROM classification: 100.00% (262154 / 262160 bytes classified; 6 bytes unknown).

Rounding does not establish completion. Exact complement: physical0x10530..0x10534 (4bytes, PRG08, CPU8520 slots[8,9,14,15]);0x19E99..0x19E9B (2bytes, PRG12, CPU9E89 slots[12,13,14,15]). Code/data alternatives remain materially ambiguous; independent review at analysis/disassembly/round004/review.md. No terminal exhaustion claim.

| Physical bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| header |0|16|16|0|
| PRG00 |2138|6054|8192|0|
| PRG01 |6|8186|8192|0|
| PRG02 |0|8192|8192|0|
| PRG03 |901|7291|8192|0|
| PRG04 |2218|5974|8192|0|
| PRG05 |6|8186|8192|0|
| PRG06 |566|7626|8192|0|
| PRG07 |6|8186|8192|0|
| PRG08 |1232|6956|8188|4|
| PRG09 |6|8186|8192|0|
| PRG10 |7403|789|8192|0|
| PRG11 |2603|5589|8192|0|
| PRG12 |7928|262|8190|2|
| PRG13 |5939|2253|8192|0|
| PRG14 |5593|2599|8192|0|
| PRG15 |12|8180|8192|0|
| CHR00..15 each |0|0|8192 Image|0|

Totals36557Code/94525Data/131072Image;1540 evidence-backed regions. Preserve all existing region IDs/metadata and commentary/source/game.asm SHA2562a68c97f32366331429ff8ab04ba251a52d8627829d8ed61fcd5f67052a95430; prior prose/review credit intact. Current expanded-source commentary is next, not claimed complete. Fresh map validate/save/export and pass-004-final checkpoint byte-identical true; maps/current.json SHA25620b933e17d8f3032291cb4ca229fd6da613cee1a1975e18d4d2fff4df73f6dc6. Verification analysis/pass-004-final-verification.json; generated/pass-004-final. ROM hash unchanged.

Fresh explicit stack audit47/60/81/1007 states, no stops. Actual CDA4 removes C042 return then uses retainedA378; savedY survives. Actor0..11 and menu/field inputs remain conditional. D4DB is exact nonstandard sequential transfer, not a balanced wrapper. Direct9A45/A8D9/B5B2 consumers reviewed; sharedB5B2 RTS Code preserved. Broad saved gameplay including credits/return reused without unchanged replay.

No toolkit issues found; see analysis/toolkit-issues.md. Disk6.4GiB free, scratch376KiB; no cleanup. No live server/competing writer, toolkit/input/source edits, archive or publication. Handoff analysis/handoffs/round-004-disassembly.md.

Primary review and exact complement/per-bank/applied records: analysis/disassembly/round004. R010/R011 fully accepted. Next separate commentary phase must preserve established source prose/review credit; then redirect may assess six residual bytes and terminal status. This worker has stopped writing; no live server or competing writer.
