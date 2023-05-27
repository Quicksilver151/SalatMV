#[derive(Debug, PartialEq, Default)]
pub enum DispType {
    #[default]
    Normal,
    Raw,
    Array,
}
#[derive(Debug, PartialEq, Default)]
pub enum OutType {
    #[default]
    Hours,
    Minutes,
}
#[derive(Debug, PartialEq, Default)]
pub enum TimeFormat {
    #[default]
    TWHour,
    TFHour,
}

#[derive(Default, Debug, PartialEq)]
pub struct Flag {
    pub help:    bool,
    pub tui:     bool,
    pub active:  bool,
    pub notify:  bool,
    pub edit:    bool,
    pub current: bool,
    pub title:   bool,
    pub disp:   DispType,
    pub output: OutType,
    pub time:   TimeFormat,
}

pub const HELP_TEXT : &str =
"\x1b[1;31mSalatMV for cli\x1b[0m
\x1b[1;35mUsage:\x1b[1;34m salat_mv \x1b[1;35m[option] \x1b[0m

\x1b[1;31mOptions:,\x1b[0m
    \x1b[1;35m-h\x1b[0m, \x1b[1;35m--help       \x1b[0mshows this help section
    \x1b[1;35m-a\x1b[0m, \x1b[1;35m--active     \x1b[0mkeeps the program always running
    \x1b[1;35m-n\x1b[0m, \x1b[1;35m--notify     \x1b[0menables notifications when using -a, edits notifications when not using -a (requires \'notify-send\' command)
    \x1b[1;35m-e\x1b[0m, \x1b[1;35m--edit       \x1b[0medit island index
    \x1b[1;35m-c\x1b[0m, \x1b[1;35m--current    \x1b[0mindicates the current time
    \x1b[1;35m-t\x1b[0m, \x1b[1;35m--title      \x1b[0mshows the title bar
    \x1b[1;35m-r\x1b[0m, \x1b[1;35m--raw-data   \x1b[0moutputs raw data in hours and minutes (incompatible with -A)
    \x1b[1;35m-A\x1b[0m, \x1b[1;35m--array      \x1b[0moutputs the data in an array (incompatible with -r, -c)
    \x1b[1;35m-m\x1b[0m, \x1b[1;35m--minutes    \x1b[0moutputs raw data in minutes
    \x1b[1;35m-H\x1b[0m, \x1b[1;35m--hour       \x1b[0mshow time in 24 hour format
    
config contains island index
config is stored in ~/.config/salat_mv/";

pub fn parse_args(mut args: Vec<String>) -> Result<Flag, Flag> {
    // let mut args : Vec<String> = env::args().collect();
    
    args.reverse();
    args.pop();
    args.reverse();
    
    // println!("{:?}",args);
    
    let mut flag: Flag = Flag::default();
    
    for arg in args {
        let arg_vec: Vec<char> = arg.chars().collect::<Vec<char>>();
        if arg_vec.len() == 1 {
            println!("===INVALID FLAG ENTERED===\n\n{}", HELP_TEXT);
            return Err(flag);
        }
        else if arg_vec[0] == '-' && arg_vec[1] == '-' {
            let argument = arg.strip_prefix("--").unwrap();
            match argument {
                "help"     => flag.help    = true,
                "tui"      => flag.tui     = true,
                "active"   => flag.active  = true,
                "notify"   => flag.notify  = true,
                "edit"     => flag.edit    = true,
                "current"  => flag.current = true,
                "title"    => flag.title   = true,
                "raw-data" => flag.disp   = DispType::Raw,
                "array"    => flag.disp   = DispType::Array,
                "minutes"  => flag.output = OutType::Minutes,
                "hour"     => flag.time   = TimeFormat::TFHour,
                _ => {
                    println!("===INVALID FLAG ENTERED===\n\n{}", HELP_TEXT);
                    return Err(flag);
                }
            }
            
        }
        else if arg_vec[0] == '-' {
            for argchar in arg_vec {
                if argchar == '-' {
                    continue;
                }
                match argchar {
                    'h' => flag.help    = true,
                    'T' => flag.tui     = true,
                    'a' => flag.active  = true,
                    'n' => flag.notify  = true,
                    'e' => flag.edit    = true,
                    'c' => flag.current = true,
                    't' => flag.title   = true,
                    'r' => flag.disp   = DispType::Raw,
                    'A' => flag.disp   = DispType::Array,
                    'm' => flag.output = OutType::Minutes,
                    'H' => flag.time   = TimeFormat::TFHour,
                    _ => {
                        println!("==INVALID FLAG ENTERED===\n\n{}", HELP_TEXT);
                        return Err(flag);
                    }
                }
            }
        }
    }
    Ok(flag)
}
