use std::{env::{self, current_exe}, fs};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::process::Command;

mod flag_parser;
use flag_parser::*;


#[derive(Default, Debug, Serialize, Deserialize)]
struct Config{
    island_index: usize,
    island_name:String,
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

fn get_data_from_file(path:&str) -> Vec<String>{
    let mut data_path: String = current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();
    data_path.push_str(path);
    
     // gets data from database
    let data : String = fs::read_to_string(data_path)
        .expect("READ THE data.txt FILE DAMMIT");
    
    let mut grouped : Vec<&str> = data.split('\n').collect();
    grouped.pop();
    
    grouped.iter().map(|x| x.parse::<String>().unwrap()).collect()
    // new
    
}

// fn get_island_data() -> Vec<String>{
//     let mut data_path: String = current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string();
//     data_path.push_str("/islands.csv");
//     
//      // gets data from database
//     let data: String = fs::read_to_string(data_path)
//         .expect("READ THE data.txt FILE DAMMIT");
//     
//     let mut grouped :Vec<&str> = data.split('\n').collect();
//     grouped.pop();
//     
//     
//     // to String
//     grouped.iter().map(|x| x.parse::<String>().unwrap()).collect()
//      
//     // println!("{}",data);
//     
//     
// }


// TODO::::::::::::::::::;
fn tui(){
    println!("tui: feature not implemented ([yet]->i hope)");
    // WIP
}

fn edit(){
    
    // start new session
    print!("\x1b[?1049h");
    println!("EDIT MODE\n changes are made to the config file\n");
    
    let raw_atoll_data : Vec<String> = get_data_from_file( "/atolls.csv");
    let raw_island_data: Vec<String> = get_data_from_file("/islands.csv");
    
    // only [row][column:  0,1,2] useful (0 = atoll_index, 1=name, 2=dhi_name)
    let atoll_data : Vec<Vec<&str>> = raw_atoll_data .iter().map(|x| x.split(';').collect()).collect();
    
    // only [row][coloumn: 0,2,3] useful (0 = time index, 2=atoll, 3=name, 4=dhi_name)
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
    
    
    // println!("edit: feature not implemented ([yet]->i hope)");
    // same here
}

fn handle_prayer_data(flag: Flag, cfg: Config){
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
    // Debug loop over each minute of the day
    //----------------------------------
    // clear_screen();
    // let mut time_minutes = 0;
    // while  time_minutes < 1440{
    // std::thread::sleep_ms(5);
    // --------------------------------
    for (i,pt) in pt_vec.iter().enumerate(){
        if flag.tui{
            tui();
            break;
        }
        if flag.edit{
            edit();
            break;
        }
        if flag.notify{
            Command::new("notify-send").args(["--urgency=low","ahahahahahahaha"]).output().expect("failed");
            // loop{
            //     std::thread::sleep_ms(1000);
            //     break;
            // }
            break;
        }
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
                print!(" /---{}",tail_next_len);
            }
            if prev_diff.is_negative() && diff.is_positive(){
                print!(" \\---{}",tail_prev_len);
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
        
    };
    //------------------
    //time_minutes += 1;
    //}
    //-----------------
}

fn main(){
    
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
    
    
    handle_prayer_data(flag, cfg); // run main thing
    
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
// TODO CURRENT DOESNT WORK AT EXACT TIME
