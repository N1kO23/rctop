extern crate systemstat;

use std::thread;
use std::time::Duration;
use std::io::{ stdout };

use systemstat::{System, Platform};
use futures::executor::block_on;

use crossterm::terminal::{ Clear, ClearType::{ CurrentLine } };

use crossterm::{
    execute,
    cursor::{ Hide, MoveTo }
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The main function of the program
fn main() {
    
    // Block main thread until process finishes
    block_on(async_main());
}

async fn async_main() {
    let sys = System::new();
    let mut term_size = crossterm::terminal::size().unwrap();
    let mut i: u16 = 0;
    let mut cpu_vec: Vec<f32> = vec![];
    while i < term_size.1 {
        println!("");
        i += 1;
    }
    loop {
        term_size = crossterm::terminal::size().unwrap();
        // Move and hide the cursor
        execute!(stdout(), MoveTo(0, 0)).ok();
        execute!(stdout(), Hide).ok();
        execute!(stdout(), Clear(CurrentLine)).ok();
        println!("RCTOP v{} [Width: {}, Height: {}]", VERSION, term_size.0, term_size.1);
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
    
        // match sys.battery_life() {
        //     Ok(battery) =>
        //         print!("\nBattery: {}%, {}h{}m remaining",
        //                battery.remaining_capacity*100.0,
        //                battery.remaining_time.as_secs() / 3600,
        //                battery.remaining_time.as_secs() % 60),
        //     Err(x) => print!("\nBattery: error: {}", x)
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
    
        let cpu_usages = get_cpu_stats(&sys);
        // cpu_vec.push(cpu_usages[4]);
        // if cpu_vec.len() > term_size.0.into() {
        //     cpu_vec.remove(0);
        // }
        for i in 0..cpu_usages.len() {
            execute!(stdout(), Clear(CurrentLine)).ok();
            print!("CPU {}:", i);
            for j in i.to_string().len()..4 {
                print!(" ");
            }
            print_bar(term_size.0 - 5, 100_f32 - &cpu_usages[i][4]);
            println!("");
            execute!(stdout(), Clear(CurrentLine)).ok();
            //println!("Load: {:.2}%", 100_f32 - &cpu_usages[i][4]);
        }
        //print_graph_stats(&cpu_vec, term_size.0 / 2, term_size.1 - 3, term_size.0, term_size.1);
    }
}

fn print_graph_stats(cpu_vec: &std::vec::Vec<f32>, max_width: u16, max_height: u16, x_offset: u16, y_offset: u16) {
    let mut index: usize = 0;
    let length = cpu_vec.len();
    for i in y_offset-max_height..y_offset {
        execute!(stdout(), MoveTo(0, i)).ok();
        execute!(stdout(), Clear(CurrentLine)).ok();
    }
    while index < max_width.into() && index < length {
        let height = max_height as f32 / 100_f32 * cpu_vec[&length - 1 - &index];
        let floored: u16 = height as u16;
        execute!(stdout(), MoveTo(x_offset - index as u16, y_offset - max_height + floored)).ok();
        if (height - floored as f32) <= 0.33 {
            print!("_");
        }
        else if (height - floored as f32) <= 0.66 {
            print!("-");
        }
        else {
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
fn print_bar(max_width: u16, percentage: f32) {
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
        }
        else if (block_count - floored as f32) <= 0.5 {
            print!("▒");
        }
        else if (block_count - floored as f32) <= 0.75 {
            print!("▓");
        }
        else {
            print!("█");
        }
    }
}

/// Fetches the current cpu usage of the system, the first index is the cpu core and the second is the exact usage
/// ### Arguments
/// * `system` - The reference to the System
/// ### Returns
/// * `vec[0][0]` - User cpu usage
/// * `vec[0][1]` - Nice cpu usage
/// * `vec[0][2]` - System cpu usage
/// * `vec[0][3]` - Interrupt cpu usage
/// * `vec[0][4]` - Idle percentage (100_f32 - vec[4] = total cpu usage)
fn get_cpu_stats(system: &System) -> std::vec::Vec<std::vec::Vec<f32>> {
    // TODO: Handle error situation
    let mut vec = vec![];
    let cpu_aggregate = system.cpu_load().unwrap();
    thread::sleep(Duration::from_secs(1));
    let cpu = cpu_aggregate.done().unwrap();
    for i in 0..cpu.len() {
        let mut vec_vec = vec![];
        vec_vec.push(cpu[i].user * 100.0);
        vec_vec.push(cpu[i].nice * 100.0);
        vec_vec.push(cpu[i].system * 100.0);
        vec_vec.push(cpu[i].interrupt * 100.0);
        vec_vec.push(cpu[i].idle * 100.0);
        vec.push(vec_vec);
    }
    return vec;
}
