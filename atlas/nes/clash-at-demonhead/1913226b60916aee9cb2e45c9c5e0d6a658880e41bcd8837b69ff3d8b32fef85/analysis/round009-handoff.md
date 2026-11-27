
## Round009 independent disassembly — ROM incomplete

ROM classification: 99.99% (262140 / 262160 bytes classified; 20 bytes unknown).

Input pass-008-final -> fresh pass-009-final. L014 verified:34 new Static Code bytes in five suffixes,16 fresh instructions, five annotations. Independently reviewed full useful supplied-A MMIO/shadow commit and actor-field contracts, exact banks [12,13,14,15], matched caller/nested frames and coherent entry state. No natural caller, execution or missing original opcode claim. Fresh primary decodes and neighbor/setter consumers: [round009/review.md](round009/review.md), reviewed-instructions.json, consumers.json and reviewed-code.txt. Four leading2B bytes and two leading5C03 bytes remain Unknown. Connected setter and neighbors already classified; no unsupported closure. No new execution coverage. Prior broad later-game/mechanics/artificial ending/THE END/natural-dialogue scenarios reviewed; unchanged practical contracts do not justify repetition. No terminal exhaustion claim.

map.validate passed; state saved/currentmap exported. Fresh maps/pass-009-final.json equals maps/current.json; generated/pass-009-final/game.asm byte-identical rebuild passed [pass-009-final-verification.json](pass-009-final-verification.json). Exact whole-ROM unique-byte counts/per-bank complements [round009/counts.json](round009/counts.json);20 unknown bytes/14 intervals [round009/unknown-ledger.json](round009/unknown-ledger.json). Historical annotations unchanged by ID, including D107/246-ROMtile and corrected shop16terminating/240repeating metadata. Input ROM SHA unchanged.

|Bank|Code|Data/image/header|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG 00|75|8117|8192|0|
|PRG 01|334|7858|8192|0|
|PRG 02|3694|4498|8192|0|
|PRG 03|0|8192|8192|0|
|PRG 04|2902|5288|8190|2|
|PRG 05|6505|1683|8188|4|
|PRG 06|759|7433|8192|0|
|PRG 07|4826|3365|8191|1|
|PRG 08|1662|6530|8192|0|
|PRG 09|1045|7147|8192|0|
|PRG 10|7297|894|8191|1|
|PRG 11|7301|891|8192|0|
|PRG 12|7614|577|8191|1|
|PRG 13|6954|1235|8189|3|
|PRG 14|3512|4679|8191|1|
|PRG 15|6512|1673|8185|7|
|CHR 00|0|8192|8192|0|
|CHR 01|0|8192|8192|0|
|CHR 02|0|8192|8192|0|
|CHR 03|0|8192|8192|0|
|CHR 04|0|8192|8192|0|
|CHR 05|0|8192|8192|0|
|CHR 06|0|8192|8192|0|
|CHR 07|0|8192|8192|0|
|CHR 08|0|8192|8192|0|
|CHR 09|0|8192|8192|0|
|CHR 10|0|8192|8192|0|
|CHR 11|0|8192|8192|0|
|CHR 12|0|8192|8192|0|
|CHR 13|0|8192|8192|0|
|CHR 14|0|8192|8192|0|
|CHR 15|0|8192|8192|0|

Prior fully reviewed commentary/source/game.asm SHA b40d7ae1d434138052b430100299b381146a8102ac79de9eeff6e497b31ba42f preserved with round009/commentary-input.asm; reading-after.asm/history retained. Full refreshed current-source commentary next; review locations relative to commentary/source, game.asm:line. Final editorial acceptance pending. L015 open for localized residual reasoning after commentary; ROM incomplete. Toolkit: existing TK001 atomic batch feature request,0newissues/0blockers; [analysis/toolkit-issues.md](toolkit-issues.md). Disk inspected2.7GiBfree, scratch92KiB. No cleanup/deletion, toolkit edits, archive or publication. All server subprocesses exited; no live writer.
