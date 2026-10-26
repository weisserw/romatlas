//! Optional SDL2 shell. Never invoked by headless execution or core tests.
use crate::{
    Game,
    audio::SampleClock,
    graphics::Frame,
    input::{Binding, InputLatch, InputMapper, PhysicalInput},
    settings::{Menu, MenuInput, Settings},
    timing::{Clock, TickRate},
};
use anyhow::{Result, anyhow, ensure};
use sdl2::{
    controller::{Axis, Button, GameController},
    event::{Event, WindowEvent},
    keyboard::{Keycode, Mod},
    pixels::PixelFormatEnum,
    rect::Rect,
    video::FullscreenType,
};
use std::{
    path::Path,
    time::{Duration, Instant},
};

fn sdl<T>(result: Result<T, String>) -> Result<T> {
    result.map_err(|error| anyhow!(error))
}
enum Pad {
    Mapped(GameController),
    Raw(sdl2::joystick::Joystick),
}
impl Pad {
    fn instance_id(&self) -> u32 {
        match self {
            Self::Mapped(p) => p.instance_id(),
            Self::Raw(p) => p.instance_id(),
        }
    }
    fn attached(&self) -> bool {
        match self {
            Self::Mapped(p) => p.attached(),
            Self::Raw(p) => p.attached(),
        }
    }
    fn identity(&self) -> (String, String) {
        match self {
            Self::Mapped(p) => (
                p.mapping().split(',').next().unwrap_or("").to_owned(),
                p.name(),
            ),
            Self::Raw(p) => (p.guid().string(), p.name()),
        }
    }
}
fn controllers(
    system: &sdl2::GameControllerSubsystem,
    joysticks: &sdl2::JoystickSubsystem,
) -> Vec<Pad> {
    (0..system.num_joysticks().unwrap_or(0))
        .filter_map(|id| {
            if system.is_game_controller(id) {
                system.open(id).ok().map(Pad::Mapped)
            } else {
                joysticks.open(id).ok().map(Pad::Raw)
            }
        })
        .collect()
}
fn choices(controllers: &[Pad]) -> Vec<(String, String)> {
    numbered_choices(controllers.iter().map(Pad::identity))
}
fn numbered_choices(devices: impl IntoIterator<Item = (String, String)>) -> Vec<(String, String)> {
    let mut counts = std::collections::BTreeMap::<String, u32>::new();
    devices
        .into_iter()
        .map(|(guid, name)| {
            let guid = guid.to_ascii_lowercase();
            let ordinal = counts.entry(guid.clone()).or_default();
            let choice = (
                format!("{guid}#{ordinal}"),
                format!("{}: {name}", *ordinal + 1),
            );
            *ordinal += 1;
            choice
        })
        .collect()
}
fn assignments(controllers: &[Pad], settings: &Settings) -> [Option<u32>; 4] {
    let ids = choices(controllers);
    let slots = assigned_indices(&ids, &settings.players);
    slots.map(|slot| slot.map(|index| controllers[index].instance_id()))
}
fn assigned_indices(devices: &[(String, String)], players: &[String; 4]) -> [Option<usize>; 4] {
    let mut used = std::collections::BTreeSet::new();
    std::array::from_fn(|player| {
        let id = &players[player];
        let found = devices.iter().enumerate().position(|(index, (key, _))| {
            !used.contains(&index)
                && if id == "auto" {
                    !players.contains(key)
                } else {
                    key == id
                }
        });
        if let Some(index) = found {
            used.insert(index);
        }
        found
    })
}
fn normalize_controller(settings: &mut Settings) {
    // Migrate the older primary selector once. Explicit player assignments win.
    if settings.players.iter().all(|id| id == "auto")
        && let Some(id) = settings.controller.take()
    {
        settings.players[0] = id;
    }
    for id in &mut settings.players {
        *id = id.to_ascii_lowercase();
        if id != "auto" && id != "none" && !id.contains('#') {
            id.push_str("#0");
        }
    }
}
fn physical(
    events: &sdl2::EventPump,
    controllers: &[Pad],
    assigned: [Option<u32>; 4],
) -> [PhysicalInput; 4] {
    let mut inputs: [PhysicalInput; 4] = Default::default();
    for scancode in events.keyboard_state().pressed_scancodes() {
        if let Some(key) = Keycode::from_scancode(scancode) {
            inputs[0].keys.insert(key.name());
        }
    }
    for (player, input) in inputs.iter_mut().enumerate() {
        if let Some(pad) = controllers
            .iter()
            .find(|p| Some(p.instance_id()) == assigned[player] && p.attached())
        {
            match pad {
                Pad::Mapped(p) => read_controller(p, input),
                Pad::Raw(p) => {
                    for button in 0..p.num_buttons() {
                        if p.button(button).unwrap_or(false) {
                            input.buttons.insert(format!("button{button}"));
                        }
                    }
                    for axis in 0..p.num_axes() {
                        let value = normalized_axis(p.axis(axis).unwrap_or(0));
                        input.axes.insert(format!("axis{axis}"), value);
                        if axis < 2 {
                            input
                                .axes
                                .insert(if axis == 0 { "leftx" } else { "lefty" }.into(), value);
                        }
                    }
                    for hat in 0..p.num_hats() {
                        let value = p.hat(hat).map_or(0, |h| h.to_raw());
                        for (bit, name) in
                            [(1, "dpup"), (2, "dpright"), (4, "dpdown"), (8, "dpleft")]
                        {
                            if value & bit != 0 {
                                input.buttons.insert(name.into());
                            }
                        }
                    }
                }
            }
        }
    }
    inputs
}
fn normalized_axis(raw: i16) -> f32 {
    f32::from(raw) / if raw < 0 { 32768.0 } else { 32767.0 }
}
fn raw_index(name: &str, prefix: &str) -> bool {
    name.strip_prefix(prefix).is_some_and(|n| {
        !n.is_empty() && n.bytes().all(|c| c.is_ascii_digit()) && n.parse::<u16>().is_ok()
    })
}

fn read_controller(controller: &GameController, input: &mut PhysicalInput) {
    for button in [
        Button::A,
        Button::B,
        Button::X,
        Button::Y,
        Button::Back,
        Button::Guide,
        Button::Start,
        Button::LeftStick,
        Button::RightStick,
        Button::LeftShoulder,
        Button::RightShoulder,
        Button::DPadUp,
        Button::DPadDown,
        Button::DPadLeft,
        Button::DPadRight,
    ] {
        if controller.button(button) {
            input.buttons.insert(button.string());
        }
    }
    for axis in [
        Axis::LeftX,
        Axis::LeftY,
        Axis::RightX,
        Axis::RightY,
        Axis::TriggerLeft,
        Axis::TriggerRight,
    ] {
        let raw = controller.axis(axis);
        input.axes.insert(
            axis.string(),
            f32::from(raw) / if raw < 0 { 32768.0 } else { 32767.0 },
        );
    }
}

fn filter_players(
    latches: &mut [InputLatch; 4],
    physical: [PhysicalInput; 4],
    dead_zone: f32,
) -> [PhysicalInput; 4] {
    std::array::from_fn(|player| latches[player].filter(physical[player].clone(), dead_zone))
}

/// Aspect-preserving fit, including windows smaller than the original frame.
pub fn viewport(logical: (u32, u32), output: (u32, u32), integer_scale: bool) -> Rect {
    viewport_with_aspect(logical, output, integer_scale, None)
}
/// Fit using a display aspect while retaining vertical native-pixel integer steps.
pub fn viewport_with_aspect(
    logical: (u32, u32),
    output: (u32, u32),
    integer_scale: bool,
    aspect: Option<(u32, u32)>,
) -> Rect {
    let height = f64::from(logical.1.max(1));
    let width = match aspect {
        Some((numerator, denominator)) if numerator != 0 && denominator != 0 => {
            height * f64::from(numerator) / f64::from(denominator)
        }
        _ => f64::from(logical.0.max(1)),
    };
    let scale = (f64::from(output.0) / width).min(f64::from(output.1) / height);
    let scale = if integer_scale && scale >= 1.0 {
        scale.floor()
    } else {
        scale
    };
    let width = (width * scale).round() as u32;
    let height = (height * scale).round() as u32;
    Rect::new(
        (output.0.saturating_sub(width) / 2) as i32,
        (output.1.saturating_sub(height) / 2) as i32,
        width.max(1),
        height.max(1),
    )
}

pub fn run(
    game: &mut impl Game,
    title: &str,
    size: (u32, u32),
    rate: TickRate,
    settings_path: &Path,
    defaults: Settings,
) -> Result<()> {
    run_with_save(game, title, size, rate, settings_path, defaults, None)
}

/// Battery-enabled frontend; failures are returned visibly to the caller.
#[allow(clippy::too_many_arguments)]
pub fn run_with_save(
    game: &mut impl Game,
    title: &str,
    size: (u32, u32),
    rate: TickRate,
    settings_path: &Path,
    defaults: Settings,
    battery: Option<(&str, &Path)>,
) -> Result<()> {
    run_with_assets(
        game,
        title,
        size,
        rate,
        settings_path,
        defaults,
        battery,
        None,
    )
}

/// Include override contents in snapshot compatibility identity.
#[allow(clippy::too_many_arguments)]
pub fn run_with_assets(
    game: &mut impl Game,
    title: &str,
    size: (u32, u32),
    rate: TickRate,
    settings_path: &Path,
    defaults: Settings,
    battery: Option<(&str, &Path)>,
    assets: Option<&Path>,
) -> Result<()> {
    if let Some((identity, path)) = battery {
        crate::saves::load(game, identity, path)?;
    }
    defaults.validate()?;
    let mut settings = if settings_path.try_exists()? {
        Settings::load(settings_path)?
    } else {
        defaults.clone()
    };
    normalize_controller(&mut settings);
    validate_bindings(&settings)?;
    let context = sdl(sdl2::init())?;
    let video = sdl(context.video())?;
    let pad_system = sdl(context.game_controller())?;
    let joystick_system = sdl(context.joystick())?;
    fit_settings(
        &mut settings,
        game.display_aspect_ratio().unwrap_or((size.0, size.1)),
    );
    let mut window = video
        .window(title, settings.width, settings.height)
        .position_centered()
        .resizable()
        .build()?;
    sdl(window.set_fullscreen(if settings.fullscreen {
        FullscreenType::Desktop
    } else {
        FullscreenType::Off
    }))?;
    let mut canvas = window.into_canvas().present_vsync().build()?;
    let textures = canvas.texture_creator();
    let mut frame = Frame::new(size.0, size.1)?;
    let mut texture = textures.create_texture_streaming(PixelFormatEnum::RGBA32, size.0, size.1)?;
    texture.set_scale_mode(sdl2::render::ScaleMode::Nearest);
    let mut menu_frame = Frame::new(640, 480)?;
    let mut menu_texture = textures.create_texture_streaming(PixelFormatEnum::RGBA32, 640, 480)?;
    menu_texture.set_scale_mode(sdl2::render::ScaleMode::Nearest);
    let audio_system = match context.audio() {
        Ok(system) => Some(system),
        Err(error) => {
            eprintln!("Audio subsystem unavailable: {error}");
            None
        }
    };
    let desired = sdl2::audio::AudioSpecDesired {
        freq: Some(48000),
        channels: Some(2),
        samples: Some(1024),
    };
    let audio = audio_system.as_ref().and_then(|system| {
        match system.open_queue::<f32, _>(None, &desired) {
            Ok(queue) => {
                queue.resume();
                Some(queue)
            }
            Err(error) => {
                eprintln!("Audio playback unavailable: {error}");
                None
            }
        }
    });
    let sample_rate = audio
        .as_ref()
        .map_or(48000, |queue| queue.spec().freq as u32);
    let mut sample_clock = SampleClock::new(rate, sample_rate)?;
    let mut clock = Clock::new(rate, 8)?;
    let mut pads = controllers(&pad_system, &joystick_system);
    let mut controller_choices = choices(&pads);
    let mut active_pad = assignments(&pads, &settings);
    let mut events = sdl(context.event_pump())?;
    let mut mapper = InputMapper::default();
    let mut latches: [InputLatch; 4] = Default::default();
    let mut menu = Menu::default();
    let mut focused = true;
    let mut paused_before = false;
    let mut previous = Instant::now();
    let mut user_paused = false;
    let state_path = settings_path.with_file_name("state.json");
    let identity = crate::debug::identity(battery.map_or("native", |(rom, _)| rom), assets)?;
    let mut message = String::new();
    let mut capture_baseline: Option<Vec<PhysicalInput>> = None;
    'running: loop {
        let mut changed = false;
        let mut boundary = false;
        for event in events.poll_iter() {
            let mut menu_command = None;
            let mut binding = None;
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window {
                    win_event: WindowEvent::FocusLost,
                    ..
                } => {
                    focused = false;
                    boundary = true;
                }
                Event::Window {
                    win_event: WindowEvent::FocusGained,
                    ..
                } => {
                    focused = true;
                    boundary = true;
                }
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } if !settings.fullscreen
                    && (160..=8192).contains(&w)
                    && (144..=8192).contains(&h) =>
                {
                    settings.width = w as u32;
                    settings.height = h as u32;
                    changed = true;
                }
                Event::ControllerDeviceAdded { .. }
                | Event::ControllerDeviceRemoved { .. }
                | Event::JoyDeviceAdded { .. }
                | Event::JoyDeviceRemoved { .. } => {
                    pads = controllers(&pad_system, &joystick_system);
                    controller_choices = choices(&pads);
                    active_pad = assignments(&pads, &settings);
                    boundary = true;
                    if menu.rebinding.is_some() {
                        menu.rebinding = None;
                    }
                }
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    keymod,
                    ..
                } if focused => {
                    let plain = !keymod.intersects(
                        Mod::LSHIFTMOD
                            | Mod::RSHIFTMOD
                            | Mod::LCTRLMOD
                            | Mod::RCTRLMOD
                            | Mod::LALTMOD
                            | Mod::RALTMOD
                            | Mod::LGUIMOD
                            | Mod::RGUIMOD,
                    );
                    if plain
                        && menu.rebinding.is_none()
                        && matches!(
                            key,
                            Keycode::F1 | Keycode::F2 | Keycode::F3 | Keycode::F4 | Keycode::F5
                        )
                        && !menu.open
                    {
                        boundary = true;
                        let result = match key {
                            Keycode::F1 => {
                                user_paused = false;
                                message.clear();
                                Ok(())
                            }
                            Keycode::F2 => {
                                user_paused = true;
                                message = "PAUSED - F1 RESUME".into();
                                Ok(())
                            }
                            Keycode::F3 => game.reset().and_then(|()| {
                                sample_clock = SampleClock::new(rate, sample_rate)?;
                                user_paused = false;
                                message.clear();
                                Ok(())
                            }),
                            Keycode::F4 => crate::states::capture(game, &sample_clock, &identity)
                                .and_then(|bytes| {
                                    crate::settings::write_atomic(&state_path, &bytes)
                                })
                                .map(|()| {
                                    message = "STATE SAVED".into();
                                }),
                            Keycode::F5 => {
                                crate::assets::read_bounded(&state_path, crate::states::MAX_BYTES)
                                    .and_then(|bytes| {
                                        crate::states::restore(
                                            game,
                                            &mut sample_clock,
                                            &identity,
                                            &bytes,
                                        )
                                    })
                                    .map(|()| {
                                        user_paused = true;
                                        message = "STATE RESTORED - F1 RESUME".into();
                                    })
                            }
                            _ => unreachable!(),
                        };
                        if let Err(error) = result {
                            message = format!("{error:#}");
                            eprintln!("{message}");
                        }
                    } else if plain && key == Keycode::F6 {
                        menu.toggle();
                        boundary = true;
                    } else if menu.open {
                        if menu.rebinding.is_some() {
                            if key == Keycode::Escape {
                                menu_command = Some(MenuInput::Cancel);
                            } else {
                                binding = Some(Binding::Key { name: key.name() });
                            }
                        } else {
                            menu_command = match key {
                                Keycode::Up => Some(MenuInput::Up),
                                Keycode::Down => Some(MenuInput::Down),
                                Keycode::Left => Some(MenuInput::Left),
                                Keycode::Right => Some(MenuInput::Right),
                                Keycode::Return => Some(MenuInput::Accept),
                                Keycode::Escape => Some(MenuInput::Cancel),
                                Keycode::Backspace => Some(MenuInput::ClearBinding),
                                _ => None,
                            };
                        }
                    }
                }
                Event::ControllerButtonDown { which, button, .. }
                    if focused && active_pad.contains(&Some(which)) =>
                {
                    if button == Button::Guide {
                        menu.toggle();
                        boundary = true;
                    } else if menu.open && menu.rebinding.is_some() {
                        // Capture is polled across every connected device below.
                    } else if menu.open {
                        menu_command = match button {
                            Button::DPadUp => Some(MenuInput::Up),
                            Button::DPadDown => Some(MenuInput::Down),
                            Button::DPadLeft => Some(MenuInput::Left),
                            Button::DPadRight => Some(MenuInput::Right),
                            Button::A => Some(MenuInput::Accept),
                            Button::B => Some(MenuInput::Cancel),
                            Button::X => Some(MenuInput::ClearBinding),
                            _ => None,
                        };
                    }
                }
                _ => {}
            }
            if let Some(command) = menu_command {
                changed |= menu.handle(command, &mut settings, &defaults, &controller_choices);
            }
            if let Some(binding) = binding {
                changed |= menu.capture(binding, &mut settings);
            }
        }
        if menu.rebinding.is_some() {
            // Capture each device independently, even when no player is assigned.
            let current = capture_inputs(&events, &pads);
            if let Some(baseline) = &mut capture_baseline {
                if let Some(binding) = capture_binding(baseline, &current) {
                    changed |= menu.capture(binding, &mut settings);
                }
            } else {
                capture_baseline = Some(current);
            }
        } else {
            capture_baseline = None;
        }
        if changed {
            normalize_controller(&mut settings);
            fit_settings(
                &mut settings,
                game.display_aspect_ratio().unwrap_or((size.0, size.1)),
            );
            settings.save(settings_path)?;
            let window = canvas.window_mut();
            sdl(window.set_fullscreen(if settings.fullscreen {
                FullscreenType::Desktop
            } else {
                FullscreenType::Off
            }))?;
            if !settings.fullscreen {
                window.set_size(settings.width, settings.height)?;
            }
            let pad = assignments(&pads, &settings);
            boundary |= pad != active_pad;
            active_pad = pad;
        }
        let now = Instant::now();
        let elapsed = now.duration_since(previous);
        previous = now;
        let physical = physical(&events, &pads, active_pad);
        let paused = user_paused || menu.open || (!focused && settings.pause_on_focus_loss);
        if boundary || paused != paused_before {
            clock.reset();
            mapper.clear();
            for (latch, player) in latches.iter_mut().zip(&physical) {
                latch.suppress(player);
            }
            if let Some(queue) = &audio {
                queue.clear();
            }
        }
        paused_before = paused;
        let physical = filter_players(&mut latches, physical, settings.dead_zone);
        if !paused {
            let advancement = clock.advance(if boundary { Duration::ZERO } else { elapsed });
            for _ in 0..advancement.ticks {
                let input = if focused {
                    mapper.sample_players(&physical, &settings)
                } else {
                    mapper.sample_players(&Default::default(), &settings)
                };
                game.tick_players(&input);
                let sample_frames = sample_clock.next_tick();
                let mut samples = Vec::new();
                game.audio(sample_frames, sample_rate, &mut samples);
                ensure!(
                    samples.len() == sample_frames * 2,
                    "game audio must supply requested stereo sample count"
                );
                for sample in &mut samples {
                    *sample = if sample.is_finite() {
                        (*sample * settings.volume).clamp(-1.0, 1.0)
                    } else {
                        0.0
                    };
                }
                if let Some(queue) = &audio {
                    if queue.size() > sample_rate * 2 {
                        queue.clear();
                    }
                    sdl(queue.queue_audio(&samples))?;
                }
            }
        }
        if let Some((identity, path)) = battery {
            crate::saves::flush(game, identity, path)?;
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        let output = sdl(canvas.output_size())?;
        if menu.open {
            draw_menu(&mut menu_frame, &menu, &settings, &controller_choices);
            menu_texture.update(None, &menu_frame.pixels, 640 * 4)?;
            sdl(canvas.copy(&menu_texture, None, viewport((640, 480), output, false)))?;
        } else {
            game.render(&mut frame);
            texture.update(None, &frame.pixels, size.0 as usize * 4)?;
            sdl(canvas.copy(
                &texture,
                None,
                viewport_with_aspect(size, output, false, game.display_aspect_ratio()),
            ))?;
        }
        if !menu.open && (!message.is_empty() || user_paused) {
            // Window title remains readable at any output size without touching game pixels.
            let _ = canvas
                .window_mut()
                .set_title(&format!("{title} - {message}"));
        } else {
            let _ = canvas.window_mut().set_title(title);
        }
        canvas.present();
        std::thread::sleep(Duration::from_millis(1));
    }
    settings.save(settings_path)?;
    if let Some((identity, path)) = battery {
        crate::saves::flush(game, identity, path)?;
    }
    Ok(())
}

fn fit_settings(settings: &mut Settings, aspect: (u32, u32)) {
    settings.width = ((u64::from(settings.height) * u64::from(aspect.0.max(1))
        + u64::from(aspect.1.max(1)) / 2)
        / u64::from(aspect.1.max(1)))
    .clamp(160, 8192) as u32;
}
fn capture_inputs(events: &sdl2::EventPump, pads: &[Pad]) -> Vec<PhysicalInput> {
    pads.iter()
        .map(|pad| {
            physical(
                events,
                std::slice::from_ref(pad),
                [Some(pad.instance_id()), None, None, None],
            )[0]
            .clone()
        })
        .collect()
}
fn capture_binding(baseline: &mut [PhysicalInput], current: &[PhysicalInput]) -> Option<Binding> {
    if baseline.len() != current.len() {
        return None;
    }
    for (before, now) in baseline.iter_mut().zip(current) {
        for button in &now.buttons {
            if !before.buttons.contains(button) {
                return Some(Binding::Button {
                    name: button.clone(),
                });
            }
        }
        before.buttons.retain(|b| now.buttons.contains(b));
        for (name, value) in &now.axes {
            let initial = *before.axes.get(name).unwrap_or(&0.0);
            if initial.abs() < 0.5 && value.abs() > 0.75 {
                return Some(Binding::Axis {
                    name: name.clone(),
                    direction: if *value < 0.0 { -1 } else { 1 },
                });
            }
            if value.abs() < 0.5 {
                before.axes.insert(name.clone(), 0.0);
            }
        }
    }
    None
}

fn validate_bindings(settings: &Settings) -> Result<()> {
    for binding in settings.bindings.values().flatten() {
        let valid = match binding {
            Binding::Key { name } => Keycode::from_name(name).is_some(),
            Binding::Button { name } => {
                Button::from_string(name).is_some() || raw_index(name, "button")
            }
            Binding::Axis { name, .. } => {
                Axis::from_string(name).is_some() || raw_index(name, "axis")
            }
        };
        ensure!(valid, "unknown SDL input binding: {binding:?}");
    }
    Ok(())
}

fn draw_menu(
    frame: &mut Frame,
    menu: &Menu,
    settings: &Settings,
    controllers: &[(String, String)],
) {
    frame.clear([20, 24, 32, 255]);
    text(
        frame,
        12,
        12,
        "SETTINGS - F6 / GUIDE / ESC TO CLOSE",
        [240, 240, 240, 255],
    );
    text(
        frame,
        12,
        32,
        "ARROWS / DPAD, ENTER / A, BACKSPACE / X CLEAR",
        [160, 180, 200, 255],
    );
    let labels = menu.labels(settings, controllers);
    let start = menu.selected / 20 * 20;
    for (index, label) in labels.iter().enumerate().skip(start).take(20) {
        let color = if index == menu.selected {
            [255, 220, 80, 255]
        } else {
            [220, 220, 220, 255]
        };
        text(
            frame,
            12,
            62 + (index - start) as i32 * 18,
            &format!(
                "{} {}",
                if index == menu.selected { ">" } else { " " },
                label
            ),
            color,
        );
    }
    if menu.rebinding.is_none() {
        text(
            frame,
            12,
            444,
            "F1 RESUME F2 PAUSE F3 RESET F4 SAVE F5 RESTORE",
            [160, 180, 200, 255],
        );
    }
    if let Some(action) = &menu.rebinding {
        text(
            frame,
            12,
            444,
            &format!("BIND {action}: PRESS KEY / PAD / AXIS. ESC CANCELS"),
            [255, 220, 80, 255],
        );
    }
}

// Original, tiny 5x7 uppercase menu font. No ROM-derived font or asset dependency.
fn text(frame: &mut Frame, x: i32, y: i32, text: &str, color: [u8; 4]) {
    for (column, ch) in text.chars().take(51).enumerate() {
        let rows = glyph(ch.to_ascii_uppercase());
        for (row, bits) in rows.iter().enumerate() {
            for bit in 0..5 {
                if bits & (1 << (4 - bit)) != 0 {
                    for dy in 0..2 {
                        for dx in 0..2 {
                            frame.pixel(
                                x + column as i32 * 12 + bit * 2 + dx,
                                y + row as i32 * 2 + dy,
                                color,
                            );
                        }
                    }
                }
            }
        }
    }
}
fn glyph(ch: char) -> [u8; 7] {
    match ch {
        'A' => [14, 17, 17, 31, 17, 17, 17],
        'B' => [30, 17, 17, 30, 17, 17, 30],
        'C' => [14, 17, 16, 16, 16, 17, 14],
        'D' => [30, 17, 17, 17, 17, 17, 30],
        'E' => [31, 16, 16, 30, 16, 16, 31],
        'F' => [31, 16, 16, 30, 16, 16, 16],
        'G' => [14, 17, 16, 23, 17, 17, 15],
        'H' => [17, 17, 17, 31, 17, 17, 17],
        'I' => [14, 4, 4, 4, 4, 4, 14],
        'J' => [7, 2, 2, 2, 2, 18, 12],
        'K' => [17, 18, 20, 24, 20, 18, 17],
        'L' => [16, 16, 16, 16, 16, 16, 31],
        'M' => [17, 27, 21, 21, 17, 17, 17],
        'N' => [17, 25, 21, 19, 17, 17, 17],
        'O' => [14, 17, 17, 17, 17, 17, 14],
        'P' => [30, 17, 17, 30, 16, 16, 16],
        'Q' => [14, 17, 17, 17, 21, 18, 13],
        'R' => [30, 17, 17, 30, 20, 18, 17],
        'S' => [15, 16, 16, 14, 1, 1, 30],
        'T' => [31, 4, 4, 4, 4, 4, 4],
        'U' => [17, 17, 17, 17, 17, 17, 14],
        'V' => [17, 17, 17, 17, 17, 10, 4],
        'W' => [17, 17, 17, 21, 21, 21, 10],
        'X' => [17, 17, 10, 4, 10, 17, 17],
        'Y' => [17, 17, 10, 4, 4, 4, 4],
        'Z' => [31, 1, 2, 4, 8, 16, 31],
        '0' => [14, 17, 19, 21, 25, 17, 14],
        '1' => [4, 12, 4, 4, 4, 4, 14],
        '2' => [14, 17, 1, 2, 4, 8, 31],
        '3' => [30, 1, 1, 14, 1, 1, 30],
        '4' => [2, 6, 10, 18, 31, 2, 2],
        '5' => [31, 16, 16, 30, 1, 1, 30],
        '6' => [14, 16, 16, 30, 17, 17, 14],
        '7' => [31, 1, 2, 4, 8, 8, 8],
        '8' => [14, 17, 17, 14, 17, 17, 14],
        '9' => [14, 17, 17, 15, 1, 1, 14],
        '>' => [16, 8, 4, 2, 4, 8, 16],
        '-' => [0, 0, 0, 31, 0, 0, 0],
        ':' => [0, 4, 4, 0, 4, 4, 0],
        '/' => [1, 2, 2, 4, 8, 8, 16],
        '%' => [17, 2, 4, 8, 16, 17, 0],
        ',' => [0, 0, 0, 0, 4, 4, 8],
        '.' => [0, 0, 0, 0, 0, 4, 4],
        '_' => [0, 0, 0, 0, 0, 0, 31],
        ' ' => [0; 7],
        _ => [14, 17, 1, 2, 4, 0, 4],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assignment_reserves_explicit_devices_and_allows_none() {
        let devices = vec![("a".into(), "ONE".into()), ("b".into(), "TWO".into())];
        assert_eq!(
            assigned_indices(
                &devices,
                &["auto".into(), "a".into(), "none".into(), "auto".into()]
            ),
            [Some(1), Some(0), None, None]
        );
        assert_eq!(
            assigned_indices(
                &devices,
                &[
                    "none".into(),
                    "missing".into(),
                    "auto".into(),
                    "auto".into()
                ]
            ),
            [None, None, Some(0), Some(1)]
        );
    }
    #[test]
    fn raw_capture_waits_for_release_and_accepts_extra_axes_and_buttons() -> Result<()> {
        let mut held = PhysicalInput::default();
        held.buttons.insert("button19".into());
        held.axes.insert("axis5".into(), -1.0);
        let mut baseline = vec![held.clone()];
        assert!(capture_binding(&mut baseline, &[held.clone()]).is_none());
        let neutral = PhysicalInput::default();
        // Axis presence is retained by SDL, including when centered.
        let mut centered = neutral.clone();
        centered.axes.insert("axis5".into(), 0.0);
        assert!(capture_binding(&mut baseline, &[centered]).is_none());
        assert!(
            matches!(capture_binding(&mut baseline, &[held]),Some(Binding::Button{name}) if name=="button19")
        );
        let mut axis = neutral.clone();
        axis.axes.insert("axis5".into(), -0.9);
        assert!(
            matches!(capture_binding(&mut baseline, &[axis]),Some(Binding::Axis{name,direction:-1}) if name=="axis5")
        );
        let mut settings = Settings::default();
        settings.bindings.get_mut("a").unwrap().extend([
            Binding::Button {
                name: "button19".into(),
            },
            Binding::Axis {
                name: "axis5".into(),
                direction: -1,
            },
        ]);
        validate_bindings(&settings)?;
        Ok(())
    }
    #[test]
    fn free_scaling_fills_default_widescreen_height() {
        let mut settings = Settings {
            height: 540,
            ..Settings::default()
        };
        fit_settings(&mut settings, (16, 9));
        assert_eq!((settings.width, settings.height), (960, 540));
        let rect = viewport_with_aspect((341, 240), (960, 540), false, Some((16, 9)));
        assert_eq!((rect.width(), rect.height()), (960, 540));
    }

    #[test]
    fn identical_controller_models_get_separate_saved_choices() -> Result<()> {
        let guid = "a".repeat(32);
        let choices =
            numbered_choices([(guid.clone(), "PAD".into()), (guid.clone(), "PAD".into())]);
        assert_eq!(choices[0].0, format!("{guid}#0"));
        assert_eq!(choices[1].0, format!("{guid}#1"));
        let defaults = Settings::default();
        let mut settings = defaults.clone();
        let mut menu = Menu {
            open: true,
            selected: 4,
            ..Default::default()
        };
        menu.handle(MenuInput::Right, &mut settings, &defaults, &choices);
        menu.handle(MenuInput::Right, &mut settings, &defaults, &choices);
        menu.handle(MenuInput::Right, &mut settings, &defaults, &choices);
        assert_eq!(settings.players[0], choices[1].0);
        settings.validate()?;
        assert!(menu.labels(&settings, &choices)[4].contains("2: PAD"));
        settings.players = std::array::from_fn(|_| "auto".into());
        settings.controller = Some(guid.to_ascii_uppercase());
        normalize_controller(&mut settings);
        assert_eq!(settings.players[0], choices[0].0);
        Ok(())
    }

    #[test]
    fn persisted_default_bindings_match_sdl_names_without_device_initialization() -> Result<()> {
        let settings = Settings::default();
        validate_bindings(&settings)?;
        for binding in settings.bindings.values().flatten() {
            match binding {
                Binding::Key { name } => {
                    assert_eq!(Keycode::from_name(name).unwrap().name(), *name)
                }
                Binding::Button { name } => {
                    assert_eq!(Button::from_string(name).unwrap().string(), *name)
                }
                Binding::Axis { name, .. } => {
                    assert_eq!(Axis::from_string(name).unwrap().string(), *name)
                }
            }
        }
        Ok(())
    }
}
