use std::char;

pub struct CLICommand {
    pub short_flag: char,
    pub long_flag: String,
    pub command: fn(),
    pub description: String
}

