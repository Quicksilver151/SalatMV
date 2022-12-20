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

    fn print_data(&self){
        println!("Fajr:\t{}\nShuruq:\t{}\nDhuhur:\t{}\nAsr:\t{}\nMagrib:\t{}\nIsha:\t{}\n",
                self.fajr   .minutes_to_time(),
                self.sun    .minutes_to_time(),
                self.dhuhur .minutes_to_time(),
                self.asr    .minutes_to_time(),
                self.magrib .minutes_to_time(),
                self.isha   .minutes_to_time()
                )

    }

}



trait TimeConversion{
    fn minutes_to_time(self) -> String;
}

impl TimeConversion for i32{
    fn minutes_to_time(self)-> String{
        let minute = &self%60;
        let mut hour = self/60;
        let mut full_time_string = "".to_owned();
        let mut time = "am";
        if hour > 12{
            hour -= 12;
            time = "pm";
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

fn main() -> Result<(), Error> {
    let args : Vec<String> = env::args().collect();
    println!("{:?}",args);

    let backup_data:String = fs::read_to_string("/home/renderinguser/QuickAccess/Projects/codestuffz/Rust/SalatMV/src/ptdata.csv")
        .expect("READ THE data.txt FILE DAMMIT");

    let data:String = fs::read_to_string("./ptdata.csv").unwrap_or(backup_data);

    let prayer_data : Vec<PrayerData> = data.parse_for_island(77);
    // let prayer_data :Vec<PrayerData> = parse_text_for_island(data,77);

    let today :usize = chrono::offset::Local::now().ordinal() as usize;

    
    prayer_data[today].print_data();

    

    Ok(())
}
