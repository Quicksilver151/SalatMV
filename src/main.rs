use std::{env::{self, current_exe}, fs};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::process::Command;
use std::time::Duration;

mod flag_parser;
use flag_parser::*;


#[derive(Default, Debug, Serialize, Deserialize)]
struct Config{
    island_index: usize,
    island_name:String,
}

#[derive(Debug)]
struct PrayerData{
    island_index: i32,
    day:    i32,
    fajr:   i32,
    sun:    i32,
    dhuhur: i32,
    asr:    i32,
    magrib: i32,
    isha:   i32,
}

struct InputData{
    flag:Flag,
    cfg:Config,
    pt_vec:Vec<i32>,
}

impl PrayerData{
    fn island_set_from_vec(&mut self, val: Vec<i32>){
        self.island_index = val[0];
        self.day    = val[1];
        self.fajr   = val[2];
        self.sun    = val[3];
        self.dhuhur = val[4];
        self.asr    = val[5];
        self.magrib = val[6];
        self.isha   = val[7];
    }
    
    fn vec_from_island_set(&self) -> Vec<i32>{
        let mut val = vec![0;8];
        val[0] = self.island_index;
        val[1] = self.day;
        val[2] = self.fajr;
        val[3] = self.sun;
        val[4] = self.dhuhur;
        val[5] = self.asr;
        val[6] = self.magrib;
        val[7] = self.isha;
        
        val
    }
    fn output(&self){
        dbg!(self);
    
    }
    fn flag_formatted_output(data:InputData){
        let (flag,cfg,pt_vec) = (data.flag,data.cfg,data.pt_vec);
        // Debug loop over each minute of the day
        //----------------------------------
        // clear_screen();
        // let mut time_minutes = 0;
        // while  time_minutes < 1440{
        // std::thread::sleep_ms(5);
        // --------------------------------
        
        // some temporary inits
        let names = vec!["Fajr","Sun","Dhuhur","Asr","Magrib","Isha"];
        
        // optional title
        if flag.disp == DispType::Normal && flag.title && !flag.tui && !flag.edit{
            let time_minutes = get_current_time_in_minutes();
            
            println!("Salat_MV-cli");
            println!("---------------------");
            println!("Time   :  {}", time_minutes.minutes_to_time(&flag.time));
            println!("Island :  {}",cfg.island_name);
            println!("---------------------");
            println!();
        }
        
        for (i,pt) in pt_vec.iter().enumerate(){
               
            match flag.disp{ // only numbers or with info
                
                DispType::Normal => print!("{}:\t",names[i]),
                DispType::Raw    => print!(""),
            }
            match flag.output{
                OutType::Hours   => print!("{}",pt.minutes_to_time(&flag.time)),
                OutType::Minutes => print!("{}",pt),
            }
            if flag.current{
                let time_minutes = get_current_time_in_minutes();
                let prev_diff = {if i > 0{pt_vec[i-1]-time_minutes}else{pt_vec[5]-time_minutes}};
                let diff = pt-time_minutes;
                let next_diff = {if i < 5{pt_vec[i+1]-time_minutes}else{pt_vec[0]-time_minutes}};
                
                let tail_prev_len = "-".repeat(10.min(prev_diff.abs()/10) as usize);
                let tail_next_len = "-".repeat(10.min(next_diff.abs()/10) as usize);
                
                // print!("\t{},\t{},\t{}",prev_diff,diff,next_diff);
                if diff.is_negative() && next_diff.is_positive(){
                    print!(" /{}",tail_next_len);
                }
                if prev_diff.is_negative() && diff.is_positive(){
                    print!(" \\{}",tail_prev_len);
                }
                if pt_vec[0] > time_minutes || pt_vec[5] < time_minutes{
                    
                    if pt == &pt_vec[5]{print!(" /{}",tail_next_len)}
                    if pt == &pt_vec[0]{print!(" \\{}",tail_prev_len)}
                }
                if pt == &time_minutes{
                    print!(" <<------------");
                }
            }
            
            println!();
            
        }
        //------------------
        //time_minutes += 1;
        //}
        //-----------------
    }
    
}


trait TimeConversion{
    fn minutes_to_time(self, time_format: &TimeType) -> String;
}

impl TimeConversion for i32{
    fn minutes_to_time(self, time_format: &TimeType)-> String{
        
        let minute = &self%60;
        let mut hour = self/60;
        let mut full_time_string = "".to_owned();
        let mut time = "";
        
        
        match time_format{
            TimeType::TWHour => {
                time = "am";
                if hour == 12{
                    time = "pm";
                }
                if hour > 12{
                    hour -= 12;
                    time = "pm";
                }
            }
            
            TimeType::TFHour => {}
        }
        
        
        if hour < 10{
            full_time_string.push('0');
        }
        full_time_string.push_str(&hour.to_string());
        full_time_string.push(':');
        
        if minute < 10{
            full_time_string.push('0');
        }
        full_time_string.push_str(&minute.to_string());
        full_time_string.push(' ');
        full_time_string.push_str(time);
        
        
        full_time_string
    }
}


fn main(){
    let new_data :PrayerData = PrayerData { island_index: 77, day: 233, fajr: 1234, sun: 1234, dhuhur: 1231, asr: 123, magrib: 12312, isha: 1231 };
    new_data.output();
}

// smol functions
fn get_current_time_in_minutes() -> i32 {
    let current_time = chrono::offset::Local::now();
    (current_time.hour() * 60 + current_time.minute()) as i32
}


fn get_number_input() -> Result<usize,std::num::ParseIntError>{
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
    let trimmed = input_text.trim();
    trimmed.parse::<usize>()   
}


fn clear_screen(){
    print!("\x1B[2J");
    print!("\x1b[1;1H");
}

