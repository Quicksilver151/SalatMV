use crate::uneval;

trait PTDataParse {
    fn parse_for_island(self, island_index: i32) -> Vec<PrayerData>;
}

impl PTDataParse for String{
    fn parse_for_island(self, island_index: i32) -> [[u32; 8]; 15372]{

        // split by line for each valid data
        let mut grouped :Vec<&str> = self.split('\n').collect();
        grouped.pop(); // remove last line
        grouped.reverse();
        grouped.pop(); // remove first line
        grouped.reverse();
        
        let mut full_list: [[u32; 8]; 15372];
        
        // split by column for each valid data
        for group in grouped{
            let columns: Vec<&str> = group.split(';').collect();
            
            if island_index != columns[0].parse::<i32>().unwrap(){
                continue;
            }
            
            let mut result : PrayerData = PrayerData { island_index: (0), day: (0), fajr: (0), sun: (0), dhuhur: (0), asr: (0), magrib: (0), isha: (0) };
            
            result.island_set_from_vec(columns.iter().map(|x| x.parse::<u32>().unwrap()).collect());
            full_list.append(&mut vec![result]);
            
        }
        
        full_list   
    }
}
fn main(){
    let data : &str = include_str!("./src/ptdata.csv");
    let pt_data = data.to_string().parse();
    uneval::to_out_dir(pt_data, "pt_data.rs");
}
