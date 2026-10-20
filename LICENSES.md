# Licensing

The combined `romatlas` CLI and the NES/SNES adapters are distributed under GPLv3
because they link the GPLv3 jgenesis core. See
[vendor/jgenesis/LICENSE](vendor/jgenesis/LICENSE) for the full license and
[vendor/jgenesis/ROMATLAS.md](vendor/jgenesis/ROMATLAS.md) for revision and local
changes. Source, build scripts and notices must accompany distribution in the
manner required by GPLv3. GPL permits commercial use.

The existing original MIT grants remain in [LICENSE](LICENSE). Shared crates,
the Game Boy adapter and `romatlas-port` retain their MIT package licenses and
notices; the native-port runtime does not link jgenesis. Generated native ports
use that independent runtime. Do not copy emulator implementation into the port
library under its MIT notice.

Asar 1.91 is an external LGPL-3.0-or-later build tool. Setup retains its LICENSE
and license-lgpl.txt in tools/asar. Its license disclaims copyright over assembler
input and output. Preserve its notices and replacement/source requirements when
redistributing the tool. Mizu, mgbdis, RGBDS and other dependency notices remain
with their respective sources. No license to a user's ROM or extracted assets is
provided by the toolkit license.


NES uses the same pinned GPLv3 jgenesis fork, including nes-core, mos6502-emu,
nes-config and its YM-OPLL build dependency. All upstream notices remain in the
vendored source. Independent `romatlas-port::nes` rendering/audio helpers are
original MIT implementations and import no emulator crate.

ca65/ld65 V2.19 (cc65 revision 555282497c3ecf8b313d87d5973093af19c35bd5) are
external assembler/linker tools under the upstream permissive, three-restriction
license retained in tools/cc65/LICENSE. Preserve its attribution and notices when
redistributing the tools. No game ROMs or extracted assets are bundled.
