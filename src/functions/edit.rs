use crate::*;


pub fn edit() {
    // start new buffer
    print!("\x1b[?1049h");
    println!("EDIT MODE\n changes are made to the config file\n");
    
    let atoll_data : Vec<AtollData>  = ATOLL_DATA
        .iter()
        .map(|x| AtollData::new_from_array(*x))
        .collect();
    let island_data: Vec<IslandData> = ISLAND_DATA
        .iter()
        .map(|x| IslandData::new_from_array(*x))
        .collect();
    
    
    clear_screen();
    // atoll title
    // println!("Index\tName\tDhiName");
    // println!("-----\t----\t-------");
         println!(
            "{0: <5} | {1: <10} | {2: <10}",
            "Index".red(), "Eng Name".red(), "Dhi Name".red(),
        );
        println!("{}","-----------------------------");   
    // print atoll list
    atoll_data
        .iter()
        .for_each(|atoll| println!("{0: <5} | {1: <10} | {2: <10}", atoll.index.to_string().blue(), atoll.en_code, atoll.dh_code));
    
    println!("\nInput a number from the first colum to select Atoll(1-20) or select a timeset(42-82):");
    
    let selected_atoll_index: u32 = get_number_input().expect("Must be a non zero positive integer");
    let selected_time_index : u32;
    
    if std::ops::RangeInclusive::new(1, 20).contains(&selected_atoll_index) {
        clear_screen();
        // island title
        println!(
            "{0: <5} | {1: <7} | {2: <15} | {3: <10}",
            "Index".red(), "Timeset".red(), "Island Name".red(), "Dhi Name".red()
        );
        println!("{}","-------------------------------------------");
        
        let mut i = 0;
        let mut selectables: Vec<u32> = vec![];
        
        // print island list
        for island in island_data.iter() {
            if island.atoll == selected_atoll_index{
                i += 1;
                selectables.append(&mut vec![island.timeset]);
                println!(
                    "{0: <5} | {1: <7} | {2: <15} | {3: <10}",
                    i.to_string().blue(), island.timeset, island.en_name, island.dh_name
                );
            }
        }
        
        println!("\nInput a number from the first column to select your island:");
        selected_time_index = selectables[get_number_input().unwrap() as usize];
        
    }
    else if std::ops::RangeInclusive::new(41, 82).contains(&selected_atoll_index) {
        selected_time_index = selected_atoll_index;
    }
    else {
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
