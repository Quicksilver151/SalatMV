
#[derive(Debug, PartialEq)]
pub enum DispType {Normal,     Raw}
#[derive(Debug, PartialEq)]
pub enum OutType  {Hours , Minutes}
#[derive(Debug, PartialEq)]
pub enum TimeType {TWHour,  TFHour}

#[derive(Debug, PartialEq)]
pub struct Flag{
    pub help    : bool,
    pub tui     : bool,
    pub notify  : bool,
    pub edit    : bool,
    pub current : bool,
    pub title   : bool,
    pub disp    : DispType,
    pub output  : OutType,
    pub time    : TimeType,
}

pub fn new_flag() -> Flag{
    Flag {help:false, tui:false, notify:false, edit:false, current:false, title:false, disp:DispType::Normal, output:OutType::Hours, time:TimeType::TWHour}
}

pub const HELP_TEXT : &str =
"SalatMV for cli

Usage: salat_mv [option]

Options:
    -h, --help       shows this help section
    -T, --tui        runs in tui mode (not implemented yet)
    -n, --notify     runs in notify mode (requires \'notify-send\' command)
    -e, --edit       edit island index
    -c, --current    indicates the current time
    -t, --title      shows the title bar
    -r, --raw-data   outputs raw data in hours and minutes
    -m, --minutes    outputs raw data in minutes
    -H, --hour       show time in 24 hour format
    
config contains island index
config is stored in ~/.config/salat_mv/";

pub fn parse_args(mut args : Vec<String> ) -> Result<Flag, Flag>{
    
    // let mut args : Vec<String> = env::args().collect();
    
    args.reverse();
    args.pop();
    args.reverse();
    
    // println!("{:?}",args);
    
    let mut flag: Flag = new_flag();
    
    for arg in args{
        
        let arg_vec : Vec<char> = arg.chars().collect::<Vec<char>>();
        if arg_vec.len() == 1{println!("===INVALID FLAG ENTERED===\n\n{}",HELP_TEXT);return Err(flag)}
        
        else if arg_vec[0] == '-' && arg_vec[1] == '-'{
            
            let argument = arg.strip_prefix("--").unwrap();
            match argument{
                "help"     => flag.help    = true,
                "tui"      => flag.tui     = true,
                "notify"   => flag.notify  = true,
                "edit"     => flag.edit    = true,
                "current"  => flag.current = true,
                "title"    => flag.title   = true,
                "raw-data" => flag.disp    = DispType::Raw,
                "minutes"  => flag.output  = OutType::Minutes,
                "hour"     => flag.time    = TimeType::TFHour,
                
                 _  => {println!("===INVALID FLAG ENTERED===\n\n{}",HELP_TEXT);return Err(flag)}
            }
        }
        
        else if arg_vec[0] == '-'{
            for argchar in arg_vec{
                if argchar == '-'{continue;}
                match argchar{
                    'h' => flag.help    = true,
                    'T' => flag.tui     = true,
                    'n' => flag.notify  = true,
                    'e' => flag.edit    = true,
                    'c' => flag.current = true,
                    't' => flag.title   = true,
                    'r' => flag.disp    = DispType::Raw,
                    'm' => flag.output  = OutType::Minutes,
                    'H' => flag.time    = TimeType::TFHour,
                    
                     _  => {println!("==INVALID FLAG ENTERED===\n\n{}",HELP_TEXT);return Err(flag)}
                }
            }
        }
    }
    Ok(flag)   
}

