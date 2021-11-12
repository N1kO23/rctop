extern crate systemstat;

use std::io::stdout;
use std::error::Error;
use std::process;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

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
    event::{Event, read, KeyCode},
};

use datafetcher::SystemData;

const REFRESH: Duration = Duration::from_secs(1);

mod ui;
mod datafetcher;
mod utils;

/// The main function of the program
fn main() {

    ui::init();
    // CTRL-C handler
    ctrlc::set_handler(move || {
        ui::exit();
        process::exit(0);
    })
    .expect("Error setting Ctrl + C handler");

    // Block main thread until process finishes
    match block_on(async_main()) {
        Ok(_) => {
            ui::exit();
            process::exit(0);
        },
        Err(e) => {
            ui::exit();
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}

async fn async_main() -> Result<String, Box<dyn Error>> {
    let mut term_size = crossterm::terminal::size()?;
    let system_data: SystemData = datafetcher::start_data_fetcher()?;
    let system_data_arc = Arc::new(Mutex::new(system_data));
    let thr_data = system_data_arc.clone();
    let thr_data_2 = system_data_arc.clone();
    datafetcher::start_fetch(thr_data, REFRESH)?;

    // thread::spawn(move ||  {
    //     loop {
    //         let mut data = thr_data.lock().unwrap();
    //         *data += "Bababooey ";
    //         drop(data);
    //         thread::sleep(Duration::from_millis(100));
    //     }
    // });

    // Create thread for keyboard events
    thread::spawn(move || -> crossterm::Result<()> {
        let thr_data2 = system_data_arc.clone();
        let term_size = crossterm::terminal::size()?;
        let mut selection: (usize, usize) = (0, 0);
        // Loop for keyboard events
        loop {
            // `read()` blocks until an `Event` is available
            match read()? {
                Event::Key(event) => {
                    println!("{:?}", event);
                    
                    match event.code {
                        // Close the program gracefully
                        KeyCode::Char('q') => {
                            ui::exit();
                            process::exit(0);
                        },
                        KeyCode::Char('c') => {
                            ui::reset();
                            ui::update_menu_header(&mut selection, term_size);
                            ui::print_system_data(&thr_data2, &mut selection, term_size);
                        },
                        KeyCode::Up => {
                            if selection.1 != 0 {
                                if selection.1 != 1 {
                                    ui::update_menu_header(&mut selection, term_size);
                                }
                                ui::print_system_data(&thr_data2, &mut selection, term_size);
                            }
                        },
                        KeyCode::Down => {
                            if selection.1 == 0 {
                                ui::update_menu_header(&mut selection, term_size);
                            }
                            ui::print_system_data(&thr_data2, &mut selection, term_size);
                        },
                        KeyCode::Left => {
                            if selection.1 == 0 {
                                ui::update_menu_header(&mut selection, term_size);
                                ui::print_system_data(&thr_data2, &mut selection, term_size);
                            }
                        },
                        KeyCode::Right => {
                            if selection.1 == 0 {
                                ui::update_menu_header(&mut selection, term_size);
                                ui::print_system_data(&thr_data2, &mut selection, term_size);
                            }
                        },
                        _ => {
                        }
                    }
                },
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(_width, _height) => {
                    ui::reset();
                },
            }
        }
        Ok(())
    });

    for _i in 0..term_size.1 {
        print!("\n");
    }
    loop {
        thread::sleep(REFRESH);
        let sys = thr_data_2.lock().unwrap();
        term_size = crossterm::terminal::size()?;
        print!(" ");

        execute!(stdout(), ResetColor, MoveTo(0, 2))?;

        // Total CPU usage is 0 at first in case of error
        let mut total_cpu: f32 = 0_f32;
        // Fetches the CPU usage for each core and prints it
        let cpu_usages = &sys.cpu;
        let cpu_count_string_length: usize = cpu_usages.count.to_string().len();
        for i in 0..cpu_usages.count {
            execute!(stdout(), Clear(CurrentLine))?;
            print!("CPU {}:", i);
            for _j in i.to_string().len()..cpu_count_string_length + 1 {
                print!(" ");
            }
            print_bar(
                term_size.0 - 8,
                100_f32 - &cpu_usages.load[i].idle,
                Color::DarkGreen,
            )?;
            println!("");
            execute!(stdout(), Clear(CurrentLine))?;
            //println!("Load: {:.2}%", 100_f32 - &cpu_usages[i][4]);
            // Sum up the cpu usages
            total_cpu += 100_f32 - &cpu_usages.load[i].idle;
        }
        // Get total cpu usage by dividing with the core count
        total_cpu = 100_f32 - total_cpu / cpu_usages.count as f32;


        println!(" ");
        execute!(stdout(), Clear(CurrentLine))?;
        print!("Memory: ");
        print_bar(
            term_size.0 - 8,
            sys.ram.used as f32 / sys.ram.total as f32 * 100_f32,
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
        let mut bottom_left_str: String = String::new();
        let mut bottom_right_str: String = String::new();
        bottom_left_str += &format!("CPU: {:.2}% ", total_cpu);
        bottom_left_str += &format!("RAM: {} / {} ", utils::parse_size(&sys.ram.used), utils::parse_size(&sys.ram.total));
        // let battery = sys.battery_life()?;
        // bottom_right_str += &format!(
        //     "Battery: {:.2}%, {}",
        //     battery.remaining_capacity * 100.0,
        //     utils::parse_time(&battery.remaining_time)
        // );
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
            bottom_left_str.truncate(term_size.0 as usize - 5);
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
/// ### Parameters
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
