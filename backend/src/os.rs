use serde::Serialize;
use sysinfo::{CpuExt, DiskExt, ProcessExt, System, SystemExt};

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct Memory {
    total: u64,
    used: u64,
    available: u64,
    free: u64,
}

impl Memory {
    pub fn new(sys: &System) -> Self {
        let mut this = Self::default();
        this.read(sys);
        this
    }

    pub fn read(&mut self, sys: &System) {
        self.total = sys.total_memory();
        self.used = sys.used_memory();
        self.available = sys.available_memory();
        self.free = sys.free_memory();
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct Swap {
    total: u64,
    used: u64,
    free: u64,
}

impl Swap {
    pub fn new(sys: &System) -> Self {
        let mut this = Self::default();
        this.read(sys);
        this
    }

    pub fn read(&mut self, sys: &System) {
        self.total = sys.total_swap();
        self.used = sys.used_swap();
        self.free = sys.free_swap();
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Disk {
    name: String,
    total: u64,
    available: u64,
}

impl Disk {
    pub fn new(disk: &sysinfo::Disk) -> Self {
        Self {
            name: disk.name().to_str().unwrap().to_owned(),
            total: disk.total_space(),
            available: disk.available_space(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Disks(Vec<Disk>);

impl Disks {
    pub fn new(sys: &System) -> Self {
        let mut this = Self::default();
        this.read(sys);
        this
    }

    pub fn read(&mut self, sys: &System) {
        self.0.clear();
        for disk in sys.disks() {
            self.0.push(Disk::new(disk))
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Kernel {
    name: String,
    uptime: u64,
    boot_time: u64,
}

impl Kernel {
    pub fn new(sys: &System) -> Self {
        let mut this = Self::default();
        this.read(sys);
        this
    }

    pub fn read(&mut self, sys: &System) {
        self.name = sys.name().unwrap_or_else(|| String::from("No Name Found"));
        self.uptime = sys.uptime();
        self.boot_time = sys.boot_time();
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct CPU {
    name: String,
    frequency: u64,
}

impl CPU {
    pub fn new(cpu: &sysinfo::Cpu) -> Self {
        Self {
            name: cpu.name().to_owned(),
            frequency: cpu.frequency(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct CPUS(Vec<CPU>);

impl CPUS {
    pub fn new(sys: &System) -> Self {
        let mut this = Self::default();
        this.read(sys);
        this
    }

    pub fn read(&mut self, sys: &System) {
        self.0.clear();
        for cpu in sys.cpus() {
            self.0.push(CPU::new(cpu))
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Components {
    mem: Memory,
    swap: Swap,
    disks: Disks,
    kernel: Kernel,
    cpus: CPUS,
}

impl Components {
    pub fn new(sys: &sysinfo::System) -> Self {
        Self {
            mem: Memory::new(&sys),
            swap: Swap::new(&sys),
            disks: Disks::new(&sys),
            kernel: Kernel::new(&sys),
            cpus: CPUS::new(&sys),
        }
    }

    pub fn read(&mut self, sys: &sysinfo::System) {
        self.mem.read(sys);
        self.swap.read(sys);
        self.disks.read(sys);
        self.kernel.read(sys);
        self.cpus.read(sys);
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ProcessInformation {
    pub name: String,
    pub pid: usize,
    pub memory: u64,
    pub run_time: u64,
}

impl ProcessInformation {
    pub fn new(proc: &sysinfo::Process) -> Self {
        Self {
            name: proc.name().to_owned(),
            pid: proc.pid().into(),
            memory: proc.memory(),
            run_time: proc.run_time(),
        }
    }
}

#[derive(Debug)]
pub struct OperatingSystem {
    sys: sysinfo::System,
    comp: Components,
}

impl OperatingSystem {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let comp = Components::new(&sys);

        Self { sys, comp }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        self.comp.read(&self.sys);
    }

    pub fn serialize_system_information(&self) -> String {
        serde_json::to_string(&self.comp).unwrap()
    }

    pub fn process_information(&self, process_name: &String) -> Option<ProcessInformation> {
        let procs = self.sys.processes_by_exact_name(process_name);
        let Some(proc) = procs.last() else {
            return None;
        };

        return Some(ProcessInformation::new(proc));
    }
}
