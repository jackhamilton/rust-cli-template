use configurator_macros::config_builder;
use cli_builder_macros::cli_builder;
use std::env;

config_builder!(
    example: String = "".to_string(),
);

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
