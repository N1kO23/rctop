extern crate systemstat;
use systemstat::{saturating_sub_bytes, Platform, System};

pub fn fetch() {
    let mut stat = System::new();
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