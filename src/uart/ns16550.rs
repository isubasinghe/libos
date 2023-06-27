use super::ty::UART;
const RBR: u32 = 0x00;
const THR: u32 = 0x00;
const DLL: u32 = 0x00;
const DLH: u32 = 0x04;
const IER: u32 = 0x04;
const IIR: u32 = 0x08;
const FCR: u32 = 0x08;
const LCR: u32 = 0x0C;
const MCR: u32 = 0x10;
const LSR: u32 = 0x14;
const MSR: u32 = 0x18;
const LPDLL: u32 = 0x20;
const LPDLH: u32 = 0x24;
const FAR: u32 = 0x70;
const TFR: u32 = 0x74;
const RFW: u32 = 0x78;
const USR: u32 = 0x7C;
const TFL: u32 = 0x80;
const SRR : u32 = 0x88;
const SRTS : u32 = 0x8C;
const SBCR: u32 = 0x90;
const SDMAM: u32 = 0x94;
const SFE: u32 = 0x98;
const SRT : u32 = 0x9C;
const STET : u32 = 0xA0;
const HTX : u32 = 0xA4;
const DMASA : u32 = 0xA8;
const CPR : u32 = 0xF4;
const UCV : u32 = 0xF8;
const CTR: u32 = 0xFC;

pub struct NS16550 {
    base_addr: *mut u8
}

impl NS16550 {
    pub fn new(base_addr: usize) -> NS16550 {
        NS16550 { base_addr: base_addr as *mut u8 }
    }
}

impl UART for NS16550 {
    fn init(&mut self) {
       // NO NEED FOR U-BOOT 
    }

    fn get(&mut self) -> char {
        
        return 'a';
    }

    fn put(&mut self, ch: char) {

    }
}
