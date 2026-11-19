# Archived ROM analysis

Final maps, review metadata and agent documents are retained verbatim.
Phase-one classification and phase-two editorial completion are separate;
see their audit records for remaining unknowns. Scratch evidence, toolkit
issue reports, ROMs, generated assembly and extracted assets are omitted.
References to omitted scratch files in historical documents are unavailable.

editorial.json stores editorial metadata and baseline references, not a
standalone assembly listing. Restore with the original ROM and pinned tools:

```sh
python3 scripts/archive.py restore PATH_TO_THIS_DIRECTORY --rom ORIGINAL_ROM --out /tmp/restored-game
```
