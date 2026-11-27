# Round 2 localized uncertainties

Current-source readability covers all classified source. The annotation inventory covers the whole ROM, including one explicit Manual fallback
byte. Evidence-backed structural classification remains incomplete by that byte;
final editorial acceptance is recorded in editorial-audit.md.
The full prior uncertainty record is preserved under
../analysis/round-02/prior-commentary/uncertainties.md.

- Physical $1CAF1..$1CAF2 (CPU $CAE1 in fixed physical bank 14) is now
  generic Data with Manual provenance under the explicit user exception. Its $60
  meaning remains unknown: redundant RTS versus spare data is still unresolved.
  No incoming edge or independent entry contract has been established; annotation
  completion does not add structural proof or establish reachability.
- The connected handler bodies, 42 seven-byte object records, stage layout and
  26-byte metatile containers, counted graphics streams, animation/image packing,
  music roots and numeric tables are now classified. Prior statements that these
  backing ranges or intervening bodies were unknown are superseded. Original
  actor/puzzle names, exact item identities, tune names and scoring units remain
  uncertain; source names describe mechanics and preserve numeric action IDs.
- Static accepted domains do not establish natural selection of every handler,
  action pointer, image, music selector or DMC descriptor. Banked CPU aliases need
  their effective physical pair. The visible handler bank is 12/13; music roots
  use mapper selection 2, animations/images 3, map roots 0 and raw CHR 4/5.
- Scratch fields are reused: object health can hold an effect owner, $BE..$C0
  serve health arithmetic as well as score/HUD work, and local handler bytes
  have scoped aliases. Ground status $65 is one while falling and zero on support;
  the ground helper can snap Y, so callers' probes are not generally position-pure.
- GetPlayerActionDamage performs no local range clamp before reading its
  twenty-two-byte table. The player-action domain is a calling precondition;
  this static source annotation does not prove all possible runtime states safe.
- Decoder token classes are corrected in all current explanations: $01..$3F
  repeats one value, $41..$7F emits low-six-bit zeroes, $81..$FF copies literals,
  and $00/$80 skips. $40 enters a zero-count do-while loop and emits 128 zeroes
  unless the output budget ends first. None of 6554 saved asset token records
  contains $40; accepted stream bounds are unaffected. Final literal suffixes
  and second stored $FF music markers belong to storage even when not read.
- The fourth decimal scale word is zero but BuildFourDecimalGlyphPairs selects
  only 1000,100,10 before retaining units. Its original purpose is unresolved.
- Generic uniform runs retain a generic role where consumer evidence does not
  distinguish padding from resource content. Several old uniform fragments now
  fall inside a supported layout/metatile container; source explains that parent
  format while preserving the exact historical structural scaffold.
- NMI semantics remain: $CAB7 balances the gate and RTIs for reentry, $CABA
  dispatches the callback, and $CABD is the default frame body. CPU $CE9F/$CEA0
  is the inline RAM-image pointer $05A7 after JSR $D360; $CEA1 resumes code.

The historical structural frontier is the isolated $1CAF1 byte. Its Manual fallback
annotation does not resolve that frontier. This batch resumes no redirect research;
prior bounded reference/stack review remains valid. Remaining game identities and
natural selector use are semantic/runtime questions, not skipped readability or
reasons to weaken supported classifications.
