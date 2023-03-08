use crate::*;


pub fn flag_formatted_output(pt_data: &PrayerData, flag: &Flag) {
    
    let (flag, pt_vec) = (flag, pt_data.vec_from_island_set());
    
    // some temporary inits
    let names = vec!["Fajr", "Sun", "Dhuhur", "Asr", "Magrib", "Isha"];
    
    // optional title
    if flag.title && !flag.tui && !flag.edit {
        let (hour, minute, second, time) = get_current_time(&flag.time);
        
        println!("Salat_MV-cli");
        println!("---------------------");
        println!(
            "Time   :  {}:{}:{}{}",
            hour.add_zero(),
            minute.add_zero(),
            second.add_zero(),
            time
        );
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



