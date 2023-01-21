// std
use std::{time::Duration, thread::sleep, env};

// crates
use chrono::prelude::*;

// include files
mod flag_parser;
mod inits;
mod db;
mod edit;
mod functions;

// use files
use flag_parser::*;
use inits::*;
use db::*;
use edit::*;
use functions::*;




impl PrayerData{
    // fn new() -> PrayerData{
    //     PrayerData { island_index: 0, day: 0, fajr: 0, sun: 0, dhuhur: 0, asr: 0, magrib: 0, isha: 0}
    // }
    
    fn island_set_from_vec(val: Vec<u32>) -> PrayerData{
        PrayerData {
            island_index: val[0],
            day   : val[1],
            fajr  : val[2],
            sun   : val[3],
            dhuhur: val[4],
            asr   : val[5],
            magrib: val[6],
            isha  : val[7]
        }
    }
    
    fn vec_from_island_set(&self) -> Vec<i32>{
        let mut val = vec![0;6];
        val[0] = self.fajr   as i32;
        val[1] = self.sun    as i32;
        val[2] = self.dhuhur as i32;
        val[3] = self.asr    as i32;
        val[4] = self.magrib as i32;
        val[5] = self.isha   as i32;
        
        val
    }
    
    fn flag_formatted_output(&self, flag:&Flag){
        let (flag,pt_vec) = (flag,self.vec_from_island_set());
        
        // some temporary inits
        let names = vec!["Fajr","Sun","Dhuhur","Asr","Magrib","Isha"];
        
        // optional title
        if flag.title && !flag.tui && !flag.edit{
            let (hour, minute, second, time) = get_current_time(&flag.time);
            
            
            println!("Salat_MV-cli");
            println!("---------------------");
            println!("Time   :  {}:{}:{} {}", hour.add_zero(), minute.add_zero(), second.add_zero(),time); // TODO: let flag -H work
            // println!("Island :  {}",cfg.island_name);
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
                let time_minutes : i32 = get_current_time_in_minutes() as i32;
                let prev_diff    : i32 = {if i > 0{pt_vec[i-1]-time_minutes}else{pt_vec[5]-time_minutes}};
                let diff         : i32 = pt - time_minutes;
                let next_diff    : i32 = {if i < 5{pt_vec[i+1]-time_minutes}else{pt_vec[0]-time_minutes}};
                
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
    }
}


trait TimeConversion{
    fn add_zero(self) -> String;
    fn to_12(self) -> (u32,String);
    fn minutes_to_time(self, time_format: &TimeFormat) -> String;
}

impl TimeConversion for i32{
    
    fn add_zero(self) -> String{
        if self < 10 {
            let mut string: String = "0".to_string();
            string.push_str(&self.to_string());
            string
        }else{
            self.to_string()
        }
    }
    
    fn to_12(self) -> (u32,String){
        let half = if self > 11 {"pm"}else{"am"};
        if self > 12{
            (self as u32 - 12,half.to_string())
        }else{
            (self as u32 ,half.to_string())
        }
    }
    
    fn minutes_to_time(self, time_format: &TimeFormat) -> String{
        
        let minute = &self%60;
        let mut hour = self as u32 /60;
        let mut period = "".to_string();
        
        match time_format{
            TimeFormat::TWHour => {(hour,period) = hour.to_12()}
            TimeFormat::TFHour => {}
        }
        
        let hour   = hour  .add_zero();
        let minute = minute.add_zero();
        format!("{}:{} {}", hour, minute, period)
        
    }
}

impl TimeConversion for u32{
    
    fn add_zero(self) -> String{
        if self < 10 {
            let mut string: String = "0".to_string();
            string.push_str(&self.to_string());
            string
        }else{
            self.to_string()
        }
    }
    
    fn to_12(self) -> (u32,String){
        let half = if self > 11 {"pm"}else{"am"};
        if self > 12{
            (self - 12,half.to_string())
        }else{
            (self,half.to_string())
        }
    }
    
    fn minutes_to_time(self, time_format: &TimeFormat) -> String{
        
        let minute = &self%60;
        let mut hour = self/60;
        let mut period = "".to_string();
        
        match time_format{
            TimeFormat::TWHour => {(hour,period) = hour.to_12()}
            TimeFormat::TFHour => {}
        }
        
        let hour   = hour  .add_zero();
        let minute = minute.add_zero();
        format!("{}:{} {}", hour, minute, period)
    }
}

// ======
// ACTIVE
// ======
fn active(prayer_data: Vec<PrayerData>, flag: &Flag){
    
    new_buffer();
    
    // active loop
    loop{
        let today: usize = chrono::offset::Local::now().ordinal() as usize;
        let pt_vec = prayer_data[today-1].vec_from_island_set();
        let current_time = get_current_time_in_minutes() as i32;
        let (_,_,seconds,_) = get_current_time(&flag.time);
        
        // let current_time = 738;
        pt_vec.iter().for_each(|x| if seconds == 0 && x == &current_time && flag.notify{notify_send("ITS TIME")});
        
        prayer_data[today].flag_formatted_output(flag);
        sleep(Duration::from_secs(1));
        clear_screen();
    }
}


// ====
// MAIN
// ====
fn main(){
    
    // init
    handle_ctrlc();
    
    // load config
    let cfg_result : Result<Config,confy::ConfyError> = confy::load("salat_mv", None);
    let mut cfg = match cfg_result{
        Ok(cfg_result)  => cfg_result,
        Err(_cfg_result) => {
            println!("Warning: config was broken so it has been autofixed");
            Config { island_index: 57, island_name: "Male'".to_string() } 
        },
    };
    
    
    // autocorrect config that is out of bounds
    if cfg.island_index < 41 || cfg.island_index > 82{
        println!("Warning: config was incorrect so it has been reset");
        cfg.island_index = 42;
        cfg.island_name = "Male'".to_string();
    }
    
    confy::store("salat_mv",None, &cfg).unwrap();
    
    // fetch flags
    let args : Vec<String> = env::args().collect();
    let flag: Flag = match flag_parser::parse_args(args){
        Ok(flag) => flag,
        Err(_flag) => return,
    };
    
    
    // main logic
    
    if flag.help{ // breakout for help
        println!("{}",HELP_TEXT);
        return;
    }
    
    // gets data from database
    let prayer_data: Vec<PrayerData> = get_island_data(cfg.island_index as u32);
    
    // gets today - 1
    let today: usize = chrono::offset::Local::now().ordinal() as usize - 1;
    
    // branch
    if flag.tui{
        // tui();
        println!("not added yet");
    }
    else if flag.edit{
        edit();
    }
    else if flag.active{
        active(prayer_data, &flag);
    }
    else{    
        prayer_data[today].flag_formatted_output(&flag);
    }
}





