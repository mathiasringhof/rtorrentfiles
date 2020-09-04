extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("rtorrentfiles")
        .version(VERSION)
        .about("Work with torrent files listed in rtorrent")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("print").arg(
                Arg::with_name("URL")
                    .required(true)
                    .help("Since the URL to the API Server")
                    .index(1),
            ),
        )
        .get_matches();
    let config = matches.value_of("url");
    match config {
        Some(url) => {
            println!("Url is: {}", url);
        }
        None => {
            println!("Url is missing");
        }
    }
}
