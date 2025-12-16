use cli_builder_macros::cli_builder;
use std::env;
use freezable_trait::freezable;

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
    toml_configurator::get_config(env!("CARGO_PKG_NAME").into())
}

cli_builder! {
    [
        CLICommand {
            short_flag: "t",
            long_flag: "test",
            command: test_command,
            description: "Run a test"
        },
    ]
}

fn test_command() {
    println!("This is a test");
}
