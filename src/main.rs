use std::{env::{self, current_exe}, fs};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use flag_parser::*;

mod flag_parser;

#[derive(Default, Debug, Serialize, Deserialize)]
struct Config{
    island_index:i32
}

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

trait PTDataParse {
    fn parse_for_island(self, island_index: i32) -> Vec<PrayerData>;
}

impl PTDataParse for String{
    fn parse_for_island(self, island_index: i32) -> Vec<PrayerData>{
        
        
        let mut grouped :Vec<&str> = self.split('\n').collect();
        grouped.pop();
        grouped.reverse();
        grouped.pop();
        grouped.reverse();
        
        let mut full_list: Vec<PrayerData> = vec![];
        
        for group in grouped{
            let columns: Vec<&str> = group.split(';').collect();
            
            if island_index != columns[0].parse::<i32>().unwrap(){
                continue;
            }
            
            let mut result : PrayerData = PrayerData { island_index: (0), day: (0), fajr: (0), sun: (0), dhuhur: (0), asr: (0), magrib: (0), isha: (0) };
            
            result.island_set_from_vec(columns.iter().map(|x| x.parse::<i32>().unwrap()).collect());
            full_list.append(&mut vec![result]);
            
        }
        
        full_list   
    }
}



// TODO::::::::::::::::::;
fn tui(){
    println!("feature not implemented ([yet]->i hope)");
}

fn edit(){

}

fn handle_prayer_data(flag: Flag, cfg: Config){
    
    // data path
    let mut data_path: String = current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();
    data_path.push_str("/ptdata.csv");
    
    // gets data from database
    let data: String = fs::read_to_string(data_path)
        .expect("READ THE data.txt FILE DAMMIT");
    
    
    let prayer_data : Vec<PrayerData> = data.parse_for_island(cfg.island_index);
    
    let today: usize = chrono::offset::Local::now().ordinal() as usize;
    
    
    let mut pt_vec = prayer_data[today].vec_from_island_set();
    pt_vec.reverse();
    pt_vec.pop();
    pt_vec.pop();
    pt_vec.reverse();
    
    let names = vec!["Fajr","Sun","Dhuhur","Asr","Magrib","Isha"];
    
    for (i,pt) in pt_vec.iter().enumerate(){
        if flag.tui{
            tui();
            break;
        }
        if flag.edit{
            edit();
            break;
        }
        match flag.disp{
            DispType::Normal => print!("{}:\t",names[i]),
            DispType::Raw    => print!(""),
        }
        match flag.output{
            OutType::Hours   => print!("{}",pt.minutes_to_time(&flag.time)),
            OutType::Minutes => print!("{}",pt),
        }
        if flag.current{
            let current_time = chrono::offset::Local::now();
            let time_minutes = (current_time.hour() * 60 + current_time.minute()) as i32;
            
            let prev_diff = {if i > 0{pt_vec[i-1]-time_minutes}else{pt_vec[5]-time_minutes}};
            let diff = pt-time_minutes;
            let next_diff = {if i < 5{pt_vec[i+1]-time_minutes}else{pt_vec[0]-time_minutes}};
            
            // print!("\t{},\t{},\t{}",prev_diff,diff,next_diff);
            if diff.is_negative() && next_diff.is_positive(){
                print!(" /------");
            }
            if prev_diff.is_negative() && diff.is_positive(){
                print!(" \\------");
            }
            if pt_vec[0] > time_minutes || pt_vec[5] < time_minutes{
                if pt == &pt_vec[5]{print!(" /------")}
                if pt == &pt_vec[0]{print!(" \\------")}
            }
            
        }
        
        println!();
        
    };
}


fn main(){
    
    // load config
    let mut cfg: Config = confy::load("salat_mv", None).unwrap();
    
    // fetch flags
    let args : Vec<String> = env::args().collect();
    let flag: Flag = match flag_parser::parse_args(args){
        Ok(flag) => flag,
        Err(_flag) => return,
    };
    
    // autocorrect config
    if cfg.island_index < 41 && cfg.island_index > 82{
        cfg.island_index = 42;
        confy::store("salat_mv",None, &cfg).unwrap();
    }
    
     
    // main logic
    if flag.help{
        println!("{}",HELP_TEXT);
        return;
    }
    
    
    handle_prayer_data(flag, cfg);
    
    
}
