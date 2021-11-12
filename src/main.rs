extern crate systemstat;

use std::io::stdout;
use std::error::Error;
use std::process;
use std::thread;
use std::time::Duration;

use ctrlc;

use futures::executor::block_on;
use systemstat::{saturating_sub_bytes, Platform, System};

use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{
    Clear,
    ClearType::{All, CurrentLine},
};
use crossterm::{
    cursor::{MoveTo},
    execute,
    event::{Event, read},
};

mod ui;
mod datafetcher;
mod utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The main function of the program
fn main() {

    ui::init();
    // CTRL-C handler
    ctrlc::set_handler(move || {
        ui::reset();
    })
    .expect("Error setting Ctrl + C handler");

    // Create thread for keyboard events
    thread::spawn(|| -> crossterm::Result<()> {
        // Loop for keyboard events
        loop {
            // `read()` blocks until an `Event` is available
            match read()? {
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(_width, _height) => execute!(stdout(), Clear(All))?,
            }
        }
    });


    // Block main thread until process finishes
    match block_on(async_main()) {
        Ok(_) => {
            ui::reset();
            process::exit(0);
        },
        Err(e) => {
            //ui::reset();
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}

async fn async_main() -> Result<String, Box<dyn Error>> {
    let sys = System::new();
    let mut term_size = crossterm::terminal::size()?;

    for _i in 0..term_size.1 {
        print!("\n");
    }
    loop {
        let mut top_left_str: String = String::new();
        let mut top_right_str: String = String::new();
        let mut bottom_left_str: String = String::new();
        let mut bottom_right_str: String = String::new();
        let temp_size = crossterm::terminal::size()?;
        // If terminal has been resized, clear everything
        if temp_size.0 != term_size.0 || temp_size.1 != term_size.1 {
            term_size = temp_size;
            execute!(stdout(), Clear(All))?;
        }
        top_left_str += &format!(
            "RCTOP v{} [Width: {}, Height: {}]",
            VERSION, term_size.0, term_size.1
        );
        let uptime = sys.uptime()?;
        top_right_str += &format!("Uptime: {}", utils::parse_time(&uptime));
        execute!(
            stdout(),
            MoveTo(0, 0),
            Clear(CurrentLine),
            SetBackgroundColor(Color::DarkCyan)
        )
        ?;
        print!(" ");
        if term_size.0 > top_left_str.len() as u16 + top_right_str.len() as u16 + 1 {
            print!("{}", top_left_str);
            for _i in 0..(term_size.0 as usize - top_left_str.len() - top_right_str.len() - 1) {
                print!(" ");
            }
            print!("{}", top_right_str);
        }
        else if term_size.0 > top_left_str.len() as u16 as u16 + 1 {
            print!("{}", top_left_str);
            for _i in 0..(term_size.0 as usize - top_left_str.len() - 1) {
                print!(" ");
            }
        } else {
            top_left_str.truncate(term_size.0 as usize + 4);
            top_left_str += "...";
            print!("{} ", top_left_str);
        }

        execute!(stdout(), ResetColor, MoveTo(0, 2))?;

        // Total CPU usage is 0 at first in case of error
        let mut total_cpu: f32 = 0_f32;
        let mut memory = vec![0, 0];
        // Fetches the CPU usage for each core and prints it
        let cpu_usages = get_cpu_stats(&sys)?;
        let cpu_count_string_length: usize = cpu_usages.len().to_string().len();
        for i in 0..cpu_usages.len() {
            execute!(stdout(), Clear(CurrentLine))?;
            print!("CPU {}:", i);
            for _j in i.to_string().len()..cpu_count_string_length + 1 {
                print!(" ");
            }
            print_bar(
                term_size.0 - 8,
                100_f32 - &cpu_usages[i][4],
                Color::DarkGreen,
            )?;
            println!("");
            execute!(stdout(), Clear(CurrentLine))?;
            //println!("Load: {:.2}%", 100_f32 - &cpu_usages[i][4]);
            // Sum up the cpu usages
            total_cpu += &cpu_usages[i][4];
        }
        // Get total cpu usage by dividing with the core count
        total_cpu = 100_f32 - total_cpu / cpu_usages.len() as f32;
        // Fetches the memory usage and prints it
        match get_mem_size(&sys) {
            Ok(mem_size) => {
                memory = mem_size;
                println!(" ");
                execute!(stdout(), Clear(CurrentLine))?;
                print!("Memory: ");
                print_bar(
                    term_size.0 - 8,
                    memory[1] as f32 / memory[0] as f32 * 100_f32,
                    Color::DarkYellow,
                )?;
                println!("");
                // execute!(stdout(), Clear(CurrentLine))?;
                // print!("Swap: ");
                // print_bar(
                //     term_size.0 - 5,
                //     memory[3] as f32 / memory[2] as f32 * 100_f32,
                //     Color::DarkYellow,
                // );
                // println!("");
            }
            Err(x) => print!("\nMemory: error: {}", x.to_string()),
        }


        //print_graph_stats(&cpu_vec, term_size.0 / 2, term_size.1 - 3, term_size.0, term_size.1);
        execute!(
            stdout(),
            MoveTo(0, term_size.1),
            Clear(CurrentLine),
            SetBackgroundColor(Color::DarkCyan)
        )
        ?;
        for _i in 0..term_size.0 {
            print!(" ");
        }
        bottom_left_str += &format!("CPU: {:.2}% ", total_cpu);
        bottom_left_str += &format!("RAM: {} / {} ", utils::parse_size(&memory[1]), utils::parse_size(&memory[0]));
        let battery = sys.battery_life()?;
        bottom_right_str += &format!(
            "Battery: {:.2}%, {}",
            battery.remaining_capacity * 100.0,
            utils::parse_time(&battery.remaining_time)
        );
        execute!(
            stdout(),
            MoveTo(0, term_size.1),
            Clear(CurrentLine),
            SetBackgroundColor(Color::DarkCyan)
        )?;
        print!(" ");
        if term_size.0 > bottom_left_str.len() as u16 + bottom_right_str.len() as u16 + 2 {
            print!("{}", bottom_left_str);
            for _i in 0..(term_size.0 as usize - bottom_left_str.len() - bottom_right_str.len() - 2) {
                print!(" ");
            }
            print!("{} ", bottom_right_str);
        }
        else if term_size.0 > bottom_left_str.len() as u16 + 1 {
            print!("{}", bottom_left_str);
            for _i in 0..(term_size.0 as usize - bottom_left_str.len() - 1) {
                print!(" ");
            }
        } else {
            bottom_left_str.truncate(term_size.0 as usize + 4);
            bottom_left_str += "...";
            print!("{} ", bottom_left_str);
        }
        execute!(stdout(), ResetColor)?;
    }
}

// fn print_graph_stats(
//     cpu_vec: &std::vec::Vec<f32>,
//     max_width: u16,
//     max_height: u16,
//     x_offset: u16,
//     y_offset: u16,
// ) {
//     let mut index: usize = 0;
//     let length = cpu_vec.len();
//     for i in y_offset - max_height..y_offset {
//         execute!(stdout(), MoveTo(0, i))?;
//         execute!(stdout(), Clear(CurrentLine))?;
//     }
//     while index < max_width.into() && index < length {
//         let height = max_height as f32 / 100_f32 * cpu_vec[&length - 1 - &index];
//         let floored: u16 = height as u16;
//         execute!(
//             stdout(),
//             MoveTo(x_offset - index as u16, y_offset - max_height + floored)
//         )
//         ?;
//         if (height - floored as f32) <= 0.33 {
//             print!("_");
//         } else if (height - floored as f32) <= 0.66 {
//             print!("-");
//         } else {
//             print!("¯");
//         }
//         index += 1;
//     }
// }

/// Prints a bar that is as long as the percentage of the given terminal width
/// ### Arguments
///
/// * `max_width` - The max width of the bar
/// * `percentage` - The percentage of the max width the bar is going to be
fn print_bar(max_width: u16, percentage: f32, color: Color) -> Result<(), Box<dyn Error>> {
    execute!(stdout(), SetForegroundColor(color))?;
    let block_count = max_width as f32 / 100_f32 * percentage;
    let mut index: u16 = 0;
    let floored = block_count as u16;
    // Print the full bars
    while index < floored {
        print!("⧛");
        index = index + 1;
    }
    // Determine the last bar from decimal
    if floored != 100 {
        if (block_count - floored as f32) <= 0.5 {
            print!("⧙");
        } else {
            print!(" ");
        }
    }
    execute!(stdout(), ResetColor)?;
    Ok(())
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
