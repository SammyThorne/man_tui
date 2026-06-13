mod display_struct;
use sysinfo::System;
fn main() {
    let mut sys = System::new_all();

    let a = display_struct::Program::new(&sys, 40410);

    if let Some(mut program) = a {
        loop {
            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
            program.refresh(&mut sys);
            println!("{}", program);
        }
    }

    println!("All done!");
}
