extern crate systemstat;

use systemstat::{saturating_sub_bytes, Platform, System, LoadAverage, PlatformMemory, Filesystem, NetworkAddrs };

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

pub struct CPULoad {
    pub user: f32,
    pub nice: f32,
    pub system: f32,
    pub interrupt: f32,
    pub idle: f32,
}

/// Contains the information about the system's CPU
/// ### Fields
/// * `count` - The system's CPU core count
/// * `load` - The system's CPU load per core
/// * `load_average` - The system's CPU load average
/// * `temp` - The system's CPU temperature per core
pub struct CPUData {
    pub count: usize,
    pub load: Vec<CPULoad>,
    pub load_average: Vec<LoadAverage>,
    pub temp: Vec<f32>,
}

/// Contains the information about the system's RAM
/// ### Fields
/// * `total` - The system's total RAM
/// * `used` - The system's used RAM
/// * `free` - The system's free RAM
/// * `percentage` - The system's used RAM percentage
/// * `platform` - The system's RAM platform specific data
pub struct RAMData {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub percentage: f32,
    pub platform: PlatformMemory,
}

/// Contains the information about the system's disk
/// ### Fields
/// * `count` - The system's disk count
/// * `total` - The system's disk space per disc
/// * `used` - The system's used disk space per disc
/// * `free` - The system's free disk space per disc
/// * `percentage` - The system's used disk space percentage per disc
pub struct DiskData {
    pub count: usize,
    pub total: Vec<u64>,
    pub used: Vec<u64>,
    pub free: Vec<u64>,
    pub percentage: Vec<f32>,
}

/// Contains the information about the system's network
/// ### Fields
/// * `count` - The system's network interface count
/// * `names` - The system's network interface names
/// * `addresses` - The system's network interface addresses
/// * `rx` - The system's network interface received bytes per interface
/// * `tx` - The system's network interface transmitted bytes per interface
pub struct NetworkData {
    pub count: usize,
    pub names: Vec<String>,
    pub adresses: Vec<Vec<NetworkAddrs>>,
    pub rx: Vec<u64>,
    pub tx: Vec<u64>,
}

pub fn start_data_fetcher() -> Result<SystemData, Box<dyn Error>> {
    // Fetch the most recent data from the system
    let data = fetch_data()?;
    Ok(data)
}


/// Starts the system data fething framework
/// ### Parameters
/// * `thr_data` - The shared data that the thread will use and update
/// * `interval` - The interval in milliseconds between each data fetching
pub fn start_fetch(thr_data: Arc<Mutex<SystemData>>, interval: Duration) -> Result<(), Box<dyn Error>> {
    thread::spawn(move || {
        loop {
            // Fetch the most recent data from the system
            match fetch_data() {
                Ok(data) => {
                    // Update the shared data
                    let mut data_lock = thr_data.lock().unwrap(); // Lock the shared data
                    *data_lock = data; // Update the shared data
                    drop(data_lock); // Drop the lock
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
            // Sleep for the interval
            thread::sleep(interval);
        }
    });
    Ok(())
}

fn fetch_data() -> Result<SystemData, Box<dyn Error>> {
    let system = System::new();
    let data: SystemData = SystemData {
        cpu: get_cpu_data(&system)?,
        ram: get_ram_data(&system)?,
        disk: get_disk_data(&system)?,
        network: NetworkData {
            count: 0,
            names: Vec::new(),
            adresses: Vec::new(),
            rx: Vec::new(),
            tx: Vec::new(),
        },
        uptime: system.uptime()?,
    };
    //data.disk = get_disk_data(&mut system)?;
    //data.network = get_network_data(&mut system)?;
    Ok(data)
}

/// Fetches the current cpu usage of the system or throws error if the fetch fails,
/// the first index is the cpu core and the second is the exact usage
/// ### Parameters
/// * `system` - The reference to the System
fn get_cpu_data(
    system: &System,
) -> Result<CPUData, Box<dyn Error>> {
    let cpu_agg = system.cpu_load()?;
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
            //platform: cpu[i].platform,
        });
    }
    let data: CPUData = CPUData {
        count: cpu.len(),
        load: load_vec,

        // TODO: Implement these
        load_average: Vec::new(),
        temp: Vec::new(),
    };
    Ok(data)
}

/// Fetches the current memory usage of the system or throws error if the fetch fails,
/// the first index is the total memory and the second is the used memory
/// ### Parameters
/// * `system` - The reference to the System
fn get_ram_data(system: &System) -> Result<RAMData, Box<dyn Error>> {
    match system.memory() {
        Ok(mem) => {
            let data: RAMData = RAMData {
                total: mem.total.as_u64(),
                used: mem.total.as_u64() - mem.free.as_u64(),
                free: mem.free.as_u64(),
                percentage: mem.total.as_u64() as f32 - mem.free.as_u64() as f32 / mem.total.as_u64() as f32 * 100_f32,
                platform: mem.platform_memory,
            };
            Ok(data)
        }
        Err(x) => Err(Box::new(x)),
    }
}

fn get_disk_data(system: &System) -> Result<DiskData, Box<dyn Error>> {
    match system.mounts() {
        Ok(mounts) => {
            let mut total: Vec<u64> = Vec::new();
            let mut used: Vec<u64> = Vec::new();
            let mut free: Vec<u64> = Vec::new();
            let mut percentage: Vec<f32> = Vec::new();
            for mount in &mounts {
                total.push(mount.total.as_u64());
                used.push(mount.total.as_u64() - mount.avail.as_u64());
                free.push(mount.free.as_u64());
                percentage.push((mount.total.as_u64() - mount.avail.as_u64()) as f32 / mount.total.as_u64() as f32 * 100_f32);
            }
            let data: DiskData = DiskData {
                count: mounts.len(),
                total: total,
                used: used,
                free: free,
                percentage: percentage,
            };
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