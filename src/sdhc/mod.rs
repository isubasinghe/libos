use core::ptr;

#[repr(u8)]
pub enum Command {
    GoIdleState = 0x00,
    SendOpCond = 0x01,
    SendIfCond = 0x08,
    SendCSD = 0x09,
    SendCID = 0x0a, 
    StopTransmission = 0x0c, 
    SendStatus = 0x0d, 
    SetBlocklen = 0x10, 
    ReadSingleBlock = 0x11, 
    ReadMultipleBlock = 0x12, 
    WriteSingleBlock = 0x18, 
    WriteMultipleBlock = 0x19,
}

pub type R1 = u8;

pub enum Response {
    R1(R1), 
    R3(R1, u32),
}


pub struct SDHCDriver<const DI:usize, const DO: usize> {
    
}

fn sd_raw_send_bytes() {}

fn sd_raw_send_command(command: u8, arg: u32) {
    
    
}


impl<const DI: usize, const DO: usize> SDHCDriver<DI, DO> {
    pub fn init() -> SDHCDriver<DI, DO>{

        unsafe {
        }
        SDHCDriver{}
    }
}



