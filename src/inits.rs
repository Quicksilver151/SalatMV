use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config{
    pub island_index: usize,
    pub island_name:String,
}

#[derive(Debug)]
pub struct PrayerData{
    pub island_index: u32,
    pub day:    u32,
    pub fajr:   u32,
    pub sun:    u32,
    pub dhuhur: u32,
    pub asr:    u32,
    pub magrib: u32,
    pub isha:   u32,
}    
pub   static PT_DAT_RAW : &str = include_str!("./ptdata.csv");
pub  static ATOLLS_DAT : &str = include_str!("./atolls.csv");
pub static ISLAND_DAT : &str = include_str!("./islands.csv");
