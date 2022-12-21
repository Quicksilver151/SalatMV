use std::{io::Error, env};
use std::fs;
use chrono::prelude::*;


struct PrayerData{
    island_index:i32,
    day: i32,
    fajr: i32,
    sun:i32,
    dhuhur:i32,
    asr:i32,
    magrib:i32,
    isha:i32,
}

impl PrayerData{
    fn island_set_from_vec(&mut self, val:Vec<i32>){
        self.island_index = val[0];
        self.day    = val[1];
        self.fajr   = val[2];
        self.sun    = val[3];
        self.dhuhur = val[4];
        self.asr    = val[5];
        self.magrib = val[6];
        self.isha   = val[7];
    }
    
    fn print_data(&self,time_format:TimeFormat){
        println!("Fajr:\t{}\nShuruq:\t{}\nDhuhur:\t{}\nAsr:\t{}\nMagrib:\t{}\nIsha:\t{}",
                self.fajr   .minutes_to_time(&time_format),
                self.sun    .minutes_to_time(&time_format),
                self.dhuhur .minutes_to_time(&time_format),
                self.asr    .minutes_to_time(&time_format),
                self.magrib .minutes_to_time(&time_format),
                self.isha   .minutes_to_time(&time_format)
                )
        
    }
        
    fn print_data_as_hours_and_minutes(&self, time_format:TimeFormat){
        println!("{}\n{}\n{}\n{}\n{}\n{}",
                self.fajr   .minutes_to_time(&time_format),
                self.sun    .minutes_to_time(&time_format),
                self.dhuhur .minutes_to_time(&time_format),
                self.asr    .minutes_to_time(&time_format),
                self.magrib .minutes_to_time(&time_format),
                self.isha   .minutes_to_time(&time_format)
        )
        
    }

    fn print_data_as_minutes(&self){
        println!("{}\n{}\n{}\n{}\n{}\n{}",
                self.fajr   ,
                self.sun    ,
                self.dhuhur ,
                self.asr    ,
                self.magrib ,
                self.isha   
                )
        
    }
}



trait TimeConversion{
    fn minutes_to_time(self,time_format:&TimeFormat) -> String;
}

impl TimeConversion for i32{
    fn minutes_to_time(self,time_format:&TimeFormat)-> String{
        let minute = &self%60;
        let mut hour = self/60;
        let mut full_time_string = "".to_owned();
        let mut time = "";
        
        match time_format{
            TimeFormat::Twelve => {
                time = "am";
                if hour > 12{
                    hour -= 12;
                    time = "pm";
                }
            }
            
            TimeFormat::TwentyFour => {}
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
    fn parse_for_island(self, island_index:i32) -> Vec<PrayerData>;
}

impl PTDataParse for String{
    fn parse_for_island(self, island_index:i32) -> Vec<PrayerData>{
        
        
        let mut grouped :Vec<&str> = self.split('\n').collect();
        grouped.pop();
        grouped.reverse();
        grouped.pop();
        grouped.reverse();
        
        let mut full_list :Vec<PrayerData> = vec![];
        
        for group in grouped{
            let columns :Vec<&str> = group.split(';').collect();
            
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


fn handle_args(args: Vec<String>){

    let help_message:String = 
"SalatMV for cli

Usage: pt [option]

Options:
    -h, --help       shows this help section
    -o, --output     just outputs data (this is done by default)
    -r               outputs raw data in hours and minutes
    -R               outputs raw data in minutes
    -H  --hour       show time in 24 hour format
    -t, --tui        opens in tui mode
    -e, --edit       edit island index

config contains island index
config is stored in ~/.config/salat_mv/"
.to_string();
    let mut time_format = TimeFormat::Twelve;
    let mut format = Format::Normal;

    for arg in &args{
        
        
        if arg == "-h" || arg == "--help"           {println!("{}",help_message);return}

        else if arg == "-H" || arg == "--hour"      {time_format = TimeFormat::TwentyFour}

        else if arg == "-o" || arg == "--output"    {}
        
        else if arg == "-r"                         {format = Format::RawData}
        
        else if arg == "-R"                         {format = Format::RawMinuteData}
        
        else if arg == "-t" || arg == "--tui" {
            tui();
            return;
        }
        
        else if arg == "-e" || arg == "--edit" {
            println!("select number");
            return;
        }else{
            println!("invalid option entred");
        }
    }
    
    handle_prayer_data(format,time_format);

    

}

enum  Format {Normal,RawData, RawMinuteData}
enum TimeFormat {TwentyFour,Twelve}

fn handle_prayer_data(output_format: Format, time_format:TimeFormat){
    
    // gets data from file
    let backup_data:String = fs::read_to_string("/home/renderinguser/QuickAccess/Projects/codestuffz/Rust/SalatMV/src/ptdata.csv")
        .expect("READ THE data.txt FILE DAMMIT");
    
    let data:String = fs::read_to_string("./ptdata.csv").unwrap_or(backup_data);
    
    //TODO: get data from config
    
    
    // parse data for selected island
    let prayer_data : Vec<PrayerData> = data.parse_for_island(77);
    // let prayer_data :Vec<PrayerData> = parse_text_for_island(data,77);
    
    let today :usize = chrono::offset::Local::now().ordinal() as usize;
    
    match output_format{
        Format::Normal => prayer_data[today].print_data(time_format),
        Format::RawData => prayer_data[today].print_data_as_hours_and_minutes(time_format),
        Format::RawMinuteData => prayer_data[today].print_data_as_minutes(),
    };
    
}

fn tui(){
    
}

fn main() -> Result<(), Error> {

    let mut args : Vec<String> = env::args().collect();
    args.reverse();
    args.pop();
    args.reverse();
    
    if !args.is_empty(){
        handle_args(args);
    }else{
        handle_prayer_data(Format::Normal, TimeFormat::Twelve);
    }
    
    
    
    Ok(())
}
