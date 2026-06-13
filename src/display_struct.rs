use std::ffi::OsStr;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

use std::fmt;

pub struct Program {
    name: Box<OsStr>,
    cpu: f32,
    ram: u64,
    pid: i32,
}

impl Program {
    pub fn new(system: &System, pid: i32) -> Option<Program> {
        if let Some(process) = system.process(Pid::from(pid as usize)) {
            return Some(Program {
                name: process.name().into(),
                cpu: process.cpu_usage(),
                ram: process.memory(),
                pid: pid,
            });
        }
        None
    }

    pub fn refresh(&mut self, system: &mut System) {
        system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing().with_cpu(),
        );

        if let Some(process) = system.process(Pid::from(self.pid as usize)) {
            self.name = process.name().into();
            self.cpu = process.cpu_usage();
            self.ram = process.memory();
        } else {
            panic!("Not a program {}", self.pid as usize);
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {:?} | pid : {} | cpu: {}% | ram : {}MB, ",
            self.name,
            self.pid,
            self.cpu,
            self.ram / (1024 * 1024)
        )
    }
}
