// std
use std::{env, thread, time::Duration, process::Command};
use thread::sleep;

// crates
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use signal_hook::{consts::SIGINT, iterator::Signals};
use colored::*;

// include files
include!(concat!(env!("OUT_DIR"), "/db.rs")); // build script output
mod functions;
mod structs;

// use files
use crate::structs::*;
use crate::functions::*;

// use crate::flag_parser::*;

// ======
// ACTIVE
// ======
fn active(prayer_data: Vec<PrayerData>, flag: &Flag) {
    new_buffer();
    
    let pt_names: Vec<_> = vec![" Fajr"," Sun"," Dhuhur"," Asr"," Maghrib"," Isha"];
    // symbols 
    // pt_names.iter().for_each(|x| notify_send(x));
    
    // active loop
    loop {
        let today: usize = chrono::offset::Local::now().ordinal() as usize - 1;
        let pt_vec = prayer_data[today].vec_from_island_set();
        let current_time = get_current_time_in_minutes() as i32;
        let (_, _, seconds, _) = get_current_time(&flag.time);
        
        for (i, x) in pt_vec.iter().enumerate(){
            
            if seconds == 0 && x == &current_time && flag.notify {
                let notif_text = pt_names[i];
                notify_send(notif_text);
            }
        };
        
        prayer_data[today].flag_formatted_output(flag);
        sleep(Duration::from_secs(1));
        clear_screen();
    }
}

// ====
// MAIN
// ====
fn main() {
    // init
    handle_ctrlc();
    
    // load config
    let cfg_result: Result<Config, confy::ConfyError> = confy::load("salat_mv", None);
    let mut cfg = match cfg_result {
        Ok(cfg_result) => cfg_result,
        Err(_cfg_result) => {
            println!("Warning: config was broken so it has been autofixed");
            Config {
                island_index: 57,
                island_name: "Male'".to_string(),
            }
        }
    };
    
    // autocorrect config that is out of bounds
    if cfg.island_index < 41 || cfg.island_index > 82 {
        println!("Warning: config was incorrect so it has been reset");
        cfg.island_index = 42;
        cfg.island_name = "Male'".to_string();
    }
    
    confy::store("salat_mv", "config", &cfg).unwrap();
    
    // fetch flags
    let args: Vec<String> = env::args().collect();
    let flag: Flag = match flag_parser::parse_args(args) {
        Ok(flag) => flag,
        Err(_flag) => return,
    };
    
    // main logic
    
    if flag.help {
        // breakout for help
        println!("{}", HELP_TEXT);
        return;
    }
    
    // gets data from database
    let prayer_data: Vec<PrayerData> = get_island_data(cfg.island_index);
    
    // gets today - 1
    let today: usize = chrono::offset::Local::now().ordinal() as usize - 1;
    
    // branch
    if flag.tui {
        // tui();
        println!("tui mode not added yet");
    } else if flag.edit {
        edit();
    } else if flag.active {
        active(prayer_data, &flag);
    } else {
        prayer_data[today].flag_formatted_output(&flag);
    }
}



//
// #[cfg(test)]
// mod test{
//     use super::main;
//     
//     #[test]
//     fn test_this(){
//         assert_eq!(main(),(1));
//     }
//     println!("testing 1 2 3");
// }

