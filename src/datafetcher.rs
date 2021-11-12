extern crate systemstat;

use systemstat::{saturating_sub_bytes, Platform, System, LoadAverage, PlatformCpuLoad, PlatformMemory, Filesystem, NetworkAddrs };

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::error::Error;
use std::vec::Vec;


/// Contains the information about the system
/// ### Fields
/// * `cpu` - The system's CPU data
/// * `ram` - The system's memory data
/// * `disk` - The system's disk data
/// * `network` - The system's network data
pub struct SystemData {
    cpu: CPUData,
    ram: RAMData,
    disk: DiskData,
    network: NetworkData,
}

/// Contains the information about the system's CPU
/// ### Fields
/// * `cpu_count` - The system's CPU core count
/// * `cpu_load` - The system's CPU load per core
/// * `cpu_load_average` - The system's CPU load average
/// * `cpu_temp` - The system's CPU temperature per core
/// * `platform` - The system's CPU platform specific data
struct CPUData {
    cpu_count: usize,
    cpu_load: Vec<CpuLoad>,
    cpu_load_average: Vec<LoadAverage>,
    cpu_temp: Vec<f32>,
    platform: PlatformCpuLoad,
}

/// Contains the information about the CPU core's load
/// ### Fields
/// * `user` - The CPU core's user load
/// * `nice` - The CPU core's nice load
/// * `system` - The CPU core's system load
/// * `interrupt` - The CPU core's interrupt load
/// * `idle` - The CPU core's idle load
/// * `total` - The CPU core's total load
struct CpuLoad {
    user: f32,
    nice: f32,
    system: f32,
    interrupt: f32,
    idle: f32,
    total: f32,
}

/// Contains the information about the system's RAM
/// ### Fields
/// * `ram_total` - The system's total RAM
/// * `ram_used` - The system's used RAM
/// * `ram_free` - The system's free RAM
/// * `ram_percentage` - The system's used RAM percentage
/// * `platform` - The system's RAM platform specific data
struct RAMData {
    ram_total: u64,
    ram_used: u64,
    ram_free: u64,
    ram_percentage: f32,
    platform: PlatformMemory,
}

/// Contains the information about the system's disk
/// ### Fields
/// * `disk_count` - The system's disk count
/// * `disk_total` - The system's disk space per disc
/// * `disk_used` - The system's used disk space per disc
/// * `disk_free` - The system's free disk space per disc
/// * `disk_percentage` - The system's used disk space percentage per disc
/// * `platform` - The system's disk platform specific data
struct DiskData {
    disk_count: u64,
    disk_total: Vec<u64>,
    disk_used: Vec<u64>,
    disk_free: Vec<u64>,
    disk_percentage: Vec<f32>,
    platform: Filesystem,
}

/// Contains the information about the system's network
/// ### Fields
/// * `interface_count` - The system's network interface count
/// * `interface_names` - The system's network interface names
/// * `interface_addresses` - The system's network interface addresses
/// * `interface_rx` - The system's network interface received bytes per interface
/// * `interface_tx` - The system's network interface transmitted bytes per interface
struct NetworkData {
    interface_count: usize,
    interface_names: Vec<String>,
    adresses: Vec<Vec<NetworkAddrs>>,
    interface_rx: Vec<u64>,
    interface_tx: Vec<u64>,
}


/// Starts the system data fething framework
/// ### Parameters
/// * `thr_data` - The shared data that the thread will use and update
/// * `interval` - The interval in milliseconds between each data fetching
pub fn start_fetch(thr_data: Arc<Mutex<SystemData>>, interval: Duration) {
    let mut system = System::new();
    thread::spawn(move ||  {
        loop {
            let mut data = thr_data.lock().unwrap();
            drop(data);
            thread::sleep(Duration::from_millis(100));
        }
    });
}

/// Fetches the current cpu usage of the system or throws error if the fetch fails,
/// the first index is the cpu core and the second is the exact usage
/// ### Parameters
/// * `system` - The reference to the System
/// ### Returns
/// * `vec[0][0]` - User cpu usage
/// * `vec[0][1]` - Nice cpu usage
/// * `vec[0][2]` - System cpu usage
/// * `vec[0][3]` - Interrupt cpu usage
/// * `vec[0][4]` - Idle percentage (100_f32 - vec[0][4] = total cpu usage for core 0)
fn get_cpu_stats(
    system: &System,
) -> Result<std::vec::Vec<std::vec::Vec<f32>>, Box<dyn Error>> {
    let cpu_aggregate = system.cpu_load();
    let cpu_agg = cpu_aggregate?;
    let mut vec = vec![];
    thread::sleep(Duration::from_secs(1));
    let cpu = cpu_agg.done()?;
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

/// Fetches the current memory usage of the system or throws error if the fetch fails,
/// the first index is the total memory and the second is the used memory
/// ### Parameters
/// * `system` - The reference to the System
/// ### Returns
/// * `vec[0]` - Total memory
/// * `vec[1]` - Used memory
/// * `vec[2]` - Total swap
/// * `vec[3]` - Used swap
fn get_mem_size(system: &System) -> Result<std::vec::Vec<u64>, Box<dyn std::error::Error>> {
    match system.memory() {
        Ok(mem) => {
            // println!(
            //     "\nMemory: {} used / {} total ({:?})",
            //     saturating_sub_bytes(mem.total, mem.free),
            //     mem.total,
            //     mem.platform_memory
            // );
            let mut vec = vec![];
            vec.push(mem.total.as_u64());
            vec.push(saturating_sub_bytes(mem.total, mem.free).as_u64());
            // if mem.platform_memory.meminfo.contains_key("SwapTotal") {
            //     match mem.platform_memory.meminfo.get("SwapTotal") {
            //         Some(x) => vec.push(x.as_u64()),
            //         None => (vec.push(0)),
            //     }
            // }
            // if mem.platform_memory.meminfo.contains_key("SwapFree") {
            //     match mem.platform_memory.meminfo.get("SwapFree") {
            //         Some(x) => vec.push(x.as_u64()),
            //         None => (vec.push(0)),
            //     }
            // }
            Ok(vec)
        }
        Err(x) => Err(Box::new(x)),
    }
}
        
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