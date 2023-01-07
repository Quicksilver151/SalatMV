use std::{env::{self, current_exe}, fs};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::process::Command;

mod flag_parser;
use flag_parser::*;


#[derive(Default, Debug, Serialize, Deserialize)]
struct Config{
    island_index: usize,
    island_name:String,
}

struct PrayerData{
    island_index: i32,
    day:    i32,
    fajr:   i32,
    sun:    i32,
    dhuhur: i32,
    asr:    i32,
    magrib: i32,
    isha:   i32,
}




impl PrayerData{
    fn island_set_from_vec(&mut self, val: Vec<i32>){
        self.island_index = val[0];
        self.day    = val[1];
        self.fajr   = val[2];
        self.sun    = val[3];
        self.dhuhur = val[4];
        self.asr    = val[5];
        self.magrib = val[6];
        self.isha   = val[7];
    }
    fn vec_from_island_set(&self) -> Vec<i32>{
        let mut val = vec![0;8];
        val[0] = self.island_index;
        val[1] = self.day;
        val[2] = self.fajr;
        val[3] = self.sun;
        val[4] = self.dhuhur;
        val[5] = self.asr;
        val[6] = self.magrib;
        val[7] = self.isha;
        
        val
        
    }
}

fn main(){

}


