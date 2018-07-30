use super::cpu_gb;

#[derive(Debug)]
pub struct GB {
    cpu: cpu_gb::CpuGB
}

impl GB {
    pub fn new() -> GB {
        GB {
            cpu: cpu_gb::CpuGB::new()
        }
    }

    pub fn power_on(&mut self) {
        self.cpu.power_up();
    }
}
