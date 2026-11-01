# Final current-state verification

Result: passed; structural classification remains incomplete.

- Classified 262156/262160 physical bytes; four isolated unknown bytes.
- Code 39605, Data 91479, CHR Image 131072. Evidence exists on every classified region;
  ranges do not overlap and Code has explicit execution views.
- Original ROM SHA256 `a6a548c4a27d3961821312a95e45ecda0eea36e247ee2d963079c336b23882c2`.
- Current map/internal store/latest checkpoint SHA256 `8fac0ec90e59abe29b84a73cee9875b8e89ff5d7c5841e626579807c6a56138c`.
- Configured release binary SHA256 `8ff56b0b1cf8d2a3aca5430c70d34280ae3db36fc6774919062db4ea63843f82`, unchanged.
- Current game.asm SHA256 `22ea01c06e0e62e6b5bfacfb258dcdb79d8cadec9435546f8560fc36060bd967`.
- Current symbols.inc SHA256 `582db20960dcf3c79b8195328b855f5652a88675cd4765be80de2b8682ce300f`.
- Reading copy SHA256 `9f83aae5978c84018886fe967483f8ba0439acfa4e7381c7d16f02350a0d146b`; exactly matches current source expansion.
- Final commentary audit: ok, byte-identical, editorial audit reviewed. The report's
  source hashes equal the actual current files and fresh verification report.
- All 8271 physical review records retain the frozen scaffold; 1087 unique units
  and 16867 source links checked. Every Code record has reviewed units.
- Configured-binary store/map validation both returned ok. Frozen baseline hashes
  match the current map/state. No ready classification lead remains.

Primary verification artifacts: commentary/audit.json, commentary/verification.json,
analysis/final-editorial/20261031-a/consistency.json, store-validation.json and
map-validation.json in that directory; analysis/coordinator-final-verification.json
contains the coordinator's independent machine-readable check.

Source quality is established by the preserved full review, final affected-source
inspection and commentary/editorial-audit.md, not by these mechanical checks alone.
The four unknowns and unresolved semantic meanings remain explicit. No archive,
cleanup, publication or toolkit change was performed.
