use crate::*;

// pub static ATOLLS_DAT: &str = include_str!("../data/atolls.csv");
pub static ISLAND_DAT: &str = include_str!("../data/islands.csv");

pub fn edit() {
    // start new buffer
    print!("\x1b[?1049h");
    println!("EDIT MODE\n changes are made to the config file\n");
    
    let atoll_data : Vec<AtollData> = ATOLL_DATA.iter().map(|x| AtollData::new_from_array(*x)).collect();
    let raw_island_data: Vec<String> = get_vec_from_db(ISLAND_DAT);
    
    // [row][column:  0,1,2]   (0 = atoll_index, 1=name, 2=dhi_name)
    // let atoll_data: Vec<Vec<&str>> = raw_atoll_data
    //     .iter()
    //     .map(|x| x.split(';').collect())
    //     .collect();
    
    // [row][coloumn: 0,2,3,4] (0 = time index, 2=atoll, 3=name, 4=dhi_name)
    let island_data: Vec<Vec<&str>> = raw_island_data
        .iter()
        .map(|x| x.split(';').collect())
        .collect();
    
    clear_screen();
    // atoll title
    // println!("Index\tName\tDhiName");
    // println!("-----\t----\t-------");
         println!(
            "{0: <5} | {1: <10} | {2: <10}",
            "Index", "Eng Name", "Dhi Name",
        );
        println!("-----------------------------");   
    // print atoll list
    atoll_data
        .iter()
        .for_each(|atoll| println!("{0: <5} | {1: <10} | {2: <10}", atoll.index, atoll.en_code, atoll.dh_code));
    println!("Input a number from the first colum to select Atoll(1-20) or select a timeset(42-82):");
    let selected_atoll_index: usize =
        get_number_input().expect("Must be a non zero positive integer");
    let selected_time_index: usize;
    
    if std::ops::RangeInclusive::new(1, 20).contains(&selected_atoll_index) {
        clear_screen();
        // island title
        println!(
            "{0: <5} | {1: <7} | {2: <15} | {3: <10}",
            "Index", "Timeset", "Island Name", "Dhi Name"
        );
        println!("-------------------------------------------");
        let mut i = 0;
        let mut selectables: Vec<usize> = vec![];
        
        // print island list
        for island in island_data.iter() {
            if island[2].parse::<usize>().unwrap_or(1) == selected_atoll_index {
                i += 1;
                selectables.append(&mut vec![island[0].parse::<usize>().unwrap_or(41)]);
                println!(
                    "{0: <5} | {1: <7} | {2: <15} | {3: <10}",
                    i, island[0], island[3], island[4]
                );
            }
        }
        
        println!("Input a number from the first column to select prefered timeset:");
        selected_time_index = selectables[get_number_input().unwrap()];
    } else if std::ops::RangeInclusive::new(41, 82).contains(&selected_atoll_index) {
        selected_time_index = selected_atoll_index;
    } else {
        println!("\x1b[?1049l");
        
        panic!("value not within range");
    }
    
    let new_cfg = Config {
        island_index: selected_time_index,
        island_name: "WIP".to_string(),
    };
    
    confy::store("salat_mv", None, &new_cfg).unwrap();
    
    // exit new buffer
    print!("\x1b[?1049l");
    
    println!("Timeset {} selected", selected_time_index);
}
