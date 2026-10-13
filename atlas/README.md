# ROMAtlas results

Results are organized as `<platform>/<game>/<full-rom-sha256>/`. Each directory
contains final maps, editorial reconstruction metadata, review records and agent
documents. Original ROMs, generated assembly, extracted assets, scratch evidence
and toolkit-issue reports are excluded.

Create or replace a result using `python3 scripts/archive.py archive WORKSPACE`.
Recover the exact commented source using your own matching ROM:

```sh
python3 scripts/archive.py restore atlas/gb/alleyway/ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2 --rom /path/to/alleyway.gb --out /tmp/alleyway-restored
```

See [INSTRUCTIONS.md](../INSTRUCTIONS.md) for the complete workflow and pinned tools.
The Alleyway result has a completed editorial review and a byte-identical rebuild;
its phase-one records preserve a 31-byte unknown interval separately.
