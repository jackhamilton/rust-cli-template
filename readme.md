Sets up a CLI app project using config_builder and cli_builder tools.

``` rust
use configurator_macros::config_builder;
use cli_builder_macros::cli_builder;
use toml_configurator::get_config;
use std::env;

config_builder!(
    // Field: Type = [default value]
    example: String = "test".to_string(),
);


cli_builder! {
    [
        CLICommand {
            // program_name -t and program_name --test both run this function
            short_flag: "t",
            long_flag: "test",
            // function name
            command: test_command,
            // for the help message and completions
            description: "Run a test"
        },
    ]
}

fn test_command() {
    println!("This is a {}", CONFIG.example);
}
```
