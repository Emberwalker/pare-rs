#[macro_use]
extern crate clap;
extern crate pare;

use clap::{App, AppSettings, Arg, SubCommand};

use pare::{main_shorten, main_delete, main_meta};
use pare::structs::Config;

fn main() {
    // Clean exit as per https://stackoverflow.com/a/30285110 (until std::env::set_exit_status)
    if let Err(e) = real_main() {
        eprintln!("An error occurred: {}", e);
        std::process::exit(-1);
    }
}

fn real_main() -> Result<(), String> {
    // Command line parsing.
    let cmds = App::new("Pare")
        .version(crate_version!())
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Robert T. <arkan@drakon.io>")
        .about("Command line client for Condenser v1.")
        .arg(Arg::with_name("apikey")
            .value_name("APIKEY")
            .long("apikey")
            .help("Override the API key in the config file.")
            .env("PARE_APIKEY")
            .takes_value(true)
            .global(true))
        .arg(Arg::with_name("server")
            .value_name("URL")
            .long("server")
            .help("Override the server URL in the config file.")
            .env("PARE_SERVER")
            .takes_value(true)
            .global(true))
        .arg(Arg::with_name("debug")
            .long("debug")
            .short("d")
            .help("Enable debug output.")
            .env("PARE_DEBUG")
            .global(true))
        .subcommand(SubCommand::with_name("shorten")
            .alias("short")
            .about("Shortens a URL.")
            .arg(Arg::with_name("code")
                .long("code")
                .help("Specifies a custom shortcode A random one is selected if not provided..")
                .takes_value(true))
            .arg(Arg::with_name("meta")
                .long("meta")
                .help("Attaches user-defined metadata to the shortcode.")
                .takes_value(true))
            .arg(Arg::with_name("URL")
                .help("URL to shorten.")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("delete")
            .alias("rm")
            .about("Deletes an existing shortcode.")
            .arg(Arg::with_name("fail-no-exist")
                .long("fail-no-exist")
                .help("Return non-zero exit code if code didn't exist."))
            .arg(Arg::with_name("CODE")
                .help("Shortcode to delete.")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("meta")
            .about("Fetches metadata about an existing shortcode.")
            .arg(Arg::with_name("json")
                .long("json")
                .help("Return the fetched metadata as a JSON object."))
            .arg(Arg::with_name("CODE")
                .help("Code to fetch metadata for.")
                .required(true)
                .index(1)))
        .get_matches();

    let raw_conf = Config {
        apikey: cmds.value_of("apikey").map(|s| s.to_string()),
        server: cmds.value_of("server").map(|s| s.to_string()),
        debug: cmds.is_present("debug"),
    };

    let conf = raw_conf.merge_with_disk();

    match cmds.subcommand() {
        ("shorten", Some(matches)) => main_shorten(
            &conf,
            matches.value_of("code"),
            matches.value_of("meta"),
            matches.value_of("URL").unwrap(),
        ),
        ("delete", Some(matches)) => main_delete(
            &conf,
            matches.is_present("fail-no-exist"),
            matches.value_of("CODE").unwrap(),
        ),
        ("meta", Some(matches)) => main_meta(
            &conf,
            matches.is_present("json"),
            matches.value_of("CODE").unwrap()
        ),
        _ => unreachable!()
    }
}
