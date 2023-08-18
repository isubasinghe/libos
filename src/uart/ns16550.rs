use super::ty::UART;
const RBR: isize = 0x00;
const THR: isize = 0x00;
const DLL: isize = 0x00;
const DLH: isize = 0x04;
const IER: isize = 0x04;
const IIR: isize = 0x08;
const FCR: isize = 0x08;
const LCR: isize = 0x0C;
const MCR: isize = 0x10;
const LSR: isize = 0x14;
const MSR: isize = 0x18;
const LPDLL: isize = 0x20;
const LPDLH: isize = 0x24;
const FAR: isize = 0x70;
const TFR: isize = 0x74;
const RFW: isize = 0x78;
const USR: isize = 0x7C;
const TFL: isize = 0x80;
const SRR : isize = 0x88;
const SRTS : isize = 0x8C;
const SBCR: isize = 0x90;
const SDMAM: isize = 0x94;
const SFE: isize = 0x98;
const SRT : isize = 0x9C;
const STET : isize = 0xA0;
const HTX : isize = 0xA4;
const DMASA : isize = 0xA8;
const CPR : isize = 0xF4;
const UCV : isize = 0xF8;
const CTR: isize = 0xFC;



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
        unsafe {


            loop {
                // line status register
                let lsr = self.base_addr.offset(LSR).read_volatile();
                // obtain transit holder register empty bit
                let thre = (lsr >> 5) & 1;
                // bit enabled  meaning the transmit holder register is empty
                if thre == 1 {
                    break;
                }
            } 
            self.base_addr.offset(THR).write_volatile(ch as u8); 
        }

    }
}
