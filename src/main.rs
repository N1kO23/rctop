extern crate systemstat;

use std::io::stdout;
use std::process;
use std::thread;
use std::time::Duration;

use ctrlc;

use futures::executor::block_on;
use systemstat::{saturating_sub_bytes, Platform, System};

use crossterm::style::{Attribute, Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{
    Clear,
    ClearType::{All, CurrentLine},
    ScrollUp,
};
use crossterm::{
    cursor::{position, Hide, MoveTo, Show},
    execute,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The main function of the program
fn main() {
    // Move and hide the cursor
    execute!(stdout(), Hide).ok();
    // Block main thread until process finishes
    block_on(async_main());
}

async fn async_main() {
    let sys = System::new();
    let mut term_size = get_term_size();
    let mut i: u16 = 0;

    ctrlc::set_handler(move || {
        println!("Received Ctrl + C! Exiting...");
        execute!(stdout(), Show, Clear(All), ResetColor, MoveTo(0, 0)).ok();
        process::exit(0);
    })
    .expect("Error setting Ctrl + C handler");

    while i < term_size.1 {
        println!("");
        i += 1;
    }
    loop {
        let temp_size = get_term_size();
        if temp_size.0 != term_size.0 || temp_size.1 != term_size.1 {
            term_size = temp_size;
            execute!(stdout(), Clear(All)).ok();
        }
        execute!(
            stdout(),
            MoveTo(0, 0),
            Clear(CurrentLine),
            SetBackgroundColor(Color::DarkCyan)
        )
        .ok();
        for _i in 0..term_size.0 {
            print!(" ");
        }
        execute!(stdout(), MoveTo(1, 0)).ok();
        print!(
            "RCTOP v{} [Width: {}, Height: {}]",
            VERSION, term_size.0, term_size.1
        );
        execute!(stdout(), ResetColor, MoveTo(0, 2)).ok();
        // match sys.mounts() {
        //     Ok(mounts) => {
        //         println!("\nMounts:");
        //         for mount in mounts.iter() {
        //             println!("{} ---{}---> {} (available {} of {})",
        //                      mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
        //         }
        //     }
        //     Err(x) => println!("\nMounts: error: {}", x)
        // }
        // match sys.mount_at("/") {
        //     Ok(mount) => {
        //         println!("\nMount at /:");
        //         println!("{} ---{}---> {} (available {} of {})",
        //                  mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
        //     }
        //     Err(x) => println!("\nMount at /: error: {}", x)
        // }
        // match sys.block_device_statistics() {
        //     Ok(stats) => {
        //         for blkstats in stats.values() {
        //             println!("{}: {:?}", blkstats.name, blkstats);
        //         }
        //     }
        //     Err(x) => println!("\nBlock statistics error: {}", x.to_string())
        // }
        // match sys.networks() {
        //     Ok(netifs) => {
        //         println!("\nNetworks:");
        //         for netif in netifs.values() {
        //             println!("{} ({:?})", netif.name, netif.addrs);
        //         }
        //     }
        //     Err(x) => println!("\nNetworks: error: {}", x)
        // }
        // match sys.networks() {
        //     Ok(netifs) => {
        //         println!("\nNetwork interface statistics:");
        //         for netif in netifs.values() {
        //             println!("{} statistics: ({:?})", netif.name, sys.network_stats(&netif.name));
        //         }
        //     }
        //     Err(x) => println!("\nNetworks: error: {}", x)
        // }
        // match sys.on_ac_power() {
        //     Ok(power) => println!(", AC power: {}", power),
        //     Err(x) => println!(", AC power: error: {}", x)
        // }
        // match sys.memory() {
        //     Ok(mem) => println!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
        //     Err(x) => println!("\nMemory: error: {}", x)
        // }
        // match sys.load_average() {
        //     Ok(loadavg) => println!("\nLoad average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
        //     Err(x) => println!("\nLoad average: error: {}", x)
        // }
        // match sys.uptime() {
        //     Ok(uptime) => println!("\nUptime: {:?}", uptime),
        //     Err(x) => println!("\nUptime: error: {}", x)
        // }
        // match sys.boot_time() {
        //     Ok(boot_time) => println!("\nBoot time: {}", boot_time),
        //     Err(x) => println!("\nBoot time: error: {}", x)
        // }
        // match sys.cpu_temp() {
        //     Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        //     Err(x) => println!("\nCPU temp: {}", x)
        // }
        // match sys.socket_stats() {
        //     Ok(stats) => println!("\nSystem socket statistics: {:?}", stats),
        //     Err(x) => println!("\nSystem socket statistics: error: {}", x.to_string())
        // }

        // Total CPU usage is 0 at first in case of error
        let mut total_cpu: f32 = 0_f32;
        let mut memory = vec![0, 0];
        // Fetches the CPU usage for each core and prints it
        match get_cpu_stats(&sys) {
            Ok(cpu_usages) => {
                let cpu_count_string_length: usize = cpu_usages.len().to_string().len();
                for i in 0..cpu_usages.len() {
                    execute!(stdout(), Clear(CurrentLine)).ok();
                    print!("CPU {}:", i);
                    for _j in i.to_string().len()..cpu_count_string_length + 1 {
                        print!(" ");
                    }
                    print_bar(
                        term_size.0 - 5,
                        100_f32 - &cpu_usages[i][4],
                        Color::DarkGreen,
                    );
                    println!("");
                    execute!(stdout(), Clear(CurrentLine)).ok();
                    //println!("Load: {:.2}%", 100_f32 - &cpu_usages[i][4]);
                    // Sum up the cpu usages
                    total_cpu += &cpu_usages[i][4];
                }
                // Get total cpu usage by dividing with the core count
                total_cpu = 100_f32 - total_cpu / cpu_usages.len() as f32;
            }
            Err(x) => print!("\nCPU usage: error: {}", x.to_string()),
        }
        // Fetches the memory usage and prints it
        match get_mem_size(&sys) {
            Ok(mem_size) => {
                memory = mem_size;
                execute!(stdout(), Clear(CurrentLine)).ok();
                print!("Memory: ");
                print_bar(
                    term_size.0 - 5,
                    memory[1] as f32 / memory[0] as f32 * 100_f32,
                    Color::DarkYellow,
                );
                println!("");
                execute!(stdout(), Clear(CurrentLine)).ok();
                print!("Swap: ");
                print_bar(
                    term_size.0 - 5,
                    memory[3] as f32 / memory[2] as f32 * 100_f32,
                    Color::DarkYellow,
                );
                println!("");
            }
            Err(x) => print!("\nMemory: error: {}", x.to_string()),
        }
        match sys.battery_life() {
            Ok(battery) => print!(
                "\nBattery: {}%, {}h{}m remaining",
                battery.remaining_capacity * 100.0,
                battery.remaining_time.as_secs() / 3600,
                battery.remaining_time.as_secs() % 60
            ),
            Err(x) => print!("\nBattery: error: {}", x),
        }

        //print_graph_stats(&cpu_vec, term_size.0 / 2, term_size.1 - 3, term_size.0, term_size.1);
        execute!(
            stdout(),
            MoveTo(0, term_size.1),
            Clear(CurrentLine),
            SetBackgroundColor(Color::DarkCyan)
        )
        .ok();
        for _i in 0..term_size.0 {
            print!(" ");
        }
        execute!(stdout(), MoveTo(1, term_size.1)).ok();
        print!("CPU: {:.2}% ", total_cpu);
        print!("RAM: {} / {} ", memory[1], memory[0]);
        execute!(stdout(), ResetColor).ok();
    }
}

/// Returns the size of the terminal as a tuple of integers.
/// First value is the width and the second value is the height
fn get_term_size() -> (u16, u16) {
    let term_size = crossterm::terminal::size();
    match term_size {
        Ok(size) => {
            return size;
        }
        Err(e) => {
            println!("Error while fetching terminal size: {}", e);
            process::exit(1);
        }
    }
}

fn print_graph_stats(
    cpu_vec: &std::vec::Vec<f32>,
    max_width: u16,
    max_height: u16,
    x_offset: u16,
    y_offset: u16,
) {
    let mut index: usize = 0;
    let length = cpu_vec.len();
    for i in y_offset - max_height..y_offset {
        execute!(stdout(), MoveTo(0, i)).ok();
        execute!(stdout(), Clear(CurrentLine)).ok();
    }
    while index < max_width.into() && index < length {
        let height = max_height as f32 / 100_f32 * cpu_vec[&length - 1 - &index];
        let floored: u16 = height as u16;
        execute!(
            stdout(),
            MoveTo(x_offset - index as u16, y_offset - max_height + floored)
        )
        .ok();
        if (height - floored as f32) <= 0.33 {
            print!("_");
        } else if (height - floored as f32) <= 0.66 {
            print!("-");
        } else {
            print!("¯");
        }
        index += 1;
    }
}

/// Prints a bar that is as long as the percentage of the given terminal width
/// ### Arguments
///
/// * `max_width` - The max width of the bar
/// * `percentage` - The percentage of the max width the bar is going to be
fn print_bar(max_width: u16, percentage: f32, color: Color) {
    execute!(stdout(), SetForegroundColor(color)).ok();
    let block_count = max_width as f32 / 100_f32 * percentage;
    let mut index: u16 = 0;
    let floored = block_count as u16;
    // Print the full bars
    while index < floored {
        print!("█");
        index = index + 1;
    }
    // Determine the last bar from decimal
    if floored != 100 {
        if (block_count - floored as f32) <= 0.25 {
            print!("░");
        } else if (block_count - floored as f32) <= 0.5 {
            print!("▒");
        } else if (block_count - floored as f32) <= 0.75 {
            print!("▓");
        } else {
            print!(" ");
        }
    }
    execute!(stdout(), ResetColor).ok();
}

/// Fetches the current cpu usage of the system or throws error if the fetch fails,
/// the first index is the cpu core and the second is the exact usage
/// ### Arguments
/// * `system` - The reference to the System
/// ### Returns
/// * `vec[0][0]` - User cpu usage
/// * `vec[0][1]` - Nice cpu usage
/// * `vec[0][2]` - System cpu usage
/// * `vec[0][3]` - Interrupt cpu usage
/// * `vec[0][4]` - Idle percentage (100_f32 - vec[0][4] = total cpu usage for core 0)
fn get_cpu_stats(
    system: &System,
) -> Result<std::vec::Vec<std::vec::Vec<f32>>, Box<dyn std::error::Error>> {
    let cpu_aggregate = system.cpu_load();
    match cpu_aggregate {
        Ok(cpu_agg) => {
            let mut vec = vec![];
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu_agg.done().unwrap();
            for i in 0..cpu.len() {
                let mut vec_vec = vec![];
                vec_vec.push(cpu[i].user * 100.0);
                vec_vec.push(cpu[i].nice * 100.0);
                vec_vec.push(cpu[i].system * 100.0);
                vec_vec.push(cpu[i].interrupt * 100.0);
                vec_vec.push(cpu[i].idle * 100.0);
                vec.push(vec_vec);
            }
            Ok(vec)
        }
        Err(x) => Err(Box::new(x)),
    }
}

/// Fetches the current memory usage of the system or throws error if the fetch fails,
/// the first index is the total memory and the second is the used memory
/// ### Arguments
/// * `system` - The reference to the System
/// ### Returns
/// * `vec[0]` - Total memory
/// * `vec[1]` - Used memory
/// * `vec[2]` - Total swap
/// * `vec[3]` - Used swap
fn get_mem_size(system: &System) -> Result<std::vec::Vec<u64>, Box<dyn std::error::Error>> {
    match system.memory() {
        Ok(mem) => {
            println!(
                "\nMemory: {} used / {} total ({:?})",
                saturating_sub_bytes(mem.total, mem.free),
                mem.total,
                mem.platform_memory
            );
            let mut vec = vec![];
            vec.push(mem.total.as_u64());
            vec.push(saturating_sub_bytes(mem.total, mem.free).as_u64());
            if mem.platform_memory.meminfo.contains_key("SwapTotal") {
                match mem.platform_memory.meminfo.get("SwapTotal") {
                    Some(x) => vec.push(x.as_u64()),
                    None => (vec.push(0)),
                }
            }
            if mem.platform_memory.meminfo.contains_key("SwapFree") {
                match mem.platform_memory.meminfo.get("SwapFree") {
                    Some(x) => vec.push(x.as_u64()),
                    None => (vec.push(0)),
                }
            }
            Ok(vec)
        }
        Err(x) => Err(Box::new(x)),
    }
}
