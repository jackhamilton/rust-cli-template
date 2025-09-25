use crate::clilib::CLICommand;
use std::env;
use freezable_trait::freezable;

mod clilib;

static PRODUCT_NAME: &str = "product_name";

#[derive(Debug)]
#[freezable]
struct Config {
    // todo: configuration
    example: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            _unknown_fields: [].into(),
            example: "".into(),
        }
    }
}

fn get_config() -> Config {
    toml_configurator::get_config(PRODUCT_NAME.to_string())
}

struct Runtime {
    commands: Vec<CLICommand>,
    default_command: Option<fn()>
}
impl Runtime {
    fn new() -> Self {
        Self {
            commands: vec![
                CLICommand {
                    short_flag: 'h',
                    long_flag: "help".into(),
                    command: help,
                    description: "Explains available commands.".into()
                },
                CLICommand {
                    short_flag: 'v',
                    long_flag: "version".into(),
                    command: version,
                    description: "Outputs tool version.".into()
                },
            ],
            default_command: Some(help)
        }
    }

    fn run(&self, arg: String) {
        for command in &self.commands {
            if let Some(long_arg) = arg.strip_prefix("--") {
                if command.long_flag == long_arg {
                    (command.command)()
                }
            } else if let Some(short_arg) = arg.strip_prefix("-")
                   && command.short_flag.to_string() == short_arg {
                (command.command)()
            }
        }
    }

    fn gen_help(&self) -> String {
        let mut message: String = "Help:\n".into();
        for command in &self.commands {
            let helpmsg = format!("\t -{}, --{}: {}\n", command.short_flag, command.long_flag, command.description);
            message.push_str(&helpmsg);
        }
        message
    }
}

fn help() {
    // I'm not losing sleep over the fact that this will likely be invoked from an existing runtime
    // object. Mind, I am losing sleep, just for different reasons.
    let runtime = Runtime::new();
    println!("{}", runtime.gen_help());
    std::process::exit(0);
}

fn version() {
    println!("{} version {} by Jack Hamilton", PRODUCT_NAME, env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

fn main() {
    let runtime = Runtime::new();
    let args: Vec<String> = env::args().collect();
    // First arg is junk
    if args.is_empty() || args.len() == 1 {
        if let Some(command) = runtime.default_command {
            command()
        } else {
            help()
        }
    } else if args.len() > 2 {
        println!("Too many arguments!");
        help()
    } else {
        let arg = args[1].clone();
        runtime.run(arg);
    }
}
