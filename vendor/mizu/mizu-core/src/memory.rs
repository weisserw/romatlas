mod dma;
pub mod interrupts;

use save_state::Savable;

use std::cell::RefCell;
use std::rc::Rc;

pub use interrupts::{InterruptManager, InterruptType};

use crate::apu::{Apu, AudioBuffers};
use crate::cartridge::Cartridge;
use crate::cpu::CpuBusProvider;
use crate::joypad::{Joypad, JoypadButton};
use crate::ppu::Ppu;
use crate::serial::{Serial, SerialDevice};
use crate::timer::Timer;
use crate::GameBoyConfig;
use dma::{BusType, Hdma, OamDma};
use interrupts::Interrupts;

#[derive(Default, Savable)]
struct BootRom {
    enabled: bool,
    data: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq, Debug, Savable)]
enum Speed {
    Normal,
    Double,
}

impl Default for Speed {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default, Savable)]
struct SpeedController {
    preparing_switch: bool,
    current_speed: Speed,
    settling_cycles: u16,
}

impl SpeedController {
    fn read_key1(&self) -> u8 {
        0x7E | ((self.current_speed as u8) << 7) | self.preparing_switch as u8
    }

    fn write_key1(&mut self, data: u8) {
        self.preparing_switch = data & 1 != 0;
    }

    fn preparing_switch(&self) -> bool {
        self.preparing_switch
    }

    fn current_speed(&self) -> Speed {
        self.current_speed
    }

    fn commit_speed_switch(&mut self) {
        assert!(self.preparing_switch);
        self.current_speed = match self.current_speed {
            Speed::Normal => Speed::Double,
            Speed::Double => Speed::Normal,
        };
        self.preparing_switch = false;
        self.settling_cycles = 2050;
    }
}

#[derive(Savable)]
struct Wram {
    data: [u8; 0x8000],
    bank: u8,
}

impl Default for Wram {
    fn default() -> Self {
        Self {
            data: [0; 0x8000],
            bank: 1,
        }
    }
}

impl Wram {
    fn read_wram0(&self, addr: u16) -> u8 {
        self.data[addr as usize & 0xFFF]
    }

    fn read_wramx(&self, addr: u16) -> u8 {
        self.data[(0x1000 * self.bank as usize) + (addr as usize & 0xFFF)]
    }

    fn write_wram0(&mut self, addr: u16, data: u8) {
        self.data[addr as usize & 0xFFF] = data;
    }

    fn write_wramx(&mut self, addr: u16, data: u8) {
        self.data[(0x1000 * self.bank as usize) + (addr as usize & 0xFFF)] = data;
    }

    fn set_wram_bank(&mut self, data: u8) {
        self.bank = data & 7;
        // bank cannot be 0
        if self.bank == 0 {
            self.bank = 1;
        }
    }

    fn get_wram_bank(&self) -> u8 {
        0xF8 | self.bank
    }
}

#[derive(Savable)]
struct Lock {
    during_boot: bool,
    is_dmg_mode: bool,
    written_to: bool,
}

impl Default for Lock {
    fn default() -> Self {
        Self {
            during_boot: true,
            is_dmg_mode: false,
            written_to: false,
        }
    }
}

impl Lock {
    fn write(&mut self, data: u8) {
        // TODO: check if this registers lock after bootrom or after
        //  first write
        if !self.written_to {
            self.written_to = true;
            self.is_dmg_mode = data & 0x4 != 0;
        }
    }

    fn finish_boot(&mut self) {
        self.during_boot = false;
    }

    /// The bootrom can write to both CGB and DMG registers during bootrom,
    /// so this should return true if we are still in bootrom
    fn is_cgb_mode(&self) -> bool {
        !self.is_dmg_mode || self.during_boot
    }
}

#[derive(Savable)]
struct UnknownRegister {
    data: u8,
    mask: u8,
}

impl UnknownRegister {
    fn new(mask: u8) -> Self {
        Self { data: 0, mask }
    }

    fn read(&self) -> u8 {
        (self.data & self.mask) | !self.mask
    }

    fn write(&mut self, data: u8) {
        self.data = data & self.mask
    }
}

// made this into a structure just to be easier to implement `Savable`
#[derive(Savable)]
struct UnknownRegisters {
    registers: [UnknownRegister; 4],
}

impl UnknownRegisters {
    pub fn new(masks: [u8; 4]) -> Self {
        Self {
            registers: [
                UnknownRegister::new(masks[0]),
                UnknownRegister::new(masks[1]),
                UnknownRegister::new(masks[2]),
                UnknownRegister::new(masks[3]),
            ],
        }
    }
}

impl std::ops::Index<usize> for UnknownRegisters {
    type Output = UnknownRegister;

    fn index(&self, index: usize) -> &Self::Output {
        &self.registers[index]
    }
}

impl std::ops::IndexMut<usize> for UnknownRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.registers[index]
    }
}

/// ROMAtlas: real CPU bus operation, sampled before peripheral time advances.
#[derive(Clone,Debug)]
pub struct BusEvent {pub kind:u8,pub address:u16,pub value:u8,pub rom_offset:Option<usize>,pub cycles:u32,pub banks:Option<(usize,usize)>,pub fixed_banks:Option<(usize,usize)>,pub target:Option<u16>,pub location:Option<BusLocation>,pub target_location:Option<BusLocation>,pub blocked:bool}
#[derive(Clone,Debug)]
pub struct BusLocation{pub space:&'static str,pub offset:usize,pub bank:Option<usize>}
#[derive(Savable)]
pub struct Bus {
    #[savable(skip)]
    trace_enabled:bool,
    #[savable(skip)]
    last_fetch:Option<usize>,
    #[savable(skip)]
    bus_events:Vec<BusEvent>,
    cartridge: Cartridge,
    ppu: Ppu,
    wram: Wram,
    interrupts: Interrupts,
    timer: Timer,
    joypad: Joypad,
    serial: Serial,
    oam_dma: OamDma,
    hdma: Hdma,
    apu: Apu,
    hram: [u8; 127],
    boot_rom: BootRom,
    speed_controller: SpeedController,
    lock: Lock,
    unknown_registers: UnknownRegisters,

    #[savable(skip)]
    serial_device: Option<Rc<RefCell<dyn SerialDevice>>>,

    stopped: bool,

    /// Used to track how many ppu cycles have elapsed
    /// when the frontend gets the elapsed value, its reset to 0
    elapsed_ppu_cycles: u32,

    config: GameBoyConfig,
}

impl Bus {
    pub fn new_without_boot_rom(cartridge: Cartridge, config: GameBoyConfig) -> Self {
        let cgb_mode = cartridge.is_cartridge_color();
        let mut lock = Lock::default();

        if !cgb_mode || config.is_dmg {
            lock.write(4);
        } else {
            // TODO: change this to take the value from the cartridge addr 0x143
            lock.write(0x80);
        }

        lock.finish_boot();

        Self {
            trace_enabled:false, last_fetch:None,bus_events:Vec::new(),
            cartridge,
            ppu: Ppu::new_skip_boot_rom(cgb_mode, config),
            wram: Wram::default(),
            interrupts: Interrupts::default(),
            timer: Timer::new_skip_boot_rom(config),
            joypad: Joypad::default(),
            serial: Serial::new_skip_boot_rom(config),
            oam_dma: OamDma::default(),
            hdma: Hdma::default(),
            apu: Apu::new_skip_boot_rom(config),
            hram: [0; 127],
            boot_rom: BootRom::default(),
            speed_controller: SpeedController::default(),
            lock,
            unknown_registers: UnknownRegisters::new([0xFF, 0xFF, 0xFF, 0x70]),
            serial_device: None,
            stopped: false,

            elapsed_ppu_cycles: 0,

            config,
        }
    }

    pub fn new_with_boot_rom(
        cartridge: Cartridge,
        boot_rom_data: Vec<u8>,
        config: GameBoyConfig,
    ) -> Self {
        let mut s = Self::new_without_boot_rom(cartridge, config);
        s.timer = Timer::default();
        s.ppu = Ppu::new(config);
        s.apu = Apu::new(config);
        s.serial = Serial::new(config);
        s.lock = Lock::default();

        if config.is_dmg {
            s.lock.write(4);
            s.lock.finish_boot();
        }

        // should always pass as another check is done in `lib.rs`, but this is needed
        // if the Bus was used elsewhere
        assert_eq!(
            boot_rom_data.len(),
            config.boot_rom_len(),
            "Bootrom length does not match"
        );

        s.boot_rom.data = boot_rom_data;
        s.boot_rom.enabled = true;
        s
    }

    pub fn configure_trace(&mut self,enabled:bool){self.trace_enabled=enabled;self.bus_events.clear();}
    pub fn begin_trace_operation(&mut self){self.bus_events.clear();self.last_fetch=None;}
    pub fn take_bus_events(&mut self)->Vec<BusEvent>{std::mem::take(&mut self.bus_events)}
    pub fn cartridge_image_hash(&self) -> [u8;32] { self.cartridge.image_hash() }
    pub fn cartridge_storage_shape(&self)->(u64,u64){self.cartridge.storage_shape()}
    pub fn physical_memory(&self,space:&str,offset:usize,length:usize)->Option<Vec<u8>>{let end=offset.checked_add(length)?;match space{"wram"=>Some(self.wram.data[..if self.config.is_dmg{0x2000}else{0x8000}].get(offset..end)?.to_vec()),"hram"=>Some(self.hram.get(offset..end)?.to_vec()),"vram"|"oam"=>self.ppu.physical_memory(space,offset,length),_=>self.cartridge.physical_memory(space,offset,length)}}
    pub fn access_location(&self,address:u16,dma:bool,write:bool)->(Option<BusLocation>,bool){
        let addr=if (0xe000..=0xfdff).contains(&address){0xc000|(address&0x1fff)}else{address};
        let conflict=if dma{None}else{self.oam_dma.conflicting_bus()};
        let blocked=match addr {0..=0x7fff=>matches!(conflict,Some(BusType::External)),0x8000..=0x9fff=>matches!(conflict,Some(BusType::Video))||((!dma||!write)&&if write { self.ppu.is_vram_write_locked() } else { self.ppu.is_vram_locked() }),0xa000..=0xdfff=>self.config.is_dmg&&matches!(conflict,Some(BusType::External)),0xfe00..=0xfe9f=>(!dma||!write)&&(conflict.is_some()||if write { self.ppu.oam_write_locked() } else { self.ppu.oam_locked() }),0xff69|0xff6b=>self.lock.is_cgb_mode()&&self.ppu.is_vram_write_locked(),_=>false};
        if blocked{return (None,true);}
        let mapped=|space,offset,bank|Some(BusLocation{space,offset,bank});
        let location=match addr {
            0..=0x7fff if write=>mapped("cartridge-controller",addr as usize,None),
            0..=0x7fff=>self.cartridge.physical_rom_offset(addr).and_then(|n|mapped("cartridge-rom",n,Some(n/0x4000))),
            0x8000..=0x9fff=>{let bank=(self.ppu.read_vram_bank()&1) as usize;mapped("vram",bank*0x2000+(addr as usize&0x1fff),Some(bank))},
            0xa000..=0xbfff=>self.cartridge.physical_ram_offset(addr).and_then(|n|mapped("cartridge-ram",n,Some(n/0x2000))).or_else(||self.cartridge.selected_rtc().and_then(|n|mapped("cartridge-rtc",n as usize,None))),
            0xc000..=0xcfff=>mapped("wram",addr as usize&0xfff,Some(0)),
            0xd000..=0xdfff=>{let bank=self.wram.bank as usize;mapped("wram",bank*0x1000+(addr as usize&0xfff),Some(bank))},
            0xfe00..=0xfe9f=>mapped("oam",addr as usize-0xfe00,None),
            0xff80..=0xfffe=>mapped("hram",addr as usize-0xff80,None),
            0xff00..=0xff7f=>mapped("io",addr as usize-0xff00,None),0xffff=>mapped("io",0xff,None),_=>None};(location,false)
    }
    fn record_access(&mut self,kind:u8,address:u16,value:u8){
        if self.trace_enabled && self.bus_events.len()<32 {
            let (location,blocked)=self.access_location(address,kind==5,kind==3||kind==4);
            self.bus_events.push(BusEvent{kind,address,value,rom_offset:if kind!=5&&matches!(self.oam_dma.conflicting_bus(),Some(BusType::External)){None}else{self.cartridge.physical_rom_offset(address)},cycles:self.elapsed_ppu_cycles,banks:None,fixed_banks:None,target:None,location,target_location:None,blocked});
        }
    }
    pub fn hardware_state(&self)->(bool,u8,u8,bool){(self.config.is_dmg,self.wram.bank,self.ppu.read_vram_bank()&1,self.speed_controller.current_speed()==Speed::Double)}
    pub fn frame_count(&self)->u64{self.ppu.frame_count}
    pub fn cartridge(&self) -> &Cartridge {
        &self.cartridge
    }

    pub fn screen_buffer(&self) -> &[u8] {
        self.ppu.screen_buffer()
    }

    #[cfg(test)]
    pub(crate) fn raw_screen_buffer(&self) -> &[u8] {
        self.ppu.raw_screen_buffer()
    }

    pub fn audio_buffers(&mut self) -> AudioBuffers<'_> {
        self.apu.get_buffers()
    }

    pub fn press_joypad(&mut self, button: JoypadButton) {
        self.joypad.press_joypad(button);
    }

    pub fn release_joypad(&mut self, button: JoypadButton) {
        self.joypad.release_joypad(button);
    }

    pub fn connect_device(&mut self, device: Rc<RefCell<dyn SerialDevice>>) {
        self.serial_device = Some(device);
    }

    pub fn disconnect_device(&mut self) {
        self.serial_device = None;
    }

    pub fn elapsed_ppu_cycles(&mut self) -> u32 {
        std::mem::replace(&mut self.elapsed_ppu_cycles, 0)
    }
}

impl Bus {
    fn on_cpu_machine_cycle(&mut self) {
        let double_speed = self.speed_controller.current_speed() == Speed::Double;

        // will be 2 in normal speed and 1 in double speed
        let cpu_clocks_added = ((!double_speed) as u8) + 1;

        // will be 4 in normal speed and 2 in double speed
        let t_clocks = cpu_clocks_added * 2;

        // In order to not crash if overflowed (in case the user is not taking the value
        // after every cpu exeution)
        self.elapsed_ppu_cycles = self.elapsed_ppu_cycles.saturating_add(t_clocks as u32);

        // The mapper is independent of CPU clock speed, and a full second
        // for the mapper is 4194304/2 clocks
        self.cartridge.clock_mapper();
        if !double_speed {
            self.cartridge.clock_mapper();
        }

        // we return after updating `elapsed_ppu_cycles` because frontend
        // depend on it
        if self.stopped {
            if self.joypad.get_keys_pressed() != 0xF {
                self.stopped = false;
            }

            return;
        }

        // PPU stays at the same speed even if CPU is in double speed
        let frame_before=self.ppu.frame_count;
        self.ppu.clock(&mut self.interrupts, t_clocks);
        if self.ppu.frame_count!=frame_before{self.record_access(6,0xff44,144);}

        // APU stays at the same speed even if CPU is in double speed,
        // the APU will handle clocking to stay in the same speed regardless
        // of the CPU speed
        self.apu.clock(double_speed, self.timer.read_div());

        // HDMA stays at the same speed even if CPU is in double speed
        if self.hdma.is_transferreing(&self.ppu) {
            // this can be filled with 1 values in double mode and 2 in normal
            // mode
            let mut values = [0; 2];
            let addr = self.hdma.get_next_src_address();
            values[0] = self.read_not_ticked(addr, None);
            let index=self.bus_events.len();self.record_access(5,addr,values[0]);if let Some(event)=self.bus_events.get_mut(index){event.target=Some(self.hdma.next_dest_address());}let location=self.access_location(self.hdma.next_dest_address(),true,true).0;if let Some(event)=self.bus_events.get_mut(index){event.target_location=location;}
            let mut values_len = 1;
            if !double_speed {
                values_len = 2;
                let addr = self.hdma.get_next_src_address();
                values[1] = self.read_not_ticked(addr, None);
                let index=self.bus_events.len();self.record_access(5,addr,values[1]);if let Some(event)=self.bus_events.get_mut(index){event.target=Some(self.hdma.next_dest_address().wrapping_add(1));}let location=self.access_location(self.hdma.next_dest_address().wrapping_add(1),true,true).0;if let Some(event)=self.bus_events.get_mut(index){event.target_location=location;}
            }

            self.hdma
                .transfer_clock(&mut self.ppu, &values[..values_len]);
        }

        // timer, DMA, and serial follow the CPU in speed and operates at double speed
        // if CPU is in double speed
        if self.speed_controller.settling_cycles == 0 {
            self.timer.clock_divider(&mut self.interrupts);
        } else {
            self.speed_controller.settling_cycles -= 1;
            if self.speed_controller.settling_cycles == 0 { self.ppu.finish_speed_switch(); }
        }
        self.joypad.update_interrupts(&mut self.interrupts);

        let serial_bit = self.serial.clock_for_bit(&mut self.interrupts);

        // TODO: this design only support gameboy sending data as master clock.
        //  Add support for gameboy as slave (maybe another gameboy as master).
        if let Some(bit) = serial_bit {
            if let Some(serial_device) = self.serial_device.as_mut() {
                if let Ok(mut serial_device) = serial_device.try_borrow_mut() {
                    let received_bit = serial_device.exchange_bit_external_clock(bit);
                    self.serial.receive_bit(received_bit);
                }
            }
        }

        if self.oam_dma.in_transfer() {
            let source=self.oam_dma.get_next_address();
            let value = self.read_not_ticked(source, None);
            if self.oam_dma.transferring_byte(){let index=self.bus_events.len();self.record_access(5,source,value);if let Some(event)=self.bus_events.get_mut(index){event.target=Some(0xfe00|(source&0xff));event.target_location=Some(BusLocation{space:"oam",offset:(source&0xff) as usize,bank:None});}}
            self.oam_dma.transfer_clock(&mut self.ppu, value);
        }
    }

    pub(crate) fn read_not_ticked(&mut self, addr: u16, block_for_dma: Option<BusType>) -> u8 {
        let dma_value = if block_for_dma.is_some() {
            self.oam_dma.current_value()
        } else {
            0xFF
        };

        let page = (addr >> 8) as u8;
        let offset = addr as u8;

        match (page, block_for_dma) {
            (0x00, _) | (0x02..=0x08, _) if self.boot_rom.enabled => {
                self.boot_rom.data[addr as usize]
            } // boot rom
            (0x02..=0x08, _) if self.boot_rom.enabled && !self.config.is_dmg => {
                self.boot_rom.data[addr as usize]
            } // boot rom
            (0x00..=0x7F, Some(BusType::External)) => dma_value, // external bus DMA conflict
            (0x00..=0x3F, _) => self.cartridge.read_rom0(addr),  // rom0
            (0x40..=0x7F, _) => self.cartridge.read_romx(addr),  // romx
            (0x80..=0x9F, Some(BusType::Video)) => dma_value,    // video bus DMA conflict
            (0x80..=0x9F, _) => self.ppu.read_vram(addr),        // ppu vram
            (0xA0..=0xDF, Some(BusType::External)) if self.config.is_dmg => dma_value, // external bus DMA conflict
            (0xA0..=0xBF, _) => self.cartridge.read_ram(addr),                         // sram
            (0xC0..=0xCF, _) => self.wram.read_wram0(addr),                            // wram0
            (0xD0..=0xDF, _) => self.wram.read_wramx(addr),                            // wramx
            (0xE0..=0xFD, _) => self.read_not_ticked(0xC000 | (addr & 0x1FFF), block_for_dma), // echo
            (0xFE, None) if offset <= 0x9F => self.ppu.read_oam(addr), // ppu oam
            (0xFE, _) if offset >= 0xA0 => 0,                          // unused
            (0xFF, _) => self.read_io(offset),                         // io registers
            _ => 0xFF,
        }
    }

    fn write_not_ticked(&mut self, addr: u16, data: u8, block_for_dma: Option<BusType>) {
        let page = (addr >> 8) as u8;
        let offset = addr as u8;

        match (page, block_for_dma) {
            (0x00..=0x7F, Some(BusType::External)) => {} // ignore writes
            (0x00..=0x7F, _) => self.cartridge.write_to_bank_controller(addr, data), // cart
            (0x80..=0x9F, Some(BusType::Video)) => {}    // ignore writes
            (0x80..=0x9F, _) => self.ppu.write_vram(addr, data), // ppu vram
            (0xA0..=0xDF, Some(BusType::External)) if self.config.is_dmg => {} // ignore writes
            (0xA0..=0xBF, _) => self.cartridge.write_ram(addr, data), // sram
            (0xC0..=0xCF, _) => self.wram.write_wram0(addr, data), // wram0
            (0xD0..=0xDF, _) => self.wram.write_wramx(addr, data), // wramx
            (0xE0..=0xFD, _) => {
                self.write_not_ticked(0xC000 | (addr & 0x1FFF), data, block_for_dma)
            } // echo
            (0xFE, None) if offset <= 0x9F => {
                self.ppu.write_oam(addr, data) // ppu oam
            }
            (0xFF, _) => self.write_io(offset, data), // io registers
            _ => {}
        }
    }

    fn read_io(&mut self, offset: u8) -> u8 {
        let addr = 0xFF00 | (offset as u16);
        match offset {
            0x00 => self.joypad.read_joypad(),              // joypad
            0x01 => self.serial.read_data(),                // serial
            0x02 => self.serial.read_control(),             // serial
            0x04 => self.timer.read_div(),                  // timer
            0x05 => self.timer.read_timer_counter(),        // timer
            0x06 => self.timer.read_timer_reload(),         // timer
            0x07 => self.timer.read_control(),              // timer
            0x0F => self.interrupts.read_interrupt_flags(), // interrupts flags
            0x10..=0x3F => self.apu.read_register(addr),    // apu
            0x40 => self.ppu.read_lcd_control(),            // ppu
            0x41 => self.ppu.read_lcd_status(),             // ppu
            0x42 => self.ppu.read_scroll_y(),               // ppu
            0x43 => self.ppu.read_scroll_x(),               // ppu
            0x44 => self.ppu.read_ly(),                     // ppu
            0x45 => self.ppu.read_lyc(),                    // ppu
            0x46 => self.oam_dma.read_register(),           // oam dma
            0x47 => self.ppu.read_dmg_bg_palette(),         // ppu
            0x48 => self.ppu.read_dmg_sprite_palettes(0),   // ppu
            0x49 => self.ppu.read_dmg_sprite_palettes(1),   // ppu
            0x4A => self.ppu.read_window_y(),               // ppu
            0x4B => self.ppu.read_window_x(),               // ppu
            0x4D if self.lock.is_cgb_mode() => self.speed_controller.read_key1(), // speed
            0x4F if !self.config.is_dmg => self.ppu.read_vram_bank(), // vram bank
            0x50 => 0xFF,                                   // boot rom stop
            0x51..=0x55 if self.lock.is_cgb_mode() => self.hdma.read_register(addr), // hdma
            0x56 if self.lock.is_cgb_mode() => {
                // TODO: implement RP port
                0xFF
            }
            0x68 if !self.config.is_dmg => self.ppu.read_cgb_bg_palettes_index(), // ppu
            0x69 if !self.config.is_dmg => self.ppu.read_cgb_bg_palettes_data(),  // ppu
            0x6A if !self.config.is_dmg => self.ppu.read_cgb_sprite_palettes_index(), // ppu
            0x6B if self.lock.is_cgb_mode() => self.ppu.read_cgb_sprite_palettes_data(), // ppu
            0x6C if !self.config.is_dmg => self.ppu.read_sprite_priority_mode(),
            0x70 if self.lock.is_cgb_mode() => self.wram.get_wram_bank(), // wram bank
            0x72 if !self.config.is_dmg => self.unknown_registers[0].read(), // unknown
            0x73 if !self.config.is_dmg => self.unknown_registers[1].read(), // unknown
            0x74 if self.lock.is_cgb_mode() => self.unknown_registers[2].read(), // unknown
            0x75 if !self.config.is_dmg => self.unknown_registers[3].read(), // unknown
            0x76 if !self.config.is_dmg => self.apu.read_pcm12(),         // apu pcm 12
            0x77 if !self.config.is_dmg => self.apu.read_pcm34(),         // apu pcm 34
            0x80..=0xFE => self.hram[addr as usize & 0x7F],               // hram
            0xFF => self.interrupts.read_interrupt_enable(),              //interrupts enable
            _ => 0xFF,
        }
    }

    fn write_io(&mut self, offset: u8, data: u8) {
        let addr = 0xFF00 | (offset as u16);

        match offset {
            0x00 => self.joypad.write_joypad(data),              // joypad
            0x01 => self.serial.write_data(data),                // serial
            0x02 => self.serial.write_control(data),             // serial
            0x04 => self.timer.write_div(data),                  // timer
            0x05 => self.timer.write_timer_counter(data),        // timer
            0x06 => self.timer.write_timer_reload(data),         // timer
            0x07 => self.timer.write_control(data),              // timer
            0x0F => self.interrupts.write_interrupt_flags(data), // interrupts flags
            0x10..=0x3F => self.apu.write_register(addr, data),  // apu
            0x40 => self.ppu.write_lcd_control(data),            // ppu
            0x41 => self.ppu.write_lcd_status(data),             // ppu
            0x42 => self.ppu.write_scroll_y(data),               // ppu
            0x43 => self.ppu.write_scroll_x(data),               // ppu
            0x44 => self.ppu.write_ly(data),                     // ppu
            0x45 => self.ppu.write_lyc(data),                    // ppu
            0x46 => self.oam_dma.write_register(data),           // dma start
            0x47 => self.ppu.write_dmg_bg_palette(data),         // ppu
            0x48 => self.ppu.write_dmg_sprite_palettes(0, data), // ppu
            0x49 => self.ppu.write_dmg_sprite_palettes(1, data), // ppu
            0x4A => self.ppu.write_window_y(data),               // ppu
            0x4B => self.ppu.write_window_x(data),               // ppu
            0x4C if self.lock.is_cgb_mode() => self.lock.write(data), // DMG/CGB lock register
            0x4D if self.lock.is_cgb_mode() => self.speed_controller.write_key1(data), // speed
            0x4F if self.lock.is_cgb_mode() => self.ppu.write_vram_bank(data), // vram bank
            0x50 => {
                self.lock.finish_boot();
                self.boot_rom.enabled = false;
                self.ppu
                    .update_cgb_mode(self.cartridge.is_cartridge_color());
            } // boot rom stop
            0x51..=0x55 if self.lock.is_cgb_mode() => self.hdma.write_register(addr, data), // hdma
            0x56 => {
                //TODO: implement RP port
            }
            0x68 if self.lock.is_cgb_mode() => self.ppu.write_cgb_bg_palettes_index(data), // ppu
            0x69 if self.lock.is_cgb_mode() => self.ppu.write_cgb_bg_palettes_data(data),  // ppu
            0x6A if self.lock.is_cgb_mode() => self.ppu.write_cgb_sprite_palettes_index(data), // ppu
            0x6B if self.lock.is_cgb_mode() => self.ppu.write_cgb_sprite_palettes_data(data), // ppu
            0x6C if self.lock.is_cgb_mode() => self.ppu.write_sprite_priority_mode(data),
            0x70 if self.lock.is_cgb_mode() => self.wram.set_wram_bank(data), // wram bank
            0x72 if !self.config.is_dmg => self.unknown_registers[0].write(data), // unknown
            0x73 if !self.config.is_dmg => self.unknown_registers[1].write(data), // unknown
            0x74 if self.lock.is_cgb_mode() => self.unknown_registers[2].write(data), // unknown
            0x75 if !self.config.is_dmg => self.unknown_registers[3].write(data), // unknown
            0x80..=0xFE => self.hram[addr as usize & 0x7F] = data,            // hram
            0xFF => self.interrupts.write_interrupt_enable(data),             // interrupts enable
            _ => {}
        }
    }
}

impl CpuBusProvider for Bus {
    /// Advance time without inventing a memory access.
    fn idle_cycle(&mut self) { self.on_cpu_machine_cycle(); }
    /// each time the cpu reads, clock the components on the bus
    fn read(&mut self, addr: u16) -> u8 {
        let result = self.read_no_oam_bug(addr);

        if self.config.is_dmg && addr & 0xFF00 == 0xFE00 {
            self.ppu.oam_bug_read();
        }

        result
    }

    /// each time the cpu writes, clock the components on the bus
    fn write(&mut self, addr: u16, data: u8) {
        self.record_access(3,addr,data);
        let bank_before=if addr==0xff4f{Some((self.ppu.read_vram_bank()&1) as usize*0x2000)}else if addr==0xff70{Some(self.wram.bank as usize*0x1000)}else{None};
        let fixed_before=self.cartridge.physical_rom_offset(0).unwrap_or(0);
        let before=self.cartridge.physical_rom_offset(0x4000).unwrap_or(0);
        self.write_not_ticked(addr, data, self.oam_dma.conflicting_bus());
        if self.trace_enabled && addr<0x8000 && !self.access_location(addr,false,true).1 && self.bus_events.len()<32 {
            let after=self.cartridge.physical_rom_offset(0x4000).unwrap_or(0);
            self.bus_events.push(BusEvent{kind:4,address:addr,value:data,rom_offset:None,cycles:self.elapsed_ppu_cycles,banks:Some((before,after)),fixed_banks:Some((fixed_before,self.cartridge.physical_rom_offset(0).unwrap_or(0))),target:None,location:Some(BusLocation{space:"cartridge-controller",offset:addr as usize,bank:None}),target_location:None,blocked:false});
        }
        if self.trace_enabled&&self.lock.is_cgb_mode()&&self.bus_events.len()<32&&bank_before.is_some(){
            let target=if addr==0xff4f{0x8000}else{0xd000};let location=self.access_location(target,true,true).0;let after=location.as_ref().map(|a|a.offset).unwrap_or(0);
            self.bus_events.push(BusEvent{kind:4,address:addr,value:data,rom_offset:None,cycles:self.elapsed_ppu_cycles,banks:Some((bank_before.unwrap(),after)),fixed_banks:None,target:Some(target),location:Some(BusLocation{space:"io",offset:addr as usize-0xff00,bank:None}),target_location:location,blocked:false});
        }
        self.on_cpu_machine_cycle();

        if self.config.is_dmg && addr & 0xFF00 == 0xFE00 {
            self.ppu.oam_bug_write();
        }
    }

    fn fetch_read(&mut self,addr:u16)->u8 {
        let index=self.bus_events.len();let value=self.read(addr);
        self.last_fetch=if let Some(event)=self.bus_events.get_mut(index){event.kind=1;Some(index)}else{None};value
    }
    fn mark_opcode(&mut self){if let Some(index)=self.last_fetch{if let Some(event)=self.bus_events.get_mut(index){event.kind=0;}}}

    // gets the interrupt type and remove it
    fn take_next_interrupt(&mut self) -> Option<InterruptType> {
        let int = self.interrupts.get_highest_interrupt();
        if let Some(int) = int {
            self.interrupts.acknowledge_interrupt(int);
        }
        int
    }

    fn peek_next_interrupt(&mut self) -> Option<InterruptType> {
        self.interrupts.get_highest_interrupt()
    }

    fn is_hdma_running(&mut self) -> bool {
        self.hdma.is_transferreing(&self.ppu)
    }

    fn enter_stop_mode(&mut self) {
        if self.speed_controller.preparing_switch() {
            assert!(!self.config.is_dmg, "Cannot switch speed in DMG");
            self.ppu.begin_speed_switch();
            self.speed_controller.commit_speed_switch();
            self.timer.write_div(0);
        } else {
            self.stopped = true;
            self.ppu.enter_stop_mode();
            // TODO: is there any special stuff to do with the apu?
            //  for CGB, sounds still play?
            // self.apu.enter_stop_mode(); ?
        }
    }
    fn speed_switching(&self) -> bool { self.speed_controller.settling_cycles > 0 }

    fn stopped(&self) -> bool {
        self.stopped
    }

    fn trigger_write_oam_bug(&mut self, addr: u16) {
        if self.config.is_dmg && addr & 0xFF00 == 0xFE00 {
            self.ppu.oam_bug_write();
        }
    }

    fn trigger_read_write_oam_bug(&mut self, addr: u16) {
        if self.config.is_dmg && addr & 0xFF00 == 0xFE00 {
            self.ppu.oam_bug_read_write();
        }
    }

    fn read_no_oam_bug(&mut self, addr: u16) -> u8 {
        let result = self.read_not_ticked(addr, self.oam_dma.conflicting_bus());
        self.record_access(2,addr,result);
        self.on_cpu_machine_cycle();
        result
    }
}
