use sysinfo::{System};

fn main() {
    let mut sys = System::new_all();
    
    for (pid,process) in sys.processes() {

        println!("{} {:?} {}KB, {}%", pid, process.name(),process.memory()/1024, process.cpu_usage());

    }

    println!("All done!");

}
