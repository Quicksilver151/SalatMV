//std
use std::{process::Command, time::Duration, fs, env, thread};
use env::current_exe;
use thread::sleep;

//crates
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use signal_hook::{consts::SIGINT, iterator::Signals};

//files
mod flag_parser;
use flag_parser::*;
mod db;
use db::*;


#[derive(Default, Debug, Serialize, Deserialize)]
struct Config{
    island_index: usize,
    island_name:String,
}

#[derive(Debug)]
pub struct PrayerData{
    island_index: u32,
    day:    u32,
    fajr:   u32,
    sun:    u32,
    dhuhur: u32,
    asr:    u32,
    magrib: u32,
    isha:   u32,
}


impl PrayerData{
    // fn island_set_from_vec(&mut self, val: Vec<u32>){
    //     self.island_index = val[0];
    //     self.day    = val[1];
    //     self.fajr   = val[2];
    //     self.sun    = val[3];
    //     self.dhuhur = val[4];
    //     self.asr    = val[5];
    //     self.magrib = val[6];
    //     self.isha   = val[7];
    // }
    
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

trait PTDataParse {
    fn parse_for_island(self, island_index: i32) -> Vec<PrayerData>;
}

fn tui(){
    
}

fn edit(){
    // start new session
    print!("\x1b[?1049h");
    println!("EDIT MODE\n changes are made to the config file\n");
    
    let raw_atoll_data : Vec<String> = get_data_from_file( "/atolls.csv");
    let raw_island_data: Vec<String> = get_data_from_file("/islands.csv");
    
    // [row][column:  0,1,2]   (0 = atoll_index, 1=name, 2=dhi_name)
    let atoll_data : Vec<Vec<&str>> = raw_atoll_data .iter().map(|x| x.split(';').collect()).collect();
    
    // [row][coloumn: 0,2,3,4] (0 = time index, 2=atoll, 3=name, 4=dhi_name)
    let island_data: Vec<Vec<&str>> = raw_island_data.iter().map(|x| x.split(';').collect()).collect();
    
    
    clear_screen();
    // print atoll list
    println!("Index\tName\tDhiName");
    println!("-----\t----\t-------");
    atoll_data .iter().for_each(|x| println!("{}\t{}\t{}",x[0],x[1],x[2]));
    println!("Input a number from the first colum to select Atoll(1 to 20) or select a timeset(41 to 82):");
    let selected_atoll_index: usize = get_number_input().expect("Must be a non zero positive integer");
    let selected_time_index : usize;
    
    if std::ops::RangeInclusive::new(1, 20).contains(&selected_atoll_index){
        clear_screen();
        // print island list for selected atoll
        println!("{0: <5} | {1: <7} | {2: <15} | {3: <10}","Index","Timeset","Island Name","Dhi Name");
        println!("-------------------------------------------");
        let mut i = 0;
        let mut selectables: Vec<usize> = vec![];
        for island in island_data.iter(){
            
            if island[2].parse::<usize>().unwrap_or(1) == selected_atoll_index{
                i += 1;
                selectables.append(&mut vec![island[0].parse::<usize>().unwrap_or(41)]);
                println!("{0: <5} | {1: <7} | {2: <15} | {3: <10}",i,island[0],island[3],island[4]);
            }
        }
        
        println!("Input a number from the first column to select prefered timeset:");
        selected_time_index =  selectables[get_number_input().unwrap()];
        
    }else if std::ops::RangeInclusive::new(41, 82).contains(&selected_atoll_index){
        selected_time_index = selected_atoll_index;
        
    }else{
        
        println!("\x1b[?1049l");
        
        panic!("value not within range");
    }
    
    
    
    let new_cfg = Config{island_index:selected_time_index, island_name:"WIP".to_string()};
    
    confy::store("salat_mv",None, &new_cfg).unwrap();
    
    // exit new session
    print!("\x1b[?1049l");
    
    println!("Timeset {} selected",selected_time_index);
    
    // println!("{}\n\n{}",atoll_data[3][0],island_data[2][3]);
    
    
}

fn active(prayer_data: Vec<PrayerData>, flag: &Flag){
    
    let today: usize = chrono::offset::Local::now().ordinal() as usize - 1;
    
    new_buffer();
    
    // active loop
    loop{
        
        let pt_vec = prayer_data[today].vec_from_island_set();
        let current_time = get_current_time_in_minutes() as i32;
        let (_,_,seconds,_) = get_current_time(&flag.time);
        
        // let current_time = 738;
        pt_vec.iter().for_each(|x| if seconds == 0 && x == &current_time && flag.notify{notify_send("ITS TIME")});
        
        prayer_data[today].flag_formatted_output(flag);
        sleep(Duration::from_secs(1));
        clear_screen();
    }
}



fn main(){
    
    handle_ctrlc();
    
    // let new_data :PrayerData = PrayerData { island_index: 77, day: 233, fajr: 1234, sun: 1234, dhuhur: 1231, asr: 123, magrib: 12312, isha: 1231 };
    // new_data.output();
    
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
    // ==========
    
    if flag.help{ // breakout for help
        println!("{}",HELP_TEXT);
        return;
    }
    
    // data path (depreciated)
    // let mut data_path: String = current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();
    // data_path.push_str("/ptdata.csv");
    
    // gets data from hard coded values
    let prayer_data: Vec<PrayerData> = get_island_data(cfg.island_index as u32);

    let today: usize = chrono::offset::Local::now().ordinal() as usize - 1;
    
    if flag.tui{
        tui();
    }
    else if flag.edit{
        edit();
    }
    
    else if flag.active{
        
        active(prayer_data, &flag);
        
        // Command::new("notify-send").args(["--urgency=low","ahahahahahahaha"]).output().expect("failed");
        // loop{
        //     std::thread::sleep_ms(1000);
        //     break;
        // }
        
    }else{
        
        prayer_data[today].flag_formatted_output(&flag);
        
    }
    
    // handle_prayer_data(flag, cfg); // run main thing
    
}




// smol functions
// ==============

// time converters
fn get_current_time_in_minutes() -> u32 {
    let current_time = chrono::offset::Local::now();
    current_time.hour() * 60 + current_time.minute()
}
fn get_current_time(format:&TimeFormat) -> (u32, u32, u32, String){
    let current_time = chrono::offset::Local::now();
    match format{
        TimeFormat::TFHour => (
            current_time.hour(),
            current_time.minute(),
            current_time.second(),
            "".to_string()),
            
        TimeFormat::TWHour => {
            let (current_hour, period) = current_time.hour().to_12();
            (current_hour, current_time.minute(), current_time.second(), period)
        },
    }
}

// input management
fn get_number_input() -> Result<usize,std::num::ParseIntError>{
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
    let trimmed = input_text.trim();
    trimmed.parse::<usize>()   
}

// get db data
fn get_island_data(timeset_index: u32) -> Vec<PrayerData>{
    let mut island_data:Vec<PrayerData> = vec![];
    for row in PTDATA{
        if row[0] == timeset_index{
            let pt_data: PrayerData = PrayerData {
                island_index: row[0],
                day    :  row[1],
                fajr   :  row[2],
                sun    :  row[3],
                dhuhur :  row[4],
                asr    :  row[5],
                magrib :  row[6],
                isha   :  row[7],
            };
            island_data.append(&mut vec![pt_data]);
        }
    }
    
    island_data    
}

// TODO: change of plans: learn to use build.rs to parse all the data at compile time
fn get_data_from_file(path:&str) -> Vec<String>{
    let mut data_path: String = current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();
    data_path.push_str(path);
    
     // gets data from database
    let data : String = fs::read_to_string(data_path)
        .expect("READ THE data.txt FILE DAMMIT");
    
    let mut grouped : Vec<&str> = data.split('\n').collect();
    grouped.pop();
    
    grouped.iter().map(|x| x.parse::<String>().unwrap()).collect()
}

// terminal screen functions
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

// handle SIGINT
fn handle_ctrlc(){
    let mut signals = Signals::new([SIGINT]).unwrap();
    
    thread::spawn(move || {
        for sig in signals.wait() {
            if sig == 2{exit_buffer();std::process::exit(0)}
        }
    });
}

// notifications
fn notify_send(message:&str){
    Command::new("notify-send").args(["--urgency=critical", message]).output().expect("failed");
}



