# Building __GAME_NAME__ locally

This is the baseline guide generated with the port scaffold. The port agent must
update it for the finished executable, dependencies, flags and validation.
Platform instructions below are recipes for a human developer, not evidence that
a platform was tested. The implementation stage must pass source-backed unit
contracts and format/static/desktop/Wasm checks; final QA runs only the short smoke test and initial exact-frame comparisons using reusable scripts and validated checkpoints. Physical-device auditing
and optional Android integration require a later user request; do not execute
these recipes to expand the implementation goal.

For desktop builds, build on the target operating system and CPU architecture. Install Rust through
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

## Android (APK integration required)

The generated project currently has a Rust CLI and an optional desktop SDL2
shell. It does not include an Android Activity, JNI entry point, Gradle project,
ROM picker or touch controls. The steps below describe the build integration a
port developer may add in separately requested work; installing an Android target alone does not produce a
playable APK. Keep the existing desktop/headless entry points working.

### Toolchain and native-code check

Install Android Studio and, through SDK Manager, Android SDK Platform-Tools,
a target SDK, NDK (Side by side) and CMake. Use the JDK required by the chosen
Android Gradle Plugin. Record exact SDK/NDK/JDK/Gradle/CMake versions in the
finished guide and pin the NDK with Gradle's `ndkVersion`. See the official
[NDK setup instructions](https://developer.android.com/studio/projects/install-ndk).

Start with `arm64-v8a` (`aarch64-linux-android`) for ARM64 phones. An x86_64
emulator needs `x86_64-linux-android` and matching SDL libraries. Install the Rust
targets and check the device-independent code from port/:

```sh
rustup target add --toolchain 1.90.0 aarch64-linux-android
cargo +1.90.0 check --release --locked --target aarch64-linux-android
```

This checks Rust code without linking SDL or packaging an app. It does not
validate Android execution. Native linking additionally needs the selected NDK's
Clang linker and ABI/API configuration; use one minimum API consistently across
Rust, SDL and Gradle. Host Linux SDL libraries cannot be used for Android.

### Add the Android application

1. Copy the `android-project` skeleton from a pinned **SDL2** source release into
   an android/ directory beside port/. Use the Java glue and native SDL sources
   from that same release. Follow the
   [SDL2 Android guide](https://github.com/libsdl-org/SDL/blob/SDL2/docs/README-android.md),
   rather than SDL3 instructions. Record the release and preserve its notices.
2. Move reusable game construction into a Rust library, retaining the desktop
   CLI. Add an Android `cdylib` target and an exported C-compatible `SDL_main`
   entry point, or a C shim that calls Rust. Match SDLActivity's library names
   and entry point, contain Rust panics at the FFI boundary, and return errors
   to the Android UI instead of terminating the process through the CLI.
3. Build SDL2 for each selected ABI with the skeleton's NDK/CMake integration.
   Configure Cargo's target linker and SDL search path for those outputs; wire
   the Rust build into Gradle so changes rebuild the native library. Package
   both the game shared library and its native dependencies for each ABI.
   Keep Cargo.lock and use `--release --locked` for Cargo invocations. Update
   this guide with the actual commands, library names and output locations.
4. Add a ROM selection screen using Android's
   [Storage Access Framework](https://developer.android.com/training/data-storage/shared/documents-files).
   A `content://` URI is not a filesystem path: read it through ContentResolver
   and copy it into bounded app-private storage before calling `Rom::open`,
   preserving SHA-256 validation. Keep settings and asset overrides in writable
   app storage.
5. Adapt the shell for pause/resume, surface recreation, audio interruption,
   Android Back, safe screen areas and input rearming. Add usable touch controls
   and a settings/menu route without F1, or explicitly require a controller.
   The desktop shell alone does not supply these mobile interactions.

### Optional later build, install and device audit

After the application integration above exists, run from android/:

```sh
./gradlew assembleDebug
adb install -r app/build/outputs/apk/debug/app-debug.apk
```

On Windows use `gradlew.bat`. Device play, settings/input/lifecycle/audio testing
belongs to a later explicitly requested device audit. Record any user-reported
results separately from the implementation's unit-test/build results. Cross-compilation and
APK assembly do not establish device behavior; no Android build or device test
has been performed for this scaffold. Release signing, keystore management and
store packaging require a separate release setup; keep signing secrets local.

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
establish compatibility. Record actual contract/build results and the final QA
report with both fresh pass results. Keep scenario scripts and checkpoint setup
recipes with the source and captured original-game evidence local. Do not launch
server graphics to complete this section. A later explicitly requested device audit
remains separate from the headless QA pipeline.

## Later packaging

Build automation, CI, app bundles and installers can be applied to existing
source ports later. Retain exact port/library source and lockfiles as their
inputs. Bundling SDL/runtime dependencies, signing/notarization, immutable
library pinning and clean-machine release testing are separate later work.

## Browser / WebAssembly

For this future-workspace scaffold, game logic and ROM asset extraction are shared
between SDL2 desktop and Canvas/Web Audio frontends. After toolkit setup:

```sh
python3 run.py web-build
python3 run.py web --port 8000
# Optional server-hosted replacement/additional assets:
python3 run.py web --assets /path/to/assets --port 8000
```

The default bind address is `0.0.0.0`; use `--host 127.0.0.1` for loopback or
`--host ADDRESS` for another interface. Open the printed URL and press Start for audio/gameplay. The configured
ROM must remain on the server; it is fetched, identity-checked and decoded at
startup. `web-dist/` contains generated Wasm,
bindings and shell only. Original assets must stay ROM-backed. Keep generated
files local. Existing workspaces are not upgraded automatically.

Browser settings support keyboard/standard gamepad rebinding, four player device
assignments, volume, scaling, dimensions and fullscreen. Settings and versioned
battery saves use browser localStorage keyed by ROM hash; keep the same server origin,
browser and profile to retain them. Start resumes after Pause or Settings. Focus loss does not pause the browser
game; browser background throttling can still limit updates. Reset reloads the
game and returns to the Start prompt, preserving settings and original battery
saves. No touch/mobile or save-state support is provided. Current desktop
Chromium is tested; Firefox uses the same browser APIs but requires separate
compatibility confirmation. Builds are single-threaded and require no cross-origin
isolation. See the toolkit's docs/porting.md for the save contract and test setup.

Implement originally supported battery saving through the optional shared Game
hooks. Desktop saves live next to settings in a ROM-hash-named JSON file. The game
validates payloads and owns save-format versions; frontends store and acknowledge
only successful writes. New port check/audit requires locked Wasm compilation;
the toolkit's synthetic browser smoke test is separate from game fidelity work.
