# Building __GAME_NAME__ locally

This is the baseline guide generated with the port scaffold. The port agent must
update it for the finished executable, dependencies, flags and validation.
Platform instructions below are recipes, not evidence that a platform was tested.

Build on the target operating system and CPU architecture. Install Rust through
[rustup](https://rustup.rs/), use Rust 1.90.0 and retain port/Cargo.lock.

## Preserve and transfer source

Keep port/src/, Cargo.toml, Cargo.lock, rust-toolchain.toml and any authored
extensions. Retain the exact edited ROMAtlas source, including uncommitted
shared-library changes; Cargo.lock does not freeze a path dependency.
The generated dependency initially points to:

```text
__TOOLKIT_PATH__/crates/romatlas-port
```

Exclude target/ directories when copying; Cargo recreates their build products.
Reference archives, extracted assets and generated binaries are unnecessary for
running the native port. Supply your own matching ROM on the destination.

For this recipient layout:

```text
games/__GAME_NAME__/port/
games/romatlas/
```

change port/Cargo.toml to use:

```toml
romatlas-port = { path = "../../romatlas/crates/romatlas-port" }
```

From a Mac/Linux terminal, substitute the server and workspace paths:

```sh
rsync -avz --progress --exclude='target/' user@server:/path/to/port-workspace/port/ ./games/__GAME_NAME__/port/
```

Transfer the edited ROMAtlas source too, preserving its root Cargo files and
source layout; exclude target/, .git/, test ROMs and generated/local game assets.
The native port needs the shared library, not the disassembly reference project.
Use Cargo directly on the recipient: generated run.py/workspace.json still record
the preparation machine's toolkit paths and frozen-reference inventory.

## macOS

Install Xcode command-line tools, Rust and SDL:

```sh
xcode-select --install
rustup toolchain install 1.90.0
brew install sdl2-compat
```

Homebrew currently supplies the SDL2 API through
[sdl2-compat](https://formulae.brew.sh/formula/sdl2-compat), backed by SDL3.
In the same Terminal session used to build:

```sh
export LIBRARY_PATH="$(brew --prefix)/lib${LIBRARY_PATH:+:$LIBRARY_PATH}"
cd ~/games/__GAME_NAME__/port
cargo +1.90.0 build --release --locked --features desktop
./target/release/__GAME_NAME__-port --rom "/full/path/to/your/game.gb"
```

If the linker reports `library 'SDL2' not found`, repeat the LIBRARY_PATH export
in that Terminal session and check `brew list sdl2-compat`. Rust and SDL libraries
must use the same architecture; use native arm64 tooling on Apple Silicon.
You can add the export to your shell startup file for future builds.
Reference: [Rust SDL2 macOS linking](https://github.com/Rust-SDL2/rust-sdl2#macos).

## Linux (Ubuntu/Debian)

```sh
sudo apt-get install build-essential libsdl2-dev
rustup toolchain install 1.90.0
cd ~/games/__GAME_NAME__/port
cargo +1.90.0 build --release --locked --features desktop
./target/release/__GAME_NAME__-port --rom "/full/path/to/your/game.gb"
```

Desktop launch requires a graphical session. Do not launch it on the headless
server. Other distributions require equivalent C build tools and SDL2 development
libraries.
Reference: [Rust SDL2 Linux requirements](https://github.com/Rust-SDL2/rust-sdl2#linux).

## Windows x64 (MSVC)

Install Rust with the MSVC toolchain and Visual Studio C++ build tools/Windows SDK
as described in the [rustup Windows guide](https://rust-lang.github.io/rustup/installation/windows.html).
Download an SDL2 MSVC development archive (SDL2-devel-2.x-VC.zip) from
[official SDL2 releases](https://github.com/libsdl-org/SDL/releases?q=SDL2&expanded=true).
Locate its lib/x64 folder containing SDL2.lib and SDL2.dll.

In PowerShell, adjust the source and SDL paths:

```powershell
rustup toolchain install 1.90.0
Set-Location C:\games\__GAME_NAME__\port
$sdlLib = "C:\libraries\SDL2\lib\x64"
$env:LIB = "$sdlLib;$env:LIB"
cargo +1.90.0 build --release --locked --features desktop
Copy-Item "$sdlLib\SDL2.dll" .\target\release\SDL2.dll
.\target\release\__GAME_NAME__-port.exe --rom "C:\games\game.gb"
```

Use forward slashes in the Cargo.toml dependency path. Keep SDL2.dll beside the
executable. This recipe is for Windows x64; other targets need matching libraries.
Reference: [Rust SDL2 Windows MSVC linking](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc).

## Headless builds and checks

SDL2 is optional. Without the desktop feature, build and run device-free:

```sh
cargo +1.90.0 build --release --locked
./target/release/__GAME_NAME__-port --rom "/full/path/to/your/game.gb" --headless 120
cargo +1.90.0 test --locked
```

Windows uses the .exe suffix. README.md must describe the finished port's controls,
asset manifests and any scripted-input flags.
In the original full workspace, `python3 run.py check` runs formatting, clippy,
headless tests and desktop compilation without launching graphics.

## Validation status

At preparation, gameplay is an unimplemented scaffold. No desktop runtime,
physical devices or other operating systems have been validated for this port.
The port agent must replace this section with actual build/test evidence,
distinguishing compilation, headless execution, desktop play and user-reported
results. Keep untested platforms/device behavior explicit; instructions do not
establish compatibility. Missing desktop validation alone does not block port
completion, and must not cause a server graphics launch.

## Later packaging

Build automation, CI, app bundles and installers can be applied to existing
source ports later. Retain exact port/library source and lockfiles as their
inputs. Bundling SDL/runtime dependencies, signing/notarization, immutable
library pinning and clean-machine release testing are separate later work.
These development instructions do not authorize publishing or distributing games.
