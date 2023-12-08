mod devices;
mod win_kernel_driver_c;
mod win_ring0_c;

use crate::devices::devices::{NvidiaVideoCard, AmdProcessor};
use crate::win_ring0_c::runner::SimpleWinDriverRunner;

use devices::sensor_info::SensorInfo;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;

use std::sync::atomic::{AtomicBool, Ordering};
use std::{io, thread};
use std::sync::{Arc, Mutex};
use std::time::Duration;


fn print_to_console(sensors: &[SensorInfo]) {
    print!("\n\t\t**************************** \n\n");
    print!("\t\t{dv_name}", dv_name = sensors[0].device_name);
    print!("\n\t\t------ \n");
    for i in 0..sensors.len() {
        print!("\t\t{s_name}: {s_value}", s_name = sensors[i].name, s_value = sensors[i].value);
        print!("\n");
    }
}

fn main() -> Result<(), NvmlError> {
    let nvml = Nvml::init()?;

    let win_driver: &mut SimpleWinDriverRunner = &mut SimpleWinDriverRunner::new();
    win_driver.initialize_driver();

    let mut gpu: NvidiaVideoCard = NvidiaVideoCard::new(&nvml);
    let mut cpu: AmdProcessor = AmdProcessor::new(win_driver);

    let sigterm = Arc::new(AtomicBool::new(false));
    let sigterm_clone = sigterm.clone();

    let _ = ctrlc::set_handler(move || {
        sigterm.store(true, Ordering::Relaxed);
    });

    loop {
        if sigterm_clone.load(Ordering::Relaxed) {
            break;
        }
        thread::sleep(Duration::from_secs(1));

        gpu.refresh();
        cpu.refresh();

        let gpu_sensors = gpu.get_sensor_info();
        let cpu_sensors = cpu.get_sensor_info();

        print!("{}[2J", 27 as char);
        print_to_console(gpu_sensors);
        print_to_console(cpu_sensors);
    }

    win_driver.clean_up();
    println!("\n\n\nProgram is about to exit. Press Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());
    Ok(())
}