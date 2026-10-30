# Additive continuation audit
Prior nonterminal handoff preserved by copy in continuation-input-handoff.md. No map/source edits.
1. Build bounded bank-aware function summaries from saved decoded instructions and finite selected dispatch domains. Balance PHA/PLA and PHP/PLP; F7CA consumes its own JSR return and tail-selects a handler, rather than creating an extra routine return. Distinguish conditional returning paths from deliberate stack-reset exits. Audit BDF0,C6A4,BC33,E0BD independently.
2. Inspect B000's upstream reset/bank-selection evidence and F843 target; inspect IRQ tail MMC3 selector/data writes and stack/RTI contracts. Follow supported fixed-slot continuation without inventing lower data banks.
3. Review special selector producers and narrow finite domains. Use existing descriptor/room metadata constraints before further experiments. Stop at actual missing selector/bank or diminishing prospect; no exhaustive arbitrary states.
4. Refresh proposals and exact unknown ledger; preserve historical research and handoff. Record actual reasons for deferrals before concluding phase ready for disassembly.
