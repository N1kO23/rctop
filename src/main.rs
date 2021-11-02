extern crate systemstat;

use serde_json::json;

use std::thread;
use std::time::Duration;
use std::io::stdout;
use std::error::Error;

use systemstat::{System, Platform, saturating_sub_bytes};
use futures::executor::block_on;

use crossterm::{
    execute, Result,
    cursor::Hide
};

/// The main function of the program
fn main() -> Result<()> {
    execute!(stdout(), Hide);
    loop {
        let term_size = crossterm::terminal::size().unwrap();
        println!("Width: {}, Height: {}", term_size.0, term_size.1);
        block_on(async_main());
    }
}

async fn async_main() {
    let sys = System::new();

    match sys.mounts() {
        Ok(mounts) => {
            println!("\nMounts:");
            for mount in mounts.iter() {
                println!("{} ---{}---> {} (available {} of {})",
                         mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
            }
        }
        Err(x) => println!("\nMounts: error: {}", x)
    }

    match sys.mount_at("/") {
        Ok(mount) => {
            println!("\nMount at /:");
            println!("{} ---{}---> {} (available {} of {})",
                     mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
        }
        Err(x) => println!("\nMount at /: error: {}", x)
    }

    match sys.block_device_statistics() {
        Ok(stats) => {
            for blkstats in stats.values() {
                println!("{}: {:?}", blkstats.name, blkstats);
            }
        }
        Err(x) => println!("\nBlock statistics error: {}", x.to_string())
    }

    match sys.networks() {
        Ok(netifs) => {
            println!("\nNetworks:");
            for netif in netifs.values() {
                println!("{} ({:?})", netif.name, netif.addrs);
            }
        }
        Err(x) => println!("\nNetworks: error: {}", x)
    }

    match sys.networks() {
        Ok(netifs) => {
            println!("\nNetwork interface statistics:");
            for netif in netifs.values() {
                println!("{} statistics: ({:?})", netif.name, sys.network_stats(&netif.name));
            }
        }
        Err(x) => println!("\nNetworks: error: {}", x)
    }

    match sys.battery_life() {
        Ok(battery) =>
            print!("\nBattery: {}%, {}h{}m remaining",
                   battery.remaining_capacity*100.0,
                   battery.remaining_time.as_secs() / 3600,
                   battery.remaining_time.as_secs() % 60),
        Err(x) => print!("\nBattery: error: {}", x)
    }
    
    match sys.on_ac_power() {
        Ok(power) => println!(", AC power: {}", power),
        Err(x) => println!(", AC power: error: {}", x)
    }

    match sys.memory() {
        Ok(mem) => println!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
        Err(x) => println!("\nMemory: error: {}", x)
    }

    match sys.load_average() {
        Ok(loadavg) => println!("\nLoad average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
        Err(x) => println!("\nLoad average: error: {}", x)
    }

    match sys.uptime() {
        Ok(uptime) => println!("\nUptime: {:?}", uptime),
        Err(x) => println!("\nUptime: error: {}", x)
    }

    match sys.boot_time() {
        Ok(boot_time) => println!("\nBoot time: {}", boot_time),
        Err(x) => println!("\nBoot time: error: {}", x)
    }

    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x)
    }

    match sys.socket_stats() {
        Ok(stats) => println!("\nSystem socket statistics: {:?}", stats),
        Err(x) => println!("\nSystem socket statistics: error: {}", x.to_string())
    }
    let cpu_usages = get_cpu_stats(&sys);
    let json_payload = json!({
        "cpu": {
            "user": cpu_usages[0],
            "nice": cpu_usages[1],
            "system": cpu_usages[2],
            "interrupt": cpu_usages[3],
            "idle": cpu_usages[4]
        }
    });

    println!("{}", json_payload);
}

fn get_cpu_stats(system: &System) -> std::vec::Vec<f32> {
    let mut vec = vec![];
    let cpu_aggregate = system.cpu_load_aggregate().unwrap();
    println!("\nMeasuring CPU load...");
    thread::sleep(Duration::from_secs(1));
    let cpu = cpu_aggregate.done().unwrap();
    vec.push(cpu.user * 100.0);
    vec.push(cpu.nice * 100.0);
    vec.push(cpu.system * 100.0);
    vec.push(cpu.interrupt * 100.0);
    vec.push(cpu.idle * 100.0);
    return vec;
}
