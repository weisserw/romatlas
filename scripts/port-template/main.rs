mod game;
mod config;
use config::ROM_SHA256;
use anyhow::{Context, Result, bail};
use romatlas_port::{assets::Rom, input::InputState};
use std::path::PathBuf;


fn main() {
    if let Err(error) = run() {
        eprintln!("{}",serde_json::json!({"ok":false,"error":format!("{error:#}")}));
        std::process::exit(1);
    }
}
fn run() -> Result<()> {
    let mut debug_stdio = false;
    let mut rom = None;
    let mut assets = None;
    let mut settings = None;
    let mut frames: Option<usize> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--debug-stdio" => debug_stdio = true,
            "--rom" => rom = Some(PathBuf::from(args.next().context("--rom needs a file")?)),
            "--assets" => assets = Some(PathBuf::from(args.next().context("--assets needs a directory")?)),
            "--settings" => settings = Some(PathBuf::from(args.next().context("--settings needs a file")?)),
            "--headless" => frames = Some(args.next().context("--headless needs a frame count")?.parse()?),
            "--help" => {
                println!("Usage: __GAME_NAME__ --rom FILE [--assets DIR] [--settings FILE] [--debug-stdio] [--headless FRAMES]\nF1 resume, F2 pause, F3 reset, F4 save state, F5 restore state, F6 settings. This scaffold is not a ported game.");
                return Ok(());
            }
            _ => bail!("unknown argument: {arg}"),
        }
    }
    let rom = Rom::open(&rom.context("supply your matching ROM with --rom")?,ROM_SHA256)?;
    if debug_stdio {
        anyhow::ensure!(frames.is_none() && settings.is_none(), "--debug-stdio cannot combine with --headless/--settings");
        let identity = romatlas_port::debug::identity(ROM_SHA256, assets.as_deref())?;
        return romatlas_port::debug::serve_with_snapshots(
            || {
                anyhow::ensure!(romatlas_port::debug::identity(ROM_SHA256, assets.as_deref())? == identity, "debug assets/build changed; restart session");
                game::Port::load(&rom, assets.as_deref())
            }, config::SIZE, config::RATE, config::settings().bindings.keys().cloned().collect(),
            identity.clone(), |_| serde_json::Value::Null,
        );
    }
    let mut port = game::Port::load(&rom,assets.as_deref())?;
    if let Some(frames) = frames {
        let frame = romatlas_port::headless(&mut port,frames,|_| InputState::default(),config::SIZE)?;
        println!("{}",serde_json::json!({"ok":true,"scaffold":true,"frames":frames,"width":frame.width,"height":frame.height}));
        return Ok(());
    }
    #[cfg(feature = "desktop")]
    {
        let path = match settings {
            Some(path) => path,
            None => romatlas_port::settings::default_path("__GAME_NAME__")?,
        };
        let battery = path.with_file_name(format!("battery-{}.json", ROM_SHA256));
        romatlas_port::desktop::run_with_assets(&mut port,"__GAME_NAME__ - PORT NOT IMPLEMENTED (F6 SETTINGS)",
            config::SIZE,config::RATE,&path,config::settings(),Some((ROM_SHA256,&battery)),assets.as_deref())?;
        Ok(())
    }
    #[cfg(not(feature = "desktop"))]
    {
        let _ = settings;
        bail!("desktop feature is disabled; use --headless FRAMES or build with --features desktop")
    }
}
