use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(commands())
}

pub fn commands() -> Vec<Command<'static>> {
    vec![update_cli(), generate_cli()]
}

fn update_cli() -> Command<'static> {
    Command::new("update")
        .about("Download new binary from github release and replace inplace the binary")
}

fn generate_cli() -> Command<'static> {
    Command::new("generate")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config-file")
                .value_name("CONFIG_FILE")
                .help("Config file who describe the filesystem to generate.")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("folder")
                .short('f')
                .long("dest-folder")
                .value_name("DEST_FOLDER")
                .help("Destination folder where the filesystem will be generated.")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(true)
                .takes_value(true),
        )
        .about("Create folder tree and download git repositories")
}
