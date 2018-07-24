#[derive(Debug, Default)]
pub struct CpuGB {    //16-bit Special Purpose
    //Registers
    //Acumulator
    reg_a: u8,
    //Flag
    reg_f: u8,

    //General Purpose
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,

    //Stack Pointer
    reg_sp: u16,
    //Program Counter
    reg_pc: u16,
}

impl CpuGB {
    pub fn new() -> CpuGB {
        CpuGB::default()
    }

    pub fn power_up(&mut self) {
        self.reg_sp = 0xfffe;
        self.reg_pc = 0x0100;
    }
}


