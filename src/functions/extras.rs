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
pub fn get_number_input() -> Result<u32, std::num::ParseIntError> {
    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
    let trimmed = input_text.trim();
    trimmed.parse::<u32>()
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
    match Command::new("notify-send")
        .args(["--app-name=Salat MV","--urgency=critical", message])
        .output(){
            Ok(_)  => println!("Prayer time"),
            Err(_) => println!("Could not find a notification daemon that supports sendings notifs via 'notify-send'"),
        }
}
