use crate::*;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub island_index: u32,
    pub island_name: String,
}

pub struct AtollData{
    pub index: u32,
    pub en_code: String,
    pub dh_code: String,
    pub ar_code: String,
}
impl AtollData{
    pub fn new_from_array(data: [&str;4]) -> AtollData{
        let new_atoll_data: AtollData = AtollData {
            index: data[0].parse::<u32>().unwrap(),
            en_code: data[1].to_string(),
            dh_code: data[2].to_string(),
            ar_code: data[3].to_string(),
        };
        
        new_atoll_data
    }
}

pub struct IslandData{
    pub timeset: u32,
    pub index:   u32,
    pub atoll:   u32,
    pub en_name: String,
    pub dh_name: String,
    pub ar_name: String,
}
impl IslandData{
    pub fn new_from_array(data: [&str;10]) -> IslandData{
        IslandData {
            timeset: data[0].parse::<u32>().unwrap(),
            index:   data[1].parse::<u32>().unwrap(),
            atoll:   data[2].parse::<u32>().unwrap(),
            en_name: data[3].to_string(),
            dh_name: data[4].to_string(),
            ar_name: data[5].to_string(),
        }   
    }
}


#[derive(Debug)]
pub struct PrayerData {
    pub island_index: u32,
    pub day:    u32,
    pub fajr:   u32,
    pub sun:    u32,
    pub dhuhur: u32,
    pub asr:    u32,
    pub magrib: u32,
    pub isha:   u32,
}
impl PrayerData {
    pub fn new_from_array(val: &[u32; 8]) -> PrayerData {
        PrayerData {
            island_index: val[0],
            day:    val[1],
            fajr:   val[2],
            sun:    val[3],
            dhuhur: val[4],
            asr:    val[5],
            magrib: val[6],
            isha:   val[7],
        }
    }
    
    pub fn vec_from_island_set(&self) -> Vec<i32> {
        let mut val = vec![0; 6];
        val[0] = self.fajr   as i32;
        val[1] = self.sun    as i32;
        val[2] = self.dhuhur as i32;
        val[3] = self.asr    as i32;
        val[4] = self.magrib as i32;
        val[5] = self.isha   as i32;
        
        val
    }
    
    pub fn flag_formatted_output(&self, flag: &Flag) {
        let (flag, pt_vec) = (flag, self.vec_from_island_set());
        
        // some temporary inits
        let names = vec!["Fajr", "Sun", "Dhuhur", "Asr", "Magrib", "Isha"];
        
        // optional title
        if flag.title && !flag.tui && !flag.edit {
            let (hour, minute, second, time) = get_current_time(&flag.time);
            
            println!("Salat_MV-cli");
            println!("---------------------");
            println!(
                "Time   :  {}:{}:{} {}",
                hour.add_zero(),
                minute.add_zero(),
                second.add_zero(),
                time
            ); // TODO: let flag -H work
               // println!("Island :  {}",cfg.island_name);
            println!("---------------------");
            println!();
        }
        
        if flag.disp == DispType::Array{
            print!("[\"")
        }
        
        for (i, pt) in pt_vec.iter().enumerate() {
            let (mut prefix, mut time_display, mut suffix) = (String::new(),String::new(),String::new());
            match flag.disp {
                // only numbers or with info
                DispType::Normal => prefix = format!("{}:\t", names[i]),
                DispType::Raw    => prefix = "".to_owned(),
                DispType::Array  => suffix = {
                    if i == 5 {
                        "\"".to_owned()
                    }
                    else{
                        "\", \"".to_owned()
                    }
                },
            }
            match flag.output {
                OutType::Hours   => time_display = pt.minutes_to_time(&flag.time).to_owned(),
                OutType::Minutes => time_display = format!("{}", pt),
            }
            if flag.current {
                let time_minutes: i32 = get_current_time_in_minutes() as i32;
                let prev_diff: i32 = {
                    if i > 0 {
                        pt_vec[i - 1] - time_minutes
                    } else {
                        pt_vec[5] - time_minutes
                    }
                };
                let diff: i32 = pt - time_minutes;
                let next_diff: i32 = {
                    if i < 5 {
                        pt_vec[i + 1] - time_minutes
                    } else {
                        pt_vec[0] - time_minutes
                    }
                };
                
                let tail_prev_len = "-".repeat(10.min(prev_diff.abs() / 10) as usize);
                let tail_next_len = "-".repeat(10.min(next_diff.abs() / 10) as usize);
                
                // print!("\t{},\t{},\t{}",prev_diff,diff,next_diff);
                if diff.is_negative() && next_diff.is_positive() {
                    suffix = format!(" /{}", tail_next_len);
                }
                if prev_diff.is_negative() && diff.is_positive() {
                    suffix = format!(" \\{}", tail_prev_len);
                }
                if pt_vec[0] > time_minutes || pt_vec[5] < time_minutes {
                    if pt == &pt_vec[5] {
                        suffix = format!(" /{}", tail_next_len)
                    }
                    if pt == &pt_vec[0] {
                        suffix = format!(" \\{}", tail_prev_len)
                    }
                }
                if pt == &time_minutes {
                    suffix = " <<------------".to_string();
                }
            }
            
            print!("{}{}{}",prefix, time_display, suffix);
            
            if flag.disp != DispType::Array{println!()}
        }
        if flag.disp == DispType::Array{
            println!("]")
        }
    }
}

pub trait TimeConversion {
    fn add_zero(self) -> String;
    fn to_12(self) -> (u32, String);
    fn minutes_to_time(self, time_format: &TimeFormat) -> String;
}

impl TimeConversion for i32 {
    fn add_zero(self) -> String {
        if self < 10 {
            let mut string: String = "0".to_string();
            string.push_str(&self.to_string());
            string
        } else {
            self.to_string()
        }
    }
    
    fn to_12(self) -> (u32, String) {
        let half = if self > 11 { "pm" } else { "am" };
        if self > 12 {
            (self as u32 - 12, half.to_string())
        } else {
            (self as u32, half.to_string())
        }
    }
    
    fn minutes_to_time(self, time_format: &TimeFormat) -> String {
        let minute = &self % 60;
        let mut hour = self as u32 / 60;
        let mut period = "".to_string();
        
        match time_format {
            TimeFormat::TWHour => (hour, period) = hour.to_12(),
            TimeFormat::TFHour => {}
        }
        
        let hour = hour.add_zero();
        let minute = minute.add_zero();
        format!("{}:{} {}", hour, minute, period)
    }
}

impl TimeConversion for u32 {
    fn add_zero(self) -> String {
        if self < 10 {
            let mut string: String = "0".to_string();
            string.push_str(&self.to_string());
            string
        } else {
            self.to_string()
        }
    }
    
    fn to_12(self) -> (u32, String) {
        let half = if self > 11 { "pm" } else { "am" };
        if self > 12 {
            (self - 12, half.to_string())
        } else {
            (self, half.to_string())
        }
    }
    
    fn minutes_to_time(self, time_format: &TimeFormat) -> String {
        let minute = &self % 60;
        let mut hour = self / 60;
        let mut period = "".to_string();
        
        match time_format {
            TimeFormat::TWHour => (hour, period) = hour.to_12(),
            TimeFormat::TFHour => {}
        }
        
        let hour = hour.add_zero();
        let minute = minute.add_zero();
        format!("{}:{} {}", hour, minute, period)
    }
}
