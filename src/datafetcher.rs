extern crate systemstat;

use systemstat::{saturating_sub_bytes, Platform, System, LoadAverage, CPULoad, PlatformMemory, Filesystem, NetworkAddrs };

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
    pub cpu: CPUData,
    pub ram: RAMData,
    pub disk: DiskData,
    pub network: NetworkData,
    pub uptime: Duration,
}

/// Contains the information about the system's CPU
/// ### Fields
/// * `cpu_count` - The system's CPU core count
/// * `cpu_load` - The system's CPU load per core
/// * `cpu_load_average` - The system's CPU load average
/// * `cpu_temp` - The system's CPU temperature per core
struct CPUData {
    cpu_count: usize,
    cpu_load: Vec<CPULoad>,
    cpu_load_average: Vec<LoadAverage>,
    cpu_temp: Vec<f32>,
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
pub fn start_fetch(thr_data: Arc<Mutex<SystemData>>, interval: Duration) -> Result<(), Box<dyn Error>> {
    thread::spawn(move || {
        loop {
            let mut data: SystemData;
            // Fetch the most recent data from the system
            fetch_data(&mut data);
            // Update the shared data with new one
            let mut shared_data = thr_data.lock().unwrap(); // Lock the shared data and fetch it
            *shared_data = data;                            // Update the shared data
            drop(shared_data);                              // Drop the lock
            // Sleep for the interval
            thread::sleep(interval);
        }
    });
    Ok(())
}

fn fetch_data(data: &mut SystemData) -> Result<(), Box<dyn Error>> {
    let mut system = System::new();
    data.uptime = system.uptime()?;
    data.cpu = get_cpu_data(&system)?;
    data.ram = get_ram_data(&system)?;
    //data.disk = get_disk_data(&mut system)?;
    //data.network = get_network_data(&mut system)?;
    Ok(())
}

/// Fetches the current cpu usage of the system or throws error if the fetch fails,
/// the first index is the cpu core and the second is the exact usage
/// ### Parameters
/// * `system` - The reference to the System
fn get_cpu_data(
    system: &System,
) -> Result<CPUData, Box<dyn Error>> {
    let cpu_aggregate = system.cpu_load();
    let cpu_agg = cpu_aggregate?;
    let mut vec = vec![];
    thread::sleep(Duration::from_secs(1));
    let cpu = cpu_agg.done()?;
    let mut load_vec: Vec<CPULoad> = vec![];
    for i in 0..cpu.len() {
        load_vec.push(CPULoad {
            user: cpu[i].user,
            nice: cpu[i].nice,
            system: cpu[i].system,
            interrupt: cpu[i].interrupt,
            idle: cpu[i].idle,
            platform: cpu[i].platform,
        });
    }
    let data: CPUData = CPUData {
        cpu_count: cpu.len(),
        cpu_load: load_vec,
        cpu_load_average: Vec::new(),
        cpu_temp: Vec::new(),
    };
    Ok(data)
}

/// Fetches the current memory usage of the system or throws error if the fetch fails,
/// the first index is the total memory and the second is the used memory
/// ### Parameters
/// * `system` - The reference to the System
fn get_ram_data(system: &System) -> Result<RAMData, Box<dyn std::error::Error>> {
    match system.memory() {
        Ok(mem) => {
            let data: RAMData;
            data.ram_total = mem.total.as_u64();
            data.ram_free = mem.free.as_u64();
            data.ram_used = data.ram_total - data.ram_free;
            data.ram_percentage = data.ram_used as f32 / data.ram_total as f32 * 100_f32;
            data.platform = mem.platform_memory;
            Ok(data)
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