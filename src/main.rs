use std::{env::{self, current_exe}, fs, thread::sleep};
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

struct ProgramData{
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
        let mut val = vec![0;6];
        val[0] = self.fajr;
        val[1] = self.sun;
        val[2] = self.dhuhur;
        val[3] = self.asr;
        val[4] = self.magrib;
        val[5] = self.isha;
        
        val
    }

    fn output(&self){
        dbg!(self);
    }

    fn flag_formatted_output(&self, flag:&Flag){
        let (flag,pt_vec) = (flag,self.vec_from_island_set());
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
            let (hour, minute, second) = get_current_time();
            
            println!("Salat_MV-cli");
            println!("---------------------");
            println!("Time   :  {}:{}:{}", hour, minute, second); // TODO: let flag -H work
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

trait PTDataParse {
    fn parse_for_island(self, island_index: i32) -> Vec<PrayerData>;
}

impl PTDataParse for String{
    fn parse_for_island(self, island_index: i32) -> Vec<PrayerData>{
        // split by line for each valid data
        let mut grouped :Vec<&str> = self.split('\n').collect();
        grouped.pop(); // remove last line
        grouped.reverse();
        grouped.pop(); // remove first line
        grouped.reverse();
        
        let mut full_list: Vec<PrayerData> = vec![];
        
        // split by column for each valid data
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

fn main(){
    // let new_data :PrayerData = PrayerData { island_index: 77, day: 233, fajr: 1234, sun: 1234, dhuhur: 1231, asr: 123, magrib: 12312, isha: 1231 };
    // new_data.output();
    // load config
    //
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
    // ==========
    
    if flag.help{ // breakout for help
        println!("{}",HELP_TEXT);
        return;
    }
    
    // data path
    let mut data_path: String = current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();
    data_path.push_str("/ptdata.csv");
    
    // gets data from database
    let raw_prayer_data: String = fs::read_to_string(data_path)
        .expect("READ THE data.txt FILE DAMMIT");
    
    
    let prayer_data: Vec<PrayerData> = raw_prayer_data.parse_for_island(cfg.island_index as i32);
    
    let today: usize = chrono::offset::Local::now().ordinal() as usize - 1;
    
    let mut pt_vec = prayer_data[today].vec_from_island_set();
    pt_vec.reverse();
    pt_vec.pop();
    pt_vec.pop();
    pt_vec.reverse();
    
    
    // let data: ProgramData = ProgramData { flag, cfg, pt_vec};
    new_buffer();
    let mut a = 0;
    loop{
        
        prayer_data[today].flag_formatted_output(&flag);
        sleep(Duration::from_secs(1));
        clear_screen();
        a +=1;
        if a == 5{break;}
    }
    exit_buffer();
    // if flag.tui{
    //     tui();
    // }
    // else if flag.edit{
    //     edit();
    // }
    // else if flag.notify{
    //     
    //     let data :ProgramData = ProgramData {flag, cfg, pt_vec };
    //     notify(data);
    //     Command::new("notify-send").args(["--urgency=low","ahahahahahahaha"]).output().expect("failed");
    //     // loop{
    //     //     std::thread::sleep_ms(1000);
    //     //     break;
    //     // }
    //     
    // }else{
    //     
    //     let data :ProgramData = ProgramData {flag, cfg, pt_vec };
    //     print_prayer_data(&data);
    // }
    // 
    // handle_prayer_data(flag, cfg); // run main thing
    
}

// smol functions
fn get_current_time_in_minutes() -> i32 {
    let current_time = chrono::offset::Local::now();
    (current_time.hour() * 60 + current_time.minute()) as i32
}
fn get_current_time() -> (u32, u32, u32){
    let current_time = chrono::offset::Local::now();
    (current_time.hour(), current_time.minute(), current_time.second())
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

fn new_buffer(){
    print!("\x1b[?1049h");
}
fn exit_buffer(){
    print!("\x1b[?1049l");
}

