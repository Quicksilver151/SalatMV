use crate::*;

// time converters
pub fn get_current_time_in_minutes() -> u32 {
    let current_time = chrono::offset::Local::now();
    current_time.hour() * 60 + current_time.minute()
}
pub fn get_current_time(format: &TimeFormat) -> (u32, u32, u32, String) {
    let current_time = chrono::offset::Local::now();
    match format {
        TimeFormat::TFHour => (
            current_time.hour(),
            current_time.minute(),
            current_time.second(),
            "".to_string(),
        ),
        
        TimeFormat::TWHour => {
            let (current_hour, period) = current_time.hour().to_12();
            (
                current_hour,
                current_time.minute(),
                current_time.second(),
                period,
            )
        }
    }
}

// input management
pub fn get_number_input() -> Result<usize, std::num::ParseIntError> {
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
    let trimmed = input_text.trim();
    trimmed.parse::<usize>()
}

// get db data
pub fn get_island_data(timeset_index: u32) -> Vec<PrayerData> {
    let mut island_data: Vec<PrayerData> = vec![];
    
    for row in PTDATA {
        if row[0] == timeset_index {
            let pt_data: PrayerData = PrayerData::new_from_array(row);
            island_data.append(&mut vec![pt_data]);
        }
    }
    
    island_data
}

pub fn get_vec_from_db(db: &str) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    // for row in ATOLL_DATA{
    //     dbg!(row);
    // }
    // panic!();
    let mut grouped: Vec<&str> = db.split('\n').collect();
    grouped.pop();
    
    grouped
        .iter()
        .map(|x| x.parse::<String>().unwrap())
        .collect()
}

// terminal screen functions
pub fn clear_screen() {
    print!("\x1B[2J");
    print!("\x1b[1;1H");
}
pub fn new_buffer() {
    print!("\x1b[?1049h");
}
pub fn exit_buffer() {
    print!("\x1b[?1049l");
}

// handle SIGINT
pub fn handle_ctrlc() {
    let mut signals = Signals::new([SIGINT]).unwrap();
    
    thread::spawn(move || {
        for sig in signals.wait() {
            if sig == 2 {
                exit_buffer();
                std::process::exit(0)
            }
        }
    });
}

// notifications
pub fn notify_send(message: &str) {
    Command::new("notify-send")
        .args(["--urgency=critical", message])
        .output()
        .expect("failed");
}
