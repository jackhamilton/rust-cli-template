use cli_builder_macros::cli_builder;
use std::env;
use freezable_trait::freezable;

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

cli_builder! {
    [
        Command {
            short_flag: 't',
            long_flag: "test",
            command: test_command,
            description: "Run a test"
        },
    ]
}

fn test_command() {
    println!("This is a test");
}
