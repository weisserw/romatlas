mod game;
use anyhow::{Context, Result, bail};
use romatlas_port::{assets::Rom, input::InputState};
use std::path::PathBuf;

const ROM_SHA256: &str = "__ROM_SHA256__";

fn main() {
    if let Err(error) = run() {
        eprintln!("{}",serde_json::json!({"ok":false,"error":format!("{error:#}")}));
        std::process::exit(1);
    }
}
fn run() -> Result<()> {
    let mut rom = None;
    let mut assets = None;
    let mut settings = None;
    let mut frames: Option<usize> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--rom" => rom = Some(PathBuf::from(args.next().context("--rom needs a file")?)),
            "--assets" => assets = Some(PathBuf::from(args.next().context("--assets needs a directory")?)),
            "--settings" => settings = Some(PathBuf::from(args.next().context("--settings needs a file")?)),
            "--headless" => frames = Some(args.next().context("--headless needs a frame count")?.parse()?),
            "--help" => {
                println!("Usage: __GAME_NAME__ --rom FILE [--assets DIR] [--settings FILE] [--headless FRAMES]\nF1 opens desktop settings. This scaffold is not a ported game.");
                return Ok(());
            }
            _ => bail!("unknown argument: {arg}"),
        }
    }
    let rom = Rom::open(&rom.context("supply your matching ROM with --rom")?,ROM_SHA256)?;
    let mut port = game::Port::load(&rom,assets.as_deref())?;
    if let Some(frames) = frames {
        let frame = romatlas_port::headless(&mut port,frames,|_| InputState::default(),(160,144))?;
        println!("{}",serde_json::json!({"ok":true,"scaffold":true,"frames":frames,"width":frame.width,"height":frame.height}));
        return Ok(());
    }
    #[cfg(feature = "desktop")]
    {
        let path = match settings {
            Some(path) => path,
            None => romatlas_port::settings::default_path("__GAME_NAME__")?,
        };
        romatlas_port::desktop::run(&mut port,"__GAME_NAME__ - PORT NOT IMPLEMENTED (F1 SETTINGS)",
            (160,144),romatlas_port::timing::TickRate::DMG,&path,romatlas_port::settings::Settings::default())?;
        Ok(())
    }
    #[cfg(not(feature = "desktop"))]
    {
        let _ = settings;
        bail!("desktop feature is disabled; use --headless FRAMES or build with --features desktop")
    }
}
