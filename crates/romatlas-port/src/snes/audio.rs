//! Independent native audio building blocks at the DSP's 32 kHz sample rate.
//! A game supplies sequence commands and BRR blocks. This is not an SPC700 CPU
//! or a cycle-exact S-DSP: interpolation is linear, register races/key-on delays,
//! pitch modulation and echo RAM bus behavior are not modeled.
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

pub const SAMPLE_RATE: u32 = 32000;
/// Stateful BRR predictor, retained across blocks and loop boundaries.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BrrDecoder {
    history: [i32; 2],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BrrBlock {
    pub samples: [i16; 16],
    pub end: bool,
    pub looping: bool,
}
impl BrrDecoder {
    pub fn reset(&mut self) {
        self.history = [0; 2];
    }
    pub fn decode(&mut self, bytes: &[u8]) -> Result<BrrBlock> {
        ensure!(bytes.len() == 9, "BRR block must contain nine bytes");
        let range = bytes[0] >> 4;
        let filter = (bytes[0] >> 2) & 3;
        let mut samples = [0; 16];
        for (i, out) in samples.iter_mut().enumerate() {
            let nibble = ((bytes[1 + i / 2] >> if i % 2 == 0 { 4 } else { 0 }) & 15) as i32;
            let signed = if nibble & 8 != 0 { nibble - 16 } else { nibble };
            let mut value = if range <= 12 {
                (signed << range) >> 1
            } else if signed < 0 {
                -2048
            } else {
                0
            };
            let [p, q] = self.history;
            value += match filter {
                0 => 0,
                1 => p + ((-p) >> 4),
                2 => p * 2 + ((-p * 3) >> 5) - q + (q >> 4),
                _ => p * 2 + ((-p * 13) >> 6) - q + ((q * 3) >> 4),
            };
            // Clamp then wrap to signed 15 bits, before doubling to PCM16.
            value = value.clamp(-32768, 32767);
            value = (value << 17) >> 17;
            self.history = [value, p];
            *out = (value * 2) as i16;
        }
        Ok(BrrBlock {
            samples,
            end: bytes[0] & 1 != 0,
            looping: bytes[0] & 2 != 0,
        })
    }
}
const PERIODS: [u32; 32] = [
    0, 2048, 1536, 1280, 1024, 768, 640, 512, 384, 320, 256, 192, 160, 128, 96, 80, 64, 48, 40, 32,
    24, 20, 16, 12, 10, 8, 6, 5, 4, 3, 2, 1,
];
const OFFSETS: [u32; 32] = [
    0, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040,
    536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 0, 0,
];
fn fires(clock: u32, rate: u8) -> bool {
    let r = usize::from(rate & 31);
    PERIODS[r] != 0 && (clock + OFFSETS[r]).is_multiple_of(PERIODS[r])
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum EnvelopeStage {
    Attack,
    Decay,
    Sustain,
    Release,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Envelope {
    pub level: u16,
    pub stage: EnvelopeStage,
    hidden: i32,
}
impl Default for Envelope {
    fn default() -> Self {
        Self {
            level: 0,
            stage: EnvelopeStage::Release,
            hidden: 0,
        }
    }
}
impl Envelope {
    pub fn key_on(&mut self) {
        self.level = 0;
        self.hidden = 0;
        self.stage = EnvelopeStage::Attack;
    }
    pub fn key_off(&mut self) {
        self.stage = EnvelopeStage::Release;
    }
    /// Advance one sample using a shared down-counter (0..30719) and raw ADSR/GAIN.
    pub fn tick(&mut self, clock: u32, adsr1: u8, adsr2: u8, gain: u8) -> u16 {
        let mut value = i32::from(self.level);
        if self.stage == EnvelopeStage::Release {
            value = (value - 8).max(0);
            self.level = value as u16;
            self.hidden = value;
            return self.level;
        }
        let rate;
        if adsr1 & 128 != 0 {
            match self.stage {
                EnvelopeStage::Attack => {
                    rate = (adsr1 & 15) * 2 + 1;
                    value += if rate == 31 { 1024 } else { 32 };
                }
                EnvelopeStage::Decay => {
                    rate = ((adsr1 >> 4) & 7) * 2 + 16;
                    value -= 1 + (value.saturating_sub(1) >> 8);
                }
                _ => {
                    rate = adsr2 & 31;
                    value -= 1 + (value.saturating_sub(1) >> 8);
                }
            }
        } else if gain & 128 == 0 {
            rate = 31;
            value = i32::from(gain) * 16;
        } else {
            rate = gain & 31;
            match (gain >> 5) & 3 {
                0 => value -= 32,
                1 => value -= 1 + (value.saturating_sub(1) >> 8),
                2 => value += 32,
                _ => value += if self.hidden < 0x600 { 32 } else { 8 },
            }
        }
        self.hidden = value;
        if self.stage == EnvelopeStage::Decay && (value >> 8) == i32::from(adsr2 >> 5) {
            self.stage = EnvelopeStage::Sustain;
        }
        if value > 2047 && self.stage == EnvelopeStage::Attack {
            self.stage = EnvelopeStage::Decay;
        }
        if fires(clock, rate) {
            self.level = value.clamp(0, 2047) as u16;
        }
        self.level
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Noise {
    bits: u16,
}
impl Default for Noise {
    fn default() -> Self {
        Self { bits: 0x4000 }
    }
}
impl Noise {
    pub fn tick(&mut self, clock: u32, rate: u8) -> i16 {
        if fires(clock, rate) {
            self.bits = (self.bits >> 1) | (((self.bits ^ (self.bits >> 1)) & 1) << 14);
        }
        (self.bits << 1) as i16
    }
}
/// Owned BRR source. Loop offsets are block indices; validate before replacing
/// a playing voice. Decoder history is preserved at loop boundaries.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sample {
    blocks: Vec<[u8; 9]>,
    loop_block: usize,
}
impl Sample {
    pub fn new(bytes: &[u8], loop_block: usize) -> Result<Self> {
        ensure!(
            !bytes.is_empty() && bytes.len().is_multiple_of(9),
            "BRR source must contain complete blocks"
        );
        ensure!(loop_block < bytes.len() / 9, "BRR loop block out of range");
        ensure!(
            bytes[bytes.len() - 9] & 1 != 0,
            "BRR source needs a terminal end block"
        );
        ensure!(
            bytes[..bytes.len() - 9]
                .chunks_exact(9)
                .all(|b| b[0] & 1 == 0),
            "BRR end before final block"
        );
        Ok(Self {
            blocks: bytes
                .chunks_exact(9)
                .map(|b| b.try_into().unwrap())
                .collect(),
            loop_block,
        })
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Voice {
    pub pitch: u16,
    pub volume: [i8; 2],
    pub adsr1: u8,
    pub adsr2: u8,
    pub gain: u8,
    pub noise: bool,
    pub echo: bool,
    pub envelope: Envelope,
    source: Option<Sample>,
    decoder: BrrDecoder,
    block: usize,
    decoded: [i16; 16],
    position: usize,
    phase: u32,
    active: bool,
}
impl Voice {
    pub fn key_on(&mut self, source: Sample) {
        self.source = Some(source);
        self.decoder.reset();
        self.block = 0;
        self.position = 0;
        self.phase = 0;
        self.decoded = self
            .decoder
            .decode(&self.source.as_ref().unwrap().blocks[0])
            .unwrap()
            .samples;
        self.active = true;
        self.envelope.key_on();
    }
    pub fn key_off(&mut self) {
        self.envelope.key_off();
    }
    pub fn active(&self) -> bool {
        self.active
    }
    fn next(&mut self) {
        self.position += 1;
        if self.position < 16 {
            return;
        }
        self.position = 0;
        let source = self.source.as_ref().unwrap();
        let header = source.blocks[self.block][0];
        if header & 1 != 0 {
            if header & 2 == 0 {
                self.active = false;
                // A high pitch can still have whole source steps pending when
                // the one-shot ends. Keep the stopped cursor canonical too.
                self.phase = 0;
                self.envelope.level = 0;
                self.envelope.key_off();
                return;
            }
            self.block = source.loop_block;
        } else {
            self.block += 1;
        }
        self.decoded = self
            .decoder
            .decode(&source.blocks[self.block])
            .unwrap()
            .samples;
    }
    fn tick(&mut self, clock: u32, noise: i16) -> i32 {
        if !self.active {
            return 0;
        }
        let current = self.decoded[self.position];
        // Peek by cloning the predictor only at block boundaries, preserving live history.
        let next = if self.position < 15 {
            self.decoded[self.position + 1]
        } else {
            let source = self.source.as_ref().unwrap();
            let header = source.blocks[self.block][0];
            if header & 3 == 1 {
                0
            } else {
                let block = if header & 1 != 0 {
                    source.loop_block
                } else {
                    self.block + 1
                };
                self.decoder
                    .clone()
                    .decode(&source.blocks[block])
                    .unwrap()
                    .samples[0]
            }
        };
        let sample = if self.noise {
            i32::from(noise)
        } else {
            i32::from(current)
                + (((i32::from(next) - i32::from(current)) * self.phase as i32) >> 12)
        };
        let env = self.envelope.tick(clock, self.adsr1, self.adsr2, self.gain);
        let value = ((sample * i32::from(env)) >> 11) & !1;
        self.phase += u32::from(self.pitch.min(0x3fff));
        while self.phase >= 4096 && self.active {
            self.phase -= 4096;
            self.next();
        }
        value
    }
}
/// Eight-tap stereo FIR echo over a private delay buffer. No emulated ARAM writes.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Echo {
    delay: Vec<[i16; 2]>,
    position: usize,
    history: [[i16; 2]; 8],
    pub fir: [i8; 8],
    pub feedback: i8,
    pub volume: [i8; 2],
    pub write_enabled: bool,
}
impl Echo {
    /// Delay is in 32-kHz stereo frames; 512 corresponds to one EDL unit.
    pub fn new(delay_frames: usize) -> Result<Self> {
        ensure!(
            (1..=7680).contains(&delay_frames),
            "echo delay outside 1..7680 samples"
        );
        Ok(Self {
            delay: vec![[0; 2]; delay_frames],
            position: 0,
            history: [[0; 2]; 8],
            fir: [0; 8],
            feedback: 0,
            volume: [0; 2],
            write_enabled: true,
        })
    }
    fn tick(&mut self, input: [i32; 2]) -> [i32; 2] {
        self.history.rotate_right(1);
        self.history[0] = self.delay[self.position];
        let mut output = [0; 2];
        for channel in 0..2 {
            for tap in 0..8 {
                output[channel] +=
                    (i32::from(self.history[tap][channel] >> 1) * i32::from(self.fir[tap])) >> 6;
            }
            output[channel] = i32::from(output[channel] as i16) & !1;
            if self.write_enabled {
                self.delay[self.position][channel] =
                    (input[channel] + ((output[channel] * i32::from(self.feedback)) >> 7))
                        .clamp(-32768, 32767) as i16
                        & !1;
            }
        }
        self.position = (self.position + 1) % self.delay.len();
        output
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mixer {
    pub voices: [Voice; 8],
    pub master: [i8; 2],
    pub noise_rate: u8,
    pub echo: Option<Echo>,
    pub mute: bool,
    noise: Noise,
    buffered_noise: Option<i16>,
    clock: u32,
}
impl Default for Mixer {
    fn default() -> Self {
        Self {
            voices: std::array::from_fn(|_| Voice::default()),
            master: [127; 2],
            noise_rate: 0,
            echo: None,
            mute: false,
            noise: Noise::default(),
            buffered_noise: None,
            clock: 0,
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MixerSnapshot {
    version: u32,
    mixer: Mixer,
}
impl Mixer {
    /// Versioned complete native synthesis state, independent of a CPU/core.
    /// Includes live samples, BRR predictor, phase, envelopes, noise and echo.
    pub fn snapshot_json(&self) -> Result<String> {
        self.validate_snapshot()?;
        Ok(serde_json::to_string(&MixerSnapshot {
            version: 1,
            mixer: self.clone(),
        })?)
    }
    /// Parse and validate a candidate before replacing any live synthesis state.
    pub fn restore_snapshot_json(&mut self, json: &str) -> Result<()> {
        ensure!(json.len() <= 16 * 1024 * 1024, "mixer snapshot too large");
        let candidate: MixerSnapshot = serde_json::from_str(json)?;
        ensure!(candidate.version == 1, "unsupported mixer snapshot version");
        candidate.mixer.validate_snapshot()?;
        *self = candidate.mixer;
        Ok(())
    }
    fn validate_snapshot(&self) -> Result<()> {
        ensure!(
            self.clock < 30720 && self.noise_rate < 32 && self.noise.bits <= 0x7fff,
            "invalid mixer clock/noise"
        );
        for voice in &self.voices {
            ensure!(
                voice.envelope.level <= 2047 && voice.phase < 4096 && voice.position < 16,
                "invalid voice envelope/phase"
            );
            ensure!(
                voice
                    .decoder
                    .history
                    .iter()
                    .all(|value| (-16384..=16383).contains(value)),
                "invalid BRR predictor"
            );
            if let Some(source) = &voice.source {
                let bytes: Vec<u8> = source.blocks.iter().flatten().copied().collect();
                Sample::new(&bytes, source.loop_block)?;
                ensure!(
                    source.blocks.len() <= 7282 && voice.block < source.blocks.len(),
                    "invalid voice source cursor"
                );
            } else {
                ensure!(
                    !voice.active && voice.block == 0,
                    "active voice without source"
                );
            }
        }
        if let Some(echo) = &self.echo {
            ensure!(
                (1..=7680).contains(&echo.delay.len()) && echo.position < echo.delay.len(),
                "invalid echo delay/cursor"
            );
        }
        Ok(())
    }

    /// Start with held power-on noise (-32768), mixing before decrementing the
    /// global counter and generating the next noise value. Unlike `default`,
    /// this models drivers which already have a held DSP output at time zero.
    /// The generator still runs during silence, mute, and noise-rate changes.
    pub fn with_buffered_noise() -> Self {
        Self {
            buffered_noise: Some(i16::MIN),
            ..Self::default()
        }
    }

    /// Advance exactly one native 32-kHz stereo sample. All state is cloneable
    /// for deterministic checkpoints; output conversion/resampling belongs to the port.
    pub fn sample(&mut self) -> [i16; 2] {
        let noise = match self.buffered_noise {
            Some(value) => value,
            None => self.noise.tick(self.clock, self.noise_rate),
        };
        let mut dry = [0i32; 2];
        let mut send = [0i32; 2];
        for voice in &mut self.voices {
            let value = voice.tick(self.clock, noise);
            for channel in 0..2 {
                let sample = (value * i32::from(voice.volume[channel])) >> 7;
                dry[channel] = (dry[channel] + sample).clamp(-32768, 32767);
                if voice.echo {
                    send[channel] = (send[channel] + sample).clamp(-32768, 32767);
                }
            }
        }
        let mut output = [0; 2];
        let echo = if let Some(echo) = &mut self.echo {
            echo.tick(send)
        } else {
            [0; 2]
        };
        for channel in 0..2 {
            let wet_volume = self.echo.as_ref().map_or(0, |e| e.volume[channel]);
            let value = ((dry[channel] * i32::from(self.master[channel])) >> 7)
                + ((echo[channel] * i32::from(wet_volume)) >> 7);
            output[channel] = if self.mute {
                0
            } else {
                value.clamp(-32768, 32767) as i16
            };
        }
        self.clock = if self.clock == 0 {
            30719
        } else {
            self.clock - 1
        };
        if self.buffered_noise.is_some() {
            self.buffered_noise = Some(self.noise.tick(self.clock, self.noise_rate));
        }
        output
    }
    pub fn render(&mut self, frames: usize, output: &mut Vec<f32>) {
        for _ in 0..frames {
            for sample in self.sample() {
                output.push(f32::from(sample) / 32768.0);
            }
        }
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    #[test]
    fn ended_one_shot_remains_saveable_at_fractional_high_pitch() {
        let mut mixer = Mixer::default();
        mixer.voices[0].pitch = 0x3fff;
        mixer.voices[0].gain = 127;
        mixer.voices[0].volume = [100; 2];
        mixer.voices[0].key_on(
            Sample::new(&[0x21, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11], 0).unwrap(),
        );
        for _ in 0..20 {
            mixer.sample();
            let saved = mixer.snapshot_json().unwrap();
            let mut restored = Mixer::default();
            restored.restore_snapshot_json(&saved).unwrap();
            let mut branch = mixer.clone();
            assert_eq!(branch.sample(), restored.sample());
        }
        assert!(!mixer.voices[0].active());
    }
    #[test]
    fn complete_mixer_snapshot_forks_live_source_predictor_noise_and_echo() {
        let mut original = Mixer::with_buffered_noise();
        original.noise_rate = 31;
        let mut echo = Echo::new(3).unwrap();
        echo.fir[0] = 64;
        echo.feedback = 32;
        echo.volume = [40, -30];
        original.echo = Some(echo);
        original.voices[0].pitch = 5000;
        original.voices[0].gain = 127;
        original.voices[0].volume = [110, -100];
        original.voices[0].echo = true;
        original.voices[0].key_on(
            Sample::new(&[0x27, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11], 0).unwrap(),
        );
        for _ in 0..21 {
            original.sample();
        }
        let snapshot = original.snapshot_json().unwrap();
        let mut restored = Mixer::default();
        restored.restore_snapshot_json(&snapshot).unwrap();
        let expected: Vec<_> = (0..91).map(|_| original.sample()).collect();
        assert!(expected.iter().any(|sample| *sample != [0; 2]));
        assert_eq!(
            expected,
            (0..91).map(|_| restored.sample()).collect::<Vec<_>>()
        );
        assert_eq!(
            original.snapshot_json().unwrap(),
            restored.snapshot_json().unwrap()
        );
        restored.voices[0].key_off();
        assert_ne!(
            original.snapshot_json().unwrap(),
            restored.snapshot_json().unwrap()
        );
        let before = restored.snapshot_json().unwrap();
        let mut invalid: serde_json::Value = serde_json::from_str(&snapshot).unwrap();
        invalid["mixer"]["voices"][0]["position"] = serde_json::json!(16);
        assert!(
            restored
                .restore_snapshot_json(&invalid.to_string())
                .is_err()
        );
        assert_eq!(restored.snapshot_json().unwrap(), before);
        invalid["version"] = serde_json::json!(99);
        assert!(
            restored
                .restore_snapshot_json(&invalid.to_string())
                .is_err()
        );
        assert_eq!(restored.snapshot_json().unwrap(), before);
    }
}
