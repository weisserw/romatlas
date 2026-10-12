use super::{Mapper, MappingResult, ONE_SECOND_MAPPER_CLOCKS};
use save_state::Savable;

/// ROMAtlas RTC: deterministic oscillator time; no host clock or implicit elapsed-time import.
#[derive(Default, Savable)]
struct RtcRegister {
    sub_second:u32,
    live:[u8;5],
    latched:[u8;5],
    latched_valid:bool,
    last_latch:u8,
}
impl RtcRegister {
    fn read_register(&self,index:u8)->u8 {
        if self.latched_valid {self.latched[index as usize]}else{self.live[index as usize]}
    }
    fn write_register(&mut self,index:u8,data:u8) {
        self.live[index as usize]=data&match index {0|1=>0x3f,2=>0x1f,3=>0xff,4=>0xc1,_=>0};
        if index==0 {self.sub_second=0;}
    }
    fn set_latch(&mut self,value:u8) {
        if self.last_latch==0&&value==1 {self.latched=self.live;self.latched_valid=true;}
        self.last_latch=value;
    }
    fn clock_second_part(&mut self) {
        if self.live[4]&0x40!=0{return;}
        self.sub_second+=1;
        if self.sub_second!=ONE_SECOND_MAPPER_CLOCKS{return;}
        self.sub_second=0;
        if self.live[0]!=59 {self.live[0]=(self.live[0]+1)&0x3f;return;}
        self.live[0]=0;
        if self.live[1]!=59 {self.live[1]=(self.live[1]+1)&0x3f;return;}
        self.live[1]=0;
        if self.live[2]!=23 {self.live[2]=(self.live[2]+1)&0x1f;return;}
        self.live[2]=0;
        let days=self.live[3] as u16+((self.live[4] as u16&1)<<8)+1;
        self.live[3]=days as u8;
        // Carry is sticky until the program clears day-high bit 7.
        self.live[4]=(self.live[4]&0xc0)|((days>>8) as u8&1)|if days>511{0x80}else{0};
    }
    fn save_battery_size(&self)->usize {self.save_size().unwrap() as usize}
    fn save_battery(&self)->Vec<u8> {save_state::save_object(self).unwrap()}
    fn load_battery(&mut self,data:&[u8]) {save_state::load_object(self,data).unwrap();}
}

#[derive(Default, Savable)]
pub struct Mbc3 {
    rom_banks: u16,
    is_2k_ram: bool,
    ram_banks: u8,
    rtc_present: bool,

    /// the bank number to use in the memory [0x4000..=0x7FFF]
    rom_bank_4000: u16,
    ram_bank: u8,

    current_rtc_register: u8,

    rtc_register: RtcRegister,

    ram_block_enable: bool,
    is_reading_ram: bool,
}

impl Mbc3 {
    pub fn new(timer: bool) -> Self {
        Self {
            rtc_present: timer,
            rom_bank_4000: 1,
            ram_block_enable: false,
            is_reading_ram:true,

            ..Self::default()
        }
    }
}

impl Mbc3 {
    fn map_ram(&self, addr: u16) -> MappingResult {
        if self.ram_banks == 0 {
            MappingResult::NotMapped
        } else if self.is_2k_ram {
            MappingResult::Addr(addr as usize & 0x7FF)
        } else {
            let addr = addr & 0x1FFF;
            let bank = self.ram_bank % self.ram_banks;

            MappingResult::Addr(bank as usize * 0x2000 + addr as usize)
        }
    }

    fn rtc_read(&mut self) -> u8 {
        self.rtc_register.read_register(self.current_rtc_register)
    }

    fn rtc_write(&mut self, data: u8) {
        self.rtc_register
            .write_register(self.current_rtc_register, data);
    }
}

impl Mapper for Mbc3 {
    fn init(&mut self, rom_banks: u16, ram_size: usize) {
        assert!(rom_banks <= 128);
        self.rom_banks = rom_banks;
        self.ram_banks = (ram_size / 0x2000) as u8;
        self.is_2k_ram = ram_size == 0x800;
    }

    fn map_read_rom0(&self, addr: u16) -> usize {
        addr as usize
    }

    fn map_read_romx(&self, addr: u16) -> usize {
        let addr = addr & 0x3FFF;

        let bank = self.rom_bank_4000 % self.rom_banks;

        bank as usize * 0x4000 + addr as usize
    }

    fn physical_ram_offset(&self,addr:u16)->Option<usize>{if self.ram_block_enable&&self.is_reading_ram {if let MappingResult::Addr(offset)=self.map_ram(addr){return Some(offset);}}None}
    fn rtc_values(&self)->Option<[u8;5]>{if self.rtc_present{Some(std::array::from_fn(|i|self.rtc_register.read_register(i as u8)))}else{None}}
    fn selected_rtc(&self)->Option<u8>{if self.ram_block_enable&&!self.is_reading_ram&&self.rtc_present&&self.current_rtc_register<5{Some(self.current_rtc_register)}else{None}}
    fn map_ram_read(&mut self, addr: u16) -> MappingResult {
        if self.ram_block_enable {
            if self.is_reading_ram {
                self.map_ram(addr)
            } else if self.rtc_present && self.current_rtc_register<5 {
                MappingResult::Value(self.rtc_read())
            } else {
                MappingResult::NotMapped
            }
        } else {
            MappingResult::NotMapped
        }
    }

    fn map_ram_write(&mut self, addr: u16, data: u8) -> MappingResult {
        if self.ram_block_enable {
            if self.is_reading_ram {
                self.map_ram(addr)
            } else {
                if self.rtc_present && self.current_rtc_register<5 {
                    self.rtc_write(data);
                }

                MappingResult::NotMapped
            }
        } else {
            MappingResult::NotMapped
        }
    }

    fn write_bank_controller_register(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_block_enable = data & 0xF == 0xA,
            0x2000..=0x3FFF => {
                self.rom_bank_4000 = (data & 0x7f) as u16;
                if self.rom_bank_4000 == 0 {
                    self.rom_bank_4000 = 1;
                }
            }
            0x4000..=0x5FFF => {
                if !(data<=3||(8..=12).contains(&data)) {self.is_reading_ram=false;self.current_rtc_register=5;return;}
                self.is_reading_ram = data <= 3;

                if self.is_reading_ram {
                    self.ram_bank = data & 3;
                } else {
                    self.current_rtc_register = data - 0x8;
                }
            }
            0x6000..=0x7FFF => {
                if self.rtc_present {
                    self.rtc_register.set_latch(data);
                }
            }
            _ => {}
        }
    }

    fn save_battery_size(&self) -> usize {
        if self.rtc_present {
            self.rtc_register.save_battery_size()
        } else {
            0
        }
    }

    fn save_battery(&self) -> Vec<u8> {
        if self.rtc_present {
            self.rtc_register.save_battery()
        } else {
            Vec::new()
        }
    }

    fn load_battery(&mut self, data: &[u8]) {
        if self.rtc_present {
            self.rtc_register.load_battery(data)
        }
    }

    fn clock(&mut self) {
        if self.rtc_present {self.rtc_register.clock_second_part();}
    }

    save_state_fns!();
}
