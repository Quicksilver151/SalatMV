use std::env;

#[derive(Debug)]
pub enum DispType {Normal,     Raw}
#[derive(Debug)]
pub enum OutType  {Hours , Minutes}
#[derive(Debug)]
pub enum TimeType {TWHour,  TFHour}

#[derive(Debug)]
pub struct Flag{
    help  : bool,
    tui   : bool,
    edit  : bool,
    disp  : DispType,
    output: OutType,
    time  : TimeType,
}

pub fn new_flag() -> Flag{
    Flag {help:false, tui:false, edit:false, disp:DispType::Normal, output:OutType::Hours, time:TimeType::TWHour}
}


pub fn parse_args(mut args : Vec<String> ) -> Result<Flag, Flag>{
    
    // let mut args : Vec<String> = env::args().collect();
    
    args.reverse();
    args.pop();
    args.reverse();
    
    println!("{:?}",args);
    
    let mut flag: Flag = new_flag();
    
    for arg in args{
        
        let arg_vec : Vec<char> = arg.chars().collect::<Vec<char>>();
        if arg_vec[0] == '-' && arg_vec[1] == '-'{
            
            let argument = arg.strip_prefix("--").unwrap();
            println!("{}",argument);
            match argument{
                "help"     => flag.help = true,
                "tui"      => flag.tui  = true,
                "edit"     => flag.edit = true,
                "raw-data" => flag.disp = DispType::Raw,
                "minutes"  => flag.output = OutType::Minutes,
                "hour"     => flag.time = TimeType::TFHour,
                 _  => {println!("invalid flag");return Err(flag)}
            }
        }
        
        else if arg_vec[0] == '-'{
            for argchar in arg_vec{
                if argchar == '-'{continue;}
                match argchar{
                    'h' => flag.help = true,
                    't' => flag.tui  = true,
                    'e' => flag.edit = true,
                    'r' => flag.disp = DispType::Raw,
                    'm' => flag.output = OutType::Minutes,
                    'H' => flag.time = TimeType::TFHour,
                     _  => {println!("invalid flag");return Err(flag)}
                }
            }
        }
    }
    Ok(flag)   
}

fn main() {
    let ok = parse_args(env::args().collect());
    
    println!("{:?}",ok.unwrap());
    
}


