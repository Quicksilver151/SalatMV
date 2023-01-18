use std::fs::write;

trait PTDataParse {
    fn parse_for_island(self, island_index: u32) -> Vec<Vec<u32>>;
}

impl PTDataParse for String{
    fn parse_for_island(self, island_index: u32) -> Vec<Vec<u32>>{

        // split by line for each valid data
        let mut grouped :Vec<&str> = self.split('\n').collect();
        grouped.pop(); // remove last line
        grouped.reverse();
        grouped.pop(); // remove first line
        grouped.reverse();
        
        
        // let mut full_list: [[u32; 8]; 15372];
        let mut full_list: Vec<Vec<u32>> = vec![];
        
        // split by column for each valid data
        for group in grouped.iter(){
            let columns: Vec<u32> = group.split(';').map(|x| x.parse::<u32>().unwrap_or(0)).collect();
            
            // skip irrelevant data
            // if island_index != columns[0]{
                // continue;
            // }
            
            let result = columns;
            // let mut result : PrayerData = PrayerData { island_index: (0), day: (0), fajr: (0), sun: (0), dhuhur: (0), asr: (0), magrib: (0), isha: (0) };
            
            // result.island_set_from_vec(columns.iter().map(|x| x.parse::<u32>().unwrap()).collect());
            full_list.append(&mut vec![result]);
        }
        
        full_list   
    }
}

fn format_as_rust_vec(pt_data:Vec<Vec<u32>>) -> String{
    let mut string : String = "".to_string();
    let default_str: String =
"pub static PTDATA: & [[u32; 8]; 15372] = &[".to_string();
    string.push_str(&default_str);
    for i in pt_data{
        string.push_str(&format!("{:?},\n",i));
        
        
    }
    string.push_str("];");
    
    string
    
}

fn main(){
    let data : &str = include_str!("./src/ptdata.csv");
    let pt_data = data.to_string().parse_for_island(77);
    let rust_code = format_as_rust_vec(pt_data);
    write("./src/db.rs", rust_code).unwrap_or(());
    // uneval::to_out_dir(pt_data, "pt_data.rs");
}
